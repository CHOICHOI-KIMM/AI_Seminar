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


def tau_mean_eyring(eta, u_s, h, tau0):
    """평균 전단응력 tau_m — 식 (19) 머리말의 Eyring 유체 gamma_dot = (tau0/eta) sinh(tau/tau0)
    를 미끄럼 전단률 |u_s|/h 에 대해 푼 값. Newtonian(tau0 None) 또는 순수구름이면 0."""
    if tau0 is None or u_s == 0.0:
        return 0.0
    return tau0 * np.arcsinh(eta * abs(u_s) / (h * tau0))


# ------------------------------------------------------- 특수적분 (18),(10),(11)


def particular_integral(r_a, wx, wy, h, E_eff, B_bulk, u2, u_bar, eta_x, eta_y):
    """식 (18) -> p_a, 식 (10) -> v_a, 식 (11) -> h_a. kappa = 0 성분은 변형 없음.

    반환: dict(p_a, v_a, h_a, Q, C, kappa)  — p_a, v_a, h_a 는 복소수.
    """
    kappa = np.hypot(wx, wy)
    with np.errstate(divide="ignore", invalid="ignore"):
        denom = E_eff * h**3 * kappa * (wx**2 / eta_x + wy**2 / eta_y)
        Q = np.where(kappa > 0.0, 48.0 * (u2 - u_bar) * wx / denom, 0.0)
        C = h * E_eff * kappa / (4.0 * B_bulk)
        p_a = r_a * (kappa * E_eff / 4.0) * (1j * Q) / (1.0 - 1j * Q - 1j * C * Q)
        v_a = np.where(kappa > 0.0, 4.0 * p_a / (E_eff * kappa), 0.0)
    h_a = r_a + v_a
    return {k: np.asarray(v)[()] for k, v in
            dict(p_a=p_a, v_a=v_a, h_a=h_a, Q=Q, C=C, kappa=kappa).items()}


# ----------------------------------------------------------- 보완함수 (21)


_GRID_A = np.logspace(-4, 2, 90)       # omega_d / 척도 탐색 격자
_GRID_B = np.logspace(-14, 2, 160)     # alpha / 척도 탐색 격자


def _smallest_positive_root(f, grid):
    """f(x) (x: (N, M)) 의 양의 반직선 격자 위 첫 부호변화를 이분법으로 정밀화.
    반환: (근, 격자에서 찾은 근 개수)."""
    rows = np.arange(grid.shape[0])
    v = f(grid)
    change = (np.sign(v[:, :-1]) * np.sign(v[:, 1:])) < 0
    k = np.argmax(change, axis=1)
    lo, hi, flo = grid[rows, k], grid[rows, k + 1], v[rows, k]
    for _ in range(60):
        mid = 0.5 * (lo + hi)
        fm = f(mid[:, None])[:, 0]
        left = np.sign(fm) == np.sign(flo)
        lo, flo = np.where(left, mid, lo), np.where(left, fm, flo)
        hi = np.where(left, hi, mid)
    return 0.5 * (lo + hi), change.sum(axis=1)


