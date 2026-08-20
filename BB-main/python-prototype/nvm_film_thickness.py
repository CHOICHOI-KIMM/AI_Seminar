"""
Nijenbanning-Venner-Moes (1994) EHL Film Thickness — 구현 및 검증
=================================================================
Unified formula for elliptic contacts covering all 4 EHL regimes.
Compared with Dowson-Higginson (M1) for TRB line contact conditions.

Reference: Nijenbanning G, Venner CH, Moes H.
"Film thickness in elastohydrodynamically lubricated elliptic contacts"
Wear 1994;176(2):217-229.
"""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')

import numpy as np
from pathlib import Path

OUT_DIR = Path(__file__).parent / "van_zoelen_results"
OUT_DIR.mkdir(exist_ok=True)


# ═══════════════════════════════════════════════════════════════════
# NVM (1994) Implementation
# ═══════════════════════════════════════════════════════════════════

def nvm_central_film(F, Rx, Ry, E_prime, eta_0, u_s, alpha):
    """
    Nijenbanning-Venner-Moes unified central film thickness.

    Args:
        F:       Contact load [N]
        Rx:      Reduced radius in rolling direction [m]
        Ry:      Reduced radius transverse to rolling [m] (Ry >> Rx for line contact)
        E_prime: Combined elastic modulus [Pa]: 2/E' = (1-v1^2)/E1 + (1-v2^2)/E2
        eta_0:   Dynamic viscosity at ambient pressure [Pa·s]
        u_s:     Sum velocity = u1 + u2 [m/s] (NOT mean velocity)
        alpha:   Pressure-viscosity coefficient [1/Pa]

    Returns:
        h_c: Central film thickness [m]
    """
    if F <= 0 or Rx <= 0 or E_prime <= 0 or eta_0 <= 0 or u_s <= 0:
        return 0.0

    # Moes dimensionless parameters
    M = (F / (E_prime * Rx**2)) * (eta_0 * u_s / (E_prime * Rx))**(-3/4)
    L = alpha * E_prime * (eta_0 * u_s / (E_prime * Rx))**(1/4)
    D = Rx / Ry  # curvature ratio (D → 0 for line contact)

    # Clamp D for numerical stability
    D = max(D, 1e-6)

    # Four asymptotic solutions (Eqs. 5-8)
    # RI: Rigid-Isoviscous
    C_RI = 145.0 * (1.0 + 0.796 * D**(14/15))**(-15/7) * D**(-1)
    h_RI = C_RI * M**(-2)

    # EI: Elastic-Isoviscous
    C_EI = 3.18 * (1.0 + 0.006 * np.log(D) + 0.63 * D**(4/7))**(-14/25) * D**(-1/15)
    h_EI = C_EI * M**(-2/15)

    # RP: Rigid-Piezoviscous
    C_RP = 1.29 * (1.0 + 0.691 * D)**(-2/3)
    h_RP = C_RP * L**(2/3)

    # EP: Elastic-Piezoviscous (= Dowson-Higginson regime)
    C_EP = 1.48 * (1.0 + 0.006 * np.log(D) + 0.63 * D**(4/7))**(-7/20) * D**(-1/24)
    h_EP = C_EP * M**(-1/12) * L**(3/4)

    # Unified blending (Eq. 9-10)
    s = 1.5 * (1.0 + np.exp(-1.2 * h_EI / h_RI))
    h_00 = 1.8 * D**(-1)

    # Isoviscous branch
    ei_bridge = (h_EI**(-4) + h_00**(-4))**(-3*s/8)
    iso_branch = (h_RI**(3*s/2) + ei_bridge)**(2*s/3)  # Wait, this should be (2/(3s))

    # Eq.(9): h = { [h_RI^(3s/2) + (h_EI^-4 + h_00^-4)^(-3s/8)]^(2/(3s))
    #               + (h_RP^-8 + h_EP^-8)^(-s/8) }^(1/s)
    iso_branch = (h_RI**(3*s/2) + ei_bridge)**(2/(3*s))

    # Piezoviscous branch
    piezo_branch = (h_RP**(-8) + h_EP**(-8))**(-s/8)

    # Combined: (iso + piezo)^(1/s)  ← NOT (iso^s + piezo^s)^(1/s)
    h_bar = (iso_branch + piezo_branch)**(1/s)

    # Recover dimensional film thickness
    # h_bar = h / (Rx * sqrt(eta_0 * u_s / (E' * Rx)))
    h_scale = Rx * np.sqrt(eta_0 * u_s / (E_prime * Rx))
    h_c = h_bar * h_scale

    return h_c, {
        'M': M, 'L': L, 'D': D,
        'h_RI': h_RI, 'h_EI': h_EI, 'h_RP': h_RP, 'h_EP': h_EP,
        's': s, 'h_bar': h_bar, 'h_scale': h_scale,
    }


