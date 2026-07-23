"""
강체 정역학 예측 vs MASTA 대조 (C-3.6)
======================================
각 점의 hub 5-DOF 하중과 MASTA 실제 베어링 반력(internal_force)을 추출하여:
 (1) 반력이 hub 하중의 선형함수인지(=강체/선형 정역학 성립) 최소자승 검정 → R², 잔차%
 (2) 계수구조가 보 정역학(R_X↔F_X,M_Y / R_Y↔F_Y,M_X)과 맞는지 확인
 (3) 영향계수[T]로 예측 → MASTA와 성분·Fr·Fa 대조
hub 입력(변환 §4.2): F_X=-Fz*1e3, F_Y=Fy*1e3, F_Z(축)=Fx*1e3, M_X=-Mz*1e3, M_Y=My*1e3
"""
import math
import numpy as np
import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

MODEL = r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안_피로하중 반영_260720.Masta"
DLC   = r"D:\AI\AI_Seminar\Main bearing\02. 자료\260714 유니슨 피로하중\DLC1.2-c-s1.$150"
N_PTS = 100
RPM2RADS = 2 * math.pi / 60


def parse(path, n):
    rows = []
    with open(path, "r", encoding="latin-1") as f:
        for ln in f.readlines()[4:]:
            p = ln.split()
            if len(p) < 8:
                continue
            try:
                v = [float(x) for x in p[:8]]
            except ValueError:
                continue
            rows.append(dict(t=v[0], rpm=v[1], Mx=v[2], My=v[3], Mz=v[4],
                             Fx=v[5], Fy=v[6], Fz=v[7]))
            if len(rows) >= n:
                break
    return rows


def vec3(v):
    for acc in (("x", "y", "z"),):
        try:
            return float(getattr(v, acc[0])), float(getattr(v, acc[1])), float(getattr(v, acc[2]))
        except Exception:
            pass
    try:
        lst = list(v)
        return float(lst[0]), float(lst[1]), float(lst[2])
    except Exception:
        return None


design = Design.load(MODEL)
assembly = design.all_parts_of_type_root_assembly()[0]
pl = list(assembly.all_parts_of_type_point_load())[0]
bearings = list(assembly.all_parts_of_type_bearing())
ipl = next(p for p in assembly.all_parts_of_type_power_load() if "input" in str(p).lower())
lc = next(c for c in assembly.design_properties.static_loads
          if getattr(c, "name", "") == "Load Case 1")

# --- 기하(축좌표) 추출 시도 ---
print("[기하 추출 시도] 컴포넌트 local_coordinate_system origin")
for c in [pl] + bearings:
    lcs = getattr(c, "local_coordinate_system", None)
    org = None
    if lcs is not None:
        for a in ("origin", "translation", "location"):
            o = getattr(lcs, a, None)
            if o is not None:
                org = vec3(o)
                if org:
                    print(f"  {c}: {a}={org}")
                    break
    if org is None:
        print(f"  {c}: local_coordinate_system={lcs} (origin 미추출)")

data = parse(DLC, N_PTS)
HUB = []      # [F_X, F_Y, F_Z, M_X, M_Y]
REACT = {str(b): {"RX": [], "RY": [], "RZ": [], "Fr": [], "Fa": [], "P": []} for b in bearings}

for rec in data:
    F_X = -rec["Fz"] * 1e3; F_Y = rec["Fy"] * 1e3; F_Z = rec["Fx"] * 1e3
    M_X = -rec["Mz"] * 1e3; M_Y = rec["My"] * 1e3
    HUB.append([F_X, F_Y, F_Z, M_X, M_Y])
    p = lc.inputs_for_point_load(pl)
    p.force_x.force = F_X; p.force_y.force = F_Y; p.axial_load.force = F_Z
    p.moment_x.moment = M_X; p.moment_y.moment = M_Y
    lc.inputs_for_power_load(ipl).speed = rec["rpm"] * RPM2RADS
    sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
    sd.perform_analysis()
    for b in bearings:
        res = sd.results_for(b)
        f = vec3(getattr(res, "internal_force", None))
        d = REACT[str(b)]
        if f:
            d["RX"].append(f[0]); d["RY"].append(f[1]); d["RZ"].append(f[2])
            d["Fr"].append(math.hypot(f[0], f[1])); d["Fa"].append(abs(f[2]))
        det = res.component_detailed_analysis
        d["P"].append(getattr(det.iso2812007, "dynamic_equivalent_load", float("nan")))

HUB = np.array(HUB)                       # (n,5)
A1 = np.hstack([HUB, np.ones((len(HUB), 1))])   # +절편
labels = ["F_X", "F_Y", "F_Z", "M_X", "M_Y", "1"]


def fit(target):
    y = np.array(target)
    coef, *_ = np.linalg.lstsq(A1, y, rcond=None)
    pred = A1 @ coef
    ss_res = np.sum((y - pred) ** 2)
    ss_tot = np.sum((y - y.mean()) ** 2)
    r2 = 1 - ss_res / ss_tot if ss_tot > 0 else float("nan")
    rms = math.sqrt(np.mean((y - pred) ** 2))
    rms_pct = rms / (math.sqrt(np.mean(y ** 2)) + 1e-30) * 100
    return coef, r2, rms_pct


print("\n" + "=" * 74)
print(f"강체/선형 정역학 검정 (반력 = 선형(hub 5-DOF)?)  N={len(HUB)}점")
print("=" * 74)
for bn, d in REACT.items():
    print(f"\n[{bn}]")
    for comp in ("RX", "RY", "RZ"):
        coef, r2, rmsp = fit(d[comp])
        terms = "  ".join(f"{labels[i]}:{coef[i]:+.4g}" for i in range(5))
        print(f"  {comp}: R²={r2:.6f}  잔차RMS={rmsp:.3f}%   계수[{terms}]  절편={coef[5]:+.3g}")
    # Fr, Fa 통계
    Fr = np.array(d["Fr"]); Fa = np.array(d["Fa"]); P = np.array(d["P"])
    print(f"  Fr: {Fr.min()/1e3:.0f}~{Fr.max()/1e3:.0f} kN, Fa: {Fa.min()/1e3:.0f}~{Fa.max()/1e3:.0f} kN")
    print(f"  MASTA Fr vs P(dyn_equiv) 최대편차: {np.max(np.abs(Fr-P))/np.mean(Fr)*100:.3f}%  (P=Fr 확인)")

print("\n완료")
