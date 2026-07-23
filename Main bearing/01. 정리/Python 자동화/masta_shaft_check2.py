"""샤프트 재질/강체 설정 정밀 추출 (E·section·rigid toggle)."""
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


def dump_all(o, maxlen=58):
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

print("=" * 70)
print("[Main Shaft 전체 non-callable attr]")
print("=" * 70)
dump_all(shaft)

# 재질/단면 접근 후보
print("\n[재질/단면 후보]")
for acc in ("shaft_material", "material_database_selector", "material_selector",
            "detailed_shaft", "shaft_detail", "sections", "outer_diameter",
            "length", "inner_diameter"):
    v = safe(shaft, acc)
    print(f"  shaft.{acc} = {v!r:.70}")
    if acc in ("detailed_shaft", "shaft_detail") and not isinstance(v, str):
        dump_all(v)

# 정적 로드케이스의 rigid/shaft 옵션
lc = next(c for c in assembly.design_properties.static_loads
          if getattr(c, "name", "") == "Load Case 1")
print("\n[StaticLoadCase 의 rigid/shaft/stiffness 관련 옵션]")
for n in sorted(dir(lc)):
    if n.startswith("_"):
        continue
    if any(k in n.lower() for k in ("rigid", "shaft", "stiffness", "flexible", "torsion")):
        print(f"    lc.{n} = {safe(lc, n)!r:.66}")

print("\n완료")
