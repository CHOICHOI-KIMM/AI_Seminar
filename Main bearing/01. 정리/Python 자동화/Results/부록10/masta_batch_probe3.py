"""
부록 10 · 1단계 API 조사 (3차) — DesignState 확보 → duplicate 실동작 → 일괄해석·추출 완결
(모델 저장 안 함)
"""
import inspect
import math
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)

import masta_clr_legacy  # noqa: F401,E402
import mastapy           # noqa: E402
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design                                    # noqa: E402
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType  # noqa: E402

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안"
         r"_피로하중 반영_유연체_FE_온도_50도_260721.Masta")
RPM2RADS = 2 * math.pi / 60

design = Design.load(MODEL)
asm = design.all_parts_of_type_root_assembly()[0]
dp = asm.design_properties
pl = list(asm.all_parts_of_type_point_load())[0]
ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
bearings = list(asm.all_parts_of_type_bearing())
lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
duty_all = list(dp.duty_cycles)[0]

print("=" * 70)
print("[1] DesignState 객체 확보")
print("=" * 70)
ds = None
# 경로 0 (확정): Load Case 1 이 속한 DesignState
try:
    v = lc0.design_state_load_case_group
    if v is not None and type(v).__name__ == "DesignState":
        ds = v
        print(f"  lc0.design_state_load_case_group → DesignState {getattr(ds,'name','')!r} 확보")
except Exception as e:
    print("  경로0 실패:", str(e).splitlines()[0][:50])
# 경로 1: dp 의 design_state 관련 속성
for n in sorted(dir(dp)):
    if "design_state" in n.lower() and not n.startswith("_"):
        print(f"  dp.{n}", "(메서드)" if callable(getattr(dp, n, None)) else
              f"= {getattr(dp, n, None)}")
# 경로 2: duty cycle 의 sub group
try:
    subs = list(duty_all.duty_cycle_design_states)
    print(f"  duty.duty_cycle_design_states: {len(subs)}개")
    for sgi in subs[:3]:
        print(f"    - {type(sgi).__name__}: name={getattr(sgi,'name','?')!r}")
        for n in sorted(dir(sgi)):
            if "design_state" in n.lower() and not n.startswith("_"):
                v = getattr(sgi, n, None)
                print(f"        .{n} = {type(v).__name__}", getattr(v, "name", ""))
except Exception as e:
    print("  sub group 실패:", str(e).splitlines()[0][:60])
# 경로 3: lc0 에서 소속 design state
for n in sorted(dir(lc0)):
    if ("design_state" in n.lower() or n == "load_case_group") and not n.startswith("_"):
        try:
            v = getattr(lc0, n)
            print(f"  lc0.{n} = {type(v).__name__} {getattr(v,'name','')!r}")
        except Exception as e:
            print(f"  lc0.{n} <ERR {str(e).splitlines()[0][:40]}>")
# 경로 4: 이름으로 조회
if ds is None:
    for nm in ("Design State 1", "DesignState", "Default"):
        try:
            ds = dp.design_state_load_case_group_named(nm)
            print(f"  design_state_load_case_group_named({nm!r}) → {type(ds).__name__}")
            break
        except Exception as e:
            print(f"  named({nm!r}) 실패: {str(e).splitlines()[0][:50]}")
print(f"  → 확보된 DesignState: {type(ds).__name__ if ds is not None else '없음'} "
      f"{getattr(ds, 'name', '')!r}")

if ds is None:
    print("\nDesignState 미확보 → 중단")
    sys.exit(0)

