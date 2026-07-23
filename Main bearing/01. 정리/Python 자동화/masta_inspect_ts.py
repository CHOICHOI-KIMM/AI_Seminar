"""v1.3 Time Series Load Case Group + Fatigue design state 구조 조사."""
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
        return f"<err {str(e).splitlines()[0][:50]}>"


def dump(obj, indent="  ", only=None):
    for name in sorted(dir(obj)):
        if name.startswith("_"):
            continue
        v = safe(obj, name)
        if only and not any(k in name.lower() for k in only):
            continue
        if isinstance(v, str) and v.startswith("<err"):
            if only:
                print(f"{indent}{name}: {v}")
            continue
        cal = callable(v)
        tn = type(v).__name__
        extra = ""
        if not cal:
            try:
                extra = f" (len={len(list(v))})" if not isinstance(v, str) else ""
            except Exception:
                pass
        s = "" if cal else repr(v)
        if len(s) > 66:
            s = s[:63] + "..."
        print(f"{indent}{name}: {tn}{extra} {'()' if cal else '= ' + s}")


design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]
dp = assembly.design_properties
point_load = list(assembly.all_parts_of_type_point_load())[0]

hr("A. time_series_load_case_groups[0] 전체 덤프")
tsg = list(dp.time_series_load_case_groups)[0]
print("  type:", type(tsg).__name__, "| name:", safe(tsg, "name"))
dump(tsg)

hr("B. tsg 의 load case / time / point 관련")
dump(tsg, only=("load_case", "time", "point", "step", "duration",
                "static", "count", "number", "speed", "rpm"))

# 그룹 안의 개별 로드케이스(=시간지점?) 탐색
hr("C. tsg 안의 로드케이스 컬렉션 찾기")
for attr in ("load_cases", "static_loads", "time_series_load_cases",
             "load_cases_in_group", "time_steps", "cases"):
    v = safe(tsg, attr)
    if isinstance(v, str):
        continue
    try:
        lst = list(v)
        print(f"  tsg.{attr}: {len(lst)}개; 예: {[str(x) for x in lst[:5]]}")
    except Exception:
        pass

hr("D. Fatigue load case design_state 조사")
fatigue_ds = None
for ds in dp.design_states:
    if "fatig" in str(ds).lower():
        fatigue_ds = ds
print("  fatigue design state:", fatigue_ds)
if fatigue_ds is not None:
    dump(fatigue_ds, only=("load_case", "static", "time", "count", "number"))
    for attr in ("static_loads", "load_cases"):
        v = safe(fatigue_ds, attr)
        try:
            lst = list(v)
            print(f"  {attr}: {len(lst)}개; 예: {[str(x) for x in lst[:6]]}")
        except Exception:
            pass

hr("E. 시계열 point load 입력 — value_vs_time 이 설정됐나?")
# tsg 의 첫 로드케이스에서 point load 입력 확인
try:
    first_case = None
    for attr in ("load_cases", "static_loads", "time_series_load_cases"):
        v = safe(tsg, attr)
        try:
            lst = list(v)
            if lst:
                first_case = lst[0]; break
        except Exception:
            pass
    print("  first_case:", first_case, type(first_case).__name__ if first_case else None)
    if first_case is not None and hasattr(first_case, "inputs_for_point_load"):
        pli = first_case.inputs_for_point_load(point_load)
        for c in ("force_x", "axial_load", "moment_x"):
            comp = safe(pli, c)
            print(f"    {c}.input_type = {safe(comp,'input_type')}")
except Exception as e:
    print("  실패:", str(e).splitlines()[0])

hr("완료")
