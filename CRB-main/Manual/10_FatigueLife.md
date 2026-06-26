# Chapter 10: Fatigue Life Calculation (ISO 16281:2025)

## 10.1 Overview

베어링 피로 수명은 **ISO 16281:2025 lamina-level 프레임워크**를 기반으로 계산된다.

### 10.1.1 계산 워크플로우

```
┌─────────────────────────────────────────────────────┐
│  INPUT                                               │
│  Geometry, Load, Speed, Lubricant, Contamination     │
└──────────────────────┬──────────────────────────────┘
                       ▼
┌──────────────────────────────────────────────────────┐
│  1. BEARING EQUILIBRIUM  (Chapter 9)                 │
│     5-DOF NR → per-roller loads Q_j, per-slice q_k   │
└──────────────────────┬──────────────────────────────┘
                       ▼
┌──────────────────────────────────────────────────────┐
│  2. LAMINA CAPACITY  (ISO 16281)                     │
│     Q_ci/Q_ce from C_r → q_ci/q_ce                  │
└──────────────────────┬──────────────────────────────┘
                       ▼
┌──────────────────────────────────────────────────────┐
│  3. LAMINA EQUIVALENT LOADS                          │
│     q_eik = (1/Z × Σ (q_k·Δx)^p)^(1/p)             │
│     q_eek = (1/Z × Σ (q_k·Δx)^p)^(1/p)             │
│     ※ q_k [N/mm] × slice_width [mm] → [N] 변환     │
└──────────────────────┬──────────────────────────────┘
                       ▼
┌──────────────────────────────────────────────────────┐
│  4. BASIC REFERENCE LIFE  L_10r  (Eq. 29)           │
│     L_10r = (Σ_k D_k)^(-8/9)                        │
│     D_k = (q_ci/q_eik)^(-9/2) + (q_ce/q_eek)^(-9/2)│
└──────────────────────┬──────────────────────────────┘
                       ▼
┌──────────────────────────────────────────────────────┐
│  5. PER-LAMINA MODIFICATION  (Eq. 31, 33)           │
│     P_sk = 0.323·Z·cosα·n_s × {loads}^(2/9)         │
│     a_ISOk = f(e_C·C_u / P_sk, κ)                   │
│     L_nmr = (Σ_k a_ISOk^(-9/8)·D_k)^(-8/9)         │
└──────────────────────┬──────────────────────────────┘
                       ▼
┌──────────────────────────────────────────────────────┐
│  OUTPUT                                              │
│  L_10r, L_nmr, P_ref, a_ISO, per-lamina detail      │
│  Damage%, Reliability%, per-ring lives               │
└──────────────────────────────────────────────────────┘
```

### 10.1.2 Edge Stress Correction (f[j,k])

ISO 16281:2025 Section 5.3.4에서는 edge stress 보정 함수 f[j,k]를 정의한다:
- **Annex C.1**: non-Hertzian 접촉 모델 기반 상세 계산 (preferred)
- **Annex C.2**: 근사 함수 (first approximation)

현재 구현에서는 **f[j,k] = 1** (Unity)을 사용한다:
- Crowned 프로필: 솔버가 slice별 독립 Hertz 접촉을 계산하므로, 프로파일 효과가 이미 q_k 분포에 반영됨
- Uncrowned 프로필: non-Hertzian 접촉 모델이 구현되지 않아 C.1 적용 불가

> **참고**: 이전 버전의 Stirling method와 StressRiseMethod 옵션은 제거됨. Stirling(2023)의 lamina capacity는 ISO 16281과 동일하며, stress rise 옵션은 독립 slice Hertz 모델에서 의미가 없음.

---

## 10.2 Input Parameters

### 10.2.1 Dynamic Load Rating C_r

ISO 281:2007, Eq. (7):

$$
C_r = b_m \cdot f_c \cdot (L_{we} \cos\alpha)^{7/9} \cdot Z^{3/4} \cdot D_{we}^{29/27}
$$

- `b_m ≈ 1.1`: 재료/제조 품질 계수
- `D_we = (D_we_max + D_we_min) / 2`: 평균 롤러 직경 [mm]
- `f_c`: ISO 281 Table 6 기하학적 계수

