"""로드 입력 v3 — ForceInputComponent 세터, 16케이스 매핑 역산, 속도 추적."""
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design

MODEL = r"D:\AI\AI_Seminar\Main bearing\02. 자료\260720 검증용 자료_롤러 프로파일\26MW_메인베어링_기본설계_v1.2_샤프트 두께,형상 2안_강체,롤러프로파일만_260720.Masta"


def hr(t=""):
    print("\n" + "=" * 70)
    if t:
        print(t); print("-" * 70)


def safe(obj, name):
    try:
        return getattr(obj, name)
    except Exception as e:
        return f"<err {str(e).splitlines()[0][:50]}>"


def dump(obj, indent="  ", only=None):
    for name in sorted(dir(obj)):
        if name.startswith("_"):
            continue
        v = safe(obj, name)
        if only and not any(k in name.lower() for k in only):
            continue
        if isinstance(v, str) and v.startswith("<err"):
            print(f"{indent}{name}: {v}"); continue
        cal = callable(v)
        tn = type(v).__name__
        s = "" if cal else repr(v)
        if len(s) > 72:
            s = s[:69] + "..."
        print(f"{indent}{name}: {tn} {'()' if cal else '= ' + s}")


design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]
point_load = list(assembly.all_parts_of_type_point_load())[0]
cases = list(assembly.design_properties.static_loads)
lc0 = cases[0]

hr("J. ForceInputComponent (force_x) 전체 — 스칼라 세팅 방법 찾기")
pli0 = lc0.inputs_for_point_load(point_load)
fx = pli0.force_x
print("  type:", type(fx).__name__)
dump(fx)

hr("K. MomentInputComponent (moment_x) 전체")
mx = pli0.moment_x
print("  type:", type(mx).__name__)
dump(mx)

hr("L. ForceSpecification enum 옵션 + 현재값")
try:
    fs = pli0.force_specification()  # method? or property
    print("  force_specification():", fs)
except Exception as e:
    print("  force_specification() 실패:", str(e).splitlines()[0])
try:
    FS = type(pli0).ForceSpecification
    print("  ForceSpecification members:", [m for m in dir(FS) if m.isupper()])
except Exception as e:
    print("  enum 실패:", str(e).splitlines()[0])

def read_val(comp):
    """ForceInputComponent 의 대표 스칼라를 여러 접근자로 시도."""
    for acc in ("constant_value", "magnitude", "value", "mean_value",
                "constant", "specified_value"):
        v = safe(comp, acc)
        if not (isinstance(v, str) and v.startswith("<err")):
            return f"{acc}={v}"
    return "?"

hr("M. 16개 케이스별 point-load 성분값 (매핑 역산)")
print(f"  {'case':10} {'force_x':>16} {'force_y':>16} {'axial':>16} "
      f"{'moment_x':>16} {'moment_y':>16}")
for lc in cases:
    pli = lc.inputs_for_point_load(point_load)
    vals = [read_val(pli.force_x), read_val(pli.force_y),
            read_val(safe(pli, "axial_load")),
            read_val(pli.moment_x), read_val(pli.moment_y)]
    print(f"  {getattr(lc,'name','?'):10} " + " ".join(f"{v:>16}" for v in vals))

hr("N. 속도 추적 — load case의 cycle/speed/duration/time 관련")
dump(lc0, only=("cycle", "speed", "duration", "time", "rpm", "rev"))
print("  -- design 레벨 --")
dump(design, only=("speed", "cycle", "rpm", "rev"))

hr("완료")
