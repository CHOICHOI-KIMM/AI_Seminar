"""§4.7.5 3. — 사다리꼴 경사 폭 w 추정 (SKF Fig 2(a) 기준) 과 겹침 그림.

조건은 §4.7.5 와 같다(Hooke & Li 복소 H/A · s = 176.0 μm · 반높이 한 변 85.5 μm 고정). w 만 바꾼다.
  ② w 스캔: Δh RMS(평균 상수차 제거, 접촉 내부 SKF 유효점) 최소 w.
     깁스 떨림 지표: 본 계산 Δh 의 짧은 파장(< 10 μm) 성분 RMS — SKF 곡선에는 떨림이 없다(§3.5.2).
  그림: §4.7.5 그림(w = 8.66, 파랑)에 추정 w 곡선(초록)을 겹친다.
    python s0_fig2_ramp.py  ->  figures/V1_fig2a_hooke_li_ramp.png · ref_data/V1_fig2a_ramp.json
"""
import io
import json

import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

import ehl_ripple as M2  # noqa: E402
import fig3_compare as F  # noqa: E402
import run_cases as RC  # noqa: E402
from s0_hooke_li import score  # noqa: E402

CASE, H, S, S_UM = F.CASE, F.H, 0.0, 176.0
C0 = RC.SKF_CENTRE_UM
A_MID = 85.5e-6 / 2
W_LEEDS = 8.66e-6
W_SCAN = (0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 4.0, 5.0, 6.0, 7.0, 8.66)   # μm


def patch(w, n=255, periods=3):
    """fig3_shape_test.trapezoid_patch 와 같은 형상, 경사 폭 w 만 인자로."""
    Pp = 125e-6
    L = periods * np.sqrt(2.0) * Pp
    g = np.arange(n) * L / n
    X, Y = np.meshgrid(g, g)
    uc = ((X + Y) / np.sqrt(2.0) + Pp / 2) % Pp - Pp / 2
    vc = ((X - Y) / np.sqrt(2.0) + Pp / 2) % Pp - Pp / 2
    q = np.maximum(np.abs(uc), np.abs(vc))
    z = 0.30e-6 * np.clip((A_MID + w / 2 - q) / w, 0.0, 1.0)
    return -(z - z.mean()), L


def ripple_rms(x, dh):
    """Δh 의 짧은 파장(< 10 μm) 성분 RMS — 등간격 x 위 FFT 고역통과."""
    k = np.fft.rfftfreq(len(x), d=x[1] - x[0])
    D = np.fft.rfft(dh - dh.mean())
    D[k < 1.0 / 10e-6] = 0.0
    return float(np.sqrt(np.mean(np.fft.irfft(D, len(x)) ** 2)))


def run(w, s, xk, hk, pk, mh, mp, xx):
    r, L = patch(w)
    dh, dp, _, _ = M2.ripple_line(r, L, CASE, S, H, xx, "complex", shifts=np.array([s]), s0_mode="hooke_li")
    dhk, _, _, _ = M2.ripple_line(r, L, CASE, S, H, xk[mh], "complex", shifts=np.array([s]), s0_mode="hooke_li")
    _, dpk, _, _ = M2.ripple_line(r, L, CASE, S, H, xk[mp], "complex", shifts=np.array([s]), s0_mode="hooke_li")
    return dh[0], dp[0], score(dhk[0], dpk[0], xk, hk, pk, mh, mp)


