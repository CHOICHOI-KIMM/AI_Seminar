"""
부록 F-4 설명용 시각화 (A1)
==================================
A1  시계열 + 샘플링점 오버레이  → 에일리어싱을 눈으로 확인
데이터: Results/*_dt_*.csv (MASTA 불필요). 재실행하면 전량 재생성.
"""
import csv
import glob
import math
import os

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import numpy as np

from c1_aiso import C_N, P_EXP

plt.rcParams["font.family"] = "Malgun Gothic"
plt.rcParams["axes.unicode_minus"] = False

OUT = os.path.join("Results", "figures")
os.makedirs(OUT, exist_ok=True)
BEARING = "Main Bearing_UW"
REF_DT = 0.1
DT0 = 0.1
C_RED, C_BLU, C_GRY = "#c0392b", "#2471a3", "#95a5a6"


def plain_log(ax, axis="both"):
    """로그축 눈금을 ASCII 숫자로 (Malgun Gothic 위첨자 마이너스 결손 회피)."""
    f = mticker.FuncFormatter(lambda v, _: ("%g" % v))
    for a in ((ax.xaxis,) if axis == "x" else (ax.yaxis,) if axis == "y"
              else (ax.xaxis, ax.yaxis)):
        a.set_major_formatter(f)
        a.set_minor_formatter(mticker.NullFormatter())


def dt_of(path):
    s = os.path.basename(path).split("_dt_")[1].split("_summary")[0].replace(".csv", "")
    return float(s)


def load_eps():
    """{dt: ε%} — UW ISO16281 modified 표본손상, 기준 dt=0.1"""
    d = {}
    for f in glob.glob(os.path.join("Results", "*_dt_*_summary.csv")):
        for r in csv.reader(open(f, encoding="utf-8-sig")):
            if len(r) > 3 and r[0] == BEARING and r[1] == "ISO16281" and r[2] == "modified":
                try:
                    d[dt_of(f)] = float(r[3])
                except ValueError:
                    pass
    ref = d[REF_DT]
    return {k: (v - ref) / ref * 100 for k, v in sorted(d.items())}


def load_series():
    """dt=0.1 전량에서 t, rpm, P(UW) 복원."""
    rows = [r for r in csv.DictReader(
        open(os.path.join("Results", "Fatigue_DLC1.2-c-s1_dt_0.1.csv"), encoding="utf-8-sig"))
        if r["bearing"] == BEARING]
    rows.sort(key=lambda r: int(r["index"]))
    t = np.array([float(r["t_s"]) for r in rows])
    rpm = np.array([float(r["rpm"]) for r in rows])
    P = np.array([C_N / ((float(r["L10_basic_rev"]) / 1e6) ** (1 / P_EXP)) for r in rows])
    return t, rpm, P


def sample_idx(n_full, dt):
    last = n_full - 1
    N = int(round(last * DT0 / dt)) + 1
    return np.array([int(round(j * last / (N - 1))) for j in range(N)])


# ───────────────────────── A1 ─────────────────────────
def fig_a1(t, P, T3P, eps):
    win = 120.0
    m = t <= win
    tw, pw = t[m], P[m] / 1e3
    fig, ax = plt.subplots(3, 1, figsize=(10, 8.4), sharex=True, sharey=True)

    ax[0].plot(tw, pw, color=C_GRY, lw=1.0)
    ax[0].set_title(f"(a) 원 시계열 — Δt = 0.1 s (전량 {len(tw)}점),  "
                    f"3P 주기 = {T3P:.3f} s", fontsize=11, loc="left")
    for k in range(int(win / T3P) + 1):
        ax[0].axvline(k * T3P, color="#d5d8dc", lw=0.7, zorder=0)
    tr = ax[0].get_xaxis_transform()
    ax[0].annotate("", xy=(0, 0.88), xytext=(T3P, 0.88), xycoords=tr,
                   arrowprops=dict(arrowstyle="<->", color=C_BLU, lw=1.6))
    ax[0].text(T3P / 2, 0.91, f"3P = {T3P:.2f} s", color=C_BLU, transform=tr,
               ha="center", va="bottom", fontsize=9.5, fontweight="bold")

    for k, (dt, c, lab) in enumerate(((4.0, C_RED, "심각한 에일리어싱"),
                                      (1.0, C_BLU, "정상 추종")), start=1):
        idx = sample_idx(len(t), dt)
        idx = idx[t[idx] <= win]
        ax[k].plot(tw, pw, color="#dfe3e6", lw=1.0, zorder=1)
        ax[k].plot(t[idx], P[idx] / 1e3, "-o", color=c, ms=4.5, lw=1.6, zorder=3)
        ax[k].set_title(f"({'bc'[k-1]}) Δt = {dt:g} s  →  Δt/T_3P = {dt/T3P:.3f},  "
                        f"3P 주기당 {T3P/dt:.2f}점,  손상오차 ε = {eps[dt]:+.2f}%   ▶ {lab}",
                        fontsize=11, loc="left", color=c)
        ax[k].set_ylabel("등가하중 P [kN]")
    ax[0].set_ylabel("등가하중 P [kN]")
    ax[2].set_xlabel("시간 [s]")
    ax[2].set_xlim(0, win)
    for a in ax:
        a.grid(alpha=0.25)
    fig.suptitle("A1. 샘플링 간격에 따른 하중 파형 추종성 — 에일리어싱의 발생",
                 fontsize=13, y=0.985)
    fig.tight_layout(rect=(0, 0, 1, 0.972))
    p = os.path.join(OUT, "F4_A1_timeseries.png")
    fig.savefig(p, dpi=150); plt.close(fig)
    return p


def main():
    eps = load_eps()
    t, rpm, P = load_series()
    rpm_mean = float(rpm.mean())
    T3P = 20.0 / rpm_mean
    print(f"평균 rpm={rpm_mean:.4f}  T_3P={T3P:.4f}s  권장 Δt≤{5/rpm_mean:.3f}s")
    print("ε:", {k: round(v, 3) for k, v in eps.items()})
    print("[생성]", fig_a1(t, P, T3P, eps))


if __name__ == "__main__":
    main()
