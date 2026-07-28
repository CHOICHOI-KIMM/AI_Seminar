"""Johns-Gohar design_load 스윕 — end_drop 과 극한응력(Myz_max) 응답 파악
v1.3 제원 고정. 문헌식 P_L(x) = 2Qd(1-v^2)/(pi*lwe*E) * ln[1/(1-(1-0.3033a/b)(2x/lwe)^2)]
"""
import os, sys, math
HERE=os.path.dirname(os.path.abspath(__file__)); RES=os.path.dirname(HERE); ROOT=os.path.dirname(RES)
sys.path.insert(0,ROOT)
import masta_clr_legacy  # noqa
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType
from mastapy.bearings import RollerBearingProfileTypes

MODEL=(r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
       r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
       r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
V13=[("element_diameter",0.11051),("roller_length",0.238048),("outer_diameter",3.6),
     ("inner_ring_width",0.3),("outer_ring_width",0.253),("width",0.31),("number_of_elements",87)]
QD=[0.2e6,0.36e6,0.6e6,1.0e6,1.6e6,3.2e6,6.0e6,12e6,30e6]

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
bs=list(asm.all_parts_of_type_bearing())
for b in bs:
    det=safe(b,"detail")
    for k,v in V13: setattr(det,k,v)
    safe(det,"roller_profile_set").active_profile_type = RollerBearingProfileTypes.JOHNS_GOHAR
ipl=next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
lc=next(c for c in asm.design_properties.static_loads if c.name=="Myz_max")
try: lc.inputs_for_power_load(ipl).speed=0.0
except Exception: pass
try: lc.inputs_for_power_load(ipl).torque=22673.0*1e3
except Exception: pass

print(f"{'Qd [MN]':>9} {'end_drop [um]':>14} {'sigma_UW [MPa]':>15} {'sigma_DW [MPa]':>15}")
print("-"*58)
for qd in QD:
    for b in bs:
        ap=safe(safe(safe(b,"detail"),"roller_profile_set"),"active_profile")
        try: ap.design_load=qd
        except Exception as e: print("  set 실패",str(e)[:50])
    det=safe(bs[0],"detail")
    pts=list(safe(det,"inner_race_and_roller_profiles"))
    drop=max(float(safe(q,"total_deviation")) for q in pts)*1e6
    sd=lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION); sd.perform_analysis()
    s={}
    for b in bs:
        key="UW" if "UW" in str(b) else "DW"
        s[key]=sc(sd.results_for(b).component_detailed_analysis,"maximum_normal_stress")/1e6
    print(f"{qd/1e6:9.2f} {drop:14.2f} {s['UW']:15.1f} {s['DW']:15.1f}")

# 참고: DIN
for b in bs:
    safe(safe(b,"detail"),"roller_profile_set").active_profile_type = RollerBearingProfileTypes.DIN_LUNDBERG
det=safe(bs[0],"detail")
pts=list(safe(det,"inner_race_and_roller_profiles"))
drop=max(float(safe(q,"total_deviation")) for q in pts)*1e6
sd=lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION); sd.perform_analysis()
s={}
for b in bs:
    key="UW" if "UW" in str(b) else "DW"
    s[key]=sc(sd.results_for(b).component_detailed_analysis,"maximum_normal_stress")/1e6
print(f"{'DIN':>9} {drop:14.2f} {s['UW']:15.1f} {s['DW']:15.1f}")
print("완료")
