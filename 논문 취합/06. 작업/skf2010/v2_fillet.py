"""§6.6.3 6. — 범프 테두리 처리 시험 (사용자 결정 2026-10-11): 구면 캐프 · Hooke & Li · 패치 2a×2a · N 128.

(a) C¹ 필렛: z = z_cap(ρ)·g(ρ), g = raised-cosine 창(ρ ∈ [R−w, R] 에서 1 → 0, 끝 기울기 0). w = 2 · 4.5 · 9 μm.
    4.5 μm = Purdue 2008 격자 간격(0.0236 b) — Purdue 해가 표현할 수 있는 가장 날카로운 테두리 척도.
(c) 입력 스펙트럼 저역 필터: R(κ)·exp(−κ²σ²/2), σ = 2.25 · 4.5 μm (가우시안 평활, 공간 폭 ≈ 2σ).
지표(§6.6.3 5. 와 동일): φ_bl · 최대 p/p_h · 간극 봉우리 입구/출구 · 압력 짧은 파장(λ < 10 μm) RMS(|x/b| ≤ 0.5).
입력 기술자: 중심 높이 · 체적(기준 캐프 대비).
    python v2_fillet.py  ->  ref_data/V2_fillet.json · figures/V2_fillet.png
"""
import io
import json

import matplotlib
import numpy as np

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

import load_sharing as LS  # noqa: E402
from v2_fig7 import CASE, N, H_BUMP, R_BUMP, bump  # noqa: E402
from v2_n256 import hf_rms  # noqa: E402

FILLETS = (2e-6, 4.5e-6, 9e-6)
SIGMAS = (2.25e-6, 4.5e-6)


def fillet(z, rho, w):
    g = np.ones_like(rho)
    m = (rho > R_BUMP - w) & (rho < R_BUMP)
    g[m] = 0.5 * (1.0 + np.cos(np.pi * (rho[m] - (R_BUMP - w)) / w))
    g[rho >= R_BUMP] = 0.0
    return z * g


def lowpass(z, L, sigma):
    n = z.shape[0]
    w = 2.0 * np.pi * np.fft.fftfreq(n, d=L / n)
    wy, wx = np.meshgrid(w, w, indexing="ij")
    return np.fft.ifft2(np.fft.fft2(z) * np.exp(-0.5 * (wx**2 + wy**2) * sigma**2)).real


def main():
    a = CASE.a_x
    L = 2.0 * a
    hs = a**2 / CASE.Rx
    x = (np.arange(N) - N // 2) * L / N
    X, Y = np.meshgrid(x, x)
    rho = np.hypot(X, Y)
    z0 = bump("cap", rho)
    variants = {"cap": z0}
    for w in FILLETS:
        variants[f"fillet_{w*1e6:g}um"] = fillet(z0, rho, w)
    for s in SIGMAS:
        variants[f"lowpass_s{s*1e6:g}um"] = lowpass(z0, L, s)
    xb, j = x / a, N // 2
    out, lines = {}, {"xb": xb}
    for name, z in variants.items():
        o = LS.solve(-z, L, CASE)
        p = o["p_hertz"][j] / CASE.p_h
        hh = o["h_tran"][j] / hs
        lines[f"{name}_p"], lines[f"{name}_h"], lines[f"{name}_z"] = p, hh, z[j] * 1e6
        out[name] = dict(z_peak_um=float(z.max() * 1e6), z_vol_ratio=float(z.sum() / z0.sum()),
                         phi_bl=o["phi_bl"], converged=o["converged"], p_max=float(p.max()),
                         x_pmax=float(xb[np.argmax(p)]), p_hf_rms=hf_rms(xb, p),
                         h_max_lead=float(hh[xb < 0].max()), h_max_trail=float(hh[xb > 0].max()))
        print(name, {k: (round(v, 4) if isinstance(v, float) else v) for k, v in out[name].items()}, flush=True)
    json.dump(out, io.open("ref_data/V2_fillet.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    np.savez("ref_data/V2_fillet_lines.npz", **lines)

    plt.rcParams["font.family"] = "Malgun Gothic"
    plt.rcParams["axes.unicode_minus"] = False
    fig, axs = plt.subplots(2, 2, figsize=(12.0, 7.5))
    groups = (("(a) C¹ 필렛", ["cap"] + [f"fillet_{w*1e6:g}um" for w in FILLETS]),
              ("(c) 저역 필터", ["cap"] + [f"lowpass_s{s*1e6:g}um" for s in SIGMAS]))
    for c, (title, names) in enumerate(groups):
        for name in names:
            axs[0, c].plot(x * 1e6, lines[f"{name}_z"], lw=1.2, label=name)
            axs[1, c].plot(xb, lines[f"{name}_p"], lw=1.1, label=name)
        axs[0, c].set(xlim=(-40, 40), xlabel="x [μm]", ylabel="범프 높이 z [μm]", title=f"{title} — 입력 단면 y = 0")
        axs[1, c].set(xlim=(-0.5, 0.5), ylim=(0, 6), xlabel="x/b", ylabel="p/p_h (Hertz 안)", title=f"{title} — 압력")
        for ax in axs[:, c]:
            ax.grid(alpha=0.3, ls=":")
            ax.legend(fontsize=8)
    fig.tight_layout()
    fig.savefig("figures/V2_fillet.png", dpi=150)


if __name__ == "__main__":
    main()
