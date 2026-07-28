"""(1) UW/DW 프로파일이 각각 무엇인지 (2) ProfileSet 객체 공유 여부 (3) width 설정 순서"""
import os, sys
HERE=os.path.dirname(os.path.abspath(__file__)); ROOT=os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0,ROOT)
import masta_clr_legacy  # noqa
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.bearings import RollerBearingProfileTypes
MODEL=(r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
       r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
       r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
def safe(o,n):
    try: return getattr(o,n)
    except Exception as e: return f"<ERR {str(e).splitlines()[0][:45]}>"
def sc(o,n):
    v=safe(o,n)
    if isinstance(v,(int,float)) and not isinstance(v,bool): return float(v)
    for a in ("value","wrapped"):
        w=safe(v,a)
        if isinstance(w,(int,float)) and not isinstance(w,bool): return float(w)
    return None
d=Design.load(MODEL); asm=d.all_parts_of_type_root_assembly()[0]
bs=list(asm.all_parts_of_type_bearing())
print("[1] 각 베어링의 프로파일 (변경 전)")
sets=[]
for b in bs:
    det=safe(b,"detail"); ps=safe(det,"roller_profile_set"); sets.append(ps)
    pts=list(safe(det,"inner_race_and_roller_profiles"))
    tot=max(float(safe(q,"total_deviation")) for q in pts)
    print(f"   {b}: {safe(ps,'active_profile_type')}  단부드롭 {tot*1e6:.2f} um  "
          f"active={type(safe(ps,'active_profile')).__name__}")
print(f"\n[2] ProfileSet 객체 공유 여부: id 동일? {id(sets[0])==id(sets[1])}  "
      f"wrapped 동일? {safe(sets[0],'wrapped')==safe(sets[1],'wrapped')}")
print("   UW만 DIN으로 변경 후 DW 확인:")
sets[0].active_profile_type = RollerBearingProfileTypes.DIN_LUNDBERG
print(f"     UW={safe(sets[0],'active_profile_type')}  DW={safe(sets[1],'active_profile_type')}")

print("\n[3] width 설정 — 링폭 먼저 낮춘 뒤 T 설정")
for b in bs:
    det=safe(b,"detail")
    for k,v in (("element_diameter",0.11051),("roller_length",0.238048),("outer_diameter",3.6)):
        setattr(det,k,v)
    for k,v in (("inner_ring_width",0.3),("outer_ring_width",0.253)):
        try: setattr(det,k,v)
        except Exception as e: print(f"   !! {k}: {str(e).splitlines()[0][:60]}")
    try: det.width = 0.31
    except Exception as e: print(f"   !! width: {str(e).splitlines()[0][:60]}")
det=safe(bs[0],"detail")
print(f"   T={sc(det,'width')} B={sc(det,'inner_ring_width')} C={sc(det,'outer_ring_width')}")
print(f"   (목표 T=0.310 B=0.300 C=0.253)")
print(f"   C_rating={sc(det,'basic_dynamic_load_rating'):,.0f}  Cu={sc(det,'fatigue_load_limit'):,.0f}")
print(f"   mass={sc(det,'mass'):,.1f} kg  (v1.3 실측 5,600.5)")
print("완료")