사용자가 `c_r_kn`을 직접 지정하면 이 계산을 대체한다.

### 10.2.2 ISO 281 f_c Factor (롤러 베어링)

ISO 281:2007 Table 6 데이터의 2차 다항식 피팅:

$$
f_c = 93.76 - 60\gamma + 15\gamma^2
$$

여기서 `γ = D_we cos(α) / d_pw`. 유효 범위: γ ∈ [0.01, 0.35], 클램핑: 60~95.

| γ | Table 6 값 | 다항식 값 | 오차 |
|---|-----------|----------|------|
| 0.05 | 90.8 | 90.9 | +0.1% |
| 0.10 | 87.9 | 87.9 | 0.0% |
| 0.15 | 85.1 | 85.1 | 0.0% |
| 0.20 | 82.4 | 82.4 | 0.0% |
| 0.25 | 79.7 | 79.7 | 0.0% |

### 10.2.3 Static Load Rating C₀ᵣ & Fatigue Load Limit C_u

**C₀ᵣ**: ISO 76:2006 Eq.(7)에서 자동 계산 또는 사용자 지정.

**C_u** (ISO 281:2007 Annex B.3.3.3 — 롤러 베어링 간이 공식):

$$
C_u = \frac{C_{0r}}{8.2}
$$

> **배경** (ISO/TR 1281-2:2008 §9.2):
> - C₀ 기준 접촉응력: 4,000 MPa (ISO 76)
> - C_u 기준 피로 한계 응력: 1,500 MPa (ISO 281:2007 §9.3.1)
> - 선접촉에서 Q ∝ p²이므로:
>
> $$\frac{C_0}{C_u} = \left(\frac{4000}{1500}\right)^2 \cdot \frac{0.2}{0.2453} \cdot \left(\frac{1500}{1250}\right)^2 \approx 8.2$$
>
> 실제 범위는 7.2~9.5이며, 대표값 8.2를 사용한다.

### 10.2.4 Dynamic Equivalent Load P

ISO 281: `P = X·F_r + Y·F_a`

| Condition | X | Y |
|-----------|---|---|
| `F_a / F_r ≤ e` (e = 1.5·tan α) | 1.0 | 0.0 |
| `F_a / F_r > e` | 0.4 | 0.4 / tan(α) |
| Pure axial (F_r ≈ 0) | 0.0 | 1 / tan(α) |

---

## 10.3 Lubrication Parameters

### 10.3.1 Viscosity at Operating Temperature (ASTM D341 Walther)

2점 보간 (ν₄₀, ν₁₀₀ 입력):

$$
\ln\ln(\nu + 0.7) = a + b \cdot \ln(T + 273.15)
$$

### 10.3.2 Reference Viscosity ν₁ (ISO 281:2007 Eq. 28-29)

$$
\nu_1 = \begin{cases}
\frac{45000}{n^{0.83} \sqrt{d_m}} & n < 1000 \text{ rpm} \quad \text{(Eq. 28)} \\[6pt]
\frac{4500}{\sqrt{n} \sqrt{d_m}} & n \geq 1000 \text{ rpm} \quad \text{(Eq. 29)}
\end{cases}
$$

여기서 d_m = d_pw (피치원 직경) [mm].

### 10.3.3 Viscosity Ratio κ

$$
\kappa = \frac{\nu_{actual}}{\nu_1}
$$

### 10.3.4 Contamination Factor e_C (ISO 281:2007 Annex A)

**자동 계산 (e_c = 0)** — ISO 281:2007 Annex A, Eq.(A.1):

$$
e_C = \min\left(C_1 \cdot \kappa^{0.68} \cdot D_{pw}^{0.55},\ 1\right) \times \left(1 - C_2 \cdot D_{pw}^{-1/3}\right)
$$

상수 C₁, C₂는 **오일 공급 방식**과 **ISO 4406 오염도 등급**에 의해 결정된다.

#### Oil Bath / Splash 윤활

