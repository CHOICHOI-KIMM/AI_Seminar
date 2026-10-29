"""§6.6.4 — M3 재검토: V2 범프(S = 0)에서 압력 복원 식 (4.2-6)(paper) 과 §7.4.1(direct) 비교 + S = ±1 보완파 참고.

조건은 V2 그대로(N 128 · 패치 2a_x · 범프 중심 = 접촉 중심 · y = 0, 사용자 결정 2026-10-29).
M2 보완파는 웹 계산기와 같은 hooke_li_all(S = 0 은 hooke_li 와 같은 경로). 한 번 푼 해에서 두 압력을 함께 낸다.
    python v2_direct.py [plot]  ->  ref_data/V2_direct.json · V2_direct_lines.npz · figures/V2_fig7b_direct.png
"""
from __future__ import annotations

import json
import sys
import time

import matplotlib
import numpy as np

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

import load_sharing as LS  # noqa: E402
from v2_fig7 import CASE, N, bump, read_skf  # noqa: E402
from v2_n256 import hf_rms  # noqa: E402

SHAPES = ("ellipsoid", "cap")
MODES = ("paper", "direct")


def grid():
    a = CASE.a_x
    L = 2.0 * a
    x = (np.arange(N) - N // 2) * L / N
    return a, L, x, np.meshgrid(x, x)


def skf_rms(xb, p):
    """SKF Fig 7(b) p/p_h 디지타이징 점에서 본 모델(선형 보간)과의 RMS 차 · |x/b| ≤ 1."""
    xs, ps = read_skf()["p_ph"]
    m = (np.abs(xs) <= 1.0) & np.isfinite(ps)
    return float(np.sqrt(np.mean((np.interp(xs[m], xb, p) - ps[m]) ** 2)))


def compute():
    a, L, x, (X, Y) = grid()
    xb, j, hs = x / a, N // 2, a**2 / CASE.Rx
    p_hz = CASE.p_h * np.sqrt(np.clip(1.0 - (X / CASE.a_x) ** 2 - (Y / CASE.a_y) ** 2, 0.0, None))
    clip = lambda p: np.clip(p, 0.0, CASE.p_lim)              # noqa: E731
    out, lines = {}, {"xb": xb}
    for shape in SHAPES:                                          # S = 0 — paper · direct
        t = time.time()
        o = LS.solve(-bump(shape, np.hypot(X, Y)), L, CASE)
        hh = o["h_tran"][j] / hs
        res = dict(phi_bl=o["phi_bl"], converged=o["converged"], h_max_lead=float(hh[xb < 0].max()),
                   h_max_trail=float(hh[xb > 0].max()), h_far=float(np.mean(hh[np.abs(xb) > 0.8])))
        lines[f"{shape}_h"] = hh
        for m in MODES:
            pt = o[f"p_tran_{m}"]
            for var, base in (("hertz", p_hz), ("mean", CASE.p_h)):
                p = clip(base + pt)[j] / CASE.p_h
                lines[f"{shape}_{m}_p_{var}"] = p
                res[f"{m}_p_max_{var}"] = float(p.max())
                res[f"{m}_x_pmax_{var}"] = float(xb[np.argmax(p)])
                res[f"{m}_hf_rms_{var}"] = hf_rms(xb, p)
                res[f"{m}_skf_rms_{var}"] = skf_rms(xb, p)
        res["seconds"] = round(time.time() - t, 1)
        out[f"S0_{shape}"] = res
        print(shape, {k: (round(v, 4) if isinstance(v, float) else v) for k, v in res.items()}, flush=True)
    for S in (1.0, -1.0):                                         # S ≠ 0 참고 — 식 (24) · H/A 근사(구면 캐프, paper)
        for s0 in ("hooke_li", "hooke_li_all"):
            t = time.time()
            o = LS.solve(-bump("cap", np.hypot(X, Y)), L, CASE, S=S, s0_mode=s0)
            p = clip(p_hz + o["p_tran"])[j] / CASE.p_h
            hh = o["h_tran"][j] / hs
            key = f"S{S:+g}_cap_{'eq24' if s0 == 'hooke_li' else 'HA'}"
            out[key] = dict(phi_bl=o["phi_bl"], converged=o["converged"], p_max_hertz=float(p.max()),
                            h_min=float(hh.min()), h_max_trail=float(hh[xb > 0].max()), seconds=round(time.time() - t, 1))
            lines[f"{key}_p_hertz"], lines[f"{key}_h"] = p, hh
            print(key, out[key], flush=True)
    np.savez("ref_data/V2_direct_lines.npz", **lines)
    with open("ref_data/V2_direct.json", "w", encoding="utf-8") as f:
        json.dump(out, f, ensure_ascii=False, indent=1)


def plot():
    """3행×2열(V2 패널 형식): 1행 SKF Fig 7(b) · 2행 타원 반구 · 3행 구면 캐프, 왼쪽 paper · 오른쪽 direct(§7.4.1)."""
    L_ = np.load("ref_data/V2_direct_lines.npz")
    J = json.load(open("ref_data/V2_direct.json", encoding="utf-8"))
    skf = read_skf()
    BL, GR = "#1f3a93", "#2e7d4f"
    name = {"paper": r"식 (4.2-6) $w^{-1}$ 역산", "direct": "§7.4.1 압력 계수 직접"}
    plt.rcParams["font.family"] = "Malgun Gothic"
    plt.rcParams["axes.unicode_minus"] = False
    fig, axs = plt.subplots(3, 2, figsize=(12.0, 9.6), sharex=True)
    for c, m in enumerate(MODES):
        panels = [("SKF Fig 7(b)", skf["p_ph"], None, skf["hRx_b2"], None)]
        for shape, nm in (("ellipsoid", "타원 반구"), ("cap", "구면 캐프")):
            r = J[f"S0_{shape}"]
            panels.append((f"{nm} · {name[m]}", (L_["xb"], L_[f"{shape}_{m}_p_hertz"]), (L_["xb"], L_[f"{shape}_{m}_p_mean"]),
                           (L_["xb"], np.maximum(L_[f"{shape}_h"], 0.0)),
                           f"최대 p/p_h {r[f'{m}_p_max_hertz']:.2f} · 짧은 파장 RMS {r[f'{m}_hf_rms_hertz']:.3f}"))
        for ax, (title, p1, p2, h, note) in zip(axs[:, c], panels):
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
            if note:
                ax.text(0.98, 0.95, note, transform=ax.transAxes, ha="right", va="top", fontsize=8)
            ax.grid(alpha=0.3, ls=":")
            h1, l1 = ax.get_legend_handles_labels()
            h2, l2 = ax2.get_legend_handles_labels()
            ax.legend(h1 + h2, l1 + l2, fontsize=8, loc="upper left")
        axs[-1, c].set_xlabel("x/b")
    fig.tight_layout()
    fig.savefig("figures/V2_fig7b_direct.png", dpi=150)


if __name__ == "__main__":
    if sys.argv[1:] != ["plot"]:
        compute()
    plot()
