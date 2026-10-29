"""§3.10 — 거칠기 · 간극 해석: 초기 간극 r 과 Δh 비교 (§3.9 S = 0 조건, Hooke & Li H/A 안 · 사용자 결정 2026-10-28).

조건은 §3.9 와 같다(사다리꼴 · s = 176.0 μm · S = 0 · h̄ = 0.302 μm) — s0_mode 만 현 기본값 hooke_li.
r = −(z − z̄) (z̄ = 패치 면적 평균), Δh = h − h̄ (DC 제외). 윗면 = z 0.30 μm 평탄부, 홈 = z 0 평탄부.
    python fig2_gap_r.py  ->  figures/V1_fig2_gap_r.png · ref_data/V1_gap_r.json
"""
import io
import json

import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

import conditions as C  # noqa: E402
import ehl_ripple as M2  # noqa: E402
from fig2_trapezoid import z_edge  # noqa: E402
from fig3_shape_test import trapezoid_patch  # noqa: E402
from run_cases import SKF_BLUE, SKF_CENTRE_UM, SKF_RED  # noqa: E402

CASE, H, S, SHIFT, P = C.LEEDS, 0.302e-6, 0.0, 176.0e-6, 125e-6


def main():
    r2, L = trapezoid_patch()
    zbar = 0.30e-6 + r2.min()                                  # r = −(z − z̄) → 윗면 r = −(0.30 − z̄)
    xs = np.linspace(-CASE.a_x, CASE.a_x, 1201)
    uc = ((xs - SHIFT) / np.sqrt(2.0) + P / 2) % P - P / 2     # y = 0 단면의 무늬 좌표(§3.9 와 같음)
    z = z_edge(np.abs(uc)) * 1e-6
    r = -(z - zbar)
    dh, dp, _, _ = M2.ripple_line(r2, L, CASE, S, H, xs, "complex", shifts=np.array([SHIFT]), s0_mode="hooke_li")
    dh, dp = dh[0], dp[0]
    top, groove = z >= 0.30e-6 - 1e-12, z <= 1e-12
    out = dict(zbar_um=zbar * 1e6, h_bar_um=H * 1e6,
               r_top_um=float(r[top].mean() * 1e6), r_groove_um=float(r[groove].mean() * 1e6),
               dh_top_um=float(dh[top].mean() * 1e6), dh_groove_um=float(dh[groove].mean() * 1e6),
               dh_min_um=float(dh.min() * 1e6), dh_max_um=float(dh.max() * 1e6),
               r_max_over_h=float(np.abs(r2).max() / H))
    out["ratio_top"] = out["dh_top_um"] / out["r_top_um"]
    out["ratio_groove"] = out["dh_groove_um"] / out["r_groove_um"]
    out["p2p_r_um"] = out["r_groove_um"] - out["r_top_um"]
    out["p2p_dh_um"] = out["dh_groove_um"] - out["dh_top_um"]
    out["eps"] = out["p2p_dh_um"] / out["p2p_r_um"]
    out["h_min_um"] = out["h_bar_um"] + out["dh_min_um"]
    nu = dh - r                                                # 탄성 변형 ν = Δh − r (§3.10.6)
    out.update(dp_top_GPa=float(dp[top].mean() / 1e9), dp_groove_GPa=float(dp[groove].mean() / 1e9),
               nu_top_um=float(nu[top].mean() * 1e6), nu_groove_um=float(nu[groove].mean() * 1e6),
               corr_dp_nu=float(np.corrcoef(dp, nu)[0, 1]), sign_agree=float(np.mean(np.sign(dp) == np.sign(nu))),
               Ek4_fund_GPa_per_um=float(CASE.E_eff * (2.0 * np.pi / (P * np.sqrt(2.0))) / 4.0 * 1e-6 / 1e9),
               p_h_GPa=CASE.p_h / 1e9)
    json.dump(out, io.open("ref_data/V1_gap_r.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)

    xu = xs * 1e6 + SKF_CENTRE_UM
    fig, (az, ax) = plt.subplots(2, 1, figsize=(6.8, 6.6), sharex=True, gridspec_kw=dict(height_ratios=[1, 2.4]))
    az.fill_between(xu, 0.0, z * 1e6, color="0.85", lw=0)             # 위: 초기 거칠기 z 와 평균 z̄ (식 (1))
    az.plot(xu, z * 1e6, color="k", lw=1.4, label="initial roughness z")
    az.axhline(zbar * 1e6, color="k", lw=1.0, ls=":", label=f"z̄ = {zbar * 1e6:.3f} μm")
    az.set(ylim=(-0.05, 0.5), yticks=np.arange(0, 0.41, 0.1), ylabel="z, [μm]")
    az.grid(True, ls=":", color="0.4", lw=0.6)
    az.legend(fontsize=8, loc="upper center", ncol=2, frameon=False)       # 곡선과 겹치지 않게 위쪽 여백에
    for e in (-CASE.a_x, CASE.a_x):
        az.axvline(SKF_CENTRE_UM + e * 1e6, color="0.6", lw=0.8, ls="--")
    ax2 = ax.twinx()
    ax.plot(xu, r * 1e6, color="k", lw=1.4, ls="--", label="initial gap r = −(z − z̄)")
    ax.plot(xu, dh * 1e6, color=SKF_BLUE, lw=1.6, label="Δh (Hooke & Li H/A)")
    ax2.plot(xu, dp / 1e9, color=SKF_RED, lw=1.6, label="Δp")
    for e in (-CASE.a_x, CASE.a_x):
        ax.axvline(SKF_CENTRE_UM + e * 1e6, color="0.6", lw=0.8, ls="--")
    ax.axhline(0.0, color="0.3", lw=0.6)
    ax.set(xlim=(50, 450), ylim=(-0.4, 0.4), yticks=np.arange(-0.4, 0.41, 0.1))
    ax2.set(ylim=(-5, 2), yticks=np.arange(-5, 3, 1))
    ax.grid(True, ls=":", color="0.4", lw=0.6)
    ax.set_xlabel("x, [μm]")
    ax.set_ylabel("r · Δh, [μm]", color=SKF_BLUE)
    ax2.set_ylabel("Δp, [GPa]", color=SKF_RED)
    h1, l1 = ax.get_legend_handles_labels()
    h2, l2 = ax2.get_legend_handles_labels()
    ax.legend(h1 + h2, l1 + l2, fontsize=8, loc="lower right")
    az.set_title(f"S = 0 · s = {SHIFT * 1e6:.1f} μm · h̄ = {H * 1e6:.3f} μm — gap ratio ε = {out['eps']:.2f}", fontsize=9)
    fig.tight_layout()
    fig.savefig("figures/V1_fig2_gap_r.png", dpi=150)
    return out


if __name__ == "__main__":
    for k, v in main().items():
        print(f"{k:14s} {v:+.4f}")
