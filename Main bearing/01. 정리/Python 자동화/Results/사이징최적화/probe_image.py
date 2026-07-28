"""MASTA 베어링 형상 이미지 추출 가능성 조사"""
import os, sys
HERE=os.path.dirname(os.path.abspath(__file__)); RES=os.path.dirname(HERE); ROOT=os.path.dirname(RES)
sys.path.insert(0,ROOT)
import masta_clr_legacy  # noqa
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
MODEL=(r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
       r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
       r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
OUT=os.path.join(HERE,"_img_test"); os.makedirs(OUT,exist_ok=True)
def safe(o,n):
    try: return getattr(o,n)
    except Exception as e: return f"ERR:{str(e).splitlines()[0][:30]}"
d=Design.load(MODEL); asm=d.all_parts_of_type_root_assembly()[0]
b=list(asm.all_parts_of_type_bearing())[0]; det=b.detail

for label,obj in (("Bearing(part)",b),("detail(TaperRollerBearing)",det),
                  ("RootAssembly",asm)):
    print(f"\n=== {label} — 이미지/도면 후보 ===")
    for n in sorted(dir(obj)):
        if n.startswith("_"): continue
        low=n.lower()
        if not any(k in low for k in ("image","view","draw","chart","picture","cad","2d","3d","report")):
            continue
        v=safe(obj,n)
        tn=type(v).__name__
        extra=""
        if hasattr(v,"size"): extra=f"  size={v.size} mode={getattr(v,'mode','')}"
        print(f"  {n:62} {tn}{extra}")

print("\n=== 저장 시험 ===")
saved=0
for n in sorted(dir(b)):
    if n.startswith("_"): continue
    v=safe(b,n)
    if type(v).__name__.endswith("ImageFile") or hasattr(v,"save"):
        try:
            fp=os.path.join(OUT,f"bearing_{n}.png"); v.save(fp)
            print(f"  OK  {n:60} -> {os.path.getsize(fp):,} B  {v.size}")
            saved+=1
        except Exception as e: print(f"  실패 {n}: {str(e)[:50]}")
for n in sorted(dir(det)):
    if n.startswith("_"): continue
    v=safe(det,n)
    if type(v).__name__.endswith("ImageFile") or hasattr(v,"save"):
        try:
            fp=os.path.join(OUT,f"detail_{n}.png"); v.save(fp)
            print(f"  OK  detail.{n:53} -> {os.path.getsize(fp):,} B  {v.size}")
            saved+=1
        except Exception as e: print(f"  실패 detail.{n}: {str(e)[:50]}")
print(f"\n저장 {saved}건")
print("\n=== report_names ===")
for label,obj in (("bearing",b),("detail",det)):
    print(f"  {label}: {safe(obj,'report_names')}")
