"""케이지 포함 여부 + 제원 연동 규칙 실측"""
import os, sys, math
HERE=os.path.dirname(os.path.abspath(__file__)); RES=os.path.dirname(HERE); ROOT=os.path.dirname(RES)
sys.path.insert(0,ROOT)
import masta_clr_legacy  # noqa
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.bearings import RollerBearingProfileTypes as RP
MODEL=(r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
       r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
       r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
COSA=math.cos(math.radians(19.0))
def safe(o,n):
    try: return getattr(o,n)
    except Exception as e: return f"ERR"
def sc(o,n):
    v=safe(o,n)
    if isinstance(v,(int,float)) and not isinstance(v,bool): return float(v)
    for a in ("value","wrapped"):
        w=safe(v,a)
        if isinstance(w,(int,float)) and not isinstance(w,bool): return float(w)
    return None
def cage(det):
    return dict(R=sc(det,"cage_pitch_radius"), W=sc(det,"cage_width"),
                th=sc(det,"cage_bridge_radial_surface_radius"),
                m=sc(det,"cage_mass"))
def parts(det):
    return dict(mass=sc(det,"mass"),
                el=sc(safe(det,"mass_properties_of_elements_from_geometry"),"mass"),
                ir=sc(safe(det,"mass_properties_of_inner_ring_from_geometry"),"mass"),
                orr=sc(safe(det,"mass_properties_of_outer_ring_from_geometry"),"mass"),
                cg=sc(det,"cage_mass"))
d=Design.load(MODEL); asm=d.all_parts_of_type_root_assembly()[0]
b=list(asm.all_parts_of_type_bearing())[0]; det=b.detail

print("=== [1] v1.4 원본 제원 (D_we 140 / L_we 350 / Z 87, Zmax 74.6) ===")
p=parts(det); c=cage(det)
print(f"  Z={sc(det,'number_of_elements'):.0f}  Zmax={sc(det,'theoretical_maximum_number_of_elements'):.2f}")
for k,v in p.items(): print(f"  {k:5} = {v if v is None else f'{v:,.2f}'}")
print(f"  cage_pitch_radius = {safe(det,'cage_pitch_radius')}")
print(f"  cage_width        = {safe(det,'cage_width')}")
print(f"  cage_mass         = {safe(det,'cage_mass')}")
s=(p['el'] or 0)+(p['ir'] or 0)+(p['orr'] or 0)
print(f"  롤러+내륜+외륜 = {s:,.2f}  vs mass {p['mass']:,.2f}  차이 {p['mass']-s:+,.2f} kg")

print("\n=== [2] Z 만 74 로 낮추면 (Zmax 74.62 이내) ===")
det.number_of_elements=74
p2=parts(det)
print(f"  cage_mass = {safe(det,'cage_mass')}  cage_width = {safe(det,'cage_width')}")
s2=(p2['el'] or 0)+(p2['ir'] or 0)+(p2['orr'] or 0)
print(f"  mass {p2['mass']:,.2f} = 3부품 {s2:,.2f} + 차이 {p2['mass']-s2:+,.2f}")

print("\n=== [3] 제원 연동 규칙 (Z 는 eta=0.92 규칙 적용) ===")
print(f"{'D_we':>7} {'L_we':>7} {'D_pw':>7} {'Z':>4} | {'cageR':>10} {'cageW':>10} {'cageTh':>9} {'cageM':>10}")
for D_we,L_we,D_pw in [(0.11051,0.238048,3.3309),(0.11051,0.500,3.3309),
                       (0.140,0.238048,3.3309),(0.200,0.500,3.3309),
                       (0.140,0.350,4.0000),(0.200,0.500,4.5000)]:
    ti,to=0.025674*D_pw,0.024654*D_pw
    dd=round((D_pw-D_we*COSA-2*ti)*1000)/1000; DD=round((D_pw+D_we*COSA+2*to)*1000)/1000
    Z=int(0.92*math.pi*D_pw/D_we)
    for k,v in (("element_diameter",D_we),("roller_length",L_we),("bore",dd),
                ("outer_diameter",DD),("inner_ring_width",L_we+0.062),
                ("outer_ring_width",L_we+0.015),("width",L_we+0.072),
                ("number_of_elements",Z)):
        try: setattr(det,k,v)
        except Exception: pass
    try: det.pitch_circle_diameter=D_pw
    except Exception: pass
    det.roller_profile_set.active_profile_type=RP.DIN_LUNDBERG
    R,W,M=sc(det,"cage_pitch_radius"),sc(det,"cage_width"),sc(det,"cage_mass")
    th=None
    for n in ("cage_thickness","cage_bridge_thickness"):
        th=sc(det,n)
        if th: break
    f=lambda x,s=1e3: "  invalid" if x is None else f"{x*s:10.3f}"
    print(f"{D_we*1e3:7.1f} {L_we*1e3:7.1f} {D_pw*1e3:7.0f} {Z:4d} | {f(R)} {f(W)} "
          f"{'  n/a' if th is None else f'{th*1e3:9.3f}'} {'  invalid' if M is None else f'{M:10.2f}'}")
    if R:
        print(f"        검산: PCD/2+D_we/4 = {(D_pw/2+D_we/4)*1e3:.3f}  |  D_we/6 = {D_we/6*1e3:.3f} mm")
print("완료")
