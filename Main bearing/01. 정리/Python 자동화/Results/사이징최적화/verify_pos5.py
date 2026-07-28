"""연결 해제 후 mount_on 재장착 시도"""
import os, sys, inspect
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
    except Exception as e: return f"<ERR {str(e).splitlines()[0][:50]}>"
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
c=safe(uw,"inner_connection")
print("  connection 메서드:", [n for n in dir(c) if not n.startswith("_") and
      any(k in n.lower() for k in ("delete","disconnect","remove"))])
ok=False
for m in ("delete","disconnect"):
    f=safe(c,m)
    if callable(f):
        try:
            f(); print(f"  conn.{m}() 성공"); ok=True; break
        except Exception as e: print(f"  conn.{m}() 실패 {str(e).splitlines()[0][:60]}")
if not ok:
    for m in ("delete_connections","disconnect_all"):
        f=safe(uw,m)
        if callable(f):
            try: f(); print(f"  uw.{m}() 성공"); ok=True; break
            except Exception as e: print(f"  uw.{m}() 실패 {str(e).splitlines()[0][:60]}")
print("  is_mounted:", safe(uw,"is_mounted"))
try:
    r=uw.try_mount_on(sh, 0.8); print("  try_mount_on(0.8) ->", r)
    for n in dir(r):
        if not n.startswith("_") and not callable(safe(r,n)):
            print(f"      {n} = {safe(r,n)}")
except Exception as e: print("  try_mount_on 실패:", str(e).splitlines()[-1][:90])
print("  is_mounted:", safe(uw,"is_mounted"), " position:", safe(uw,"position"))
try: run("UW 0.8 재장착 후")
except Exception as e: print("  해석 실패:", str(e).splitlines()[-1][:80])
print("완료")
