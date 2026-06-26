"""
Van Zoelen Side Flow Model — 논문 검증 스크립트
===============================================
논문: Gao et al. (2026) "Film thickness decay in grease lubricated wide elliptical contacts"
       Tribology International

Van Zoelen의 side flow 모델 (Eq. 1-3)을 구현하고, 논문의 Fig. 3, 4, 6 및 Table 3-4를
재현하여 구현의 정확성을 검증합니다.

핵심 수식:
  h_c(t) = (1/6 × ρ̄_c² × F(0) × t + h_{c,0}^{-2})^{-1/2}        ...(1)
  ρ(p)   = ρ_0 × (5.9e8 + 1.34p) / (5.9e8 + p)                   ...(2)
  F(0)   ≈ (2/l_t) × (p_h/b²) × (a/η_0) × π × ((0.5πα p_h)^{3/2} + 1)^{-2/3}  ...(3)
"""

import numpy as np
import matplotlib.pyplot as plt
from dataclasses import dataclass
from pathlib import Path

# ─── 출력 디렉토리 ─────────────────────────────────────────────────
OUT_DIR = Path(__file__).parent / "van_zoelen_results"
OUT_DIR.mkdir(exist_ok=True)


# ─── 데이터 클래스 ─────────────────────────────────────────────────
@dataclass
class ContactParams:
    """접촉 조건 파라미터"""
    p_h: float      # 최대 Hertz 압력 [Pa]
    a: float         # 구름 방향 반접촉폭 [m]
    b: float         # 횡방향 반접촉폭 [m]
    eta_0: float     # 상온 동점도 [Pa·s]
    alpha: float     # 압력-점도 계수 [1/Pa]
    rho_0: float     # 상온 밀도 [kg/m³]
    l_t: float       # 총 트랙 길이 [m]
    label: str = ""


# ─── Van Zoelen 모델 구현 ──────────────────────────────────────────

def dowson_higginson_density(p: float, rho_0: float) -> float:
    """Dowson-Higginson 압축성 밀도 (Eq. 2)."""
    return rho_0 * (5.9e8 + 1.34 * p) / (5.9e8 + p)


def normalized_density(p_h: float, rho_0: float) -> float:
    """무차원 밀도 ρ̄_c = ρ(p_h) / ρ_0."""
    return dowson_higginson_density(p_h, rho_0) / rho_0


def side_flow_parameter(cp: ContactParams) -> float:
    """Side flow 파라미터 F(0) (Eq. 3).

    F(0) ≈ (2/l_t) × (p_h/b²) × (a/η_0) × π × ((0.5π α p_h)^{3/2} + 1)^{-2/3}
    """
    term = 0.5 * np.pi * cp.alpha * cp.p_h  # 무차원 압력-점도 파라미터
    viscosity_correction = (term**1.5 + 1.0) ** (-2.0 / 3.0)

    F0 = (2.0 / cp.l_t) * (cp.p_h / cp.b**2) * (cp.a / cp.eta_0) \
         * np.pi * viscosity_correction

    return F0


def film_thickness_decay(t: np.ndarray, h_c0: float, cp: ContactParams) -> np.ndarray:
    """Van Zoelen side flow 모델에 의한 중심 막두께 감쇠 (Eq. 1).

    h_c(t) = (1/6 × ρ̄_c² × F(0) × t + h_{c,0}^{-2})^{-1/2}

    Args:
        t: 시간 배열 [s]
        h_c0: 초기 중심 막두께 [m]
        cp: 접촉 파라미터

    Returns:
        h_c(t) 배열 [m]
    """
    rho_bar = normalized_density(cp.p_h, cp.rho_0)
    F0 = side_flow_parameter(cp)

    decay_coeff = (1.0 / 6.0) * rho_bar**2 * F0
    h_c = (decay_coeff * t + h_c0**(-2)) ** (-0.5)

    return h_c


# ─── 논문 실험 조건 정의 ───────────────────────────────────────────

# Table 1 & 2: 재료 물성
E_steel = 206e9    # GPa → Pa
nu_steel = 0.3
E_glass = 63e9
nu_glass = 0.2

# 환산 탄성계수 E'
E_prime = 2.0 / ((1 - nu_steel**2) / E_steel + (1 - nu_glass**2) / E_glass)

# 실험 장비 트랙 반경
R_track_ball = 0.040     # PCS EHD 디스크 트랙 반경 [m]
R_ball = 9.53e-3          # 볼 반경 [m]
R_track_roller = 0.045   # WAM 디스크 트랙 반경 [m]
R_roller_x = 3.91e-3     # 롤러 구름방향 반경 [m]

