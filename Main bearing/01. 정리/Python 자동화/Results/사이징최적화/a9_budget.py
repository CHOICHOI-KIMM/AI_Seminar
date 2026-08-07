# -*- coding: utf-8 -*-
"""부록 9 설정에서 해석식 통과율 · 제약 활성도 · 예산 추정"""
import os
import sys

import numpy as np

H = (r"d:\AI\AI_Seminar\Main bearing\01. 정리\Python 자동화"
     r"\Results\사이징최적화")
sys.path.insert(0, H)
import nsga_eval as ne     # noqa: E402
import nsga_s3_run as S3   # noqa: E402
import sizing_geom as sg   # noqa: E402

Z1 = 1.0
Z2 = [round(3.5 + 0.1 * i, 1) for i in range(26)]
AL = list(range(15, 31))
DPW_LO, DPW_HI = 3300, 4800
DCAP = 5.0            # m — 새 C6

rng = np.random.default_rng(0)
N = 60000
cnt, ok = {}, 0
for _ in range(N):
    z2 = float(rng.choice(Z2))
    al = int(rng.integers(15, 31))
    dpw = int(rng.integers(DPW_LO, DPW_HI + 1))
    dwe = int(rng.integers(S3.DWE_LO, S3.DWE_HI + 1))
    u = int(rng.integers(0, S3.U_MAX + 1))
    lwe = S3.lwe_of(dwe, u)
    g = ne.geom(dpw / 1e3, al, dwe / 1e4, lwe / 1e4, True)
    v = [x for x in sg.constraints(g, Z1, z2)
         if not x.startswith(("C4", "C5", "C6"))]
    if g["outer_diameter"] > DCAP:
        v.append("C6 D >5000mm")
    for x in v:
        cnt[x] = cnt.get(x, 0) + 1
    if not v:
        ok += 1

print(f"표본 {N:,} · 해석식 통과 {ok:,} ({100*ok/N:.1f}%) · "
      f"탈락 {100*(N-ok)/N:.1f}%")
print("제약별 활성 빈도 (중복 포함)")
for k, v in sorted(cnt.items(), key=lambda t: -t[1]):
    print(f"  {k:26s} {v:7,} ({100*v/N:5.1f}%)")

print("\nC4·C5·C7·C8·C9 자동만족 확인 (z1 = 1.0 · z2 ≥ 3.5)")
lw_max = min(2.5 * S3.DWE_HI, S3.LWE_HI) / 1e4      # m
T_max = sg.T_OVER_LWE * lw_max
print(f"  L_w 최대 {lw_max*1e3:,.1f} mm → T 최대 {T_max*1e3:,.1f} mm")
print(f"  C4  z2−z1 ≥ 1.5 : 최소 {3.5-1.0:.1f} m  → 항상 만족")
print(f"  C5  z1 ≥ 0.3    : {Z1:.1f} m           → 항상 만족")
print(f"  C7  z2−z1 ≥ T+0.1 : 최소 {3.5-1.0:.2f} ≥ {T_max+0.1:.3f} → "
      f"{'항상 만족' if 2.5 >= T_max+0.1 else '활성 가능'}")
print(f"  C8  z1 ≥ T/2      : {Z1:.1f} ≥ {T_max/2:.3f} → "
      f"{'항상 만족' if Z1 >= T_max/2 else '활성 가능'}")
print(f"  C9  T/2 ≤ 0.5     : {T_max/2:.3f} ≤ 0.5 → "
      f"{'항상 만족' if T_max/2 <= sg.SHAFT_TAIL else '활성 가능'}")

# ── 예산 추정 ─────────────────────────────────────────────────────
P = 100 * ok / N
S_EVAL = 0.32          # s — MASTA 1점 (부록 8 S3-c 0.306 · §8-8 0.352)
print(f"\n예산 추정 (MASTA {S_EVAL:.2f} s/점 · 해석식 통과 {P:.0f}% 가정)")
print("  개체수   세대   총평가   MASTA(추정)   시간(추정)   z2 수준당 개체")
for pop in (104, 130, 156, 182, 224):
    for gen in (60, 100, 150):
        ev = pop * gen
        m = ev * P / 100
        print(f"  {pop:5}  {gen:5}  {ev:8,}  {m:11,.0f}   "
              f"{m*S_EVAL/3600:8.2f}h   {pop/26:8.1f}")