def dowson_higginson_central(q, Rx, E_prime, eta_0, u_m, alpha):
    """
    Dowson-Toyoda (1978) central film thickness for line contact.

    Args:
        q:       Load per unit length [N/m]
        Rx:      Reduced radius [m]
        E_prime: Combined elastic modulus [Pa]
        eta_0:   Dynamic viscosity [Pa·s]
        u_m:     Mean entraining velocity [m/s]
        alpha:   Pressure-viscosity coefficient [1/Pa]

    Returns:
        h_c: Central film thickness [m]
    """
    U = eta_0 * u_m / (E_prime * Rx)
    G = alpha * E_prime
    W = q / (E_prime * Rx)

    H_c = 3.06 * U**0.69 * G**0.56 * W**(-0.10)
    return H_c * Rx


# ═══════════════════════════════════════════════════════════════════
# Verification: NVM vs DH for TRB conditions
# ═══════════════════════════════════════════════════════════════════

if __name__ == "__main__":
    print("=" * 70)
    print("  NVM (1994) vs Dowson-Higginson (1977) — TRB Line Contact")
    print("=" * 70)

    # Material
    E_roller = 206e9  # Pa
    E_ring = 206e9
    nu = 0.3
    E_prime = 2.0 / ((1 - nu**2)/E_roller + (1 - nu**2)/E_ring)

    # Lubricant (ISO VG 68 mineral oil at 70°C)
    # ν₄₀=68, ν₁₀₀=8 → at 70°C ≈ 15 mm²/s
    nu_op = 15.0  # mm²/s at 70°C
    rho_oil = 870.0  # kg/m³
    eta_0 = nu_op * 1e-6 * rho_oil  # Pa·s
    alpha = 20e-9  # 1/Pa (= 20 GPa⁻¹)

    print(f"\n  Material: E' = {E_prime/1e9:.1f} GPa")
    print(f"  Lubricant: η₀ = {eta_0:.4f} Pa·s, α = {alpha*1e9:.0f} GPa⁻¹")

    # TRB geometry: d_pw=150, D_we=13.75, L_we=22, α=12°, Z=20
    d_pw = 150e-3    # m
    D_we = 13.75e-3  # m (mean roller diameter)
    L_we = 22e-3     # m
    alpha_deg = 12.0
    Z = 20

    # Equivalent radii for TRB roller-inner raceway
    R_roller = D_we / 2  # = 6.875 mm
    alpha_rad = np.radians(alpha_deg)
    # Inner raceway: convex roller on concave raceway (line contact)
    R_inner_race = 100e-3  # approximate inner race radius [m]
    Rx = 1.0 / (1.0/R_roller + 1.0/R_inner_race) if R_inner_race > 0 else R_roller
    Ry_line = 1e6  # very large for line contact (effectively infinite)

    print(f"\n  Geometry: Rx = {Rx*1e3:.2f} mm, Ry = ∞ (line contact)")
    print(f"  D = Rx/Ry = {Rx/Ry_line:.2e}")

    # ─── Speed sweep ───
    print(f"\n{'─'*70}")
    print(f"  Speed Sweep (q = 50 N/mm)")
    print(f"{'─'*70}")

    q_per_mm = 50.0  # N/mm → typical max-loaded slice
    q_per_m = q_per_mm * 1e3  # N/m

    print(f"\n  {'n [rpm]':>8}  {'u_m [m/s]':>10}  {'DH h_c [μm]':>12}  {'NVM h_c [μm]':>13}  {'NVM/DH':>8}  {'M':>10}  {'L':>6}")
    print(f"  {'─'*75}")

    for n_rpm in [500, 1000, 1500, 2000, 3000, 5000]:
        omega = n_rpm * 2 * np.pi / 60
        # Simplified: u_m ≈ ω × R_pw × (1-γ²)/2 where γ = D_we/(d_pw)
        gamma = D_we * np.cos(alpha_rad) / d_pw
        u_m = omega * d_pw/2 * (1 - gamma**2) / 2  # m/s
        u_s = 2 * u_m  # sum velocity for NVM

        # DH (line contact)
        h_dh = dowson_higginson_central(q_per_m, Rx, E_prime, eta_0, u_m, alpha)

        # NVM (elliptic → line contact limit)
        # For line contact: F = q × L_we, Ry → ∞
        F_total = q_per_m * L_we  # total load on roller
        h_nvm, params = nvm_central_film(F_total, Rx, Ry_line, E_prime, eta_0, u_s, alpha)

        ratio = h_nvm / h_dh if h_dh > 0 else 0

        print(f"  {n_rpm:>8}  {u_m:>10.4f}  {h_dh*1e6:>12.3f}  {h_nvm*1e6:>13.3f}  {ratio:>8.3f}  "
              f"{params['M']:>10.1f}  {params['L']:>6.1f}")

    # ─── Load sweep ───
    print(f"\n{'─'*70}")
    print(f"  Load Sweep (n = 1500 rpm)")
    print(f"{'─'*70}")

    n_rpm = 1500
    omega = n_rpm * 2 * np.pi / 60
    gamma = D_we * np.cos(alpha_rad) / d_pw
    u_m = omega * d_pw/2 * (1 - gamma**2) / 2
    u_s = 2 * u_m

    print(f"\n  {'q [N/mm]':>9}  {'DH h_c [μm]':>12}  {'NVM h_c [μm]':>13}  {'NVM/DH':>8}")
    print(f"  {'─'*50}")

    for q_per_mm in [10, 20, 50, 100, 200, 500]:
        q_per_m = q_per_mm * 1e3
        h_dh = dowson_higginson_central(q_per_m, Rx, E_prime, eta_0, u_m, alpha)
        F_total = q_per_m * L_we
        h_nvm, _ = nvm_central_film(F_total, Rx, Ry_line, E_prime, eta_0, u_s, alpha)
        ratio = h_nvm / h_dh if h_dh > 0 else 0
        print(f"  {q_per_mm:>9}  {h_dh*1e6:>12.3f}  {h_nvm*1e6:>13.3f}  {ratio:>8.3f}")

    # ─── Regime analysis ───
    print(f"\n{'─'*70}")
    print(f"  Regime Analysis (which asymptote dominates?)")
    print(f"{'─'*70}")

    q_per_m = 50e3
    F_total = q_per_m * L_we
    _, p = nvm_central_film(F_total, Rx, Ry_line, E_prime, eta_0, u_s, alpha)

    print(f"\n  M = {p['M']:.1f}, L = {p['L']:.1f}, D = {p['D']:.2e}")
    print(f"  h_RI  = {p['h_RI']:.4e}  (Rigid-Isoviscous)")
    print(f"  h_EI  = {p['h_EI']:.4e}  (Elastic-Isoviscous)")
    print(f"  h_RP  = {p['h_RP']:.4e}  (Rigid-Piezoviscous)")
    print(f"  h_EP  = {p['h_EP']:.4e}  (Elastic-Piezoviscous) ← DH regime")
    print(f"  s     = {p['s']:.3f}")
    print(f"  h_bar = {p['h_bar']:.4e}  (unified)")

    dominant = max([('RI', p['h_RI']), ('EI', p['h_EI']),
                    ('RP', p['h_RP']), ('EP', p['h_EP'])], key=lambda x: x[1])
    print(f"\n  Dominant regime: {dominant[0]} (h = {dominant[1]:.4e})")
    print(f"  → TRB는 EP(Piezoviscous-Elastic) 영역에서 작동 = DH와 동일 영역")

    # ─── Paper validation: circular contact ───
    print(f"\n{'─'*70}")
    print(f"  Paper Validation: Fig. 3 조건 (PCS EHD ball-on-disc)")
    print(f"{'─'*70}")

    # Ball: R = 9.53 mm, E' for steel-glass
    E_steel = 206e9; nu_steel = 0.3
    E_glass = 63e9; nu_glass = 0.2
    E_prime_bg = 2.0 / ((1-nu_steel**2)/E_steel + (1-nu_glass**2)/E_glass)
    R_ball = 9.53e-3
    Rx_ball = R_ball  # ball on flat
    Ry_ball = R_ball  # circular contact

    # Li/M grease
    eta_0_lim = 0.38
    alpha_lim = 30e-9
    F_ball = 20.0  # N

    print(f"  E' = {E_prime_bg/1e9:.1f} GPa, R = {R_ball*1e3:.2f} mm")
    print(f"  η₀ = {eta_0_lim} Pa·s, α = {alpha_lim*1e9:.0f} GPa⁻¹")

    for u_m_test in [0.100, 0.150, 0.200, 0.300]:
        u_s_test = 2 * u_m_test
        h_nvm_ball, p_ball = nvm_central_film(F_ball, Rx_ball, Ry_ball, E_prime_bg, eta_0_lim, u_s_test, alpha_lim)

        # Table 2 h_cff values from paper
        h_cff_paper = {0.100: 353, 0.150: 493, 0.200: 500, 0.300: 655}

        paper_nm = h_cff_paper.get(u_m_test, 0)
        nvm_nm = h_nvm_ball * 1e9

        diff_pct = (nvm_nm - paper_nm) / paper_nm * 100 if paper_nm > 0 else 0

        print(f"  u_m = {u_m_test*1e3:.0f} mm/s: NVM = {nvm_nm:.0f} nm, "
              f"Paper h_cff = {paper_nm:.0f} nm, diff = {diff_pct:+.1f}%"
              f"  (M={p_ball['M']:.1f}, L={p_ball['L']:.1f})")

    print(f"\n{'='*70}")
    print(f"  완료")
    print(f"{'='*70}")
