"""
로드케이스 입력 API 탐색 (읽기전용)
====================================
목적: Point Load 에 6분력(Fx/Fy/Fz/Mx/My/Mz)을 넣고 로터 속도를 세팅하는
      mastapy API 경로를 찾는다. 시계열 하중 반복해석 계획의 근거 확보용.
"""
import masta_clr_legacy  # noqa: F401
import mastapy
MASTA_DIR = r"C:\Program Files\SMT\MASTA 14.1.1"
mastapy.init(MASTA_DIR)
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

MODEL = r"D:\AI\AI_Seminar\Main bearing\02. 자료\260720 검증용 자료_롤러 프로파일\26MW_메인베어링_기본설계_v1.2_샤프트 두께,형상 2안_강체,롤러프로파일만_260720.Masta"


def hr(t=""):
    print("\n" + "=" * 70)
    if t:
        print(t); print("-" * 70)


def dump(obj, maxn=200, indent="  ", only=None):
    for name in dir(obj):
        if name.startswith("_"):
            continue
        try:
            v = getattr(obj, name)
        except Exception as e:
            print(f"{indent}{name}: <err {e}>")
            continue
        callable_ = callable(v)
        tn = type(v).__name__
        if only and not any(k in name.lower() for k in only):
            continue
        s = "" if callable_ else repr(v)
        if len(s) > 70:
            s = s[:67] + "..."
        tag = "()" if callable_ else f"= {s}"
        print(f"{indent}{name}: {tn} {tag}")


design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]

# Point Load 파트 찾기
hr("A. Root assembly 파트 & Point Load 탐색")
point_load = None
for getter in dir(assembly):
    if getter.startswith("all_parts_of_type_point_load"):
        try:
            items = list(getattr(assembly, getter)())
        except Exception:
            items = []
        if items:
            point_load = items[0]
            print(f"  [{getter}] → {len(items)}개; 첫: {point_load}")
if point_load is None:
    # 이름으로 fallback
    for g in ("all_parts_of_type_component", "components"):
        try:
            for c in getattr(assembly, g)():
                if "point" in str(c).lower():
                    point_load = c; print("  이름매칭 point load:", c); break
        except Exception:
            pass
        if point_load:
            break
print("  point_load type:", type(point_load).__name__ if point_load else None)

# 속도/파워 관련 파트도 탐색
hr("B. 속도 관련 파트(power load / speed) 탐색")
for getter in dir(assembly):
    if getter.startswith("all_parts_of_type_") and any(
            k in getter for k in ("power", "shaft", "load")):
        try:
            items = list(getattr(assembly, getter)())
        except Exception:
            items = []
        if items:
            print(f"  [{getter}] → {[str(x) for x in items]}")

# 정적 로드케이스 하나
lc = list(design.design_properties.static_loads if hasattr(design, "design_properties")
          else assembly.design_properties.static_loads)[0]
hr(f"C. static load case: {lc} / type={type(lc).__name__}")
print("  ── load/point/force/moment/speed/for 관련 멤버 ──")
dump(lc, only=("load", "point", "force", "moment", "speed", "for", "input"))

# 로드케이스에서 point_load 의 입력 얻기 시도
hr("D. 로드케이스에서 Point Load 입력 객체 얻기 시도")
cand_methods = [m for m in dir(lc) if ("for" in m.lower() or "load_case" in m.lower())
                and callable(getattr(lc, m, None))]
print("  후보 메서드:", cand_methods)
pl_input = None
for m in ("static_load_for", "load_case_for", "inputs_for", "load_for"):
    fn = getattr(lc, m, None)
    if fn and point_load is not None:
        try:
            pl_input = fn(point_load)
            print(f"  [OK] lc.{m}(point_load) → {type(pl_input).__name__}")
            break
        except Exception as e:
            print(f"  lc.{m}(point_load) 실패: {e}")

if pl_input is not None:
    hr(f"E. Point Load 입력 객체 전체 덤프: {type(pl_input).__name__}")
    dump(pl_input, only=("force", "moment", "specif", "fx", "fy", "fz",
                         "mx", "my", "mz", "magnitude", "component", "angular"))
    print("\n  ── 전체 속성(참고) ──")
    dump(pl_input)

hr("완료")
