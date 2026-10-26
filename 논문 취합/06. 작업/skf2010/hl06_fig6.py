"""§8.6 Hooke & Li 2006 Part 1 Fig 6 재현 — 원문 맞춤 곡선(디지타이징) vs 본 모델(H/A 안) 점.

조건(사용자 결정 2026-10-16): §8.4 와 같음 — Ry/Rx 25 · E′ 226 GPa · HD h_cen · H/A 안, Table 1 9 케이스 × θ 0·45·90° × 41 점.
Fig 6 축: Λ = (λ/b) P^1.5 S^-2,  |a/A| = |H(0)/A|,  |P|λ/E′A = |P(0) b/E′A| · (λ/b).
교차 확인용 해석 곡선: 원문 식 (19)–(21)(ehl_ripple.hooke_li_ratio, 본 구현 읽기) + 장파장 식 (23)(24)(25).

    python hl06_fig6.py  ->  ref_data/HL06_fig6_model.json · figures/HL06_fig6_model.png · ref_data/HL06_fig6_compare.json
"""
from __future__ import annotations

import csv
import json
import os

import matplotlib
import matplotlib.ticker
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
import numpy as np  # noqa: E402

from ehl_ripple import hooke_li_ratio  # noqa: E402
from hl06_sine import LAM_B, TABLE1, centre, make_case  # noqa: E402

plt.rcParams.update({"font.family": ["Malgun Gothic", "DejaVu Sans"], "axes.unicode_minus": False, "font.size": 12})
SUP = str.maketrans("-0123456789", "⁻⁰¹²³⁴⁵⁶⁷⁸⁹")
DECADE = matplotlib.ticker.FuncFormatter(lambda v, _: "10" + str(int(round(np.log10(v)))).translate(SUP))
THETAS = (0, 45, 90)
DIG = "ref_data/HL06_fig6_fit_digitized.csv"


def c_of(name):
    P, S, _ = TABLE1[name]
    return P**1.5 / S**2


def fit_curve(case, lam_b, theta):
    """원문 맞춤식: |H/A| (식 19·21) 와 |Pλ/E′A| (식 19 + 23·24·25)."""
    a = case.a_x
    k = 2.0 * np.pi / (lam_b * a)
    t = np.radians(theta)
    wx, wy = k * np.cos(t), k * np.sin(t)
    HA = hooke_li_ratio(wx, wy, case)["HA"]
    short = (HA - 1.0) * np.pi / 2.0                         # 식 (21) 역산 = 식 (19)
    wb, xb = np.abs(wx) * a, np.abs(wy) * a
    Px = wb**2 / 8.0
    Py = 0.06 * (1.0 - np.exp(-90.0 * xb)) + 0.21 * xb + 0.005 * xb**2
    F = lam_b * (Px**1.25 + Py**1.25) ** 0.8                 # 식 (23)
    dF = np.gradient(F, lam_b)
    long_ = np.where((F > np.pi / 2) & (dF < 0), -np.pi / 2, -F)   # 식 (24)
    total = short + long_ + np.pi / 2                        # 식 (25)
    return np.abs(HA), np.abs(total)


