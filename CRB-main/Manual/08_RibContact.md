# Chapter 8: Large-End Rib Contact

## 8.1 Overview

TRB의 대단 리브(large-end rib, guide flange)는 롤러의 축방향 위치를 제한하고, 축하중의 일부를 전달한다. 롤러 대단 구면(sphere)과 리브 필렛(concave) 간의 접촉은 **타원형 Hertz 점접촉**으로 해석한다.

```
     Rib (concave, R_rib)
  ╱──────────────╲
  │              │
  │   ●          │ ← roller sphere (convex, R_sph)
  │              │
  ╲──────────────╱
```

## 8.2 Equivalent Radii

롤러 대단 구면(볼록)과 리브 토로이달 면(자오면: 오목, 원주면: 볼록) 간의 접촉.

리브 표면은 회전체(surface of revolution)이며, 두 주곡률 방향에서 다른 곡률을 가진다:
- **자오면 (meridional)**: 리브 필렛 — 오목, 반경 R_rib
- **원주면 (circumferential)**: 회전체 곡률 κ_φ = sin(ψ)/r — 필렛 접촉각에 의해 결정

> **물리적 근거**: 회전체의 원주 방향 주곡률은 `κ_φ = sin(ψ) / r`이다. 여기서 ψ는 자오선 접선과 반경 방향의 각도, r은 베어링 축에서 접촉점까지의 거리이다.
> 리브 면(rib face)은 반경 방향에서 α_rib 각도만큼 기울어져 있으므로, 필렛 접촉점에서 ψ ≈ α_rib이다.
> 따라서 **R_rib_circ = r_contact / sin(α_rib)** 이며 유한한 값을 가진다.
> 이전 버전의 "평면(flat)" 가정(κ_φ ≈ 0)은 리브 면(face)만 고려한 것이고, 실제 접촉은 필렛(fillet) 위에서 발생하여 표면 법선이 기울어져 있기 때문에 원주 방향 곡률이 0이 아니다.

### 접촉점 반경 위치 (r_contact)

접촉점 반경은 **리브 베이스 위치(r_base)**와 **접촉 높이(h_c)**의 합으로 결정된다:

$$
r_{contact} = r_{base} + h_c
$$

#### 리브 베이스 위치 (r_base)

롤러 대단 내측 모서리의 반경 위치. 리브가 시작되는 지점이다:

$$
\gamma = \frac{\alpha_i + \alpha_o}{2} \quad \text{(roller tilt angle)}
$$

$$
r_{base} = \frac{d_{pw}}{2} + \frac{l_{we}}{2} \sin\gamma - \frac{d_{we,max}}{2} \cos\gamma
$$

| 항 | 의미 |
|---|---|
| `d_pw/2` | 피치원 반경 — 롤러 중앙의 기준점 |
| `+(l_we/2)·sin(γ)` | 롤러 대단 중심의 반경방향 오프셋 |
| `−(d_we_max/2)·cos(γ)` | 구면 중심에서 리브면까지의 반경방향 투영 |

#### 접촉 높이 h_c (사용자 입력 또는 자동)

실제 접촉점은 리브 베이스가 아닌, 리브 면(rib face) 위의 h_c 높이에서 발생한다.

**입력 방식:**
- **Auto** (기본): `h_c = h_rib / 2` (리브 면 중앙)
- **Manual**: 사용자가 직접 h_c 값을 지정 (0 ~ h_rib 범위로 클램핑)

> **참고**: Liu 2023 논문의 Eq.8은 h_c에서의 기하학적 관계를 기술하는 식이며,
> h_c 자체를 계산하는 공식이 아니다. Liu 논문에서 h_c = 4mm은 TEHL 시뮬레이션의
> 설계 입력값으로 사용되었다. 따라서 본 소프트웨어에서도 h_c를 입력 파라미터로 처리한다.

**유효 범위 검증:**

$$
r_{base} \leq r_{contact} \leq r_{base} + h_{rib}
$$

범위를 벗어나면 `InvalidGeometry` 오류가 발생한다.

**문헌 대응:**

| 본 소프트웨어 | 문헌 표기 | 참고 |
|-------------|----------|------|
| `r_base` | 이전 `r_contact` | 롤러 대단 내측 모서리 |
| `h_c` | `h_c` (Liu 2023, Eq.8-9) | 접촉 높이 |
| `r_contact` | `R_i + h_c` (Liu 2023) | 실제 접촉점 반경 |
| `r_contact` | `H` (Zhang 2025, Eq.6) | SR과 기하각도로 결정 |

