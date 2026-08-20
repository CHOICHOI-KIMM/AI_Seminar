"""
Eq. 24 (Full Integral) vs Eq. 25 (Approximate) 비교
=====================================================
2012 Venner/Lugt 논문의 두 수식을 모두 구현하고:
1. F(0) 값 비교
2. 막두께 감쇠 곡선 비교
3. 계산 속도 비교
4. Fig. 3 실험 데이터와 비교
"""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8', errors='replace')

import numpy as np
from scipy.integrate import quad
import matplotlib.pyplot as plt
import time as timer
from pathlib import Path

OUT_DIR = Path(__file__).parent / "van_zoelen_results"
OUT_DIR.mkdir(exist_ok=True)


# ═══════════════════════════════════════════════════════════════════
# 물성 모델
# ═══════════════════════════════════════════════════════════════════

P_R = 1.96e8  # Roelands 기준 압력 [Pa]


def rho_DH(p, rho_0):
    """Dowson-Higginson 밀도 (Eq. 16)."""
    return rho_0 * (5.9e8 + 1.34 * p) / (5.9e8 + p)


def eta_roelands(p, eta_0, z):
    """Roelands 점도 (Eq. 17).
    η(p) = η₀ exp((ln(η₀)+9.67)(-1 + (1+p/p_r)^z))
    """
    S = np.log(eta_0) + 9.67
    return eta_0 * np.exp(S * (-1.0 + (1.0 + p / P_R)**z))


def eta_barus(p, eta_0, alpha):
    """Barus 점도 (지수 모델). η(p) = η₀ exp(αp)."""
    return eta_0 * np.exp(alpha * p)


def alpha_to_z(alpha, eta_0):
    """Barus α → Roelands z 변환.
    저압에서 두 모델 일치 조건: α = z(ln(η₀)+9.67)/p_r
    """
    S = np.log(eta_0) + 9.67
    return alpha * P_R / S


def z_to_alpha(z, eta_0):
    """Roelands z → Barus α 변환."""
    S = np.log(eta_0) + 9.67
    return z * S / P_R


# ═══════════════════════════════════════════════════════════════════
# Eq. 24: Full Integral (수치 적분)
# ═══════════════════════════════════════════════════════════════════

def F_k_eq24_roelands(p_h, a, b, eta_0, z, rho_0, l_t, y_over_b=0.0):
    """Eq. 24 with Roelands viscosity + Dowson-Higginson density.

    F_k(y) = (2 ρ₀² p_h²)/(l_t b²) ∫_{a⁻}^{a⁺} (η(p)⁻¹ ρ(p)⁻² p⁻¹) dx

    At y=0: p(x) = p_h √(1-(x/a)²), a⁺=-a⁻=a
    Substitution x = a sinθ → p = p_h cosθ, dx = a cosθ dθ
    """
    def integrand(theta):
        p = p_h * np.cos(theta) * np.sqrt(1.0 - y_over_b**2)
        if p < 1e-3:  # avoid p=0 singularity
            return 0.0
        eta = eta_roelands(p, eta_0, z)
        rho = rho_DH(p, rho_0)
        # original integrand: 1/(η ρ² p) × dx
        # with substitution: 1/(η ρ² p) × a cosθ dθ
        # but p = p_h cosθ √(1-y²/b²), dx = a cosθ dθ
        # so the p⁻¹ × a cosθ = a cosθ / (p_h cosθ √(1-y²/b²)) = a / (p_h √(1-y²/b²))
        # This simplification only works at y=0
        return 1.0 / (eta * rho**2 * p) * a * np.cos(theta)

    # y=0인 경우 적분 범위: -π/2 ~ π/2
    val, _ = quad(integrand, -np.pi/2, np.pi/2, limit=200)

    return (2.0 * rho_0**2 * p_h**2) / (l_t * b**2) * val


def F_k_eq24_barus(p_h, a, b, eta_0, alpha, rho_0, l_t, y_over_b=0.0):
    """Eq. 24 with Barus viscosity + Dowson-Higginson density."""
    def integrand(theta):
        p = p_h * np.cos(theta) * np.sqrt(1.0 - y_over_b**2)
        if p < 1e-3:
            return 0.0
        eta = eta_barus(p, eta_0, alpha)
        rho = rho_DH(p, rho_0)
        return 1.0 / (eta * rho**2 * p) * a * np.cos(theta)

    val, _ = quad(integrand, -np.pi/2, np.pi/2, limit=200)
    return (2.0 * rho_0**2 * p_h**2) / (l_t * b**2) * val


