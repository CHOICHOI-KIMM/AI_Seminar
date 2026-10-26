"""§4.7.7 — FFT 파장당 절점 수 검토: Leeds Fig 3(a), S = 0, Hooke & Li H/A 안, N 255 vs 511 (사용자 결정 2026-10-16).

조건은 §4.7.4 와 같다(사다리꼴 입력 · L = 530.3 μm · s = 19.0 μm 고정 · y = 0). N 만 255 → 511(홀수, Nyquist 성분 없음).
    python v1b_nodes.py  ->  figures/V1b_fig3a_nodes.png · ref_data/V1b_nodes.json
"""
import io
import json

import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

import ehl_ripple as M2  # noqa: E402
import fig3_compare as F  # noqa: E402
import fig3_shape_test as T  # noqa: E402
from s0_hooke_li import score  # noqa: E402

CASE, H, S = F.CASE, F.H, 0.0
NS = {255: ("this work · H/A, N 255 (Δx 2.08 μm)", "#1f77b4", "-"),
      511: ("this work · H/A, N 511 (Δx 1.04 μm)", "#ff7f0e", "--")}


def main():
    ph = json.load(io.open("ref_data/V1b_phase.json", encoding="utf-8"))
    s = ph["a"]["fit"]["complex"]["s_um"] * 1e-6
    xl, hl, pl = F.leeds("a")
    mi = np.abs(xl) <= CASE.a_x
    mh, mp = mi & np.isfinite(hl), mi & np.isfinite(pl)
    xx = np.linspace(-CASE.a_x, CASE.a_x, 1201)
    Rv = F.harm(xx, F.rough(xx, s), s)
    Hl, Pl = F.harm(xl[mh], hl[mh], s), F.harm(xl[mp], pl[mp], s)
    out = dict(s_um=s * 1e6)

    fig, ax = plt.subplots(figsize=(7.0, 4.8))
    ax2 = ax.twinx()
    fh, fp = np.isfinite(hl), np.isfinite(pl)
    ax.plot(xl[fh] * 1e6, hl[fh] * 1e6, color="k", lw=2.2, label="Leeds NN (Fig 3a)")
    ax2.plot(xl[fp] * 1e6, pl[fp] / 1e9, color="k", lw=2.2)
    for n, (lab, col, ls) in NS.items():
        r, L = T.trapezoid_patch(n=n)
        dx = L / n
        dh, dp, resid, _ = M2.ripple_line(r, L, CASE, S, H, xx, "complex", shifts=np.array([s]), s0_mode="hooke_li")
        dh, dp = dh[0], dp[0]
        ax.plot(xx * 1e6, dh * 1e6, color=col, ls=ls, lw=1.3, label=lab)
        ax2.plot(xx * 1e6, dp / 1e9, color=col, ls=ls, lw=1.3)
        dhl, _, _, _ = M2.ripple_line(r, L, CASE, S, H, xl[mh], "complex", shifts=np.array([s]), s0_mode="hooke_li")
        _, dpp, _, _ = M2.ripple_line(r, L, CASE, S, H, xl[mp], "complex", shifts=np.array([s]), s0_mode="hooke_li")
        Ho, Po = F.harm(xx, dh, s), F.harm(xx, dp, s)
        out[str(n)] = dict(dx_um=dx * 1e6, nyquist_um=2 * dx * 1e6, imag_resid=float(resid),
                           score=score(dhl[0], dpp[0], xl, hl, pl, mh, mp),
                           harmonics=[dict(n=k + 1, lam_um=176.8 / (k + 1), nodes=176.8e-6 / (k + 1) / dx,
                                           r_amp_um=abs(Rv[k]) * 1e6,
                                           dh_ratio_leeds=abs(Hl[k]) / abs(Rv[k]), dh_ratio=abs(Ho[k]) / abs(Rv[k]),
                                           dh_ph_leeds=float(np.degrees(np.angle(Hl[k] / Rv[k]))),
                                           dh_ph=float(np.degrees(np.angle(Ho[k] / Rv[k]))),
                                           dp_amp_leeds=abs(Pl[k]) / 1e9, dp_amp=abs(Po[k]) / 1e9,
                                           dp_ph_leeds=float(np.degrees(np.angle(Pl[k] / Rv[k]))),
                                           dp_ph=float(np.degrees(np.angle(Po[k] / Rv[k]))))
                                      for k in range(F.N_H)])
    for e in (-CASE.a_x, CASE.a_x):
        ax.axvline(e * 1e6, color="0.6", lw=0.8, ls="--")
    ax.set(xlim=(-200, 200), ylim=(-0.3, 0.4), xlabel=f"x-position [μm] · S = 0 · s = {s*1e6:.1f} μm",
           ylabel="Δh [μm] (Leeds: h − h_s)")
    ax2.set(ylim=(-5, 2), ylabel="Δp [GPa] (Leeds: p − p_s)")
    ax.grid(True, ls=":", color="0.5", lw=0.6)
    ax.legend(fontsize=8, loc="lower left")
    fig.suptitle("Leeds Fig 3(a) NN (black) vs Hooke & Li H/A, trapezoid input — N 255 (blue) vs N 511 (orange dashed)",
                 fontsize=8)
    fig.tight_layout()
    fig.savefig("figures/V1b_fig3a_nodes.png", dpi=150)
    json.dump(out, io.open("ref_data/V1b_nodes.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)

    for n in NS:
        o, sc = out[str(n)], out[str(n)]["score"]
        print(f"N {n}: Δx {o['dx_um']:.2f} μm · Nyquist {o['nyquist_um']:.2f} μm · nRMS_h {sc['nrms_h']:.3f} corr_h {sc['corr_h']:.3f}"
              f" · nRMS_p {sc['nrms_p']:.3f} corr_p {sc['corr_p']:.3f} · imag {o['imag_resid']:.1e}")
    print(" n  λ   절점 255·511 | Δh/r L·255·511 | Δh위상 L·255·511 | Δp L·255·511 | Δp위상 L·255·511")
    for a, b in zip(out["255"]["harmonics"], out["511"]["harmonics"]):
        if a["r_amp_um"] < 0.02:
            continue
        print(f"{a['n']:2d} {a['lam_um']:6.1f} {a['nodes']:5.1f}·{b['nodes']:5.1f} | {a['dh_ratio_leeds']:.2f}·{a['dh_ratio']:.2f}·{b['dh_ratio']:.2f} | "
              f"{a['dh_ph_leeds']:+4.0f}·{a['dh_ph']:+4.0f}·{b['dh_ph']:+4.0f} | "
              f"{a['dp_amp_leeds']:.3f}·{a['dp_amp']:.3f}·{b['dp_amp']:.3f} | {a['dp_ph_leeds']:+4.0f}·{a['dp_ph']:+4.0f}·{b['dp_ph']:+4.0f}")


if __name__ == "__main__":
    main()
