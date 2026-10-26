"""§8 그림 — 본 모델(hl06_sine.py)을 Hooke & Li 2006 Part 1 Fig 3·Fig 4 형식으로 그리고 원문 디지타이징을 겹친다.

    python hl06_plot.py            ->  figures/HL06_fig3_model.png · figures/HL06_fig4_model.png · ref_data/HL06_compare.json
    python hl06_plot.py --film ref ->  figures/HL06_fig3_REF.png · figures/HL06_fig4_REF.png · ref_data/HL06_compare_REF.json
      (§8.5: D·E·F 원문 디지타이징 중심 간극, H Pan–Hamrock 선접촉 — HL06_model_REF.json · HL06_model_fig3_REF.npz,
       H/A 안 · E′ 226 GPa 선만)
축 범위·좌우축 비는 원문 그림(디지타이징 축 보정 ref_data/HL06_calib.json)과 같게 둔다.
본 모델: Ry/Rx 25, E′ 226 GPa 선 + E′ 117 GPa 와의 폭은 음영. 판정 없이 수치만(§8.3.2).
원문(회색): 디지타이징 값(bbox 중심 기준).
"""
from __future__ import annotations

import argparse
import csv
import json

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
import numpy as np  # noqa: E402
from matplotlib.lines import Line2D  # noqa: E402
from matplotlib.ticker import FuncFormatter  # noqa: E402

plt.rcParams.update({"font.family": ["Malgun Gothic", "DejaVu Sans"], "axes.unicode_minus": False, "font.size": 9,
                     "mathtext.fontset": "dejavusans"})
CAL = {(c["image"], c["axis"]): c for c in json.load(open("ref_data/HL06_calib.json", encoding="utf-8"))["calibration"]}
MODEL, LAM = None, None                                     # __main__ 에서 --film 에 따라 읽음
STY = {"hooke_li": dict(color="C0", ls="-", lw=1.2), "skf": dict(color="C3", ls="--", lw=1.2)}
QLW = {"H": 2.0, "P": 0.9}                                   # 선 굵기: h 굵게 · p 가늘게
LBL = {"hooke_li": "본 모델 H/A 안", "skf": "본 모델 식 (24) 안"}
REF = dict(color="0.45")
TAG, TAG2 = "R25_E226", "R25_E117"
SUP = str.maketrans("-0123456789", "⁻⁰¹²³⁴⁵⁶⁷⁸⁹")
DECADE = FuncFormatter(lambda v, _: "10" + str(int(round(np.log10(v)))).translate(SUP))   # U+2212 없는 위첨자


def read_csv(path):
    with open(path, encoding="utf-8") as f:
        return list(csv.DictReader(r for r in f if not r.startswith("#")))


def scale(img, part):
    """오른축/왼축 눈금비 (원문 그림 그대로)."""
    return CAL[(img, f"{part} left H/A")]["px_per_unit"] / CAL[(img, f"{part} right PbE'A")]["px_per_unit"]


def band(ax, x, y, y2, mode, label=None, lw=None):
    """y2 None 이면 음영(E′ 117 폭) 없이 선만."""
    ax.plot(x, y, label=label, **{**STY[mode], **({"lw": lw} if lw else {})})
    if y2 is not None:
        ax.fill_between(x, np.minimum(y, y2), np.maximum(y, y2), color=STY[mode]["color"], alpha=0.25, lw=0)


# ------------------------------------------------------------------ Fig 4


def model4(case, th, mode, q, part, tag=TAG):
    return np.array(MODEL["cases"][f"{case}_{tag}"][f"t{th}_{mode}"][f"{q}_{part}"])


def ref4(rows, case, th, part, q):
    p = [(float(r["lam_b"]), float(r["value"])) for r in rows
         if r["case"] == case and int(r["theta"]) == th and r["part"] == part and r["quantity"] == q]
    return np.array(p).T if p else (np.array([]), np.array([]))


