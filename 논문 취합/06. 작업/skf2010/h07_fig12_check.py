"""§5.3 중간 파장 검토 — H/A 근사(s0_mode="hooke_li_all")의 입구 보완파 진폭 |h_c/A| vs Hooke 2007 P1 Fig 12 (디지타이징).

조건(사용자 결정 2026-10-16): 원문 Fig 10 조건 P 20 · S 7 · p_H 1 GPa · τ0 8 MPa, E′ 226 GPa 가정(117 GPa 는 민감도).
  §8.4 관례: Ry/Rx 25 타원접촉 · HD h_cen · Barus. v/u = 0.5 · 0.95 → SKF S = 2(v/u − 1) = −1.0 · −0.1.
  횡방향(ω_y = 0) 단일 성분, |h_c/A| = |h_t − h_a(S)| (r_a = 1, 입구 진폭 크기).
산출: figures/H07P1_fig12_check.png · ref_data/H07P1_fig12_check.json
"""
import csv
import json

import numpy as np
import matplotlib
import matplotlib.ticker
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

from conditions import Case  # noqa: E402
from ehl_ripple import ripple_component  # noqa: E402

plt.rcParams.update({"font.family": ["Malgun Gothic", "DejaVu Sans"], "axes.unicode_minus": False, "font.size": 11})
SUP = str.maketrans("-0123456789", "⁻⁰¹²³⁴⁵⁶⁷⁸⁹")
P, SG, PH, TAU0 = 20.0, 7.0, 1.0e9, 8e6
C_LAM = P**1.5 / SG**2                                   # Λ = (λ/b) · C_LAM
LAM = 10.0 ** np.linspace(np.log10(0.03), np.log10(30), 161)


def case_fig10(E, ratio=25.0, Rx=0.01, eta0=0.05):
    alpha = P / PH
    U = (SG / (alpha * E)) ** 4
    return Case("H07 Fig10", E, eta0, alpha, U * E * Rx / eta0, Rx, ratio * Rx, PH, tau0=TAU0)


def hc_amp(case, vu):
    S = 2.0 * (vu - 1.0)
    a = case.a_x
    k = 2.0 * np.pi / (LAM / C_LAM * a)
    s = ripple_component(np.ones_like(k) + 0j, k, 0.0 * k, case.h_cen, case, S, s0_mode="hooke_li_all")
    return np.abs(s["h_cf"]), np.abs(s["h_a"])


def main():
    out = {"Lambda": LAM.tolist()}
    rows = list(csv.DictReader(open("ref_data/H07P1_fig12_digitized.csv", encoding="utf-8")))
    fig, ax = plt.subplots(figsize=(8, 5.5))
    for kind, mk, lab in (("plus", "+", "원문 v/u = 0.95 (+)"), ("square", "s", "원문 v/u = 0.5 (■)"),
                          ("unknown", ".", "원문 미구분")):
        p = [(float(r["Lambda"]), float(r["hc_A"])) for r in rows if r["kind"] == kind
             and not (kind == "square" and abs(float(r["Lambda"]) - 0.117) < 0.003)]   # 오검출 의심 1점 제외
        if p:
            x, y = np.array(p).T
            ax.semilogx(x, y, mk, color="0.55", ms=5 if kind != "unknown" else 3, mfc="0.55" if kind == "square" else "none",
                        label=lab)
    for vu, col in ((0.95, "C0"), (0.5, "C3")):
        for E, ls in ((226e9, "-"), (117e9, ":")):
            hc, ha = hc_amp(case_fig10(E), vu)
            out[f"vu{vu}_E{int(E/1e9)}"] = {"hc": hc.tolist(), "ha": ha.tolist()}
            ax.semilogx(LAM, hc, ls, color=col, lw=1.8 if ls == "-" else 1.2,
                        label=f"본 근사 v/u = {vu}, E′ {int(E/1e9)} GPa")
            if E == 226e9:
                i = int(np.argmax(hc))
                print(f"v/u {vu} E226: max |h_c/A| {hc[i]:.3f} @ Λ {LAM[i]:.3g};  "
                      + " ".join(f"Λ{L}:{np.interp(np.log10(L), np.log10(LAM), hc):.2f}" for L in (0.1, 0.2, 0.5, 1, 3, 10)))
    ax.set_xlim(3e-2, 3e1)
    ax.xaxis.set_major_formatter(matplotlib.ticker.FuncFormatter(lambda v, _: "10" + str(int(round(np.log10(v)))).translate(SUP)))
    ax.set_ylim(0, 1.05)
    ax.set_xlabel("Wavelength Λ = (λx/b) S⁻² P¹·⁵")
    ax.set_ylabel("Comp. amp. ratio |h_c/A|")
    ax.grid(True, which="major", ls=":", color="0.75")
    ax.legend(fontsize=9, loc="upper right")
    ax.set_title("Hooke 2007 P1 Fig 12 (회색) vs 본 H/A 근사 입구 보완파 진폭\n(Fig 10 조건 P 20 · S 7 · p_H 1 GPa · τ0 8 MPa, Ry/Rx 25, HD)",
                 fontsize=10)
    fig.tight_layout()
    fig.savefig("figures/H07P1_fig12_check.png", dpi=130)
    json.dump(out, open("ref_data/H07P1_fig12_check.json", "w", encoding="utf-8"), indent=1)


if __name__ == "__main__":
    main()
