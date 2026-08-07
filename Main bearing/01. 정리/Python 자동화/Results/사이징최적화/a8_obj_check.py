# -*- coding: utf-8 -*-
"""목적함수 설정 검토 — 같은 평가자료에서 네 가지 정식화를 비교한다"""
import csv
import os

import numpy as np

H = (r"d:\AI\AI_Seminar\Main bearing\01. 정리\Python 자동화"
     r"\Results\사이징최적화")
SETS = (("부록 8 S3-c (C6 유지)", os.path.join(
    H, "부록8_NSGA", "S3_본최적화", "eval_cache.csv")),
        ("§8-8 (C6 해제)", os.path.join(
            H, "부록8_DPW", "S3_C6해제", "eval_cache.csv")))


def rd(p):
    return list(csv.DictReader(open(p, encoding="utf-8-sig")))


def front(P):
    """2목적 최소화 비지배집합 인덱스"""
    k = []
    for i in range(len(P)):
        d = ((P[:, 0] <= P[i, 0]) & (P[:, 1] <= P[i, 1])
             & ((P[:, 0] < P[i, 0]) | (P[:, 1] < P[i, 1])))
        if not d.any():
            k.append(i)
    return k


def uniq(P, k):
    s, o = set(), []
    for i in sorted(k, key=lambda j: (P[j, 0], P[j, 1])):
        t = (round(P[i, 0], 6), round(P[i, 1], 6))
        if t not in s:
            s.add(t)
            o.append(i)
    return o


for tag, p in SETS:
    R = [r for r in rd(p)
         if 0 < float(r["sigma_max_MPa"]) < 2100 and float(r["z1"]) >= 1.0]
    b = np.array([float(r["mass_brg_kg"]) for r in R]) / 1e3
    s = np.array([float(r["mass_shaft_kg"]) for r in R]) / 1e3
    t = np.array([float(r["mass_total_kg"]) for r in R]) / 1e3
    print(f"\n{'='*66}\n{tag} · 가능해 {len(R):,}")
    print(f"  상관  베어링↔샤프트 {np.corrcoef(b, s)[0,1]:+.3f} · "
          f"베어링↔총질량 {np.corrcoef(b, t)[0,1]:+.3f} · "
          f"샤프트↔총질량 {np.corrcoef(s, t)[0,1]:+.3f}")

    # 네 정식화
    fA = uniq(np.c_[b, t], front(np.c_[b, t]))          # 현행
    fB = uniq(np.c_[b, s], front(np.c_[b, s]))          # 베어링 vs 샤프트
    iT, iB = int(np.argmin(t)), int(np.argmin(b))
    print(f"  ⒜ (베어링, 총질량)  프론트 {len(fA):3}건")
    print(f"  ⒝ (베어링, 샤프트)  프론트 {len(fB):3}건")
    print(f"  ⒞ 총질량 단일       최적 1점  베어링 {b[iT]:6.2f} · "
          f"샤프트 {s[iT]:6.2f} · 총 {t[iT]:7.2f}")
    print(f"  ⒟ 베어링 단일       최적 1점  베어링 {b[iB]:6.2f} · "
          f"샤프트 {s[iB]:6.2f} · 총 {t[iB]:7.2f}  "
          f"← 총질량 {t[iB]-t[iT]:+.2f} t ({100*(t[iB]-t[iT])/t[iT]:+.1f}%)")

    # 베어링 단일목적으로 뽑으면 샤프트가 어떻게 되는가 (상위 N)
    o = np.argsort(b)
    for n in (1, 10, 50):
        j = o[:n]
        print(f"     베어링 최경량 {n:2}건 평균: 베어링 {b[j].mean():6.2f} · "
              f"샤프트 {s[j].mean():6.2f} · 총 {t[j].mean():7.2f}")

    # 슬라이스별 베어링↔샤프트
    z2s = sorted({float(r["z2"]) for r in R})
    print("   z2 별 베어링↔샤프트 상관")
    for z in z2s:
        m = np.array([float(r["z2"]) == z for r in R])
        if m.sum() < 30:
            continue
        print(f"     z2 {z:.1f} ({int(m.sum()):5,}점) "
              f"{np.corrcoef(b[m], s[m])[0,1]:+.3f}")
