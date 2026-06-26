"""
Fig. 3 정밀 재현: 원형 접촉 (k=1) 막두께 감쇠
================================================
PCS EHD ball-on-disc, Li/M grease, 200 & 300 mm/s

Van Zoelen model (Eq. 1-3)을 구현하고,
h_c0 best-fit + 원본 그래프와의 오차를 정량 분석합니다.
"""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8', errors='replace')

import numpy as np
import matplotlib.pyplot as plt
from scipy.optimize import minimize_scalar
from pathlib import Path

OUT_DIR = Path(__file__).parent / "van_zoelen_results"
OUT_DIR.mkdir(exist_ok=True)

# ─── Van Zoelen 모델 ──────────────────────────────────────────────

def normalized_density(p_h, rho_0):
    rho_ph = rho_0 * (5.9e8 + 1.34 * p_h) / (5.9e8 + p_h)
    return rho_ph / rho_0

def side_flow_parameter(p_h, a, b, eta_0, alpha, l_t):
    term = 0.5 * np.pi * alpha * p_h
    visc_corr = (term**1.5 + 1.0) ** (-2.0 / 3.0)
    return (2.0 / l_t) * (p_h / b**2) * (a / eta_0) * np.pi * visc_corr

def film_decay(t, h_c0, coeff):
    """h_c(t) = (coeff × t + h_{c,0}^{-2})^{-1/2}"""
    return (coeff * t + h_c0**(-2)) ** (-0.5)


# ─── PCS EHD Ball-on-Disc 조건 (Table 1, 2) ───────────────────────

p_h     = 0.48e9       # 0.48 GPa
a       = 141e-6       # 141 μm
b       = 141e-6       # 141 μm (원형: k=1)
eta_0   = 0.38         # Pa·s (calibrated grease viscosity)
alpha   = 30e-9        # 1/Pa (= 30 GPa⁻¹)
rho_0   = 891.0        # kg/m³
R_ball  = 9.53e-3      # m
R_track = 0.040        # m (PCS EHD disc track radius)
l_t     = 2*np.pi*R_track + 2*np.pi*R_ball  # 총 트랙 길이

h_cff_200 = 500e-9     # 500 nm (Table 2)
h_cff_300 = 655e-9     # 655 nm (Table 2)

rho_bar = normalized_density(p_h, rho_0)
F0 = side_flow_parameter(p_h, a, b, eta_0, alpha, l_t)
C_theory = (1.0/6.0) * rho_bar**2 * F0  # 이론 감쇠 계수


# ─── Fig. 3 원본에서 정밀 디지타이즈한 실험 데이터 ──────────────────

# 200 mm/s (검정 사각형) — 모델과 잘 일치
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

# 300 mm/s (빨간 원) — 후반부 산란 큼 (spin/slip에 의한 재공급)
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

# ─── 원본 Fig. 3의 모델 곡선에서 디지타이즈한 값 ─────────────────
# (검은 실선 — Van Zoelen model line from the paper)
# Y축: 0-120 nm, X축: 0-2000 s, 주요 그리드라인 참조하여 정밀 판독
t_orig_model = np.array([
    0,    10,   20,   30,   50,   75,   100,  150,  200,
    300,  400,  500,  600,  700,  800,  1000,
    1200, 1500, 1800, 2000
])
h_orig_model = np.array([
    108,  90,   78,   70,   58,   48,   43,   36,   32,
    27,   24.5, 22.5, 21,   20,   19,   17.5,
    16.5, 15,   14.5, 14
])


# ─── 원본 모델 곡선에 대한 best-fit 감쇠 계수 ─────────────────────

def fit_error(log_C, t_data, h_data, h_c0):
    """감쇠 계수에 대한 RMSE."""
    C = np.exp(log_C)
    h_pred = film_decay(t_data, h_c0, C) * 1e9
    return np.sqrt(np.mean((h_pred - h_data)**2))

# 원본 모델 곡선에 best-fit (h_c0 = 110 nm 고정)
h_c0 = 110e-9
res = minimize_scalar(
    fit_error, bounds=(np.log(C_theory * 0.5), np.log(C_theory * 5)),
    method='bounded', args=(t_orig_model[1:], h_orig_model[1:], h_c0)
)
C_fit = np.exp(res.x)

print("=" * 65)
print("  Fig. 3: PCS EHD Ball-on-Disc, Li/M grease (k=1)")
print("=" * 65)
print(f"\n  ── 모델 파라미터 ──")
print(f"  p_h     = {p_h/1e9:.3f} GPa")
print(f"  a = b   = {a*1e6:.1f} μm  (k=1)")
print(f"  η₀      = {eta_0:.3f} Pa·s")
print(f"  α       = {alpha*1e9:.1f} GPa⁻¹")
print(f"  ρ₀      = {rho_0:.0f} kg/m³")
print(f"  l_t     = {l_t*1e3:.2f} mm")
print(f"  ρ̄_c     = {rho_bar:.4f}")
print(f"  F(0)    = {F0:.4e} m⁻²s⁻¹")
print(f"  h_c0    = {h_c0*1e9:.0f} nm")

print(f"\n  ── 감쇠 계수 비교 ──")
print(f"  이론값 (Eq.1-3):    C = {C_theory:.4e} m⁻²s⁻¹")
print(f"  Best-fit (원본곡선): C = {C_fit:.4e} m⁻²s⁻¹")
print(f"  보정 비율:           {C_fit/C_theory:.2f}×")

# F(0)으로 환산
F0_fit = C_fit / ((1.0/6.0) * rho_bar**2)
print(f"\n  ── 보정된 F(0) ──")
print(f"  F(0)_이론  = {F0:.4e}")
print(f"  F(0)_보정  = {F0_fit:.4e}")
print(f"  비율       = {F0_fit/F0:.2f}×")
print(f"  (가능 원인: Roelands→Barus α 변환 차이, η₀ 보정 등)")


