"""FE 모델 하중경로 진단: 인가하중 vs 베어링 반력(내부힘) 비교."""
import math
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

MODELS = {
    "강체(비-FE)": r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_260720.Masta",
    "FE": r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_유연체_FE_260721.Masta",
}
RPM2RADS = 2 * math.pi / 60
# DLC1.2-c-s1 index 0
REC = dict(rpm=5.09167, Mx=23992.2, My=-13666.5, Mz=4814.75, Fx=3017.07, Fy=126.45, Fz=-4965.6)


def safe(o, n):
    try:
        return getattr(o, n)
    except Exception as e:
        return f"<ERR {str(e).splitlines()[0][:26]}>"


def g(o, path):
    """점(.) 경로 체인 getter → 실패 시 None."""
    cur = o
    for part in path.split("."):
        try:
            cur = getattr(cur, part)
        except Exception:
            return None
        if cur is None:
            return None
    return cur


def fnum(v, default=float("nan")):
    return v if isinstance(v, (int, float)) else default


def vec3(v):
    try:
        return float(v.x), float(v.y), float(v.z)
    except Exception:
        return None


for tag, path in MODELS.items():
    print("\n" + "=" * 66)
    print(f"[{tag}]")
    print("=" * 66)
    design = Design.load(path)
    asm = design.all_parts_of_type_root_assembly()[0]
    pl = list(asm.all_parts_of_type_point_load())[0]
    bearings = list(asm.all_parts_of_type_bearing())
    shaft = list(asm.all_parts_of_type_shaft())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
    lc = next(c for c in asm.design_properties.static_loads
              if getattr(c, "name", "") == "Load Case 1")

    print("  파트:", [str(x) for x in asm.all_parts_of_type_component()])
    print("  shaft.is_replaced_by_fe =", safe(shaft, "is_replaced_by_fe"))
    try:
        print("  FE 파트:", [str(x) for x in asm.all_parts_of_type_fe_part()])
    except Exception:
        pass

    p = lc.inputs_for_point_load(pl)
    fx = -REC["Fz"] * 1e3; fy = REC["Fy"] * 1e3; fa = REC["Fx"] * 1e3
    p.force_x.force = fx; p.force_y.force = fy; p.axial_load.force = fa
    p.moment_x.moment = -REC["Mz"] * 1e3; p.moment_y.moment = REC["My"] * 1e3
    pll = lc.inputs_for_power_load(ipl)
    pll.speed = REC["rpm"] * RPM2RADS
    pll.torque = REC["Mx"] * 1e3

    sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
    sd.perform_analysis()

    print(f"\n  인가 하중: Fx(반경X)={fx/1e3:9.1f} kN  Fy={fy/1e3:8.1f} kN  축={fa/1e3:8.1f} kN")
    sx = sy = sz = 0.0
    for b in bearings:
        res = sd.results_for(b)
        f = vec3(safe(res, "internal_force"))
        d = safe(res, "component_detailed_analysis")
        sig = fnum(g(d, "maximum_normal_stress_inner")) / 1e6
        s0 = fnum(g(d, "iso762006.safety_factor"))
        if f:
            sx += f[0]; sy += f[1]; sz += f[2]
            fr = math.hypot(f[0], f[1])
            print(f"  {str(b):18} 반력 Fr={fr/1e3:9.1f} kN  Fa={f[2]/1e3:8.1f} kN "
                  f"| σin={sig:7.1f} MPa  s0={s0:8.2f}")
        else:
            print(f"  {str(b):18} 반력 없음 | σin={sig:7.1f} MPa  s0={s0:8.2f}")
    print(f"  베어링 반력 합계: X={sx/1e3:9.1f} kN  Y={sy/1e3:8.1f} kN  Z(축)={sz/1e3:8.1f} kN")
    print(f"  → 반경 X 전달률 = {abs(sx/fx)*100:5.1f}%   축 전달률 = {abs(sz/fa)*100:5.1f}%")

print("\n완료")