def solve_psi(wx, wy, h, E_eff, B_bulk, u_bar, u2, eta_x, eta_y, tol=1e-12, itmax=200):
    """식 (21) 의 복소 파수 psi = omega_d + i alpha — 논문 풀이법(사용자 결정 U11, 2026-09-29).

    SKF 의 "실수부로 실수부를, 허수부로 허수부를 차례로 추정"을 원저자 Hooke [20]
    (식 19 아래) 기술대로 구현한다. 분모를 곱한 형태
        P(psi) = (psi - wc)(1 + K Om) - i A Om (psi^2/eta_x + wy^2/eta_y) = 0,
        Om = sqrt(psi^2 + wy^2),  A = E'h^3/(48 u_bar),  K = E'h/(4B),  wc = wx u2/u_bar
    의 실수부를 omega_d 에 대해, 허수부를 alpha 에 대해 번갈아 풀며 매번 양의 근을 택한다.
    wy = 0 이면 두 식이 Hooke 원문의 2차식과 같다. 2D 성분은 양의 반직선 구간탐색 +
    이분법으로 근을 구한다(Leeds 시험에서 전 성분 양의 근이 정확히 1개 — 작업내역서 §3.6).
    psi(-wx, wy) = -conj psi(wx, wy) 대칭으로 |wx| 만 푼다. wx = 0 성분은 대칭상 omega_d = 0.

    주의: 논문(정리본) 식 (21) 대괄호 첫 항은 psi^2/eta_y 로 표기되어 있으나
    식 (16),(18) 및 Hooke [21] 식 (29) 와의 정합상 psi^2/eta_x 가 맞다.

    스칼라·배열 모두 받는다. 반환: (psi, 반복횟수, 수렴여부)
    """
    wx_a, wy_a = np.broadcast_arrays(np.asarray(wx, float), np.asarray(wy, float))
    shape = wx_a.shape
    wx_f, wy_f = wx_a.ravel(), wy_a.ravel()
    sgn = np.where(wx_f < 0, -1.0, 1.0)
    wc_all = np.abs(wx_f) * u2 / u_bar
    live = np.where((wc_all > 0) | (wy_f != 0))[0]
    psi = np.zeros(wx_f.shape, complex)
    if live.size == 0:
        return psi.reshape(shape)[()], 0, True

    wc, wyl = wc_all[live][:, None], wy_f[live][:, None]
    A, K = E_eff * h**3 / (48.0 * u_bar), E_eff * h / (4.0 * B_bulk)

    def P(a, b):
        z = a + 1j * b
        om = np.sqrt(z**2 + wyl**2)
        return (z - wc) * (1.0 + K * om) - 1j * A * om * (z**2 / eta_x + wyl**2 / eta_y)

    scale = np.maximum(wc, np.abs(wyl))
    has_x = wc[:, 0] > 0
    a, b = wc[:, 0].copy(), np.zeros(len(live))
    ok = False
    for it in range(1, itmax + 1):
        a0, b0 = a.copy(), b.copy()
        ra, _ = _smallest_positive_root(lambda x: P(x, b[:, None]).real, scale * _GRID_A)
        a = np.where(has_x, ra, 0.0)
        b, _ = _smallest_positive_root(lambda x: P(a[:, None], x).imag, scale * _GRID_B)
        if np.all(np.abs(a - a0) + np.abs(b - b0) <= tol * (np.abs(a) + np.abs(b))):
            ok = True
            break
    psi[live] = sgn[live] * a + 1j * b
    return psi.reshape(shape)[()], it, ok


def solve_psi_newton(wx, wy, h, E_eff, B_bulk, u_bar, u2, eta_x, eta_y,
                     n_steps=60, n_newton=8, tol=1e-10):
    """식 (21) 을 뉴턴법 + 연속법으로 푼다 (solve_psi 와 같은 방정식, 다른 풀이법).

    전단박화로 eta_x 가 작아지면 짧은 파장에서 우변이 psi^3 로 커져 논문의 축차대입이
    발산한다. g(psi; t) = psi - wx' - i t N(psi)/D(psi) 에서 t 를 0 -> 1 로 올리며
    psi = wx' (t = 0) 에서 이어지는 근을 추적한다.
    반환: (psi, 최대 상대잔차 |g|/max(|psi|,1), 수렴여부)
    """
    wxp = np.asarray(wx * u2 / u_bar, dtype=complex)
    wy = np.asarray(wy, dtype=float)
    c_n, c_d = E_eff * h**3, E_eff * h / (4.0 * B_bulk)
    live = (np.abs(wxp) + np.abs(wy)) > 0.0              # DC 성분 제외

    def g_dg(psi, t):
        s = np.sqrt(psi**2 + wy**2)
        ds = np.where(live, psi / np.where(live, s, 1.0), 0.0)
        a = psi**2 / eta_x + wy**2 / eta_y
        N, dN = c_n * s * a, c_n * (ds * a + s * 2.0 * psi / eta_x)
        D, dD = 48.0 * u_bar * (1.0 + c_d * s), 48.0 * u_bar * c_d * ds
        g = psi - wxp - 1j * t * N / D
        dg = 1.0 - 1j * t * (dN * D - N * dD) / D**2
        return g, dg

    psi = wxp.copy()
    for t in np.linspace(0.0, 1.0, n_steps + 1)[1:]:
        for _ in range(n_newton):
            g, dg = g_dg(psi, t)
            psi = np.where(live, psi - g / dg, 0.0)
    g, _ = g_dg(psi, 1.0)
    res = np.max(np.where(live, np.abs(g) / np.maximum(np.abs(psi), 1.0), 0.0))
    return psi[()], res, bool(res <= tol)


