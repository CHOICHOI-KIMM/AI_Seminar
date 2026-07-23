"""B>A 메커니즘 분석: 281 vs 16281 수명차 원인 지표 추출 (md 미반영, 콘솔 분석용)."""
import math
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

MODEL = r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_260720.Masta"
DLC   = r"D:\AI\AI_Seminar\Main bearing\02. 자료\260714 유니슨 피로하중\DLC1.2-c-s1.$150"
RPM2RADS = 2 * math.pi / 60
POINTS = [0, 3000]   # 저부하/고부하 대표


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
            rows.append(dict(t=v[0], rpm=v[1], Mx=v[2], My=v[3], Mz=v[4], Fx=v[5], Fy=v[6], Fz=v[7]))
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


def dump(o, only):
    for n in sorted(dir(o)):
        if n.startswith("_"):
            continue
        if not any(k in n.lower() for k in only):
            continue
        try:
            v = getattr(o, n)
        except Exception:
            continue
        if callable(v):
            continue
        s = repr(v)
        if len(s) > 60:
            s = s[:57] + "..."
        print(f"      {n} = {s}")


design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]
pl = list(assembly.all_parts_of_type_point_load())[0]
bearings = list(assembly.all_parts_of_type_bearing())
ipl = next(p for p in assembly.all_parts_of_type_power_load() if "input" in str(p).lower())
lc = next(c for c in assembly.design_properties.static_loads if getattr(c, "name", "") == "Load Case 1")
data = parse(DLC)

for idx in POINTS:
    rec = data[idx]
    p = lc.inputs_for_point_load(pl)
    p.force_x.force = -rec["Fz"] * 1e3; p.force_y.force = rec["Fy"] * 1e3; p.axial_load.force = rec["Fx"] * 1e3
    p.moment_x.moment = -rec["Mz"] * 1e3; p.moment_y.moment = rec["My"] * 1e3
    lc.inputs_for_power_load(ipl).speed = rec["rpm"] * RPM2RADS
    sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION); sd.perform_analysis()
    print("\n" + "=" * 74)
    print(f"[point {idx}] rpm={rec['rpm']:.2f}")
    for b in bearings:
        d = g(sd.results_for(b), "component_detailed_analysis")
        L10 = g(d, "iso2812007.basic_rating_life_cycles")
        L10m = g(d, "iso2812007.modified_rating_life_cycles")
        L10r = g(d, "isots162812008.basic_reference_rating_life_cycles")
        L10mr = g(d, "isots162812008.modified_reference_rating_life_cycles")
        nel = g(d, "number_of_elements_in_contact")
        mis = g(d, "relative_misalignment")
        sin_ = g(d, "maximum_normal_stress_inner")
        sout = g(d, "maximum_normal_stress_outer")
        print(f"\n  [{b}]  접촉요소={nel}  미스얼라인={mis}")
        print(f"    L10(281기본)   = {L10:.4e}")
        print(f"    L10r(16281기본)= {L10r:.4e}   → L10r/L10 = {L10r/L10:.4f}  (>1: 16281이 유리=수명↑)")
        print(f"    L10m(281수정)  = {L10m:.4e}")
        print(f"    L10mr(16281수정)={L10mr:.4e}   → L10mr/L10m = {L10mr/L10m:.4f}")
        print(f"    최대응력 내/외 = {sin_/1e6:.0f} / {sout/1e6:.0f} MPa")
        print(f"    -- 16281 detail: 미스얼라인/엣지/롤/응력/하중분포 관련 --")
        dump(g(d, "isots162812008"), ("misalign", "edge", "lamina", "row",
                                      "stress", "load", "truncation", "tilt", "maximum"))
        print(f"    -- detail rows/미스얼라인/틸트 --")
        dump(d, ("misalign", "tilt", "truncation", "edge", "row", "number_of_elements"))

print("\n완료")