| Cleanliness (ISO 4406) | 설명 | C₁ | C₂ |
|------------------------|------|-----|-----|
| —/13/10 | 고청정 (β₆≥200, β₁₀≥200) | 0.1710 | 0.3796 |
| **—/15/12** | **일반 산업 (기본값)** | **0.0864** | **0.6796** |
| —/17/14 | 약간 오염 (β₂₅≥75) | 0.0411 | 1.1410 |
| —/19/16 | 심한 오염 (필터 없음) | 0.0178 | 1.8570 |
| —/21/18 | 매우 심한 오염 | 0.0085 | 2.6620 |

#### 순환 오일 + 온라인 필터

| Cleanliness (ISO 4406) | C₁ | C₂ |
|------------------------|-----|-----|
| —/13/10 | 0.2288 | 0.2700 |
| —/15/12 | 0.1148 | 0.4920 |
| —/17/14 | 0.0617 | 0.8310 |
| —/19/16 | 0.0297 | 1.3560 |
| —/21/18 | 0.0133 | 1.9970 |

#### 그리스 윤활

| Cleanliness (ISO 4406) | C₁ | C₂ |
|------------------------|-----|-----|
| —/13/10 | 0.1500 | 0.4500 |
| —/15/12 | 0.0750 | 0.7500 |
| —/17/14 | 0.0380 | 1.2500 |
| —/19/16 | 0.0165 | 2.0000 |
| —/21/18 | 0.0075 | 2.8500 |

**수동 입력 (e_c > 0)**: 사용자가 직접 e_C 값을 지정한다. 자동 계산을 사용하지 않는다.

> **MASTA 검증**: Oil bath + Normal (—/15/12), κ=3.413, D_pw=51.56:
> e_C = min(0.0864 × 3.413^0.68 × 51.56^0.55, 1) × (1 − 0.6796 × 51.56^(−1/3)) = **0.8178** (MASTA: 0.8174, 차이 0.05%)

---

## 10.4 Life Modification Factor a_ISO

### 10.4.1 Bearing-Level a_ISO (ISO 281:2007)

ISO 281:2007 Tables 6-8 (closed-form piecewise fit):

$$
a_{ISO} = 0.1 \times \left[1 - \left(1.5859 - \frac{C_1}{\kappa^{C_2}}\right) \cdot (e_C \cdot C_u/P)^{0.4}\right]^{-9.185}
$$

κ 범위별 상수:

| κ 범위 | C₁ | C₂ |
|--------|----|----|
| < 0.4 | 1.3993 | 0.054381 |
| 0.4 ≤ κ < 1.0 | 1.2348 | 0.19087 |
| 1.0 ≤ κ ≤ 4.0 | 1.2348 | 0.071739 |
| > 4.0 | κ=4 값 사용 | (상한 플래토) |

범위: `a_ISO ∈ [0.1, 50]`

### 10.4.2 Per-Lamina a_ISOk (ISO 16281:2025 Formula 33)

ISO 16281:2025에서는 a_ISO를 **lamina별로 개별 계산**한다. 각 lamina k에 대해:

**Step 1**: Per-lamina reference load P_sk (Formula 31):

$$
P_{sk} = 0.323 \cdot Z \cdot \cos\alpha \cdot n_s \cdot \left\{ \frac{q_{eik}^{9/2} + \left(1.038 \cdot \frac{q_{ci}}{q_{ce}} \cdot q_{eek}\right)^{9/2}}{1 + \left(1.038 \cdot \frac{q_{ci}}{q_{ce}}\right)^{9/2}} \right\}^{2/9}
$$

**Step 2**: Per-lamina a_ISOk:

$$
a_{ISOk} = f\left(\frac{e_C \cdot C_u}{P_{sk}},\ \kappa\right)
$$

§10.4.1의 동일한 공식을 사용하되, P 대신 P_sk를 대입한다.

**물리적 의미**: 하중 집중 lamina(edge)는 P_sk 높음 → C_u/P_sk 낮음 → a_ISOk 낮음. 경하중 lamina(center)는 a_ISOk 높음.

---

## 10.5 Raceway Capacity Factors

내/외륜의 동적 부하 용량 인자 (Lundberg-Palmgren, 선접촉):

