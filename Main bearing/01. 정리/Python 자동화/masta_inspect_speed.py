"""v1.3 — Power Load 존재 여부 + 속도 입력 API 확인."""
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
        s = "" if cal else repr(v)
        if len(s) > 66:
            s = s[:63] + "..."
        print(f"{indent}{name}: {tn} {'()' if cal else '= ' + s}")


design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]
dp = assembly.design_properties

hr("1. Power Load 파트 존재?")
pls = []
for g in ("all_parts_of_type_power_load",):
    v = safe(assembly, g)
    try:
        pls = list(v())
    except Exception as e:
        print("  err:", e)
print(f"  power_load 파트: {len(pls)}개 -> {[str(x) for x in pls]}")

# 전체 파트 목록도(어떤 게 추가됐나)
hr("1b. Root assembly 전체 파트")
for g in dir(assembly):
    if g.startswith("all_parts_of_type_") and g.count("_") <= 5:
        try:
            items = list(getattr(assembly, g)())
        except Exception:
            items = []
        if items:
            print(f"  [{g[len('all_parts_of_type_'):]}] {[str(x) for x in items]}")

static = list(dp.static_loads)
lc0 = static[0]
hr("2. static load case 의 input_power_load / output_power_load / power_loads")
for n in ("input_power_load", "output_power_load", "power_loads"):
    print(f"  {n}:", safe(lc0, n))

if pls:
    pl = pls[0]
    hr(f"3. inputs_for_power_load({pl}) — 속도 세터")
    pll = lc0.inputs_for_power_load(pl)
    print("  type:", type(pll).__name__)
    dump(pll, only=("speed", "angular", "velocity", "rpm", "rotation",
                    "power", "torque", "specif", "target", "input"))
    print("\n  ── 전체 속성 ──")
    dump(pll)

hr("완료")