# 총 트랙 길이
l_t_ball = 2 * np.pi * R_track_ball + 2 * np.pi * R_ball       # 볼-디스크
l_t_roller = 2 * np.pi * R_track_roller + 2 * np.pi * R_roller_x  # 롤러-디스크

# 그리스 물성 (Table 2)
# Li/M
eta_0_LiM = 0.38        # Pa·s
alpha_LiM = 30e-9       # 1/Pa  (= 30 GPa^-1)
rho_0_LiM = 891.0       # kg/m³

# Li/SS
eta_0_LiSS = 0.135      # Pa·s
alpha_LiSS = 21.5e-9    # 1/Pa
rho_0_LiSS = 900.0      # kg/m³


# ─── Case 1: Fig. 3 — 원형 접촉 (볼, k=1) ─────────────────────────
cp_ball = ContactParams(
    p_h=0.48e9,          # 0.48 GPa
    a=141e-6,            # 141 μm
    b=141e-6,            # 141 μm (원형: k = b/a = 1)
    eta_0=eta_0_LiM,
    alpha=alpha_LiM,
    rho_0=rho_0_LiM,
    l_t=l_t_ball,
    label="Ball (k=1), Li/M"
)

# ─── Case 2: Fig. 4 — 타원비 변화 (k = 1, 2, 10, 50) ──────────────
def make_ellipticity_case(k: float) -> ContactParams:
    """p_h=0.22 GPa, a=39.2 μm 고정, b=k×a로 타원비 변경."""
    a = 39.2e-6
    return ContactParams(
        p_h=0.22e9,
        a=a,
        b=k * a,
        eta_0=eta_0_LiM,
        alpha=alpha_LiM,
        rho_0=rho_0_LiM,
        l_t=l_t_roller,
        label=f"k={k}"
    )


# ─── Case 3: Fig. 6 — Wide elliptical (롤러, k≈69), Li/M & Li/SS ──
cp_roller_LiM = ContactParams(
    p_h=0.22e9,
    a=39.2e-6,
    b=2710e-6,           # k = 2710/39.2 ≈ 69.1
    eta_0=eta_0_LiM,
    alpha=alpha_LiM,
    rho_0=rho_0_LiM,
    l_t=l_t_roller,
    label="Roller (k≈69), Li/M"
)

cp_roller_LiSS = ContactParams(
    p_h=0.22e9,
    a=39.2e-6,
    b=2710e-6,
    eta_0=eta_0_LiSS,
    alpha=alpha_LiSS,
    rho_0=rho_0_LiSS,
    l_t=l_t_roller,
    label="Roller (k≈69), Li/SS"
)

# Table 2: fully flooded 막두께 h_cff [m]
h_cff_LiM = {100: 0.353e-6, 150: 0.493e-6, 200: 0.524e-6}
h_cff_LiSS = {100: 0.138e-6, 150: 0.200e-6, 200: 0.278e-6}

# 논문 Section 5.2: 초기 starvation ratio h_{c,0}/h_{cff}
#   100 mm/s: 0.91, 150 mm/s: 0.65, 200 mm/s: 0.61
starvation_ratio_LiM = {100: 0.91, 150: 0.65, 200: 0.61}

# Table 4: 0° 스큐에서의 실험 평균 감쇠율 ḣ_ave [nm/s]
exp_decay_rate_LiM = {100: 0.028, 150: 0.020, 200: 0.022}  # 논문 수정: Table 4
exp_decay_rate_LiSS = {100: 0.010, 150: 0.005, 200: 0.006}

# Table 3: 0° 스큐에서의 실험 평균 감쇠율 (Li/M, Fig.5에서)
exp_decay_rate_LiM_table3 = {100: 0.022, 150: 0.035, 200: 0.033}


# ═══════════════════════════════════════════════════════════════════
# 검증 1: 중간 파라미터 출력
# ═══════════════════════════════════════════════════════════════════