def F_k_eq24_incompressible_roelands(p_h, a, b, eta_0, z, rho_0, l_t):
    """Eq. 24, Roelands viscosity, 비압축성 (ρ=ρ₀ 고정)."""
    def integrand(theta):
        p = p_h * np.cos(theta)
        if p < 1e-3:
            return 0.0
        eta = eta_roelands(p, eta_0, z)
        return 1.0 / (eta * rho_0**2 * p) * a * np.cos(theta)

    val, _ = quad(integrand, -np.pi/2, np.pi/2, limit=200)
    return (2.0 * rho_0**2 * p_h**2) / (l_t * b**2) * val


# ═══════════════════════════════════════════════════════════════════
# Eq. 25: Approximate (해석적)
# ═══════════════════════════════════════════════════════════════════

def F_k_eq25(p_h, a, b, eta_0, alpha, l_t, y_over_b=0.0):
    """Eq. 25 (closed-form approximation).
    F_k(y) ≈ (2/l_t)(p_h/b²)(a/η₀)π ((0.5πα p_h √(1-y²/b²))^{3/2} + 1)^{-2/3}
    """
    p_eff = p_h * np.sqrt(1.0 - y_over_b**2)
    term = 0.5 * np.pi * alpha * p_eff
    visc_corr = (term**1.5 + 1.0) ** (-2.0 / 3.0)
    return (2.0 / l_t) * (p_h / b**2) * (a / eta_0) * np.pi * visc_corr


# ═══════════════════════════════════════════════════════════════════
# 막두께 감쇠 공통 함수
# ═══════════════════════════════════════════════════════════════════

def film_decay(t, h_c0, F0, rho_0, p_h):
    """Eq. 27: h_c(t) = (1/6 ρ̄² F(0) t + h_{c,0}^{-2})^{-1/2}"""
    rho_bar = rho_DH(p_h, rho_0) / rho_0
    C = (1.0 / 6.0) * rho_bar**2 * F0
    return (C * t + h_c0**(-2)) ** (-0.5)


# ═══════════════════════════════════════════════════════════════════
# 테스트 조건 (Fig. 3: PCS EHD ball-on-disc, Li/M)
# ═══════════════════════════════════════════════════════════════════

p_h     = 0.48e9
a       = 141e-6
b       = 141e-6
eta_0   = 0.38
alpha   = 30e-9
rho_0   = 891.0
R_ball  = 9.53e-3
R_track = 0.040
l_t     = 2*np.pi*R_track + 2*np.pi*R_ball
z       = alpha_to_z(alpha, eta_0)
h_c0    = 110e-9

# Wide elliptical 조건도 비교 (Fig. 6)
p_h_w   = 0.22e9
a_w     = 39.2e-6
b_w     = 2710e-6
l_t_w   = 2*np.pi*0.045 + 2*np.pi*3.91e-3
z_w     = z  # 같은 그리스


# ═══════════════════════════════════════════════════════════════════
# F(0) 비교
# ═══════════════════════════════════════════════════════════════════

print("=" * 70)
print("  Eq. 24 (Full Integral) vs Eq. 25 (Approximate) 비교")
print("=" * 70)

print(f"\n  ── Roelands 파라미터 ──")
print(f"  α (Barus)     = {alpha*1e9:.1f} GPa⁻¹")
print(f"  z (Roelands)  = {z:.4f}")
print(f"  η₀            = {eta_0:.3f} Pa·s")
print(f"  ln(η₀)+9.67   = {np.log(eta_0)+9.67:.4f}")
print(f"  α 검증: z→α   = {z_to_alpha(z, eta_0)*1e9:.1f} GPa⁻¹")

# 점도 프로파일 비교
print(f"\n  ── 점도 모델 비교 (η/η₀) ──")
print(f"  {'p [GPa]':>8}  {'Roelands':>10}  {'Barus':>10}  {'비율':>8}")
print(f"  {'─'*40}")
for p_test in [0.01e9, 0.05e9, 0.1e9, 0.2e9, 0.3e9, 0.48e9]:
    eta_r = eta_roelands(p_test, eta_0, z) / eta_0
    eta_b = eta_barus(p_test, eta_0, alpha) / eta_0
    print(f"  {p_test/1e9:>8.2f}  {eta_r:>10.2f}  {eta_b:>10.2f}  {eta_b/eta_r:>8.2f}")


