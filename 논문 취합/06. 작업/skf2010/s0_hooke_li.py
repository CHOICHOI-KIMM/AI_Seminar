"""§4.7.4 — S = 0 에 Hooke & Li 2006 복소 H/A 를 적용해 Leeds Fig 3(a) 와 점별 비교 (작업내역서 §4.7.3 단계 ②–④).

입력·조건은 §4.6.3 과 같다: 사다리꼴(Leeds 'Simulated') 패치 · 스냅숏 s = 19.0 μm(복소 맞춤값, 고정) · y = 0.
SKF 안(식 24, 실수 h_t)과 Hooke & Li 안(복소 H/A)을 Leeds NN 과 나란히 그린다.
  ③ 지표: 접촉 내부 Leeds 유효점의 정규화 RMS(Leeds p-p 기준)·상관, 고조파(176.8/n μm) Δh/r·위상·Δp.
     Hooke & Li 로 s 를 다시 맞춘 값은 민감도로만 병기(비교 그림·지표는 s 고정).
  ④ 정합: (k_x, 0) 성분의 |H/A| 와 SKF h_t/r_a, 맞춤 범위(λ/b < 2) 밖 성분의 비중.
    python s0_hooke_li.py  ->  figures/V1b_fig3a_hooke_li.png · ref_data/V1b_hooke_li.json
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

CASE, H, S = F.CASE, F.H, 0.0
MODES = {"skf": ("this work · SKF eq (24), real h_t", "#d62728"),
         "hooke_li": ("this work · Hooke & Li 2006, complex H/A", "#1f77b4")}


def score(dh, dp, xl, hl, pl, mh, mp):
    """접촉 내부 Leeds 유효점에서 정규화 RMS·상관 (Δh 는 평균 상수차 제거)."""
    d = dh - dh.mean() + hl[mh].mean()
    nh = float(np.sqrt(np.mean((d - hl[mh]) ** 2)) / np.ptp(hl[mh]))
    np_ = float(np.sqrt(np.mean((dp - pl[mp]) ** 2)) / np.ptp(pl[mp]))
    return dict(nrms_h=nh, corr_h=float(np.corrcoef(d, hl[mh])[0, 1]),
                nrms_p=np_, corr_p=float(np.corrcoef(dp, pl[mp])[0, 1]))


def main():
    ph = json.load(io.open("ref_data/V1b_phase.json", encoding="utf-8"))
    s = ph["a"]["fit"]["complex"]["s_um"] * 1e-6
    xl, hl, pl = F.leeds("a")
    mi = np.abs(xl) <= CASE.a_x
    mh, mp = mi & np.isfinite(hl), mi & np.isfinite(pl)
    r, L = T.trapezoid_patch()
    xx = np.linspace(-CASE.a_x, CASE.a_x, 1201)
    Rv = F.harm(xx, F.rough(xx, s), s)                       # 위상 기준: 수직 모서리 r (§4.6.3 과 동일)
    Hl, Pl = F.harm(xl[mh], hl[mh], s), F.harm(xl[mp], pl[mp], s)
    out = dict(s_um=s * 1e6)

    fig, ax = plt.subplots(figsize=(7.0, 4.8))
    ax2 = ax.twinx()
    fh, fp = np.isfinite(hl), np.isfinite(pl)
    ax.plot(xl[fh] * 1e6, hl[fh] * 1e6, color="k", lw=2.2, label="Leeds NN (Fig 3a)")
    ax2.plot(xl[fp] * 1e6, pl[fp] / 1e9, color="k", lw=2.2)
    sols = {}
    for mode, (lab, col) in MODES.items():
        dh, dp, resid, sol = M2.ripple_line(r, L, CASE, S, H, xx, "complex", shifts=np.array([s]), s0_mode=mode)
        dh, dp = dh[0], dp[0]
        sols[mode] = sol
        ax.plot(xx * 1e6, dh * 1e6, color=col, lw=1.3, label=lab)
        ax2.plot(xx * 1e6, dp / 1e9, color=col, lw=1.3)
        dhl, dpl, _, _ = M2.ripple_line(r, L, CASE, S, H, xl[mh], "complex", shifts=np.array([s]), s0_mode=mode)
        _, dpp, _, _ = M2.ripple_line(r, L, CASE, S, H, xl[mp], "complex", shifts=np.array([s]), s0_mode=mode)
        Ho, Po = F.harm(xx, dh, s), F.harm(xx, dp, s)
        out[mode] = dict(score=score(dhl[0], dpp[0], xl, hl, pl, mh, mp), imag_resid=float(resid),
                         mean_dh_um=float(dh.mean() * 1e6),
                         harmonics=[dict(n=n + 1, r_amp_um=abs(Rv[n]) * 1e6,
                                         dh_ratio_leeds=abs(Hl[n]) / abs(Rv[n]), dh_ratio=abs(Ho[n]) / abs(Rv[n]),
                                         dh_ph_leeds=float(np.degrees(np.angle(Hl[n] / Rv[n]))),
                                         dh_ph=float(np.degrees(np.angle(Ho[n] / Rv[n]))),
                                         dp_amp_leeds=abs(Pl[n]) / 1e9, dp_amp=abs(Po[n]) / 1e9,
                                         dp_ph_leeds=float(np.degrees(np.angle(Pl[n] / Rv[n]))),
                                         dp_ph=float(np.degrees(np.angle(Po[n] / Rv[n]))))
                                    for n in range(F.N_H)])
    for e in (-CASE.a_x, CASE.a_x):
        ax.axvline(e * 1e6, color="0.6", lw=0.8, ls="--")
    ax.set(xlim=(-200, 200), ylim=(-0.3, 0.4), xlabel=f"x-position [μm] · S = 0 · s = {s*1e6:.1f} μm",
           ylabel="Δh [μm] (Leeds: h − h_s)")
    ax2.set(ylim=(-5, 2), ylabel="Δp [GPa] (Leeds: p − p_s)")
    ax.grid(True, ls=":", color="0.5", lw=0.6)
    ax.legend(fontsize=8, loc="lower left")
    fig.suptitle("Leeds Fig 3(a) NN (black) vs this work, trapezoid input — SKF eq (24) (red) vs Hooke & Li complex H/A (blue)",
                 fontsize=8)
    fig.tight_layout()
    fig.savefig("figures/V1b_fig3a_hooke_li.png", dpi=150)

    # s 재맞춤 (민감도) — Hooke & Li 안으로 Δh RMS 최소 s
    shifts = np.arange(0.0, 125e-6 * np.sqrt(2.0), 0.5e-6)
    dhs, _, _, _ = M2.ripple_line(r, L, CASE, S, H, xl[mh], "complex", shifts=shifts, s0_mode="hooke_li")
    e = np.sqrt(np.mean(((dhs - dhs.mean(1, keepdims=True)) - (hl[mh] - hl[mh].mean())) ** 2, axis=1))
    k = int(np.argmin(e))
    out["hooke_li"]["s_refit_um"] = float(shifts[k] * 1e6)
    out["hooke_li"]["s_refit_rms_um"] = float(e[k] * 1e6)

    # ④ 정합: (k_x, 0) 성분의 |H/A| vs SKF h_t/r_a · 맞춤 범위 밖 비중
    sol_h, sol_s = sols["hooke_li"], sols["skf"]
    n = r.shape[0]
    R = np.fft.fft2(r) / r.size
    rows = []
    for kx in (3, 6, 9, 15, 18):
        ha = sol_h["hooke_li"]["HA"][0, kx]
        rows.append(dict(kx=kx, lam_um=L / kx * 1e6, lam_b=L / kx / CASE.a_x, Lam=float(sol_h["hooke_li"]["Lam"][0, kx]),
                         HA_abs=float(abs(ha)), HA_deg=float(np.degrees(np.angle(ha))),
                         skf_ht_r=float(abs(sol_s["h_t"][0, kx] / R[0, kx]))))
    oof = sol_h["hooke_li"]["out_of_fit"]
    energy = np.abs(R) ** 2
    out["consistency"] = dict(rows=rows, out_of_fit_count=int(oof.sum()),
                              out_of_fit_energy_share=float(energy[oof].sum() / energy.sum()))
    json.dump(out, io.open("ref_data/V1b_hooke_li.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)

    for mode in MODES:
        sc = out[mode]["score"]
        print(f"[{mode}] nRMS_h {sc['nrms_h']:.3f} corr_h {sc['corr_h']:.3f} · nRMS_p {sc['nrms_p']:.3f} corr_p {sc['corr_p']:.3f}"
              f" · 허수잔차 {out[mode]['imag_resid']:.1e} · 평균 Δh {out[mode]['mean_dh_um']:+.3f} μm")
    print(f"[hooke_li] s 재맞춤 {out['hooke_li']['s_refit_um']:.1f} μm (RMS {out['hooke_li']['s_refit_rms_um']:.4f} μm) · 고정 s {s*1e6:.1f}")
    print(" n   λ[μm]  Δh/r  L · SKF · HL | Δh위상 L · SKF · HL | Δp[GPa] L · SKF · HL | Δp위상 L · SKF · HL")
    for a, b in zip(out["skf"]["harmonics"], out["hooke_li"]["harmonics"]):
        if a["r_amp_um"] < 0.02:
            continue
        print(f"{a['n']:2d} {176.8/a['n']:6.1f}  {a['dh_ratio_leeds']:.2f} · {a['dh_ratio']:.2f} · {b['dh_ratio']:.2f} | "
              f"{a['dh_ph_leeds']:+4.0f} · {a['dh_ph']:+4.0f} · {b['dh_ph']:+4.0f} | "
              f"{a['dp_amp_leeds']:.3f} · {a['dp_amp']:.3f} · {b['dp_amp']:.3f} | "
              f"{a['dp_ph_leeds']:+4.0f} · {a['dp_ph']:+4.0f} · {b['dp_ph']:+4.0f}")
    c = out["consistency"]
    print(" kx  λ[μm]  λ/b   Λ    |H/A| ∠[°]  SKF h_t/r")
    for q in c["rows"]:
        print(f"{q['kx']:3d} {q['lam_um']:6.1f} {q['lam_b']:5.2f} {q['Lam']:5.2f}  {q['HA_abs']:.3f} {q['HA_deg']:+5.1f}  {q['skf_ht_r']:.3f}")
    print(f"맞춤 범위 밖(λ/b ≥ 2) 성분 {c['out_of_fit_count']} 개 · r 에너지 비중 {c['out_of_fit_energy_share']:.2%}")


if __name__ == "__main__":
    main()
