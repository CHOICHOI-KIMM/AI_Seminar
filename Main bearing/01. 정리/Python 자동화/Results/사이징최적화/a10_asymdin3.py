# -*- coding: utf-8 -*-
"""§10-12.7.8 사전 — δ 상한 확장 + 기하 제약 검토

2차에서 세 설계 모두 δ 가 탐색 상한 40 mm 에 붙었다. 상한에 붙은 해를
「최적」이라 적을 수는 없으므로 80 mm 까지 열어 안쪽에 있는지 확인한다.

동시에 **기하 제약**을 함께 본다 — δ 는 프로파일 원점을 롤러 중앙에서
밀어내므로, 반길이 대비 비율 δ/a 가 커지면 가공·검사가 어려워진다.
δ/a 를 함께 기록해 30 % 선을 넘는지 표시한다.
"""
import csv
import json
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import a10_profile_lib as L      # noqa: E402
import a10_asymdin as AD         # noqa: E402
import a10_asymdin2 as A2        # noqa: E402

OUT = A2.OUT
RANKS = A2.RANKS
DELTAS = (40, 50, 60, 70, 80)
CAP = 0.30                        # 기하 제약 검토선 — δ / (L_we/2)


def main():
    P = {int(x["rank"]): x for x in csv.DictReader(
        open(L.PARETO, encoding="utf-8-sig"))}
    base = {int(k): v for k, v in json.load(
        open(os.path.join(OUT, "baseline.json"), encoding="utf-8")).items()}
    prev = {int(k): v for k, v in json.load(
        open(os.path.join(OUT, "best2.json"), encoding="utf-8")).items()}
    rig = L.Rig()
    rig.load_case()
    t0 = time.perf_counter()
    rows, best = [], {}

    for rk in RANKS:
        rig.build(P[rk])
        d = rig.uw.detail
        Lwe, Dwe = d.effective_roller_length, d.element_diameter
        half = Lwe * 1e3 / 2.0
        p0 = prev[rk]["p"]
        kl0, kr0, _, fl0, fr0 = p0
        print("=" * 76)
        print(f"#{rk} · 반길이 {half:.1f} mm · 2차 최적 k {kl0:g}/{kr0:g} "
              f"δ {p0[2]:g} (δ/a {p0[2]/half*100:.1f}%) score "
              f"{prev[rk]['s'][4]:.4f}")
        print(f"{'δ':>5s} {'δ/a':>7s} {'k_L':>5s} {'k_R':>6s} {'σ':>8s} "
              f"{'mL':>6s} {'mR':>6s} {'end_L':>7s} {'end_R':>7s} "
              f"{'y*%':>6s} {'s1':>6s} {'s3':>6s} {'score':>7s} {'기하':>5s}")
        for dd in DELTAS:
            for kl in (round(kl0 - 0.4, 3), kl0, round(kl0 + 0.4, 3)):
                for kr in (round(kr0 - 0.1, 3), kr0, round(kr0 + 0.1, 3)):
                    if kl <= 0 or kr <= 0:
                        continue
                    p = (kl, kr, dd, fl0, fr0)
                    f = A2.asym_din2(Lwe, Dwe, *p)
                    try:
                        rig.set_user(f, A2.NPTS)
                        m, off = rig.solve(f"u4_{rk}_{dd}_{kl}_{kr}")
                    except Exception as e:
                        print(f"    [실패] δ{dd} k{kl}/{kr} "
                              f"{str(e).splitlines()[0][:40]}")
                        continue
                    g = AD.metrics(m, off, Lwe)
                    sc = A2.score2(g, base[rk])
                    ratio = dd / half
                    r = dict(rank=rk, stage="U4", k_L=kl, k_R=kr,
                             delta_mm=dd, f_L=fl0, f_R=fr0,
                             delta_over_a=round(100 * ratio, 2),
                             geom_ok=int(ratio <= CAP),
                             s1=sc[0] if sc else None,
                             s2=sc[1] if sc else None,
                             s3=sc[2] if sc else None,
                             s4=sc[3] if sc else None,
                             score=sc[4] if sc else None,
                             feasible=int(sc is not None))
                    r.update({k: (g or {}).get(k) for k in
                              ("sigma_MPa", "end_L_MPa", "end_L_pct",
                               "margin_L_pct", "margin_L_lam", "end_R_MPa",
                               "end_R_pct", "margin_R_pct", "margin_R_lam",
                               "y_star_pct")})
                    rows.append(r)
                    if sc and (rk not in best or sc[4] > best[rk]["s"][4]):
                        best[rk] = dict(p=p, g=g, s=sc,
                                        ratio=round(100 * ratio, 2))
                    if kl == kl0 and kr == kr0:
                        print(f"{dd:5d} {100*ratio:6.1f}% {kl:5g} {kr:6g} "
                              f"{g['sigma_MPa']:8.1f} "
                              f"{g['margin_L_lam']:6.2f} "
                              f"{g['margin_R_lam']:6.2f} "
                              f"{g['end_L_MPa']:7.1f} {g['end_R_MPa']:7.1f} "
                              f"{g['y_star_pct']:6.1f} "
                              + (f"{sc[0]:6.3f} {sc[2]:6.3f} {sc[4]:7.4f}"
                                 if sc else f"{'—':>6s} {'—':>6s} "
                                            f"{'위반':>7s}")
                              + f" {'OK' if ratio <= CAP else '초과':>5s}",
                              flush=True)
        w = best.get(rk)
        if w:
            print(f"  ▶ #{rk} 확장 최적 δ {w['p'][2]:g} ({w['ratio']}%) · "
                  f"k {w['p'][0]:g}/{w['p'][1]:g} · score {w['s'][4]:.4f} "
                  f"(2차 {prev[rk]['s'][4]:.4f} · "
                  f"Δ{w['s'][4]-prev[rk]['s'][4]:+.4f})")

    with open(os.path.join(OUT, "asymdin3_delta.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)
    json.dump({str(k): dict(p=v["p"], s=v["s"], ratio=v["ratio"], g=v["g"])
               for k, v in best.items()},
              open(os.path.join(OUT, "best3.json"), "w"), indent=1)
    print(f"\n[완료] {len(rows)}행 · {(time.perf_counter()-t0)/60:.1f}분")


if __name__ == "__main__":
    main()