# ── Case 1: Circular contact (Fig. 3) ──
print(f"\n{'─'*70}")
print(f"  Case 1: Circular contact (k=1), p_h={p_h/1e9} GPa")
print(f"{'─'*70}")

F_eq24_roel = F_k_eq24_roelands(p_h, a, b, eta_0, z, rho_0, l_t)
F_eq24_bar  = F_k_eq24_barus(p_h, a, b, eta_0, alpha, rho_0, l_t)
F_eq24_inc  = F_k_eq24_incompressible_roelands(p_h, a, b, eta_0, z, rho_0, l_t)
F_eq25      = F_k_eq25(p_h, a, b, eta_0, alpha, l_t)

print(f"\n  {'방법':>30}  {'F(0)':>14}  {'비율(vs Eq25)':>14}")
print(f"  {'─'*60}")
print(f"  {'Eq.25 (근사, Barus α)':>30}  {F_eq25:>14.4e}  {F_eq25/F_eq25:>14.2f}")
print(f"  {'Eq.24 + Barus η':>30}  {F_eq24_bar:>14.4e}  {F_eq24_bar/F_eq25:>14.2f}")
print(f"  {'Eq.24 + Roelands η':>30}  {F_eq24_roel:>14.4e}  {F_eq24_roel/F_eq25:>14.2f}")
print(f"  {'Eq.24 + Roelands + 비압축':>30}  {F_eq24_inc:>14.4e}  {F_eq24_inc/F_eq25:>14.2f}")


# ── Case 2: Wide elliptical contact (Fig. 6) ──
print(f"\n{'─'*70}")
print(f"  Case 2: Wide elliptical (k≈69), p_h={p_h_w/1e9} GPa")
print(f"{'─'*70}")

F_eq24_roel_w = F_k_eq24_roelands(p_h_w, a_w, b_w, eta_0, z_w, rho_0, l_t_w)
F_eq24_bar_w  = F_k_eq24_barus(p_h_w, a_w, b_w, eta_0, alpha, rho_0, l_t_w)
F_eq25_w      = F_k_eq25(p_h_w, a_w, b_w, eta_0, alpha, l_t_w)

print(f"\n  {'방법':>30}  {'F(0)':>14}  {'비율(vs Eq25)':>14}")
print(f"  {'─'*60}")
print(f"  {'Eq.25 (근사, Barus α)':>30}  {F_eq25_w:>14.4e}  {F_eq25_w/F_eq25_w:>14.2f}")
print(f"  {'Eq.24 + Barus η':>30}  {F_eq24_bar_w:>14.4e}  {F_eq24_bar_w/F_eq25_w:>14.2f}")
print(f"  {'Eq.24 + Roelands η':>30}  {F_eq24_roel_w:>14.4e}  {F_eq24_roel_w/F_eq25_w:>14.2f}")


# ═══════════════════════════════════════════════════════════════════
# 계산 속도 비교
# ═══════════════════════════════════════════════════════════════════

print(f"\n{'─'*70}")
print(f"  계산 속도 비교 (10,000회 반복)")
print(f"{'─'*70}")

N_iter = 10000

t0 = timer.perf_counter()
for _ in range(N_iter):
    F_k_eq25(p_h, a, b, eta_0, alpha, l_t)
t_eq25 = timer.perf_counter() - t0

t0 = timer.perf_counter()
for _ in range(N_iter):
    F_k_eq24_roelands(p_h, a, b, eta_0, z, rho_0, l_t)
t_eq24_r = timer.perf_counter() - t0

t0 = timer.perf_counter()
for _ in range(N_iter):
    F_k_eq24_barus(p_h, a, b, eta_0, alpha, rho_0, l_t)
t_eq24_b = timer.perf_counter() - t0

print(f"  Eq.25 (해석적):         {t_eq25*1e3:.1f} ms  ({t_eq25/N_iter*1e6:.1f} μs/call)")
print(f"  Eq.24 + Barus:          {t_eq24_b*1e3:.1f} ms  ({t_eq24_b/N_iter*1e6:.1f} μs/call)")
print(f"  Eq.24 + Roelands:       {t_eq24_r*1e3:.1f} ms  ({t_eq24_r/N_iter*1e6:.1f} μs/call)")
print(f"  속도 비율 (Eq24R/Eq25): {t_eq24_r/t_eq25:.0f}×")


