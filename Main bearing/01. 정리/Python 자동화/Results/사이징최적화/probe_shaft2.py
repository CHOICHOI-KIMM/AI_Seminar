"""ShaftProfile / DesignShaftSection 내부 구조"""
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
SKIP={"cast_to","wrapped","invalid_properties","read_only_properties","TYPE","cast",
      "cast_or_none","disconnect_from_masta","documentation_url","initialize_lifetime_service",
      "all_properties_are_invalid","all_properties_are_read_only"}
def safe(o,n):
    try: return getattr(o,n)
    except Exception as e: return f"<ERR {str(e).splitlines()[0][:35]}>"
def dump(o,title,ind="  "):
    print(f"{ind}--- {title} ({type(o).__name__}) ---")
    for n in sorted(dir(o)):
        if n.startswith("_") or n in SKIP: continue
        v=safe(o,n)
        if callable(v): continue
        s=repr(v)
        print(f"{ind}  {n:34} = {s[:70]}")
d=Design.load(MODEL); asm=d.all_parts_of_type_root_assembly()[0]
sh=list(asm.all_parts_of_type_shaft())[0]; ad=safe(sh,"active_definition")
dump(ad,"active_definition")
op=safe(ad,"outer_profile")
dump(op,"outer_profile","    ")
for n in ("points","profile_points","shaft_profile_points","all_points"):
    v=safe(op,n)
    if v is not None and not isinstance(v,str):
        try:
            pts=list(v); print(f"    outer_profile.{n}: {len(pts)}개")
            dump(pts[0],f"{n}[0]","        ")
            break
        except Exception as e: print(f"    {n} 열거실패 {str(e)[:50]}")
ss=list(safe(ad,"design_shaft_sections"))
dump(ss[0],"design_shaft_sections[0]","    ")
for side in ("left","right"):
    v=safe(ss[0],side)
    if v is not None and not isinstance(v,str): dump(v,f"section.{side}","        ")
print("완료")
