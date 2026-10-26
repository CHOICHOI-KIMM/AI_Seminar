"""§5 V1c — Leeds Fig 3 S ≠ 0 (S = +1 · −1) 를 보완함수 근사 'hooke_li_all' 로 다시 비교 (사용자 결정 2026-10-16).

근사: h_t = (H/A)_Hooke&Li r_a, h_c = h_t − h_a(S), 특수적분 · ψ 는 기존 S ≠ 0 (식 18 · 21, ψ²/η_x).
조건: Leeds 실제 조건(conditions.LEEDS, h = 0.302 μm), 입력 Leeds 'Simulated' 사다리꼴(§4.6.3), 위상 s 는 §4.6.3 채택값.
산출: figures/V1c_fig3.png (그림만, 지표 계산 없음 — 사용자 결정).
"""
import io
import json

import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

import ehl_ripple as M2  # noqa: E402
import fig3_compare as F  # noqa: E402
from fig3_shape_test import trapezoid_patch  # noqa: E402


def main(path="figures/V1c_fig3.png"):
    ph = json.load(io.open("ref_data/V1b_phase.json", encoding="utf-8"))
    r, L = trapezoid_patch()
    xx = np.linspace(-F.CASE.a_x, F.CASE.a_x, 1201)
    panels = [p for p in F.PANELS if p[1] != 0]
    fig, axes = plt.subplots(len(panels), 1, figsize=(7.0, 9.0))
    for ax, (key, S, cap) in zip(axes, panels):
        s = ph[key]["fit"]["complex"]["s_um"] * 1e-6
        xl, hl, pl = F.leeds(key)
        ax2 = ax.twinx()
        fh, fp = np.isfinite(hl), np.isfinite(pl)
        ax.plot(xl[fh] * 1e6, hl[fh] * 1e6, color="k", lw=2.2, label="Leeds NN (Fig 3)")
        ax2.plot(xl[fp] * 1e6, pl[fp] / 1e9, color="k", lw=2.2)
        dh, dp, resid, _ = M2.ripple_line(r, L, F.CASE, S, F.H, xx, "complex", shifts=np.array([s]),
                                          s0_mode="hooke_li_all")
        ax.plot(xx * 1e6, dh[0] * 1e6, color="#1f77b4", lw=1.3, label="this work · H/A approx. (S ≠ 0)")
        ax2.plot(xx * 1e6, dp[0] / 1e9, color="#1f77b4", lw=1.3)
        print(key, f"S={S:+g} s={s*1e6:.1f} um  imag resid {resid:.1e}")
        for e in (-F.CASE.a_x, F.CASE.a_x):
            ax.axvline(e * 1e6, color="0.6", lw=0.8, ls="--")
        ax.set_xlim(-200, 200)
        ax.set_ylim(-0.3, 0.4)
        ax2.set_ylim(-5, 2)
        ax.set_xlabel("x-position  [μm]\n" + cap + f" · s = {s*1e6:.1f} μm")
        ax.set_ylabel("Δh  [μm]  (Leeds: h − h_s)")
        ax2.set_ylabel("Δp  [GPa]  (Leeds: p − p_s)")
        ax.grid(True, ls=":", color="0.5", lw=0.6)
        ax.legend(fontsize=8, loc="lower left")
    fig.suptitle("Leeds Fig 3 NN (black) vs this work, H/A approx. for S ≠ 0\n"
                 "trapezoid edges · upper Δp, lower Δh · dashed = contact edges", fontsize=9)
    fig.tight_layout()
    fig.savefig(path, dpi=150)
    print(path)


if __name__ == "__main__":
    main()
