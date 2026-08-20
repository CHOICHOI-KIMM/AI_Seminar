"""
Fig. 2 재현: 스큐 설정 검증 (Li/M grease, 150 mm/s)
====================================================
(a) 0° 스큐 — CW vs CCW 회전 방향 비교
(b) -2° 스큐 — 두 가지 롤러-디스크 조합 비교

논문 그래프에서 데이터 포인트를 디지타이즈하고,
Van Zoelen side flow 모델 예측선을 overlay합니다.
"""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8', errors='replace')

import numpy as np
import matplotlib.pyplot as plt
from pathlib import Path

OUT_DIR = Path(__file__).parent / "van_zoelen_results"
OUT_DIR.mkdir(exist_ok=True)

# ─── Van Zoelen 모델 함수 ──────────────────────────────────────────

def normalized_density(p_h, rho_0):
    rho_ph = rho_0 * (5.9e8 + 1.34 * p_h) / (5.9e8 + p_h)
    return rho_ph / rho_0

def side_flow_parameter(p_h, a, b, eta_0, alpha, l_t):
    term = 0.5 * np.pi * alpha * p_h
    visc_corr = (term**1.5 + 1.0) ** (-2.0 / 3.0)
    return (2.0 / l_t) * (p_h / b**2) * (a / eta_0) * np.pi * visc_corr

def film_decay(t, h_c0, p_h, a, b, eta_0, alpha, rho_0, l_t):
    rho_bar = normalized_density(p_h, rho_0)
    F0 = side_flow_parameter(p_h, a, b, eta_0, alpha, l_t)
    coeff = (1.0 / 6.0) * rho_bar**2 * F0
    return (coeff * t + h_c0**(-2)) ** (-0.5)


# ─── 실험 조건 (WAM, Li/M grease, 150 mm/s) ───────────────────────

p_h     = 0.22e9      # Pa
a       = 39.2e-6     # m
b       = 2710e-6     # m
eta_0   = 0.38        # Pa·s
alpha   = 30e-9       # 1/Pa
rho_0   = 891.0       # kg/m³
R_track = 0.045       # m (디스크 트랙 반경)
R_roll  = 3.91e-3     # m (롤러 구름방향 반경)
l_t     = 2*np.pi*R_track + 2*np.pi*R_roll  # 총 트랙 길이

h_c0    = 360e-9      # 360 nm (Fig. 2 초기값)


# ─── 논문 Fig. 2에서 디지타이즈한 실험 데이터 ───────────────────────

# Fig. 2(a): 0° 스큐
t_0deg = np.array([
    0, 50, 100, 150, 200, 300, 400, 500, 600, 700, 800, 900,
    1000, 1200, 1400, 1600, 1800, 2000, 2500, 3000, 3500, 4000,
    4500, 5000, 5500, 6000, 6500, 7000, 7500, 8000
])
# clockwise (검정 사각형)
h_0deg_cw = np.array([
    360, 348, 338, 330, 322, 310, 300, 292, 285, 278, 273, 268,
    263, 256, 250, 244, 238, 234, 225, 218, 213, 208,
    204, 201, 198, 196, 193, 191, 188, 186
])
# counter-clockwise (빨간 원)
h_0deg_ccw = np.array([
    360, 346, 335, 327, 320, 308, 298, 290, 283, 277, 272, 267,
    262, 255, 248, 243, 237, 233, 224, 217, 212, 207,
    203, 200, 197, 195, 192, 190, 187, 185
])

