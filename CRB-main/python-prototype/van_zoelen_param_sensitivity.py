"""
Fig. 3 불일치 원인 추적: 파라미터 민감도 분석
==============================================
어떤 파라미터가 ~1.7× 차이를 만드는지 체계적으로 분석합니다.
"""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')

import numpy as np
import matplotlib.pyplot as plt
from pathlib import Path

OUT_DIR = Path(__file__).parent / "van_zoelen_results"
OUT_DIR.mkdir(exist_ok=True)

# ─── 모델 함수 ────────────────────────────────────────────────────

def F0_eq25(p_h, a, b, eta_0, alpha, l_t):
    term = 0.5 * np.pi * alpha * p_h
    visc = (term**1.5 + 1.0) ** (-2.0/3.0)
    return (2.0/l_t) * (p_h/b**2) * (a/eta_0) * np.pi * visc

def decay_coeff(F0, p_h, rho_0):
    rho_bar = rho_0 * (5.9e8 + 1.34*p_h) / (5.9e8 + p_h) / rho_0
    return (1.0/6.0) * rho_bar**2 * F0

def h_at_t(t, h_c0, C):
    return (C * t + h_c0**(-2)) ** (-0.5)


# ─── 기준 파라미터 ────────────────────────────────────────────────

p_h   = 0.48e9
a     = 141e-6
b     = 141e-6
eta_0 = 0.38
alpha = 30e-9
rho_0 = 891.0
R_ball  = 9.53e-3
R_track = 0.040
h_c0  = 110e-9

# 현재 l_t 해석: 2π(R_ball + R_track)
l_t_current = 2*np.pi*(R_ball + R_track)  # 311.2 mm

# ─── 원본 그래프 목표값 ────────────────────────────────────────────

# 원본 Fig.3 모델선에서 읽은 값
target = {500: 24e-9, 1000: 19e-9}

# 목표 C 역산 (장시간 근사: h ≈ (C×t)^{-0.5})
C_target_500  = 1.0 / (target[500]**2 * 500)   # from t=500
C_target_1000 = 1.0 / (target[1000]**2 * 1000)  # from t=1000
C_target = (C_target_500 + C_target_1000) / 2

C_current = decay_coeff(F0_eq25(p_h, a, b, eta_0, alpha, l_t_current), p_h, rho_0)

print("=" * 65)
print("  Fig. 3 불일치 원인 추적")
print("=" * 65)
print(f"\n  현재 감쇠 계수 C = {C_current:.4e}")
print(f"  목표 감쇠 계수 C ≈ {C_target:.4e}")
print(f"  필요 보정 비율: {C_target/C_current:.2f}×")


# ═══════════════════════════════════════════════════════════════════
# 1. l_t 해석 변형
# ═══════════════════════════════════════════════════════════════════

print(f"\n{'─'*65}")
print(f"  1. l_t 해석에 따른 F(0) 변화")
print(f"{'─'*65}")

l_t_options = {
    "2π(R_ball+R_track) [현재]":        2*np.pi*(R_ball+R_track),
    "2πR_track only (디스크만)":         2*np.pi*R_track,
    "2πR_ball only (볼만)":             2*np.pi*R_ball,
    "2(R_ball+R_track) [리터럴]":        2*(R_ball+R_track),
    "π(R_ball+R_track) [반둘레]":        np.pi*(R_ball+R_track),
}

print(f"\n  {'해석':>35}  {'l_t [mm]':>10}  {'F(0)':>12}  {'C':>12}  {'h(500s)':>8}  {'비율':>6}")
print(f"  {'─'*90}")

for label, lt in l_t_options.items():
    F = F0_eq25(p_h, a, b, eta_0, alpha, lt)
    C = decay_coeff(F, p_h, rho_0)
    h500 = h_at_t(500, h_c0, C) * 1e9
    ratio = C / C_current
    marker = " ◄" if abs(h500 - 24) < 3 else ""
    print(f"  {label:>35}  {lt*1e3:>10.2f}  {F:>12.4e}  {C:>12.4e}  {h500:>8.1f}  {ratio:>5.1f}×{marker}")

print(f"  {'목표':>35}  {'─':>10}  {'─':>12}  {C_target:>12.4e}  {'24.0':>8}  {C_target/C_current:>5.1f}×")


# ═══════════════════════════════════════════════════════════════════
# 2. η₀ 변형 (l_t 고정)
# ═══════════════════════════════════════════════════════════════════

print(f"\n{'─'*65}")
print(f"  2. η₀ 변형 (F(0) ∝ 1/η₀)")
print(f"{'─'*65}")

print(f"\n  {'η₀ [Pa·s]':>12}  {'F(0)':>12}  {'C':>12}  {'h(500s)':>8}  {'비율':>6}")
print(f"  {'─'*55}")

for eta_test in [0.38, 0.30, 0.25, 0.22, 0.20, 0.15]:
    F = F0_eq25(p_h, a, b, eta_test, alpha, l_t_current)
    C = decay_coeff(F, p_h, rho_0)
    h500 = h_at_t(500, h_c0, C) * 1e9
    marker = " ◄" if abs(h500 - 24) < 2 else ""
    print(f"  {eta_test:>12.2f}  {F:>12.4e}  {C:>12.4e}  {h500:>8.1f}  {C/C_current:>5.1f}×{marker}")


# ═══════════════════════════════════════════════════════════════════
# 3. n_c (접촉 수) 변형
# ═══════════════════════════════════════════════════════════════════

print(f"\n{'─'*65}")
print(f"  3. n_c (접촉 수) 변형 — F(0) = n_c × F_k(0)")
print(f"{'─'*65}")

