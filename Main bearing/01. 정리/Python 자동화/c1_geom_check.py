"""
G-8.2b: 실제 베어링 축위치 + TRB 유효하중중심 + 중력 확인
=========================================================
가정 기하(z_UW=0.5, z_DW=3.0 → L=2.5)로 계산한 Fr 이 MASTA 반력 대비 UW +29%, DW +81% 과대.
§C-3.6 회귀 M_Y 계수 0.277 → L_eff ≈ 3.61 m 를 시사. 실제 값을 조회하여 원인 규명.
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
REC = dict(rpm=5.09167, Mx=23992.2, My=-13666.5, Mz=4814.75,
           Fx=3017.07, Fy=126.45, Fz=-4965.6)

GEO = ("offset", "position", "axial", "centre", "center", "width", "distance",
       "contact_angle", "effective", "apex", "load_centre", "mass", "diameter")


def safe(o, n):
    try:
        v = getattr(o, n)
        return None if callable(v) else v
    except Exception as e:
        return f"<ERR {str(e).splitlines()[0][:34]}>"


def dump(o, lab, keys=GEO, ind="    "):
    print(f"{ind}[{lab}] {type(o).__name__}")
    for n in sorted(dir(o)):
        if n.startswith("_") or not any(k in n.lower() for k in keys):
            continue
        v = safe(o, n)
        if v is not None:
            print(f"{ind}  {n:46} = {v}")


design = Design.load(MODEL)
asm = design.all_parts_of_type_root_assembly()[0]
pl = list(asm.all_parts_of_type_point_load())[0]
bearings = list(asm.all_parts_of_type_bearing())
shaft = list(asm.all_parts_of_type_shaft())[0]
ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
lc = next(c for c in asm.design_properties.static_loads
          if getattr(c, "name", "") == "Load Case 1")

print("=" * 74)
print("[1] 파트 축방향 위치")
print("=" * 74)
for part in [pl] + bearings:
    print(f"  {str(part):22}")
    for attr in ("offset", "axial_offset", "position"):
        v = safe(part, attr)
        if v is not None:
            print(f"      {attr:20} = {v}")
    for cn in ("connections", "coordinate_system"):
        pass
# 마운팅 위치는 shaft 상의 socket/offset 으로도 조회
print("\n[2] 샤프트 상 마운트 offset")
for m in ("mounted_components", "components_with_offsets"):
    v = safe(shaft, m)
    print(f"  shaft.{m} = {v}")
try:
    for c in shaft.mounted_components:
        print(f"    {str(c):24} offset={safe(c,'offset')}")
except Exception as e:
    print("   ", e)

print("\n" + "=" * 74)
print("[3] 베어링 상세 기하 (유효하중중심/접촉각)")
print("=" * 74)
for b in bearings:
    print(f"\n  --- {b} ---")
    bd = safe(b, "bearing_detail") or safe(b, "detail")
    if bd is not None and not isinstance(bd, str):
        dump(bd, "BearingDesign")
        for sub in ("left_element_inner_ring", "elements", "rows"):
            o = safe(bd, sub)
            if o is not None and not isinstance(o, (str, int, float)):
                try:
                    dump(o, f"BearingDesign.{sub}", ind="      ")
                except Exception:
                    pass

print("\n" + "=" * 74)
print("[4] 해석 1점: 반력 성분 + 중력 확인")
print("=" * 74)
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

FX, FY = -REC["Fz"] * 1e3, REC["Fy"] * 1e3
MX, MY = -REC["Mz"] * 1e3, REC["My"] * 1e3
print(f"  인가: F_X={FX/1e3:9.1f} F_Y={FY/1e3:8.1f} kN   "
      f"M_X={MX/1e3:9.1f} M_Y={MY/1e3:9.1f} kN·m")
sx = sy = 0.0
comp = {}
for b in bearings:
    res = sd.results_for(b)
    f = safe(res, "internal_force")
    try:
        x, y, z = float(f.x), float(f.y), float(f.z)
    except Exception:
        continue
    comp[str(b)] = (x, y, z)
    sx += x; sy += y
    print(f"  {str(b):20} R_X={x/1e3:10.1f}  R_Y={y/1e3:9.1f}  R_Z={z/1e3:9.1f} kN")
print(f"  {'합계':20} ΣX={sx/1e3:10.1f}  ΣY={sy/1e3:9.1f} kN")
print(f"  힘평형 잔차: X={(sx+FX)/1e3:+.1f} / {(sx-FX)/1e3:+.1f} kN   "
      f"Y={(sy+FY)/1e3:+.1f} / {(sy-FY)/1e3:+.1f} kN")
print("    → 잔차가 0 이 아니면 중력(자중) 또는 부호규약 차이")

# 유효 스팬 역산: R_A = F·(b/L) ∓ M/L 구조 가정 시 M 계수 = 1/L_eff
names = list(comp)
if len(names) == 2:
    (xa, ya, _), (xb, yb, _) = comp[names[0]], comp[names[1]]
    print(f"\n  [역산] 두 반력의 모멘트 짝힘 성분으로 유효스팬 추정")
    print(f"    ΣX={sx/1e3:.1f} kN, 인가 F_X={FX/1e3:.1f} kN")
    print(f"    Y성분: R_A,Y={ya/1e3:.1f}, R_B,Y={yb/1e3:.1f}, ΣY={sy/1e3:.1f}, "
          f"M_X={MX/1e3:.1f} kN·m")
    dy = (ya - yb) / 2.0
    if dy:
        print(f"    짝힘 (R_A,Y−R_B,Y)/2 = {dy/1e3:.1f} kN → L_eff ≈ "
              f"{abs(MX/dy):.4f} m  (가정 2.5 m)")

print("\n완료")
