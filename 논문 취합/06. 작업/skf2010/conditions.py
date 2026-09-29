"""SKF(2010) 모델 — 공통 조건 계산 (계획서 S1).

Hertz 타원접촉 / Hamrock-Dowson 중앙유막 / Moes 파라미터 / Dowson-Higginson 체적탄성계수.
변수명은 작업계획서 §2.0 기호 대응표를 따른다.
"""
from __future__ import annotations

from dataclasses import dataclass

import numpy as np
from scipy.optimize import brentq

# ---------------------------------------------------------------- Hertz


def ellipticity(Rx: float, Ry: float) -> float:
    """타원율 k = 1.0339 (Ry/Rx)^0.636 (Hamrock-Dowson 근사).

    Rx == Ry 이면 정확값 1 을 반환한다 (근사식은 1.0339 를 준다).
    """
    if abs(Ry / Rx - 1.0) < 1e-9:
        return 1.0
    return 1.0339 * (Ry / Rx) ** 0.636


def complete_E(Rx: float, Ry: float) -> float:
    """제2종 완전타원적분 근사 Ee = 1.0003 + 0.5968 Rx/Ry.

    Rx == Ry 이면 정확값 pi/2 를 반환한다 (근사식은 1.5971 로 1.7 % 높다).
    k = 1, Ee = pi/2 를 넣으면 아래 타원 공식은 원형접촉 정확해로 환원된다.
    """
    if abs(Ry / Rx - 1.0) < 1e-9:
        return np.pi / 2.0
    return 1.0003 + 0.5968 * Rx / Ry


def hertz_from_load(F: float, Rx: float, Ry: float, E_eff: float):
    """하중 F -> (a_x 구름방향 반폭, a_y 횡방향 반폭, p_h).

    R = 1/(1/Rx + 1/Ry) 는 Hamrock 관례. 원형접촉에서는 ellipticity()/complete_E()
    가 정확값 (k = 1, Ee = pi/2) 을 돌려주므로 이 식이 Hertz 원형접촉 해로 환원된다.
    Hamrock-Dowson 근사식을 그대로 쓰면 Ry/Rx = 1 에서 k = 1.0339 가 되어
    Moes M 이 약 5 % 어긋난다.
    """
    k = ellipticity(Rx, Ry)
    Ee = complete_E(Rx, Ry)
    R = 1.0 / (1.0 / Rx + 1.0 / Ry)
    a_y = (6.0 * k**2 * Ee * F * R / (np.pi * E_eff)) ** (1.0 / 3.0)
    a_x = (6.0 * Ee * F * R / (np.pi * k * E_eff)) ** (1.0 / 3.0)
    p_h = 3.0 * F / (2.0 * np.pi * a_x * a_y)
    return a_x, a_y, p_h


def load_from_ph(p_h: float, Rx: float, Ry: float, E_eff: float) -> float:
    """최대 Hertz 압력 p_h 에 대응하는 하중 F 를 역산."""
    return brentq(lambda F: hertz_from_load(F, Rx, Ry, E_eff)[2] - p_h, 1e-6, 1e9)


# ------------------------------------------------------- 무차원 파라미터


def dimensionless(F, Rx, E_eff, eta0, u_bar, alpha):
    """U, G, W, Moes M, L."""
    U_dim = eta0 * u_bar / (E_eff * Rx)
    G_dim = alpha * E_eff
    W_dim = F / (E_eff * Rx**2)
    moes_M = W_dim * (2.0 * U_dim) ** (-0.75)
    moes_L = G_dim * (2.0 * U_dim) ** 0.25
    return U_dim, G_dim, W_dim, moes_M, moes_L


def h_central(F, Rx, Ry, E_eff, eta0, u_bar, alpha) -> float:
    """Hamrock-Dowson 중앙 유막두께 h_cen (m)."""
    k = ellipticity(Rx, Ry)
    U_dim, G_dim, W_dim, _, _ = dimensionless(F, Rx, E_eff, eta0, u_bar, alpha)
    Hc = (
        2.69
        * U_dim**0.67
        * G_dim**0.53
        * W_dim ** (-0.067)
        * (1.0 - 0.61 * np.exp(-0.73 * k))
    )
    return Hc * Rx


# ------------------------------------------------- 윤활유 물성 (점도/밀도)


def eta_barus(eta0: float, alpha: float, p: float) -> float:
    """Barus 점도-압력 관계 eta = eta0 exp(alpha p)."""
    return eta0 * np.exp(alpha * p)


def density_ratio(p: float) -> float:
    """Dowson-Higginson rho/rho0, p [Pa]."""
    pg = p / 1e9
    return (0.59 + 1.34 * pg) / (0.59 + pg)


def bulk_modulus(p: float) -> float:
    """B = rho (dp/drho), Dowson-Higginson 밀도법칙 해석 미분. 반환 단위 Pa.

    r(p) = (0.59 + 1.34 p) / (0.59 + p)          [p in GPa]
    dr/dp = 0.2006 / (0.59 + p)^2
    B = r / (dr/dp) = (0.59 + 1.34 p)(0.59 + p) / 0.2006   [GPa]
    """
    pg = p / 1e9
    return (0.59 + 1.34 * pg) * (0.59 + pg) / 0.2006 * 1e9


