"""
사이징 최적화 Step 0 — 기준 모델(v1.4 3안·베어링/롤러 확대·무프리로드·50°C)
설계변수 전수 파악: 베어링 에디터 제원 + 배치(z1·z2) + 정격 + 쓰기가능 여부
"""
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)

import masta_clr_legacy  # noqa: F401,E402
import mastapy  # noqa: E402
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design  # noqa: E402

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")


def safe(o, n):
    try:
        return getattr(o, n)
    except Exception as e:
        return f"<ERR {str(e).splitlines()[0][:40]}>"


def scal(v):
    """Overridable_* 래퍼 포함 스칼라 추출"""
    if isinstance(v, (int, float)) and not isinstance(v, bool):
        return float(v)
    for a in ("value", "wrapped"):
        w = safe(v, a)
        if isinstance(w, (int, float)) and not isinstance(w, bool):
            return float(w)
    return None


design = Design.load(MODEL)
asm = design.all_parts_of_type_root_assembly()[0]
pl = list(asm.all_parts_of_type_point_load())[0]
bearings = list(asm.all_parts_of_type_bearing())

print("=" * 80)
print("사이징 최적화 기준 모델 설계변수")
print(os.path.basename(MODEL))
print("=" * 80)

# ── 배치 ────────────────────────────────────────────────────────────────
print("\n[1] 배치 (축좌표, 축방향 = X)")
for c in [pl] + bearings:
    lcs = safe(c, "local_coordinate_system")
    org = None
    for a in ("origin", "translation", "location"):
        o = safe(lcs, a)
        if o is not None and not isinstance(o, str):
            try:
                org = tuple(round(float(x), 6) for x in list(o)[:3])
                break
            except Exception:
                pass
    print(f"  {str(c):24} origin = {org}")

# ── 베어링 제원 ─────────────────────────────────────────────────────────
GEOM = [
    ("d   Bore dia.",        ("bore", "inner_diameter", "bore_diameter")),
    ("D   Outer dia.",       ("outer_diameter",)),
    ("B   Width",            ("width",)),
    ("α   Contact angle",    ("contact_angle",)),
    ("D_we Roller dia.",     ("element_diameter",)),
    ("L_we Roller length",   ("roller_length", "element_length")),
    ("L_eff Effective len.", ("effective_roller_length",)),
    ("Z   Number of elem.",  ("number_of_elements",)),
    ("Z_max 이론최대",        ("theoretical_maximum_number_of_elements",)),
    ("PCD",                  ("pitch_circle_diameter",)),
    ("Rows",                 ("number_of_rows",)),
    ("C   기본동정격",         ("basic_dynamic_load_rating",)),
    ("Cu  피로한계하중",       ("fatigue_load_limit",)),
    ("C0  기본정정격",         ("basic_static_load_rating",)),
    ("Inner ring width",     ("inner_ring_width",)),
    ("Outer ring width",     ("outer_ring_width",)),
    ("Inner race dia.",      ("inner_race_diameter",)),
    ("Outer race dia.",      ("outer_race_diameter",)),
]

for b in bearings:
    print(f"\n[2] {b} — b.detail 제원")
    d = safe(b, "detail")
    for label, cands in GEOM:
        got, used = None, None
        for nm in cands:
            v = safe(d, nm)
            s = scal(v)
            if s is not None:
                got, used = s, nm
                break
        if label.startswith("α") and got is not None:
            print(f"  {label:22} = {got:.6f} rad = {math.degrees(got):.3f}°   [{used}]")
        else:
            print(f"  {label:22} = {got}   [{used}]")
    print(f"  preload                = {safe(b, 'preload')}")
    print(f"  axial_displacement_preload = {safe(b, 'axial_displacement_preload')}")

# ── detail 전체 속성 덤프 (쓰기가능 설계변수 후보 색출) ────────────────
b = bearings[0]
d = safe(b, "detail")
print(f"\n[3] b.detail 전체 스칼라 속성 (설계변수 후보) — {type(d).__name__}")
for name in sorted(dir(d)):
    if name.startswith("_"):
        continue
    v = safe(d, name)
    if callable(v):
        continue
    s = scal(v)
    if s is None:
        continue
    print(f"  {name:56} = {s:,.6g}")

print("\n완료")
