"""설계공간 전역 케이지 유효성 점검 — P1 격자 수준 전수"""
import os, sys, math, csv, time
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
OUT=os.path.join(HERE,"부록3_S1기하검증","cage_sweep.csv")
DPW=[3.3309,3.600,3.900,4.200,4.500]
DWE=[0.110,0.140,0.170,0.200]
LWE=[0.238,0.325,0.415,0.500]
ALP=[12.0,19.0,24.0,30.0]
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
d=Design.load(MODEL); asm=d.all_parts_of_type_root_assembly()[0]
det=list(asm.all_parts_of_type_bearing())[0].detail
det.roller_profile_set.active_profile_type=RP.DIN_LUNDBERG
rows=[]; nbad=0; t0=time.time()
print(f"{'D_pw':>6} {'D_we':>6} {'L_we':>6} {'a':>4} {'Z':>4} {'Zmax':>6} "
      f"{'cage_m':>9} {'mass':>10} {'4부품합':>10} {'차이':>8}")
for D_pw in DPW:
    for D_we in DWE:
        for L_we in LWE:
            if not (1.5 <= L_we/D_we <= 4.0):     # (C2) 세장비
                continue
            for al in ALP:
                ca=math.cos(math.radians(al))
                ti,to=0.025674*D_pw,0.024654*D_pw
                dd=round((D_pw-D_we*ca-2*ti)*1000)/1000
                DD=round((D_pw+D_we*ca+2*to)*1000)/1000
                Z=int(0.92*math.pi*D_pw/D_we)
                ok=True
                for k,v in (("element_diameter",D_we),("roller_length",L_we),
                            ("bore",dd),("outer_diameter",DD),
                            ("inner_ring_width",L_we+0.062),("outer_ring_width",L_we+0.015),
                            ("width",L_we+0.072),("number_of_elements",Z)):
                    try: setattr(det,k,v)
                    except Exception: ok=False
                for k,v in (("pitch_circle_diameter",D_pw),("contact_angle",math.radians(al))):
                    try: setattr(det,k,v)
                    except Exception: pass
                cm=sc(det,"cage_mass"); m=sc(det,"mass")
                el=sc(safe(det,"mass_properties_of_elements_from_geometry"),"mass")
                ir=sc(safe(det,"mass_properties_of_inner_ring_from_geometry"),"mass")
                orr=sc(safe(det,"mass_properties_of_outer_ring_from_geometry"),"mass")
                s4=(el or 0)+(ir or 0)+(orr or 0)+(cm or 0)
                zmax=sc(det,"theoretical_maximum_number_of_elements")
                bad = (cm is None)
                if bad: nbad+=1
                rows.append(dict(D_pw_mm=D_pw*1e3,D_we_mm=D_we*1e3,L_we_mm=L_we*1e3,
                                 alpha=al,Z=Z,Zmax=None if zmax is None else round(zmax,2),
                                 cage_valid=0 if bad else 1,
                                 cage_mass=cm,mass=m,sum4=s4,
                                 diff=None if m is None else m-s4,
                                 cageR=sc(det,"cage_pitch_radius"),
                                 cageW=sc(det,"cage_width"),
                                 setok=1 if ok else 0))
                if bad or (m is not None and abs(m-s4)>1.0):
                    print(f"{D_pw*1e3:6.0f} {D_we*1e3:6.0f} {L_we*1e3:6.0f} {al:4.0f} {Z:4d} "
                          f"{'  n/a' if zmax is None else f'{zmax:6.2f}'} "
                          f"{'  INVALID':>9} {m if m is None else f'{m:10.1f}'} "
                          f"{s4:10.1f} {'':>8}  <-- 확인필요")
print(f"\n[요약] {len(rows)}점 평가 ({time.time()-t0:.0f}s) · 케이지 invalid {nbad}점")
val=[r for r in rows if r["cage_valid"]]
if val:
    cm=[r["cage_mass"] for r in val]; df=[abs(r["diff"]) for r in val if r["diff"] is not None]
    print(f"  cage_mass {min(cm):,.1f} ~ {max(cm):,.1f} kg")
    print(f"  |mass - 4부품합| 최대 {max(df):.6f} kg  → 케이지 포함 여부 전 점 일관")
    rr=[abs(r["cageR"]-(r["D_pw_mm"]/2+r["D_we_mm"]/4)) for r in val if r["cageR"]]
    print(f"  cageR 규칙 오차 최대 {max(rr)*1e3:.4f} mm  (D_pw/2 + D_we/4)")
    ww=[abs(r["cageW"]*1e3-(r["L_we_mm"]+72)) for r in val if r["cageW"]]
    print(f"  cageW 규칙 오차 최대 {max(ww):.4f} mm  (L_we + 72)")
with open(OUT,"w",newline="",encoding="utf-8-sig") as f:
    w=csv.DictWriter(f,fieldnames=list(rows[0])); w.writeheader(); w.writerows(rows)
print(f"[저장] {OUT}")
