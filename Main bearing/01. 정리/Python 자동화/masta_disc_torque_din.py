"""탐색: (1) Input Power Load 토크 인가, (2) 샤프트 DIN743 무한수명 안전율 API."""
import math
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

MODEL = r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_260720.Masta"
DLC   = r"D:\AI\AI_Seminar\Main bearing\02. 자료\260714 유니슨 피로하중\DLC1.2-c-s1.$150"
RPM2RADS = 2 * math.pi / 60


def safe(o, n):
    try:
        return getattr(o, n)
    except Exception as e:
        return f"<ERR {str(e).splitlines()[0][:32]}>"


def dump(o, only=None, ind="    "):
    for n in sorted(dir(o)):
        if n.startswith("_"):
            continue
        if only and not any(k in n.lower() for k in only):
            continue
        v = safe(o, n)
        cal = callable(v)
        s = "" if cal else repr(v)
        if len(s) > 58:
            s = s[:55] + "..."
        print(f"{ind}{n}: {type(v).__name__} {'()' if cal else '= '+s}")


def parse1(path, idx):
    rows = []
    with open(path, "r", encoding="latin-1") as f:
        for ln in f.readlines()[4:]:
            p = ln.split()
            if len(p) < 8:
                continue
            try:
                rows.append([float(x) for x in p[:8]])
            except ValueError:
                pass
    v = rows[idx]
    return dict(t=v[0], rpm=v[1], Mx=v[2], My=v[3], Mz=v[4], Fx=v[5], Fy=v[6], Fz=v[7])


design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]
pl = list(assembly.all_parts_of_type_point_load())[0]
bearings = list(assembly.all_parts_of_type_bearing())
shaft = list(assembly.all_parts_of_type_shaft())[0]
ipl = next(p for p in assembly.all_parts_of_type_power_load() if "input" in str(p).lower())
opl = next(p for p in assembly.all_parts_of_type_power_load() if "output" in str(p).lower())
lc = next(c for c in assembly.design_properties.static_loads if getattr(c, "name", "") == "Load Case 1")

print("=" * 72)
print("(1) Input Power Load 로드케이스 — torque/speed 관련")
print("=" * 72)
pll = lc.inputs_for_power_load(ipl)
dump(pll, ("torque", "speed", "power", "input_method", "target"))

rec = parse1(DLC, 3000)   # 고토크 대표점
print(f"\n[torque+speed 동시 설정 시험] 파일 Mx(토크)={rec['Mx']:.0f} kNm")
p = lc.inputs_for_point_load(pl)
p.force_x.force = -rec["Fz"] * 1e3; p.force_y.force = rec["Fy"] * 1e3; p.axial_load.force = rec["Fx"] * 1e3
p.moment_x.moment = -rec["Mz"] * 1e3; p.moment_y.moment = rec["My"] * 1e3
pll.speed = rec["rpm"] * RPM2RADS
try:
    pll.torque = rec["Mx"] * 1e3
    print("  ipl.torque 설정 OK →", safe(pll, "torque"))
except Exception as e:
    print("  ipl.torque 설정 실패:", str(e).splitlines()[0])
print("  ipl.speed =", safe(pll, "speed"), " torque_input_method =", safe(pll, "torque_input_method"))

try:
    sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
    sd.perform_analysis()
    print("  → 해석 OK (torque+speed 공존 가능)")
except Exception as e:
    print("  → 해석 예외:", str(e).splitlines()[0])
    sd = None

print("\n" + "=" * 72)
print("(2) 샤프트 결과 — DIN743 / 안전율 / 무한수명")
print("=" * 72)
if sd is not None:
    sres = sd.results_for(shaft)
    print("  results_for(shaft) type:", type(sres).__name__)
    dump(sres, ("din", "743", "safety", "fatigue", "infinite", "rating",
                "section", "endurance", "damage", "factor"))
    # 섹션/레이팅 컬렉션 탐색
    for attr in ("shaft_section_results", "section_results", "rating",
                 "shaft_rating", "din743", "fatigue_safety_factor"):
        v = safe(sres, attr)
        print(f"  sres.{attr} = {v!r:.60}")
        if not isinstance(v, str) and v is not None and not callable(v):
            try:
                lst = list(v)
                print(f"     → {len(lst)}개, 예시 dump:")
                if lst:
                    dump(lst[0], ("din", "safety", "fatigue", "infinite", "factor"), ind="       ")
            except Exception:
                dump(v, ("din", "safety", "fatigue", "infinite", "factor"), ind="       ")
print("\n완료")
