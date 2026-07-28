"""롤러 프로파일 모델 설정 비교 (DIN/Lundberg vs John Gohar) + 관련 변수 색출"""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__)); ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)
import masta_clr_legacy  # noqa
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design

M = {
 "v1.3 (부록4)": (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
   r"\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_유연체_FE_온도_50도_260721.Masta"),
 "v1.4 (채택파일)": (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
   r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta"),
}
KEY = ("profile","crown","lundberg","gohar","din","logarith","relief","chamfer",
       "radius","taper","end","edge","length","modification","race","element")

def safe(o,n):
    try: return getattr(o,n)
    except Exception as e: return f"<ERR {str(e).splitlines()[0][:38]}>"

for tag,p in M.items():
    print("="*78); print(tag); print("="*78)
    d = Design.load(p); asm = d.all_parts_of_type_root_assembly()[0]
    b = list(asm.all_parts_of_type_bearing())[0]; det = safe(b,"detail")
    print(f"\n[detail 프로파일 관련 속성]")
    for n in sorted(dir(det)):
        if n.startswith("_"): continue
        if not any(k in n.lower() for k in KEY): continue
        v = safe(det,n)
        if callable(v): continue
        s = repr(v)
        if len(s) > 78: s = s[:75]+"..."
        print(f"  {n:52} = {s}")
    rp = safe(det,"roller_profile")
    print(f"\n  roller_profile = {rp}  (type={type(rp).__name__})")
    try:
        print(f"  enum 후보: {[e.name for e in type(rp)]}")
        print(f"  현재 선택: {rp.name} (value={rp.value})")
    except Exception as e:
        print("  enum 열거 실패:", str(e)[:60])
print("완료")
