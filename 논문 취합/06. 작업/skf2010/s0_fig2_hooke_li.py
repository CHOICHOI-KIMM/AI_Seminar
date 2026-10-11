"""§4.7.5 — S = 0 에 Hooke & Li 2006 복소 H/A 를 적용해 SKF 2010 Fig 2(a) 와 점별 비교.

조건(사용자 결정 2026-10-11): 사다리꼴(Leeds 'Simulated') 입력 · s = 176.0 μm 고정(§3.4) · y = 0 ·
접촉중심 = SKF 도메인 중앙 250 μm(§3.1 가정). 비교 대상은 Hooke & Li 안 하나(§4.7.4 2. 방법론).
지표: 접촉 내부 SKF 유효점의 정규화 RMS(SKF p-p 기준)·상관, 고조파(176.8/n μm) Δh/r·위상·Δp.
    python s0_fig2_hooke_li.py  ->  figures/V1_fig2a_hooke_li.png · ref_data/V1_fig2a_hooke_li.json
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
import run_cases as RC  # noqa: E402
from s0_hooke_li import score  # noqa: E402

CASE, H, S, S_UM = F.CASE, F.H, 0.0, 176.0
C0 = RC.SKF_CENTRE_UM


def main():
    s = S_UM * 1e-6
    xk, hk, pk = RC.load_fig2("a")
    mi = np.abs(xk) <= CASE.a_x
    mh, mp = mi & np.isfinite(hk), mi & np.isfinite(pk)
    r, L = T.trapezoid_patch()
    xx = np.linspace(-CASE.a_x, CASE.a_x, 1201)
    Rv = F.harm(xx, F.rough(xx, s), s)                       # 위상 기준: 수직 모서리 r (§4.6.3 과 동일)
    Hk, Pk = F.harm(xk[mh], hk[mh], s), F.harm(xk[mp], pk[mp], s)

    dh, dp, resid, _ = M2.ripple_line(r, L, CASE, S, H, xx, "complex", shifts=np.array([s]), s0_mode="hooke_li")
    dh, dp = dh[0], dp[0]
    dhk, _, _, _ = M2.ripple_line(r, L, CASE, S, H, xk[mh], "complex", shifts=np.array([s]), s0_mode="hooke_li")
    _, dpk, _, _ = M2.ripple_line(r, L, CASE, S, H, xk[mp], "complex", shifts=np.array([s]), s0_mode="hooke_li")
    Ho, Po = F.harm(xx, dh, s), F.harm(xx, dp, s)
    out = dict(s_um=S_UM, score=score(dhk[0], dpk[0], xk, hk, pk, mh, mp), imag_resid=float(resid),
               mean_diff_um=float((dhk[0].mean() - hk[mh].mean()) * 1e6),
               harmonics=[dict(n=n + 1, r_amp_um=abs(Rv[n]) * 1e6,
                               dh_ratio_skf=abs(Hk[n]) / abs(Rv[n]), dh_ratio=abs(Ho[n]) / abs(Rv[n]),
                               dh_ph_skf=float(np.degrees(np.angle(Hk[n] / Rv[n]))),
                               dh_ph=float(np.degrees(np.angle(Ho[n] / Rv[n]))),
                               dp_amp_skf=abs(Pk[n]) / 1e9, dp_amp=abs(Po[n]) / 1e9,
                               dp_ph_skf=float(np.degrees(np.angle(Pk[n] / Rv[n]))),
                               dp_ph=float(np.degrees(np.angle(Po[n] / Rv[n]))))
                          for n in range(F.N_H)])
    json.dump(out, io.open("ref_data/V1_fig2a_hooke_li.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)

    fig, ax = plt.subplots(figsize=(7.0, 4.8))
    ax2 = ax.twinx()
    fh, fp = np.isfinite(hk), np.isfinite(pk)
    ax.plot(xk[fh] * 1e6 + C0, hk[fh] * 1e6, color="k", lw=2.2, label="SKF 2010 Fig 2(a)")
    ax2.plot(xk[fp] * 1e6 + C0, pk[fp] / 1e9, color="k", lw=2.2)
    ax.plot(xx * 1e6 + C0, dh * 1e6, color="#1f77b4", lw=1.3, label="this work · Hooke & Li 2006, complex H/A")
    ax2.plot(xx * 1e6 + C0, dp / 1e9, color="#1f77b4", lw=1.3)
    for e in (-CASE.a_x, CASE.a_x):
        ax.axvline(e * 1e6 + C0, color="0.6", lw=0.8, ls="--")
    ax.set(xlim=(50, 450), ylim=(-0.4, 0.4), xlabel=f"x [μm] (SKF axis, contact centre 250) · S = 0 · s = {S_UM:.1f} μm",
           ylabel="Δh [μm]")
    ax2.set(ylim=(-5, 2), ylabel="Δp [GPa]")
    ax.grid(True, ls=":", color="0.5", lw=0.6)
    ax.legend(fontsize=8, loc="lower left")
    fig.suptitle("SKF 2010 Fig 2(a) (black) vs this work, trapezoid input, Hooke & Li complex H/A (blue)", fontsize=8)
    fig.tight_layout()
    fig.savefig("figures/V1_fig2a_hooke_li.png", dpi=150)

    sc = out["score"]
    print(f"nRMS_h {sc['nrms_h']:.3f} corr_h {sc['corr_h']:.3f} · nRMS_p {sc['nrms_p']:.3f} corr_p {sc['corr_p']:.3f}"
          f" · 평균차 {out['mean_diff_um']:+.3f} μm · 허수잔차 {out['imag_resid']:.1e}")
    print(" n   λ[μm]  Δh/r SKF · HL | Δh위상 SKF · HL | Δp[GPa] SKF · HL | Δp위상 SKF · HL")
    for q in out["harmonics"]:
        if q["r_amp_um"] < 0.02:
            continue
        print(f"{q['n']:2d} {176.8/q['n']:6.1f}  {q['dh_ratio_skf']:.2f} · {q['dh_ratio']:.2f} | "
              f"{q['dh_ph_skf']:+4.0f} · {q['dh_ph']:+4.0f} | {q['dp_amp_skf']:.3f} · {q['dp_amp']:.3f} | "
              f"{q['dp_ph_skf']:+4.0f} · {q['dp_ph']:+4.0f}")


if __name__ == "__main__":
    main()
