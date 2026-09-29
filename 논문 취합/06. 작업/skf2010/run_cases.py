"""SKF(2010) 모델 검증 실행 — 계획서 §6.

현재 구현: V0 (M2 단일 정현파 해석/극한 검사).
"""
from __future__ import annotations

import numpy as np
import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

import conditions as C
import ehl_ripple as M2

RESULTS: list[tuple[str, str, str, bool]] = []   # (ID, 항목, 관측, 합격)


def rec(tid, label, observed, ok):
    RESULTS.append((tid, label, observed, bool(ok)))


# ---------------------------------------------------------------- V0-1


def v0_1_dimensionless():
    """Q, C 가 단위계에 무관한 무차원량인지 수치로 확인.

    SI(m, Pa, s) 와 (mm, MPa, s) 두 단위계에서 같은 값이 나와야 한다.
    """
    Ls, Ps = 1e3, 1e-6          # m -> mm, Pa -> MPa
    base = dict(r_a=0.3e-6, wx=2 * np.pi / 100e-6, wy=2 * np.pi / 150e-6,
                h=0.2e-6, E_eff=219e9, B_bulk=8.7e9,
                u2=0.20, u_bar=0.15, eta_x=1.2, eta_y=1.2)
    si = M2.particular_integral(**base)

    scaled = dict(
        r_a=base["r_a"] * Ls, wx=base["wx"] / Ls, wy=base["wy"] / Ls,
        h=base["h"] * Ls, E_eff=base["E_eff"] * Ps, B_bulk=base["B_bulk"] * Ps,
        u2=base["u2"] * Ls, u_bar=base["u_bar"] * Ls,
        eta_x=base["eta_x"] * Ps, eta_y=base["eta_y"] * Ps,
    )
    sc = M2.particular_integral(**scaled)

    eq = max(abs(si["Q"] - sc["Q"]) / abs(si["Q"]),
             abs(si["C"] - sc["C"]) / abs(si["C"]))
    # h_a 는 길이이므로 Ls 배가 되어야 한다
    eh = abs(sc["h_a"] - si["h_a"] * Ls) / abs(si["h_a"] * Ls)
    rec("V0-1", "Q, C 단위계 불변 (무차원)", f"상대차 {eq:.2e}", eq < 1e-12)
    rec("V0-1", "h_a 길이차원 스케일링", f"상대차 {eh:.2e}", eh < 1e-12)
    return si


# ---------------------------------------------------------------- V0-2


def v0_2_newtonian():
    """tau_m -> 0 에서 Eyring 등가점도가 Newtonian 으로 환원되는가 (식 19)."""
    eta = 1.2
    ex, ey = M2.eta_equivalent(eta, 0.0, 8e6)
    e1 = max(abs(ex - eta), abs(ey - eta)) / eta
    rec("V0-2", "식(19) tau_m=0 -> eta_x=eta_y=eta", f"상대차 {e1:.2e}", e1 < 1e-14)

    ex2, ey2 = M2.eta_equivalent(eta, 1.0e3, 8e6)      # tau_m/tau0 = 1.25e-4
    e2 = max(abs(ex2 - eta), abs(ey2 - eta)) / eta
    rec("V0-2", "식(19) tau_m/tau0 << 1 연속성", f"상대차 {e2:.2e}", e2 < 1e-6)

    ex3, ey3 = M2.eta_equivalent(eta, 8e6, 8e6)        # tau_m/tau0 = 1
    ok = ex3 < eta and ey3 < eta and ex3 < ey3
    rec("V0-2", "식(19) tau_m=tau0 전단박화 (eta_x<eta_y<eta)",
        f"eta_x={ex3:.4f}, eta_y={ey3:.4f}, eta={eta}", ok)


# ---------------------------------------------------------------- V0-3


