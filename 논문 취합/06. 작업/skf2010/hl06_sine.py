"""§8 M2 사인파 검토 — Hooke & Li 2006 Part 1 Fig 3·4 대응 계산 (작업내역서 §8).

조건(사용자 결정 2026-10-12, §8.2):
  무차원 P·S·p_H 만 사용(Table 1), α = P/p_H, Barus 점도, Dowson–Higginson 압축성.
  선접촉 → Ry/Rx 큰 타원접촉(10·25·50), b ↔ a_x, R_eff ↔ Rx, h = 본 구현 Hamrock–Dowson h_cen.
  Rx·η0/ū 분할은 척도 — 무차원 결과 불변을 테스트로 먼저 확인(불변 아니면 중단).
  E′: M2 자체는 불변이나 HD h_cen 이 G 를 따로 써 hR/b² 가 약 3 % 변함
      → 226·117 GPa 두 값으로 계산해 폭으로 보고(사용자 결정 2026-10-12).
  S = 0 두 안: "skf"(식 24) · "hooke_li"(H/A).
단일 사인파 r = A cos(ω x) (t = 0, 거칠기 최대가 중심):
  횡방향 θ = 0 → (wx, wy) = (κ, 0), 종방향 θ = 90° → (0, κ), κ = 2π/λ.
  Δh(x) = Re[h_a e^{iωx} + h_c e^{-iωa} e^{iψ(x+a)}]  (ehl_ripple.ripple_line 과 같은 조립)
  중심 복소비 H(0)/A = h_a + h_c e^{i(ψ-ω)a},  P(0)b/E′A = (p_a + p_c e^{i(ψ-ω)a}) a_x/E′.

    python hl06_sine.py  ->  ref_data/HL06_model.json · ref_data/HL06_model_fig3.npz
    python hl06_sine.py --film ref  ->  ref_data/HL06_model_REF.json · ref_data/HL06_model_fig3_REF.npz
      (§8.5, 사용자 결정 2026-10-15: Fig 4 케이스 D·E·F 는 원문 Fig 4 1행 디지타이징 중심 간극(HL06_fig4_hcen.json),
       Fig 3 케이스 H 는 Pan & Hamrock 1989 선접촉 중심막(NASA RP-1255 식 21-60); Ry/Rx 25 · E′ 226 GPa · H/A 안만)
"""
from __future__ import annotations

import argparse
import json
import os

import numpy as np

from conditions import Case
from ehl_ripple import ripple_component

TABLE1 = {"A": (10, 4.75, 0.5e9), "B": (20, 4.00, 1.0e9), "C": (40, 3.36, 2.0e9),   # 원문 Table 1 (P, S, p_H)
          "D": (10, 7.13, 0.5e9), "E": (20, 6.00, 1.0e9), "F": (40, 5.05, 2.0e9),
          "G": (10, 9.51, 0.5e9), "H": (20, 8.00, 1.0e9), "I": (40, 6.73, 2.0e9)}
LAM_B = 10.0 ** (-4 + 0.15 * np.arange(41))          # 원문 41 점 격자
MODES = ("skf", "hooke_li")
RATIOS = (10.0, 25.0, 50.0)                            # Ry/Rx (§8.2.2)
E_SET = (226e9, 117e9)                                 # E′ 폭 (§8.2.5)
HCEN = json.load(open("ref_data/HL06_fig4_hcen.json", encoding="utf-8"))["cases"]   # 원문 평활해 중심 간극


def make_case(name, ratio=25.0, E=226e9, Rx=0.01, eta0=0.05):
    """Table 1 (P, S, p_H) → Case. E, Rx, eta0 는 척도(불변 테스트 대상)."""
    P, S, ph = TABLE1[name]
    alpha = P / ph
    U = (S / (alpha * E)) ** 4                         # S = G U^0.25, G = αE′
    u = U * E * Rx / eta0                              # U = η0 ū /(E′ Rx)
    return Case(name, E, eta0, alpha, u, Rx, ratio * Rx, ph)


def h_pan_hamrock(case):
    """Pan & Hamrock (1989) 선접촉 중심막 (NASA RP-1255 식 21-60) → 본 모델 반폭 a_x 로 환산한 h [m].

    H_c = 2.922 W'^-0.166 U^0.692 G^0.470,  W' = 2π (p_H/E′)²,  p_H/E′ = P/G,  b/R = 4 p_H/E′
    → hR/b² = H_c / (16 (P/G)²),  h = (hR/b²) a_x² / Rx.
    """
    G = case.alpha * case.E_eff
    U = case.eta0 * case.u_bar / (case.E_eff * case.Rx)
    if case.name in TABLE1:                            # Table 1 케이스면 P · S 일치 확인 (웹 계산기 등 일반 케이스는 생략)
        P, S, _ = TABLE1[case.name]
        assert np.isclose(case.alpha * case.p_h, P, rtol=1e-12), (case.name, case.alpha * case.p_h, P)
        assert np.isclose(G * U**0.25, S, rtol=1e-12), (case.name, G * U**0.25, S)
    pe = case.p_h / case.E_eff                         # = P/G
    Hc = 2.922 * (2.0 * np.pi * pe**2) ** -0.166 * U**0.692 * G**0.470
    return Hc / (16.0 * pe**2) * case.a_x**2 / case.Rx