# ─── 모델 곡선 계산 ────────────────────────────────────────────────

t_model = np.linspace(0.1, 2000, 2000)
h_theory = film_decay(t_model, h_c0, C_theory) * 1e9   # 이론 (Eq.1-3 직접)
h_fitted = film_decay(t_model, h_c0, C_fit) * 1e9       # best-fit


# ─── 정량 비교 테이블 ─────────────────────────────────────────────

print(f"\n  ── 시점별 비교 (단위: nm) ──")
print(f"  {'t [s]':>7}  {'원본 모델':>10}  {'Eq.1-3':>10}  {'Best-fit':>10}  {'200mm/s':>10}  {'300mm/s':>10}")
print(f"  {'─'*62}")

for t_c in [0, 50, 100, 200, 500, 1000, 1500]:
    h_o = np.interp(t_c, t_orig_model, h_orig_model)
    h_t = film_decay(np.array([max(t_c, 0.1)]), h_c0, C_theory)[0] * 1e9
    h_f = film_decay(np.array([max(t_c, 0.1)]), h_c0, C_fit)[0] * 1e9
    idx2 = np.argmin(np.abs(t_200 - t_c))
    idx3 = np.argmin(np.abs(t_300 - t_c))
    h_e2 = h_200[idx2] if abs(t_200[idx2] - t_c) < 30 else float('nan')
    h_e3 = h_300[idx3] if abs(t_300[idx3] - t_c) < 30 else float('nan')
    print(f"  {t_c:>7}  {h_o:>10.1f}  {h_t:>10.1f}  {h_f:>10.1f}  {h_e2:>10.1f}  {h_e3:>10.1f}")

# 논문 검증: h/h_cff = 0.18 → 0.04 in 17 min
print(f"\n  ── 논문 기술 검증 (best-fit) ──")
h_0 = h_c0
h_17min = film_decay(np.array([17*60.0]), h_c0, C_fit)[0]
print(f"  h_c0/h_cff = {h_0/h_cff_200:.2f}  (논문: ~0.18)")
print(f"  h_c(17min)/h_cff = {h_17min/h_cff_200:.2f}  (논문: ~0.04)")


# ─── 플롯 ─────────────────────────────────────────────────────────

fig, axes = plt.subplots(1, 2, figsize=(15, 6))

# ── (a) 전체 비교 ──
ax = axes[0]
ax.scatter(t_200, h_200, c='black', s=18, marker='s',
           label=r'$u_m$ = 200 mm/s (exp)', zorder=5)
ax.scatter(t_300, h_300, c='red', s=18, marker='o',
           label=r'$u_m$ = 300 mm/s (exp)', zorder=5)
ax.plot(t_model, h_fitted, 'k-', linewidth=2,
        label=f'Van Zoelen model (best-fit, C×{C_fit/C_theory:.2f})')
ax.plot(t_model, h_theory, 'b--', linewidth=1.5, alpha=0.6,
        label='Eq.1-3 direct (no correction)')

ax.set_xlabel('Time, s', fontsize=12)
ax.set_ylabel(r'Central film thickness $h_c$, nm', fontsize=12)
ax.set_title('(a) Fig. 3 Reproduction', fontsize=13)
ax.legend(fontsize=9, loc='upper right')
ax.set_xlim(0, 2000)
ax.set_ylim(0, 120)
ax.grid(True, alpha=0.3)

# ── (b) 잔차 분석 ──
ax = axes[1]
h_fit_at_200 = film_decay(t_200, h_c0, C_fit) * 1e9
h_fit_at_300 = film_decay(t_300, h_c0, C_fit) * 1e9

resid_200 = h_200 - h_fit_at_200
resid_300 = h_300 - h_fit_at_300

ax.scatter(t_200, resid_200, c='black', s=18, marker='s', label='200 mm/s', zorder=5)
ax.scatter(t_300, resid_300, c='red', s=18, marker='o', label='300 mm/s', zorder=5)
ax.axhline(0, color='k', linewidth=0.8)
ax.fill_between([0, 2000], [-5, -5], [5, 5], color='green', alpha=0.1, label='±5 nm band')

rmse_200 = np.sqrt(np.mean(resid_200**2))
rmse_300_early = np.sqrt(np.mean(resid_300[:20]**2))  # <500s
rmse_300_late = np.sqrt(np.mean(resid_300[20:]**2))   # >500s

ax.annotate(f'RMSE (200mm/s) = {rmse_200:.1f} nm\n'
            f'RMSE (300mm/s, t<500s) = {rmse_300_early:.1f} nm\n'
            f'RMSE (300mm/s, t>500s) = {rmse_300_late:.1f} nm',
            xy=(1000, 12), fontsize=9,
            bbox=dict(boxstyle='round,pad=0.3', facecolor='lightyellow', alpha=0.8))

ax.set_xlabel('Time, s', fontsize=12)
ax.set_ylabel('Residual (exp - model), nm', fontsize=12)
ax.set_title('(b) Residual Analysis (best-fit model)', fontsize=13)
ax.legend(fontsize=9)
ax.set_xlim(0, 2000)
ax.set_ylim(-15, 20)
ax.grid(True, alpha=0.3)

fig.suptitle('Fig. 3: Circular Contact (k=1), Li/M grease — PCS EHD ball-on-disc',
             fontsize=13, fontweight='bold')
fig.tight_layout()
fig.savefig(OUT_DIR / 'fig3_reproduction.png', dpi=150)
print(f"\n  → 저장: {OUT_DIR / 'fig3_reproduction.png'}")
plt.close(fig)
