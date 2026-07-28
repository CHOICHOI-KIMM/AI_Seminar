"""기준 모델의 샤프트 FE 치환 여부·하우징 FE 확인 + Z 변경 가능성 시험"""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__)); ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)
import masta_clr_legacy  # noqa
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")

def safe(o, n):
    try: return getattr(o, n)
    except Exception as e: return f"<ERR {str(e).splitlines()[0][:40]}>"

d = Design.load(MODEL)
asm = d.all_parts_of_type_root_assembly()[0]
print("[샤프트]")
for sh in asm.all_parts_of_type_shaft():
    print(f"  {sh}: is_replaced_by_fe = {safe(sh,'is_replaced_by_fe')}")
    for a in ("outer_diameter","inner_diameter","length","bore"):
        print(f"     {a} = {safe(sh,a)}")
print("[FE 파트]")
try:
    for fe in asm.all_parts_of_type_fe_part():
        print(f"  {fe}  is_housing={safe(fe,'is_housing')}")
except Exception as e: print("  없음/실패:", str(e)[:60])
print("[샤프트 단면]")
for sh in asm.all_parts_of_type_shaft():
    try:
        secs = list(safe(sh, "sections") or [])
        print(f"  {sh}: {len(secs)} sections")
        for s in secs[:12]:
            print(f"    {s} | OD={safe(s,'outer_diameter')} ID={safe(s,'inner_diameter')} L={safe(s,'length')}")
    except Exception as e: print("   조회 실패:", str(e)[:60])
print("[Z 변경 시험]")
b = list(asm.all_parts_of_type_bearing())[0]
det = safe(b, "detail")
print("  변경 전 Z =", safe(det, "number_of_elements"), " C =", safe(det, "basic_dynamic_load_rating"))
try:
    det.number_of_elements = 68
    print("  변경 후 Z =", safe(det, "number_of_elements"), " C =", safe(det, "basic_dynamic_load_rating"),
          " Cu =", safe(det, "fatigue_load_limit"))
except Exception as e:
    print("  변경 실패:", str(e).splitlines()[0][:80])
print("완료")
