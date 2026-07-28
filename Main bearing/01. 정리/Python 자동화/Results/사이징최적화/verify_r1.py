"""R1 검증 — (a) DIN 자동스케일 (b) z1/z2 이동 가능성 (c) 코너설계에서 DIN의 최적근접성"""
import os, sys, math
HERE=os.path.dirname(os.path.abspath(__file__)); RES=os.path.dirname(HERE); ROOT=os.path.dirname(RES)
sys.path.insert(0,ROOT)
import masta_clr_legacy  # noqa
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType
from mastapy.bearings import RollerBearingProfileTypes as RP

MODEL=(r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
       r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
       r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
COSA=math.cos(math.radians(19.0))
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
def drop(det):
    pts=list(safe(det,"inner_race_and_roller_profiles"))
    return max(float(safe(q,"total_deviation")) for q in pts)*1e6

d=Design.load(MODEL); asm=d.all_parts_of_type_root_assembly()[0]
bs=list(asm.all_parts_of_type_bearing()); sh=list(asm.all_parts_of_type_shaft())[0]

print("=== (b) 베어링 축방향 위치 이동 가능성 ===")
b=bs[0]
cands=[n for n in dir(b) if not n.startswith("_") and any(k in n.lower()
       for k in ("offset","position","location","mount"))]
print("  후보 속성:", cands)
for n in cands:
    print(f"    {n} = {safe(b,n)}")
conn=safe(b,"inner_connection")
print("  inner_connection 속성:", [n for n in dir(conn) if not n.startswith("_")
      and any(k in n.lower() for k in ("offset","position","socket"))] if conn else None)
try:
    ms=list(safe(sh,"mountable_components") or [])
    print(f"  shaft.mountable_components: {[str(x) for x in ms]}")
except Exception as e: print("  mountable_components 실패:", str(e)[:60])
for n in ("offset","axial_offset","position"):
    print(f"  b.{n} 쓰기시험:", end=" ")
    try:
        v0=sc(b,n)
        if v0 is None: print("속성없음"); continue
        setattr(b,n,v0+0.1); print(f"OK {v0} -> {sc(b,n)}"); setattr(b,n,v0)
    except Exception as e: print("실패", str(e).splitlines()[0][:50])

print("\n=== (a) DIN Lundberg 자동 스케일 ===")
print(f"{'D_we':>7} {'L_we':>7} {'D_pw':>7} | {'end_drop':>10} {'drop/D_we':>10} {'drop/L_we':>10}")
for D_we,L_we,D_pw in [(0.11051,0.238048,3.3309),(0.11051,0.500,3.3309),
                       (0.200,0.238048,3.3309),(0.200,0.500,3.3309),
                       (0.140,0.400,4.0000),(0.200,0.500,4.5000)]:
    ti,to=0.025674*D_pw,0.024654*D_pw
    dd=round((D_pw-D_we*COSA-2*ti)*1000)/1000; DD=round((D_pw+D_we*COSA+2*to)*1000)/1000
    Z=int(0.92*math.pi*D_pw/D_we)
    for b_ in bs:
        det=safe(b_,"detail")
        for k,v in (("element_diameter",D_we),("roller_length",L_we),
                    ("bore",dd),("outer_diameter",DD),
                    ("inner_ring_width",L_we+0.062),("outer_ring_width",L_we+0.015),
                    ("width",L_we+0.072),("number_of_elements",Z)):
            try: setattr(det,k,v)
            except Exception: pass
        try: det.pitch_circle_diameter = D_pw
        except Exception: pass
        safe(det,"roller_profile_set").active_profile_type = RP.DIN_LUNDBERG
    det=safe(bs[0],"detail")
    dr=drop(det)
    print(f"{sc(det,'element_diameter')*1e3:7.1f} {sc(det,'roller_length')*1e3:7.1f} "
          f"{sc(det,'pitch_circle_diameter')*1e3:7.1f} | {dr:10.2f} "
          f"{dr/(sc(det,'element_diameter')*1e6):10.5f} {dr/(sc(det,'roller_length')*1e6):10.5f}")
print("완료")