def h_digitized(case):
    """원문 Fig 4 1행 평활해 중심 간극 hR/b² (디지타이징) → 본 모델 반폭 a_x 로 환산한 h [m]. 케이스 D·E·F 만."""
    hb = HCEN[case.name]["h_center_hR_b2"]
    return hb * case.a_x**2 / case.Rx


def h_ref(case):
    """§8.5 막두께: D·E·F 는 원문 디지타이징, H 는 Pan & Hamrock."""
    return h_digitized(case) if case.name in HCEN else h_pan_hamrock(case)


def centre(case, lam_b, theta, mode, h=None):
    """중심 복소비 (H(0)/A, P(0)b/E′A) 배열. h = None 이면 case.h_cen."""
    h = case.h_cen if h is None else h
    a = case.a_x
    k = 2.0 * np.pi / (lam_b * a)
    if theta in (0, 90):
        wx, wy = (k, 0.0 * k) if theta == 0 else (0.0 * k, k)
    else:                                              # 경사 방향(§8.6 Fig 6, θ = 45°)
        wx, wy = k * np.cos(np.radians(theta)), k * np.sin(np.radians(theta))
    s = ripple_component(np.ones_like(k) + 0j, wx, wy, h, case, 0.0, s0_mode=mode)
    ph = np.exp(1j * (s["psi"] - wx) * a)
    H = s["h_a"] + s["h_cf"] * ph
    Pc = (s["p_a"] + s["p_cf"] * ph) * a / case.E_eff
    return H, Pc


def line(case, lam_b, mode, xb, h=None):
    """횡방향 공간 분포 Δh/A, Δp b/E′A (x/b = xb, t = 0). h = None 이면 case.h_cen."""
    h = case.h_cen if h is None else h
    a = case.a_x
    k = np.array([2.0 * np.pi / (lam_b * a)])
    s = ripple_component(np.ones(1) + 0j, k, np.zeros(1), h, case, 0.0, s0_mode=mode)
    x = xb * a
    ea = np.exp(1j * k * x)
    ec = np.exp(-1j * k * a) * np.exp(1j * s["psi"] * (x + a))
    dh = (s["h_a"] * ea + s["h_cf"] * ec).real
    dp = ((s["p_a"] * ea + s["p_cf"] * ec) * a / case.E_eff).real
    return dh, dp


def invariance(modes=MODES, hfun=lambda c: c.h_cen):
    """척도 Rx·η0 를 바꿔 무차원 결과의 최대 상대차(E′ 는 폭으로 따로 보고)."""
    worst = 0.0
    for name in ("D", "F"):
        ref = None
        for Rx, eta0 in ((0.01, 0.05), (0.04, 0.05), (0.01, 0.8)):
            c = make_case(name, 25.0, 226e9, Rx, eta0)
            h = hfun(c)
            out = [np.concatenate(centre(c, LAM_B, th, m, h)) for th in (0, 90) for m in modes]
            out.append(np.array([h * c.Rx / c.a_x**2]))
            v = np.concatenate(out)
            if ref is None:
                ref = v
                continue
            d = np.abs(v - ref) / np.maximum(np.abs(ref), 1.0)
            worst = max(worst, float(np.nanmax(d)))
    return worst


