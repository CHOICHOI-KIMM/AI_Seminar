"""방법 C-1용 카탈로그 상수 추출: C, C_u, e, 동적 X/Y, d_m 등 (베어링 상수)."""
import math
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

MODEL = r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_260720.Masta"
RPM2RADS = 2 * math.pi / 60


def safe(o, n):
    try:
        return getattr(o, n)
    except Exception as e:
        return f"<err {str(e).splitlines()[0][:40]}>"


def dump(obj, only, indent="  "):
    for name in sorted(dir(obj)):
        if name.startswith("_"):
            continue
        if not any(k in name.lower() for k in only):
            continue
        v = safe(obj, name)
        if callable(v):
            print(f"{indent}{name}()")
            continue
        s = repr(v)
        if len(s) > 66:
            s = s[:63] + "..."
        print(f"{indent}{name}: {type(v).__name__} = {s}")


design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]
pl = list(assembly.all_parts_of_type_point_load())[0]
bearings = list(assembly.all_parts_of_type_bearing())
ipl = next(p for p in assembly.all_parts_of_type_power_load() if "input" in str(p).lower())
lc = next(c for c in assembly.design_properties.static_loads
          if getattr(c, "name", "") == "Load Case 1")

# 대표 하중 1점(축하중 있는 조건)으로 해석 → detail에서 X/Y/e 확인
p = lc.inputs_for_point_load(pl)
p.force_x.force = 4965600; p.force_y.force = 126450; p.axial_load.force = 3017070
p.moment_x.moment = -4814750; p.moment_y.moment = -13666500
lc.inputs_for_power_load(ipl).speed = 5.09 * RPM2RADS
sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
sd.perform_analysis()

KEYS_BEARING = ("rating", "fatigue", "load_limit", "diameter", "angle", "row",
                "number_of", "dynamic", "static", "cu", "pitch", "mean", "element")
KEYS_ISO = ("factor", "e_lim", "radial", "axial", "equivalent", "load_rating",
            "fatigue", "limit", "load_ratio")

for b in bearings:
    print("\n" + "=" * 72)
    print(f"[{b}]  베어링 파트(TaperRollerBearing) 상수")
    print("-" * 72)
    dump(b, KEYS_BEARING)
    d = sd.results_for(b).component_detailed_analysis
    print(f"\n  -- detail 직속 (정격/한계/기하) --")
    dump(d, ("rating", "fatigue", "limit", "diameter", "pitch", "mean",
             "dynamic_load", "static_load", "number_of_rows", "contact_angle",
             "basic_dynamic", "basic_static"))
    print(f"\n  -- detail.iso2812007 (X/Y/e/등가하중/정격) --")
    iso = safe(d, "iso2812007")
    if not isinstance(iso, str):
        dump(iso, KEYS_ISO)

print("\n완료")