# ═══════════════════════════════════════════════════════════════════
# Fig. 3 막두께 감쇠 비교
# ═══════════════════════════════════════════════════════════════════

# 실험 데이터 (200 mm/s)
t_200 = np.array([
    8,  15,  22,  30,  40,  50,  60,  75,  90,  105,
    120, 140, 160, 180, 200, 225, 250, 280, 310, 350,
    400, 450, 500, 560, 630, 700, 780, 860, 950, 1050,
    1150, 1250, 1350, 1450, 1550
])
h_200 = np.array([
    108, 95,  85,  78,  72,  65,  60,  55,  50,  47,
    44,  42,  39,  37,  36,  34,  32,  31,  30,  28,
    27,  26,  25,  24.5, 24,  23,  22.5, 22,  21.5, 21,
    20.5, 20,  20,  19.5, 19
])

t_300 = np.array([
    8,  15,  22,  30,  40,  50,  60,  75,  90,  105,
    120, 140, 160, 180, 200, 225, 250, 280, 310, 350,
    400, 450, 500, 600, 700, 800, 900, 1000, 1100,
    1200, 1300, 1350, 1400, 1450, 1500
])
h_300 = np.array([
    105, 90,  78,  68,  62,  57,  53,  49,  45,  43,
    41,  38,  36,  34,  33,  31,  30,  29,  28,  27,
    26,  25.5, 25,  24,  25,  28,  30,  28,  32,
    30,  38,  37,  40,  35,  32
])

t_model = np.linspace(0.1, 2000, 2000)
h_eq25     = film_decay(t_model, h_c0, F_eq25, rho_0, p_h) * 1e9
h_eq24_bar = film_decay(t_model, h_c0, F_eq24_bar, rho_0, p_h) * 1e9
h_eq24_roe = film_decay(t_model, h_c0, F_eq24_roel, rho_0, p_h) * 1e9

print(f"\n{'─'*70}")
print(f"  Fig. 3 감쇠 곡선 비교 (h_c0={h_c0*1e9:.0f} nm)")
print(f"{'─'*70}")
print(f"\n  {'t [s]':>7}  {'Eq.25':>8}  {'Eq24+Bar':>9}  {'Eq24+Roel':>10}  {'200mm/s':>8}")
print(f"  {'─'*50}")
for t_c in [0, 50, 100, 200, 500, 1000, 1500]:
    h25 = film_decay(np.array([max(t_c, 0.1)]), h_c0, F_eq25, rho_0, p_h)[0] * 1e9
    h24b = film_decay(np.array([max(t_c, 0.1)]), h_c0, F_eq24_bar, rho_0, p_h)[0] * 1e9
    h24r = film_decay(np.array([max(t_c, 0.1)]), h_c0, F_eq24_roel, rho_0, p_h)[0] * 1e9
    idx = np.argmin(np.abs(t_200 - t_c))
    h_exp = h_200[idx] if abs(t_200[idx] - t_c) < 30 else float('nan')
    print(f"  {t_c:>7}  {h25:>8.1f}  {h24b:>9.1f}  {h24r:>10.1f}  {h_exp:>8.1f}")


# ═══════════════════════════════════════════════════════════════════
# 플롯
# ═══════════════════════════════════════════════════════════════════

fig, axes = plt.subplots(2, 2, figsize=(14, 11))

# ── (a) 점도 모델 비교 ──
ax = axes[0, 0]
p_range = np.linspace(0, 0.5e9, 500)
eta_r_arr = [eta_roelands(p, eta_0, z)/eta_0 for p in p_range]
eta_b_arr = [eta_barus(p, eta_0, alpha)/eta_0 for p in p_range]
ax.plot(p_range/1e9, eta_r_arr, 'b-', linewidth=2, label='Roelands (Eq.17)')
ax.plot(p_range/1e9, eta_b_arr, 'r--', linewidth=2, label='Barus (exp)')
ax.axvline(p_h/1e9, color='gray', linestyle=':', alpha=0.5, label=f'p_h={p_h/1e9} GPa')
ax.set_xlabel('Pressure [GPa]', fontsize=11)
ax.set_ylabel(r'$\eta/\eta_0$', fontsize=11)
ax.set_title('(a) Viscosity models', fontsize=12)
ax.legend(fontsize=9)
ax.set_yscale('log')
ax.grid(True, alpha=0.3)