$$
f_i = (1-\gamma)^{29/27} \cdot (1+\gamma)^{-1/4}
$$

$$
f_o = (1+\gamma)^{29/27} \cdot (1-\gamma)^{-1/4}
$$

**물리적 의미**: 내륜은 접촉 곡률 반경이 작아 Hertz 응력이 높고 용량이 낮다(f_i < f_o).

> **중요**: 이 인자들은 **정규화하지 않고** 원시(raw) 값으로 사용한다. 카탈로그 C_r에서 per-roller 용량을 산출할 때 b_m으로 나누어 이론적 기반을 맞춘다.

---

## 10.6 Lamina-Level Life Calculation

### 10.6.1 Per-Roller Capacity (Q_ci, Q_ce)

$$
Q_{ci} = \frac{C_r}{b_m \cdot Z \cdot \sin\alpha} \times f_i, \quad Q_{ce} = \frac{C_r}{b_m \cdot Z \cdot \sin\alpha} \times f_o
$$

### 10.6.2 Per-Lamina Capacity (q_ci, q_ce)

$$
q_{ci} = Q_{ci} \times \left(\frac{1}{n_s}\right)^{7/9}, \quad q_{ce} = Q_{ce} \times \left(\frac{1}{n_s}\right)^{7/9}
$$

### 10.6.3 Lamina Equivalent Loads (Eq. 5.6, 5.7)

접촉 솔버 출력 q_k [N/mm]를 per-lamina force [N]로 변환하여 사용한다:

$$
q_{e,i,k} = \left(\frac{1}{Z} \sum_{j=1}^{Z} (q_{k,j} \cdot \Delta x)^4\right)^{1/4}
$$

$$
q_{e,o,k} = \left(\frac{1}{Z} \sum_{j=1}^{Z} (q_{k,j} \cdot \Delta x)^{4.5}\right)^{1/4.5}
$$

여기서 `Δx = L_we / n_s` [mm] (lamina 폭). 내륜(회전) 지수 = 4, 외륜(정지) 지수 = 4.5.

> **단위 주의**: 솔버의 q_k는 선하중 [N/mm]이다. ISO 16281의 lamina capacity q_ci는 per-lamina force [N]이므로, q_k에 slice_width를 곱하여 단위를 맞춘다.

### 10.6.4 Lamina Life

$$
L_{k,inner} = \left(\frac{q_{ci} / f_{s,k}}{q_{e,i,k}}\right)^{4.5}, \quad L_{k,outer} = \left(\frac{q_{ce} / f_{s,k}}{q_{e,o,k}}\right)^{4.5}
$$

### 10.6.5 Basic Reference Rating Life L₁₀ᵣ (Eq. 29)

$$
L_{10r} = \left[\sum_{k=1}^{n_s} \left(\frac{1}{L_{k,inner}} + \frac{1}{L_{k,outer}}\right)\right]^{-8/9}
$$

### 10.6.6 Modified Reference Rating Life L_nmr (Formula 33)

**Per-lamina a_ISOk**를 적용한 수정 수명:

$$
L_{nmr} = a_1 \left( \sum_{k=1}^{n_s} \left\{ a_{ISOk}^{-9/8} \left[ \left(\frac{q_{ci}}{q_{eik}}\right)^{-9/2} + \left(\frac{q_{ce}}{q_{eek}}\right)^{-9/2} \right] \right\} \right)^{-8/9}
$$

베어링 전체 유효 a_ISO = L_nmr / L_10r 로 보고한다.

### 10.6.7 Per-Ring Life (보고용)

$$
L_{inner} = \left(\sum_k L_{k,inner}^{-9/8}\right)^{-8/9}, \quad L_{outer} = \left(\sum_k L_{k,outer}^{-9/8}\right)^{-8/9}
$$

### 10.6.8 Reference Load P_ref

두 가지 방식의 P_ref를 동시 출력한다:

#### (a) Back-calculated P_ref

ISO 281의 L₁₀ = (C_r/P)^(10/3) 관계를 ISO 16281 L₁₀ᵣ로 역산:

$$
P_{ref} = \frac{C_r}{L_{10r}^{3/10}}
$$

