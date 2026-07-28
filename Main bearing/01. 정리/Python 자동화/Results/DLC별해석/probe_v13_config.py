"""부록4 모델(v1.3 50도)의 강성 구성·샤프트·베어링 제원을 v1.4와 동일 기준으로 확인"""
import os, sys, math
HERE = os.path.dirname(os.path.abspath(__file__)); ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)
import masta_clr_legacy  # noqa
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design

MODELS = {
 "v1.3 (부록4 기준)": (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
   r"\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_유연체_FE_온도_50도_260721.Masta"),
 "v1.4 (현 기준파일)": (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
   r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta"),
}

def safe(o, n):
    try: return getattr(o, n)
    except Exception as e: return f"<ERR {str(e).splitlines()[0][:32]}>"

def sc(o, n):
    v = safe(o, n)
    if isinstance(v,(int,float)) and not isinstance(v,bool): return float(v)
    for a in ("value","wrapped"):
        w = safe(v,a)
        if isinstance(w,(int,float)) and not isinstance(w,bool): return float(w)
    return None

for tag, path in MODELS.items():
    print("="*70); print(tag); print("="*70)
    d = Design.load(path)
    asm = d.all_parts_of_type_root_assembly()[0]
    print("파트:", [type(p).__name__ for p in asm.all_parts()])
    for sh in asm.all_parts_of_type_shaft():
        print(f"  Shaft {sh}: is_replaced_by_fe={safe(sh,'is_replaced_by_fe')} length={safe(sh,'length')}")
    for b in asm.all_parts_of_type_bearing():
        det = safe(b,"detail")
        lcs = safe(b,"local_coordinate_system"); org=None
        for a in ("origin","translation","location"):
            o = safe(lcs,a)
            if o is not None and not isinstance(o,str):
                try: org = tuple(round(float(x),4) for x in list(o)[:3]); break
                except Exception: pass
        ca = sc(det,"contact_angle")
        print(f"  [{b}] z={org}  outer_component={safe(b,'outer_component')}  preload={safe(b,'preload')}")
        print(f"      d={sc(det,'bore')} D={sc(det,'outer_diameter')} B={sc(det,'width')} "
              f"PCD={sc(det,'pitch_circle_diameter')}")
        print(f"      D_we={sc(det,'element_diameter')} L_we={sc(det,'roller_length')} "
              f"Z={sc(det,'number_of_elements')} Zmax={sc(det,'theoretical_maximum_number_of_elements')}")
        print(f"      alpha={None if ca is None else round(math.degrees(ca),3)} deg  "
              f"C={sc(det,'basic_dynamic_load_rating')} Cu={sc(det,'fatigue_load_limit')} "
              f"mass={sc(det,'mass')}")
    print(f"  정적 로드케이스 수 = {len(list(asm.design_properties.static_loads))}")
print("완료")
