"""
부록 10 · 1단계 API 조사 (4차, 최종) — 케이스별 결과 추출 완결 + 1e-6 대조
경로: CompoundSystemDeflection.results_for(bearing) → BearingCompoundSystemDeflection
      → component_analysis_cases (케이스별 리스트) → 각각의 L10
(모델 저장 안 함)
"""
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
N_CASE = 3

design = Design.load(MODEL)
asm = design.all_parts_of_type_root_assembly()[0]
dp = asm.design_properties
pl = list(asm.all_parts_of_type_point_load())[0]
ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
uw = next(b for b in asm.all_parts_of_type_bearing() if "UW" in str(b))
lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
ds = lc0.design_state_load_case_group

import c1_pin  # noqa: E402
data = c1_pin.parse_dlc(c1_pin.DLC)


def set_loads(lc, rec):
    p = lc.inputs_for_point_load(pl)
    p.force_x.force = -rec["Fz"] * 1e3
    p.force_y.force = rec["Fy"] * 1e3
    p.axial_load.force = rec["Fx"] * 1e3
    p.moment_x.moment = -rec["Mz"] * 1e3
    p.moment_y.moment = rec["My"] * 1e3
    pw = lc.inputs_for_power_load(ipl)
    pw.speed = rec["rpm"] * RPM2RADS
    pw.torque = rec["Mx"] * 1e3


# ── 배치: 케이스 3개(서로 다른 하중) ──
made = []
for i in range(N_CASE):
    lc = lc0.duplicate(ds, f"probe_pt{i}")
    set_loads(lc, data[i])
    made.append(lc)
duty = dp.add_duty_cycle("probe_dc")
for lc in made:
    duty.add_static_load(lc)
csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
t0 = time.perf_counter()
csd.perform_analysis()
print(f"[배치] {N_CASE}케이스 일괄 해석 {time.perf_counter()-t0:.2f}s")

res = list(csd.results_for(uw))[0]
print(f"analysis_time 속성 = {getattr(res, 'analysis_time', '<없음>')}")
cases = list(res.component_analysis_cases)
print(f"component_analysis_cases: {len(cases)}개")
batch_l10 = {}
for j, c in enumerate(cases):
    nm = None
    for path in ("static_load_case.name", "load_case.name", "name"):
        o = c
        try:
            for part in path.split("."):
                o = getattr(o, part)
            nm = o
            break
        except Exception:
            pass
    try:
        l10 = c.component_detailed_analysis.iso2812007.basic_rating_life_cycles
    except Exception as e:
        print(f"  [{j}] 매핑={nm!r} L10 실패: {str(e).splitlines()[0][:60]}")
        # 구조 힌트
        hint = [n for n in dir(c) if not n.startswith("_")][:12]
        print(f"      type={type(c).__name__} 속성: {hint}")
        continue
    print(f"  [{j}] 매핑={nm!r}  L10={l10:.6e}")
    batch_l10[j if nm is None else str(nm)] = l10

# ── 단일: 같은 3개 하중을 순차 해석 ──
print("\n[단일] 동일 3점 순차 해석")
single_l10 = []
for i in range(N_CASE):
    set_loads(lc0, data[i])
    sd = lc0.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
    t0 = time.perf_counter()
    sd.perform_analysis()
    el = time.perf_counter() - t0
    v = sd.results_for(uw).component_detailed_analysis.iso2812007.basic_rating_life_cycles
    single_l10.append(v)
    print(f"  index{i}: L10={v:.6e}  ({el:.2f}s)")

# ── 대조 ──
print("\n[대조] 배치 vs 단일 (상대오차, 기준 ≤1e-6)")
bl = list(batch_l10.values())
if len(bl) == N_CASE:
    # 매핑 순서: 이름으로 맞추되 불명 시 순서 가정
    keys = list(batch_l10.keys())
    for i in range(N_CASE):
        # 이름에 pt{i} 가 있으면 해당 항목 사용
        key = next((k for k in keys if f"pt{i}" in str(k)), keys[i])
        rel = abs(batch_l10[key] / single_l10[i] - 1)
        print(f"  index{i} ↔ {key!r}: {rel:.2e}  {'✅' if rel <= 1e-6 else '❌'}")
else:
    print(f"  케이스 수 불일치: 배치 {len(bl)} vs 단일 {N_CASE}")

print("\n완료 (저장 안 함)")
