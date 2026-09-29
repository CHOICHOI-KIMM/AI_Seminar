"""SKF(2010) 모델 M2 — 전유막 리플 모델 (계획서 §2.2).

주파수 성분 하나에 대한 해석해. 모든 함수는 스칼라 또는 배열에 동작한다.
변수명은 작업계획서 §2.0 기호 대응표를 따른다.
"""
from __future__ import annotations

import numpy as np

# ----------------------------------------------------- Eyring 등가점도 (19)


def eta_equivalent(eta, tau_m, tau0):
    """식 (19). tau0 is None 또는 tau_m/tau0 -> 0 이면 Newtonian (eta, eta)."""
    if tau0 is None or tau_m == 0.0:
        return eta, eta
    x = tau_m / tau0
    if abs(x) < 1e-8:                       # sinh(x)/x -> 1, cosh(x) -> 1
        return eta, eta
    return eta / np.cosh(x), eta * x / np.sinh(x)


# ------------------------------------------------------- 특수적분 (18),(10),(11)


def particular_integral(r_a, wx, wy, h, E_eff, B_bulk, u2, u_bar, eta_x, eta_y):
    """식 (18) -> p_a, 식 (10) -> v_a, 식 (11) -> h_a.

    반환: dict(p_a, v_a, h_a, Q, C, kappa)  — p_a, v_a, h_a 는 복소수.
    """
    kappa = np.hypot(wx, wy)
    if kappa == 0.0:
        return dict(p_a=0j, v_a=0j, h_a=complex(r_a), Q=0.0, C=0.0, kappa=0.0)

    denom = E_eff * h**3 * kappa * (wx**2 / eta_x + wy**2 / eta_y)
    Q = 48.0 * (u2 - u_bar) * wx / denom
    C = h * E_eff * kappa / (4.0 * B_bulk)

    p_a = r_a * (kappa * E_eff / 4.0) * (1j * Q) / (1.0 - 1j * Q - 1j * C * Q)
    v_a = 4.0 * p_a / (E_eff * kappa)
    h_a = r_a + v_a
    return dict(p_a=p_a, v_a=v_a, h_a=h_a, Q=Q, C=C, kappa=kappa)


# ----------------------------------------------------------- 보완함수 (21)


def solve_psi(wx, wy, h, E_eff, B_bulk, u_bar, u2, eta_x, eta_y,
              relax=0.5, tol=1e-12, itmax=2000):
    """식 (21) 의 복소 파수 psi 를 축차대입으로 푼다.

    주의: 논문(정리본) 식 (21) 대괄호 첫 항은 psi^2/eta_y 로 표기되어 있으나
    식 (16),(18) 과의 정합상 psi^2/eta_x 가 맞다 (계획서 §2.2(b) 경고).

    반환: (psi, 반복횟수, 수렴여부)
    """
    wxp = wx * u2 / u_bar                      # omega_x' = wx u2 / u_bar
    psi = complex(wxp, 0.0)
    if wxp == 0.0 and wy == 0.0:
        return 0j, 0, True

    for it in range(1, itmax + 1):
        s = np.sqrt(psi**2 + wy**2 + 0j)
        num = E_eff * h**3 * s * (psi**2 / eta_x + wy**2 / eta_y)
        den = 48.0 * u_bar * (1.0 + E_eff * h * s / (4.0 * B_bulk))
        target = wxp + 1j * num / den
        step = target - psi
        psi = psi + relax * step
        if abs(step) <= tol * max(abs(psi), 1.0):
            return psi, it, True
    return psi, itmax, False


# ------------------------------------------ 진폭감쇠 (22),(23),(24)

#: 계획서 §4.2 U1 — 식 (22) 밑수의 3개 후보
U1_VARIANTS = {
    "A_paper": lambda S: (1.0 + S) / 2.0,        # 논문 표기 그대로
    "B_half": lambda S: 1.0 + S / 2.0,           # = u2/u_bar (OCR 오독 가설)
    "C_abs": lambda S: (1.0 + abs(S)) / 2.0,     # 절대값 가설
}


def ar_param_nn(ar_param, Q, S, variant="B_half"):
    """식 (23) K, 식 (22) nabla_nn. 밑수가 음수면 NaN 을 돌려준다."""
    K_nn = 1.0 - np.tanh(0.25 * abs(Q) / ar_param)
    base = U1_VARIANTS[variant](S)
    if base < 0.0:
        return float("nan"), K_nn
    return 0.8 * ar_param * base ** (0.1 + 0.5 * K_nn), K_nn


def total_amplitude(r_a, ar_nn):
    """식 (24) h_t."""
    return r_a / (1.0 + 0.15 * ar_nn + 0.015 * ar_nn**2)


# ------------------------------------------------------------ 성분 조립


def ripple_component(r_a, lam_x, lam_y, h, case, S, ar_param,
                     tau_m=0.0, variant="B_half"):
    """거칠기 성분 하나에 대한 전유막 해. 계획서 §2.2 (a)~(d).

    반환 dict: p_a, h_a, h_cf, p_cf, psi, Q, C, ar_nn, K_nn, h_t
    """
    wx = 2.0 * np.pi / lam_x if lam_x else 0.0
    wy = 2.0 * np.pi / lam_y if lam_y else 0.0

    eta = case.eta0 * np.exp(case.alpha * case.p_h)          # Barus
    eta_x, eta_y = eta_equivalent(eta, tau_m, case.tau0)
    B_bulk = _bulk(case.p_h)

    u2 = case.u_bar * (1.0 + S / 2.0)                        # S = (u2-u1)/u_bar

    pi_ = particular_integral(r_a, wx, wy, h, case.E_eff, B_bulk,
                              u2, case.u_bar, eta_x, eta_y)
    psi, nit, ok = solve_psi(wx, wy, h, case.E_eff, B_bulk, case.u_bar, u2,
                             eta_x, eta_y)

    ar_nn, K_nn = ar_param_nn(ar_param, pi_["Q"], S, variant)
    h_t = total_amplitude(r_a, ar_nn)
    h_cf = h_t - pi_["h_a"]                                  # 계획서 §2.2(c)
    kappa_c = np.sqrt(psi**2 + wy**2 + 0j)
    p_cf = case.E_eff * kappa_c * h_cf / 4.0                 # 식 (10) 역관계

    return dict(p_a=pi_["p_a"], h_a=pi_["h_a"], Q=pi_["Q"], C=pi_["C"],
                psi=psi, psi_iters=nit, psi_ok=ok,
                ar_nn=ar_nn, K_nn=K_nn, h_t=h_t, h_cf=h_cf, p_cf=p_cf)


def _bulk(p):
    from conditions import bulk_modulus
    return bulk_modulus(p)