### Meridional Plane (자오면)

$$
R_x = \frac{1}{\frac{1}{R_{sph}} - \frac{1}{R_{rib}}}
$$

**조건**: `R_sph < R_rib` (구가 오목면 내부에 있어야 함). 위반 시 InvalidGeometry 오류.

### Circumferential Direction (원주 방향)

#### ψ 각도의 기하학적 유도

회전체(surface of revolution)의 원주 주곡률 반경 공식:

$$
R_{circ} = \frac{r}{\sin\psi}
$$

여기서 $\psi$는 **표면 법선과 회전축(베어링 축) 사이의 각도**이다.

리브 면의 경우:
- 리브 면은 베어링 축에서 $\alpha_f$ = 80.145° 기울어진 원뿔면
- 리브 면 법선은 리브 면에 수직 → 법선-축 사이 각도 = $90° - \alpha_f$
- $\alpha_{rib} = 90° - \alpha_f$ 이므로, **$\psi = \alpha_{rib}$**

```
  베어링 축 (z)
      |
      |  αf (리브면-축)
      | /  ← 리브 면
      |/
      +------→ 반경 (r)

  법선-축 각도 ψ = 90° − αf = α_rib
```

**극한 검증:**

| 리브 면 형태 | αf (축 기준) | α_rib | ψ | R_circ | 의미 |
|---|---|---|---|---|---|
| 평면 디스크 (축⊥) | 90° | 0° | 0° | ∞ | 곡률 없음 ✓ |
| 원통면 (축∥) | 0° | 90° | 90° | r | 원통 반경 ✓ |
| HR30306J | 80.145° | 9.855° | 9.855° | r/0.171 ≈ 6r | ✓ |

#### 접촉점 반경 및 원주 곡률

$$
r_{contact} = \frac{d_{pw}}{2} + \frac{l_{we}}{2} \sin\gamma - \frac{d_{we,max}}{2} \cos\gamma + h_c
$$

$$
R_{rib,circ} = \frac{r_{contact}}{\sin(\alpha_{rib})}
$$

#### 등가 반경 (볼록-오목, conforming)

$$
R_y = \frac{1}{\frac{1}{R_{sph}} - \frac{1}{R_{rib,circ}}}
$$

**조건**: `R_sph < R_rib_circ` (구가 오목면 내부에 있어야 함).
원주 접선방향에서 리브 면은 축방향 단면 기준 오목 곡면이므로, 자오면과 동일하게 conforming 접촉이다.

**R_rib_circ 설정 방법:**
- **Auto (기본)**: `R_rib_circ = r_contact / sin(α_rib)` — 회전체 곡률 정리 기반 자동 계산
- **Manual**: 사용자 직접 입력 (CAD 측정값 또는 특수 형상)

### 물리적 해석

- `R_x > R_sph`: 자오면에서 등가 반경이 R_sph보다 크다 (오목-볼록 conformity)
- `R_y > R_sph`: 원주 방향에서도 오목-볼록 conformity (R_rib_circ >> R_sph)
- `R_rib → ∞`이면 `R_x → R_sph` (평면 리브)
- `R_sph → R_rib`이면 `R_x → ∞` (conformal contact, 접근)
- `R_x > R_y`이므로 접촉 타원 장축은 **자오면(meridional) 방향**
- `α_rib`이 작을수록 R_rib_circ가 커지고, R_y → R_sph에 가까워진다

> **타원 방향**: R_x > R_y (자오면 등가 반경이 더 큼) → 장반축 a는 자오면 방향, 단반축 b는 원주 방향. 컨투어 차트에서 x축=원주(접선), y축=자오(법선)으로 표시한다.

## 8.3 Hamrock-Brewe Approximation

타원 적분을 직접 계산하는 대신, Hamrock-Brewe 근사식을 사용한다 (오차 < 2%):

$$
\text{ratio} = R_y / R_x \quad (\geq 1 \text{이 되도록 정렬})
$$

### Ellipticity Ratio

$$
k_e = 1.0339 \times \text{ratio}^{0.6360}
$$

`k_e = a/b` (접촉 타원의 장반축/단반축 비)

### Complete Elliptic Integral of the First Kind

$$
\mathcal{F} = 1.5277 + 0.6023 \ln(\text{ratio})
$$

### Complete Elliptic Integral of the Second Kind

$$
\mathcal{E} = 1.0003 + \frac{0.5968}{\text{ratio}}
$$