def panel4(ax, rows, case, th, part, img, ylim, stats, modes, shade):
    s = scale(img, part)
    axr = ax.twinx()
    for q, a, sym in (("H", ax, dict(marker="^", mfc="0.45")), ("P", axr, dict(marker="^", mfc="none"))):
        for mode in modes:
            band(a, LAM, model4(case, th, mode, q, part), model4(case, th, mode, q, part, TAG2) if shade else None, mode,
                 LBL[mode] if q == "H" else None, lw=QLW[q])
        x, v = ref4(rows, case, th, part, q)
        if part == "re":                                         # 원문처럼 기호 옆 검정 글자 h · p
            ok = np.where(np.isfinite(v))[0]
            k = ok[np.argmax(v[ok] < 0.9)] if q == "H" else ok[np.argmax(np.abs(v[ok]) > 0.3 * np.abs(v[ok]).max())]
            if q == "H":                                         # h: 식 (24) 점선과 겹치지 않게 평탄부 아래 왼쪽
                a.text(x[k] / 30, 0.72, "h", fontsize=11, color="k")
            else:
                a.annotate("p", (x[k], v[k]), xytext=(-16, -13), textcoords="offset points", fontsize=11, color="k")
        a.semilogx(x, v, ls="none", mec="0.45", ms=4, label=f"원문 {'h ▲' if q == 'H' else 'p △'}", **sym)
        ok = np.isfinite(v)
        for mode in modes:
            y = np.interp(np.log10(x[ok]), np.log10(LAM), model4(case, th, mode, q, part))
            d = y - v[ok]
            i = int(np.argmax(np.abs(d))) if len(d) else 0
            stats[f"{case}_t{th}_{part}_{q}_{mode}"] = dict(
                n=int(ok.sum()), max_abs_diff=float(np.abs(d).max()) if len(d) else None,
                lam_at_max=float(x[ok][i]) if len(d) else None,
                ref_peak=float(np.abs(v[ok]).max()) if len(d) else None)
    ax.set_ylim(*ylim)
    axr.set_ylim(ylim[0] * s, ylim[1] * s)
    ax.axhline(0, color="k", lw=0.6)
    ax.set_xlim(1e-4, 1e2)
    ax.xaxis.set_major_formatter(DECADE)
    return axr


