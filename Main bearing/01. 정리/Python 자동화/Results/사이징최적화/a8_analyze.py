"""
부록 8 S3-c — 프론트 분석
============================
**평가 전량(`eval_cache.csv`)에서 프론트를 다시 계산한다.** §6-11.5 가 같은
방식을 썼다 — NSGA 가 보고하는 최종 프론트는 「최종 집단 안에서의」 비지배집합
이라, 과거 세대에 있던 더 좋은 설계에 지배되는 점이 섞일 수 있다.

산출: 부록8_NSGA/S3_본최적화/a8_pareto.csv + 화면 요약
"""
import csv
import os
import sys

import numpy as np
from pymoo.indicators.hv import HV

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

import a8_eval  # noqa: E402

OUT = os.path.join(HERE, "부록8_NSGA", "S3_본최적화")
CACHE = os.path.join(OUT, "eval_cache.csv")
GENLOG = os.path.join(OUT, "s3_genlog.csv")
LIMIT = 2100.0


def rd(p):
    with open(p, encoding="utf-8-sig") as f:
        return list(csv.DictReader(f))


def front_of(rows):
    """(베어링, 총질량) 2목적 비지배집합 — 베어링 오름차순"""
    P = np.array([[float(r["mass_brg_kg"]) / 1e3,
                   float(r["mass_total_kg"]) / 1e3] for r in rows])
    keep = []
    for i in range(len(P)):
        d = ((P[:, 0] <= P[i, 0]) & (P[:, 1] <= P[i, 1])
             & ((P[:, 0] < P[i, 0]) | (P[:, 1] < P[i, 1])))
        if not d.any():
            keep.append(i)
    keep.sort(key=lambda i: (P[i, 0], P[i, 1]))
    # 목적값이 같은 설계는 하나만 남긴다
    out, seen = [], set()
    for i in keep:
        k = (round(P[i, 0], 6), round(P[i, 1], 6))
        if k not in seen:
            seen.add(k)
            out.append(rows[i])
    return out


def main():
    rows = rd(CACHE)
    fe = [r for r in rows
          if 0 < float(r["sigma_max_MPa"]) < LIMIT and float(r["z1"]) >= 1.0]
    print(f"평가 {len(rows):,} · 가능해 {len(fe):,} "
          f"({100*len(fe)/len(rows):.1f}%)")

    res = {}
    for tag, lo in (("a", 1.0), ("b", 1.5)):
        sub = [r for r in fe if float(r["z1"]) >= lo]
        F = front_of(sub)
        res[tag] = F
        P = np.array([[float(r["mass_brg_kg"]) / 1e3,
                       float(r["mass_total_kg"]) / 1e3] for r in F])
        nclip = int((P[:, 0] >= 40.0).sum())
        hv40 = HV(ref_point=np.array([40.0, 250.0]))
        hv50 = HV(ref_point=np.array([50.0, 250.0]))
        ok = P[(P[:, 0] < 40) & (P[:, 1] < 250)]
        print(f"\n[{tag}] z1 >= {lo}  가능해 {len(sub):,} · 프론트 {len(F)}건")
        print(f"    베어링 {P[:,0].min():.3f} ~ {P[:,0].max():.3f} t · "
              f"총질량 {P[:,1].min():.3f} ~ {P[:,1].max():.3f} t")
        print(f"    HV(40,250) {hv40(ok):,.1f} (클리핑 {nclip}건) · "
              f"HV(50,250) {hv50(P):,.1f}")

    # 프론트 CSV
    p = os.path.join(OUT, "a8_pareto.csv")
    cols = list(res["a"][0].keys()) + ["subset", "rank", "t_mm"]
    with open(p, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=cols)
        w.writeheader()
        for tag in ("a", "b"):
            for i, r in enumerate(res[tag], 1):
                d = float(r["bore_mm"])
                idm = a8_eval.shaft_id(d / 1e3) * 1e3
                w.writerow(dict(r, subset=f"z1>={1.0 if tag=='a' else 1.5}",
                                rank=i, t_mm=round((d - idm) / 2, 1)))
    print(f"\n[저장] {p}")

    # 세대별 진행 — 10세대 간격 + 최솟값 갱신 세대
    G = rd(GENLOG)
    star, b1, b2 = set(), 1e9, 1e9
    for g in G:
        f1, f2 = float(g["f1_min"]), float(g["f2_min"])
        if f1 < b1 - 1e-9 or f2 < b2 - 1e-9:
            star.add(int(g["gen"]))
        b1, b2 = min(b1, f1), min(b2, f2)
    print(f"\n갱신(★) 세대 {sorted(star)}")
    print(f"최종 HV {float(G[-1]['hv']):,.2f} · 프론트 {G[-1]['n_front']}")


if __name__ == "__main__":
    main()
