# Chapter 5: Hertz Line Contact Theory

## 5.1 Combined Elastic Modulus

두 탄성체의 접촉에서, 재료 물성을 하나의 등가 탄성 계수로 결합한다:

$$
E^* = \frac{1}{\frac{1 - \nu_1^2}{E_1} + \frac{1 - \nu_2^2}{E_2}}
$$

여기서:
- `E_1, E_2`: 각 접촉체의 Young's modulus [GPa]
- `ν_1, ν_2`: Poisson's ratio [-]

**표준 베어링강**: E₁ = E₂ = 210 GPa, ν₁ = ν₂ = 0.3이면:

$$
E^* = \frac{210}{2 \times 0.91} \approx 115.4 \text{ GPa}
$$

## 5.2 Hertz Contact Half-Width

두 원통의 선접촉(line contact)에서 접촉 반폭 `b`:

$$
b = \sqrt{\frac{4 q R_{eq}}{\pi E^*}}
$$

여기서:
- `q`: 단위 길이당 하중 [N/mm]
- `R_eq`: 등가 반경 [mm]
- `E*`: 결합 탄성 계수 [MPa] (GPa에서 ×1000 변환)

### 유도

Hertz 이론에서 반원통 압력 분포 `p(x) = p₀√(1 - x²/b²)`를 가정하고, 접촉 영역의 경계 조건(면 내 변형 = 초기 간극)을 적용하면 위 관계가 유도된다.

## 5.3 Maximum Hertz Contact Pressure

선접촉에서의 최대 접촉 응력:

$$
p_{max} = \frac{2q}{\pi b}
$$

이는 반타원형 압력 분포의 정점이다. 접촉 반폭 `b`를 대입하면:

$$
p_{max} = \sqrt{\frac{q E^*}{\pi R_{eq}}}
$$

### 수치 예시

| Parameter | Value |
|-----------|-------|
| q | 500 N/mm |
| R_eq | 5 mm |
| E* | 115,384 MPa |
| **b** | **0.166 mm** |
| **p_max** | **1,916 MPa** |

## 5.4 Hertz Approach (Elastic Deformation)

선접촉에서의 탄성 접근량 δ:

$$
\delta = \frac{2q}{\pi E^*} \left[\ln\left(\frac{4 R_{eq}}{b}\right) - \frac{1}{2}\right]
$$

이 관계는 `δ = f(q)`에서 `q`에 대해 음함수적(implicit)이므로, 주어진 δ에서 q를 구하려면 Newton-Raphson 반복이 필요하다.

### 5.4.1 Dual-Raceway Approach Equation

TRB에서 슬라이스 접근량은 outer raceway와 inner raceway **양쪽의 Hertz 변형**을 모두 소화해야 한다. 또한 inner raceway 접촉 법선(α_i)이 outer raceway 접촉 법선(α_o)과 다르므로, inner 쪽 변형은 `cos(α_o − α_i)`로 사영한다:

$$
\delta_{hertz,outer}(q) + \delta_{hertz,inner}(q) \cdot \cos(\alpha_o - \alpha_i) = \delta_{available}
$$

여기서:
- `δ_hertz,outer(q)`: outer raceway에서의 Hertz 접근량 (R_eq_outer 사용)
- `δ_hertz,inner(q)`: inner raceway에서의 Hertz 접근량 (R_eq_inner 사용)
- `cos(α_o − α_i)`: inner→outer 법선 방향 사영 계수

**물리적 의미**: 같은 하중 q에서 inner(conforming, R_eq 작음)가 outer(non-conforming, R_eq 큼)보다 변형이 크다. 양쪽 변형의 합이 가용 접근량을 소화하므로, outer-only 모델 대비 **시스템이 더 유연**(같은 δ에서 q가 작아짐)해진다.

> **cos_alpha_diff = 0 인 경우**: legacy outer-only 모델로 폴백한다. 이는 하위 호환성을 위한 것이다.

### 5.4.2 Newton-Raphson for q from δ (Dual-Raceway)

목적: 주어진 가용 접근량 `δ_available` [mm]에서 선하중 `q` [N/mm]을 산출.

1. **초기 추정**: `q₀ ≈ 0.3 × E* × √R_eq_outer × δ^(10/9)` (dual이므로 계수 축소)
2. **반복**: `f(q) = δ_outer(q) + δ_inner(q)·cos(α_o−α_i) - δ_available = 0`
   - 잔차: `f = dual_approach(q) - δ_available`
   - 수치 미분: `df = [dual_approach(q + Δq) - f] / Δq`
   - 갱신: `q ← q - f/df`
3. **수렴 판정**: `|f| < 10⁻¹²`

## 5.5 Palmgren Line Contact Exponent

