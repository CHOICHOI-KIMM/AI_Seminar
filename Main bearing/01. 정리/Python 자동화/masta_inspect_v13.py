"""v1.3 피로하중 반영 모델 구조 조사 — 로드케이스 컨테이너 & Point Load 구성."""
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design

MODEL = r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_260720.Masta"


def hr(t=""):
    print("\n" + "=" * 70)
    if t:
        print(t); print("-" * 70)


def safe(o, n):
    try:
        return getattr(o, n)
    except Exception as e:
        return f"<err {str(e).splitlines()[0][:45]}>"


design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]
dp = assembly.design_properties

hr("1. 로드케이스 컨테이너 개수/이름")
for name in ("static_loads", "duty_cycles", "design_states",
             "time_series_load_cases", "load_cases", "analysis_cases"):
    v = safe(dp, name)
    if isinstance(v, str) and v.startswith("<err"):
        v2 = safe(design, name)
        v = v2 if not (isinstance(v2, str) and v2.startswith("<err")) else v
    try:
        lst = list(v)
        print(f"  {name}: {len(lst)}개 -> {[str(x) for x in lst][:8]}"
              + (" ..." if len(lst) > 8 else ""))
    except Exception:
        print(f"  {name}: {v}")

hr("1b. dp 의 load/time/duty/case 관련 멤버 훑기")
for n in sorted(dir(dp)):
    if n.startswith("_"):
        continue
    if any(k in n.lower() for k in ("time_series", "duty", "static_load",
                                    "design_state", "load_case")):
        v = safe(dp, n)
        tn = type(v).__name__
        extra = ""
        try:
            extra = f" (len={len(list(v))})"
        except Exception:
            pass
        print(f"  {n}: {tn}{extra}")

point_load = list(assembly.all_parts_of_type_point_load())[0]

# static load cases 의 point load 입력 상태
static = list(safe(dp, "static_loads") or [])
hr(f"2. static_loads {len(static)}개 — Point Load 입력 타입 점검")
for lc in static[:20]:
    pli = lc.inputs_for_point_load(point_load)
    fx = pli.force_x
    it = safe(fx, "input_type")
    fval = safe(fx, "force")
    tsl = safe(pli, "time_series_load_case")
    print(f"  [{getattr(lc,'name','?'):16}] force_x.input_type={it} "
          f"force={fval} time_series_lc={type(tsl).__name__ if not isinstance(tsl,str) else tsl}")
if len(static) > 20:
    print(f"  ... (총 {len(static)}개)")

# 시계열 로드케이스가 있으면 파고들기
hr("3. Time Series Load Case 탐색")
tsl_list = safe(dp, "time_series_load_cases")
if isinstance(tsl_list, str) or tsl_list is None:
    print("  dp.time_series_load_cases 접근 불가:", tsl_list)
else:
    try:
        tsls = list(tsl_list)
        print(f"  {len(tsls)}개")
        for t in tsls[:5]:
            print("   -", t, "|", type(t).__name__)
    except Exception as e:
        print("  열거 실패:", e)

# value_vs_time 이 설정된 케이스가 있는지 (피로 시계열이 point load에 들어갔는지)
hr("4. Point Load 에 시계열(value_vs_time) 입력 여부 — 첫 static case")
if static:
    pli = static[0].inputs_for_point_load(point_load)
    for comp_name in ("force_x", "force_y", "axial_load", "moment_x", "moment_y"):
        comp = safe(pli, comp_name)
        if isinstance(comp, str):
            print(f"  {comp_name}: {comp}"); continue
        print(f"  {comp_name}: input_type={safe(comp,'input_type')} "
              f"value(force/moment)={safe(comp,'force') if 'force' in comp_name or comp_name=='axial_load' else safe(comp,'moment')}")

hr("완료")