# ------------------------------------------ 진폭감쇠 (22),(23),(24)

def ar_param_nn(ar_param, Q, S):
    """식 (23) K, 식 (22) nabla_nn — 논문 표기 그대로.

        K        = 1 - tanh(0.25 |Q| / nabla)
        nabla_nn = 0.8 nabla ((1 + S) / 2)^(0.1 + 0.5 K)

    밑수 (1+S)/2 는 S < -1 에서 음수가 되어 정의되지 않는다. 논문 그림의 조건은
    S = 0, +-1 뿐이므로 그 범위 밖 사용은 오류로 막는다.
    """
    if S < -1.0:
        raise ValueError(f"식 (22)는 S >= -1 에서만 정의된다 (S = {S})")
    K_nn = 1.0 - np.tanh(0.25 * abs(Q) / ar_param)
    base = (1.0 + S) / 2.0
    return 0.8 * ar_param * base ** (0.1 + 0.5 * K_nn), K_nn


def total_amplitude(r_a, ar_nn):
    """식 (24) h_t."""
    return r_a / (1.0 + 0.15 * ar_nn + 0.015 * ar_nn**2)


# ------------------------------------------------------------ 성분 조립


def ripple_component(r_a, wx, wy, h, case, S, hc_mode="complex", psi_solver="paper"):
    """거칠기 성분(스칼라 또는 스펙트럼 배열)에 대한 전유막 해. 계획서 §2.2 (a)~(d).

    tau_m 은 Eyring 식으로 미끄럼 속도에서 구한다(tau_mean_eyring).
    2D 성분의 진폭감쇠 변수는 [14] 식 [23][24] 를 따른다(계획서 U10):
        lam = min(lam_x, lam_y),  r = lam_x / lam_y,
        f(r) = exp(1 - 1/r) (r > 1), 1 (그 외),   nabla = f(r) * lam sqrt(M) / (a sqrt(L))
    hc_mode (계획서 U9): "complex"   h_cf = h_t - h_a
                         "magnitude" h_cf = (|h_t| - |h_a|) exp(i arg h_a)
    h_t, h_a, h_cf 는 원 거칠기 위상 기준 진폭이다.
    psi_solver: "paper"  논문 풀이법(solve_psi, Hooke 교대 풀이) — 기본값
                "newton" 뉴턴+연속법(solve_psi_newton) — 교차검증 전용(작업내역서 §3.6)

    반환 dict: p_a, h_a, h_cf, p_cf, psi, Q, C, ar_nn, K_nn, h_t, tau_m
    """
    eta = case.eta0 * np.exp(case.alpha * case.p_h)          # Barus
    tau_m = tau_mean_eyring(eta, S * case.u_bar, h, case.tau0)
    eta_x, eta_y = eta_equivalent(eta, tau_m, case.tau0)
    B_bulk = _bulk(case.p_h)

    u2 = case.u_bar * (1.0 + S / 2.0)                        # S = (u2-u1)/u_bar

    pi_ = particular_integral(r_a, wx, wy, h, case.E_eff, B_bulk,
                              u2, case.u_bar, eta_x, eta_y)
    solver = {"paper": solve_psi, "newton": solve_psi_newton}[psi_solver]
    psi, nit, ok = solver(wx, wy, h, case.E_eff, B_bulk, case.u_bar, u2, eta_x, eta_y)

    with np.errstate(divide="ignore", invalid="ignore"):
        lam_x = 2.0 * np.pi / np.abs(wx)
        lam_y = 2.0 * np.pi / np.abs(wy)
        r_xy = lam_x / lam_y
        f_r = np.where(np.isfinite(r_xy) & (r_xy > 1.0), np.exp(1.0 - 1.0 / r_xy), 1.0)
        ar_param = f_r * case.ar_param(np.minimum(lam_x, lam_y))
    with np.errstate(invalid="ignore"):
        ar_nn, K_nn = ar_param_nn(ar_param, pi_["Q"], S)
        h_t = total_amplitude(r_a, ar_nn)
    h_t = np.where(pi_["kappa"] > 0.0, h_t, 0.0)[()]        # DC: 0*inf 방지 (진폭 0)
    if hc_mode == "complex":
        h_cf = h_t - pi_["h_a"]
    elif hc_mode == "magnitude":
        h_cf = (np.abs(h_t) - np.abs(pi_["h_a"])) * np.exp(1j * np.angle(pi_["h_a"]))
    else:
        raise ValueError(hc_mode)
    kappa_c = np.sqrt(psi**2 + wy**2 + 0j)
    p_cf = case.E_eff * kappa_c * h_cf / 4.0                 # Hooke [21] 식 (28)

    return dict(p_a=pi_["p_a"], h_a=pi_["h_a"], Q=pi_["Q"], C=pi_["C"],
                psi=psi, psi_iters=nit, psi_ok=ok, tau_m=tau_m,
                ar_nn=ar_nn, K_nn=K_nn, h_t=h_t, h_cf=h_cf, p_cf=p_cf)