def main():
    res = {"lam_b": LAM_B.tolist(), "cases": {}}
    lam_f = 10.0 ** np.linspace(-5, 3, 801)                   # 해석 곡선용 촘촘한 격자
    fits = {}
    for name in TABLE1:
        c = make_case(name)
        d = {"c": c_of(name)}
        for th in THETAS:
            H, Pb = centre(c, LAM_B, th, "hooke_li")
            d[f"t{th}"] = {"amp": np.abs(H).tolist(), "pres": (np.abs(Pb) * LAM_B).tolist()}
            fits[(name, th)] = fit_curve(c, lam_f, th)
        res["cases"][name] = d
    os.makedirs("ref_data", exist_ok=True)
    with open("ref_data/HL06_fig6_model.json", "w", encoding="utf-8") as f:
        json.dump(res, f, ensure_ascii=False, indent=1)

    dig = []
    if os.path.exists(DIG):
        with open(DIG, encoding="utf-8") as f:
            dig = list(csv.DictReader(r for r in f if not r.startswith("#")))

    # 본 모델 점 vs 해석 맞춤 곡선(같은 케이스) 최대 차 — 디지타이징 곡선과의 차는 아래 dig 비교
    stats = {"model_vs_fit_eq": {}, "fit_eq_vs_digitized": {}}
    for name in TABLE1:
        for th in THETAS:
            fa, fp = fits[(name, th)]
            m = res["cases"][name][f"t{th}"]
            ia = np.interp(np.log10(LAM_B), np.log10(lam_f), fa)
            ip = np.interp(np.log10(LAM_B), np.log10(lam_f), fp)
            inside = (LAM_B * c_of(name) >= 1e-2) & (LAM_B * c_of(name) <= 1e2)   # Fig 6 축 범위만
            da = np.where(inside, np.abs(np.array(m["amp"]) - ia), 0.0)
            dp = np.where(inside, np.abs(np.array(m["pres"]) - ip), 0.0)
            short = inside & (LAM_B < 2.0)                                       # 맞춤식 범위 λ/b < 2
            stats.setdefault("model_vs_fit_eq_short", {})[f"{name}_t{th}"] = dict(
                amp_max=float(np.max(np.where(short, da, 0))), pres_max=float(np.max(np.where(short, dp, 0))))
            ka, kp = int(np.argmax(da)), int(np.argmax(dp))
            stats["model_vs_fit_eq"][f"{name}_t{th}"] = dict(
                amp_max=float(da[ka]), amp_Lambda=float(LAM_B[ka] * c_of(name)),
                pres_max=float(dp[kp]), pres_Lambda=float(LAM_B[kp] * c_of(name)))

    fig, ax = plt.subplots(3, 1, figsize=(10, 17))          # §4.6.3 처럼 세로 3행
    cmap = plt.get_cmap("viridis")
    cs = sorted({round(c_of(n), 2) for n in TABLE1})
    col = {cv: cmap(i / (len(cs) - 1)) for i, cv in enumerate(cs)}
    for j, th in enumerate(THETAS):
        a = ax[j]
        ar = a.twinx()
        rows = [r for r in dig if int(float(r["theta"])) == th]
        if rows:
            L = np.array([float(r["Lambda"]) for r in rows])
            cur = [r.get("curve", "unknown") for r in rows]
            va = np.array([float(r["value_left_axis"]) for r in rows])
            vp = np.array([float(r["value_right_axis"]) for r in rows])
            isa = np.array([cc == "amp" for cc in cur])
            isp = np.array([cc == "pres" for cc in cur])
            unk = ~(isa | isp)
            a.semilogx(L[isa], va[isa], ".", color="0.6", ms=2)
            ar.semilogx(L[isp], vp[isp], ".", color="0.6", ms=2)
            ar.semilogx(L[unk], vp[unk], ".", color="0.8", ms=2)
        for name in TABLE1:
            cv = round(c_of(name), 2)
            m = res["cases"][name][f"t{th}"]
            Lm = LAM_B * c_of(name)
            a.semilogx(Lm, m["amp"], "o", ms=5, color=col[cv], mfc=col[cv], mec="none")
            ar.semilogx(Lm, m["pres"], "s", ms=5, color=col[cv], mfc="none", mew=0.8)
        a.set_xlim(1e-2, 1e2)
        a.set_ylim(0, 1.2)
        ar.set_ylim(0, 3)
        a.xaxis.set_major_formatter(DECADE)
        a.set_xlabel("Wavelength Λ = (λ/b) P¹·⁵ S⁻²")
        a.set_title(f"({'abc'[j]}) θ = {th}°", y=-0.24)                      # 패널 아래 캡션
        a.grid(True, which="major", ls=":", color="0.75")
        a.set_ylabel("Amplitude ratio |a/A|  (●)")
        ar.set_ylabel("Pressure ratio |P|λ/E′A  (□)")
    from matplotlib.lines import Line2D
    hs = [Line2D([], [], color="0.6", marker=".", ls="none", label="원문 맞춤 곡선(디지타이징)")]
    hs += [Line2D([], [], color="k", marker="o", ls="none", ms=4, label="본 모델 |a/A| (●)"),
           Line2D([], [], color="k", marker="s", mfc="none", ls="none", ms=4, label="본 모델 |P|λ/E′A (□)")]
    hs += [Line2D([], [], color=col[cv], marker="o", ls="none", ms=5, label=f"P¹·⁵S⁻² = {cv}") for cv in cs]
    fig.legend(handles=hs, loc="lower center", ncol=4, fontsize=11, bbox_to_anchor=(0.5, -0.06))
    fig.suptitle("Hooke & Li 2006 Fig 6 형식 — 원문 맞춤 곡선(회색) vs 본 모델 H/A 안\n(Ry/Rx 25, E′ 226 GPa, HD h_cen, 9 케이스)")
    fig.tight_layout()
    os.makedirs("figures", exist_ok=True)
    fig.savefig("figures/HL06_fig6_model.png", dpi=110, bbox_inches="tight")

    # 해석 맞춤 곡선 vs 디지타이징 (교차 확인): 디지타이징 점마다 가장 가까운 해석 곡선(7 가닥 중)과의 차
    for th in THETAS:
        rows = [r for r in dig if int(float(r["theta"])) == th]
        if not rows:
            continue
        for q, key in (("amp", "value_left_axis"), ("pres", "value_right_axis")):
            sel = [r for r in rows if r.get("curve") == q]
            if not sel:
                continue
            L = np.array([float(r["Lambda"]) for r in sel])
            v = np.array([float(r[key]) for r in sel])
            curves = []
            for name in TABLE1:
                fa, fp = fits[(name, th)]
                Lf = lam_f * c_of(name)
                curves.append(np.interp(np.log10(L), np.log10(Lf), fa if q == "amp" else fp))
            dmin = np.min(np.abs(np.array(curves) - v[None, :]), axis=0)
            stats["fit_eq_vs_digitized"][f"t{th}_{q}"] = dict(
                n=int(len(v)), median=float(np.median(dmin)), p95=float(np.percentile(dmin, 95)), max=float(dmin.max()))
            mod = [np.interp(np.log10(L), np.log10(LAM_B * c_of(n)), res["cases"][n][f"t{th}"][q]) for n in TABLE1]
            dm = np.min(np.abs(np.array(mod) - v[None, :]), axis=0)        # 가장 가까운 케이스 점열과의 차
            for tag, sel_ in (("all", np.ones_like(L, bool)), ("L<=1", L <= 1.0), ("L>1", L > 1.0)):
                if sel_.any():
                    stats.setdefault("model_vs_digitized", {})[f"t{th}_{q}_{tag}"] = dict(
                        n=int(sel_.sum()), median=float(np.median(dm[sel_])), max=float(dm[sel_].max()))
    with open("ref_data/HL06_fig6_compare.json", "w", encoding="utf-8") as f:
        json.dump(stats, f, ensure_ascii=False, indent=1)
    return stats


if __name__ == "__main__":
    st = main()
    for k, v in st["fit_eq_vs_digitized"].items():
        print("fit_eq vs dig", k, {a: round(b, 3) for a, b in v.items()})
    for th in THETAS:
        rows = {k: v for k, v in st["model_vs_fit_eq"].items() if k.endswith(f"_t{th}")}
        am = max(rows.items(), key=lambda kv: kv[1]["amp_max"])
        pm = max(rows.items(), key=lambda kv: kv[1]["pres_max"])
        print(f"θ={th}: model vs fit_eq  amp max {am[1]['amp_max']:.3f} ({am[0]}, Λ {am[1]['amp_Lambda']:.3g})"
              f" · pres max {pm[1]['pres_max']:.3f} ({pm[0]}, Λ {pm[1]['pres_Lambda']:.3g})")
