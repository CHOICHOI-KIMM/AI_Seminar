"""Rigid Material 1 의 탄성계수 E 추출 + 강재 대비 정량화."""
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design

MODEL = r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_260720.Masta"
E_STEEL = 2.06e11  # 강재 기준 206 GPa


def safe(o, n):
    try:
        return getattr(o, n)
    except Exception as e:
        return f"<ERR {str(e).splitlines()[0][:34]}>"


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
ad = shaft.active_definition
sm = ad.shaft_material
print("shaft_material type:", type(sm).__name__, "| 이름:", safe(ad, "material"))

print("\n[shaft_material 전체 속성]")
dump_all(sm)

# E 직접 시도 (여러 이름)
print("\n[E 접근자 직접 시도]")
E = None
for acc in ("modulus_of_elasticity", "youngs_modulus", "young_modulus",
            "e_youngs_modulus", "elastic_modulus", "modulus"):
    v = safe(sm, acc)
    print(f"  sm.{acc} = {v!r}")
    if isinstance(v, (int, float)):
        E = v

# 내부 material 객체가 따로 있으면 그쪽도
for macc in ("material", "material_for_reports", "base_material"):
    m = safe(sm, macc)
    if not isinstance(m, str) and m is not None and not callable(m):
        print(f"\n[sm.{macc} = {type(m).__name__}]")
        dump_all(m)

if isinstance(E, (int, float)):
    print(f"\n=== 정량화 ===")
    print(f"  E(Rigid Material 1) = {E:.4e} Pa = {E/1e9:.1f} GPa")
    print(f"  강재 E = {E_STEEL:.3e} Pa = {E_STEEL/1e9:.0f} GPa")
    print(f"  배율 = {E/E_STEEL:.1f}x  (>>1이면 사실상 강체)")
else:
    print("\n(E 미검출 — 위 전체 덤프에서 modulus 관련 확인 필요)")
print("\n완료")