def print_params(cp: ContactParams, label: str):
    """핵심 파라미터 계산 및 출력."""
    k = cp.b / cp.a
    rho_bar = normalized_density(cp.p_h, cp.rho_0)
    F0 = side_flow_parameter(cp)

    # Eq. 3 분해
    term = 0.5 * np.pi * cp.alpha * cp.p_h
    visc_corr = (term**1.5 + 1.0) ** (-2.0 / 3.0)

    print(f"\n{'='*60}")
    print(f"  {label}")
    print(f"{'='*60}")
    print(f"  p_h       = {cp.p_h/1e9:.3f} GPa")
    print(f"  a         = {cp.a*1e6:.1f} μm")
    print(f"  b         = {cp.b*1e6:.1f} μm")
    print(f"  k = b/a   = {k:.1f}")
    print(f"  η₀        = {cp.eta_0:.3f} Pa·s")
    print(f"  α         = {cp.alpha*1e9:.1f} GPa⁻¹")
    print(f"  ρ₀        = {cp.rho_0:.0f} kg/m³")
    print(f"  l_t       = {cp.l_t*1e3:.2f} mm")
    print(f"  ───────────────────────────────")
    print(f"  ρ̄_c       = {rho_bar:.4f}")
    print(f"  0.5πα p_h = {term:.4f}")
    print(f"  visc_corr = {visc_corr:.6f}")
    print(f"  F(0)      = {F0:.6e} m⁻² s⁻¹")
    print(f"  1/6 ρ̄²F₀  = {(1/6)*rho_bar**2*F0:.6e} m⁻² s⁻¹ (감쇠 계수)")

    return F0, rho_bar


# ═══════════════════════════════════════════════════════════════════
# 검증 2: Fig. 3 재현 — 원형 접촉 막두께 감쇠
# ═══════════════════════════════════════════════════════════════════

def verify_fig3():
    """Fig. 3: 원형 접촉 (k=1), 200 & 300 mm/s, Li/M grease."""
    print("\n" + "█"*60)
    print("  검증 2: Fig. 3 — 원형 접촉 막두께 감쇠 (k=1)")
    print("█"*60)

    F0, rho_bar = print_params(cp_ball, "PCS EHD Ball-on-Disc (Li/M)")

    # Fig. 3에서 읽은 대략적 초기값 (측정 시작점)
    # 200 mm/s: ~90 nm에서 시작 → ~25 nm까지 약 1000s
    # 300 mm/s: ~90 nm에서 시작
    # 논문 텍스트에서: circular contact에서 h/h_cff = 0.18 → 0.04 in 17 min
    # at 200 mm/s: h_cff = 500 nm, so h_c0 ≈ 0.18 × 500 = 90 nm
    h_c0_fig3 = 90e-9  # 90 nm (Fig. 3 시작점 추정)

    t = np.linspace(0, 1200, 1000)
    h_c = film_thickness_decay(t, h_c0_fig3, cp_ball) * 1e9  # → nm

    # 논문 검증: h/h_cff drops from 0.18 to 0.04 in 17 min = 1020 s
    t_17min = 1020.0
    h_at_17min = film_thickness_decay(np.array([t_17min]), h_c0_fig3, cp_ball)[0] * 1e9
    ratio_start = h_c0_fig3 / (500e-9)
    ratio_end = h_at_17min * 1e-9 / (500e-9)

    print(f"\n  ── Fig. 3 검증 (200 mm/s) ──")
    print(f"  h_c(0)      = {h_c0_fig3*1e9:.1f} nm")
    print(f"  h_c(1020s)  = {h_at_17min:.1f} nm")
    print(f"  h/h_cff:  {ratio_start:.2f} → {ratio_end:.2f}  (논문: 0.18 → 0.04)")
    print(f"  17분 감쇠 비율: {h_at_17min / (h_c0_fig3*1e9):.2f}")

    # 플롯
    fig, ax = plt.subplots(1, 1, figsize=(8, 5))
    ax.plot(t, h_c, 'k-', linewidth=2, label='Van Zoelen model')

    # 논문 Fig. 3에서 대략적으로 읽은 실험 데이터 포인트
    # (정확한 데이터가 없으므로 논문 그래프에서 추정)
    t_exp_200 = np.array([0, 50, 100, 200, 300, 500, 700, 900, 1020])
    h_exp_200 = np.array([90, 70, 60, 47, 40, 32, 27, 23, 20])  # 추정값

    ax.scatter(t_exp_200, h_exp_200, c='black', s=40, marker='s',
               label='200 mm/s (approx.)', zorder=5)

    ax.set_xlabel('Time [s]', fontsize=12)
    ax.set_ylabel('Central film thickness [nm]', fontsize=12)
    ax.set_title('Fig. 3 Verification: Circular Contact (k=1), Li/M grease', fontsize=13)
    ax.legend(fontsize=11)
    ax.set_xlim(0, 1200)
    ax.set_ylim(0, 100)
    ax.grid(True, alpha=0.3)
    fig.tight_layout()
    fig.savefig(OUT_DIR / 'fig3_circular_contact.png', dpi=150)
    print(f"  → 그래프 저장: {OUT_DIR / 'fig3_circular_contact.png'}")
    plt.close(fig)


