"""SafetyFactorItem 식별자 확인 → 피로/정적 구분."""
import math
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

MODEL = r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_260720.Masta"
DLC   = r"D:\AI\AI_Seminar\Main bearing\02. 자료\260714 유니슨 피로하중\DLC1.2-c-s1.$150"
RPM2RADS = 2 * math.pi / 60


def safe(o, n):
    try:
        return getattr(o, n)
    except Exception as e:
        return f"<ERR {str(e).splitlines()[0][:26]}>"


def parse1(path, idx):
    with open(path, "r", encoding="latin-1") as f:
        rows = []
        for ln in f.readlines()[4:]:
            p = ln.split()
            if len(p) >= 8:
                try:
                    rows.append([float(x) for x in p[:8]])
                except ValueError:
                    pass
    v = rows[idx]
    return dict(rpm=v[1], Mx=v[2], My=v[3], Mz=v[4], Fx=v[5], Fy=v[6], Fz=v[7])


design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]
pl = list(assembly.all_parts_of_type_point_load())[0]
shaft = list(assembly.all_parts_of_type_shaft())[0]
ipl = next(p for p in assembly.all_parts_of_type_power_load() if "input" in str(p).lower())
lc = next(c for c in assembly.design_properties.static_loads if getattr(c, "name", "") == "Load Case 1")

rec = parse1(DLC, 3000)
p = lc.inputs_for_point_load(pl)
p.force_x.force = -rec["Fz"] * 1e3; p.force_y.force = rec["Fy"] * 1e3; p.axial_load.force = rec["Fx"] * 1e3
p.moment_x.moment = -rec["Mz"] * 1e3; p.moment_y.moment = rec["My"] * 1e3
pll = lc.inputs_for_power_load(ipl); pll.speed = rec["rpm"] * RPM2RADS; pll.torque = rec["Mx"] * 1e3
sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION); sd.perform_analysis()
sres = sd.results_for(shaft)

print("[SafetyFactorItem 전체 attr + str]")
for i, it in enumerate(list(sres.safety_factors.items)):
    print(f"\n item[{i}] str={str(it)!r}")
    for n in sorted(dir(it)):
        if n.startswith("_"):
            continue
        v = safe(it, n)
        if not callable(v):
            print(f"    {n} = {v!r:.50}")

print("\n[ShaftSectionSystemDeflection[0] — fatigue/safety/infinite/reliab]")
sec = list(sres.shaft_section_results)[0]
print("  type:", type(sec).__name__)
for n in sorted(dir(sec)):
    if n.startswith("_"):
        continue
    if any(k in n.lower() for k in ("fatigue", "safety", "infinite", "reliab", "din", "static")):
        print(f"    {n} = {safe(sec, n)!r:.50}")
print("\n완료")
