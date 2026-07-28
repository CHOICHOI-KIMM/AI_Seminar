"""MASTA 질량 구성 확정 — detail.mass 가 어디까지 포함하는가"""
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
V13=[("element_diameter",0.11051),("roller_length",0.238048),("outer_diameter",3.6),
     ("inner_ring_width",0.3),("outer_ring_width",0.253),("width",0.31),
     ("number_of_elements",87)]
def safe(o,n):
    try: return getattr(o,n)
    except Exception as e: return None
def sc(o,n):
    v=safe(o,n)
    if isinstance(v,(int,float)) and not isinstance(v,bool): return float(v)
    for a in ("value","wrapped"):
        w=safe(v,a)
        if isinstance(w,(int,float)) and not isinstance(w,bool): return float(w)
    return None
d=Design.load(MODEL); asm=d.all_parts_of_type_root_assembly()[0]
bs=list(asm.all_parts_of_type_bearing()); sh=list(asm.all_parts_of_type_shaft())[0]
for b in bs:
    det=b.detail
    for k,v in V13: setattr(det,k,v)
    det.roller_profile_set.active_profile_type = RP.DIN_LUNDBERG
det=bs[0].detail
print("=== 베어링 (v1.3 제원) ===")
print(f"  detail.mass                = {sc(det,'mass'):,.2f} kg")
mp=safe(det,"mass_properties_of_elements_from_geometry")
print(f"  요소(롤러) 질량            = {sc(mp,'mass'):,.2f} kg" if mp is not None else "  요소 질량 조회 실패")
bp=safe(bs[0],"mass_properties_from_design")
print(f"  bearing.mass_properties_from_design = {sc(bp,'mass'):,.2f} kg" if bp is not None else "")
for n in sorted(dir(det)):
    if n.startswith("_"): continue
    if not any(k in n.lower() for k in ("mass","density","material")): continue
    v=safe(det,n)
    if callable(v): continue
    print(f"  {n:44} = {str(v)[:56]}")
em=safe(det,"element_material")
if em is not None:
    print("  [element_material]", [f"{n}={sc(em,n)}" for n in dir(em)
          if not n.startswith('_') and 'densit' in n.lower()])
print("\n=== 샤프트 ===")
print(f"  mass_of_shaft_body = {sc(sh,'mass_of_shaft_body'):,.2f} kg")
mps=safe(sh,"mass_properties_from_design")
print(f"  mass_properties_from_design = {sc(mps,'mass'):,.2f} kg" if mps is not None else "")
ad=safe(sh,"active_definition"); sm=safe(ad,"shaft_material")
if sm is not None:
    print("  [shaft_material]", [f"{n}={sc(sm,n)}" for n in dir(sm)
          if not n.startswith('_') and 'densit' in n.lower()])
print("\n=== 해석적 대조 (밀도 7850 가정) ===")
D_we,L_we,Z=0.11051,0.238048,87
mi,mo=sc(det,'mean_inner_race_diameter'),sc(det,'mean_outer_race_diameter')
d0,D0=sc(det,'bore'),sc(det,'outer_diameter')
B,C=sc(det,'inner_ring_width'),sc(det,'outer_ring_width')
vr=math.pi/4*D_we**2*L_we*Z
vi=math.pi/4*(mi**2-d0**2)*B
vo=math.pi/4*(D0**2-mo**2)*C
print(f"  롤러 {vr*7850:8,.0f} kg / 내륜 {vi*7850:8,.0f} kg / 외륜 {vo*7850:8,.0f} kg"
      f"  합 {(vr+vi+vo)*7850:9,.0f} kg")
print(f"  MASTA detail.mass = {sc(det,'mass'):,.0f} kg  차이 {(sc(det,'mass')/((vr+vi+vo)*7850)-1)*100:+.1f}%")
print("완료")
