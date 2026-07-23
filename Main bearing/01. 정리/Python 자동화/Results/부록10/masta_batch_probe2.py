"""
부록 10 · 1단계 API 조사 (2차) — 시그니처 확정 + 실동작 검증 (모델 저장 안 함)
[A2] duplicate/add_static_load/create_load_cases/add_duty_cycle 시그니처·실동작
[C2] 전용 duty cycle 구성 → 일괄 해석 시간 측정
[D2] results_for 제너레이터 구조: 케이스 매핑·L10 추출
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


def sig(obj, name):
    try:
        print(f"  {name}{inspect.signature(getattr(obj, name))}")
    except Exception as e:
        print(f"  {name}: <시그니처 확인 불가 {e}>")


design = Design.load(MODEL)
asm = design.all_parts_of_type_root_assembly()[0]
dp = asm.design_properties
pl = list(asm.all_parts_of_type_point_load())[0]
ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
bearings = list(asm.all_parts_of_type_bearing())
lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
duty_all = list(dp.duty_cycles)[0]

print("=" * 70)
print("[A2] 시그니처")
print("=" * 70)
sig(lc0, "duplicate")
sig(duty_all, "add_static_load")
sig(duty_all, "create_load_cases")
sig(dp, "add_duty_cycle")
sig(duty_all, "remove_static_load")

print("\n" + "=" * 70)
print("[A2] 실동작 — 케이스 5개 생성 (시계열 index 0~4)")
print("=" * 70)
import c1_pin  # noqa: E402
data = c1_pin.parse_dlc(c1_pin.DLC)

made = []
t0 = time.perf_counter()
for i in range(5):
    name = f"probe_pt{i}"
    lc = None
    for label, fn in ((f"duplicate('{name}')", lambda: lc0.duplicate(name)),
                      (f"duty.add_static_load('{name}')",
                       lambda: duty_all.add_static_load(name))):
        try:
            lc = fn()
            if i == 0:
                print(f"  성공 경로: {label} → {type(lc).__name__} name={lc.name!r}")
            break
        except Exception as e:
            if i == 0:
                print(f"  실패: {label} — {str(e).splitlines()[0][:70]}")
    if lc is None:
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
print(f"  생성·하중설정: {len(made)}개  ({time.perf_counter()-t0:.2f}s)")
print(f"  static_loads 총 개수: {len(list(dp.static_loads))} (기존 17 + {len(made)})")

print("\n" + "=" * 70)
print("[C2] 전용 duty cycle 구성 시도")
print("=" * 70)
duty_new = None
try:
    duty_new = dp.add_duty_cycle("probe_dc")
    print(f"  add_duty_cycle('probe_dc') → {type(duty_new).__name__}")
    for lc in made:
        try:
            duty_new.add_static_load(lc)      # 기존 케이스 연결 시도
        except Exception:
            pass
    print(f"  probe_dc 케이스 수: {duty_new.number_of_load_cases}")
except Exception as e:
    print(f"  add_duty_cycle 실패: {str(e).splitlines()[0][:70]}")

target = duty_new if (duty_new is not None and
                      float(getattr(duty_new, "number_of_load_cases", 0)) >= len(made)) else duty_all
print(f"  일괄해석 대상: {target.name!r} ({target.number_of_load_cases:.0f}케이스)")

print("\n" + "=" * 70)
print("[C2] 일괄 해석 시간 (2회 — 웜업 효과 분리)")
print("=" * 70)
csd = target.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
for it in (1, 2):
    t0 = time.perf_counter()
    csd.perform_analysis()
    el = time.perf_counter() - t0
    ncase = float(target.number_of_load_cases)
    print(f"  {it}회차: {el:.2f}s / {ncase:.0f}케이스 = {el/ncase*1000:.0f} ms/케이스")

print("\n" + "=" * 70)
print("[D2] results_for 제너레이터 구조")
print("=" * 70)
res_list = list(csd.results_for(bearings[0]))
print(f"  results_for(UW) → {len(res_list)}개 항목")
for j, r in enumerate(res_list[:4]):
    nm = None
    for attr in ("load_case", "static_load_case", "name"):
        try:
            v = getattr(r, attr)
            nm = getattr(v, "name", v)
            if nm:
                break
        except Exception:
            pass
    l10 = None
    try:
        d = r.component_detailed_analysis
        l10 = d.iso2812007.basic_rating_life_cycles
    except Exception as e:
        # compound 내 개별 케이스 접근 구조가 다를 수 있음 → 하위 탐색
        sub = [n for n in dir(r) if not n.startswith("_")][:14]
        print(f"    [{j}] type={type(r).__name__} 매핑={nm!r} L10 직접실패 → 속성: {sub}")
        continue
    print(f"    [{j}] type={type(r).__name__} 매핑={nm!r} L10={l10:.5e}")

print("\n완료 (저장 안 함)")
