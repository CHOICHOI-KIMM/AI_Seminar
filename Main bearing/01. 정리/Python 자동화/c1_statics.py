"""
G-8.2: 강체 정역학 Fr ↔ MASTA P 역산 대조 (MASTA 미사용)
========================================================
목적 3가지
 (1) 베어링별 기본동적정격 C 확정 (G1 완결)
 (2) 강체 정역학이 FE 유연체 모델 결과에도 성립하는지 검증
 (3) 방법 C-1 알고리즘 2~4단계(정역학→Fr/Fa→P) 완성

정역학 (§C-3.3, 기하 §C-3.6): 축=Z, 하중점 P@z=0, UW(A)@z=0.5, DW(B)@z=3.0
  L = z_B − z_A = 2.5 ,  a = z_P − z_A = −0.5 ,  b = z_B − z_P = 3.0
  R_A,X = F_X·(b/L) − M_Y/L      R_B,X = F_X·(a/L) + M_Y/L
  R_A,Y = F_Y·(b/L) + M_X/L      R_B,Y = F_Y·(a/L) − M_X/L
좌표변환 (§4.2): F_X=−Fz·1e3, F_Y=Fy·1e3, F_Z=Fx·1e3, M_X=−Mz·1e3, M_Y=My·1e3
"""
import csv
import glob
import math
import os
import statistics as st

from c1_aiso import C_N, P_EXP

DLC = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\260714 유니슨 피로하중\DLC1.2-c-s1.$150")
Z_P, Z_A, Z_B = 0.0, 0.5, 3.0
L = Z_B - Z_A
A_ = Z_P - Z_A
B_ = Z_B - Z_P
E_LIM = 0.5165          # 한계 하중비 e (§C-3.1)
Y1 = 0.4 / math.tan(math.atan(E_LIM / 1.5))   # TRB: e=1.5tanα, Y=0.4cotα
NAME_A, NAME_B = "Main Bearing_UW", "Main Bearing_DW"


def parse_dlc(path):
    rows = []
    for ln in open(path, encoding="latin-1").readlines()[4:]:
        p = ln.split()
        if len(p) < 8:
            continue
        try:
            v = [float(x) for x in p[:8]]
        except ValueError:
            continue
        rows.append(dict(t=v[0], rpm=v[1], Mx=v[2], My=v[3], Mz=v[4],
                         Fx=v[5], Fy=v[6], Fz=v[7]))
    return rows


def hub(rec):
    """시계열 → MASTA 좌표 hub 6분력 [N, N·m]."""
    return (-rec["Fz"] * 1e3, rec["Fy"] * 1e3, rec["Fx"] * 1e3,
            -rec["Mz"] * 1e3, rec["My"] * 1e3)


def statics(rec):
    """→ (Fr_A, Fr_B, F_Z) 강체 2지점 정역학."""
    FX, FY, FZ, MX, MY = hub(rec)
    rax = FX * (B_ / L) - MY / L
    ray = FY * (B_ / L) + MX / L
    rbx = FX * (A_ / L) + MY / L
    rby = FY * (A_ / L) - MX / L
    return math.hypot(rax, ray), math.hypot(rbx, rby), FZ


def axial_split(fr_a, fr_b, ka):
    """TRB쌍 유도축력 분배 (§C-3.3(2)). ka = 외부 축력(A를 미는 방향 +)."""
    sa, sb = 0.5 * fr_a / Y1, 0.5 * fr_b / Y1
    if ka >= sa - sb:
        return ka + sb, sb
    return sa, sa - ka


def equiv_load(fr, fa):
    """ISO 281 카탈로그 X·Y 등가하중."""
    if fr <= 0:
        return abs(fa) * Y1
    return fr if abs(fa) / fr <= E_LIM else 0.4 * fr + Y1 * abs(fa)