# ------------------------------------------------------------- 케이스 정의


@dataclass(frozen=True)
class Case:
    name: str
    E_eff: float        # Pa
    eta0: float         # Pa s
    alpha: float        # 1/Pa
    u_bar: float        # m/s
    Rx: float           # m
    Ry: float           # m
    p_h: float          # Pa
    tau0: float | None = None   # Pa (Eyring), None = Newtonian
    p_lim: float = 4.5e9        # Pa

    # -- 파생량 ---------------------------------------------------------
    @property
    def F(self) -> float:
        return load_from_ph(self.p_h, self.Rx, self.Ry, self.E_eff)

    @property
    def hertz(self):
        return hertz_from_load(self.F, self.Rx, self.Ry, self.E_eff)

    @property
    def a_x(self) -> float:
        return self.hertz[0]

    @property
    def a_y(self) -> float:
        return self.hertz[1]

    @property
    def dims(self):
        return dimensionless(self.F, self.Rx, self.E_eff, self.eta0,
                             self.u_bar, self.alpha)

    @property
    def h_cen(self) -> float:
        return h_central(self.F, self.Rx, self.Ry, self.E_eff, self.eta0,
                         self.u_bar, self.alpha)

    def at_speed(self, u_bar: float) -> "Case":
        return Case(self.name, self.E_eff, self.eta0, self.alpha, u_bar,
                    self.Rx, self.Ry, self.p_h, self.tau0, self.p_lim)

    def ar_param(self, lam: float) -> float:
        """Venner-Lubrecht 진폭감쇠 파라미터 nabla = lam sqrt(M) / (a_x sqrt(L))."""
        _, _, _, M, L = self.dims
        return lam * np.sqrt(M) / (self.a_x * np.sqrt(L))


# 계획서 §4.1 K1~K8: Deolalikar(Purdue 2008) 범프 예제
DEOLALIKAR = Case(
    name="Deolalikar bump (p_h = 0.63 GPa, pure rolling)",
    E_eff=219e9,
    eta0=0.075,
    alpha=16.7e-9,
    u_bar=0.15,
    Rx=1.0 / (1.0 / 0.02 + 1.0 / 0.12),      # = 17.143 mm
    Ry=1.0 / (1.0 / 0.02 + 1.0 / -0.023),    # = 153.33 mm
    p_h=0.63e9,
    tau0=None,
    p_lim=4.5e9,
)

# 계획서 §4.1 K11: Felix-Quinonez(Leeds 2005) flat-top 예제 (원형접촉 R = 12.7 mm)
LEEDS = Case(
    name="Leeds flat-top (p_h = 0.508 GPa)",
    E_eff=117e9,
    eta0=1.36,
    alpha=27.8e-9,
    u_bar=0.025,
    Rx=12.7e-3,
    Ry=12.7e-3,
    p_h=0.508e9,
    tau0=8e6,
)


# ------------------------------------------------------------ 자체 검사


def self_check(verbose: bool = True) -> list[tuple[str, float, float, float, bool]]:
    """계획서 §4.1 K2~K5, K11 과 대조. (항목, 계산값, 논문값, 상대오차, 합격)"""
    rows = []

    def chk(label, got, ref, tol):
        err = abs(got - ref) / abs(ref)
        rows.append((label, got, ref, err, err <= tol))

    c = DEOLALIKAR
    k = ellipticity(c.Rx, c.Ry)
    chk("K2 타원율 k", k, 4.193, 0.02)
    chk("K2 a_y [um]", c.a_y * 1e6, 795.0, 0.02)
    chk("K2 a_x [um]", c.a_x * 1e6, 189.0, 0.02)
    chk("K3 하중 F [N]", c.F, 197.0, 0.02)
    _, G_dim, _, M, L = c.dims
    chk("K5 G = alpha E'", G_dim, 3670.0, 0.02)
    chk("K5 Moes M", M, 799.1, 0.02)
    chk("K5 Moes L", L, 5.72, 0.02)
    chk("K4 h_cen [um]", c.h_cen * 1e6, 0.1543, 0.02)
    chk("K4 범프높이/h_cen", 1.2e-6 / c.h_cen, 8.0, 0.05)

    d = LEEDS
    chk("K11 a_x [um]", d.a_x * 1e6, 174.0, 0.03)
    chk("K11 h_cen [um]", d.h_cen * 1e6, 0.302, 0.15)
    _, _, _, Md, Ld = d.dims
    chk("K11 Moes M", Md, 97.58, 0.03)
    chk("K11 Moes L", Ld, 8.46, 0.03)

    chk("B(p=0) [GPa]", bulk_modulus(0.0) / 1e9, 1.74, 0.05)

    if verbose:
        print(f"{'항목':<22}{'계산':>12}{'논문':>12}{'상대오차':>10}  판정")
        for label, got, ref, err, ok in rows:
            print(f"{label:<22}{got:>12.4f}{ref:>12.4f}{err*100:>9.2f}%  "
                  f"{'PASS' if ok else 'FAIL'}")
    return rows


if __name__ == "__main__":
    self_check()