def main_ref():
    """§8.5 막두께 안(h_ref): Ry/Rx 25, E′ 226 GPa, H/A 안만."""
    m, ratio, E = "hooke_li", 25.0, 226e9
    worst = invariance((m,), h_ref)
    print(f"척도 불변 최대 상대차 (REF) = {worst:.2e}")
    if not worst < 1e-8:
        raise SystemExit("무차원 결과가 척도에 따라 변함 — 중단(§8.2.5)")

    res = {"invariance_max_rel": worst, "lam_b": LAM_B.tolist(),
           "film": "D·E·F: 원문 Fig 4 1행 디지타이징 중심 간극 · H: Pan-Hamrock 1989 line (RP-1255 eq 21-60)",
           "cases": {}}
    tag = f"R{int(ratio)}_E{int(E / 1e9)}"
    for name in ("D", "E", "F"):
        c = make_case(name, ratio, E)
        h = h_ref(c)
        d = {"hR_b2": h * c.Rx / c.a_x**2, "hR_b2_HD": c.h_cen * c.Rx / c.a_x**2,
             "hR_b2_PH": h_pan_hamrock(c) * c.Rx / c.a_x**2,
             "b_over_line": c.a_x * c.E_eff / (4 * c.p_h * c.Rx)}
        for th in (0, 90):
            H, P = centre(c, LAM_B, th, m, h)
            d[f"t{th}_{m}"] = {"H_re": H.real.tolist(), "H_im": H.imag.tolist(),
                               "P_re": P.real.tolist(), "P_im": P.imag.tolist()}
        res["cases"][f"{name}_{tag}"] = d
        print(f"{name}_{tag}", f"hR/b2(ref)={d['hR_b2']:.5g}", f"(PH)={d['hR_b2_PH']:.5g}", f"(HD)={d['hR_b2_HD']:.5g}")

    xb = np.linspace(-1.0, 1.0, 801)
    fig3 = {"xb": xb}
    c = make_case("H", ratio, E)
    h = h_ref(c)
    for lb in (0.1, 0.7, 3.0):
        dh, dp = line(c, lb, m, xb, h)
        fig3[f"{tag}_{lb}_{m}_dh"] = dh
        fig3[f"{tag}_{lb}_{m}_dp"] = dp
    res["cases"][f"H_{tag}"] = {"hR_b2": h * c.Rx / c.a_x**2, "hR_b2_HD": c.h_cen * c.Rx / c.a_x**2}
    print(f"H_{tag}", f"hR/b2(ref, PH)={res['cases'][f'H_{tag}']['hR_b2']:.5g}")
    os.makedirs("ref_data", exist_ok=True)
    np.savez("ref_data/HL06_model_fig3_REF.npz", **fig3)
    with open("ref_data/HL06_model_REF.json", "w", encoding="utf-8") as f:
        json.dump(res, f, ensure_ascii=False, indent=1)


def main():
    worst = invariance()
    print(f"척도 불변 최대 상대차 = {worst:.2e}")
    if not worst < 1e-8:
        raise SystemExit("무차원 결과가 척도에 따라 변함 — 중단(§8.2.5)")

    res = {"invariance_max_rel": worst, "lam_b": LAM_B.tolist(), "cases": {}}
    for name, ratio, E in ((n, r, e) for n in ("D", "E", "F") for r in RATIOS for e in E_SET):
        c = make_case(name, ratio, E)
        key = f"{name}_R{int(ratio)}_E{int(E / 1e9)}"
        d = {"b_over_line": c.a_x * c.E_eff / (4 * c.p_h * c.Rx),   # 선접촉 b/R = 4p_H/E′ 대비
             "hR_b2": c.h_cen * c.Rx / c.a_x**2}
        for th in (0, 90):
            for m in MODES:
                H, P = centre(c, LAM_B, th, m)
                d[f"t{th}_{m}"] = {"H_re": H.real.tolist(), "H_im": H.imag.tolist(),
                                   "P_re": P.real.tolist(), "P_im": P.imag.tolist()}
        res["cases"][key] = d
        print(key, f"b/b_line={d['b_over_line']:.4f}", f"hR/b2={d['hR_b2']:.4f}")

    xb = np.linspace(-1.0, 1.0, 801)
    fig3 = {"xb": xb}
    for ratio, E in ((r, e) for r in RATIOS for e in E_SET):
        c = make_case("H", ratio, E)
        tag = f"R{int(ratio)}_E{int(E / 1e9)}"
        for lb in (0.1, 0.7, 3.0):
            for m in MODES:
                dh, dp = line(c, lb, m, xb)
                fig3[f"{tag}_{lb}_{m}_dh"] = dh
                fig3[f"{tag}_{lb}_{m}_dp"] = dp
        res["cases"][f"H_{tag}"] = {"hR_b2": c.h_cen * c.Rx / c.a_x**2,
                                    "b_over_line": c.a_x * c.E_eff / (4 * c.p_h * c.Rx)}
    os.makedirs("ref_data", exist_ok=True)
    np.savez("ref_data/HL06_model_fig3.npz", **fig3)
    with open("ref_data/HL06_model.json", "w", encoding="utf-8") as f:
        json.dump(res, f, ensure_ascii=False, indent=1)


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--film", choices=("hd", "ref"), default="hd",
                    help="hd: Hamrock–Dowson h_cen (기본) · ref: D·E·F 원문 디지타이징, H Pan & Hamrock (§8.5)")
    main_ref() if ap.parse_args().film == "ref" else main()
