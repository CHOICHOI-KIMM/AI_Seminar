"""material_details(ShaftMaterial)에서 E 추출 + 정량화."""
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design

MODEL = r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_260720.Masta"
E_STEEL = 2.06e11


def safe(o, n):
    try:
        return getattr(o, n)
    except Exception as e:
        return f"<ERR {str(e).splitlines()[0][:30]}>"


def dump_all(o, maxlen=64):
    for n in sorted(dir(o)):
        if n.startswith("_"):
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
md = shaft.active_definition.shaft_material.material_details
print("material_details type:", type(md).__name__)
print("\n[material_details 전체]")
dump_all(md)

E = None
for acc in ("modulus_of_elasticity", "youngs_modulus", "young_modulus",
            "elastic_modulus", "modulus", "e"):
    v = safe(md, acc)
    if isinstance(v, (int, float)):
        E = v
        print(f"\n  → E({acc}) = {v}")
        break

print("\n=== 정량화 ===")
if isinstance(E, (int, float)):
    print(f"  E(Rigid Material 1) = {E:.4e} Pa = {E/1e9:.3g} GPa")
    print(f"  강재 E = 206 GPa")
    print(f"  배율 = {E/E_STEEL:.1f}x")
    dens = safe(md, "density")
    pois = safe(md, "poissons_ratio")
    print(f"  density={dens}, poisson={pois}")
else:
    print("  E 미검출 — 위 덤프 확인")
print("\n완료")