- L₁₀ᵣ는 **ISO 16281** 라미나 기반 수명 (ISO 281 L₁₀와 다름)
- "하중이 균일했다면 이 L₁₀ᵣ를 주었을 등가하중"
- 참고: MASTA는 같은 공식이지만 슬라이스 수/접촉 모델 차이로 수치가 다를 수 있음

#### (b) Damage-weighted P_ref (진단 지표)

라미나별 P_sk를 손상 기여도로 가중 평균:

$$
P_{ref,dmg} = \frac{\sum_k P_{sk} \cdot D_k}{\sum_k D_k}
$$

여기서 $D_k = L_{k,inner}^{-1} + L_{k,outer}^{-1}$ (lamina damage contribution).

- 에지 로딩/미스얼라인먼트에 민감한 진단 지표
- P_ref,dmg / P_ref > 1 이면 하중 불균일 경고
- 크라운 최적화, 미스얼라인먼트 평가 시 유용

**P_ref ratio** = P_ref,dmg / P_ref:
- ratio = 1.0: 하중이 균일 (이상적 크라운)
- ratio > 1.0: 에지 로딩/미스얼라인먼트로 하중 집중
- ratio가 클수록 하중 분포 불균일이 심함

두 P_ref 모두 **보고용 참고값**이며, 실제 수명 계산(L₁₀ᵣ, L_nmr)에는 영향 없음.

---

## 10.7 Method 1: ISO 16281 (Standard)

ISO 16281:2025 표준의 lamina-level 수명 계산.

- **Lamina capacity**: C_r에서 역산 (§10.6.1–10.6.2)
- **Edge stress**: f[j,k] = 1 (§10.1.2 참조)
- **Modified life**: Per-lamina a_ISOk (Formula 33)

---

## 10.8 Exponent Summary (Lundberg-Palmgren)

| Level | Inner equiv | Outer equiv | Life exp | Combination |
|-------|------------|------------|----------|-------------|
| **Bearing** (ISO 281) | 10/3 | max | 10/3 | ring Weibull 9/8 |
| **Lamina** (ISO 16281) | **4** | **4.5** | **4.5** | **-8/9** |

---

## 10.11 Design Life & Reliability

### 10.11.1 Design Life

Operating Conditions에서 `design_life_hours` (기본 100시간)를 설정한다.

### 10.11.2 Damage %

$$
\text{Damage}_{\%} = \frac{T_{design}}{L_{hours}} \times 100
$$

### 10.11.3 Weibull Reliability

Lundberg-Palmgren 선접촉 Weibull 분포 (slope e = 10/9):

$$
R(T) = \exp\left(-\left(\frac{T}{\eta}\right)^{10/9}\right), \quad \eta = \frac{L_{10,hours}}{(-\ln 0.9)^{9/10}}
$$

L₁₀ 수명에서 신뢰도 = 90% (비신뢰도 = 10%).

---

## 10.12 Output Summary

### 10.12.1 Result Fields

| Field | Unit | Description |
|-------|------|-------------|
| `method` | - | Iso16281 |
| `l_10_basic` | 10⁶ rev | Basic reference rating life L₁₀ᵣ |
| `l_nm_hours` | hours | Modified reference rating life L_nmr |
| `l_10_inner` | 10⁶ rev | Inner ring basic reference life |
| `l_10_outer` | 10⁶ rev | Outer ring basic reference life |
| `weakest_lamina` | - | Index of weakest slice |
| `a_iso` | - | Effective bearing-level a_ISO (= L_nmr / L_10r) |
| `kappa` | - | Viscosity ratio κ |
| `c_dyn` | kN | Dynamic load rating C_r |
| `p_equiv` | kN | Dynamic equivalent load P (ISO 281 X·Fr + Y·Fa) |
| `p_ref` | kN | Back-calculated: C_r / L₁₀ᵣ(ISO16281)^(3/10) |
| `p_ref_damage` | kN | Damage-weighted avg P_sk (에지 로딩 진단 지표) |

### 10.12.2 Per-Lamina Detail