def ripple_line(r, L, case, S, h, x, hc_mode="complex", y0=0.0, psi_solver="paper",
                shifts=None):
    """정사각 주기 패치 r[iy, ix] (간극 섭동, 한 변 L) 의 y = y0 단면 Delta h(x), Delta p(x).

    x 는 접촉 중심 기준(입구 x = -a_x), 시각 t = 0 스냅숏.
    특수적분은 거친면과 함께 움직이고(exp(i wx x)), 보완파는 입구에서 정의되어
    exp(i psi x'), x' = x + a_x 로 전파·감쇠한다(Hooke [21] 식 (25)(26)).
    SKF 의 h_cf 는 원 거칠기 위상 기준이므로 입구 위상 exp(-i wx a_x) 를 곱해
    입구에서 전체 리플 = h_t 가 되게 한다.

    shifts: 거칠기 무늬 이동량 배열(스냅숏 시각에 해당). r(x - s) 는 성분 진폭에
    exp(-i wx s) 를 곱한 것과 같으므로 한 번의 해로 여러 스냅숏을 얻는다.
    None 이면 s = 0 한 개이고 dh, dp 는 1차원, 아니면 (len(shifts), len(x)).

    반환: (dh, dp, 허수부 최대 잔차, 성분 해 dict)
    """
    n = r.shape[0]
    R = np.fft.fft2(r) / r.size
    R[0, 0] = 0.0                                            # 평균 간극은 h 로 별도
    w = 2.0 * np.pi * np.fft.fftfreq(n, d=L / n)
    wy, wx = np.meshgrid(w, w, indexing="ij")

    sol = ripple_component(R, wx, wy, h, case, S, hc_mode, psi_solver)
    a = case.a_x
    inlet = np.exp(-1j * wx * a)
    hc_in, pc_in = sol["h_cf"] * inlet, sol["p_cf"] * inlet
    ey = np.exp(1j * wy * y0)
    s = np.atleast_1d(0.0 if shifts is None else shifts)
    ph = np.exp(-1j * np.outer(w, s))                       # (kx, 스냅숏)

    dh = np.empty((len(s), len(x)), dtype=complex)
    dp = np.empty((len(s), len(x)), dtype=complex)
    for i, xi in enumerate(x):
        ea = np.exp(1j * wx * xi) * ey
        ec = np.exp(1j * sol["psi"] * (xi + a)) * ey
        dh[:, i] = (sol["h_a"] * ea + hc_in * ec).sum(axis=0) @ ph   # ky 합 → kx 별
        dp[:, i] = (sol["p_a"] * ea + pc_in * ec).sum(axis=0) @ ph
    resid = max(np.max(np.abs(dh.imag)) / np.max(np.abs(dh.real)),
                np.max(np.abs(dp.imag)) / np.max(np.abs(dp.real)))
    if shifts is None:
        dh, dp = dh[0], dp[0]
    return dh.real, dp.real, resid, sol


def _bulk(p):
    from conditions import bulk_modulus
    return bulk_modulus(p)
