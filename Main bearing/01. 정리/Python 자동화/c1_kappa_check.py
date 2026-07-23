"""
MASTA가 a_ISO에 쓰는 κ(점도비)의 정의 확인
==========================================
ISO 281 원문 두 경로:
  (27) κ = ν / ν₁ ,  ν₁ = 45000·n^(−0.83)·D_pw^(−0.5)   [n<1000 r/min]
  (30) κ ≈ Λ^1.3   (Λ = 유막 파라미터, 실제 유막두께/복합조도 기반)
MASTA API가 보고하는 점도비·유막 관련 값을 모두 덤프하여 어느 쪽인지 판정.
"""
import math
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3"
         r"_샤프트 두께,형상 2안_피로하중 반영_유연체_FE_260721.masta")
RPM2RADS = 2 * math.pi / 60
# DLC1.2-c-s1 index 0
REC = dict(rpm=5.09167, Mx=23992.2, My=-13666.5, Mz=4814.75,
           Fx=3017.07, Fy=126.45, Fz=-4965.6)
NU, DPW_MM = 98.7, 3330.0

KEYS = ("viscosity", "kappa", "lambda", "film", "lubric", "ratio", "oil",
        "thickness", "roughness", "temperature", "speed", "diameter", "pitch")


def dump(obj, label, depth=0):
    pad = "  " * (depth + 1)
    print(f"{pad}[{label}] {type(obj).__name__}")
    for n in sorted(dir(obj)):
        if n.startswith("_"):
            continue
        if not any(k in n.lower() for k in KEYS):
            continue
        try:
            v = getattr(obj, n)
        except Exception as e:
            v = f"<ERR {str(e).splitlines()[0][:40]}>"
        if callable(v):
            continue
        print(f"{pad}  {n:52} = {v}")


design = Design.load(MODEL)
asm = design.all_parts_of_type_root_assembly()[0]
pl = list(asm.all_parts_of_type_point_load())[0]
bearings = list(asm.all_parts_of_type_bearing())
ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
lc = next(c for c in asm.design_properties.static_loads
          if getattr(c, "name", "") == "Load Case 1")

p = lc.inputs_for_point_load(pl)
p.force_x.force = -REC["Fz"] * 1e3
p.force_y.force = REC["Fy"] * 1e3
p.axial_load.force = REC["Fx"] * 1e3
p.moment_x.moment = -REC["Mz"] * 1e3
p.moment_y.moment = REC["My"] * 1e3
pll = lc.inputs_for_power_load(ipl)
pll.speed = REC["rpm"] * RPM2RADS
pll.torque = REC["Mx"] * 1e3

sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
sd.perform_analysis()

rpm = REC["rpm"]
nu1 = 45000.0 * (rpm ** -0.83) * (DPW_MM ** -0.5)
print("=" * 74)
print(f"[이론] rpm={rpm}  D_pw={DPW_MM} mm  ν={NU} mm²/s")
print(f"  식(28) ν₁ = {nu1:.4f} mm²/s")
print(f"  식(27) κ = ν/ν₁ = {NU/nu1:.5f}")
print("=" * 74)

for b in bearings:
    print(f"\n{'='*74}\n[{b}]\n{'='*74}")
    res = sd.results_for(b)
    d = getattr(res, "component_detailed_analysis", None)
    for obj, lab in ((res, "BearingSystemDeflection"), (d, "DetailedAnalysis")):
        if obj is None:
            continue
        dump(obj, lab)
        for sub in ("iso2812007", "isots162812008", "lubrication", "bearing_design"):
            o = getattr(obj, sub, None)
            if o is not None and not isinstance(o, (str, int, float)):
                try:
                    dump(o, f"{lab}.{sub}", 1)
                except Exception:
                    pass

print("\n완료")
