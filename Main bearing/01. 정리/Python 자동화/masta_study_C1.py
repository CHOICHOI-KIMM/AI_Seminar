"""
부록 C-1 스터디: 데미지 계산 방법 비교 (A/B/C) + 계산시간 정량분석
================================================================
A. ISO16281 per-step (수정 L10mr) → Miner
B. ISO281   per-step (수정 L10m)  → Miner
C. ISO281 등가하중(LRD): 점별 P → 회전수가중 p제곱평균 P_eq → 해석식 L10 → aISO(회전수가중평균) → 단일 손상
수명기준=수정, 지표=표본손상 D + 등가 L10 수명(Rev), p=10/3(롤러).
결과는 stdout 리포트 → 프로세스.md 부록 C-1에 전사(CSV 미출력).
"""
import math
import time
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

MODEL = r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_260720.Masta"
DLC   = r"D:\AI\AI_Seminar\Main bearing\02. 자료\260714 유니슨 피로하중\DLC1.2-c-s1.$150"
N_PTS = 6001
DT    = 0.1
P_EXP = 10.0 / 3.0          # 롤러
SCALE_FACTOR = 45040.0      # FatigueHours DLC1.2-c-s1 (30년 반복횟수)
DESIGN_YEARS = 30.0
RPM2RADS = 2 * math.pi / 60


def parse(path, n):
    rows = []
    with open(path, "r", encoding="latin-1") as f:
        for ln in f.readlines()[4:]:
            p = ln.split()
            if len(p) < 8:
                continue
            try:
                v = [float(x) for x in p[:8]]
            except ValueError:
                continue
            rows.append(dict(t=v[0], rpm=v[1], Mx=v[2], My=v[3], Mz=v[4],
                             Fx=v[5], Fy=v[6], Fz=v[7]))
            if len(rows) >= n:
                break
    return rows


def g(o, path):
    cur = o
    for pp in path.split("."):
        try:
            cur = getattr(cur, pp)
        except Exception:
            return None
        if cur is None:
            return None
    return cur


def fin(x):
    return isinstance(x, (int, float)) and math.isfinite(x)


t0 = time.perf_counter()
design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]
pl = list(assembly.all_parts_of_type_point_load())[0]
bearings = list(assembly.all_parts_of_type_bearing())
ipl = next(p for p in assembly.all_parts_of_type_power_load() if "input" in str(p).lower())
lc = next(c for c in assembly.design_properties.static_loads
          if getattr(c, "name", "") == "Load Case 1")
t_load = time.perf_counter() - t0

data = parse(DLC, N_PTS)
# 베어링별 점별 데이터 수집
rec_b = {str(b): [] for b in bearings}
solve_times = []

for i, rec in enumerate(data):
    ts = time.perf_counter()
    p = lc.inputs_for_point_load(pl)
    p.force_x.force = -rec["Fz"] * 1000
    p.force_y.force = rec["Fy"] * 1000
    p.axial_load.force = rec["Fx"] * 1000
    p.moment_x.moment = -rec["Mz"] * 1000
    p.moment_y.moment = rec["My"] * 1000
    lc.inputs_for_power_load(ipl).speed = rec["rpm"] * RPM2RADS
    sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
    sd.perform_analysis()
    rev = (rec["rpm"] / 60.0) * DT
    for b in bearings:
        d = g(sd.results_for(b), "component_detailed_analysis")
        rec_b[str(b)].append(dict(
            rev=rev,
            P=g(d, "iso2812007.dynamic_equivalent_load"),
            L10b=g(d, "iso2812007.basic_rating_life_cycles"),
            L10m=g(d, "iso2812007.modified_rating_life_cycles"),
            L10r=g(d, "isots162812008.basic_reference_rating_life_cycles"),
            L10mr=g(d, "isots162812008.modified_reference_rating_life_cycles"),
            aISO=g(d, "iso2812007.life_modification_factor_for_systems_approach"),
        ))
    solve_times.append(time.perf_counter() - ts)