# ═══════════════════════════════════════════════════════════════════
# 검증 3: Fig. 4 재현 — 타원비 효과
# ═══════════════════════════════════════════════════════════════════

def verify_fig4():
    """Fig. 4: 타원비 k=1,2,10,50에 따른 감쇠 비교."""
    print("\n" + "█"*60)
    print("  검증 3: Fig. 4 — 타원비(k) 효과")
    print("█"*60)

    h_c0 = 310e-9  # 310 nm (논문 명시)

    k_values = [1, 2, 10, 50]
    colors = ['black', 'blue', 'red', 'green']

    fig, ax = plt.subplots(1, 1, figsize=(10, 6))

    for k, color in zip(k_values, colors):
        cp = make_ellipticity_case(k)
        F0, rho_bar = print_params(cp, f"Ellipticity k={k}")

        # 감쇠 시간 스케일 추정: h_c가 310 → 125 nm이 되는 시간
        decay_coeff = (1.0 / 6.0) * rho_bar**2 * F0
        # h_c(t) = (decay_coeff * t + h_c0^{-2})^{-0.5} = 125e-9
        # decay_coeff * t + h_c0^{-2} = (125e-9)^{-2}
        # t = ((125e-9)^{-2} - h_c0^{-2}) / decay_coeff
        h_target = 125e-9
        t_target = ((h_target**(-2)) - h_c0**(-2)) / decay_coeff

        print(f"  t(310→125 nm) = {t_target:.1f} s")

        # 적절한 시간 범위
        t_max = max(t_target * 3, 100)
        t = np.logspace(-1, np.log10(t_max), 2000)
        h_c = film_thickness_decay(t, h_c0, cp) * 1e9

        ax.plot(t, h_c, color=color, linewidth=2, label=f'k = {k}')

    ax.set_xscale('log')
    ax.set_xlabel('Time [s]', fontsize=12)
    ax.set_ylabel('Central film thickness [nm]', fontsize=12)
    ax.set_title('Fig. 4 Verification: Effect of Ellipticity on Film Thickness Decay', fontsize=13)
    ax.legend(fontsize=11)
    ax.set_ylim(0, 350)
    ax.grid(True, alpha=0.3, which='both')

    # 논문 텍스트 검증: k 10배 증가 → 감쇠 시간 ~100배
    ax.axhline(y=125, color='gray', linestyle='--', alpha=0.5, label='h=125 nm')

    fig.tight_layout()
    fig.savefig(OUT_DIR / 'fig4_ellipticity_effect.png', dpi=150)
    print(f"\n  → 그래프 저장: {OUT_DIR / 'fig4_ellipticity_effect.png'}")
    plt.close(fig)

    # k 10배 → t 100배 검증
    print(f"\n  ── k 스케일링 검증 ──")
    for i in range(len(k_values) - 1):
        k1, k2 = k_values[i], k_values[i+1]
        cp1, cp2 = make_ellipticity_case(k1), make_ellipticity_case(k2)
        F01 = side_flow_parameter(cp1)
        F02 = side_flow_parameter(cp2)
        # F(0) ∝ 1/b² = 1/(k×a)², so t ∝ 1/F(0) ∝ k²
        ratio = F01 / F02
        print(f"  k: {k1}→{k2} (×{k2/k1:.0f}), F(0) ratio = {ratio:.1f}, "
              f"decay time ratio ≈ {ratio:.1f} (이론: {(k2/k1)**2:.0f})")


# ═══════════════════════════════════════════════════════════════════
# 검증 4: Fig. 6 재현 — Wide elliptical, 속도별 비교
# ═══════════════════════════════════════════════════════════════════

