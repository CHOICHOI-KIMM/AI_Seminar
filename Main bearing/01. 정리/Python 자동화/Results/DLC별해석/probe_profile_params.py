"""active_profile 객체의 파라미터 직접 덤프 (DIN Lundberg vs Johns Gohar) + 크라우닝 량 비교"""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__)); ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)
import masta_clr_legacy  # noqa
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design

M = {
 "v1.3 DIN_LUNDBERG": (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
   r"\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_유연체_FE_온도_50도_260721.Masta"),
 "v1.4 JOHNS_GOHAR": (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
   r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta"),
}
SKIP = {"cast_to","wrapped","report_names","invalid_properties","read_only_properties",
        "all_properties_are_invalid","all_properties_are_read_only"}

def safe(o,n):
    try: return getattr(o,n)
    except Exception as e: return f"<ERR {str(e).splitlines()[0][:34]}>"

for tag,p in M.items():
    print("="*74); print(tag); print("="*74)
    d = Design.load(p); asm = d.all_parts_of_type_root_assembly()[0]
    b = list(asm.all_parts_of_type_bearing())[0]; det = safe(b,"detail")
    ps = safe(det,"roller_profile_set"); ap = safe(ps,"active_profile")
    print(f"[active_profile] {type(ap).__name__}")
    for n in sorted(dir(ap)):
        if n.startswith("_") or n in SKIP: continue
        v = safe(ap,n)
        if callable(v): continue
        s = repr(v)
        if len(s)>60: s=s[:57]+"..."
        print(f"   {n:46} = {s}")
    for race in ("inner_race_and_roller_profiles","outer_race_and_roller_profiles"):
        try:
            pts=list(safe(det,race))
            off=[float(safe(q,"offset_from_roller_centre")) for q in pts]
            tot=[float(safe(q,"total_deviation")) for q in pts]
            print(f"  [{race}] {len(pts)}pt  offset {min(off)*1000:.2f}~{max(off)*1000:.2f} mm"
                  f"  최대편차 {max(tot)*1e6:.2f} um  (중앙 {tot[len(tot)//2]*1e6:.2f} um)")
        except Exception as e:
            print(f"  [{race}] 실패 {str(e)[:50]}")
    print(f"  L_we = {safe(det,'effective_roller_length')}  D_we = {safe(det,'element_diameter')}")
print("완료")
