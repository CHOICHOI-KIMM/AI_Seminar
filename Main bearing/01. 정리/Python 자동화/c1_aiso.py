"""
ISO 281:2007 a_ISO 구현 + MASTA 점별 값 대조 검증 (G4)
======================================================
원문: AI_Seminar/CRB-main/Reference/ISO_281_2007.md
  §9.3.3.3.1 식(28): ν₁ = 45000·n^(−0.83)·D_pw^(−0.5)   [n < 1000 r/min]
  §9.3.4 식(34)~(36) **반경 롤러 베어링**:
    a_ISO = 0.1[1 − (1.5859 − 1.3993/κ^0.054381)·(e_C·C_u/P)^0.4]^(−9.185)   0.1≤κ<0.4
    a_ISO = 0.1[1 − (1.5859 − 1.2348/κ^0.19087 )·(e_C·C_u/P)^0.4]^(−9.185)   0.4≤κ<1
    a_ISO = 0.1[1 − (1.5859 − 1.2348/κ^0.071739)·(e_C·C_u/P)^0.4]^(−9.185)   1≤κ≤4
  ※ 롤러식의 κ항에는 지수 0.83 이 **없음**(볼 베어링 식 31~33에만 있음), 지수는 −9.185.
  §9.3.4: κ>4 → κ=4 사용. κ<0.1 은 계산 불가(범위 밖).
"""
import csv
import glob
import math
import os
import statistics as st

# ── 카탈로그 상수 (§C-3.1, MASTA 리포트/API) ──
C_N   = 22228e3      # 기본동적정격 [N]
CU_N  = 3929e3       # 피로한계하중 [N]
E_C   = 0.9236       # 오염계수
DM_MM = 3330.0       # D_pw ≈ 평균지름 [mm]
NU    = 98.7         # 운전 동점도 [mm²/s]
P_EXP = 10.0 / 3.0   # 수명지수 (롤러)


def nu1(rpm, dpw_mm=DM_MM):
    """ISO 281 식(28)/(29) 기준 동점도 [mm²/s]."""
    if rpm < 1000.0:
        return 45000.0 * (rpm ** -0.83) * (dpw_mm ** -0.5)
    return 4500.0 * (rpm ** -0.5) * (dpw_mm ** -0.5)


def kappa(rpm, nu=NU, dpw_mm=DM_MM):
    return nu / nu1(rpm, dpw_mm)


def a_iso_radial_roller(kap, ratio):
    """ISO 281 식(34)~(36). ratio = e_C·C_u/P."""
    k = min(kap, 4.0)                      # §9.3.4: κ>4 → 4
    if k < 0.1:
        raise ValueError(f"κ={k:.3f} < 0.1 : ISO 281 계산 범위 밖")
    if k < 0.4:
        term = 1.5859 - 1.3993 / (k ** 0.054381)
    elif k < 1.0:
        term = 1.5859 - 1.2348 / (k ** 0.19087)
    else:
        term = 1.5859 - 1.2348 / (k ** 0.071739)
    inner = 1.0 - term * (ratio ** 0.4)
    if inner <= 0:
        return 50.0
    return min(0.1 * inner ** (-9.185), 50.0)


# ─────────────────── 검증 ───────────────────
def _load(path):
    out = {}
    for r in csv.DictReader(open(path, encoding="utf-8-sig")):
        try:
            l10 = float(r["L10_basic_rev"]); l10m = float(r["L10m_mod_rev"])
            rpm = float(r["rpm"]); b = r["bearing"]
        except (ValueError, KeyError):
            continue
        if l10 <= 0 or l10m <= 0:
            continue
        P = C_N / ((l10 / 1e6) ** (1.0 / P_EXP))     # L10=(C/P)^p·1e6 역산
        out.setdefault(b, []).append((rpm, P, l10m / l10))
    return out


def main():
    files = [f for f in sorted(glob.glob(os.path.join("Results", "*_dt_*.csv")))
             if "summary" not in f]
    path = max(files, key=lambda f: sum(1 for _ in open(f, encoding="utf-8-sig")))
    print(f"[데이터] {os.path.basename(path)}   (a_ISO,MASTA = L10m_mod / L10_basic, a1=1)")
    print(f"[상수] C={C_N/1e3:,.0f} kN  C_u={CU_N/1e3:,.0f} kN  e_C={E_C}  "
          f"D_pw={DM_MM:,.0f} mm  ν={NU} mm²/s")

    for b, rows in _load(path).items():
        errs, ks, rs, ams = [], [], [], []
        for rpm, P, am in rows:
            k = kappa(rpm)
            ratio = E_C * CU_N / P
            pred = a_iso_radial_roller(k, ratio)
            errs.append((pred - am) / am * 100.0)
            ks.append(k); rs.append(ratio); ams.append(am)
        print("\n" + "=" * 68)
        print(f"[{b}]  {len(rows)}점")
        print("=" * 68)
        print(f"  κ           : {min(ks):.4f} ~ {max(ks):.4f}   "
              f"(<0.4 {sum(1 for k in ks if k < 0.4)}점)")
        print(f"  e_C·C_u/P   : {min(rs):.4f} ~ {max(rs):.4f}")
        print(f"  a_ISO MASTA : {min(ams):.4f} ~ {max(ams):.4f}  (평균 {st.mean(ams):.4f})")
        print(f"  ── ISO 281 식(34~36) 대조 ──")
        print(f"  평균오차 {st.mean(errs):+.3f}%   최대 |오차| {max(abs(e) for e in errs):.3f}%"
              f"   RMS {math.sqrt(st.mean(e*e for e in errs)):.3f}%")


if __name__ == "__main__":
    main()
