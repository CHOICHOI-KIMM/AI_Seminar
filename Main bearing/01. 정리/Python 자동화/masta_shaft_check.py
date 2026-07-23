"""샤프트 강체 여부 실측: 재질·탄성계수·강체설정 확인."""
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design

MODEL = r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_260720.Masta"


def safe(o, n):
    try:
        return getattr(o, n)
    except Exception as e:
        return f"<err {str(e).splitlines()[0][:38]}>"


def dump(o, only):
    for n in sorted(dir(o)):
        if n.startswith("_"):
            continue
        if only and not any(k in n.lower() for k in only):
            continue
        v = safe(o, n)
        if callable(v):
            continue
        s = repr(v)
        if len(s) > 62:
            s = s[:59] + "..."
        print(f"    {n}: {type(v).__name__} = {s}")


design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]
shaft = list(assembly.all_parts_of_type_shaft())[0]

print("=" * 70)
print(f"[Main Shaft] rigid/material/stiffness 관련")
print("=" * 70)
dump(shaft, ("rigid", "material", "modulus", "young", "elastic", "stiffness",
             "flexible", "density", "treat", "fe", "detailed"))

print("\n[재질(material) 상세]")
mat = safe(shaft, "material")
if not isinstance(mat, str):
    dump(mat, ("modulus", "young", "elastic", "density", "poisson", "name"))
else:
    print("  material 접근 불가:", mat)

print("\n[샤프트 강체/모델링 옵션 후보 - 전체 attr 중 rigid/flex/fe]")
for n in sorted(dir(shaft)):
    if n.startswith("_"):
        continue
    if any(k in n.lower() for k in ("rigid", "flex", "fe_", "detailed", "treat", "model")):
        print(f"    {n} = {safe(shaft, n)!r:.80}")

# 설계 레벨 / 로드케이스 레벨 강체 옵션
print("\n[design/assembly 레벨 rigid 옵션]")
for obj, nm in [(design, "design"), (assembly, "assembly"),
                (assembly.design_properties, "design_properties")]:
    for n in sorted(dir(obj)):
        if n.startswith("_"):
            continue
        if any(k in n.lower() for k in ("rigid", "treat_shaft", "shaft_as")):
            print(f"    {nm}.{n} = {safe(obj, n)!r:.70}")

print("\n완료")
