"""축방향 배치 실측 — B·C·T 와 콘/컵 배면 살두께 관계"""
import os, sys, math
HERE=os.path.dirname(os.path.abspath(__file__)); RES=os.path.dirname(HERE); ROOT=os.path.dirname(RES)
sys.path.insert(0,ROOT)
import masta_clr_legacy  # noqa
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.bearings import RollerBearingProfileTypes as RP
M={"v1.3":(r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
   r"\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_유연체_FE_온도_50도_260721.Masta"),
   "v1.4":(r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
   r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")}
KEYS=["width","assembled_width","inner_ring_width","outer_ring_width","cage_width",
      "roller_length","effective_roller_length","element_diameter","element_offset",
      "outer_ring_offset","effective_centre_from_front_face","contact_angle","cone_angle",
      "element_taper_angle","offset_of_contact_on_inner_race_at_nominal_contact_angle",
      "offset_of_contact_on_outer_race_at_nominal_contact_angle","element_radius",
      "inner_ring_back_face_corner_radius","inner_ring_front_face_corner_radius",
      "outer_ring_back_face_corner_radius","outer_ring_front_face_corner_radius"]
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
V={}
for tag,p in M.items():
    d=Design.load(p); asm=d.all_parts_of_type_root_assembly()[0]
    det=list(asm.all_parts_of_type_bearing())[0].detail
    V[tag]={k:sc(det,k) for k in KEYS}
ANG={"contact_angle","cone_angle","element_taper_angle"}
print(f"{'항목':46} {'v1.3':>14} {'v1.4':>14}")
print("-"*78)
for k in KEYS:
    a,b=V["v1.3"][k],V["v1.4"][k]
    f=lambda x: "None" if x is None else (f"{math.degrees(x):14.4f}" if k in ANG else f"{x*1e3:14.4f}")
    print(f"{k:46} {f(a)} {f(b)}")
print("\n=== 파생 (mm) ===")
for tag in ("v1.3","v1.4"):
    v=V[tag]
    T,B,C=v["width"]*1e3,v["inner_ring_width"]*1e3,v["outer_ring_width"]*1e3
    Lwe=v["effective_roller_length"]*1e3; Dwe=v["element_diameter"]*1e3
    cone=v["cone_angle"]; Lax=Lwe*math.cos(cone)
    eo=v["element_offset"]*1e3; oro=v["outer_ring_offset"]*1e3
    print(f"\n[{tag}]  T={T:.2f} B={B:.2f} C={C:.2f} L_we={Lwe:.2f} D_we={Dwe:.2f}")
    print(f"   롤러 축투영 L_ax = L_we*cos(cone {math.degrees(cone):.3f}deg) = {Lax:.3f}")
    print(f"   T-B={T-B:.3f}  T-C={T-C:.3f}  B+C-T={B+C-T:.3f}")
    print(f"   B-L_ax={B-Lax:.3f}  C-L_ax={C-Lax:.3f}  T-L_ax={T-Lax:.3f}")
    print(f"   element_offset={eo:.4f}  outer_ring_offset={oro:.4f}")
    print(f"   B/T={B/T:.5f} C/T={C/T:.5f}  B/L_we={B/Lwe:.5f} C/L_we={C/Lwe:.5f} T/L_we={T/Lwe:.5f}")
    print(f"   (T-L_we)={T-Lwe:.3f}  (B-L_we)={B-Lwe:.3f}  (C-L_we)={C-Lwe:.3f}")
    print(f"   비 (T-L_we)/D_we={(T-Lwe)/Dwe:.5f}  (B-L_we)/D_we={(B-Lwe)/Dwe:.5f}"
          f"  (C-L_we)/D_we={(C-Lwe)/Dwe:.5f}")
print("완료")