| Field | Unit | Description |
|-------|------|-------------|
| `q_equiv_inner` | N | Lamina equivalent load (inner ring) |
| `q_equiv_outer` | N | Lamina equivalent load (outer ring) |
| `l_10_inner` | 10⁶ rev | Lamina basic life (inner) |
| `l_10_outer` | 10⁶ rev | Lamina basic life (outer) |
| `p_sk` | kN | Per-lamina reference load (ISO 16281 Eq.31) |
| `a_iso_k_inner` | - | Per-lamina life modification factor (inner raceway) |
| `a_iso_k_outer` | - | Per-lamina life modification factor (outer raceway) |

### 10.12.3 Life Tab Display (MASTA-style)

Life 탭에서 확인할 수 있는 상세 항목:

| 구분 | 항목 |
|------|------|
| **ISO 16281 Rating** | Duration(hr), Speed(rpm), n_s, P_ref(back-calc.), P_ref(damage-wt.), P(ISO 281), a_ISO |
| **Applied Loads** | F_r, F_x, F_y, F_a, F_a/F_r |
| **Lamina Capacity** | q_ci, q_ce [kN] |
| **Roller Capacity** | Q_ci, Q_ce [kN], f_ci, f_co |
| **Basic Reference Life** | L₁₀ᵣ, L₁₀ᵣₕ(hr), inner/outer(Mrev + hr), weakest, Damage%, Reliability%, Unreliability% |
| **Modified Reference Life** | L₁₀ₘᵣ, L₁₀ₘᵣₕ(hr), Damage%, Reliability%, Unreliability% |
| **Modification Factors** | a₁, a_ISO (bearing-level), a_ISO min/max (per-lamina range), e_C (auto/manual), C_u/P, Weibull e |
| **Load Rating** | C_r, C₀ᵣ, P, C_r/P, C_u (= C₀/8.2), C_u/P |
| **ISO 281 Load Factors** | e(demarcation), F_a/F_r, exceeds e?, X, Y |
| **Lubrication** | Type, ν₄₀, ν₁₀₀, T_op, ν(T_op), ν₁(ref), κ, φ_s, ρ_oil |
| **Capacity / Geometry** | b_m, f_c, γ |

---

## 10.13 MASTA Validation Summary

NSK HR30306J (C_r=59.5, C₀ᵣ=60 kN, F_r=30 kN, F_a=9 kN, n=1500 rpm) 기준.

### 10.13.1 Capacity & Lubrication (하중 분포 무관)

| 항목 | TRB | MASTA | 차이 | 판정 |
|------|-----|-------|------|------|
| Q_ci (kN) | 14.14 | 14.16 | -0.1% | 일치 |
| Q_ce (kN) | 24.18 | 24.20 | -0.1% | 일치 |
| C_u (kN) | 7.293 | 7.317 | -0.3% | 일치 |
| ν₁ (mm²/s) | 16.18 | 16.18 | 0% | 완벽 |
| ν(68°C) (mm²/s) | 54.26 | 55.23 | -1.8% | 보간 차이 |
| κ | 3.354 | 3.413 | -1.8% | ν 차이에 기인 |
| e_C (auto) | 0.8174 | 0.8174 | 0% | 완벽 |

### 10.13.2 Per-Roller Loads

| Roller | Angle | TRB Q (N) | MASTA Q (N) | 차이 |
|--------|-------|-----------|-------------|------|
| 1 | 0° | 8510.6 | 8487 | +0.3% |
| 2 | 25.7° | 7771.9 | 7757 | +0.2% |
| 3 | 51.4° | 5758.7 | 5773 | -0.2% |
| 4 | 77.1° | 3134.3 | 3175 | -1.3% |
| 5 | 102.9° | 974.8 | 949 | +2.7% |
| 6 | 128.6° | 2.5 | 0 | 경계 |

롤러별 총 하중 Q는 1~3% 이내 일치.

### 10.13.3 ISO 16281 Life Results (n_s = 40)

