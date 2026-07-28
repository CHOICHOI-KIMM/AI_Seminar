"""ProfileSet 내부 변수 파악 — DIN Lundberg / Johns Gohar 파라미터"""
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

def safe(o,n):
    try: return getattr(o,n)
    except Exception as e: return f"<ERR {str(e).splitlines()[0][:34]}>"

def dump(o, title, ind="    "):
    print(f"{ind}--- {title} ({type(o).__name__}) ---")
    for n in sorted(dir(o)):
        if n.startswith("_"): continue
        v = safe(o,n)
        if callable(v): continue
        s = repr(v)
        if len(s) > 70: s = s[:67]+"..."
        print(f"{ind}  {n:44} = {s}")

for tag,p in M.items():
    print("="*78); print(tag); print("="*78)
    d = Design.load(p); asm = d.all_parts_of_type_root_assembly()[0]
    b = list(asm.all_parts_of_type_bearing())[0]; det = safe(b,"detail")
    for setname in ("roller_profile_set","inner_race_profile_set","outer_race_profile_set"):
        ps = safe(det,setname)
        if isinstance(ps,str) or ps is None: 
            print(f"  {setname}: {ps}"); continue
        dump(ps, setname, "  ")
        for sub in ("profile","profile_to_use","user_specified_profile","profile_data"):
            v = safe(ps,sub)
            if v is not None and not isinstance(v,str) and not callable(v):
                dump(v, f"{setname}.{sub}", "      ")
    prof = safe(det,"inner_race_and_roller_profiles")
    try:
        pts = list(prof)
        print(f"\n  inner_race_and_roller_profiles: {len(pts)} points")
        if pts:
            dump(pts[0], "point[0]", "    ")
            print("    (일부 점 값)")
            for q in pts[:3] + pts[len(pts)//2:len(pts)//2+1] + pts[-2:]:
                vals = {n: safe(q,n) for n in dir(q)
                        if not n.startswith("_") and isinstance(safe(q,n),(int,float))}
                print("      ", {k: round(v,8) for k,v in vals.items()})
    except Exception as e:
        print("  profiles 조회 실패:", str(e)[:70])
print("완료")
