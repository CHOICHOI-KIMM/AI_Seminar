"""mount_on(offset) 으로 개별 축위치 변경이 해석에 반영되는지 + 샤프트 단면 읽기"""
import os, sys
HERE=os.path.dirname(os.path.abspath(__file__)); RES=os.path.dirname(HERE); ROOT=os.path.dirname(RES)
sys.path.insert(0,ROOT)
import masta_clr_legacy  # noqa
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType
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
bs=list(asm.all_parts_of_type_bearing()); sh=list(asm.all_parts_of_type_shaft())[0]
uw=[b for b in bs if "UW" in str(b)][0]; dw=[b for b in bs if "DW" in str(b)][0]
ipl=next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
lc=next(c for c in asm.design_properties.static_loads if c.name=="Myz_max")
try: lc.inputs_for_power_load(ipl).speed=0.0; lc.inputs_for_power_load(ipl).torque=22673e3
except Exception: pass
def run(tag):
    sd=lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION); sd.perform_analysis()
    s={}
    for b in bs:
        k="UW" if "UW" in str(b) else "DW"
        s[k]=sc(sd.results_for(b).component_detailed_analysis,"maximum_normal_stress")/1e6
    print(f"  {tag:26} UW={safe(uw,'position')[0]:.3f} DW={safe(dw,'position')[0]:.3f}"
          f"  sig_UW={s['UW']:8.1f} sig_DW={s['DW']:8.1f}")
run("기준 0.5 / 3.0")
print("  mount_on(shaft, 0.8) ->", uw.mount_on(sh, 0.8))
run("UW mount_on 0.8")
print("  mount_on(shaft, 3.4) ->", dw.mount_on(sh, 3.4))
run("DW mount_on 3.4")
uw.mount_on(sh,0.5); dw.mount_on(sh,3.0); run("원복 0.5 / 3.0")

print("\n=== 샤프트 단면 (active_definition) ===")
ad=safe(sh,"active_definition")
print("  타입:", type(ad).__name__)
try:
    secs=list(safe(ad,"design_shaft_sections") or [])
    print(f"  design_shaft_sections: {len(secs)}개")
    if secs:
        s0=secs[0]
        print("   [0] 속성:", [n for n in dir(s0) if not n.startswith("_") and
              any(k in n.lower() for k in ("offset","outer","inner","diameter","length"))])
        for i,s_ in enumerate(secs[:4]+secs[-2:]):
            vals={n:sc(s_,n) for n in dir(s_) if not n.startswith("_") and
                  any(k in n.lower() for k in ("offset","outer","inner"))}
            print(f"   sec{i}: {({k:round(v,4) for k,v in vals.items() if v is not None})}")
except Exception as e: print("  실패", str(e)[:80])
for prof in ("outer_profile","inner_profile"):
    v=safe(ad,prof)
    print(f"  {prof}: {type(v).__name__}")
    if not isinstance(v,str) and v is not None:
        try:
            pts=list(v)
            print(f"     {len(pts)} points; [0] 속성 =",
                  [n for n in dir(pts[0]) if not n.startswith("_")][:12])
        except Exception as e: print("     열거 실패", str(e)[:50])
print("완료")
