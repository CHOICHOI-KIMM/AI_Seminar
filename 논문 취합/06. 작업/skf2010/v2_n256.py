"""§5.6.3 5. — 격자 N 효과: 구면 캐프 · Hooke & Li · 패치 2a×2a 그대로, N 128 → 256 (사용자 결정 2026-10-11).

압력 곡선의 계단·떨림이 격자(접촉 경계 표본화)에서 오는지 판별한다.
지표(y = 0, |x/b| ≤ 0.5): 최대 p/p_h · 압력의 짧은 파장(λ < 10 μm) 성분 RMS · 간극 봉우리 · φ_bl.
그림: 3행×3열 — 열 = 식 (24) N128 · H/A N128 · H/A N256 (2행 3열 타원 반구 N256 은 미계산).
    python v2_n256.py [plot]  ->  ref_data/V2_lines_n256.npz · V2_n256.json · figures/V2_fig7b_panels3.png
"""
import io
import json

import matplotlib
import numpy as np

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

import load_sharing as LS  # noqa: E402
from v2_fig7 import CASE, N, bump, read_skf  # noqa: E402

N2 = 256


def hf_rms(xb, p, lam_cut=10e-6):
    """중앙 구간(|x/b| ≤ 0.5)의 짧은 파장(λ < lam_cut) 성분 RMS — 등간격 FFT 고역통과."""
    m = np.abs(xb) <= 0.5
    q = p[m] - p[m].mean()
    dx = (xb[1] - xb[0]) * CASE.a_x
    k = np.fft.rfftfreq(len(q), d=dx)
    Q = np.fft.rfft(q)
    Q[k < 1.0 / lam_cut] = 0.0
    return float(np.sqrt(np.mean(np.fft.irfft(Q, len(q)) ** 2)))


def metrics(xb, p, hh, phi):
    return dict(phi_bl=float(phi), p_max=float(p.max()), x_pmax=float(xb[np.argmax(p)]),
                p_hf_rms=hf_rms(xb, p), h_max_lead=float(hh[xb < 0].max()), h_max_trail=float(hh[xb > 0].max()))


def compute():
    a = CASE.a_x
    L = 2.0 * a
    hs = a**2 / CASE.Rx
    x = (np.arange(N2) - N2 // 2) * L / N2
    X, Y = np.meshgrid(x, x)
    o = LS.solve(-bump("cap", np.hypot(X, Y)), L, CASE, s0_mode="hooke_li")
    xb, j = x / a, N2 // 2
    lines = {"xb": xb, "p_hertz": o["p_hertz"][j] / CASE.p_h, "p_mean": o["p_mean"][j] / CASE.p_h,
             "h": o["h_tran"][j] / hs}
    np.savez("ref_data/V2_lines_n256.npz", **lines)
    L1 = np.load("ref_data/V2_lines.npz")
    res = {"N128": metrics(L1["xb"], L1["hooke_li_cap_p_hertz"], L1["hooke_li_cap_h"],
                           json.load(io.open("ref_data/V2_results.json", encoding="utf-8"))["hooke_li_cap"]["phi_bl"]),
           "N256": metrics(xb, lines["p_hertz"], lines["h"], o["phi_bl"]),
           "N256_converged": bool(o["converged"])}
    res["skf_eq24_N128"] = metrics(L1["xb"], L1["skf_cap_p_hertz"], L1["skf_cap_h"],
                                   json.load(io.open("ref_data/V2_results.json", encoding="utf-8"))["skf_cap"]["phi_bl"])
    json.dump(res, io.open("ref_data/V2_n256.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    for k, v in res.items():
        print(k, v if isinstance(v, bool) else {q: round(w, 4) for q, w in v.items()})


def plot():
    L1 = np.load("ref_data/V2_lines.npz")
    L2 = np.load("ref_data/V2_lines_n256.npz")
    skf = read_skf()
    BL, GR = "#1f3a93", "#2e7d4f"
    plt.rcParams["font.family"] = "Malgun Gothic"
    plt.rcParams["axes.unicode_minus"] = False
    fig, axs = plt.subplots(3, 3, figsize=(17.0, 9.6), sharex=True)
    cols = [("SKF 식 (24) · N128", L1, "skf", 128), ("Hooke & Li H/A · N128", L1, "hooke_li", 128),
            ("Hooke & Li H/A · N256", L2, None, 256)]
    for c, (cname, D, mode, n) in enumerate(cols):
        rows = [("SKF Fig 7(b)", skf["p_ph"], None, skf["hRx_b2"])]
        for shape, nm in (("ellipsoid", "타원 반구"), ("cap", "구면 캐프")):
            if mode is None:
                rows.append((f"본 모델 — {nm} · {cname}",) + ((D["xb"], D["p_hertz"]), (D["xb"], D["p_mean"]),
                            (D["xb"], np.maximum(D["h"], 0.0))) if shape == "cap" else (f"{nm} · N256 미계산", None, None, None))
            else:
                k = f"{mode}_{shape}"
                rows.append((f"본 모델 — {nm} · {cname}", (D["xb"], D[f"{k}_p_hertz"]), (D["xb"], D[f"{k}_p_mean"]),
                             (D["xb"], np.maximum(D[f"{k}_h"], 0.0))))
        for ax, (title, p1, p2, h) in zip(axs[:, c], rows):
            ax.set_title(title, fontsize=11)
            if p1 is None:
                ax.text(0.5, 0.5, "미계산 (사용자 범위: 구면 캐프만)", ha="center", va="center", transform=ax.transAxes,
                        color="0.5")
                ax.set(xlim=(-1, 1), ylim=(0, 7))
                continue
            ax2 = ax.twinx()
            ax.plot(*p1, color=BL, lw=1.3, label="p/p_h" if p2 is None else "p/p_h (Hertz 안)")
            if p2 is not None:
                ax.plot(*p2, color=BL, lw=1.1, ls="--", label="p/p_h (p_mean 안)")
            ax2.plot(*h, color=GR, lw=1.3, label="hR_x/b²")
            ax.set(xlim=(-1, 1), ylim=(0, 7))
            ax2.set_ylim(0, 1)
            ax.set_ylabel("p/p_h", color=BL)
            ax2.set_ylabel("hR_x/b²", color=GR)
            ax.grid(alpha=0.3, ls=":")
            h1, l1 = ax.get_legend_handles_labels()
            h2, l2 = ax2.get_legend_handles_labels()
            ax.legend(h1 + h2, l1 + l2, fontsize=8, loc="upper left")
        axs[-1, c].set_xlabel("x/b")
    fig.tight_layout()
    fig.savefig("figures/V2_fig7b_panels3.png", dpi=150)


if __name__ == "__main__":
    import sys
    if sys.argv[1:] != ["plot"]:
        compute()
    plot()
