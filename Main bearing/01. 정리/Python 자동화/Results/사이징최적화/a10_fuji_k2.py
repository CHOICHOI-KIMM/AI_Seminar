# -*- coding: utf-8 -*-
"""§10-12.6 보강 — 가공성 제약 K2 <= 0.5 전용 비대칭 탐색

특허(NTN US8398312B2)는 가공을 위해 중앙 직선부 >= 전장의 50 %,
즉 K2 <= 0.5 를 권장한다. S1 격자에는 K2 = 0.4 수준이 있었으나 대칭이라
가능해가 없었고, S2/S3 는 S1 상위 씨앗(K2 = 0.85 ~ 1.0)만 상속했다.
따라서 K2 <= 0.5 에서 비대칭이 가능한지는 아직 답이 없다 — 여기서 답한다.
"""
import csv
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import a10_profile_lib as L      # noqa: E402
import a10_fujiwara as F         # noqa: E402

OUTD = F.OUTD
RANKS = (1, 103, 210)
K2S = (0.35, 0.5)
K1S = (4.0, 6.0, 8.0, 12.0)
ZMS = (250e-6, 400e-6, 600e-6, 900e-6)
RATIOS = (0.15, 0.3, 0.5)


def main():
    P = {int(x["rank"]): x for x in csv.DictReader(
        open(L.PARETO, encoding="utf-8-sig"))}
    base = json.load(open(os.path.join(OUTD, "baseline.json"),
                          encoding="utf-8"))
    rig = L.Rig()
    rig.load_case()
    rows = []
    for rk in RANKS:
        rig.build(P[rk])
        b = base[str(rk)]
        Lwe, Q = b["L_we_mm"] / 1e3, b["P_max_N"]
        got, n = [], 0
        for K2 in K2S:
            for K1 in K1S:
                for zm in ZMS:
                    for ra in RATIOS:
                        p = (K1, K2, zm, K1, K2, zm * ra)
                        m = F.run_p(rig, Lwe, Q, p, f"k2_{rk}_{n}")
                        r = F.row_of(rk, "K2cap",
                                     f"K1={K1} K2={K2} zmL={zm*1e6:.0f} "
                                     f"r={ra}", p, m, b)
                        rows.append(r)
                        got.append(r)
                        n += 1
            fe = [g for g in got if g["feasible"]]
            print(f"  #{rk:<4} K2={K2:<5} 누적 {n:3d} · 가능 {len(fe):3d}",
                  flush=True)
        fe = [g for g in got if g["feasible"]]
        if fe:
            t = max(fe, key=lambda r: r["score"])
            print(f"  #{rk:<4} ▶ K2<=0.5 최적 {t['tag']} · score "
                  f"{t['score']:.4f} · σ {t['sigma_MPa']:.1f} · edge_L "
                  f"{t['edge_L_MPa']:.1f} · margin_L {t['margin_L_mm']} · "
                  f"y* {t['y_star_mm']}", flush=True)
        else:
            print(f"  #{rk:<4} ▶ K2<=0.5 가능해 없음 — 가공성 제약을 지키면 "
                  f"σ 제약을 만족할 수 없다", flush=True)
    out = os.path.join(OUTD, "k2cap.csv")
    with open(out, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=F.COLS)
        w.writeheader()
        w.writerows(rows)
    print("\n[저장]", out, len(rows), "행")


if __name__ == "__main__":
    main()