Palmgren의 경험적 관계: 선접촉에서 하중-변형 관계는 다음과 같이 근사된다:

$$
q = C \cdot \delta^{10/9}
$$

여기서 지수 10/9 ≈ 1.111은 Hertz 선접촉의 비선형성을 반영한다. 점접촉의 3/2 (= 1.5)보다 작으며, 이는 선접촉이 점접촉보다 강성이 높음(비선형성이 약함)을 의미한다.

## 5.6 Weber Bulk Deformation (진단용)

Weber의 sub-surface 변형 공식:

$$
\delta_{bulk} = \frac{4q(1 - \nu^2)}{\pi E} \left[\ln\left(\frac{2\sqrt{h_1 h_2}}{b}\right) - \frac{\nu}{2(1-\nu)}\right] \times 1000 \quad [\mu m]
$$

여기서:
- `q`: 단위 길이당 하중 [N/mm], `E`: 평균 Young's modulus [MPa], `ν`: Poisson's ratio
- `b`: 접촉 반폭 [mm], `h₁`: 롤러 깊이 [mm], `h₂`: 레이스웨이 깊이 [mm]

### ⚠️ 주의: Hertz approach와 더하지 말 것 (double-counting)

Hertz mutual-approach 공식과 Weber 공식은 **같은 prefactor**를 공유:

$$
\frac{2}{E^*} = \frac{4(1-\nu^2)}{E} \quad \text{(two equal bodies)}
$$

두 공식은 **동일한 탄성 변형 적분의 다른 closure**일 뿐이다:
- **Hertz (§5.4)**: cylinder radius R로 적분을 closure → `[ln(4R/b) − 0.5]`
- **Weber (본 절)**: body depth √(h₁h₂)로 closure → `[ln(2√(h₁h₂)/b) − ν/(2(1-ν))]`

수치적으로 두 값이 거의 같음 (steel, typical TRB에서 `δ_weber / δ_hertz ≈ 98.5%`).

**결론: Hertz approach `δ`는 이미 두 body의 bulk 변형을 포함한 전체 상호 접근량.** Weber를 Hertz에 추가 적용하면 bulk를 이중으로 계산하게 된다. Palmgren, Harris, Johnson의 표준 베어링 해석은 모두 Hertz approach 단독으로 변형–하중 관계를 기술.

### Weber의 용도

`h_bulk_k`, `h_bulk_k_outer` 필드는 **sub-surface 변형 진단용 참고값**으로만 보고된다. Newton-Raphson 잔차식에 참여하지 않으며, 슬라이스 강성 합성에도 포함되지 않는다.

## 5.7 Tangent Stiffness

각 슬라이스의 Hertz 접선 강성(numerical):

$$
K_{hertz,k} = \frac{\partial q}{\partial \delta}\bigg|_{\delta_k} \approx \frac{q(\delta_k + \Delta\delta) - q(\delta_k)}{\Delta\delta}
$$

단위: [N/mm per μm]. Gen3 solver의 Jacobian 구성, 슬라이스 합성 강성 계산에 사용.

### 슬라이스 합성 강성 (outer normal 기준)

Dual-raceway 모델에서 outer normal 방향 슬라이스 compliance는 outer·inner Hertz 스프링의 직렬:

$$
\frac{1}{K_{combined,k}} = \frac{1}{K_{hertz,k}^{outer}} + \frac{\cos^2(\alpha_o - \alpha_i)}{K_{hertz,k}^{inner}}
$$

inner raceway의 변형은 inner normal 방향에서 일어나므로, outer normal 방향으로 사영할 때 `cos²(α_o − α_i)`가 곱해진다. Weber bulk는 이미 각 Hertz 항에 내재되어 있으므로 별도 스프링으로 포함하지 않는다.

## 5.8 Complete Slice Contact Result

슬라이스 `k`에서의 접촉 해석 결과:

| Output | Unit | Description |
|--------|------|-------------|
| `δ_k` | μm | Approach at slice k (rigid gap − profile correction) |
| `q_k`, `q_k_outer`, `q_k_inner` | N/mm | Line load (outer normal; inner raceway 사영) |
| `b_k`, `b_k_outer` | mm | Contact half-width (inner/outer) |
| `p_max_k`, `p_max_k_outer` | MPa | Max Hertz contact pressure (inner/outer) |
| `h_bulk_k`, `h_bulk_k_outer` | μm | Weber bulk deformation — **진단용, Hertz에 내재, 더하지 말 것** |
| `K_hertz_k`, `K_hertz_k_outer` | N/mm/μm | Hertz tangent stiffness (inner/outer) |
| `K_combined_k` | N/mm/μm | Outer-normal 슬라이스 합성 강성 (§5.7) |
| `in_contact` | bool | Contact status |