def v0_3_pure_rolling():
    """순수구름 S=0 (u2=u_bar) -> Q=0, p_a=0, h_a=r_a (변형 없음)."""
    r_a = 0.3e-6
    kw = dict(r_a=r_a, wx=2 * np.pi / 100e-6, wy=2 * np.pi / 150e-6,
              h=0.2e-6, E_eff=219e9, B_bulk=8.7e9, u_bar=0.15,
              eta_x=1.2, eta_y=1.2)
    pr = M2.particular_integral(u2=0.15, **kw)
    ok = pr["Q"] == 0.0 and pr["p_a"] == 0 and abs(pr["h_a"] - r_a) < 1e-30
    rec("V0-3", "순수구름 -> Q=0, p_a=0, h_a=r_a",
        f"Q={pr['Q']:.1e}, |p_a|={abs(pr['p_a']):.1e}, h_a/r_a={pr['h_a'].real/r_a:.6f}", ok)

    # Q -> 0 연속성: p_a 가 (u2-u_bar) 에 1차 비례하는가
    amp = []
    for du in (1e-4, 1e-5, 1e-6):
        p = M2.particular_integral(u2=0.15 + du, **kw)
        amp.append(abs(p["p_a"]) / du)
    lin = max(abs(a / amp[-1] - 1.0) for a in amp)
    rec("V0-3", "Q->0 에서 p_a ∝ (u2-u_bar) 1차 수렴", f"기울기 편차 {lin:.2e}", lin < 1e-3)


# ---------------------------------------------------------------- V0-4


def v0_4_flattening_limit():
    """식 (18),(10),(11) 을 손으로 정리한 닫힌형과 대조하고 극한 거동을 본다.

        h_a / r_a = (1 - i Q C) / (1 - i Q (1 + C))

    이 항등식은 Q, C 전 범위에서 성립해야 한다. 극한은
      Q -> 0   : h_a -> r_a          (변형 없음)
      Q -> inf : h_a -> r_a C/(1+C)  (압축성이 남기는 잔류 리플)
      C = 0    : |h_a|/r_a = 1/sqrt(1+Q^2)  (비압축성, 1/Q 로 평탄화)
    """
    r_a = 0.3e-6
    kw = dict(r_a=r_a, wx=2 * np.pi / 100e-6, wy=0.0, h=0.2e-6,
              E_eff=219e9, u_bar=0.15, eta_x=1.2, eta_y=1.2)

    worst = 0.0
    for B in (8.7e9, 1.7e9, 1e30):
        for du in (1e-3, 1e-1, 1e1, 1e3, 1e5):
            p = M2.particular_integral(B_bulk=B, u2=0.15 + du, **kw)
            Q, Cv = p["Q"], p["C"]
            closed = (1.0 - 1j * Q * Cv) / (1.0 - 1j * Q * (1.0 + Cv))
            worst = max(worst, abs(p["h_a"] / r_a - closed))
    rec("V0-4", "h_a/r_a = (1-iQC)/(1-iQ(1+C)) 닫힌형 항등",
        f"최대 편차 {worst:.2e} (Q, C 15조합)", worst < 1e-12)

    p = M2.particular_integral(B_bulk=8.7e9, u2=0.15 + 1e5, **kw)
    Cv = p["C"]
    err = abs(abs(p["h_a"]) / r_a - Cv / (1 + Cv)) / (Cv / (1 + Cv))
    rec("V0-4", "Q->inf 에서 h_a/r_a -> C/(1+C)",
        f"h_a/r_a={abs(p['h_a'])/r_a:.6f}, C/(1+C)={Cv/(1+Cv):.6f}, 상대차 {err:.1e}",
        err < 1e-6)

    # 비압축성: |h_a|/r_a 가 1/Q 로 감소하는가 (du 10배 -> 비 10배)
    rs = []
    for du in (1e3, 1e4, 1e5):
        p2 = M2.particular_integral(B_bulk=1e30, u2=0.15 + du, **kw)
        rs.append(abs(p2["h_a"]) / r_a)
    decade = [rs[i] / rs[i + 1] for i in range(len(rs) - 1)]
    rec("V0-4", "비압축성(C=0): |h_a|/r_a ∝ 1/Q 로 평탄화",
        f"속도 10배당 비 {decade[0]:.3f}, {decade[1]:.3f}",
        all(abs(d - 10.0) < 0.05 for d in decade))


