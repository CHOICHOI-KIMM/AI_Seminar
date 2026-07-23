"""유연체 모델 검증: 샤프트 E + 미스얼라인/수명비(281 vs 16281) — 강체와 대비용."""
import math
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

MODEL = r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_유연체_260720.Masta"
DLC   = r"D:\AI\AI_Seminar\Main bearing\02. 자료\260714 유니슨 피로하중\DLC1.2-c-s1.$150"
RPM2RADS = 2 * math.pi / 60
POINTS = [0, 3000]


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


design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]
pl = list(assembly.all_parts_of_type_point_load())[0]
bearings = list(assembly.all_parts_of_type_bearing())
ipl = next(p for p in assembly.all_parts_of_type_power_load() if "input" in str(p).lower())
lc = next(c for c in assembly.design_properties.static_loads if getattr(c, "name", "") == "Load Case 1")
shaft = list(assembly.all_parts_of_type_shaft())[0]

# 샤프트 재질 E (강체 아님 확인)
md = g(shaft, "active_definition.shaft_material.material_details")
E = g(md, "modulus_of_elasticity")
print(f"[샤프트] 재질={g(md,'material_name')}  E={E:.4e} Pa = {E/1e9:.4g} GPa  (강재 207GPa 대비 {E/2.07e11:.2f}x)")
print(f"         is_replaced_by_fe={g(shaft,'is_replaced_by_fe')}")

data = parse(DLC)
for idx in POINTS:
    rec = data[idx]
    p = lc.inputs_for_point_load(pl)
    p.force_x.force = -rec["Fz"] * 1e3; p.force_y.force = rec["Fy"] * 1e3; p.axial_load.force = rec["Fx"] * 1e3
    p.moment_x.moment = -rec["Mz"] * 1e3; p.moment_y.moment = rec["My"] * 1e3
    lc.inputs_for_power_load(ipl).speed = rec["rpm"] * RPM2RADS
    sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION); sd.perform_analysis()
    print(f"\n=== point {idx} (rpm {rec['rpm']:.2f}) ===")
    for b in bearings:
        d = g(sd.results_for(b), "component_detailed_analysis")
        L10 = g(d, "iso2812007.basic_rating_life_cycles")
        L10m = g(d, "iso2812007.modified_rating_life_cycles")
        L10r = g(d, "isots162812008.basic_reference_rating_life_cycles")
        L10mr = g(d, "isots162812008.modified_reference_rating_life_cycles")
        nel = g(d, "number_of_elements_in_contact")
        mis = g(d, "relative_misalignment")
        print(f"  [{b}] 접촉요소={nel}  미스얼라인={mis:.3e} rad")
        print(f"     L10r/L10(기본)={L10r/L10:.4f}   L10mr/L10m(수정)={L10mr/L10m:.4f}   (>1: 16281 수명↑=A손상↓)")
print("\n완료")
