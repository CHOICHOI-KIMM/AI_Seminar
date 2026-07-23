"""6001점 전량 소요시간 추정 — 연속 30점 실측 후 외삽."""
import math
import time
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

MODEL = r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_260720.Masta"
DLC   = r"D:\AI\AI_Seminar\Main bearing\02. 자료\260714 유니슨 피로하중\DLC1.2-c-s1.$150"
N     = 30
RPM2RADS = 2 * math.pi / 60


def parse(path):
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
    return rows


t0 = time.perf_counter()
design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]
pl = list(assembly.all_parts_of_type_point_load())[0]
bearings = list(assembly.all_parts_of_type_bearing())
ipl = next(p for p in assembly.all_parts_of_type_power_load() if "input" in str(p).lower())
cases = {getattr(c, "name", "?"): c for c in assembly.design_properties.static_loads}
lc = cases.get("Load Case 1") or list(assembly.design_properties.static_loads)[0]
t_load = time.perf_counter() - t0
print(f"모델 로드+init: {t_load:.1f}s")

data = parse(DLC)
print(f"파싱 {len(data)}점, 연속 {N}점 측정 시작")

per = []
for i in range(N):
    rec = data[i]
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
    for b in bearings:
        res = sd.results_for(b)
        d = res.component_detailed_analysis
        _ = getattr(d, "iso2812007").basic_rating_life_time  # 실제 추출 부하 반영
    dt = time.perf_counter() - ts
    per.append(dt)
    print(f"  [{i:2d}] {dt:.2f}s")

# 첫 점은 워밍업일 수 있어 별도 집계
warm = per[0]
steady = per[1:]
avg = sum(steady) / len(steady)
mn, mx = min(steady), max(steady)
total_6001 = avg * 6001

print("\n" + "=" * 60)
print(f"1점(워밍업, idx0): {warm:.2f}s")
print(f"정상 {len(steady)}점 평균: {avg:.2f}s (min {mn:.2f} / max {mx:.2f})")
print(f"모델 로드 1회: {t_load:.1f}s")
print("-" * 60)
print(f"6001점 예상: {total_6001:.0f}s = {total_6001/60:.1f}분 = {total_6001/3600:.2f}시간")
print(f"  (+로드 {t_load:.0f}s 포함 총 {(total_6001+t_load)/3600:.2f}시간)")
print("완료")