## 8.4 Contact Ellipse

### Sum of Curvatures

$$
\Sigma\rho = \frac{1}{R_x} + \frac{1}{R_y}
$$

### Semi-axis a (장반축)

$$
a = \left(\frac{3 k_e^2 \mathcal{E} F_{rib}}{\pi E^* \Sigma\rho}\right)^{1/3} \quad \text{(Johnson } E^* \text{ 규약)}
$$

> Harris 규약(E')에서는 계수가 6: `a = (6k²εF/(πE'Σρ))^(1/3)`. 수치 결과 동일.

### Semi-axis b (단반축)

$$
b = \frac{a}{k_e}
$$

## 8.5 Combined Elastic Modulus

접촉 해석에서 두 물체의 탄성 특성을 하나의 등가 계수로 표현한다. 문헌에 따라 **두 가지 규약**이 존재한다.

### Harris 규약 (E') — 베어링 업계 표준

Harris & Kotzalas, SKF/NSK 카탈로그 등 베어링 업계에서 널리 사용:

$$
E' = \frac{2}{\frac{1-\nu_1^2}{E_1} + \frac{1-\nu_2^2}{E_2}}
$$

표준 베어링강(SUJ2/100Cr6): `E = 210 GPa`, `ν = 0.3` → **`E' = 230.8 GPa`**

이 규약에서의 Hertz 공식:
- 선접촉 반폭: `b² = 8qR/(πE')`
- 점접촉 반축: `a = (6k²εQ/(πE'Σρ))^(1/3)`

### Johnson 규약 (E*) — 접촉역학 표준

Johnson "Contact Mechanics", Hamrock "Tribology" 등 접촉역학 교과서에서 사용:

$$
E^* = \frac{1}{\frac{1-\nu_1^2}{E_1} + \frac{1-\nu_2^2}{E_2}}
$$

표준 베어링강: **`E* = 115.4 GPa`**

이 규약에서의 Hertz 공식:
- 선접촉 반폭: `b² = 4qR/(πE*)`
- 점접촉 반축: `a = (3k²εQ/(πE*Σρ))^(1/3)`

### 관계 및 본 소프트웨어의 선택

$$
E' = 2 E^*
$$

factor 2의 차이는 접촉 공식의 계수(6↔3, 8↔4)에서 정확히 상쇄되므로, **두 규약의 수치 결과는 완전히 동일**하다.

**본 소프트웨어는 Johnson 규약(E*)을 채택한다.** 이유:
1. 선접촉(레이스웨이)과 점접촉(리브)에서 동일한 `combined_elastic_modulus()` 함수를 공유
2. Johnson의 Hertz 공식(`b² = 4qR/(πE*)`)이 접촉역학 교재의 표준 형태와 직접 대응
3. 코드 내 일관성 우선 (하나의 E* 정의로 통일)

| 규약 | 기호 | Steel 값 | 선접촉 계수 | 점접촉 계수 |
|------|------|---------|-----------|-----------|
| Harris | E' | 230.8 GPa | 8 | 6 |
| **Johnson (채택)** | **E*** | **115.4 GPa** | **4** | **3** |

**단위 변환**: 수식 내에서는 `E*`를 **MPa** 단위로 사용한다 (`E*_MPa = E*_GPa × 1000`).

## 8.6 Maximum Contact Stress

$$
p_{max} = \frac{3 F_{rib}}{2\pi a b}
$$

Hertz 점접촉의 **반타원체(semi-ellipsoidal) 압력 분포**에서의 최대값이다.

### 8.6.1 압력 분포

접촉 타원 내부 임의의 점 `(x, y)`에서의 접촉 압력:

$$
p(x, y) = p_{max} \sqrt{1 - \frac{x^2}{a^2} - \frac{y^2}{b^2}}
$$

타원 경계(`x²/a² + y²/b² = 1`)에서 `p = 0`, 중심(`x = y = 0`)에서 `p = p_max`이다.

### 8.6.2 평균 접촉 압력

$$
p_{mean} = \frac{F_{rib}}{\pi a b} = \frac{2}{3} p_{max}
$$

반타원체 분포의 특성상 평균 압력은 최대 압력의 2/3이다.

### 8.6.3 Stress Scaling

점접촉의 응력-하중 관계를 유도하면:

- `a ∝ F^{1/3}`, `b ∝ F^{1/3}` (Hertz 이론)
- `a × b ∝ F^{2/3}`
- `p_max = 3F / (2πab) ∝ F / F^{2/3} = F^{1/3}`

