"""샤프트 단면 실측 — 재구성 규칙 수립용"""
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
sh=list(asm.all_parts_of_type_shaft())[0]
ad=safe(sh,"active_definition")
print("length =", safe(sh,"length"))
for prof in ("outer_profile","inner_profile"):
    v=safe(ad,prof)
    try:
        pts=list(v)
        print(f"\n[{prof}] {len(pts)} points")
        p0=pts[0]
        attrs=[n for n in dir(p0) if not n.startswith("_") and not callable(safe(p0,n))
               and n not in ("cast_to","wrapped","invalid_properties","read_only_properties",
                             "all_properties_are_invalid","all_properties_are_read_only")]
        print("   속성:", attrs)
        print(f"   {'i':>3} " + " ".join(f"{a[:14]:>15}" for a in attrs))
        idx=list(range(min(6,len(pts))))+[len(pts)//2]+list(range(max(0,len(pts)-4),len(pts)))
        for i in sorted(set(idx)):
            q=pts[i]
            vals=[]
            for a in attrs:
                x=sc(q,a)
                vals.append(f"{x:15.5f}" if x is not None else f"{str(safe(q,a))[:15]:>15}")
            print(f"   {i:3d} " + " ".join(vals))
    except Exception as e:
        print(f"[{prof}] 열거 실패:", str(e)[:70])
secs=safe(ad,"design_shaft_sections")
try:
    ss=list(secs)
    print(f"\n[design_shaft_sections] {len(ss)}개")
    if ss:
        s0=ss[0]
        print("   속성:", [n for n in dir(s0) if not n.startswith("_")][:20])
except Exception as e: print("\n[design_shaft_sections] 실패:", str(e)[:60])
print("완료")