def main():
    s = S_UM * 1e-6
    xk, hk, pk = RC.load_fig2("a")
    mi = np.abs(xk) <= CASE.a_x
    mh, mp = mi & np.isfinite(hk), mi & np.isfinite(pk)
    xx = np.linspace(-CASE.a_x, CASE.a_x, 1201)
    Rv = F.harm(xx, F.rough(xx, s), s)
    Hk, Pk = F.harm(xk[mh], hk[mh], s), F.harm(xk[mp], pk[mp], s)

    rows, curves = [], {}
    for w_um in W_SCAN:
        dh, dp, sc = run(w_um * 1e-6, s, xk, hk, pk, mh, mp, xx)
        curves[w_um] = (dh, dp)
        rows.append(dict(w_um=w_um, ripple_nm=ripple_rms(xx, dh) * 1e9, **sc))
        print(f"w {w_um:5.2f} μm · nRMS_h {sc['nrms_h']:.3f} corr_h {sc['corr_h']:.3f} · "
              f"nRMS_p {sc['nrms_p']:.3f} corr_p {sc['corr_p']:.3f} · 떨림 RMS {rows[-1]['ripple_nm']:.1f} nm", flush=True)
    best = min(rows, key=lambda q: q["nrms_h"])
    w_best = best["w_um"]
    dh, dp = curves[w_best]
    Ho, Po = F.harm(xx, dh, s), F.harm(xx, dp, s)
    harm = [dict(n=n + 1, r_amp_um=abs(Rv[n]) * 1e6,
                 dh_ratio_skf=abs(Hk[n]) / abs(Rv[n]), dh_ratio=abs(Ho[n]) / abs(Rv[n]),
                 dh_ph_skf=float(np.degrees(np.angle(Hk[n] / Rv[n]))), dh_ph=float(np.degrees(np.angle(Ho[n] / Rv[n]))),
                 dp_amp_skf=abs(Pk[n]) / 1e9, dp_amp=abs(Po[n]) / 1e9,
                 dp_ph_skf=float(np.degrees(np.angle(Pk[n] / Rv[n]))), dp_ph=float(np.degrees(np.angle(Po[n] / Rv[n]))))
            for n in range(F.N_H)]
    skf_ripple = None
    json.dump(dict(s_um=S_UM, scan=rows, w_best_um=w_best, harmonics_best=harm),
              io.open("ref_data/V1_fig2a_ramp.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(f"→ Δh RMS 최소 w = {w_best} μm")
    for q in harm:
        if q["r_amp_um"] < 0.02:
            continue
        print(f"  n{q['n']} Δh/r SKF {q['dh_ratio_skf']:.2f} · {q['dh_ratio']:.2f} | 위상 {q['dh_ph_skf']:+.0f} · {q['dh_ph']:+.0f} | "
              f"Δp {q['dp_amp_skf']:.3f} · {q['dp_amp']:.3f} | Δp위상 {q['dp_ph_skf']:+.0f} · {q['dp_ph']:+.0f}")

    fig, ax = plt.subplots(figsize=(7.0, 4.8))
    ax2 = ax.twinx()
    fh, fp = np.isfinite(hk), np.isfinite(pk)
    ax.plot(xk[fh] * 1e6 + C0, hk[fh] * 1e6, color="k", lw=2.2, label="SKF 2010 Fig 2(a)")
    ax2.plot(xk[fp] * 1e6 + C0, pk[fp] / 1e9, color="k", lw=2.2)
    for w_um, col in ((8.66, "#1f77b4"), (w_best, "#2ca02c")):
        dh_, dp_ = curves[w_um]
        ax.plot(xx * 1e6 + C0, dh_ * 1e6, color=col, lw=1.3, label=f"Hooke & Li · ramp w = {w_um:g} μm")
        ax2.plot(xx * 1e6 + C0, dp_ / 1e9, color=col, lw=1.3)
    for e in (-CASE.a_x, CASE.a_x):
        ax.axvline(e * 1e6 + C0, color="0.6", lw=0.8, ls="--")
    ax.set(xlim=(50, 450), ylim=(-0.4, 0.4), xlabel=f"x [μm] (SKF axis, contact centre 250) · S = 0 · s = {S_UM:.1f} μm",
           ylabel="Δh [μm]")
    ax2.set(ylim=(-5, 2), ylabel="Δp [GPa]")
    ax.grid(True, ls=":", color="0.5", lw=0.6)
    ax.legend(fontsize=8, loc="lower left")
    fig.suptitle("SKF 2010 Fig 2(a) (black) vs Hooke & Li complex H/A — Leeds ramp (blue) and fitted ramp (green)", fontsize=8)
    fig.tight_layout()
    fig.savefig("figures/V1_fig2a_hooke_li_ramp.png", dpi=150)


if __name__ == "__main__":
    main()
