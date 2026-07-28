"""TRB 폭·축방향 기하 실측 (도면 기호 B, C, T, a, 2beta 대응)"""
import os, sys, math
HERE = os.path.dirname(os.path.abspath(__file__)); ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)
import masta_clr_legacy  # noqa
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design

M = {
 "v1.3": (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
   r"\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_유연체_FE_온도_50도_260721.Masta"),
 "v1.4": (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
   r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta"),
}
KEYS = ["bore","outer_diameter","width","assembled_width","inner_ring_width","outer_ring_width",
        "cage_width","pitch_circle_diameter","element_diameter","roller_length",
        "effective_roller_length","number_of_elements","contact_angle","cone_angle","cup_angle",
        "element_taper_angle","element_radius","element_offset","outer_ring_offset",
        "effective_centre_from_front_face","mean_inner_race_diameter","mean_outer_race_diameter",
        "roller_end_radius","width_setting_inner_and_outer_ring_width","mass",
        "basic_dynamic_load_rating","fatigue_load_limit"]

def safe(o,n):
    try: return getattr(o,n)
    except Exception: return None
def sc(o,n):
    v=safe(o,n)
    if isinstance(v,(int,float)) and not isinstance(v,bool): return float(v)
    for a in ("value","wrapped"):
        w=safe(v,a)
        if isinstance(w,(int,float)) and not isinstance(w,bool): return float(w)
    return None

out={}
for tag,p in M.items():
    d=Design.load(p); asm=d.all_parts_of_type_root_assembly()[0]
    b=list(asm.all_parts_of_type_bearing())[0]; det=safe(b,"detail")
    out[tag]={k:sc(det,k) for k in KEYS}

ANG={"contact_angle","cone_angle","cup_angle","element_taper_angle"}
print(f"{'항목':40} {'v1.3':>16} {'v1.4':>16}")
print("-"*76)
for k in KEYS:
    a,bb=out["v1.3"][k],out["v1.4"][k]
    if k in ANG:
        f=lambda x: "None" if x is None else f"{math.degrees(x):.4f} deg"
    else:
        f=lambda x: "None" if x is None else f"{x:.6g}"
    print(f"{k:40} {f(a):>16} {f(bb):>16}")

print("\n[파생 검산]")
for tag in M:
    o=out[tag]
    pcd,dwe,al,d0,D0=o["pitch_circle_diameter"],o["element_diameter"],o["contact_angle"],o["bore"],o["outer_diameter"]
    mi,mo=o["mean_inner_race_diameter"],o["mean_outer_race_diameter"]
    print(f"  [{tag}]")
    print(f"    PCD - Dwe*cos(a)        = {pcd-dwe*math.cos(al):.6f}   (mean_inner 실측 {mi})")
    print(f"    PCD + Dwe*cos(a)        = {pcd+dwe*math.cos(al):.6f}   (mean_outer 실측 {mo})")
    print(f"    t_i = (mean_inner-d)/2  = {(mi-d0)/2*1000:.3f} mm   t_i/PCD = {(mi-d0)/2/pcd:.5f}   t_i/Dwe = {(mi-d0)/2/dwe:.4f}")
    print(f"    t_o = (D-mean_outer)/2  = {(D0-mo)/2*1000:.3f} mm   t_o/PCD = {(D0-mo)/2/pcd:.5f}   t_o/Dwe = {(D0-mo)/2/dwe:.4f}")
    print(f"    T - L_we                = {(o['width']-o['effective_roller_length'])*1000:.3f} mm")
    print(f"    B + C - T               = {(o['inner_ring_width']+o['outer_ring_width']-o['width'])*1000:.3f} mm")
    print(f"    B/T = {o['inner_ring_width']/o['width']:.4f}   C/T = {o['outer_ring_width']/o['width']:.4f}")
    print(f"    B/L_we = {o['inner_ring_width']/o['effective_roller_length']:.4f}   C/L_we = {o['outer_ring_width']/o['effective_roller_length']:.4f}")
print("완료")