# ---------------------------------------------------------------- V0-5


def v0_5_complementary_wave():
    """식 (21) 수렴성과 물리적 타당성: beta = Im(psi) > 0 (감쇠), beta ∝ h^3 경향."""
    case = C.DEOLALIKAR
    eta = case.eta0 * np.exp(case.alpha * case.p_h)
    B = C.bulk_modulus(case.p_h)
    wx, wy = 2 * np.pi / 100e-6, 2 * np.pi / 150e-6

    betas, oks, its = [], [], []
    for h in (0.10e-6, 0.15e-6, 0.20e-6, 0.30e-6):
        psi, nit, ok = M2.solve_psi(wx, wy, h, case.E_eff, B, case.u_bar,
                                    case.u_bar, eta, eta)
        betas.append(psi.imag)
        oks.append(ok)
        its.append(nit)
    rec("V0-5", "식(21) 축차대입 수렴", f"반복 {min(its)}~{max(its)}회", all(oks))
    rec("V0-5", "감쇠율 beta = Im(psi) > 0", f"beta = {min(betas):.3e}~{max(betas):.3e} 1/m",
        all(b > 0 for b in betas))
    rec("V0-5", "beta 가 h 증가에 따라 단조증가",
        "단조증가" if all(np.diff(betas) > 0) else "비단조",
        bool(all(np.diff(betas) > 0)))

    # 순수미끄럼(정지 거친면 u2=0) -> omega_x' = 0, psi 는 순허수(전파 없이 감쇠만)
    psi0, _, ok0 = M2.solve_psi(wx, wy, 0.15e-6, case.E_eff, B, case.u_bar,
                                0.0, eta, eta)
    rec("V0-5", "u2=0 (정지 거친면) -> Re(psi)=0, 순수감쇠",
        f"Re={psi0.real:.2e}, Im={psi0.imag:.3e}", abs(psi0.real) < 1e-6 and psi0.imag > 0 and ok0)


# ---------------------------------------------------------------- V0-6


def v0_6_amplitude_reduction():
    """식 (24) 진폭감쇠 곡선의 극한과 단조성, 그리고 U1 3안 비교."""
    nab = np.logspace(-2, 2, 200)
    ht = M2.total_amplitude(1.0, nab)
    rec("V0-6", "nabla->0 에서 h_t/r_a -> 1", f"{ht[0]:.6f}", abs(ht[0] - 1) < 2e-3)
    rec("V0-6", "nabla=100 에서 h_t/r_a -> 0", f"{ht[-1]:.2e}", ht[-1] < 1e-2)
    rec("V0-6", "h_t/r_a 단조감소", "단조감소" if all(np.diff(ht) < 0) else "비단조",
        bool(all(np.diff(ht) < 0)))

    # U1 3안: 순수구름 S=0 (Newtonian, Q=0 -> K=1) 및 정지범프 S=-2
    rows = []
    for S in (0.0, -2.0):
        for name in M2.U1_VARIANTS:
            nn, K = M2.ar_param_nn(10.0, 0.0, S, name)
            rows.append((S, name, nn, K))
    nan_A = np.isnan([r[2] for r in rows if r[0] == -2 and r[1] == "A_paper"][0])
    rec("V0-6", "U1 변형 A(논문표기) 는 S=-2 에서 정의 불가",
        "NaN 발생" if nan_A else "값 존재", bool(nan_A))
    okB = [r[2] for r in rows if r[0] == -2 and r[1] == "B_half"][0]
    rec("V0-6", "U1 변형 B(1+S/2) 는 S=-2 에서 nabla_nn=0 (감쇠 없음)",
        f"nabla_nn={okB:.3e}", abs(okB) < 1e-12)
    return nab, ht, rows


# ---------------------------------------------------------------- V0-7


