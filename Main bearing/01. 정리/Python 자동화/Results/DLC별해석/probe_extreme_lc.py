"""_Test 모델 극한하중 로드케이스 검토 — 엑셀(ExtremeLoads) 대조
좌표 변환 기대식(§4.2): FX=-Fz·1e3, FY=+Fy·1e3, AX=+Fx·1e3, MX=-Mz·1e3, MY=+My·1e3, TQ=+Mx·1e3
SF(1.1/1.35) 적용 여부 자동 판별(raw/×SF 둘 다 대조)"""
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
         r"\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안"
         r"_피로하중 반영_유연체_FE_온도_50도_260721_Test.Masta")

# 엑셀 ExtremeLoads (kN·kNm, 파일좌표) — (지표, Max/Min, DLC, Mx, My, Mz, Fx, Fy, Fz, SF)
EXCEL = [
    ("Mx",  "Max", "DLC1.3-k-s5",       61192,   37757,   51424,  3274.4,  -386.6, -6975.4, 1.35),
    ("Mx",  "Min", "DLC5.1-c-s09",     -22925,  -32745,   -8806,  4962.4,  -249.0, -6651.8, 1.35),
    ("My",  "Max", "DLC2.2.4-b-s09",    23670,  139746,  6686.6,   549.5,  -188.4, -5414.6, 1.10),
    ("My",  "Min", "DLC2.2.4-b-s11",    27453, -190493, -7340.3,  1841.8,   413.1, -5190.0, 1.10),
    ("Mz",  "Max", "DLC2.2.4-b-s11",    10308,  -735.3,  185249,   972.3,  -303.3, -6062.3, 1.10),
    ("Mz",  "Min", "DLC2.2.4-b-s07",    17013,  -52162, -183500,  -972.9,   231.5, -5286.7, 1.10),
    ("Myz", "Max", "DLC2.2.4-b-s04",    22673, -196996,  -49588,   544.2,   230.3, -5268.1, 1.10),
    ("Myz", "Min", "DLC1.5-d-nv-y1-r1", 58208,    0.84,   -21.4,  2662.2,  -142.1, -6652.3, 1.35),
    ("Fx",  "Max", "DLC2.1.1-b-s06",    51143,  -54956,   17904,  7235.5,   416.3, -6734.5, 1.35),
    ("Fx",  "Min", "DLC2.3-b-y3-g1-r2",  90.3,  -21879,   -1917, -3825.7,   -97.8, -5429.5, 1.10),
    ("Fy",  "Max", "DLC6.2-y10-s6",     -40.3,  -20224,  -14206,   348.8,  2579.0, -5058.4, 1.10),
    ("Fy",  "Min", "DLC6.2-y04-s3",      33.3,  -42039,   15795,   329.1, -2538.0, -5320.3, 1.10),
    ("Fz",  "Max", "DLC6.2-y01-s6",      -2.0, -1062.9,   31221,   -35.6,  -251.6, -2299.1, 1.10),
    ("Fz",  "Min", "DLC1.3-k-s4",       61059,  -37291,   69956,  2513.3,   -51.2, -7591.4, 1.35),
    ("Fyz", "Max", "DLC1.3-k-s4",       55311,   33079,   46422,  2575.3,  -688.1, -7573.1, 1.35),
    ("Fyz", "Min", "DLC6.2-y01-s6",      -2.0, -1062.9,   31221,   -35.6,  -251.6, -2299.1, 1.10),
]


def expected(Mx, My, Mz, Fx, Fy, Fz, s=1.0):
    """파일좌표 kN·kNm → MASTA 입력 [N, N·m] (§4.2 변환)."""
    return dict(FX=-Fz * 1e3 * s, FY=Fy * 1e3 * s, AX=Fx * 1e3 * s,
                MX=-Mz * 1e3 * s, MY=My * 1e3 * s, TQ=Mx * 1e3 * s)


def g(o, path, default=None):
    cur = o
    for p in path.split("."):
        try:
            cur = getattr(cur, p)
        except Exception:
            return default
        if cur is None:
            return default
    return cur


design = Design.load(MODEL)
asm = design.all_parts_of_type_root_assembly()[0]
pl = list(asm.all_parts_of_type_point_load())[0]
ipl = next(p for p in asm.all_parts_of_type_power_load()
           if "input" in str(p).lower())
cases = list(asm.design_properties.static_loads)
print(f"[모델] 로드케이스 {len(cases)}개")

mrows = []
for lc in cases:
    nm = getattr(lc, "name", "?")
    p = lc.inputs_for_point_load(pl)
    pw = lc.inputs_for_power_load(ipl)
    m = dict(name=nm,
             FX=g(p, "force_x.force"), FY=g(p, "force_y.force"),
             AX=g(p, "axial_load.force"),
             MX=g(p, "moment_x.moment"), MY=g(p, "moment_y.moment"),
             TQ=g(pw, "torque"), SPD=g(pw, "speed"))
    mrows.append(m)
    print(f"  {nm:24} FX={m['FX']:>14,.0f} FY={m['FY']:>12,.0f} AX={m['AX']:>13,.0f} "
          f"MX={m['MX']:>14,.0f} MY={m['MY']:>14,.0f} TQ={m['TQ']:>14,.0f} "
          f"SPD={m['SPD'] if m['SPD'] is not None else '-'}")

print("\n" + "=" * 110)
print("대조 — 각 MASTA 케이스를 엑셀 16건(raw / ×SF)과 매칭 (성분별 최대 상대오차 최소 기준)")
print("=" * 110)
KEYS = ("FX", "FY", "AX", "MX", "MY", "TQ")
for m in mrows:
    if m["FX"] is None:
        continue
    best = None
    for (idx, mm, dlc, Mx, My, Mz, Fx, Fy, Fz, sf) in EXCEL:
        for tag, s in (("raw", 1.0), (f"×SF{sf:g}", sf)):
            e = expected(Mx, My, Mz, Fx, Fy, Fz, s)
            errs = []
            for kk in KEYS:
                ref = e[kk]
                got = m[kk]
                den = max(abs(ref), 1e3)
                errs.append(abs((got or 0) - ref) / den * 100)
            worst = max(errs)
            if best is None or worst < best[0]:
                best = (worst, f"{idx}-{mm}", dlc, tag, errs, e)
    worst, lab, dlc, tag, errs, e = best
    ok = "✅" if worst < 0.5 else ("⚠️" if worst < 5 else "❌")
    print(f"\n[{m['name']}] ↔ {lab} ({dlc}, {tag})  최대오차 {worst:.2f}% {ok}")
    if worst >= 0.5:
        for kk, er in zip(KEYS, errs):
            print(f"    {kk}: MASTA {m[kk]:>15,.0f}  기대 {e[kk]:>15,.0f}  오차 {er:.2f}%")
print("\n완료")