def main():
    print(f"[기하] z_P=0, z_UW={Z_A}, z_DW={Z_B}  →  L={L}, a={A_}, b={B_}")
    print(f"[TRB] e={E_LIM}  →  α={math.degrees(math.atan(E_LIM/1.5)):.2f}°, Y1={Y1:.4f}")

    data = parse_dlc(DLC)
    files = [f for f in sorted(glob.glob(os.path.join("Results", "*_dt_*.csv")))
             if "summary" not in f]
    path = max(files, key=lambda f: sum(1 for _ in open(f, encoding="utf-8-sig")))
    print(f"[대조] {os.path.basename(path)}  (FE 유연체 모델 결과)\n")

    # MASTA P 역산
    masta = {}
    for r in csv.DictReader(open(path, encoding="utf-8-sig")):
        try:
            i = int(r["index"]); l10 = float(r["L10_basic_rev"]); b = r["bearing"]
        except (ValueError, KeyError):
            continue
        if l10 <= 0:
            continue
        masta.setdefault(b, {})[i] = C_N / ((l10 / 1e6) ** (1.0 / P_EXP))

    idxs = sorted(set(masta.get(NAME_A, {})) & set(masta.get(NAME_B, {})))
    print(f"대조 점수: {len(idxs)}\n")

    stats = {NAME_A: [], NAME_B: []}
    ratios = {NAME_A: [], NAME_B: []}
    frs = {NAME_A: [], NAME_B: []}
    for i in idxs:
        rec = data[i]
        fra, frb, fz = statics(rec)
        faa, fab = axial_split(fra, frb, fz)
        for nm, fr, fa in ((NAME_A, fra, faa), (NAME_B, frb, fab)):
            P = equiv_load(fr, fa)
            pm = masta[nm][i]
            stats[nm].append((P - pm) / pm * 100.0)
            ratios[nm].append(abs(fa) / fr if fr else 0)
            frs[nm].append((fr, fa, P, pm))

    for nm in (NAME_A, NAME_B):
        e = stats[nm]
        fr0, fa0, P0, pm0 = frs[nm][0]
        print("=" * 72)
        print(f"[{nm}]")
        print("=" * 72)
        print(f"  Fr(정역학)  : {min(f[0] for f in frs[nm])/1e3:9.1f} ~ "
              f"{max(f[0] for f in frs[nm])/1e3:9.1f} kN")
        print(f"  Fa(TRB분배) : {min(f[1] for f in frs[nm])/1e3:9.1f} ~ "
              f"{max(f[1] for f in frs[nm])/1e3:9.1f} kN")
        print(f"  |Fa|/Fr     : {min(ratios[nm]):.4f} ~ {max(ratios[nm]):.4f}  "
              f"(e={E_LIM}, 초과 {sum(1 for r in ratios[nm] if r > E_LIM)}점)")
        print(f"  P(MASTA역산): {min(f[3] for f in frs[nm])/1e3:9.1f} ~ "
              f"{max(f[3] for f in frs[nm])/1e3:9.1f} kN")
        print(f"  ── P(C-1) vs P(MASTA) ──")
        print(f"  평균 {st.mean(e):+8.3f}%   중앙 {st.median(e):+8.3f}%   "
              f"최대|오차| {max(abs(x) for x in e):8.3f}%   "
              f"RMS {math.sqrt(st.mean(x*x for x in e)):.3f}%")
        # C 확정: P_C1 이 옳다면 C_implied = C_N × (P_C1/P_MASTA)
        ci = [C_N * (f[2] / f[3]) for f in frs[nm]]
        print(f"  → 함의 C = {st.mean(ci)/1e3:,.0f} kN  "
              f"[{min(ci)/1e3:,.0f} ~ {max(ci)/1e3:,.0f}]  (가정 C={C_N/1e3:,.0f} kN)")

    print("\n[참고] 첫 점(index=0) 상세")
    for nm in (NAME_A, NAME_B):
        fr, fa, P, pm = frs[nm][0]
        print(f"  {nm:18} Fr={fr/1e3:8.1f}  Fa={fa/1e3:8.1f}  "
              f"P_C1={P/1e3:8.1f}  P_MASTA={pm/1e3:8.1f} kN")


if __name__ == "__main__":
    main()
