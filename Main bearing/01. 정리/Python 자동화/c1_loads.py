"""
G-8.2d: 영향계수 기반 베어링 하중 예측 + MASTA 대조 (MASTA 미사용)
==================================================================
R_b,c = Σ_k T[b][c][k]·u_k + R0[b][c]      u = (F_X,F_Y,F_Z,M_X,M_Y,M_Z)
Fr = √(R_X²+R_Y²) , Fa = R_Z
P  = Fr                       (|Fa|/Fr ≤ e)
   = 0.4·Fr + Y1·|Fa|         (|Fa|/Fr > e)
대조: P_MASTA = C/(L10_basic/1e6)^(1/p)
"""
import csv
import glob
import json
import math
import os
import statistics as st

from c1_aiso import C_N, P_EXP

DLC = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\260714 유니슨 피로하중\DLC1.2-c-s1.$150")
E_LIM = 0.5165
Y1 = 0.4 / math.tan(math.atan(E_LIM / 1.5))     # TRB: e=1.5tanα, Y=0.4cotα → 1.1617
INF = json.load(open("c1_influence.json", encoding="utf-8"))
T, R0, INPUTS = INF["T"], INF["R0"], INF["inputs"]
BEARINGS = INF["bearings"]


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
    """시계열 → MASTA hub 6분력 [N, N·m] (§4.2 좌표변환)."""
    return {"F_X": -rec["Fz"] * 1e3, "F_Y": rec["Fy"] * 1e3, "F_Z": rec["Fx"] * 1e3,
            "M_X": -rec["Mz"] * 1e3, "M_Y": rec["My"] * 1e3, "M_Z": rec["Mx"] * 1e3}


def bearing_loads(rec):
    """→ {bearing: (Fr, Fa)}"""
    u = hub(rec)
    out = {}
    for b in BEARINGS:
        r = []
        for ci, c in enumerate("XYZ"):
            r.append(R0[b][ci] + sum(T[b][c][k] * u[k] for k in INPUTS))
        out[b] = (math.hypot(r[0], r[1]), r[2])
    return out


def equiv_load(fr, fa):
    if fr <= 0:
        return abs(fa) * Y1
    return fr if abs(fa) / fr <= E_LIM else 0.4 * fr + Y1 * abs(fa)


def main():
    print(f"[영향계수] {INF['model']}")
    print(f"[TRB] e={E_LIM}  α={math.degrees(math.atan(E_LIM/1.5)):.2f}°  Y1={Y1:.4f}")
    data = parse_dlc(DLC)
    files = [f for f in sorted(glob.glob(os.path.join("Results", "*_dt_*.csv")))
             if "summary" not in f]
    path = max(files, key=lambda f: sum(1 for _ in open(f, encoding="utf-8-sig")))
    print(f"[대조]   {os.path.basename(path)}\n")

    masta = {}
    for r in csv.DictReader(open(path, encoding="utf-8-sig")):
        try:
            i = int(r["index"]); l10 = float(r["L10_basic_rev"]); b = r["bearing"]
        except (ValueError, KeyError):
            continue
        if l10 > 0:
            masta.setdefault(b, {})[i] = C_N / ((l10 / 1e6) ** (1.0 / P_EXP))

    idxs = sorted(set.intersection(*(set(masta[b]) for b in BEARINGS)))
    print(f"대조 점수: {len(idxs)}")

    for b in BEARINGS:
        errs, frs, fas, rats = [], [], [], []
        for i in idxs:
            fr, fa = bearing_loads(data[i])[b]
            P = equiv_load(fr, fa)
            pm = masta[b][i]
            errs.append((P - pm) / pm * 100.0)
            frs.append(fr); fas.append(fa); rats.append(abs(fa) / fr if fr else 0)
        print("\n" + "=" * 70)
        print(f"[{b}]")
        print("=" * 70)
        print(f"  Fr   : {min(frs)/1e3:9.1f} ~ {max(frs)/1e3:9.1f} kN")
        print(f"  Fa   : {min(fas)/1e3:9.1f} ~ {max(fas)/1e3:9.1f} kN")
        print(f"  |Fa|/Fr: {min(rats):.4f} ~ {max(rats):.4f}   "
              f"(e={E_LIM} 초과 {sum(1 for r in rats if r > E_LIM)}점)")
        print(f"  ── P(C-1) vs P(MASTA) 점별 ──")
        print(f"  평균 {st.mean(errs):+7.3f}%   중앙 {st.median(errs):+7.3f}%   "
              f"최대|오차| {max(abs(x) for x in errs):7.3f}%   "
              f"RMS {math.sqrt(st.mean(x*x for x in errs)):.3f}%")
        aerr = sorted(abs(x) for x in errs)
        q = lambda p: aerr[min(len(aerr) - 1, int(len(aerr) * p))]
        print(f"  |오차| 백분위: 50%={q(.5):.3f}  90%={q(.9):.3f}  "
              f"99%={q(.99):.3f}   >5% 인 점 {sum(1 for x in aerr if x > 5)}개")

        # ★ 손상 관점 판정: 회전수가중 p제곱평균 등가하중 P_eq
        num_c1 = num_m = den = 0.0
        for j, i in enumerate(idxs):
            rpm = data[i]["rpm"]
            w = 0.5 if j in (0, len(idxs) - 1) else 1.0     # 사다리꼴
            n = rpm * w
            fr, fa = bearing_loads(data[i])[b]
            num_c1 += n * equiv_load(fr, fa) ** P_EXP
            num_m += n * masta[b][i] ** P_EXP
            den += n
        peq_c1 = (num_c1 / den) ** (1 / P_EXP)
        peq_m = (num_m / den) ** (1 / P_EXP)
        d_err = ((peq_c1 / peq_m) ** P_EXP - 1) * 100      # 손상 ∝ P_eq^p
        print(f"  ── 등가하중 P_eq (손상 관점) ──")
        print(f"  P_eq(C-1) = {peq_c1/1e3:9.1f} kN   P_eq(MASTA) = {peq_m/1e3:9.1f} kN"
              f"   → 오차 {(peq_c1/peq_m-1)*100:+.3f}%")
        print(f"  ★ 손상 환산 오차 = {d_err:+.3f}%  (D ∝ P_eq^{P_EXP:.3f})")


if __name__ == "__main__":
    main()
