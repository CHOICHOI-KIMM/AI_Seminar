# -*- coding: utf-8 -*-
"""D = D_pw + (여유) — 여유의 범위와 무엇이 정하는가"""
import csv
import os
import sys

import numpy as np

H = (r"d:\AI\AI_Seminar\Main bearing\01. 정리\Python 자동화"
     r"\Results\사이징최적화")
sys.path.insert(0, H)
import nsga_eval as ne   # noqa: E402
import nsga_s3_run as S3  # noqa: E402


def rd(p):
    return list(csv.DictReader(open(p, encoding="utf-8-sig")))


A = []
for p in (os.path.join(H, "부록8_NSGA", "S3_본최적화", "eval_cache.csv"),
          os.path.join(H, "부록8_DPW", "S3_C6해제", "eval_cache.csv")):
    A += [r for r in rd(p) if float(r["z1"]) >= 1.0]
fe = [r for r in A if 0 < float(r["sigma_max_MPa"]) < 2100]
g = np.array([float(r["D_mm"]) - float(r["D_pw_mm"]) for r in fe])
print(f"가능해 {len(fe):,} · 여유 D − D_pw = {g.min():.0f} ~ {g.max():.0f} mm "
      f"(중앙 {np.median(g):.0f})")
print(f"  D 범위 {min(float(r['D_mm']) for r in fe):,.0f} ~ "
      f"{max(float(r['D_mm']) for r in fe):,.0f}")

# 여유 = D_we·cosα + L_w·sinα + 2·t_o — 격자 전체에서 최소·최대
print("\n격자 전 구간에서 여유의 이론 범위 (해석식 · 정수화 켬)")
lo, hi = 1e9, -1e9
arg = {}
for dwe in range(S3.DWE_LO, S3.DWE_HI + 1, 5):
    for al in S3.AL_OPT:
        for u in (0, S3.U_MAX):
            lwe = S3.lwe_of(dwe, u)
            gg = ne.geom(4.5, al, dwe / 1e4, lwe / 1e4, True)
            d = (gg["outer_diameter"] - gg["D_pw"]) * 1e3
            if d < lo:
                lo, arg["min"] = d, (dwe / 10, al, lwe / 10)
            if d > hi:
                hi, arg["max"] = d, (dwe / 10, al, lwe / 10)
print(f"  최소 {lo:.0f} mm  (D_we {arg['min'][0]:.1f} · α {arg['min'][1]} · "
      f"L_w {arg['min'][2]:.1f})")
print(f"  최대 {hi:.0f} mm  (D_we {arg['max'][0]:.1f} · α {arg['max'][1]} · "
      f"L_w {arg['max'][2]:.1f})")

print("\nD 상한별 · 그 D 에 도달하려면 D_pw 가 어디까지 가야 하는가")
print("  D 상한   D_pw 상한(=D−최소여유)   D_pw 하한 참고(=D−최대여유)")
for cap in (4000, 4500, 5000):
    print(f"  {cap:,}   {cap-lo:>10,.0f}            {cap-hi:>10,.0f}")

# 가능해에서 D 별 최선 총질량 — 이미 답이 얼마나 보이는가
print("\n기존 가능해에서 D 구간별 최선 (참고 · 탐색 편향 있음)")
D = np.array([float(r["D_mm"]) for r in fe])
t = np.array([float(r["mass_total_kg"]) for r in fe]) / 1e3
for c in (4000, 4250, 4500, 4750, 5000, 5250, 5500, 6000):
    m = D <= c
    if m.sum():
        print(f"  D ≤ {c:,}  가능해 {int(m.sum()):6,}  총질량 최소 "
              f"{t[m].min():7.2f} t")
    else:
        print(f"  D ≤ {c:,}  가능해      0")