F_single = F0_eq25(p_h, a, b, eta_0, alpha, l_t_current)
print(f"\n  {'n_c':>6}  {'F(0)':>12}  {'C':>12}  {'h(500s)':>8}  {'h(1000s)':>8}  {'비율':>6}")
print(f"  {'─'*60}")

for nc in [1, 2, 3]:
    F = nc * F_single
    C = decay_coeff(F, p_h, rho_0)
    h500 = h_at_t(500, h_c0, C) * 1e9
    h1000 = h_at_t(1000, h_c0, C) * 1e9
    marker = " ◄" if abs(h500 - 24) < 3 else ""
    print(f"  {nc:>6}  {F:>12.4e}  {C:>12.4e}  {h500:>8.1f}  {h1000:>8.1f}  {C/C_current:>5.1f}×{marker}")

print(f"  {'목표':>6}  {'─':>12}  {C_target:>12.4e}  {'24.0':>8}  {'19.0':>8}  {C_target/C_current:>5.1f}×")


# ═══════════════════════════════════════════════════════════════════
# 4. 복합 변형
# ═══════════════════════════════════════════════════════════════════

print(f"\n{'─'*65}")
print(f"  4. l_t 해석 + η₀ 복합 효과")
print(f"{'─'*65}")

combos = [
    ("현재 (l_t=311mm, η₀=0.38)", l_t_current, 0.38),
    ("l_t=디스크만, η₀=0.38",     2*np.pi*R_track, 0.38),
    ("l_t=311mm, η₀=base_oil",   l_t_current, 0.25),
    ("l_t=디스크만, η₀=0.30",     2*np.pi*R_track, 0.30),
]

print(f"\n  {'조합':>30}  {'h(500s)':>8}  {'h(1000s)':>8}  {'h(1500s)':>8}  {'비율':>6}")
print(f"  {'─'*65}")

for label, lt, eta in combos:
    F = F0_eq25(p_h, a, b, eta, alpha, lt)
    C = decay_coeff(F, p_h, rho_0)
    h500 = h_at_t(500, h_c0, C)*1e9
    h1000 = h_at_t(1000, h_c0, C)*1e9
    h1500 = h_at_t(1500, h_c0, C)*1e9
    print(f"  {label:>30}  {h500:>8.1f}  {h1000:>8.1f}  {h1500:>8.1f}  {C/C_current:>5.1f}×")

print(f"  {'원본 Fig.3 모델선 (추정)':>30}  {'~24':>8}  {'~19':>8}  {'~15':>8}  {C_target/C_current:>5.1f}×")


# ═══════════════════════════════════════════════════════════════════
# 플롯: 감쇠 곡선 비교
# ═══════════════════════════════════════════════════════════════════

t_200 = np.array([8,15,22,30,40,50,60,75,90,105,120,140,160,180,200,225,250,280,
                  310,350,400,450,500,560,630,700,780,860,950,1050,1150,1250,1350,1450,1550])
h_200 = np.array([108,95,85,78,72,65,60,55,50,47,44,42,39,37,36,34,32,31,
                  30,28,27,26,25,24.5,24,23,22.5,22,21.5,21,20.5,20,20,19.5,19])

t_300 = np.array([8,15,22,30,40,50,60,75,90,105,120,140,160,180,200,225,250,280,
                  310,350,400,450,500,600,700,800,900,1000,1100,1200,1300,1350,1400,1450,1500])
h_300 = np.array([105,90,78,68,62,57,53,49,45,43,41,38,36,34,33,31,30,29,
                  28,27,26,25.5,25,24,25,28,30,28,32,30,38,37,40,35,32])

t_m = np.linspace(0.1, 2000, 2000)

fig, ax = plt.subplots(1, 1, figsize=(10, 7))

ax.scatter(t_200, h_200, c='black', s=20, marker='s', label='200 mm/s (exp)', zorder=5)
ax.scatter(t_300, h_300, c='red', s=20, marker='o', label='300 mm/s (exp)', zorder=5)

cases = [
    ("Eq.25 현재 (l_t=311mm, η₀=0.38)", l_t_current, 0.38, 1, 'green',   '--', 1.5),
    ("l_t=disc only (251mm)",            2*np.pi*R_track, 0.38, 1, 'orange',  '--', 1.5),
    ("n_c=2",                            l_t_current, 0.38, 2, 'blue',    '-',  2.5),
    ("η₀=0.25 (base oil est.)",          l_t_current, 0.25, 1, 'purple',  '--', 1.5),
]

for label, lt, eta, nc, color, ls, lw in cases:
    F = nc * F0_eq25(p_h, a, b, eta, alpha, lt)
    C = decay_coeff(F, p_h, rho_0)
    h = h_at_t(t_m, h_c0, C) * 1e9
    ax.plot(t_m, h, color=color, linestyle=ls, linewidth=lw, label=label)

ax.set_xlabel('Time, s', fontsize=12)
ax.set_ylabel(r'Central film thickness $h_c$, nm', fontsize=12)
ax.set_title('Fig. 3: Parameter sensitivity — which matches the paper?', fontsize=13)
ax.legend(fontsize=9, loc='upper right')
ax.set_xlim(0, 2000)
ax.set_ylim(0, 120)
ax.grid(True, alpha=0.3)
fig.tight_layout()
fig.savefig(OUT_DIR / 'fig3_param_sensitivity.png', dpi=150)
print(f"\n  → 저장: {OUT_DIR / 'fig3_param_sensitivity.png'}")
plt.close(fig)
