# -*- coding: utf-8 -*-
"""§8-6.5 를 위한 프론트 분석"""
import csv
import math
import os
import sys

import numpy as np

H = (r"d:\AI\AI_Seminar\Main bearing\01. 정리\Python 자동화"
     r"\Results\사이징최적화")
sys.path.insert(0, H)
import sizing_geom as sg  # noqa: E402

D = os.path.join(H, "부록8_NSGA", "S3_본최적화")
P = [r for r in csv.DictReader(open(os.path.join(D, "a8_pareto.csv"),
                                    encoding="utf-8-sig"))
     if r["subset"] == "z1>=1.0"]
C = list(csv.DictReader(open(os.path.join(D, "eval_cache.csv"),
                             encoding="utf-8-sig")))
fe = [r for r in C if 0 < float(r["sigma_max_MPa"]) < 2100
      and float(r["z1"]) >= 1.0]

print(f"SHAFT_TAIL = {sg.SHAFT_TAIL} m · t_i/t_o = {sg.TI_OVER_B:.5f}"
      f"/{sg.TO_OVER_C:.5f} · ID_OVER_OD = {sg.ID_OVER_OD}")


def f(r, k):
    return float(r[k])


# ── ① 샤프트 질량 분해: 단면 vs 길이 ────────────────────────────────
print("\n[①] 샤프트 질량 분해  m = ρ·(π/4)(d²−ID²)·(z2+tail)")
a, b = P[0], P[-1]
for tag, r in (("#1", a), ("#8", b)):
    d_ = f(r, "bore_mm")
    idm = d_ - 2 * f(r, "t_mm")
    A = math.pi / 4 * (d_ ** 2 - idm ** 2) / 1e6      # m²
    L = f(r, "z2") + sg.SHAFT_TAIL
    print(f"  {tag}  d {d_:,.0f} · ID {idm:,.0f} · A {A:.4f} m² · "
          f"L {L:.2f} m · m {A*L*7850/1e3:.2f} t "
          f"(실측 {f(r,'mass_shaft_kg')/1e3:.2f})")
A1 = math.pi / 4 * (f(a, "bore_mm") ** 2
                    - (f(a, "bore_mm") - 2 * f(a, "t_mm")) ** 2) / 1e6
A8 = math.pi / 4 * (f(b, "bore_mm") ** 2
                    - (f(b, "bore_mm") - 2 * f(b, "t_mm")) ** 2) / 1e6
L1, L8 = f(a, "z2") + sg.SHAFT_TAIL, f(b, "z2") + sg.SHAFT_TAIL
m1, m8 = A1 * L1 * 7.85, A8 * L8 * 7.85
print(f"  단면만 바꾼 경우 {A8*L1*7.85:.2f} t (기여 {A8*L1*7.85-m1:+.2f})")
print(f"  길이만 바꾼 경우 {A1*L8*7.85:.2f} t (기여 {A1*L8*7.85-m1:+.2f})")
print(f"  전체 {m1:.2f} → {m8:.2f} t ({m8-m1:+.2f})")

# ── ② d 는 무엇이 정하는가 ──────────────────────────────────────────
print("\n[②] d = D_pw − D_we·cosα − L_w·sinα − 2·t_i  (해석식 확인)")
print("   #   D_pw    α   D_we    L_w   Dwe·cosα  Lw·sinα   2t_i     d(식)"
      "    d(실측)")
for i, r in enumerate(P, 1):
    dpw, al = f(r, "D_pw_mm"), f(r, "alpha")
    dwe, lw = f(r, "D_we_mm"), f(r, "L_w_mm")
    ca, sa = math.cos(math.radians(al)), math.sin(math.radians(al))
    ti = sg.TI_OVER_B * f(r, "B_mm")
    print(f"  {i}  {dpw:6,.0f}  {al:3.0f}  {dwe:6.1f}  {lw:6.1f}  "
          f"{dwe*ca:8.1f}  {lw*sa:7.1f}  {2*ti:6.1f}  "
          f"{dpw-dwe*ca-lw*sa-2*ti:8.1f}  {f(r,'bore_mm'):8,.0f}")

X = np.array([[f(r, "alpha"), f(r, "L_w_mm"), f(r, "D_we_mm"),
               f(r, "bore_mm"), f(r, "z2"), f(r, "mass_brg_kg") / 1e3,
               f(r, "mass_total_kg") / 1e3] for r in P])
nm = ["α", "L_w", "D_we", "d", "z2", "베어링", "총질량"]
print("\n  프론트 8점 상관 (d 기준)")
for i in range(len(nm)):
    if nm[i] == "d":
        continue
    print(f"    d ↔ {nm[i]:5s} {np.corrcoef(X[:,3], X[:,i])[0,1]:+.3f}")

# ── ③ z2 계단 — 프론트가 갈라지는 이유 ──────────────────────────────
print("\n[③] z2 수준별 가능해와 최적점")
for z in sorted({f(r, "z2") for r in fe}):
    S = [r for r in fe if f(r, "z2") == z]
    mt = np.array([f(r, "mass_total_kg") / 1e3 for r in S])
    mb = np.array([f(r, "mass_brg_kg") / 1e3 for r in S])
    j = int(np.argmin(mt))
    onf = sum(1 for r in P if f(r, "z2") == z)
    print(f"  z2 {z:.1f}  가능해 {len(S):5,} · 총질량 최소 {mt.min():7.2f} t "
          f"(베어링 {mb[j]:5.2f}) · 프론트 {onf}점")

# ── ④ 수렴 — 후반 개선 ──────────────────────────────────────────────
G = list(csv.DictReader(open(os.path.join(D, "s3_genlog.csv"),
                             encoding="utf-8-sig")))
hv = np.array([float(g["hv"]) for g in G])
f2 = np.array([float(g["f2_min"]) for g in G])
f1 = np.array([float(g["f1_min"]) for g in G])
print(f"\n[④] 수렴  최종 HV {hv[-1]:,.3f}")
for g0 in (100, 120, 140, 149):
    print(f"  {g0}→150 세대 HV {hv[-1]-hv[g0-1]:+.3f} · "
          f"f1 {f1[-1]-f1[g0-1]:+.4f} · f2 {f2[-1]-f2[g0-1]:+.4f}")
last = max(i for i in range(1, len(hv)) if hv[i] - hv[i-1] > 1e-9)
print(f"  HV 가 조금이라도 오른 마지막 세대 {last+1} · "
      f"프론트 크기 {G[-1]['n_front']}")

# ── ⑤ 목적 두 개의 상관 ─────────────────────────────────────────────
mb = np.array([f(r, "mass_brg_kg") for r in fe]) / 1e3
mt = np.array([f(r, "mass_total_kg") for r in fe]) / 1e3
print(f"\n[⑤] 가능해 {len(fe):,}점에서 f1↔f2 상관 {np.corrcoef(mb, mt)[0,1]:+.3f}")
for z in sorted({f(r, "z2") for r in fe}):
    S = [r for r in fe if f(r, "z2") == z]
    x = np.array([f(r, "mass_brg_kg") for r in S]) / 1e3
    y = np.array([f(r, "mass_total_kg") for r in S]) / 1e3
    print(f"   z2 {z:.1f} 안에서만 {np.corrcoef(x, y)[0,1]:+.3f} ({len(S):,}점)")