def v0_7_ar_param_crosscheck():
    """nabla = lam sqrt(M)/(a_x sqrt(L)) 정의 검산 (계획서 K10 flat-top 주기)."""
    lam = (90e-6 + 35e-6) * np.sqrt(2.0)            # = 176.8 um
    got = C.LEEDS.ar_param(lam)
    _, _, _, M_, L_ = C.LEEDS.dims
    hand = lam * np.sqrt(M_) / (C.LEEDS.a_x * np.sqrt(L_))
    rec("V0-7", "nabla 정의 검산 (Leeds, lam=176.8um)",
        f"nabla={got:.4f} (M={M_:.2f}, L={L_:.2f}, a_x={C.LEEDS.a_x*1e6:.1f}um)",
        abs(got - hand) < 1e-12)

    d = C.DEOLALIKAR
    rec("V0-7", "nabla (Deolalikar, lam=2*18um 범프폭)",
        f"nabla={d.ar_param(36e-6):.4f}", True)


# ---------------------------------------------------------------- V0-8


def v0_8_assembly():
    """조립 일관성: h_a + h_cf = h_t (계획서 §2.2(c))."""
    case = C.DEOLALIKAR
    out = M2.ripple_component(r_a=0.6e-6, lam_x=100e-6, lam_y=150e-6,
                              h=case.h_cen, case=case, S=0.0,
                              ar_param=case.ar_param(100e-6))
    resid = abs(out["h_a"] + out["h_cf"] - out["h_t"])
    rec("V0-8", "h_a + h_cf = h_t 항등", f"잔차 {resid:.2e} m", resid < 1e-18)
    rec("V0-8", "순수구름에서 h_a = r_a (복소부 0)",
        f"h_a={out['h_a'].real*1e6:.4f}+{out['h_a'].imag*1e6:.1e}j um",
        abs(out["h_a"].imag) < 1e-20)
    return out


# ---------------------------------------------------------------- 그림


def plot_v0(nab, ht, path="figures/V0_amplitude_reduction.png"):
    import os
    os.makedirs(os.path.dirname(path), exist_ok=True)
    fig, ax = plt.subplots(figsize=(7, 4.4))
    ax.semilogx(nab, ht, "k-", lw=2,
                label=r"eq.(24) with $\nabla$  (Venner-Lubrecht)")
    for name, ls in zip(("A_paper", "B_half", "C_abs"), ("--", "-.", ":")):
        nn = np.array([M2.ar_param_nn(v, 0.0, 0.0, name)[0] for v in nab])
        ax.semilogx(nab, M2.total_amplitude(1.0, nn), ls, lw=1.6,
                    label=f"eq.(22) $\\nabla_{{nn}}$, S=0, U1={name}")
    ax.set_xlabel(r"$\nabla$")
    ax.set_ylabel(r"$h_{\rm t}/r_{\rm a}$")
    ax.set_title("V0-6  amplitude reduction (eq. 24) and U1 variants")
    ax.grid(True, which="both", alpha=0.3)
    ax.legend(fontsize=8)
    fig.tight_layout()
    fig.savefig(path, dpi=150)
    plt.close(fig)
    return path


# ---------------------------------------------------------------- main


def run_v0():
    v0_1_dimensionless()
    v0_2_newtonian()
    v0_3_pure_rolling()
    v0_4_flattening_limit()
    v0_5_complementary_wave()
    nab, ht, u1rows = v0_6_amplitude_reduction()
    v0_7_ar_param_crosscheck()
    v0_8_assembly()
    png = plot_v0(nab, ht)

    print(f"{'ID':<7}{'항목':<44}{'관측':<46}판정")
    print("-" * 110)
    for tid, label, obs, ok in RESULTS:
        print(f"{tid:<7}{label:<44}{obs:<46}{'PASS' if ok else 'FAIL'}")
    npass = sum(r[3] for r in RESULTS)
    print("-" * 110)
    print(f"{npass}/{len(RESULTS)} PASS    그림: {png}")

    print("\n[U1 3안 비교]  ar_param=10, Q=0")
    print(f"{'S':>6}{'variant':>10}{'nabla_nn':>14}{'K':>8}")
    for S, name, nn, K in u1rows:
        print(f"{S:>6.1f}{name:>10}{nn:>14.4f}{K:>8.3f}")
    return npass == len(RESULTS)


if __name__ == "__main__":
    ok = run_v0()
    raise SystemExit(0 if ok else 1)
