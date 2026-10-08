"""§4.4 ⑤ — 입력 형상 시험: Leeds Fig 1 'Simulated' 사다리꼴 모서리 (시험 계산만 · 채택은 별도 동의).

Leeds Fig 1 의 Simulated 단면(대각선 방향)에서 N 점선과 겹치지 않는 첫 홈의 두 경사를 읽었다
(`digitize` 는 이 파일 주석의 수치 — 점선 조각 358 px 을 경사별로 묶어 x(y) 직선 맞춤):
  내려가는 경사 x(0.15 μm) = 56.5 · 0 → 0.3 μm 폭 12.9 μm(대각선)
  올라가는 경사 x(0.15 μm) = 112.3 · 폭 11.6 μm(대각선)
→ 반높이 홈 폭 55.8 μm(대각선) = 39.5 μm(변 수직), 경사 폭 평균 12.25 μm(대각선) = 8.66 μm(수직).
  결함 반높이 한 변 = 125 − 39.5 = 85.5 μm.
2D 형상: q = max(|u|,|v|) (정사각 축 좌표, 결함 중심 기준)
  z = 0.30 · clip((a_mid + w/2 − q)/w, 0, 1),  a_mid = 85.5/2 μm,  w = 8.66 μm
위상 s 는 ③ 과 같은 값(수직 모서리로 맞춘 복소 s)을 그대로 써서 형상 효과만 본다.
산출: figures/V1b_fig3_shape.png · ref_data/V1b_shape_harmonics.json
"""
import io
import json

import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

import ehl_ripple as M2  # noqa: E402
import run_cases as RC  # noqa: E402
import fig3_compare as F  # noqa: E402

A_MID = 85.5e-6 / 2
W_RAMP = 8.66e-6


def trapezoid_patch(n=255, periods=3):
    Pp = 125e-6
    L = periods * np.sqrt(2.0) * Pp
    g = np.arange(n) * L / n
    X, Y = np.meshgrid(g, g)
    uc = ((X + Y) / np.sqrt(2.0) + Pp / 2) % Pp - Pp / 2
    vc = ((X - Y) / np.sqrt(2.0) + Pp / 2) % Pp - Pp / 2
    q = np.maximum(np.abs(uc), np.abs(vc))
    z = 0.30e-6 * np.clip((A_MID + W_RAMP / 2 - q) / W_RAMP, 0.0, 1.0)
    return -(z - z.mean()), L


def rough_trap(x, s):
    uc = ((x - s) / np.sqrt(2.0) + 62.5e-6) % 125e-6 - 62.5e-6
    z = 0.30e-6 * np.clip((A_MID + W_RAMP / 2 - np.abs(uc)) / W_RAMP, 0.0, 1.0)
    return -(z - z.mean())


