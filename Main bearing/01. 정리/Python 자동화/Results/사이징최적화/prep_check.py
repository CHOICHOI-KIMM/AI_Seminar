"""부록1 사전검증 — v1.3 제원 이식 + 프로파일 전환 + C/Cu 확인 + 응력 속성 색출"""
import os, sys, math
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)
import masta_clr_legacy  # noqa
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType
from mastapy.bearings import RollerBearingProfileTypes

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
V13 = dict(element_diameter=0.11051, roller_length=0.238048,
           outer_diameter=3.6, width=0.31, number_of_elements=87)

def safe(o,n):
    try: return getattr(o,n)
    except Exception as e: return f"<ERR {str(e).splitlines()[0][:40]}>"
def sc(o,n):
    v=safe(o,n)
    if isinstance(v,(int,float)) and not isinstance(v,bool): return float(v)
    for a in ("value","wrapped"):
        w=safe(v,a)
        if isinstance(w,(int,float)) and not isinstance(w,bool): return float(w)
    return None

d = Design.load(MODEL)
asm = d.all_parts_of_type_root_assembly()[0]
bearings = list(asm.all_parts_of_type_bearing())
print("[이식 전]")
det0 = safe(bearings[0],"detail")
for k in V13: print(f"   {k:22} = {sc(det0,k)}")
print(f"   C={sc(det0,'basic_dynamic_load_rating'):,.0f}  Cu={sc(det0,'fatigue_load_limit'):,.0f}")

print("\n[v1.3 제원 이식]")
for b in bearings:
    det = safe(b,"detail")
    for k,v in V13.items():
        try: setattr(det,k,v)
        except Exception as e: print(f"   !! {k} 설정 실패: {str(e).splitlines()[0][:60]}")
det = safe(bearings[0],"detail")
for k in V13: print(f"   {k:22} = {sc(det,k)}")
print(f"   T={sc(det,'width')} B={sc(det,'inner_ring_width')} C_ring={sc(det,'outer_ring_width')}")
print(f"   L_we={sc(det,'effective_roller_length')} roller_end_R={sc(det,'roller_end_radius')}")
print(f"   PCD={sc(det,'pitch_circle_diameter')} Zmax={sc(det,'theoretical_maximum_number_of_elements')}")
print(f"   C={sc(det,'basic_dynamic_load_rating'):,.0f} (목표 22,227,979)")
print(f"   Cu={sc(det,'fatigue_load_limit'):,.0f} (목표 3,929,017)")

print("\n[프로파일 전환 시험]")
for b in bearings:
    ps = safe(safe(b,"detail"),"roller_profile_set")
    print(f"   {b} 현재 = {safe(ps,'active_profile_type')}")
    for attr in ("active_profile_type",):
        try:
            setattr(ps, attr, RollerBearingProfileTypes.DIN_LUNDBERG)
            print(f"     -> {attr} 설정 성공: {safe(ps,'active_profile_type')}")
        except Exception as e:
            print(f"     !! {attr} 실패: {str(e).splitlines()[0][:70]}")
det = safe(bearings[0],"detail")
ap = safe(safe(det,"roller_profile_set"),"active_profile")
print(f"   active_profile = {type(ap).__name__}")
try:
    pts=list(safe(det,"inner_race_and_roller_profiles"))
    tot=[float(safe(q,"total_deviation")) for q in pts]
    print(f"   단부드롭 = {max(tot)*1e6:.2f} um  (DIN 기대 244.27)")
except Exception as e: print("   프로파일점 실패", str(e)[:50])

print("\n[극한 LC 1건 해석 -> 응력 속성 색출]")
pl = list(asm.all_parts_of_type_point_load())[0]
ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
lc = next(c for c in asm.design_properties.static_loads if c.name=="Mx_max")
q = lc.inputs_for_power_load(ipl)
try: q.speed = 0.0
except Exception as e: print("   speed 실패", str(e)[:50])
sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION); sd.perform_analysis()
res = sd.results_for(bearings[0]).component_detailed_analysis
print(f"   결과객체 = {type(res).__name__}")
for n in sorted(dir(res)):
    if n.startswith("_"): continue
    if "stress" not in n.lower(): continue
    v = safe(res,n)
    if callable(v): continue
    s=repr(v)
    print(f"     {n:56} = {s[:60]}")
print("완료")