def fig4(modes, shade, hlabel, title, out):
    rows = read_csv("ref_data/HL06_fig4.csv")
    sm = read_csv("ref_data/HL06_fig4_smooth.csv")
    hcen = json.load(open("ref_data/HL06_fig4_hcen.json", encoding="utf-8"))["cases"]
    fig = plt.figure(figsize=(14, 12))
    gs = fig.add_gridspec(4, 3, height_ratios=[1.0, 0.6, 1.2, 1.0], hspace=0.35, wspace=0.55)
    stats = {}
    for j, case in enumerate("DEF"):
        # 1행: 평활해 — 원문 곡선 + 본 구현 HD h_cen (접촉 내부 수평선)
        ax = fig.add_subplot(gs[0, j])
        axr = ax.twinx()
        for cv, a in (("h", ax), ("p", axr)):
            p = sorted((float(r["x_b"]), float(r["value"])) for r in sm if r["case"] == case and r["curve"] == cv)
            x, v = np.array(p).T
            a.plot(x, v, color="0.45", lw=1.0, label=f"원문 {'간극' if cv == 'h' else '압력'}")
        h1 = MODEL["cases"][f"{case}_{TAG}"]["hR_b2"]
        hc = hcen[case]["h_center_hR_b2"]
        if shade:
            h2 = MODEL["cases"][f"{case}_{TAG2}"]["hR_b2"]
            ax.fill_between([-1, 1], h2, h1, color="C0", alpha=0.35, lw=0)
            stats[f"{case}_hcen"] = dict(ref=hc, model_E226=h1, model_E117=h2, ratio_E226=h1 / hc, ratio_E117=h2 / hc)
        else:
            stats[f"{case}_hcen"] = dict(ref=hc, model=h1, ratio=h1 / hc)
        ax.plot([-1, 1], [h1, h1], color="C0", lw=1.2, label=hlabel)
        ax.set_xlim(-2, 1.2)
        ax.set_ylim(0, {"D": 1.0, "E": 0.15, "F": 0.03}[case])        # 원문 축
        axr.set_ylim(0, 1.5)
        ax.set_title(f"케이스 {case} (P {MODEL_P[case][0]} · S {MODEL_P[case][1]})")
        ax.set_xlabel("x/b")
        if j == 0:
            ax.set_ylabel("평활해\nhR/b²")
            ax.legend(fontsize=7, loc="upper right")
        if j == 2:
            axr.set_ylabel("p/p_H")
        # 2행: 횡 θ = 0 (위 허수부, 아래 실수부)
        img = f"HL06_fig4_trans_{case}"
        ai = fig.add_subplot(gs[1, j])
        ar = fig.add_subplot(gs[2, j], sharex=ai)
        ri = panel4(ai, rows, case, 0, "im", img, (-0.1, 1.2), stats, modes, shade)
        rr = panel4(ar, rows, case, 0, "re", img, (-1.2, 1.2), stats, modes, shade)
        plt.setp(ai.get_xticklabels(), visible=False)
        ai.text(0.03, 0.8, "imaginary", transform=ai.transAxes)
        ar.text(0.03, 0.85, "real", transform=ar.transAxes)

        if j == 0:
            ai.set_ylabel("횡 θ=0\nH/A")
            ar.set_ylabel("H/A")
        if j == 2:
            ri.set_ylabel("P b/E′A")
            rr.set_ylabel("P b/E′A")
        # 3행: 종 θ = 90°
        img = f"HL06_fig4_inline_{case}"
        a3 = fig.add_subplot(gs[3, j])
        r3 = panel4(a3, rows, case, 90, "re", img, (-1.2, 1.2), stats, modes, shade)
        a3.set_xlabel("λ/b")
        for a in (ar, a3):                                       # 원문처럼 실수부 패널 오른쪽 위 케이스 글자
            a.text(0.88, 0.85, case, transform=a.transAxes, fontsize=11)
        if j == 0:
            a3.set_ylabel("종 θ=90°\nH/A")
        if j == 2:
            r3.set_ylabel("P b/E′A")
    hs = [Line2D([], [], color=STY[m]["color"], ls=STY[m]["ls"], lw=1.5, label=LBL[m]) for m in modes]
    hs += [Line2D([], [], color="k", lw=QLW["H"], label="굵은 선 h"), Line2D([], [], color="k", lw=QLW["P"], label="가는 선 p"),
           Line2D([], [], color="0.45", marker="^", ls="none", label="원문 h ▲"),
           Line2D([], [], color="0.45", marker="^", mfc="none", ls="none", label="원문 p △")]
    fig.legend(handles=hs, loc="lower center", ncol=len(hs), fontsize=9, bbox_to_anchor=(0.5, 0.04))
    fig.suptitle(title)
    fig.savefig(out, dpi=110, bbox_inches="tight")
    return stats


MODEL_P = {"D": (10, 7.13), "E": (20, 6.00), "F": (40, 5.05)}


# ------------------------------------------------------------------ Fig 3


def fig3(npz, modes, shade, title, out):
    rows = read_csv("ref_data/HL06_fig3.csv")
    m = np.load(npz)
    xb = m["xb"]
    fig, ax = plt.subplots(3, 2, figsize=(10, 10), sharex=True)
    stats = {}
    for j, (q, ylim, yl) in enumerate((("dh", (-1.2, 1.2), "Δh/A"), ("dp", (-2, 2), "Δp b/E′A"))):
        for i, lb in enumerate((0.1, 0.7, 3.0)):
            a = ax[i, j]
            for kind in ("curve", "env_hi", "env_lo"):
                p = sorted((float(r["x_b"]), float(r["value"])) for r in rows
                           if r["quantity"] == q and abs(float(r["lam_b"]) - lb) < 1e-9 and r["kind"] == kind)
                if not p:
                    continue
                x, v = np.array(p).T
                if kind == "curve":
                    a.plot(x, v, lw=1.0, label="원문", **REF)
                    sel = (x > -0.9) & (x < 0.8)                  # 출구 스파이크 제외
                    for mode in modes:
                        y = np.interp(x[sel], xb, m[f"{TAG}_{lb}_{mode}_{q}"])
                        stats[f"{q}_{lb}_{mode}"] = dict(rms_diff=float(np.sqrt(np.mean((y - v[sel]) ** 2))),
                                                         ref_rms=float(np.sqrt(np.mean(v[sel] ** 2))),
                                                         model_rms=float(np.sqrt(np.mean(y ** 2))))
                elif lb == 0.1:
                    a.plot(x, v, lw=0.6, label="원문 포락선" if kind == "env_hi" else None, **REF)
            for mode in modes:
                band(a, xb, m[f"{TAG}_{lb}_{mode}_{q}"], m[f"{TAG2}_{lb}_{mode}_{q}"] if shade else None, mode, LBL[mode])
            a.axhline(0, color="k", lw=0.6)
            for xv in (-1, 1):
                a.axvline(xv, color="0.8", lw=0.6)
            a.set_xlim(-2, 2)
            a.set_ylim(*ylim)
            a.set_ylabel(f"λ/b = {lb}\n{yl}")
            if i == 0:
                a.set_title(f"{q[1]}  ({yl})", fontsize=12, fontstyle="italic")      # 열 위 h · p
            if i == 2:
                a.set_xlabel("x/b")
    ax[1, 0].legend(fontsize=8, loc="lower left")
    fig.suptitle(title)
    fig.tight_layout()
    fig.savefig(out, dpi=110)
    return stats