def main():
    ph = json.load(io.open("ref_data/V1b_phase.json", encoding="utf-8"))
    shapes = {"vertical": RC.flat_top_patch(), "trapezoid": trapezoid_patch()}
    xx = np.linspace(-F.CASE.a_x, F.CASE.a_x, 1201)
    fig, axes = plt.subplots(3, 2, figsize=(13.0, 13.5))
    out = {}
    for row, (key, S, cap) in enumerate(F.PANELS):
        s = ph[key]["fit"]["complex"]["s_um"] * 1e-6
        xl, hl, pl = F.leeds(key)
        mi = np.abs(xl) <= F.CASE.a_x
        mh, mp = mi & np.isfinite(hl), mi & np.isfinite(pl)
        Rv = F.harm(xx, F.rough(xx, s), s)          # 위상 기준은 두 형상 공통(수직 모서리 r)
        Hl, Pl = F.harm(xl[mh], hl[mh], s), F.harm(xl[mp], pl[mp], s)
        modes = ("complex",) if S == 0 else ("complex", "magnitude")
        out[key] = {}
        for col, mode in enumerate(("complex", "magnitude")):
            ax = axes[row, col]
            if mode not in modes:
                ax.axis("off")
                continue
            ax2 = ax.twinx()
            ax.plot(xl * 1e6, hl * 1e6, color="k", lw=2.2, label="Leeds NN")
            ax2.plot(xl * 1e6, pl / 1e9, color="k", lw=2.2)
            res = {}
            for shp, (r, L) in shapes.items():
                dh, dp, _, _ = M2.ripple_line(r, L, F.CASE, S, F.H, xx, mode, shifts=np.array([s]))
                dh, dp = dh[0], dp[0]
                c = "#d62728" if shp == "vertical" else "#2ca02c"
                ax.plot(xx * 1e6, dh * 1e6, color=c, lw=1.2, label=f"this work · {shp}")
                ax2.plot(xx * 1e6, dp / 1e9, color=c, lw=1.2)
                Ho, Po = F.harm(xx, dh, s), F.harm(xx, dp, s)
                res[shp] = [dict(n=n + 1, r_amp_um=abs(Rv[n]) * 1e6,
                                 dh_ratio_leeds=abs(Hl[n]) / abs(Rv[n]), dh_ratio=abs(Ho[n]) / abs(Rv[n]),
                                 dh_ph_leeds=float(np.degrees(np.angle(Hl[n] / Rv[n]))),
                                 dh_ph=float(np.degrees(np.angle(Ho[n] / Rv[n]))),
                                 dp_amp_leeds=abs(Pl[n]) / 1e9, dp_amp=abs(Po[n]) / 1e9,
                                 dp_ph_leeds=float(np.degrees(np.angle(Pl[n] / Rv[n]))),
                                 dp_ph=float(np.degrees(np.angle(Po[n] / Rv[n]))))
                             for n in range(F.N_H)]
                res[shp + "_mean_dh_um"] = float(np.mean(dh) * 1e6)
            out[key][mode] = res
            for e in (-F.CASE.a_x, F.CASE.a_x):
                ax.axvline(e * 1e6, color="0.6", lw=0.8, ls="--")
            ax.set_xlim(-200, 200)
            ax.set_ylim(-0.3, 0.4)
            ax2.set_ylim(-5, 2)
            ax.set_xlabel("x-position [μm]\n" + cap + f" · U9 {mode}")
            ax.set_ylabel("Δh [μm]")
            ax2.set_ylabel("Δp [GPa]")
            ax.grid(True, ls=":", color="0.5", lw=0.6)
            ax.legend(fontsize=7, loc="lower left")
    fig.suptitle("Input-shape test: vertical edges (red) vs Leeds 'Simulated' trapezoid edges (green) · black = Leeds NN",
                 fontsize=9)
    fig.tight_layout()
    fig.savefig("figures/V1b_fig3_shape.png", dpi=130)
    json.dump(out, io.open("ref_data/V1b_shape_harmonics.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    for key, v in out.items():
        for mode, res in v.items():
            print(f"({key}) {mode}")
            for a, b in zip(res["vertical"], res["trapezoid"]):
                if a["r_amp_um"] < 0.02:
                    continue
                print(f"   n{a['n']} Δh/r L {a['dh_ratio_leeds']:.2f} 수직 {a['dh_ratio']:.2f} 사다리꼴 {b['dh_ratio']:.2f} | "
                      f"위상 L {a['dh_ph_leeds']:+.0f} 수직 {a['dh_ph']:+.0f} 사다리꼴 {b['dh_ph']:+.0f} | "
                      f"Δp L {a['dp_amp_leeds']:.3f} 수직 {a['dp_amp']:.3f} 사다리꼴 {b['dp_amp']:.3f} | "
                      f"Δp 위상 L {a['dp_ph_leeds']:+.0f} 수직 {a['dp_ph']:+.0f} 사다리꼴 {b['dp_ph']:+.0f}")


def plot_trapezoid(path="figures/V1b_fig3_shape3.png"):
    """③ 비교 그림과 같은 크기·형식으로 Leeds NN 과 사다리꼴 형상 결과만 그린다."""
    ph = json.load(io.open("ref_data/V1b_phase.json", encoding="utf-8"))
    r, L = trapezoid_patch()
    xx = np.linspace(-F.CASE.a_x, F.CASE.a_x, 1201)
    fig, axes = plt.subplots(3, 1, figsize=(7.0, 13.5))
    for ax, (key, S, cap) in zip(axes, F.PANELS):
        s = ph[key]["fit"]["complex"]["s_um"] * 1e-6
        xl, hl, pl = F.leeds(key)
        ax2 = ax.twinx()
        # 그림에서만 NaN(수직 급변) 앞뒤 유효점을 이어 끊김 없이 그린다 — 데이터·고조파는 NaN 그대로
        fh, fp = np.isfinite(hl), np.isfinite(pl)
        ax.plot(xl[fh] * 1e6, hl[fh] * 1e6, color="k", lw=2.2, label="Leeds NN (Fig 3)")
        ax2.plot(xl[fp] * 1e6, pl[fp] / 1e9, color="k", lw=2.2)
        for mode in (("complex",) if S == 0 else ("complex", "magnitude")):
            dh, dp, _, _ = M2.ripple_line(r, L, F.CASE, S, F.H, xx, mode, shifts=np.array([s]))
            lab = "this work · trapezoid" + ("" if S == 0 else f" ({mode})")
            ax.plot(xx * 1e6, dh[0] * 1e6, color=F.COL[mode], lw=1.3, label=lab)
            ax2.plot(xx * 1e6, dp[0] / 1e9, color=F.COL[mode], lw=1.3)
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
    fig.suptitle("Leeds Fig 3 NN (black) vs this work with Leeds 'Simulated' trapezoid edges — "
                 "upper Δp, lower Δh; dashed = contact edges", fontsize=8)
    fig.tight_layout()
    fig.savefig(path, dpi=150)
    print(path)


if __name__ == "__main__":
    import sys
    plot_trapezoid() if sys.argv[1:] == ["plot"] else main()
