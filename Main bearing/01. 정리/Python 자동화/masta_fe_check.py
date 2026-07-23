"""FE 모델에서 샤프트 DIN743 SF가 안 나오는 원인 진단."""
import math
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

MODEL = r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_유연체_FE_260721.Masta"
RPM2RADS = 2 * math.pi / 60


def safe(o, n):
    try:
        return getattr(o, n)
    except Exception as e:
        return f"<ERR {str(e).splitlines()[0][:30]}>"


design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]
shaft = list(assembly.all_parts_of_type_shaft())[0]
pl = list(assembly.all_parts_of_type_point_load())[0]
ipl = next(p for p in assembly.all_parts_of_type_power_load() if "input" in str(p).lower())
lc = next(c for c in assembly.design_properties.static_loads if getattr(c, "name", "") == "Load Case 1")

print("[샤프트 상태]")
print("  is_replaced_by_fe      =", safe(shaft, "is_replaced_by_fe"))
print("  fe_substructure_replacing_this =", safe(shaft, "fe_substructure_replacing_this"))
md = safe(shaft, "active_definition")
mat = safe(md, "shaft_material")
print("  재질 =", safe(md, "material"))

# FE 서브스트럭처 존재 여부
try:
    fes = list(assembly.all_parts_of_type_fe_part())
    print("  FE 파트:", [str(x) for x in fes])
except Exception as e:
    print("  FE 파트 조회 실패:", e)

# 해석 1점 후 샤프트 결과 확인
p = lc.inputs_for_point_load(pl)
p.force_x.force = 5e6; p.force_y.force = 1e5; p.axial_load.force = 3e6
p.moment_x.moment = -5e6; p.moment_y.moment = -1.3e7
pll = lc.inputs_for_power_load(ipl); pll.speed = 5.0 * RPM2RADS; pll.torque = 2.4e7
sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION); sd.perform_analysis()

sres = sd.results_for(shaft)
print("\n[ShaftSystemDeflection]")
print("  type =", type(sres).__name__)
print("  shaft_rating_method =", safe(sres, "shaft_rating_method"))
grp = safe(sres, "safety_factors")
print("  safety_factors =", type(grp).__name__ if not isinstance(grp, str) else grp)
try:
    items = list(grp.items)
    print(f"  items: {len(items)}개")
    for it in items:
        print("    -", safe(it, "description"), "| SF =", safe(it, "safety_factor"))
except Exception as e:
    print("  items 조회 실패:", e)
print("  worst_inf =", safe(sres, "shaft_section_end_with_worst_fatigue_safety_factor_for_infinite_life"))
try:
    print("  shaft_section_results:", len(list(sres.shaft_section_results)), "개")
except Exception as e:
    print("  section_results 실패:", e)
print("\n완료")
