"""표본3(DLC6.4-a-s3) 저속 κ 검토 — MASTA 실측 vs 이론식(27)(28). 대표 4점."""
import csv, math, os, sys
HERE=os.path.dirname(os.path.abspath(__file__)); ROOT=os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0,ROOT)
import masta_fatigue as mf
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

MODEL=(r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
       r"\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_유연체_FE_온도_50도_260721.Masta")
NU,DPW=294.637,3328.6
data=[{k:float(v) for k,v in r.items()} for r in csv.DictReader(
    open(os.path.join(HERE,"DLC6.4-a-s3","raw.csv"),encoding="utf-8-sig"))]
rpms=[r["rpm"] for r in data]
# 대표 점: rpm 최대 / 평균 근접 / 극저속(+) / 음수 최소
import bisect
idx_max=rpms.index(max(rpms)); idx_min=rpms.index(min(rpms))
mean=sum(rpms)/len(rpms)
idx_mean=min(range(len(rpms)),key=lambda i:abs(rpms[i]-mean))
pos=[ (abs(r-0.005),i) for i,r in enumerate(rpms) if 0<r<0.02]
idx_low=min(pos)[1] if pos else idx_mean
PICKS=[("rpm최대",idx_max),("평균급",idx_mean),("극저속",idx_low),("음수rpm",idx_min)]

design=Design.load(MODEL); asm=design.all_parts_of_type_root_assembly()[0]
dp=asm.design_properties
pl=list(asm.all_parts_of_type_point_load())[0]
ipl=next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
uw=next(b for b in asm.all_parts_of_type_bearing() if "UW" in str(b))
lc0=next(c for c in dp.static_loads if c.name=="Load Case 1")
print(f"{'구분':8}{'rpm':>10}{'κ이론(식28)':>12}{'κ MASTA':>10}{'ν1 MASTA':>12}{'a_ISO(281)':>11}{'L10mr[rev]':>12}")
for tag,i in PICKS:
    rec=data[i]
    mf.set_loads(lc0,pl,ipl,rec)
    sd=lc0.analysis_of(AnalysisType.SYSTEM_DEFLECTION); sd.perform_analysis()
    d=sd.results_for(uw).component_detailed_analysis
    iso=d.iso2812007
    km=getattr(iso,"viscosity_ratio",None)
    n1=getattr(iso,"reference_kinematic_viscosity",None)
    n1=n1*1e6 if n1 else None
    try: a=iso.modified_rating_life_cycles/iso.basic_rating_life_cycles
    except Exception: a=float("nan")
    try: lmr=d.isots162812008.modified_reference_rating_life_cycles
    except Exception: lmr=float("nan")
    r=rec["rpm"]
    kt=NU/(45000.0*abs(r)**-0.83*DPW**-0.5) if abs(r)>1e-9 else 0.0
    print(f"{tag:8}{r:>10.4f}{kt:>12.4f}{km if km is not None else float('nan'):>10.4f}"
          f"{n1 if n1 else float('nan'):>12.1f}{a:>11.4f}{lmr:>12.3e}")
print("\n이론: ν1=45000·|n|^-0.83·Dpw^-0.5, κ=ν/ν1 (ν=294.637, Dpw=3328.6) · ISO 유효범위 κ≥0.1")
