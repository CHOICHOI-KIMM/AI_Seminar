"""
부록 10 · 1단계 API 조사 (읽기 전용 — 모델 저장 안 함)
======================================================
판정 항목
 [A] Load Case 복제/생성/삭제 API 존재 여부
 [B] 케이스별 하중 개별 설정 가능 여부
 [C] duty cycle(또는 다중 케이스) 일괄 해석 트리거 존재 여부
 [D] 케이스별 결과 추출 가능 여부 (일괄 해석 후)
대상: 부록 8 모델 (50°C 유연체 FE)
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
KEY = ("load_case", "duty", "cycle", "add", "create", "duplicat", "remove",
       "delete", "copy", "static_load", "time_series", "analysis", "name")


def probe(obj, label, keys=KEY, show_call=True):
    print(f"\n--- [{label}] type={type(obj).__name__} ---")
    hits = []
    for n in sorted(dir(obj)):
        if n.startswith("_"):
            continue
        if not any(k in n.lower() for k in keys):
            continue
        try:
            v = getattr(obj, n)
        except Exception as e:
            print(f"    {n:44} <ERR {str(e).splitlines()[0][:40]}>")
            continue
        kind = "()" if callable(v) else "="
        if callable(v):
            if show_call:
                print(f"    {n:44} (메서드)")
        else:
            s = str(v)
            print(f"    {n:44} = {s[:60]}")
        hits.append(n)
    return hits


print("=" * 70)
print("모델 로드:", os.path.basename(MODEL))
design = Design.load(MODEL)
asm = design.all_parts_of_type_root_assembly()[0]
dp = asm.design_properties
pl = list(asm.all_parts_of_type_point_load())[0]
ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
bearings = list(asm.all_parts_of_type_bearing())

# ── [0] 기존 구조 파악 ──
print("\n" + "=" * 70)
print("[0] 기존 하중 구조")
print("=" * 70)
sls = list(dp.static_loads)
print(f"  static_loads: {len(sls)}개")
for c in sls:
    print(f"    - name={getattr(c, 'name', '?')!r}  type={type(c).__name__}")

probe(dp, "design_properties (하중 관련)",
      keys=("load", "duty", "cycle", "static", "time_series"))

# duty cycle 접근 시도
duty = None
for attr in ("duty_cycles", "load_case_groups", "duty_cycle"):
    try:
        v = getattr(dp, attr)
        lst = list(v) if hasattr(v, "__iter__") else [v]
        print(f"\n  dp.{attr}: {[str(getattr(x,'name',x)) for x in lst]}")
        if lst:
            duty = lst[0]
    except Exception as e:
        print(f"  dp.{attr}: <없음: {str(e).splitlines()[0][:50]}>")

if duty is not None:
    probe(duty, "duty cycle 객체")

# ── [A] Load Case 복제/생성 ──
print("\n" + "=" * 70)
print("[A] Load Case 복제/생성/삭제")
print("=" * 70)
lc0 = sls[0]
probe(lc0, "LoadCase 객체 (복제·삭제 후보)",
      keys=("duplicat", "copy", "delete", "remove", "name", "add"))

new_cases = []
t0 = time.perf_counter()
for label, fn in (
    ("lc0.duplicate()", lambda: lc0.duplicate()),
    ("duty.add_static_load_case('t1')",
     lambda: duty.add_static_load_case("probe_t1") if duty else None),
    ("dp.add_static_load('t2')", lambda: dp.add_static_load("probe_t2")),
):
    try:
        r = fn()
        print(f"  {label:40} → 성공: {type(r).__name__} name={getattr(r,'name','?')!r}")
        if r is not None:
            new_cases.append(r)
    except Exception as e:
        print(f"  {label:40} → 실패: {str(e).splitlines()[0][:70]}")
print(f"  (소요 {time.perf_counter()-t0:.2f}s)")
print(f"  생성 후 static_loads 개수: {len(list(dp.static_loads))}")

# ── [B] 케이스별 하중 개별 설정 ──
print("\n" + "=" * 70)
print("[B] 케이스별 하중 개별 설정")
print("=" * 70)
REC = [dict(rpm=5.09167, Fx=3017.07, Fy=126.45, Fz=-4965.6,
            Mx=23992.2, My=-13666.5, Mz=4814.75),
       dict(rpm=5.0939, Fx=3016.6, Fy=138.2, Fz=-4929.8,
            Mx=24006.6, My=-13130.1, Mz=4670.5)]
targets = [lc0] + new_cases[:1]
ok_b = True
for i, (lc, rec) in enumerate(zip(targets, REC)):
    try:
        p = lc.inputs_for_point_load(pl)
        p.force_x.force = -rec["Fz"] * 1e3
        p.force_y.force = rec["Fy"] * 1e3
        p.axial_load.force = rec["Fx"] * 1e3
        p.moment_x.moment = -rec["Mz"] * 1e3
        p.moment_y.moment = rec["My"] * 1e3
        pw = lc.inputs_for_power_load(ipl)
        pw.speed = rec["rpm"] * RPM2RADS
        pw.torque = rec["Mx"] * 1e3
        print(f"  케이스[{i}] {getattr(lc,'name','?')!r}: 하중 설정 성공 "
              f"(force_x={p.force_x.force:,.0f})")
    except Exception as e:
        ok_b = False
        print(f"  케이스[{i}]: 실패 — {str(e).splitlines()[0][:70]}")

# ── [C] 일괄 해석 트리거 ──
print("\n" + "=" * 70)
print("[C] 일괄 해석 트리거 (duty cycle / compound)")
print("=" * 70)
if duty is not None:
    hits = probe(duty, "duty cycle 해석 메서드", keys=("analysis", "perform", "run"))
batch = None
for label, fn in (
    ("duty.analysis_of(SYSTEM_DEFLECTION)",
     lambda: duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION) if duty else None),
    ("lc0.analysis_of + 개별 (fallback 확인용)",
     lambda: lc0.analysis_of(AnalysisType.SYSTEM_DEFLECTION)),
):
    try:
        t0 = time.perf_counter()
        r = fn()
        if r is None:
            continue
        print(f"  {label:44} → 객체: {type(r).__name__}")
        try:
            t1 = time.perf_counter()
            r.perform_analysis()
            print(f"     perform_analysis() 성공  ({time.perf_counter()-t1:.2f}s)")
            batch = r
            break
        except Exception as e:
            print(f"     perform_analysis 실패: {str(e).splitlines()[0][:70]}")
    except Exception as e:
        print(f"  {label:44} → 실패: {str(e).splitlines()[0][:70]}")

# ── [D] 케이스별 결과 추출 ──
print("\n" + "=" * 70)
print("[D] 일괄 해석 결과에서 케이스별 추출")
print("=" * 70)
if batch is not None:
    probe(batch, "일괄 해석 결과 객체", keys=("load_case", "results", "static", "analysis"))
    try:
        res = batch.results_for(bearings[0])
        print(f"  results_for(UW) → {type(res).__name__}")
        d = getattr(res, "component_detailed_analysis", None)
        if d is not None:
            v = getattr(getattr(d, "iso2812007", None), "basic_rating_life_cycles", None)
            print(f"  L10 basic = {v}")
    except Exception as e:
        print(f"  results_for 실패: {str(e).splitlines()[0][:70]}")
else:
    print("  (일괄 해석 객체 없음 — [C] 실패)")

print("\n완료 (모델 저장 안 함)")
