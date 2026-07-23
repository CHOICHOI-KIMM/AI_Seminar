"""16케이스 point-load 성분값 매핑 역산 (올바른 접근자 .force/.moment)."""
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design

MODEL = r"D:\AI\AI_Seminar\Main bearing\02. 자료\260720 검증용 자료_롤러 프로파일\26MW_메인베어링_기본설계_v1.2_샤프트 두께,형상 2안_강체,롤러프로파일만_260720.Masta"

design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]
point_load = list(assembly.all_parts_of_type_point_load())[0]
cases = list(assembly.design_properties.static_loads)


def fv(comp, attr):
    try:
        return getattr(comp, attr)
    except Exception as e:
        return f"ERR:{str(e).splitlines()[0][:30]}"


print("단위: force=N, moment=Nm.  (파일은 kN, kNm)")
print(f"{'case':10} {'force_x':>14} {'force_y':>14} {'axial':>14} "
      f"{'moment_x':>14} {'moment_y':>14} {'moment_z':>18}")
for lc in cases:
    pli = lc.inputs_for_point_load(point_load)
    fx = fv(pli.force_x, "force")
    fy = fv(pli.force_y, "force")
    ax = fv(getattr(pli, "axial_load", None), "force")
    mx = fv(pli.moment_x, "moment")
    my = fv(pli.moment_y, "moment")
    # moment_z 는 invalid 예상 — 확인
    try:
        mz = getattr(pli.moment_z, "moment")
    except Exception as e:
        mz = f"invalid({str(e).splitlines()[0][:14]})"

    def f(v):
        return f"{v:14.1f}" if isinstance(v, (int, float)) else f"{str(v):>14}"
    print(f"{getattr(lc,'name','?'):10} {f(fx)} {f(fy)} {f(ax)} "
          f"{f(mx)} {f(my)} {str(mz):>18}")

# ForceSpecification 현재 모드도 케이스별로 (혹시 다를 수 있음)
print("\nForceSpecification 현재값(첫 케이스):")
pli = cases[0].inputs_for_point_load(point_load)
try:
    print("  ", getattr(pli, "force_specification_options"))
except Exception as e:
    print("   force_specification_options invalid:", str(e).splitlines()[0][:40])
print("완료")
