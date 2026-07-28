"""위치 이동이 해석에 실제로 반영되는가 — 샤프트 내부 이동으로 시험"""
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
    print(f"  {tag:28} UW pos={safe(uw,'position')[0]:.3f} DW pos={safe(dw,'position')[0]:.3f}"
          f"  sigma_UW={s['UW']:.1f}  sigma_DW={s['DW']:.1f}")
    return s
run("기준 (0.5 / 3.0)")
r=uw.set_position_of_component_and_connected_components((0.8,0.0,0.0))
print("  UW 0.5->0.8 반환:", r, type(r).__name__)
for n in dir(r):
    if not n.startswith("_") and not callable(safe(r,n)):
        print(f"      {n} = {safe(r,n)}")
run("UW 0.8 로 이동 후")
dw.set_position_of_component_and_connected_components((2.5,0.0,0.0))
run("DW 2.5 로 이동 후")
print("\n샤프트 length:", safe(sh,"length"), " 쓰기가능? ")
try: sh.length=4.0; print("   OK ->", safe(sh,"length"))
except Exception as e: print("   실패:", str(e).splitlines()[0][:70])
try:
    secs=safe(sh,"sections")
    print("   sections type:", type(secs).__name__)
except Exception as e: print("   sections 실패", str(e)[:50])
sd_=safe(sh,"shaft_design") if not isinstance(safe(sh,"shaft_design"),str) else None
print("   shaft_design:", type(sd_).__name__ if sd_ else safe(sh,"shaft_design"))
if sd_:
    print("   후보:", [n for n in dir(sd_) if not n.startswith("_") and
          any(k in n.lower() for k in ("length","section","outer","inner","profile"))][:20])
print("완료")
