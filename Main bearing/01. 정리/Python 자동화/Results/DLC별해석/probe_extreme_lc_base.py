"""사이징 최적화 — 기준 모델(v1.4 무프리로드)의 기존 로드케이스 전수 확인.
극한 16 케이스가 이미 들어있는지, 각 케이스의 인가 하중/속도/토크 실측."""
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
RADS2RPM = 60 / (2 * math.pi)


def safe(o, n):
    try:
        return getattr(o, n)
    except Exception as e:
        return f"<ERR {str(e).splitlines()[0][:32]}>"


def sc(o, n):
    v = safe(o, n)
    if isinstance(v, (int, float)) and not isinstance(v, bool):
        return float(v)
    for a in ("value", "wrapped"):
        w = safe(v, a)
        if isinstance(w, (int, float)) and not isinstance(w, bool):
            return float(w)
    return None


design = Design.load(MODEL)
asm = design.all_parts_of_type_root_assembly()[0]
dp = asm.design_properties
pl = list(asm.all_parts_of_type_point_load())[0]
ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())

lcs = list(dp.static_loads)
print(f"[정적 로드케이스] 총 {len(lcs)}개\n")
hdr = (f"{'name':32} {'FX[kN]':>10} {'FY[kN]':>10} {'AX[kN]':>10} "
       f"{'MX[kNm]':>11} {'MY[kNm]':>11} {'rpm':>8} {'TQ[kNm]':>11}")
print(hdr)
print("-" * len(hdr))
for lc in lcs:
    nm = str(safe(lc, "name"))
    try:
        p = lc.inputs_for_point_load(pl)
        fx = sc(safe(p, "force_x"), "force")
        fy = sc(safe(p, "force_y"), "force")
        ax = sc(safe(p, "axial_load"), "force")
        mx = sc(safe(p, "moment_x"), "moment")
        my = sc(safe(p, "moment_y"), "moment")
    except Exception as e:
        print(f"{nm:32} <point load 조회 실패: {str(e).splitlines()[0][:40]}>")
        continue
    try:
        q = lc.inputs_for_power_load(ipl)
        spd = sc(q, "speed")
        tq = sc(q, "torque")
    except Exception:
        spd = tq = None

    def g(v, d=1e3):
        return "     None" if v is None else f"{v/d:10.1f}"
    print(f"{nm:32} {g(fx)} {g(fy)} {g(ax)} {g(mx):>11} {g(my):>11} "
          f"{'    None' if spd is None else f'{spd*RADS2RPM:8.3f}'} "
          f"{'       None' if tq is None else f'{tq/1e3:11.1f}'}")

print("\n[듀티사이클]")
for dc in list(safe(dp, "duty_cycles") or []):
    print("  ", dc)

print("\n[Design State / LoadCase 그룹]")
try:
    for g_ in list(dp.load_case_groups):
        print("  ", g_)
except Exception as e:
    print("   조회 실패:", str(e).splitlines()[0][:60])

print("\n완료")
