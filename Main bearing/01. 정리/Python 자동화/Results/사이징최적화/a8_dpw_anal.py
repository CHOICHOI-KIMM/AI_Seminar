# -*- coding: utf-8 -*-
"""§8-8.5 를 위한 분석 — 20세대 시점"""
import csv
import os
import sys

import numpy as np

H = (r"d:\AI\AI_Seminar\Main bearing\01. 정리\Python 자동화"
     r"\Results\사이징최적화")
sys.path.insert(0, H)
import a8_eval  # noqa: E402

NEW = os.path.join(H, "부록8_DPW", "S3_C6해제")
OLD = os.path.join(H, "부록8_DPW", "S3_본최적화")


def rd(p):
    return list(csv.DictReader(open(p, encoding="utf-8-sig")))


def f(r, k):
    return float(r[k])


old_keys = {r["key"] for r in rd(os.path.join(OLD, "eval_cache.csv"))}
A = rd(os.path.join(NEW, "eval_cache.csv"))
run = [r for r in A if r["key"] not in old_keys]      # 이번 실행이 새로 평가
fe = [r for r in run if 0 < f(r, "sigma_max_MPa") < 2100 and f(r, "z1") >= 1.0]
print(f"캐시 {len(A):,} (워밍 {len(old_keys):,}) · 이번 실행 신규 {len(run):,} "
      f"· 그중 가능해 {len(fe):,} ({100*len(fe)/max(len(run),1):.1f}%)")

x = np.array([f(r, "D_pw_mm") for r in fe])
print(f"\n[가능해] D_pw {x.min():,.0f} ~ {x.max():,.0f} · 중앙 "
      f"{np.median(x):,.0f} · 평균 {x.mean():,.0f}")
for lo in range(4400, 5500, 100):
    m = (x >= lo) & (x < lo + 100)
    if m.sum():
        mt = np.array([f(r, "mass_total_kg") for r in fe])[m] / 1e3
        mb = np.array([f(r, "mass_brg_kg") for r in fe])[m] / 1e3
        ms = np.array([f(r, "mass_shaft_kg") for r in fe])[m] / 1e3
        sg_ = np.array([f(r, "sigma_max_MPa") for r in fe])[m]
        print(f"  {lo:,}~{lo+99:,}  {m.sum():5,}점 "
              f"({100*m.mean():4.1f}%)  총질량 {mt.min():7.2f}  "
              f"베어링 {mb.min():6.2f}  샤프트 {ms.min():6.2f}  "
              f"σ중앙 {np.median(sg_):,.0f}")

F = rd(os.path.join(NEW, "s3_checkpoint.csv"))
print(f"\n[프론트] {len(F)}건")
xd = np.array([f(r, "D_pw_mm") for r in F])
print(f"  D_pw {xd.min():,.0f} ~ {xd.max():,.0f} · 중앙 {np.median(xd):,.0f}")
print("  #   D_pw    d     t   d/t   α  D_we   L_w   z1   z2   베어링   샤프트"
      "   총질량    σ")
for i, r in enumerate(sorted(F, key=lambda q: f(q, "mass_brg_t")), 1):
    d_ = f(r, "bore_mm")
    idm = a8_eval.shaft_id(d_ / 1e3) * 1e3
    t = (d_ - idm) / 2
    print(f" {i:2}  {f(r,'D_pw_mm'):,.0f} {d_:,.0f} {t:5.1f} {d_/t:4.1f} "
          f"{f(r,'alpha'):3.0f} {f(r,'D_we_mm'):5.1f} {f(r,'L_we_mm'):5.1f} "
          f"{f(r,'z1'):.1f}  {f(r,'z2'):.1f}  {f(r,'mass_brg_t'):6.2f} "
          f"{f(r,'mass_total_t')-2*f(r,'mass_brg_t'):7.2f} "
          f"{f(r,'mass_total_t'):7.2f} {f(r,'sigma_max_MPa'):7.1f}")

# ── D_pw 가 무엇을 바꾸는가 (가능해 상관) ──────────────────────────
mb = np.array([f(r, "mass_brg_kg") for r in fe]) / 1e3
ms = np.array([f(r, "mass_shaft_kg") for r in fe]) / 1e3
mt = np.array([f(r, "mass_total_kg") for r in fe]) / 1e3
sg_ = np.array([f(r, "sigma_max_MPa") for r in fe])
bo = np.array([f(r, "bore_mm") for r in fe])
print("\n[상관] 가능해에서 D_pw 와")
for nm, v in (("d(bore)", bo), ("베어링질량", mb), ("샤프트질량", ms),
              ("총질량", mt), ("σ", sg_)):
    print(f"   {nm:10s} {np.corrcoef(x, v)[0,1]:+.3f}")

# ── (z1,z2) 슬라이스 안에서 두 목적 ────────────────────────────────
print("\n[슬라이스] (z1,z2) 고정 시 두 목적의 관계")
print(" z1   z2   점수   f1↔f2 상관   베어링폭   샤프트폭   2·베어링폭")
for z1 in sorted({f(r, "z1") for r in fe}):
    for z2 in sorted({f(r, "z2") for r in fe}):
        S = [r for r in fe if f(r, "z1") == z1 and f(r, "z2") == z2]
        if len(S) < 30:
            continue
        a_ = np.array([f(r, "mass_brg_kg") for r in S]) / 1e3
        b_ = np.array([f(r, "mass_total_kg") for r in S]) / 1e3
        c_ = np.array([f(r, "mass_shaft_kg") for r in S]) / 1e3
        print(f" {z1:.1f}  {z2:.1f}  {len(S):5,}   "
              f"{np.corrcoef(a_, b_)[0,1]:+.3f}      "
              f"{a_.max()-a_.min():6.2f}     {c_.max()-c_.min():6.2f}"
              f"     {2*(a_.max()-a_.min()):6.2f}")

# D_pw 까지 고정하면?
print("\n[슬라이스²] (z1,z2,D_pw±5) 까지 고정 시")
print(" z2   D_pw   점수   f1↔f2 상관   베어링폭   샤프트폭")
for z2 in sorted({f(r, "z2") for r in fe}):
    for dp in (4500, 5000, 5480):
        S = [r for r in fe if f(r, "z2") == z2 and f(r, "z1") == 1.0
             and abs(f(r, "D_pw_mm") - dp) <= 5]
        if len(S) < 15:
            continue
        a_ = np.array([f(r, "mass_brg_kg") for r in S]) / 1e3
        b_ = np.array([f(r, "mass_total_kg") for r in S]) / 1e3
        c_ = np.array([f(r, "mass_shaft_kg") for r in S]) / 1e3
        print(f" {z2:.1f}  {dp:,}  {len(S):4,}   "
              f"{np.corrcoef(a_, b_)[0,1]:+.3f}      "
              f"{a_.max()-a_.min():6.2f}     {c_.max()-c_.min():6.2f}")