$$
p_{max} \propto F_{rib}^{1/3}
$$

**선접촉**(raceway contact)에서는 `p_max ∝ q^{1/2}`이므로, 점접촉이 하중 증가에 대해 응력 상승이 더 완만하다.

| 접촉 유형 | 응력 스케일링 | 하중 2배 시 응력 증가 |
|-----------|-------------|----------------------|
| 점접촉 (리브) | `p_max ∝ F^(1/3)` | ×1.26 (26% 증가) |
| 선접촉 (레이스웨이) | `p_max ∝ q^(1/2)` | ×1.41 (41% 증가) |

### 8.6.4 Subsurface Stress Field

Hertz 점접촉에서 최대 전단응력(von Mises 기준)은 표면 아래에서 발생한다:

$$
z_{max\text{-}shear} \approx 0.48 \times b \quad (\text{for } k_e \approx 1)
$$

$$
\tau_{max} \approx 0.31 \times p_{max}
$$

이 위치는 구름 피로(rolling contact fatigue, RCF)의 균열 발생 기점이 된다.

### 8.6.5 수치 예제

NSK HR30306J 기준 (`R_sph = 50 mm`, `R_rib = 1500 mm`):

**Step 1**: 접촉점 위치 및 등가 반경
- `γ = (11.33° + 8.38°)/2 = 9.855°` (HR30306J 기준)
- `r_contact = 46/2 + (15.3/2)·sin(9.855°) − (16.284/2)·cos(9.855°) = 21.11 mm`
- `R_rib_circ = 21.11 / sin(80°) = 21.44 mm` (Auto 모드, α_rib=80° 가정)
- `R_x = 1/(1/50 - 1/1500) = 51.72 mm` (자오면, 볼록-오목)
- `R_y = 1/(1/50 + 1/21.44) = 15.03 mm` (원주면, 유한한 리브 원주 곡률 반영)

**Step 2**: 결합 탄성 계수 (Steel, E = 210 GPa, ν = 0.3)
- Johnson: `E* = 1/[2×(1-0.09)/210] = 115.4 GPa` ← 코드 사용값
- Harris: `E' = 2E* = 230.8 GPa` ← 베어링 카탈로그 표기값

**Step 3**: Hamrock-Brewe 계수
- `ratio = max(R_x, R_y) / min(R_x, R_y) = 51.72 / 50 = 1.034`
- `k_e = 1.0339 × 1.034^0.636 = 1.056`
- `E_e = 1.0003 + 0.5968 / 1.034 = 1.577`

**Step 4**: 접촉 타원 (`F_rib = 500 N`)
- `Σρ = 1/51.72 + 1/50 = 0.03934 mm⁻¹`
- Johnson: `a = (3 × 1.056² × 1.577 × 500 / (π × 115400 × 0.03934))^(1/3) = 0.570 mm`
- Harris: `a = (6 × 1.056² × 1.577 × 500 / (π × 230800 × 0.03934))^(1/3) = 0.570 mm` (동일)
- `b = 0.570 / 1.056 = 0.540 mm`
- 장축(a) = 자오면(meridional) 방향, 단축(b) = 원주(circumferential) 방향

**Step 5**: 최대 접촉 응력
- `p_max = 3 × 500 / (2π × 0.570 × 0.540) = 777 MPa`

**Step 6**: 스핀 모멘트
- `M_spin = (3/8) × 0.002 × 500 × 0.570 = 0.214 N·mm`

## 8.7 Spin Moment

롤러 구면과 리브 사이의 회전(spinning) 마찰에 의한 모멘트:

$$
M_{spin} = \frac{3}{8} \mu F_{rib} \cdot a
$$

여기서 μ는 운전 조건에 따라 결정된다:
- **Dry path** (operating 미지정): `μ = 0.002` 상수 (참고용)
- **EHL path** (operating 지정 + 회전 중): `μ = mu_eff`(EHL evaluation, §8.7A)

### 물리적 의미

스핀 모멘트는 롤러 대단 구면이 리브면 위에서 **피벗 회전(pivoting)**하면서 발생하는 마찰 토크이다. 이 값은:
- 베어링 토크 손실의 일부를 구성
- 윤활막 두께에 민감 (EHL 조건에서 μ 감소)
- 고속·고축하중 조건에서 발열원이 됨

계수 `3/8`은 Hertz 반타원체 압력 분포를 타원 면적에 대해 적분한 결과로, 균일 압력 가정 시의 `1/2`보다 작다:

