# -*- coding: utf-8 -*-
"""§10-12.6 그림 — 프로파일 형상(위) · 길이방향 응력분포(아래)"""
import json
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
SRC = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "fujiwara",
                   "final.json")
OUT = os.path.join(HERE, "figures", "fujiwara_a10.png")
STY = {"기준 (DIN Lundberg)": ("0.35", "-", 2.0),
       "대칭 최적": ("tab:orange", "--", 1.8),
       "비대칭 최적": ("tab:red", "-", 2.2)}

D = json.load(open(SRC, encoding="utf-8"))
ranks = sorted(D, key=lambda k: int(k))
fig, ax = plt.subplots(2, len(ranks), figsize=(15.5, 8.0),
                       gridspec_kw=dict(height_ratios=[1, 1.35]))

for j, rk in enumerate(ranks):
    a0, a1 = ax[0][j], ax[1][j]
    half = None
    for rec in D[rk]:
        c, ls, lw = STY.get(rec["tag"], ("tab:blue", "-", 1.5))
        if rec["profile"]:
            y = [p[0] for p in rec["profile"]]
            z = [p[1] for p in rec["profile"]]
            a0.plot(y, z, ls, color=c, lw=lw, label=rec["tag"])
        d = [(p, s) for p, s in rec["dist"]]
        a1.plot([p for p, _ in d], [s for _, s in d], ls, color=c, lw=lw,
                label=rec["tag"])
        half = rec["metrics"]["L_we_mm"] / 2.0
        # 최대응력 위치 표시
        ys = rec["metrics"]["y_star_mm"]
        if ys is not None:
            a1.plot([ys], [rec["metrics"]["sigma_MPa"]], "o", color=c,
                    ms=7, mec="white", mew=1.2, zorder=5)
    a0.invert_yaxis()
    a0.set_title(f"#{rk}   ($L_{{we}}$ = {2*half:.1f} mm)", fontsize=12)
    a0.set_ylabel("낙차 $z$ [μm]" if j == 0 else "")
    a0.grid(alpha=0.3)
    a0.axvline(0, color="0.8", lw=0.8)
    a1.axhline(2100, color="tab:red", ls=":", lw=1.2)
    a1.text(0.02, 2100, " σ ≤ 2,100", fontsize=8, color="tab:red",
            va="bottom", transform=a1.get_yaxis_transform())
    a1.set_xlabel("롤러 축방향 위치 $y$ [mm]  (음수 = 하중 치우침 쪽)")
    a1.set_ylabel("내륜 접촉응력 [MPa]" if j == 0 else "")
    a1.grid(alpha=0.3)
    a1.set_ylim(0, 2450)
    for a in (a0, a1):
        a.set_xlim(-half * 1.03, half * 1.03)
ax[0][0].legend(fontsize=8.5, loc="lower center", framealpha=0.9)
ax[1][0].legend(fontsize=8.5, loc="lower left", framealpha=0.9)
fig.suptitle("Fujiwara 수정 로그 프로파일 — 형상(위)과 그 결과인 "
             "길이방향 응력분포(아래) · ● = 최대응력 위치", fontsize=13)
fig.tight_layout(rect=(0, 0, 1, 0.95))
os.makedirs(os.path.dirname(OUT), exist_ok=True)
fig.savefig(OUT, dpi=150)
print("[저장]", OUT)
