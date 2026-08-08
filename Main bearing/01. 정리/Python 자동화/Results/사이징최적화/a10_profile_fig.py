# -*- coding: utf-8 -*-
"""§10-12 그림 — 끝단응력 대 접촉손실 트레이드오프 (σ ≤ 2,100 가능/불가능)"""
import csv
import os

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt      # noqa: E402
from matplotlib import font_manager  # noqa: E402

for f in ("Malgun Gothic", "NanumGothic", "Gulim"):
    if any(f == x.name for x in font_manager.fontManager.ttflist):
        plt.rcParams["font.family"] = f
        break
plt.rcParams["axes.unicode_minus"] = False

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "profile_study.csv")
OUT = os.path.join(HERE, "figures", "profile_a10.png")
LIM = 2100.0
STY = {"DIN_LUNDBERG": ("tab:blue", "o", "DIN Lundberg (axial_offset)"),
       "JOHNS_GOHAR": ("tab:red", "s", "Johns-Gohar (design_load)"),
       "LUNDBERG": ("tab:green", "^", "Lundberg (load)")}

R = list(csv.DictReader(open(SRC, encoding="utf-8-sig")))
ranks = sorted({int(r["rank"]) for r in R})
fig, axes = plt.subplots(1, len(ranks), figsize=(15, 5.0), sharey=True)

for ax, rk in zip(axes, ranks):
    sub = [r for r in R if int(r["rank"]) == rk]
    for prof, (c, m, lab) in STY.items():
        g = [r for r in sub if r["profile"] == prof]
        if not g:
            continue
        x = [float(r["loss_pct"]) for r in g]
        y = [float(r["edge_MPa"]) for r in g]
        ax.plot(x, y, "-", color=c, lw=1.0, alpha=0.45, zorder=1)
        for r, xi, yi in zip(g, x, y):
            ok = float(r["sigma_MPa"]) <= LIM
            ax.scatter(xi, yi, s=68, marker=m, zorder=3,
                       facecolor=c if ok else "none",
                       edgecolor=c, linewidth=1.5,
                       label=lab if r is g[0] else None)
    # 현행 설정 표시
    cur = next(r for r in sub if r["param"] == "axial_offset=0")
    ax.annotate(f"현행\nσ {float(cur['sigma_MPa']):.0f}",
                (float(cur["loss_pct"]), float(cur["edge_MPa"])),
                textcoords="offset points", xytext=(10, 14), fontsize=9,
                color="tab:blue", fontweight="bold",
                arrowprops=dict(arrowstyle="->", color="tab:blue", lw=1.2))
    ax.set_title(f"#{rk}  ($L_{{we}}$ = {float(sub[0]['L_we']):.0f} mm)",
                 fontsize=11)
    ax.set_xlabel("접촉 손실 [% of $L_{we}$]")
    ax.grid(alpha=0.3)
axes[0].set_ylabel("끝단 응력 $\\sigma_{edge,inner}$ [MPa]")
axes[0].legend(fontsize=8.5, loc="upper right", framealpha=0.92)
fig.suptitle("롤러 프로파일 비교 — 끝단 응력 대 접촉 손실 "
             "(채움 = σ$_{max}$ ≤ 2,100 만족 · 빈 표식 = 위반)",
             fontsize=12.5)
fig.tight_layout(rect=(0, 0, 1, 0.94))
os.makedirs(os.path.dirname(OUT), exist_ok=True)
fig.savefig(OUT, dpi=150)
print("[저장]", OUT)
