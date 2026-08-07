# -*- coding: utf-8 -*-
"""(베어링, 샤프트) 프론트가 총질량 최적을 담는가 · 두 프론트의 포함관계"""
import csv
import os

import numpy as np

H = (r"d:\AI\AI_Seminar\Main bearing\01. 정리\Python 자동화"
     r"\Results\사이징최적화")
SETS = (("부록 8 S3-c", os.path.join(H, "부록8_NSGA", "S3_본최적화",
                                     "eval_cache.csv")),
        ("§8-8 C6해제", os.path.join(H, "부록8_DPW", "S3_C6해제",
                                     "eval_cache.csv")))


def rd(p):
    return list(csv.DictReader(open(p, encoding="utf-8-sig")))


def front_idx(P):
    k = []
    for i in range(len(P)):
        d = ((P[:, 0] <= P[i, 0]) & (P[:, 1] <= P[i, 1])
             & ((P[:, 0] < P[i, 0]) | (P[:, 1] < P[i, 1])))
        if not d.any():
            k.append(i)
    return k


for tag, p in SETS:
    R = [r for r in rd(p)
         if 0 < float(r["sigma_max_MPa"]) < 2100 and float(r["z1"]) >= 1.0]
    b = np.array([float(r["mass_brg_kg"]) for r in R]) / 1e3
    s = np.array([float(r["mass_shaft_kg"]) for r in R]) / 1e3
    t = 2 * b + s

    A = set(front_idx(np.c_[b, t]))      # (베어링, 총질량)
    B = set(front_idx(np.c_[b, s]))      # (베어링, 샤프트)

    def uk(idx, P):
        return {(round(P[i, 0], 6), round(P[i, 1], 6)) for i in idx}

    ua = uk(A, np.c_[b, t])
    ub = uk(B, np.c_[b, s])
    print(f"\n{'='*62}\n{tag} · 가능해 {len(R):,}")
    print(f"  (베어링,총질량) 프론트 {len(ua)}건 · "
          f"(베어링,샤프트) 프론트 {len(ub)}건")
    print(f"  A ⊂ B 인가 (행 기준): A {len(A)} 중 B 에 든 것 "
          f"{len(A & B)} ({100*len(A & B)/len(A):.0f}%)")
    print(f"  총질량 최소점이 B 에 있는가: "
          f"{'예' if int(np.argmin(t)) in B else '아니오'}")
    print(f"  베어링 최소점이 B 에 있는가: "
          f"{'예' if int(np.argmin(b)) in B else '아니오'}")

    # B 프론트 위에서 총질량이 어떻게 퍼지는가
    ib = sorted(B, key=lambda i: b[i])
    tb = t[ib]
    print(f"  B 프론트 위 총질량 {tb.min():.2f} ~ {tb.max():.2f} t "
          f"(최소 총질량 대비 최대 +{tb.max()-tb.min():.2f} t)")
    ia = sorted(A, key=lambda i: b[i])
    ta = t[ia]
    print(f"  A 프론트 위 총질량 {ta.min():.2f} ~ {ta.max():.2f} t")

    # 가중치 w 에 대해 (2b+s 대신 w·b+s) 최적이 B 에 있는가
    hit = sum(1 for w in np.linspace(0.2, 10, 50)
              if int(np.argmin(w * b + s)) in B)
    print(f"  가중합 w·베어링 + 샤프트 (w 0.2~10 · 50개) 최적이 B 에 든 비율 "
          f"{hit}/50")
    hitA = sum(1 for w in np.linspace(0.2, 10, 50)
               if int(np.argmin(w * b + s)) in A)
    print(f"  같은 것이 A 에 든 비율 {hitA}/50")