FILM = {   # --film: 입력 · 모드 · E′ 117 음영 · 막두께 라벨 · 제목 · 출력
    "hd": dict(model="ref_data/HL06_model.json", npz="ref_data/HL06_model_fig3.npz", modes=("hooke_li", "skf"),
               shade=True, hlabel="본 구현 HD h_cen",
               t4="Hooke & Li 2006 Fig 4 형식 — 본 모델(선·음영 E′ 117~226 GPa, Ry/Rx 25) vs 원문(회색 ▲ h · △ p)",
               t3="Hooke & Li 2006 Fig 3 형식 — 케이스 H(P 20 · S 8), 횡 θ=0, t=0\n"
                  "본 모델(접촉 내부 |x/b| ≤ 1만 정의, 음영 E′ 117~226 GPa) vs 원문(회색)",
               out4="figures/HL06_fig4_model.png", out3="figures/HL06_fig3_model.png", cmp="ref_data/HL06_compare.json"),
    "ref": dict(model="ref_data/HL06_model_REF.json", npz="ref_data/HL06_model_fig3_REF.npz", modes=("hooke_li",),
                shade=False, hlabel="본 구현 h = 원문 중심 간극(디지타이징)",
                t4="Hooke & Li 2006 Fig 4 형식 — 본 모델(h = 원문 평활해 중심 간극, Ry/Rx 25, E′ 226 GPa)"
                   " vs 원문(회색 ▲ h · △ p)",
                t3="Hooke & Li 2006 Fig 3 형식 — 케이스 H(P 20 · S 8), 횡 θ=0, t=0\n"
                   "본 모델(h = Pan–Hamrock 선접촉, Ry/Rx 25, E′ 226 GPa, 접촉 내부 |x/b| ≤ 1만 정의) vs 원문(회색)",
                out4="figures/HL06_fig4_REF.png", out3="figures/HL06_fig3_REF.png", cmp="ref_data/HL06_compare_REF.json"),
}


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--film", choices=FILM, default="hd")
    ap.add_argument("--model", help="본 모델 json (기본: --film 값)")
    ap.add_argument("--npz", help="본 모델 Fig 3 npz (기본: --film 값)")
    ap.add_argument("--outdir", help="그림 2개·비교 json 을 이 폴더에 (시험용)")
    args = ap.parse_args()
    c = FILM[args.film]
    if args.outdir:
        c = {**c, **{k: f"{args.outdir}/{c[k].split('/')[-1]}" for k in ("out4", "out3", "cmp")}}
    MODEL = json.load(open(args.model or c["model"], encoding="utf-8"))
    LAM = np.array(MODEL["lam_b"])
    out = {"fig4": fig4(c["modes"], c["shade"], c["hlabel"], c["t4"], c["out4"]),
           "fig3": fig3(args.npz or c["npz"], c["modes"], c["shade"], c["t3"], c["out3"])}
    with open(c["cmp"], "w", encoding="utf-8") as f:
        json.dump(out, f, ensure_ascii=False, indent=1)
    for k, v in out.items():
        for kk, vv in v.items():
            print(k, kk, {a: (round(b, 3) if isinstance(b, float) else b) for a, b in vv.items()})
