"""active_definition(SimpleShaftDefinition)에서 재질 E·단면·강체여부 확정."""
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design

MODEL = r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_260720.Masta"


def safe(o, n):
    try:
        return getattr(o, n)
    except Exception as e:
        return f"<ERR {str(e).splitlines()[0][:34]}>"


def dump(o, only=None, maxlen=60):
    for n in sorted(dir(o)):
        if n.startswith("_"):
            continue
        if only and not any(k in n.lower() for k in only):
            continue
        v = safe(o, n)
        if callable(v):
            continue
        s = repr(v)
        if len(s) > maxlen:
            s = s[:maxlen] + "..."
        print(f"    {n} = {s}")


design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]
shaft = list(assembly.all_parts_of_type_shaft())[0]
ad = shaft.active_definition
print("active_definition type:", type(ad).__name__)

print("\n[재질/E/강체/단면 관련]")
dump(ad, ("material", "modulus", "young", "elastic", "rigid", "density",
          "poisson", "diameter", "section", "bore", "outer", "inner",
          "treat", "flexible", "stiff"))

print("\n[material 객체 상세]")
for macc in ("material", "shaft_material"):
    m = safe(ad, macc)
    if not isinstance(m, str) and m is not None:
        print(f"  {macc}:", type(m).__name__)
        dump(m, ("modulus", "young", "elastic", "density", "poisson", "name",
                 "hardness", "tensile", "yield"))
        break

print("\n[단면(sections) — 외경/내경]")
sec = safe(ad, "sections")
if not isinstance(sec, str):
    try:
        for i, s in enumerate(list(sec)[:6]):
            od = safe(s, "outer_diameter"); idm = safe(s, "inner_diameter")
            print(f"  section[{i}]: OD={od} ID={idm}  ({type(s).__name__})")
    except Exception as e:
        print("  sections 열거 실패:", e)

print("\n완료")