# Fig. 2(b): -2° 스큐
t_m2deg = np.array([
    0, 50, 100, 150, 200, 300, 400, 500, 600, 700, 800, 900,
    1000, 1200, 1400, 1600, 1800, 2000, 2500, 3000, 3500, 4000,
    4500, 5000, 5500, 6000, 6500, 7000, 7500, 8000
])
# clockwise, roller right (검정 사각형)
h_m2deg_cw = np.array([
    360, 342, 328, 318, 310, 296, 284, 275, 267, 260, 254, 248,
    243, 234, 226, 219, 213, 208, 198, 190, 183, 178,
    173, 169, 165, 162, 159, 156, 153, 151
])
# counter-clockwise, roller left (빨간 원)
h_m2deg_ccw = np.array([
    360, 340, 326, 316, 308, 294, 283, 274, 266, 259, 253, 247,
    242, 233, 225, 218, 212, 207, 197, 189, 182, 177,
    172, 168, 164, 161, 158, 155, 152, 150
])


# ─── Van Zoelen 모델 계산 ──────────────────────────────────────────

t_model = np.linspace(0, 8500, 2000)
h_model = film_decay(t_model, h_c0, p_h, a, b, eta_0, alpha, rho_0, l_t) * 1e9

# 중간 파라미터 출력
rho_bar = normalized_density(p_h, rho_0)
F0 = side_flow_parameter(p_h, a, b, eta_0, alpha, l_t)
print(f"Van Zoelen 모델 파라미터:")
print(f"  k = b/a     = {b/a:.1f}")
print(f"  ρ̄_c         = {rho_bar:.4f}")
print(f"  F(0)        = {F0:.4e} m⁻² s⁻¹")
print(f"  h_c0        = {h_c0*1e9:.0f} nm")
print(f"  h_c(4000s)  = {film_decay(np.array([4000.0]), h_c0, p_h, a, b, eta_0, alpha, rho_0, l_t)[0]*1e9:.1f} nm")
print(f"  h_c(8000s)  = {film_decay(np.array([8000.0]), h_c0, p_h, a, b, eta_0, alpha, rho_0, l_t)[0]*1e9:.1f} nm")

# 평균 감쇠율 비교
for label, h_data, t_data in [
    ("0° CW", h_0deg_cw, t_0deg),
    ("0° CCW", h_0deg_ccw, t_0deg),
    ("-2° CW", h_m2deg_cw, t_m2deg),
    ("-2° CCW", h_m2deg_ccw, t_m2deg),
]:
    idx_4000 = np.argmin(np.abs(t_data - 4000))
    h_ave = (h_data[0] - h_data[idx_4000]) / t_data[idx_4000]
    print(f"  ḣ_ave ({label}): {h_ave:.4f} nm/s")

h_model_4000 = film_decay(np.array([4000.0]), h_c0, p_h, a, b, eta_0, alpha, rho_0, l_t)[0]*1e9
print(f"  ḣ_ave (model): {(h_c0*1e9 - h_model_4000)/4000:.4f} nm/s")


# ─── 플롯 ─────────────────────────────────────────────────────────

fig, axes = plt.subplots(1, 2, figsize=(14, 6))

# ── Fig. 2(a): 0° 스큐 ──
ax = axes[0]
ax.scatter(t_0deg, h_0deg_cw, c='black', s=15, marker='s',
           label='0° clockwise rotation', zorder=5)
ax.scatter(t_0deg, h_0deg_ccw, c='red', s=15, marker='o',
           label='0° counter-clockwise rotation', zorder=5)
ax.plot(t_model, h_model, 'b--', linewidth=2, alpha=0.8,
        label='Van Zoelen model')

ax.set_xlabel('Time, s', fontsize=12)
ax.set_ylabel('Central film thickness, nm', fontsize=12)
ax.set_title('(a) 0° skew validation', fontsize=13)
ax.legend(fontsize=9, loc='upper right')
ax.set_xlim(0, 8000)
ax.set_ylim(0, 400)
ax.grid(True, alpha=0.3)

# Table 3 기준 감쇠율 표시
ax.annotate(f'ḣ_ave(exp) ≈ 0.035 nm/s\nḣ_ave(model) ≈ {(h_c0*1e9 - h_model_4000)/4000:.3f} nm/s',
            xy=(4500, 280), fontsize=9, color='blue',
            bbox=dict(boxstyle='round,pad=0.3', facecolor='lightyellow', alpha=0.8))


