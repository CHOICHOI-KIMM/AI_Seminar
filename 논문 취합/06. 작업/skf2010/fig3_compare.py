"""§4.4 ③·④ — Leeds Fig 3 NN 과 본 계산 비교 그림 + 고조파 위상 분해.

위상 s 는 Δh 맞춤 s(복소 뺄셈)를 두 U9 안에 공통으로 쓴다(사용자 결정 2026-10-08,
`ref_data/V1b_phase.json`). Δh 기준이 달라(Leeds h − h_s · 본 계산 평균 기준, §4.2)
그림은 그대로 겹치고 평균 차는 따로 적는다. 고조파 분해는 §3.5.1 과 같은 방법
(기본 주기 125√2 μm · 접촉 내부 · 최소제곱 · 위상은 원 거칠기 r 기준).
산출: figures/V1b_fig3_compare.png · ref_data/V1b_harmonics.json
"""
import io
import json

import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

import conditions as C  # noqa: E402
import ehl_ripple as M2  # noqa: E402
import run_cases as RC  # noqa: E402

CASE, H = C.LEEDS, 0.302e-6
P = 125e-6 * np.sqrt(2.0)
N_H = 6
PANELS = (("a", 0.0, "(a) S = 0 · Leeds Fig 3(a) t = 0"), ("b", 1.0, "(b) S = 1 · Leeds Fig 6(d) t = 3/500 s"),
          ("c", -1.0, "(c) S = -1 · Leeds Fig 7(a) t = 0"))
COL = {"complex": "#d62728", "magnitude": "#1f77b4"}


def leeds(key):
    d = np.genfromtxt(f"ref_data/V1b_leeds_fig3_{key}.csv", delimiter=",", skip_header=2, encoding="utf-8")
    return d[:, 0] * 1e-6, d[:, 1] * 1e-6, d[:, 2] * 1e9


def rough(x, s):
    uc = ((x - s) / np.sqrt(2.0) + 62.5e-6) % 125e-6 - 62.5e-6
    z = np.where(np.abs(uc) <= 45e-6, 0.30e-6, 0.0)
    return -(z - z.mean())


def harm(x, y, s):
    cols = [np.ones_like(x)]
    for n in range(1, N_H + 1):
        w = 2 * np.pi * n / P
        cols += [np.cos(w * (x - s)), np.sin(w * (x - s))]
    c, *_ = np.linalg.lstsq(np.array(cols).T, y, rcond=None)
    return np.array([c[2 * n - 1] + 1j * c[2 * n] for n in range(1, N_H + 1)])


def main():
    ph = json.load(io.open("ref_data/V1b_phase.json", encoding="utf-8"))
    r, L = RC.flat_top_patch()
    xx = np.linspace(-CASE.a_x, CASE.a_x, 1201)
    fig, axes = plt.subplots(3, 1, figsize=(7.0, 13.5))
    out = {}
    for ax, (key, S, cap) in zip(axes, PANELS):
        s = ph[key]["fit"]["complex"]["s_um"] * 1e-6
        xl, hl, pl = leeds(key)
        ax2 = ax.twinx()
        ax.plot(xl * 1e6, hl * 1e6, color="k", lw=2.2, label="Leeds NN (Fig 3)")
        ax2.plot(xl * 1e6, pl / 1e9, color="k", lw=2.2)
        res = {}
        mi = (np.abs(xl) <= CASE.a_x)
        mh, mp = mi & np.isfinite(hl), mi & np.isfinite(pl)
        Rl = harm(xx, rough(xx, s), s)
        Hl, Pl = harm(xl[mh], hl[mh], s), harm(xl[mp], pl[mp], s)
        modes = ("complex",) if S == 0 else ("complex", "magnitude")
        for mode in modes:
            dh, dp, _, _ = M2.ripple_line(r, L, CASE, S, H, xx, mode, shifts=np.array([s]))
            dh, dp = dh[0], dp[0]
            lab = "this work" + ("" if S == 0 else f" ({mode})")
            ax.plot(xx * 1e6, dh * 1e6, color=COL[mode], lw=1.3, label=lab)
            ax2.plot(xx * 1e6, dp / 1e9, color=COL[mode], lw=1.3)
            Ho, Po = harm(xx, dh, s), harm(xx, dp, s)
            dl = np.interp(xx, xl[mh], hl[mh])
            res[mode] = dict(
                mean_dh_leeds_um=float(np.mean(hl[mh]) * 1e6), mean_dh_ours_um=float(np.mean(dh) * 1e6),
                harm=[dict(n=n + 1, lam_um=P / (n + 1) * 1e6, r_amp_um=abs(Rl[n]) * 1e6,
                           dh_amp_leeds=abs(Hl[n]) * 1e6, dh_amp_ours=abs(Ho[n]) * 1e6,
                           dh_ph_leeds=float(np.degrees(np.angle(Hl[n] / Rl[n]))),
                           dh_ph_ours=float(np.degrees(np.angle(Ho[n] / Rl[n]))),
                           dp_amp_leeds=abs(Pl[n]) / 1e9, dp_amp_ours=abs(Po[n]) / 1e9,
                           dp_ph_leeds=float(np.degrees(np.angle(Pl[n] / Rl[n]))),
                           dp_ph_ours=float(np.degrees(np.angle(Po[n] / Rl[n]))))
                      for n in range(N_H)])
        out[key] = dict(S=S, s_um=s * 1e6, modes=res)
        for e in (-CASE.a_x, CASE.a_x):
            ax.axvline(e * 1e6, color="0.6", lw=0.8, ls="--")
        ax.set_xlim(-200, 200)
        ax.set_ylim(-0.3, 0.4)
        ax2.set_ylim(-5, 2)
        ax.set_xlabel("x-position  [μm]\n" + cap + f" · s = {s*1e6:.1f} μm")
        ax.set_ylabel("Δh  [μm]  (Leeds: h − h_s)")
        ax2.set_ylabel("Δp  [GPa]  (Leeds: p − p_s)")
        ax.grid(True, ls=":", color="0.5", lw=0.6)
        ax.legend(fontsize=8, loc="lower left")
    fig.suptitle("Leeds Fig 3 NN (black) vs this work — upper curves Δp, lower curves Δh; dashed = contact edges",
                 fontsize=9)
    fig.tight_layout()
    fig.savefig("figures/V1b_fig3_compare.png", dpi=150)
    json.dump(out, io.open("ref_data/V1b_harmonics.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    for key, v in out.items():
        for mode, rr in v["modes"].items():
            print(f"({key}) {mode} · 평균 Δh Leeds {rr['mean_dh_leeds_um']:+.4f} · 본 {rr['mean_dh_ours_um']:+.4f} μm")
            for h in rr["harm"]:
                if h["r_amp_um"] < 0.02:
                    continue
                print(f"   n{h['n']} λ{h['lam_um']:6.1f} r {h['r_amp_um']:.3f} | Δh/r L {h['dh_amp_leeds']/h['r_amp_um']:.2f} "
                      f"본 {h['dh_amp_ours']/h['r_amp_um']:.2f} · 위상 L {h['dh_ph_leeds']:+5.0f} 본 {h['dh_ph_ours']:+5.0f} | "
                      f"Δp L {h['dp_amp_leeds']:.3f} 본 {h['dp_amp_ours']:.3f} GPa · 위상 L {h['dp_ph_leeds']:+5.0f} "
                      f"본 {h['dp_ph_ours']:+5.0f}")


if __name__ == "__main__":
    main()