# ── (b) 피적분함수 비교 ──
ax = axes[0, 1]
theta_range = np.linspace(-np.pi/2 * 0.999, np.pi/2 * 0.999, 500)

def integrand_roelands(theta):
    p = p_h * np.cos(theta)
    eta = eta_roelands(p, eta_0, z)
    rho = rho_DH(p, rho_0)
    return 1.0 / (eta * rho**2 * p) * a * np.cos(theta)

def integrand_barus(theta):
    p = p_h * np.cos(theta)
    eta = eta_barus(p, eta_0, alpha)
    rho = rho_DH(p, rho_0)
    return 1.0 / (eta * rho**2 * p) * a * np.cos(theta)

ig_r = [integrand_roelands(th) for th in theta_range]
ig_b = [integrand_barus(th) for th in theta_range]

ax.plot(theta_range * 180/np.pi, ig_r, 'b-', linewidth=2, label='Roelands')
ax.plot(theta_range * 180/np.pi, ig_b, 'r--', linewidth=2, label='Barus')
ax.set_xlabel(r'$\theta$ [deg]', fontsize=11)
ax.set_ylabel('Integrand value', fontsize=11)
ax.set_title('(b) Eq.24 integrand comparison', fontsize=12)
ax.legend(fontsize=9)
ax.grid(True, alpha=0.3)

# ── (c) Fig. 3 감쇠 비교 ──
ax = axes[1, 0]
ax.scatter(t_200, h_200, c='black', s=15, marker='s', label='200 mm/s (exp)', zorder=5)
ax.scatter(t_300, h_300, c='red', s=15, marker='o', label='300 mm/s (exp)', zorder=5)
ax.plot(t_model, h_eq25, 'g-.', linewidth=2, label=f'Eq.25 (approx)')
ax.plot(t_model, h_eq24_bar, 'r--', linewidth=2, label=f'Eq.24 + Barus')
ax.plot(t_model, h_eq24_roe, 'b-', linewidth=2.5, label=f'Eq.24 + Roelands')

ax.set_xlabel('Time, s', fontsize=11)
ax.set_ylabel(r'$h_c$ [nm]', fontsize=11)
ax.set_title('(c) Fig. 3 reproduction', fontsize=12)
ax.legend(fontsize=9, loc='upper right')
ax.set_xlim(0, 2000)
ax.set_ylim(0, 120)
ax.grid(True, alpha=0.3)

# ── (d) F(0) 바 차트 ──
ax = axes[1, 1]
labels = ['Eq.25\n(approx)', 'Eq.24\nBarus', 'Eq.24\nRoelands', 'Eq.24\nRoel+Incomp']
values = [F_eq25, F_eq24_bar, F_eq24_roel, F_eq24_inc]
colors_bar = ['green', 'red', 'blue', 'cyan']
bars = ax.bar(labels, [v/F_eq25 for v in values], color=colors_bar, alpha=0.7, edgecolor='black')
ax.axhline(1.0, color='gray', linestyle='--', alpha=0.5)
ax.set_ylabel(r'$\mathcal{F}(0)$ / $\mathcal{F}_{Eq.25}(0)$', fontsize=11)
ax.set_title(f'(d) F(0) ratios (circular, p_h={p_h/1e9} GPa)', fontsize=12)
for bar_item, v in zip(bars, values):
    ax.text(bar_item.get_x() + bar_item.get_width()/2, bar_item.get_height() + 0.02,
            f'{v/F_eq25:.2f}×', ha='center', fontsize=10, fontweight='bold')
ax.grid(True, alpha=0.3, axis='y')

fig.suptitle('Eq. 24 (Full Integral) vs Eq. 25 (Approximate) — Van Zoelen Model',
             fontsize=14, fontweight='bold')
fig.tight_layout()
fig.savefig(OUT_DIR / 'eq24_vs_eq25.png', dpi=150)
print(f"\n  → 저장: {OUT_DIR / 'eq24_vs_eq25.png'}")
plt.close(fig)