print("\n" + "=" * 70)
print("[2] duplicate(ds, name) 실동작 — 5케이스")
print("=" * 70)
import c1_pin  # noqa: E402
data = c1_pin.parse_dlc(c1_pin.DLC)
made = []
t0 = time.perf_counter()
for i in range(5):
    try:
        lc = lc0.duplicate(ds, f"probe_pt{i}")
    except Exception as e:
        print(f"  duplicate 실패[{i}]: {str(e).splitlines()[0][:70]}")
        break
    rec = data[i]
    p = lc.inputs_for_point_load(pl)
    p.force_x.force = -rec["Fz"] * 1e3
    p.force_y.force = rec["Fy"] * 1e3
    p.axial_load.force = rec["Fx"] * 1e3
    p.moment_x.moment = -rec["Mz"] * 1e3
    p.moment_y.moment = rec["My"] * 1e3
    pw = lc.inputs_for_power_load(ipl)
    pw.speed = rec["rpm"] * RPM2RADS
    pw.torque = rec["Mx"] * 1e3
    made.append(lc)
print(f"  생성·하중설정 {len(made)}개 ({time.perf_counter()-t0:.2f}s), "
      f"static_loads={len(list(dp.static_loads))}개")

if not made:
    sys.exit(0)

print("\n" + "=" * 70)
print("[3] 전용 duty cycle 'probe_dc' + 일괄 해석 (2회)")
print("=" * 70)
duty_new = dp.add_duty_cycle("probe_dc")
for lc in made:
    duty_new.add_static_load(lc)
print(f"  probe_dc 케이스 수: {duty_new.number_of_load_cases:.0f}")
csd = duty_new.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
for it in (1, 2):
    t0 = time.perf_counter()
    csd.perform_analysis()
    el = time.perf_counter() - t0
    n = float(duty_new.number_of_load_cases)
    print(f"  {it}회차: {el:.2f}s / {n:.0f}케이스 = {el/max(n,1)*1000:.0f} ms/케이스")

print("\n" + "=" * 70)
print("[4] 케이스별 결과 추출·매핑 + 단일해석 대조")
print("=" * 70)
res_list = list(csd.results_for(bearings[0]))
print(f"  results_for(UW): {len(res_list)}개")
vals = {}
for j, r in enumerate(res_list):
    nm = None
    for attr in ("load_case_name", "name"):
        try:
            nm = getattr(r, attr)
            if nm:
                break
        except Exception:
            pass
    if nm is None:
        try:
            nm = r.static_load_case.name
        except Exception:
            pass
    try:
        l10 = r.component_detailed_analysis.iso2812007.basic_rating_life_cycles
    except Exception:
        l10 = None
        attrs = [n for n in dir(r) if not n.startswith("_")
                 and any(k in n.lower() for k in ("case", "name", "detail", "analys"))]
        print(f"    [{j}] {type(r).__name__} 매핑={nm!r} L10 실패 → 후보속성 {attrs[:8]}")
        continue
    print(f"    [{j}] 매핑={nm!r}  L10={l10:.6e}")
    if nm:
        vals[str(nm)] = l10

# 단일 해석 대조 (probe_pt0 하중을 Load Case 1 에 넣고 단독 해석)
rec = data[0]
p = lc0.inputs_for_point_load(pl)
p.force_x.force = -rec["Fz"] * 1e3
p.force_y.force = rec["Fy"] * 1e3
p.axial_load.force = rec["Fx"] * 1e3
p.moment_x.moment = -rec["Mz"] * 1e3
p.moment_y.moment = rec["My"] * 1e3
pw = lc0.inputs_for_power_load(ipl)
pw.speed = rec["rpm"] * RPM2RADS
pw.torque = rec["Mx"] * 1e3
sd = lc0.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
t0 = time.perf_counter()
sd.perform_analysis()
el1 = time.perf_counter() - t0
l10_single = sd.results_for(bearings[0]).component_detailed_analysis.iso2812007.basic_rating_life_cycles
print(f"\n  단일 해석(index0): L10={l10_single:.6e}  ({el1:.2f}s)")
for key in ("probe_pt0",):
    if key in vals:
        rel = abs(vals[key] / l10_single - 1)
        print(f"  배치 '{key}' 대비 상대오차 = {rel:.2e}  "
              f"{'✅ ≤1e-6' if rel <= 1e-6 else '❌ 초과'}")

print("\n완료 (저장 안 함)")