def verify_fig6():
    """Fig. 6: Wide elliptical contact (k≈69), 0° 스큐, 100/150/200 mm/s."""
    print("\n" + "█"*60)
    print("  검증 4: Fig. 6 — Wide Elliptical Contact (k≈69)")
    print("█"*60)

    F0_LiM, rho_bar_LiM = print_params(cp_roller_LiM, "Roller Li/M (k≈69)")
    F0_LiSS, rho_bar_LiSS = print_params(cp_roller_LiSS, "Roller Li/SS (k≈69)")

    speeds = [100, 150, 200]
    t = np.linspace(0, 12000, 2000)  # 약 3시간

    fig, axes = plt.subplots(1, 2, figsize=(14, 6))

    # ─── Fig. 6(a): Li/M ───
    ax = axes[0]
    colors_speed = {100: 'blue', 150: 'red', 200: 'green'}

    print(f"\n  ── Li/M 감쇠율 비교 (Table 4 vs. 모델) ──")
    print(f"  {'속도':>8}  {'h_c0 [nm]':>10}  {'ḣ_ave(모델)':>12}  {'ḣ_ave(실험)':>12}  {'차이':>8}")

    for speed in speeds:
        h_c0 = starvation_ratio_LiM[speed] * h_cff_LiM[speed]
        h_c = film_thickness_decay(t, h_c0, cp_roller_LiM) * 1e9

        ax.plot(t, h_c, color=colors_speed[speed], linewidth=2,
                label=f'{speed} mm/s (model)')

        # 평균 감쇠율 계산: (h_c0 - h_c(4000)) / 4000
        h_at_4000 = film_thickness_decay(np.array([4000.0]), h_c0, cp_roller_LiM)[0] * 1e9
        h_ave_model = (h_c0 * 1e9 - h_at_4000) / 4000.0

        # Table 4 실험값 (0° 스큐, Fig.6 기준)
        h_ave_exp = exp_decay_rate_LiM[speed]

        diff_pct = (h_ave_model - h_ave_exp) / h_ave_exp * 100
        print(f"  {speed:>5} mm/s  {h_c0*1e9:>10.1f}  {h_ave_model:>12.4f}  {h_ave_exp:>12.4f}  {diff_pct:>+7.1f}%")

    ax.set_xlabel('Time [s]', fontsize=12)
    ax.set_ylabel('Central film thickness [nm]', fontsize=12)
    ax.set_title('Fig. 6(a): Li/M grease, 0° skew', fontsize=13)
    ax.legend(fontsize=10)
    ax.set_xlim(0, 12000)
    ax.set_ylim(0, 400)
    ax.grid(True, alpha=0.3)

    # ─── Fig. 6(b): Li/SS ───
    ax = axes[1]

    print(f"\n  ── Li/SS 감쇠율 비교 (Table 4 vs. 모델) ──")
    print(f"  {'속도':>8}  {'h_c0 [nm]':>10}  {'ḣ_ave(모델)':>12}  {'ḣ_ave(실험)':>12}  {'차이':>8}")

    for speed in speeds:
        # Li/SS starvation ratio 불명 → h_cff를 초기값으로 사용 (fully flooded 시작)
        h_c0 = h_cff_LiSS[speed]
        h_c = film_thickness_decay(t, h_c0, cp_roller_LiSS) * 1e9

        ax.plot(t, h_c, color=colors_speed[speed], linewidth=2,
                label=f'{speed} mm/s (model)')

        h_at_4000 = film_thickness_decay(np.array([4000.0]), h_c0, cp_roller_LiSS)[0] * 1e9
        h_ave_model = (h_c0 * 1e9 - h_at_4000) / 4000.0
        h_ave_exp = exp_decay_rate_LiSS[speed]

        diff_pct = (h_ave_model - h_ave_exp) / h_ave_exp * 100 if h_ave_exp > 0 else 0
        print(f"  {speed:>5} mm/s  {h_c0*1e9:>10.1f}  {h_ave_model:>12.4f}  {h_ave_exp:>12.4f}  {diff_pct:>+7.1f}%")

    ax.set_xlabel('Time [s]', fontsize=12)
    ax.set_ylabel('Central film thickness [nm]', fontsize=12)
    ax.set_title('Fig. 6(b): Li/SS grease, 0° skew', fontsize=13)
    ax.legend(fontsize=10)
    ax.set_xlim(0, 12000)
    ax.set_ylim(0, 350)
    ax.grid(True, alpha=0.3)

    fig.tight_layout()
    fig.savefig(OUT_DIR / 'fig6_wide_elliptical.png', dpi=150)
    print(f"\n  → 그래프 저장: {OUT_DIR / 'fig6_wide_elliptical.png'}")
    plt.close(fig)


# ═══════════════════════════════════════════════════════════════════
# 검증 5: Table 3 비교 — 스큐 각도별 감쇠율 (모델 기준선)
# ═══════════════════════════════════════════════════════════════════