$$
M_{spin} = \mu \int_0^{2\pi} \int_0^{r(\theta)} p(r,\theta) \cdot r^2 \, dr \, d\theta = \frac{3}{8} \mu F a
$$

---

## 8.7A Rib EHL/TEHL Analysis

리브-롤러단(flange-roller end) 접촉의 EHL/TEHL 평가. 운전 조건이 주어지고 베어링이 회전 중일 때 자동 활성화되며, `RibContactResult.ehl: Option<RibEhlResult>`에 결과가 채워진다.

> **연구 보고서 §4.3 / §5.1 참조**: 보고서가 "TRB 정체성과 직결되는 핵심 영역"으로 강조한 rib EHL을 회귀식 기반으로 구현. 2D Reynolds + DC-FFT 정밀 해석은 후속 작업으로 분리.

### 8.7A.1 해석 흐름

```
Hertz ellipse (a, b, p_max, k)         ── §8.3-8.6
        ↓
Speeds: u_entrain, u_slide, SRR        ── compute_rib_speeds (lubrication.rs)
        ↓
Dimensionless U, G, W                   ── eta_0, alpha_pv, E', R_x
        ↓
Hamrock-Dowson elliptical              ── h_c, h_min (isothermal)
        ↓
Murch-Wilson φ_T  +  starvation φ_s    ── h_c × φ_T × φ_s
        ↓
λ = h_min / σ_c   →   regime           ── classify_lambda
        ↓
Traction (Eyring | Carreau-Yasuda)     ── traction_coefficient dispatcher
        ↓
Asperity sharing (Clarke ξ = 1−erf(λ))  ── mu_eff = (1−ξ)·μ_ehl + ξ·μ_boundary
        ↓
Blok-Jaeger flash temperature          ── flash_temperature
```

### 8.7A.2 무차원 그룹 (Hamrock-Dowson elliptical)

