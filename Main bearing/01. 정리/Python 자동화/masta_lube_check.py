"""윤활유 설정 확인 + 1점 해석으로 점도비/aISO/유막 영향 점검."""
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
        return f"<err {str(e).splitlines()[0][:45]}>"


def dump(obj, indent="  ", only=None):
    for name in sorted(dir(obj)):
        if name.startswith("_"):
            continue
        v = safe(obj, name)
        if only and not any(k in name.lower() for k in only):
            continue
        if isinstance(v, str) and v.startswith("<err"):
            if only:
                print(f"{indent}{name}: {v}")
            continue
        cal = callable(v)
        s = "" if cal else repr(v)
        if len(s) > 70:
            s = s[:67] + "..."
        print(f"{indent}{name}: {type(v).__name__} {'()' if cal else '= ' + s}")


design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]
pl = list(assembly.all_parts_of_type_point_load())[0]
bearings = list(assembly.all_parts_of_type_bearing())
ipl = next(p for p in assembly.all_parts_of_type_power_load() if "input" in str(p).lower())
cases = {getattr(c, "name", "?"): c for c in assembly.design_properties.static_loads}
lc = cases.get("Load Case 1") or list(assembly.design_properties.static_loads)[0]
print("모델 로드 OK | 드라이버:", getattr(lc, "name", "?"),
      "| 베어링:", [str(b) for b in bearings], "| 속도입력:", ipl)

# DLC 첫 점 (검증용) 하중
rec = dict(rpm=5.09167, Fx=3016.56, Fy=138.153, Fz=-4929.76, My=-13130.1, Mz=4670.50)
# (파일럿 index1 근사값 — 정확값은 파일에서, 여기선 대략)
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
    print("\n" + "=" * 70)
    print(f"[{b}] 윤활/점도 관련 결과")
    print("-" * 70)
    dump(d, only=("visco", "lambda", "film", "lubric", "oil", "temperature",
                  "contamination", "density"))
    print("  -- 수명(윤활 영향) --")
    for path, lbl in [("iso2812007.life_modification_factor_for_systems_approach", "aISO"),
                      ("iso2812007.viscosity_ratio", "viscosity_ratio"),
                      ("iso2812007.contamination_factor", "contamination_factor"),
                      ("iso2812007.basic_rating_life_time", "L10h_basic"),
                      ("iso2812007.modified_rating_life_time", "L10mh_mod")]:
        cur = d
        ok = True
        for pp in path.split("."):
            cur = safe(cur, pp)
            if isinstance(cur, str) and cur.startswith("<err"):
                ok = False; break
        print(f"    {lbl} = {cur if ok else 'n/a'}")

print("\n" + "=" * 70)
print("LubricationDetail (첫 베어링) 상세")
print("-" * 70)
d0 = bearings[0]
res0 = sd.results_for(d0)
lub = safe(res0.component_detailed_analysis, "lubrication")
if not isinstance(lub, str):
    dump(lub, only=("name", "iso", "grade", "viscosity", "density", "oil",
                    "type", "additive", "temperature", "kinematic"))
print("완료")