def verify_table3():
    """Table 3: 스큐 각도별 감쇠율. 모델은 0°만 예측 (스큐 무시).
    스큐에 따른 차이는 모델의 한계를 보여줌."""
    print("\n" + "█"*60)
    print("  검증 5: Table 3 — 스큐 각도별 감쇠율 비교")
    print("█"*60)

    # Table 3 전체 데이터
    table3_data = {
        100: {+2: 0.019, +1: 0.020, 0: 0.022, -1: 0.025, -2: 0.029},
        150: {+2: 0.022, +1: 0.029, 0: 0.035, -1: 0.037, -2: 0.039},
        200: {+2: 0.022, +1: 0.025, 0: 0.033, -1: 0.035, -2: 0.037},
    }

    # Fig. 5에서 사용된 초기 막두께: 300 nm (100 mm/s), 350 nm (150 mm/s)
    # 200 mm/s의 경우 약 300-350 nm 추정
    h_c0_fig5 = {100: 300e-9, 150: 350e-9, 200: 300e-9}

    print(f"\n  {'속도':>6}  {'스큐':>5}  {'ḣ(실험)':>10}  {'ḣ(모델)':>10}  {'차이':>8}  {'비고':>12}")
    print(f"  {'─'*65}")

    for speed in [100, 150, 200]:
        h_c0 = h_c0_fig5[speed]
        h_at_4000 = film_thickness_decay(
            np.array([4000.0]), h_c0, cp_roller_LiM
        )[0] * 1e9
        h_ave_model = (h_c0 * 1e9 - h_at_4000) / 4000.0

        for skew in [+2, +1, 0, -1, -2]:
            h_ave_exp = table3_data[speed][skew]
            diff_pct = (h_ave_model - h_ave_exp) / h_ave_exp * 100

            note = "← 모델 기준" if skew == 0 else ""
            print(f"  {speed:>4} mm/s  {skew:>+3}°  {h_ave_exp:>10.3f}  "
                  f"{h_ave_model:>10.4f}  {diff_pct:>+7.1f}%  {note}")
        print()


# ═══════════════════════════════════════════════════════════════════
# 검증 6: 핵심 스케일링 법칙 검증
# ═══════════════════════════════════════════════════════════════════

def verify_scaling():
    """논문의 핵심 결론 검증: k×10 → 감쇠 시간 ×100."""
    print("\n" + "█"*60)
    print("  검증 6: 감쇠 시간 스케일링 법칙")
    print("█"*60)

    h_c0 = 310e-9
    h_target = 125e-9

    k_values = [1, 2, 5, 10, 20, 50, 69.1]
    print(f"\n  h_c: {h_c0*1e9:.0f} → {h_target*1e9:.0f} nm 감쇠 소요 시간:")
    print(f"  {'k':>6}  {'F(0)':>14}  {'t [s]':>12}  {'t [min]':>10}  {'t [hr]':>10}  {'t/t(k=1)':>10}")

    t_ref = None
    for k in k_values:
        cp = make_ellipticity_case(k)
        F0 = side_flow_parameter(cp)
        rho_bar = normalized_density(cp.p_h, cp.rho_0)
        decay_coeff = (1.0 / 6.0) * rho_bar**2 * F0
        t_decay = (h_target**(-2) - h_c0**(-2)) / decay_coeff

        if t_ref is None:
            t_ref = t_decay

        print(f"  {k:>6.1f}  {F0:>14.4e}  {t_decay:>12.1f}  {t_decay/60:>10.1f}  "
              f"{t_decay/3600:>10.2f}  {t_decay/t_ref:>10.1f}")


# ═══════════════════════════════════════════════════════════════════
# 실행
# ═══════════════════════════════════════════════════════════════════

if __name__ == "__main__":
    print("╔══════════════════════════════════════════════════════════════╗")
    print("║  Van Zoelen Side Flow Model — 논문 검증                     ║")
    print("║  Gao et al. (2026), Tribology International                 ║")
    print("╚══════════════════════════════════════════════════════════════╝")

    # 검증 1: 파라미터 확인
    print_params(cp_ball, "Case 1: PCS EHD Ball (k=1)")
    print_params(cp_roller_LiM, "Case 2: WAM Roller Li/M (k≈69)")
    print_params(cp_roller_LiSS, "Case 3: WAM Roller Li/SS (k≈69)")

    # 검증 2-6
    verify_fig3()
    verify_fig4()
    verify_fig6()
    verify_table3()
    verify_scaling()

    print("\n" + "="*60)
    print("  검증 완료. 결과 저장 위치:", OUT_DIR)
    print("="*60)
