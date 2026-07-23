"""50도 모델의 윤활 조건 확인 — 80도 모델 대비 kappa/a_ISO 영향 정량화."""
import math
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

BASE = r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
MODELS = {
    "80도(기존)": BASE + r"\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_유연체_FE_260721.Masta",
    "50도(신규)": BASE + r"\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_유연체_FE_온도_50도_260721.Masta",
}
RPM2RADS = 2 * math.pi / 60
REC = dict(rpm=5.09167, Mx=23992.2, My=-13666.5, Mz=4814.75, Fx=3017.07, Fy=126.45, Fz=-4965.6)
KEYS = ("viscosity_ratio", "reference_kinematic_viscosity", "contamination_factor",
        "lubricant_film_temperature", "kinematic_viscosity")

for tag, path in MODELS.items():
    print("\n" + "=" * 62); print(f"[{tag}]"); print("=" * 62)
    d = Design.load(path)
    asm = d.all_parts_of_type_root_assembly()[0]
    pl = list(asm.all_parts_of_type_point_load())[0]
    bs = list(asm.all_parts_of_type_bearing())
    ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
    lc = next(c for c in asm.design_properties.static_loads if getattr(c, "name", "") == "Load Case 1")
    p = lc.inputs_for_point_load(pl)
    p.force_x.force = -REC["Fz"]*1e3; p.force_y.force = REC["Fy"]*1e3
    p.axial_load.force = REC["Fx"]*1e3
    p.moment_x.moment = -REC["Mz"]*1e3; p.moment_y.moment = REC["My"]*1e3
    pll = lc.inputs_for_power_load(ipl); pll.speed = REC["rpm"]*RPM2RADS; pll.torque = REC["Mx"]*1e3
    sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION); sd.perform_analysis()
    b = bs[0]; res = sd.results_for(b); det = getattr(res, "component_detailed_analysis", None)
    for src, lab in ((res, "res"), (det, "det"), (getattr(det, "iso2812007", None), "iso281")):
        if src is None: continue
        for n in dir(src):
            if n.startswith("_") or not any(k in n for k in KEYS): continue
            try:
                v = getattr(src, n)
                if not callable(v): print(f"  {lab}.{n:44} = {v}")
            except Exception: pass
    # 수명
    for bb in bs:
        dd = sd.results_for(bb).component_detailed_analysis
        try:
            l10 = dd.iso2812007.basic_rating_life_cycles
            l10m = dd.iso2812007.modified_rating_life_cycles
            print(f"  {str(bb):18} L10={l10:.4e}  L10m={l10m:.4e}  a_ISO={l10m/l10:.5f}")
        except Exception as e: print("  ", e)
print("\n완료")