| 항목 | TRB | MASTA | 차이 | 비고 |
|------|-----|-------|------|------|
| q_ci (kN) | 0.802 | 0.803 | -0.1% | |
| q_ce (kN) | 1.372 | 1.373 | -0.1% | |
| L₁₀ᵣ (Mrev) | **20.81** | **19.43** | **+7.1%** | 아래 분석 참조 |
| a_ISO | 1.139 | 1.154 | -1.3% | |
| L₁₀ₘᵣ (Mrev) | 23.70 | 22.41 | +5.8% | |
| P_ref,bc (kN) | 23.94 | 29.44 | - | P_ref 정의 다름 |
| P_ref,dmg (kN) | 31.70 | - | - | 우리 독자 지표 |
| P_ref ratio | 1.324 | - | - | 하중 불균일 지표 |

### 10.13.4 L₁₀ᵣ 차이 원인 분석

**롤러별 총 하중(Q)은 거의 동일하지만 L₁₀ᵣ가 7% 차이나는 이유**:

ISO 16281의 L₁₀ᵣ는 **전 라미나 손상 합산**으로 계산된다:

$$L_{10r} = \left\{ \sum_{k=1}^{n_s} \left[ \left(\frac{q_{ci}}{q_{eik}}\right)^{-9/2} + \left(\frac{q_{ce}}{q_{eek}}\right)^{-9/2} \right] \right\}^{-8/9}$$

지수 9/2 = 4.5로 인해 **라미나별 하중 분포의 비균일성에 극도로 민감**하다.

동일한 Q에서도 라미나 하중 분포가 달라지면 q_eik가 달라진다:

```
TRB (tilting 없음):       MASTA (θy ≈ 0.24 mrad):
┌────────────────────┐    ┌────────────────────┐
│ ██████████████████ │    │ ██████████████████████│
│ (균등 분포)         │    │ (한쪽 에지에 집중)     │
└────────────────────┘    └────────────────────┘
에지 피크 낮음 → L₁₀ᵣ↑   에지 피크 높음 → L₁₀ᵣ↓
```

**비선형 증폭 효과**: 에지 라미나 하중이 213N→300N으로 40% 증가하면,
lamina damage는 (300/213)^4.5 = **4.6배** 증가한다.

| 차이 요인 | 영향 방향 | 추정 크기 |
|----------|----------|----------|
| **Tilting (θy ≈ 0.24 mrad)** | 우리 수명이 더 긴 쪽 | +4~6% |
| **프로파일 차이** | 불확실 | +1~3% |
| 내/외륜 독립 접촉 모델 | 미미 | <1% |
| ν(T_op) 보간 차이 → κ 차이 | a_ISO에만 영향 | ~1% |
| **합계** | | **~+7%** |

> **핵심**: Q(롤러 총하중)가 일치해도 L₁₀ᵣ는 다를 수 있다. ISO 16281은 전 라미나를 합산하는 모델이므로, 같은 Q에서도 라미나 내부 분포(tilting, 프로파일)에 의해 수명이 결정적으로 달라진다. 이것이 ISO 281(단일점 모델)과 ISO 16281(라미나 모델)의 근본적 차이이다.

### 10.13.5 P_ref 정의 차이

| | TRB P_ref(bc) | MASTA P_ref |
|--|---------------|-------------|
| 공식 | C_r / L₁₀ᵣ(**ISO 16281**)^(3/10) | C_r / L₁₀(**ISO 281**)^(3/10) 추정 |
| L₁₀ 기준 | 라미나 기반 ~20.81 Mrev | 단순 (C/P)^(10/3) = 9.80 Mrev |
| 결과 | 23.94 kN | 29.44 kN |

MASTA의 P_ref ≈ 29.44는 ISO 281 L₁₀에서 역산한 값. ISO 16281 L₁₀ᵣ가 ISO 281 L₁₀보다 약 2배 긴 것이 수치 차이의 원인이다.

### 10.13.6 변경 이력

- **2026-03-18**: MASTA 비교 전면 갱신. 롤러별 Q 비교 추가, L₁₀ᵣ 차이 원인(tilting/라미나 분포) 분석, P_ref 이중 출력, n_s=40 결과 반영.
- **2026-03-17**: ν₁ 공식 분기 오류 수정 (n≥1000에서 Eq.28→Eq.29). 수정 전 ν₁=14.48, κ=3.75로 10% 과대 → 수정 후 MASTA와 일치.
