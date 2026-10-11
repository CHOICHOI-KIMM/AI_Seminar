"""V2 — SKF Fig 7(b) 범프 예제 점별 비교 (작업내역서 §5 · 계획서 §6 V2).

Deolalikar 범프(높이 1.2 μm · 밑면 반경 18 μm, SKF 추정) · 순수구름 · p_h 0.63 GPa · 패치 2a_x · N 128.
범프 형상 두 안(§5.4 결정 3):
  ellipsoid: z = h √(1 − ρ²/R²)
  cap      : 구면 캐프 (밑면 반경 R · 높이 h · 곡률반경 (R² + h²)/2h)
압력 두 안(U15): hertz = 변동분 + Hertz 분포, mean = 변동분 + p_h. 간극은 h_tran.
축: x/a_x · p/p_h · hR_x/a_x² (Fig 7 의 b = 본 문서 a_x).

    python v2_fig7.py [plot]  ->  ref_data/V2_lines.npz · V2_results.json · figures/V2_fig7b_panels.png
    (plot: 저장된 선만 다시 그림)
"""
from __future__ import annotations

import json

import matplotlib
import numpy as np

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

import conditions as CO  # noqa: E402
import load_sharing as LS  # noqa: E402

CASE = CO.DEOLALIKAR
N, H_BUMP, R_BUMP = 128, 1.2e-6, 18e-6


def bump(shape, rho):
    if shape == "ellipsoid":
        return H_BUMP * np.sqrt(np.clip(1.0 - (rho / R_BUMP) ** 2, 0.0, None))
    Rs = (R_BUMP**2 + H_BUMP**2) / (2.0 * H_BUMP)
    return np.where(rho < R_BUMP, np.sqrt(np.clip(Rs**2 - rho**2, 0.0, None)) - (Rs - H_BUMP), 0.0)


def read_skf():
    """digitize_fig7 CSV → {곡선: (x/b, 값)} (급변 열은 같은 x 두 점)."""
    d = np.genfromtxt("ref_data/V2_skf_fig7b.csv", delimiter=",", skip_header=2, dtype=None, encoding="utf-8")
    return {k: (np.array([r[1] for r in d if r[0] == k]), np.array([r[2] for r in d if r[0] == k]))
            for k in ("p_ph", "hRx_b2")}


def compute():
    a = CASE.a_x
    L = 2.0 * a
    x = (np.arange(N) - N // 2) * L / N
    X, Y = np.meshgrid(x, x)
    hs = a**2 / CASE.Rx                                      # hR_x/b² 의 환산 기준
    xb, j, out, lines = x / a, N // 2, {}, {"xb": x / a}
    for shape in ("ellipsoid", "cap"):
        o = LS.solve(-bump(shape, np.hypot(X, Y)), L, CASE)
        res = dict(phi_bl=o["phi_bl"], c_rho=o["c_rho"], converged=o["converged"],
                   contact_area=float(np.mean(o["dry"]["p"] > 0)), h_mean_nm=float(o["h_mean"] * 1e9))
        for var in ("hertz", "mean"):
            p = o[f"p_{var}"][j] / CASE.p_h
            lines[f"{shape}_p_{var}"] = p
            res[f"p_max_{var}"] = float(p.max())
            res[f"x_pmax_{var}"] = float(xb[np.argmax(p)])
        hh = o["h_tran"][j] / hs
        lines[f"{shape}_h"] = hh
        for side, m in (("lead", xb < 0), ("trail", xb > 0)):          # 입구쪽(x<0) · 출구쪽(x>0)
            res[f"h_max_{side}"] = float(hh[m].max())
            res[f"x_hmax_{side}"] = float(xb[m][np.argmax(hh[m])])
        zc = xb[hh <= 0.0]
        res["h_le0_x"] = [float(zc.min()), float(zc.max())] if zc.size else None
        res["h_far"] = float(np.mean(hh[np.abs(xb) > 0.8]))
        out[shape] = res
        print(shape, {k: (round(v, 4) if isinstance(v, float) else v) for k, v in res.items()})
    skf = read_skf()
    (xp, pp), (xh, hh) = skf["p_ph"], skf["hRx_b2"]
    out["skf"] = dict(p_max=float(pp.max()), x_pmax=float(xp[np.argmax(pp)]),
                      h_max_trail=float(hh[xh > 0].max()), x_hmax_trail=float(xh[xh > 0][np.argmax(hh[xh > 0])]),
                      h_max_lead=float(hh[xh < 0].max()), x_hmax_lead=float(xh[xh < 0][np.argmax(hh[xh < 0])]),
                      h_le0_x=[float(xh[hh <= 0.005].min()), float(xh[hh <= 0.005].max())],
                      h_far=float(hh[np.abs(xh) > 0.8].mean()), p_far=float(pp[np.abs(xp) > 0.8].mean()))
    print("SKF", out["skf"])
    np.savez("ref_data/V2_lines.npz", **lines)
    with open("ref_data/V2_results.json", "w", encoding="utf-8") as f:
        json.dump(out, f, ensure_ascii=False, indent=1)


def plot():
    """세로 3단: SKF Fig 7(b) / 타원 반구 / 구면 캐프 — SKF 와 같은 축·색(파랑 p/p_h · 초록 hR_x/b²).
    간극은 max(h_tran, 0)(유량평형 'free space' 와 같은 양의 간극, 사용자 결정 2026-10-09)."""
    L_ = np.load("ref_data/V2_lines.npz")
    skf = read_skf()
    BL, GR = "#1f3a93", "#2e7d4f"
    panels = [("SKF Fig 7(b)", skf["p_ph"], None, skf["hRx_b2"]),
              ("본 모델 — 타원 반구", (L_["xb"], L_["ellipsoid_p_hertz"]), (L_["xb"], L_["ellipsoid_p_mean"]),
               (L_["xb"], np.maximum(L_["ellipsoid_h"], 0.0))),
              ("본 모델 — 구면 캐프", (L_["xb"], L_["cap_p_hertz"]), (L_["xb"], L_["cap_p_mean"]),
               (L_["xb"], np.maximum(L_["cap_h"], 0.0)))]
    plt.rcParams["font.family"] = "Malgun Gothic"
    plt.rcParams["axes.unicode_minus"] = False
    fig, axs = plt.subplots(3, 1, figsize=(6.4, 9.6), sharex=True)
    for ax, (title, p1, p2, h) in zip(axs, panels):
        ax2 = ax.twinx()
        ax.plot(*p1, color=BL, lw=1.3, label="p/p_h" if p2 is None else "p/p_h (Hertz 안)")
        if p2 is not None:
            ax.plot(*p2, color=BL, lw=1.1, ls="--", label="p/p_h (p_mean 안)")
        ax2.plot(*h, color=GR, lw=1.3, label="hR_x/b²")
        ax.set(xlim=(-1, 1), ylim=(0, 7))
        ax2.set_ylim(0, 1)
        ax.set_ylabel("p/p_h", color=BL)
        ax2.set_ylabel("hR_x/b²", color=GR)
        ax.set_title(title, fontsize=11)
        ax.grid(alpha=0.3, ls=":")
        h1, l1 = ax.get_legend_handles_labels()
        h2, l2 = ax2.get_legend_handles_labels()
        ax.legend(h1 + h2, l1 + l2, fontsize=8, loc="upper left")
    axs[-1].set_xlabel("x/b")
    fig.tight_layout()
    fig.savefig("figures/V2_fig7b_panels.png", dpi=150)


if __name__ == "__main__":
    import sys
    if sys.argv[1:] != ["plot"]:
        compute()
    plot()