$$
U = \frac{\eta_0\, u_{entrain}}{E'\, R_x}, \quad
G = \alpha\, E', \quad
W = \frac{F_{rib}}{E'\, R_x^2}
$$

여기서:
- `η_0` = Roelands 입구 점도 (operating temperature 보정)
- `α` = 압력-점도 계수 [1/Pa]
- `E'` = 2 × E* (Hamrock-Dowson 규약, 본 솔버 §8.5의 두 배)
- `R_x` = §8.2의 자오면 등가 반경 [m]

### 8.7A.3 유막두께

**Hamrock-Dowson 1981 elliptical** (k = a/b ≥ 1):
$$
H_c = 2.69\, U^{0.67}\, G^{0.53}\, W^{-0.067}\,(1 - 0.61\,e^{-0.73 k})
$$
$$
H_{min} = 3.63\, U^{0.68}\, G^{0.49}\, W^{-0.073}\,(1 - e^{-0.68 k})
$$

물리 단위: `h = H × R_x`.

**Murch-Wilson 열보정 (φ_T)**: §14.3A.2와 동일 식. SRR ≈ 2 (rib 순수 슬라이딩) 조건에서 φ_T < 1로 유막 감소.

**기아 보정 (φ_s)**: `OperatingConditions.starvation_factor` 적용.

### 8.7A.4 Speeds (rib 특수 처리)

리브 면은 inner ring과 함께 회전하고 롤러 끝면은 자체 spin → inner-rotating 좌표계에서 rib는 정지, 롤러 끝면만 운동. 따라서:

- **u_slide** = `|ω_roller × r_large_end|` (compute_trb_kinematics의 `u_slide_rib`)
- **u_entrain** = `u_slide / 2` (mean of (u_roller_end + 0))
- **SRR** = 2.0 (pure-sliding limit)

> **한계**: `compute_trb_kinematics`는 cone apex 모델을 사용하며, `α_i = α_o`인 평행 cone에서는 `ω_roller = 0`을 반환한다. 이 경우 EHL은 None으로 평가됨. 실제 TRB는 항상 `α_i ≠ α_o`이므로 정상 동작.

### 8.7A.5 Lambda Regime

$$
\sigma_c = \sqrt{\sigma_{roller}^2 + \sigma_{rib}^2}, \quad \Lambda = \frac{h_{min}}{\sigma_c}
$$

리브 면은 inner ring 가공 표면을 공유하므로 `σ_rib = OperatingConditions.rq_inner_eff()`로 근사.

분류 (§14.4와 동일):
- **Λ ≥ 3**: FullEhl (완전 유막)
- **1 ≤ Λ < 3**: Mixed (혼합 윤활)
- **Λ < 1**: Boundary (경계 윤활)

### 8.7A.6 Traction Dispatcher

`OperatingConditions.traction_model`에 따라 분기:

| 모델 | 식 | 권장 적용처 |
|------|-----|-----------|
| **Eyring** (default) | τ = τ₀·sinh⁻¹(η·γ̇/τ₀) | 일반 광유, 표준 운전 조건 |
| **Carreau-Yasuda** | τ = η_eff·γ̇, η_eff = η_∞ + (η_0−η_∞)·[1+(λγ̇)^a]^((n−1)/a) | rib·고 SRR (보고서 §4.3 권장), EV ULV |

τ_lim = 0.10·p_mean 캡 공유 (§14.3B 참조).

### 8.7A.7 혼합 윤활 (Clarke asperity sharing)

$$
\xi = 1 - \mathrm{erf}(\Lambda), \quad
\mu_{eff} = (1-\xi)\,\mu_{ehl} + \xi\,\mu_{boundary}
$$

여기서 `μ_boundary = 0.10` (boundary 영역 광유 + EP).

### 8.7A.8 Flash Temperature

**Blok-Jaeger** band source (§14에서 pub로 노출):
$$
\Delta T_{flash} = \mu_{eff}\, p_{asp}\, V_{slide} \big/ \big(2 k_s \sqrt{\pi V b / (2 \kappa_s)}\big)
$$

리브는 점접촉이므로 b = ellipse semi-minor (a, b 중 작은 값) 사용.

### 8.7A.9 출력 — `RibEhlResult`

| 필드 | 단위 | 의미 |
|------|------|------|
| h_c_um, h_min_um | μm | 중심·최소 유막두께 (φ_T·φ_s 보정 후) |
| sigma_composite_um | μm | √(σ_roller² + σ_rib²) |
| lambda_ratio, regime | — | Λ + FullEhl/Mixed/Boundary 분류 |
| mu_eff, mu_ehl | — | 유효 마찰 (혼합) / 유체 마찰 |
| asperity_load_ratio | — | ξ (Clarke) |
| p_asperity_mpa | MPa | 접촉점 분담 압력 |
| flash_temp_c | °C | Blok-Jaeger ΔT |
| srr, u_entrain_m_s, u_slide_m_s | — / m/s | 속도/슬라이딩 비율 |
| thermal_factor | — | φ_T (Murch-Wilson) |
| u_param, g_param, w_param, k_ellipse | — | 무차원 그룹 + ellipticity |

### 8.7A.10 검증 — `rib_contact::tests`

| 테스트 | 검증 내용 |
|--------|----------|
| `test_rib_ehl_none_when_operating_omitted` | operating None → ehl None (backward-compat), spin moment μ=0.002 사용 |
| `test_rib_ehl_none_when_static` | n_rpm=0 → ehl None |
| `test_rib_ehl_none_when_no_axial_load` | q_axial=0 → ehl None |
| `test_rib_ehl_loaded_rotating` | 정상 운전: h_c>0, h_min ≤ h_c, 0 < μ_eff < 0.20, SRR ≈ 2.0, 0 < φ_T ≤ 1 |
| `test_rib_ehl_changes_spin_moment` | dry vs wet path: a/b/p_max 동일, spin moment 변경 |
| `test_rib_ehl_speed_increases_film_thickness` | n_rpm 500 → 3000: h_c 증가 |
| `test_rib_ehl_eyring_vs_carreau` | 동일 운전, 두 모델 μ 차이 + h_min 동일 (확률 검증) |
| `test_rib_ehl_rough_surface_increases_asperity` | Rq 0.05 vs 0.6 μm: smooth Λ > rough Λ, rough가 asperity load 더 큼 |
| `test_rib_ehl_load_sweep_consistency` | q 100~1000 N: h_min 양수 유지, p_max 단조 증가 |
| `test_hamrock_dowson_elliptical_trends` | k=1 vs k=20: H_c, H_min 모두 증가, 0.3 < H_min/H_c < 1.0 |
| `test_hamrock_dowson_elliptical_zero_inputs` | u/g/w = 0 → (0,0) |

### 8.7A.11 한계 / Out of Scope

- **2D Reynolds + DC-FFT** 정밀 해석은 별도 후속 플랜 (현재 회귀식만)
- **Rib kinematics**: cone apex 모델만 사용 — `α_i = α_o` (cylindrical-like 평행 cone)에서는 ω_roller=0, EHL None
- **Lundberg axial profile**: rib 끝단 응력 집중 — 현재 미반영
- **Bearing current / EDM** 영향: EV 응용 후속 플랜
- **ZDDP 트리보필름 화학**: AdditiveType factor만 (Λ_perm 보정용)

---

## 8.8 Input/Output Summary

### Input

| Symbol | Description | Source |
|--------|-------------|--------|
| `R_sph` | Roller sphere radius [mm] | RollerProfile.r_sph |
| `R_rib` | Rib fillet radius — meridional [mm] | RacewayGeometry.r_rib |
| `R_rib_circ` | Rib circumferential radius [mm] | RacewayGeometry.r_rib_circ (null=Auto: r_contact/sin(α_rib)) |
| `F_rib` | Net axial force on rib [N] | `Q_j × sin(α_o − α_i) / cos(α_i)` |
| `E_roller` | Roller elastic modulus [GPa] | Material.e_roller |
| `E_ring` | Ring elastic modulus [GPa] | Material.e_ring |
| `ν` | Poisson's ratio | Material.nu |

### Output

| Symbol | Unit | Description |
|--------|------|-------------|
| `f_rib` | N | Applied axial force |
| `a_ellipse` | mm | Contact ellipse semi-major axis |
| `b_ellipse` | mm | Contact ellipse semi-minor axis |
| `p_max_rib` | MPa | Maximum contact stress |
| `spin_moment` | N·mm | Spin friction moment |
| `h_c_mm` | mm | 리브 면 접촉 높이 (Liu 2023, Eq.8) |
| `r_contact_mm` | mm | 접촉점 반경 위치 = r_base + h_c |
| `r_rib_circ_mm` | mm | 사용된 원주 방향 곡률반경 |

### Derived Quantities (UI 표시용)

| Quantity | Formula | Description |
|----------|---------|-------------|
| `2a` | `2 × a_ellipse` | 타원 장축 길이 [mm] |
| `2b` | `2 × b_ellipse` | 타원 단축 길이 [mm] |
| `k_e = a/b` | `a_ellipse / b_ellipse` | 타원율 (ellipticity ratio) |
| `A_contact` | `π × a × b` | 접촉 면적 [mm²] |
| `p_mean` | `F / (π × a × b)` | 평균 접촉 압력 [MPa] |

## 8.9 Alert Criteria

- `p_max_rib > 1500 MPa`: Warning (리브 응력 과대)
- 전형적 범위: 500~2000 MPa (정상 운전), > 2500 MPa (위험)

### 리브 응력 허용 기준 가이드라인

| 등급 | p_max 범위 | 판단 |
|------|-----------|------|
| 정상 | < 1000 MPa | 안전 — 장수명 기대 |
| 주의 | 1000~1500 MPa | 보통 — 윤활 상태 확인 |
| 경고 | 1500~2500 MPa | 과대 — 프로파일/하중 재검토 |
| 위험 | > 2500 MPa | 표면 손상 위험 — 설계 변경 필요 |

## 8.10 Stress Contour Visualization

Stress Contour 탭에서 **Rib Contact** 토글을 선택하면 리브 접촉 결과를 시각적으로 확인할 수 있다.

### 8.10.1 Bar Chart (좌측)

전 롤러에 대한 `p_max_rib` 분포를 바 차트로 표시:
- **X축**: 롤러 위치 ψ [deg]
- **Y축**: 최대 접촉 응력 p_max [MPa]
- **색상**: 응력 수준별 Blue → Yellow → Red 그라데이션
- **Hover 정보**: ψ, p_max, F_rib, a, b, Spin Moment

하중 영역(load zone) 내 롤러만 리브 접촉이 발생하며, 하중 영역 밖 롤러는 `p_max = 0`이다.

### 8.10.2 Contact Ellipse (우측)

최대 하중 롤러의 접촉 타원을 히트맵 + SVG로 시각화:

```
     Circumferential (b)
         ← 2b →
     ╭─────────╮ ↑
    ╱ ╱─────╲   ╲ │
   │ │ ● p_max│  │ 2a  Meridional (a)
    ╲ ╲─────╱   ╱ │
     ╰─────────╯ ↓
```

- **축 방향**: x축 = 원주(circumferential, 단축 b), y축 = 자오(meridional, 장축 a)
- **동심 타원**: 압력 분포(p_max → 0)를 색상 그라데이션으로 표현
  - 중심: 최대 응력 (밝은 빨강)
  - 외곽: 접촉 경계 (앰버 윤곽선)
- **치수선**: `a` (장축, 자오 방향), `b` (단축, 원주 방향) [mm]
- **요약 테이블**: p_max, F_rib, a/b 타원율, Spin Moment

### 8.10.3 Raceway 접촉 응력과의 비교

리브 접촉과 레이스웨이 접촉의 특성 비교:

| 항목 | 레이스웨이 (선접촉) | 리브 (점접촉) |
|------|-------------------|--------------|
| 접촉 형태 | 직사각형 (b × L_eff) | 타원 (a × b) |
| 접촉 면적 | 크다 | 작다 |
| 압력 분포 | 반원통형 | 반타원체 |
| 응력 수준 | 일반적으로 낮다 | 높을 수 있다 |
| 스케일링 | p ∝ q^(1/2) | p ∝ F^(1/3) |
| 주요 영향인자 | 크라운, 프로파일 | R_sph, R_rib |

레이스웨이 p_max는 slicing으로 축 방향 분포를 보지만, 리브 p_max는 **단일 점접촉**이므로 롤러당 하나의 값만 존재한다.

## 8.11 Hertz Approach & Tangent Stiffness

### 8.11.1 접근량 (Approach)

Hertz 점접촉의 접근량(approach, 탄성 변형):

$$
\delta_{rib} = \frac{a^2 \Sigma\rho}{2 k_e \mathcal{F}} \quad [\text{mm}]
$$

여기서 $\mathcal{F}$는 제1종 완전 타원 적분(Hamrock-Brewe 근사).

단위 변환: 코드에서는 `δ_rib`를 **μm** 단위로 저장한다 (`×1000`).

### 8.11.2 접선 강성 (Tangent Stiffness)

Hertz 점접촉에서 `F ∝ δ^(3/2)` (비선형)이므로, 미분 강성:

$$
K_{rib} = \frac{dF}{d\delta} = \frac{3}{2} \frac{F_{rib}}{\delta_{rib}} \quad [\text{N/μm}]
$$

이 강성은 Coupled 모드 내부 반복에서 수렴 속도에 영향을 미친다.

### 8.11.3 수치 예제

NSK HR30306J 기준 (`F_rib = 500 N`, `R_rib_circ = 21.44 mm` (Auto)):
- `δ_rib = 0.570² × 0.03934 / (2 × 1.056 × 1.560) × 1000 = 3.88 μm`
- `K_rib = 1.5 × 500 / 3.88 = 193 N/μm`

## 8.12 Rib Contact Mode

리브 접촉을 베어링 평형에 반영하는 두 가지 모드를 제공한다.

### 8.12.1 PostProcess (기본)

- 리브 접촉은 평형 수렴 **이후** 후처리로만 계산
- 평형 잔차에 리브 힘 미포함
- 빠르고 안정적
- 대부분의 일반적인 TRB 해석에 적합

### 8.12.2 Coupled

- 리브 Hertz 변형(δ_rib)이 롤러의 **유효 접근량을 감소**시킴
- 각 롤러에 대해 내부 고정점 반복(fixed-point iteration):
  1. `delta_eff = delta_rigid - δ_rib × sin(α_o)`
  2. `Q = Gen1(delta_eff)` → `F_rib = Q × sin(α_o−α_i)/cos(α_i)`
  3. `δ_rib = Hertz(F_rib)` → 1번으로
  4. |Δδ_rib| < 0.001 μm 수렴 시 종료 (통상 3~5회)
- 리브 컴플라이언스가 **개별 롤러 하중 Q_j**를 직접 변경
- preload displacement 계산에도 동일 반복 적용
- **모든 preload 모드에서 결과 차이 발생**
- 높은 축하중 비율, 프리로드 민감 해석에 유용

### 8.12.3 선택 가이드

| 조건 | 권장 모드 |
|------|----------|
| 일반 해석, 빠른 설계 검토 | PostProcess |
| 높은 F_a/F_r 비율 (> 0.5) | Coupled |
| 프리로드 민감도 분석 | Coupled |
| Gen1↔Gen3 비교 | PostProcess (일관성) |

### Input/Output 추가

| Symbol | Unit | Description |
|--------|------|-------------|
| `delta_rib` | μm | Hertz 접근량 |
| `k_rib` | N/μm | 접선 강성 dF/dδ |
| `rib_contact_mode` | - | PostProcess \| Coupled (SolverParams) |