warm = solve_times[0]
steady = solve_times[1:]
t_solve_avg = sum(steady) / len(steady)


def method_A(rows):
    return sum(r["rev"] / r["L10mr"] for r in rows if fin(r["L10mr"]) and r["L10mr"] > 0)


def method_B(rows):
    return sum(r["rev"] / r["L10m"] for r in rows if fin(r["L10m"]) and r["L10m"] > 0)


def method_C(rows):
    N = sum(r["rev"] for r in rows)
    # C(정격) 역산: C = P*(L10b/1e6)^(1/p), 점별 평균
    Cs = [r["P"] * (r["L10b"] / 1e6) ** (1.0 / P_EXP)
          for r in rows if fin(r["P"]) and fin(r["L10b"]) and r["P"] > 0 and r["L10b"] > 0]
    C = sum(Cs) / len(Cs)
    P_eq = (sum(r["rev"] * r["P"] ** P_EXP for r in rows if fin(r["P"])) / N) ** (1.0 / P_EXP)
    a_iso = sum(r["rev"] * r["aISO"] for r in rows if fin(r["aISO"])) / N
    L10b_eq = (C / P_eq) ** P_EXP * 1e6
    L10m_C = a_iso * L10b_eq
    D = N / L10m_C
    return D, dict(N=N, C=C, P_eq=P_eq, a_iso=a_iso, L10b_eq=L10b_eq, L10m_C=L10m_C)


# --- 계산시간(집계) 측정: 반복 실행 평균 ---
def time_it(fn, rows, reps=100):
    t = time.perf_counter()
    for _ in range(reps):
        fn(rows)
    return (time.perf_counter() - t) / reps


print("=" * 72)
print(f"부록 C-1 스터디 | DLC1.2-c-s1 | {N_PTS}점 표본 | p={P_EXP:.4f}")
print("=" * 72)
print("\n[계산시간]")
print(f"  모델 로드+init         : {t_load:6.2f} s (1회)")
print(f"  MASTA solve 워밍업(1점) : {warm:6.2f} s")
print(f"  MASTA solve 정상 점당   : {t_solve_avg*1000:6.1f} ms  (×{N_PTS} = {t_solve_avg*N_PTS:.1f} s)")

for bn, rows in rec_b.items():
    tA = time_it(method_A, rows)
    tB = time_it(method_B, rows)
    tC = time_it(lambda r: method_C(r)[0], rows)
    DA = method_A(rows)
    DB = method_B(rows)
    DC, cinfo = method_C(rows)
    N = cinfo["N"]
    print("\n" + "-" * 72)
    print(f"[{bn}]  (N_rev={N:.4f})")
    print(f"  집계시간  A(16281m)={tA*1e6:6.1f}µs  B(281m)={tB*1e6:6.1f}µs  C(LRD)={tC*1e6:7.1f}µs")
    print(f"  C 파라미터: C정격={cinfo['C']/1000:.1f}kN  P_eq={cinfo['P_eq']/1000:.1f}kN  ā_ISO={cinfo['a_iso']:.4f}")
    print(f"  {'방법':24} {'표본손상 D':>14} {'등가L10(Rev)':>15} {'30년손상':>13} {'피로SF':>10} {'등가수명(yr)':>13}")
    for name, D in [("A. ISO16281 per-step", DA), ("B. ISO281 per-step", DB), ("C. ISO281 등가하중(LRD)", DC)]:
        life = N / D if D > 0 else float("inf")
        d30 = D * SCALE_FACTOR
        sf = 1.0 / d30 if d30 > 0 else float("inf")
        yr = DESIGN_YEARS / d30 if d30 > 0 else float("inf")
        print(f"  {name:24} {D:14.5e} {life:15.5e} {d30:13.5e} {sf:10.1f} {yr:13.1f}")
    print(f"  A기준%:  B/A={DB/DA*100:.1f}%  C/A={DC/DA*100:.1f}%   |   ScaleFactor={SCALE_FACTOR:.0f}(30yr)")

print("\n" + "=" * 72)
print("완료")
