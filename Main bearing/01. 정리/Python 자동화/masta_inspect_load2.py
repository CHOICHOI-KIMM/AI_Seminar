"""로드케이스 입력 API 탐색 v2 — PointLoad/Shaft 입력 + 속도 + 시계열 기능."""
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design

MODEL = r"D:\AI\AI_Seminar\Main bearing\02. 자료\260720 검증용 자료_롤러 프로파일\26MW_메인베어링_기본설계_v1.2_샤프트 두께,형상 2안_강체,롤러프로파일만_260720.Masta"


def hr(t=""):
    print("\n" + "=" * 70)
    if t:
        print(t); print("-" * 70)


def dump(obj, indent="  ", only=None):
    for name in sorted(dir(obj)):
        if name.startswith("_"):
            continue
        try:
            v = getattr(obj, name)
        except Exception as e:
            es = str(e).splitlines()[0]
            if only is None or any(k in name.lower() for k in only):
                print(f"{indent}{name}: <err {es[:60]}>")
            continue
        if only and not any(k in name.lower() for k in only):
            continue
        cal = callable(v)
        tn = type(v).__name__
        s = "" if cal else repr(v)
        if len(s) > 72:
            s = s[:69] + "..."
        print(f"{indent}{name}: {tn} {'()' if cal else '= ' + s}")


design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]
point_load = list(assembly.all_parts_of_type_point_load())[0]
shaft = list(assembly.all_parts_of_type_shaft())[0]
lc = list(assembly.design_properties.static_loads)[0]
print("load case:", lc, "| point_load:", point_load, "| shaft:", shaft)

hr("E. lc.inputs_for_point_load(point_load) 전체 덤프")
pli = lc.inputs_for_point_load(point_load)
print("  type:", type(pli).__name__)
dump(pli)

hr("F. lc.inputs_for_shaft(shaft) — 속도/각속도 관련")
shi = lc.inputs_for_shaft(shaft)
print("  type:", type(shi).__name__)
dump(shi, only=("speed", "angular", "velocity", "rpm", "rotation", "rpm", "cycle"))

hr("G. lc.inputs_for_abstract_shaft_or_housing — 속도 관련")
try:
    ash = list(assembly.all_parts_of_type_abstract_shaft_or_housing())[0]
    shi2 = lc.inputs_for_abstract_shaft_or_housing(ash)
    print("  type:", type(shi2).__name__)
    dump(shi2, only=("speed", "angular", "velocity", "rpm", "rotation", "cycle"))
except Exception as e:
    print("  실패:", str(e).splitlines()[0])

hr("H. Point Load 파트 자체 — 속도/파워 관련")
dump(point_load, only=("speed", "angular", "velocity", "power", "rotation"))

hr("I. create_time_series_load_case 시그니처 확인")
import inspect as _pyinspect
f = getattr(lc, "create_time_series_load_case", None)
print("  존재:", f is not None)
try:
    print("  doc:", (f.__doc__ or "").strip()[:400])
except Exception as e:
    print("  doc err:", e)

hr("완료")
