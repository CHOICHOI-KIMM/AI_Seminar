"""
부록 5 Step 0 — 신규 모델(v1.4 3안·베어링/롤러 확대·프리로드·50°C) 상수 추출
==============================================================================
스크리닝(run_dlc_screening.py) 상수 6종을 신규 모델에서 실측한다.
  기하 : L_, A_, B_        (Point Load / UW / DW 축좌표)
  정격 : C_N, CU_N          (basic dynamic load rating, fatigue load limit)
  하중비: E_LIM, Y1          (e, Y1=0.4cotα ; α는 e=1.5tanα 역산과 대조)
  윤활 : NU50, EC50, DPW    (50°C 동점도, 오염계수, 피치직경)
기준선(v1.3 50°C) 대비 변화율을 함께 출력.
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
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType  # noqa: E402

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_프리로드 적용_온도_50도_260726.Masta")

RPM2RADS = 2 * math.pi / 60

# 기준선 = run_dlc_screening.py 현행 상수 (v1.3 50°C)
BASE = dict(L_=2.5, A_=-0.5, B_=3.0, E_LIM=0.5165, Y1=1.1617,
            C_N=22228e3, CU_N=3929e3, NU50=294.637, EC50=0.888378, DPW=3328.6)

# 대표 하중 1점 (masta_bearing_const.py 와 동일 — Fa/Fr>e 분기 확인용)
REP = dict(fx=4965600.0, fy=126450.0, fz=3017070.0,
           mx=-4814750.0, my=-13666500.0, rpm=5.09)


def safe(o, n, default=None):
    try:
        v = getattr(o, n)
        return default if v is None else v
    except Exception:
        return default


def num(o, *names):
    """후보 속성명 중 먼저 잡히는 스칼라 반환 → (값, 사용된 이름)"""
    for n in names:
        v = safe(o, n)
        if isinstance(v, (int, float)) and not isinstance(v, bool):
            return float(v), n
    return None, None


def vec3(v):
    try:
        return tuple(round(float(x), 6) for x in list(v)[:3])
    except Exception:
        return None


def origin_of(c):
    lcs = safe(c, "local_coordinate_system")
    if lcs is None:
        return None
    for a in ("origin", "translation", "location"):
        o = safe(lcs, a)
        if o is not None:
            g = vec3(o)
            if g:
                return g
    return None


def delta(new, old):
    if new is None or old in (None, 0):
        return "  n/a"
    return f"{(new / old - 1) * 100:+7.2f}%"


print("=" * 78)
print("부록 5 Step 0 — 모델 상수 추출")
print(f"MODEL: {os.path.basename(MODEL)}")
print("=" * 78)

design = Design.load(MODEL)
asm = design.all_parts_of_type_root_assembly()[0]
pl = list(asm.all_parts_of_type_point_load())[0]
bearings = list(asm.all_parts_of_type_bearing())
ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())

lcs_all = list(asm.design_properties.static_loads)
lc = next((c for c in lcs_all if getattr(c, "name", "") == "Load Case 1"), lcs_all[0])
print(f"\n[Load Case] 사용: {getattr(lc, 'name', lc)}  (총 {len(lcs_all)}개)")

# ── 1. 기하 (L_, A_, B_) ────────────────────────────────────────────────
print("\n" + "-" * 78)
print("1. 기하 — 축좌표 (Point Load 기준 z, 단위 m)")
print("-" * 78)
pos = {}
for c in [pl] + bearings:
    g = origin_of(c)
    pos[str(c)] = g
    print(f"  {str(c):40} origin = {g}")

zp = pos.get(str(pl))
zs = [(str(b), pos.get(str(b))) for b in bearings]
if zp and all(g for _, g in zs):
    z_pl = zp[2]
    zb = sorted([(g[2], n) for n, g in zs])          # z 오름차순 = UW(앞), DW(뒤)
    (z_uw, n_uw), (z_dw, n_dw) = zb[0], zb[1]
    L_new = z_dw - z_uw
    A_new = z_pl - z_uw          # 오버행이면 음수
    B_new = z_dw - z_pl
    print(f"\n  UW = {n_uw} @ z={z_uw:.4f} · DW = {n_dw} @ z={z_dw:.4f} · PL @ z={z_pl:.4f}")
    print(f"  → L_ = {L_new:.4f}  (기준 {BASE['L_']}, {delta(L_new, BASE['L_'])})")
    print(f"  → A_ = {A_new:.4f}  (기준 {BASE['A_']})")
    print(f"  → B_ = {B_new:.4f}  (기준 {BASE['B_']}, {delta(B_new, BASE['B_'])})")
else:
    print("\n  !! origin 미추출 — 축좌표 수동 확인 필요 (MASTA GUI 또는 shaft offset)")

# ── 2. 시스템 디플렉션 1점 해석 ─────────────────────────────────────────
print("\n" + "-" * 78)
print("2. 대표 하중 1점 System Deflection (정격/윤활 상수 추출용)")
print("-" * 78)
p = lc.inputs_for_point_load(pl)
p.force_x.force = REP["fx"]; p.force_y.force = REP["fy"]; p.axial_load.force = REP["fz"]
p.moment_x.moment = REP["mx"]; p.moment_y.moment = REP["my"]
lc.inputs_for_power_load(ipl).speed = REP["rpm"] * RPM2RADS
sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
sd.perform_analysis()
print("  해석 완료")

# ── 3. 베어링별 상수 ────────────────────────────────────────────────────
FIELDS = [
    ("C  기본동정격 [N]", ("basic_dynamic_load_rating", "dynamic_load_rating",
                        "basic_dynamic_load_rating_radial")),
    ("Cu 피로한계하중 [N]", ("fatigue_load_limit", "basic_fatigue_load_limit")),
    ("C0 기본정정격 [N]", ("basic_static_load_rating", "static_load_rating")),
    ("Dpw 피치직경 [mm]", ("pitch_diameter", "mean_diameter", "bearing_pitch_diameter",
                        "element_pitch_diameter")),
    ("접촉각 α [deg]", ("contact_angle", "nominal_contact_angle")),
    ("롤러 수 Z", ("number_of_elements", "number_of_rolling_elements",
                "number_of_elements_per_row")),
    ("롤러 직경 [mm]", ("element_diameter", "roller_diameter", "mean_element_diameter")),
    ("롤러 유효길이 [mm]", ("element_length", "roller_length", "effective_roller_length")),
    ("열 수", ("number_of_rows",)),
]
ISO_FIELDS = [
    ("e  한계하중비", ("e_limiting_value", "limiting_value_e", "e")),
    ("X  동경방향계수", ("dynamic_radial_load_factor", "radial_load_factor")),
    ("Y  동축방향계수", ("dynamic_axial_load_factor", "axial_load_factor")),
    ("P  등가하중 [N]", ("dynamic_equivalent_load", "equivalent_dynamic_load")),
]
LUB_FIELDS = [
    ("ν  동점도 [mm²/s]", ("lubricant_kinematic_viscosity", "kinematic_viscosity",
                        "operating_viscosity", "viscosity")),
    ("ν1 기준점도 [mm²/s]", ("reference_kinematic_viscosity", "required_viscosity")),
    ("κ  점도비", ("viscosity_ratio", "calculated_viscosity_ratio")),
    ("e_C 오염계수", ("contamination_factor",
                  "contamination_factor_from_calculated_viscosity_ratio")),
    ("작동온도 [°C]", ("operating_temperature", "lubricant_temperature")),
]

found = {}
for b in bearings:
    print("\n" + "=" * 78)
    print(f"[{b}]")
    print("-" * 78)
    d = sd.results_for(b).component_detailed_analysis
    rec = {}
    for label, cands in FIELDS:
        v, used = num(b, *cands)
        if v is None:
            v, used = num(d, *cands)
        rec[label] = v
        print(f"  {label:22} = {v if v is None else round(v, 4)}   [{used}]")
    iso = safe(d, "iso2812007")
    print("  -- ISO 281:2007 --")
    for label, cands in ISO_FIELDS:
        v, used = (None, None) if iso is None else num(iso, *cands)
        if v is None:
            v, used = num(d, *cands)
        rec[label] = v
        print(f"  {label:22} = {v if v is None else round(v, 6)}   [{used}]")
    print("  -- 윤활/50°C --")
    for label, cands in LUB_FIELDS:
        v, used = num(d, *cands)
        if v is None and iso is not None:
            v, used = num(iso, *cands)
        rec[label] = v
        print(f"  {label:22} = {v if v is None else round(v, 6)}   [{used}]")
    found[str(b)] = rec

# ── 4. 기준선 대비 요약 ─────────────────────────────────────────────────
print("\n" + "=" * 78)
print("4. 스크리닝 상수 갱신 후보 (기준선 = v1.3 50°C 현행값)")
print("=" * 78)
print(f"{'상수':22} {'기준선':>14} {'신규(실측)':>14} {'변화':>10}")
print("-" * 66)


def pick(label):
    """베어링들 중 최댓값(정격) / 첫값 사용 — 두 베어링 동일 사양이면 무관"""
    vals = [r.get(label) for r in found.values() if r.get(label) is not None]
    return max(vals) if vals else None


rows = [
    ("C_N   [N]",   BASE["C_N"],  pick("C  기본동정격 [N]")),
    ("CU_N  [N]",   BASE["CU_N"], pick("Cu 피로한계하중 [N]")),
    ("DPW   [mm]",  BASE["DPW"],  pick("Dpw 피치직경 [mm]")),
    ("E_LIM",       BASE["E_LIM"], pick("e  한계하중비")),
    ("NU50  [mm²/s]", BASE["NU50"], pick("ν  동점도 [mm²/s]")),
    ("EC50",        BASE["EC50"], pick("e_C 오염계수")),
]
for name, old, new in rows:
    ns = "   미추출" if new is None else f"{new:14.4f}"
    print(f"{name:22} {old:14.4f} {ns} {delta(new, old):>10}")

e_new = pick("e  한계하중비")
if e_new:
    alpha = math.degrees(math.atan(e_new / 1.5))
    y1 = 0.4 / math.tan(math.radians(alpha))
    print(f"\n  e={e_new:.4f} → α = {alpha:.2f}°  (TRB 규칙 e=1.5tanα)")
    print(f"  Y1 = 0.4cotα = {y1:.4f}   (기준 {BASE['Y1']}, {delta(y1, BASE['Y1'])})")
    a_direct = pick("접촉각 α [deg]")
    if a_direct:
        print(f"  * MASTA 직접 접촉각 = {a_direct:.3f}° (역산 {alpha:.2f}° 와 대조)")

print("\n완료")