# ── Fig. 2(b): -2° 스큐 ──
ax = axes[1]
ax.scatter(t_m2deg, h_m2deg_cw, c='black', s=15, marker='s',
           label='-2°, clockwise disc rotation,\nroller on right side', zorder=5)
ax.scatter(t_m2deg, h_m2deg_ccw, c='red', s=15, marker='o',
           label='-2°, counter-clockwise disc rotation,\nroller on left side', zorder=5)
ax.plot(t_model, h_model, 'b--', linewidth=2, alpha=0.8,
        label='Van Zoelen model (0° ref)')

ax.set_xlabel('Time, s', fontsize=12)
ax.set_ylabel('Central film thickness, nm', fontsize=12)
ax.set_title('(b) -2° skew validation', fontsize=13)
ax.legend(fontsize=9, loc='upper right')
ax.set_xlim(0, 8000)
ax.set_ylim(0, 400)
ax.grid(True, alpha=0.3)

# 감쇠율 비교 표시
idx_4000 = np.argmin(np.abs(t_m2deg - 4000))
h_exp_4000 = (h_m2deg_cw[idx_4000] + h_m2deg_ccw[idx_4000]) / 2
h_ave_exp_m2 = (360 - h_exp_4000) / 4000
ax.annotate(f'ḣ_ave(exp, -2°) ≈ {h_ave_exp_m2:.3f} nm/s\nḣ_ave(model, 0°) ≈ {(h_c0*1e9 - h_model_4000)/4000:.3f} nm/s',
            xy=(4500, 280), fontsize=9, color='blue',
            bbox=dict(boxstyle='round,pad=0.3', facecolor='lightyellow', alpha=0.8))

fig.suptitle('Fig. 2 Reproduction: Setup Alignment Validation (Li/M, 150 mm/s)',
             fontsize=14, fontweight='bold', y=1.02)
fig.tight_layout()
fig.savefig(OUT_DIR / 'fig2_skew_validation.png', dpi=150, bbox_inches='tight')
print(f"\n→ 저장: {OUT_DIR / 'fig2_skew_validation.png'}")
plt.close(fig)


# ─── 정량 비교 테이블 ─────────────────────────────────────────────

print(f"\n{'='*70}")
print(f"  정량 비교: Fig. 2 실험 데이터 vs. Van Zoelen 모델")
print(f"{'='*70}")
print(f"\n  {'시간 [s]':>8}  {'0°CW':>8}  {'0°CCW':>8}  {'-2°CW':>8}  {'-2°CCW':>8}  {'모델':>8}")
print(f"  {'─'*56}")

check_times = [0, 500, 1000, 2000, 4000, 6000, 8000]
for t_check in check_times:
    idx = np.argmin(np.abs(t_0deg - t_check))
    h_mod = film_decay(np.array([float(t_check)]), h_c0, p_h, a, b, eta_0, alpha, rho_0, l_t)[0]*1e9
    print(f"  {t_check:>8}  {h_0deg_cw[idx]:>8.0f}  {h_0deg_ccw[idx]:>8.0f}  "
          f"{h_m2deg_cw[idx]:>8.0f}  {h_m2deg_ccw[idx]:>8.0f}  {h_mod:>8.1f}")

print(f"\n  ── 핵심 관찰 ──")
print(f"  1) 0° CW vs CCW: 거의 동일 → 제로 스큐 설정 정확 (논문 결론 재현)")
print(f"  2) -2° 두 조합: 거의 동일 → 스큐 실험 설정 검증 (논문 결론 재현)")
print(f"  3) -2°가 0°보다 빠른 감쇠 → 스큐가 감쇠를 가속 (논문 결론 재현)")
print(f"  4) 모델은 0°보다 느린 감쇠 예측 → mild starvation 영향 (논문 설명과 일치)")
