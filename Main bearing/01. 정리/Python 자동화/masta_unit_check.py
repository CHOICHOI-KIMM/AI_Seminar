"""API 입력단위 검증 — 기존 케이스 point-load 값을 raw로 읽어 GUI와 대조."""
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design

MODEL = r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_260720.Masta"

design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]
pl = list(assembly.all_parts_of_type_point_load())[0]
cases = {getattr(c, "name", "?"): c for c in assembly.design_properties.static_loads}


def rd(comp, attr):
    try:
        return getattr(comp, attr)
    except Exception as e:
        return f"ERR:{str(e).splitlines()[0][:20]}"


for name in ("Mx_max", "Load Case 1"):
    lc = cases.get(name)
    if lc is None:
        print(f"[{name}] 없음"); continue
    p = lc.inputs_for_point_load(pl)
    fx = rd(p.force_x, "force"); fy = rd(p.force_y, "force")
    ax = rd(getattr(p, "axial_load", None), "force")
    mx = rd(p.moment_x, "moment"); my = rd(p.moment_y, "moment")
    print(f"\n=== [{name}] point-load API raw 값 ===")
    print(f"  force_x  (.force)  = {fx}")
    print(f"  force_y  (.force)  = {fy}")
    print(f"  axial    (.force)  = {ax}")
    print(f"  moment_x (.moment) = {mx}")
    print(f"  moment_y (.moment) = {my}")

    def num(v):
        return v if isinstance(v, (int, float)) else None
    print("  ── 해석 ──")
    for lbl, v in [("force_x", fx), ("force_y", fy), ("axial", ax)]:
        n = num(v)
        if n is not None:
            print(f"    {lbl}: raw={n:,.1f}  →  N이면 {n/1000:,.2f} kN / kN이면 {n:,.0f} kN")
    for lbl, v in [("moment_x", mx), ("moment_y", my)]:
        n = num(v)
        if n is not None:
            print(f"    {lbl}: raw={n:,.1f}  →  Nm이면 {n/1000:,.1f} kNm / kNm이면 {n:,.0f} kNm")

print("\n완료")
