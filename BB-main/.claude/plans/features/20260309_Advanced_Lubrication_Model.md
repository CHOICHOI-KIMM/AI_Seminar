# Advanced Lubrication Model (Basic / Advanced 이중 모드)

## TL;DR

### Quick Summary
기존 EHL 윤활 해석을 **Basic** 모드로 유지하고, 최신 논문/규격 기반의 **Advanced** 모드를 추가하여 정밀도를 단계적으로 향상시킨다. Advanced 모드는 **Masjedi-Khonsari (2015) 통합 유막두께 공식** (조도 효과 내장), Roelands 압력-점도, Eyring sinh⁻¹ 트랙션, 물리 기반 기아 모델, 개선된 열보정을 포함한다.

### Deliverables
1. **LubricationModel enum** (Basic / Advanced) — 사용자 선택 가능
2. **Masjedi-Khonsari (2015) 유막두께 공식** — 조도 효과 통합, GT 모델 대체
3. **Roelands 압력-점도 모델** — 비선형 η(p) 관계
4. **Eyring sinh⁻¹ 트랙션 커브** — SRR 의존 마찰 계수
5. **물리 기반 기아 모델** — 그리스 기유 출혈 + 속도 의존 φ_s
6. **개선된 열보정** — Murch-Wilson + flash temperature
7. **UI 모드 전환** — Basic/Advanced 토글
8. **Manual/14_Lubrication.md 업데이트**

### Estimated Effort
- Phase 1 (인프라 + 유변학 + M-K 유막): 중간~높음
- Phase 2 (트랙션): 중간
- Phase 3 (기아 + 열): 중간
- Phase 4 (UI + 문서): 낮음

---

## 설계 원칙

### 1. Basic 모드는 건드리지 않는다
- 기존 `compute_film_thickness`, `compute_traction` 함수는 **그대로 유지**
- Basic = 현재 구현 (Dowson-Higginson + GT + Eyring 단순화)
- Advanced = 새 함수 경로로 분기

### 2. 공통 인터페이스 유지
- Basic과 Advanced 모두 동일한 `FilmThicknessResult`, `TractionSummary` 출력
- UI는 모드에 무관하게 동일한 데이터 구조를 렌더링
- 차이점은 결과값의 정밀도와 추가 진단 필드뿐

### 3. 점진적 구현
- 각 서브 모델은 독립적으로 구현/테스트 가능
- Phase 1만으로도 유의미한 정밀도 향상

---

## 현재 모델 (Basic) 한계 요약

| 영역 | Basic 구현 | 한계 |
|------|-----------|------|
| 압력-점도 | 선형 α_pv (Barus) | GPa급에서 과대 예측 |
| 유막두께 | Dowson-Higginson (1977) 회귀식 | 등온/뉴턴/완전충전 가정, 조도 무시 |
| 트랙션 | μ_ehl = τ₀/p_mean (상수) | SRR 무관, zero-SRR 극한만 |
| 혼합윤활 | Greenwood-Tripp (1970) | 탄성만, 소성 변형 무시 |
| 조도 물성 | η·β·σ = 0.04, √σ/β = 0.14 (상수) | 표면 상태 반영 불가 |
| 기아 | φ_s = 사용자 상수 | 물리 모델 없음 |
| 열보정 | Gupta φ_T (단일값) | 접촉 내 온도 분포 무시 |

---

## Phase 1: 인프라 + 비뉴턴 유변학

### Task 1.1: LubricationModel enum 및 입력 확장

**types.rs**에 모드 선택 타입 추가:

```rust
/// 윤활 해석 모델 선택
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LubricationModel {
    /// Dowson-Higginson + GT + Eyring 단순화 (기존)
    Basic,
    /// Roelands + Eyring sinh⁻¹ + KE 조도 + 물리 기아
    Advanced,
}

impl Default for LubricationModel {
    fn default() -> Self { LubricationModel::Basic }
}
```

`OperatingConditions`에 필드 추가:

```rust
pub struct OperatingConditions {
    // ... 기존 필드 유지 ...

    /// 윤활 해석 모델 선택 (Basic / Advanced)
    #[serde(default)]
    pub lubrication_model: LubricationModel,

    // === Advanced 모드 전용 파라미터 (serde default) ===

    /// Eyring stress τ₀ [MPa]. 기본 5 MPa (광유).
    #[serde(default = "default_tau_eyring")]
    pub tau_eyring: f64,

    /// Roelands 압력-점도 지수 Z_r. 기본 0.67 (광유).
    #[serde(default = "default_z_roelands")]
    pub z_roelands: f64,

    /// 윤활유 열전도도 k_fluid [W/(m·K)]. 기본 0.15.
    #[serde(default = "default_k_fluid")]
    pub k_fluid: f64,

    /// 점도-온도 계수 β_visc [1/K]. 기본 0.04.
    #[serde(default = "default_beta_visc")]
    pub beta_visc: f64,
}
```

- [x] types.rs에 `LubricationModel` enum 추가
- [x] OperatingConditions에 Advanced 전용 필드 추가 (serde default)
- [x] TS 미러링 (bearing.ts)
- [x] 기존 Basic 경로 동작 영향 없음 검증

### Task 1.2: Roelands 압력-점도 모델

현재 Barus 모델: `η(p) = η₀ × exp(α × p)` — 고압에서 점도를 과대 예측한다.

**Roelands (1966)** 모델:
```
η(p) = η₀ × exp{ (ln(η₀) + 9.67) × [(1 + p/p_r)^Z_r − 1] }

여기서:
  p_r = 196.2 MPa (Roelands 기준 압력)
  Z_r = α_pv × p_r / (ln(η₀) + 9.67)  (무차원 지수)
```

**영향 범위**:
- 유막두께 공식 자체는 α_pv 기반 (D-H 회귀식)이므로 변경 불필요
- **트랙션 계산**에서 접촉 내 유효 점도 η_eff를 Roelands로 구함
- **Eyring 트랙션** 계산 시 p-dependent η 사용

구현 위치: `lubrication.rs` 내 새 함수

```rust
/// Roelands pressure-viscosity: η(p) at given contact pressure.
///
/// Reference: Roelands, C.J.A. (1966), "Correlational Aspects of the
/// Viscosity-Temperature-Pressure Relationship of Lubricating Oils"
fn roelands_viscosity(eta_0: f64, p_pa: f64, z_r: f64) -> f64 {
    let p_r = 196.2e6; // [Pa]
    let log_term = (eta_0.ln() + 9.67) * ((1.0 + p_pa / p_r).powf(z_r) - 1.0);
    eta_0 * log_term.exp()
}
```

- [x] `roelands_viscosity()` 함수 구현
- [x] 테스트: p=0에서 η=η₀, p→∞에서 η < Barus 예측
- [x] 테스트: 광유 (Z_r≈0.67)와 PAO (Z_r≈0.5) 비교

### Task 1.3: Masjedi-Khonsari (2015) 통합 유막두께 공식

Dowson-Higginson은 **매끈한 표면**을 가정하므로 조도 효과를 별도 모델(GT)로 처리해야 한다.
Masjedi-Khonsari (2015)는 **표면 조도를 유막두께 공식에 직접 통합**하여, GT/KE 같은 별도 조도 접촉 모델 없이도 혼합윤활 영역을 연속적으로 처리한다.

**핵심 공식** (Line Contact, Masjedi & Khonsari 2015):

```
중심 유막두께:
  H_c = h_c/R = a₁ × U^a₂ × G^a₃ × W^a₄ × (1 + a₅ × σ̄^a₆ × V^a₇ × W^a₈)

최소 유막두께:
  H_min = h_min/R = b₁ × U^b₂ × G^b₃ × W^b₄ × (1 + b₅ × σ̄^b₆ × V^b₇ × W^b₈)

조도 하중 분율 (GT 대체):
  F_a/F = c₁ × σ̄^c₂ × U^c₃ × W^c₄ × G^c₅

조도 접촉 면적 분율:
  A_a/A_H = d₁ × σ̄^d₂ × U^d₃ × W^d₄ × G^d₅

여기서:
  U = η₀ × u_m / (E' × R)         — 속도 파라미터
  G = α × E'                       — 재료 파라미터
  W = w / (E' × R × L)             — 하중 파라미터
  σ̄ = σ_combined / R               — 무차원 표면 조도
  V = σ̄ × √(2) / (R × β × η_asp)  — 조도 형상 파라미터

  σ_combined = √(R_q1² + R_q2²)    — 합성 RMS 조도
  a₁~a₈, b₁~b₈, c₁~c₅, d₁~d₅    — 회귀 계수 (논문 Table)
```

**M-K 회귀 계수** (Line Contact):
```
중심 유막 (H_c):
  a₁ = 3.06, a₂ = 0.69, a₃ = 0.56, a₄ = -0.10
  a₅ = 0.18, a₆ = 0.20, a₇ = -0.50, a₈ = 0.26

최소 유막 (H_min):
  b₁ = 2.65, b₂ = 0.70, b₃ = 0.54, b₄ = -0.13
  b₅ = 0.57, b₆ = 0.24, b₇ = -0.42, b₈ = 0.19

조도 하중 분율 (F_a/F):
  c₁ = 6.30e-3, c₂ = 1.58, c₃ = -0.40, c₄ = 0.82, c₅ = -0.37

접촉 면적 분율 (A_a/A_H):
  d₁ = 7.80e-4, d₂ = 1.90, d₃ = -0.50, d₄ = 0.95, d₅ = -0.45
```

> **참고**: σ̄ → 0 (매끈한 표면) 극한에서 보정항 `(1 + a₅×σ̄^a₆×...)` → 1이 되어
> 기존 Dowson-Toyoda / Dowson-Higginson 공식과 수렴한다.

**장점**:
1. **별도 GT/KE 모델 불필요** — 조도 하중 분율을 M-K 공식이 직접 제공
2. **연속적 전이** — Λ>3 (EHL) → Λ<1 (경계) 전이가 자연스럽게 포착됨
3. **최신 수치 EHL 기반** — 고해상도 수치 해법으로 검증된 회귀 계수
4. **추가 입력 최소** — R_q1, R_q2 (표면 조도)만 추가

**구현**:

```rust
/// Masjedi-Khonsari (2015) integrated film thickness formula.
///
/// Includes surface roughness effects directly in the film thickness
/// calculation, eliminating the need for a separate GT/KE asperity model.
///
/// Reference: Masjedi, M. & Khonsari, M.M. (2015), "On the effect of
/// surface roughness in point-contact EHL: Formulas for film thickness
/// and asperity load", Tribology International, 82, 228-244.
pub struct MasjediKhonsariParams {
    /// Combined RMS surface roughness σ [m]
    pub sigma_combined: f64,
    /// Asperity tip radius β [m]
    pub beta_asperity: f64,
    /// Asperity density η [1/m²]
    pub eta_asperity_density: f64,
}

/// M-K film thickness result (central + minimum + asperity fractions)
pub struct MKFilmResult {
    /// Central film thickness [m]
    pub h_c: f64,
    /// Minimum film thickness [m]
    pub h_min: f64,
    /// Asperity load fraction F_a/F (replaces GT γ_a)
    pub load_fraction: f64,
    /// Asperity contact area fraction A_a/A_H
    pub area_fraction: f64,
}

/// Compute film thickness using Masjedi-Khonsari (2015) formula.
fn compute_film_mk(
    u_param: f64,      // U = η₀·u_m/(E'·R)
    g_param: f64,      // G = α·E'
    w_param: f64,      // W = w/(E'·R·L)
    sigma_bar: f64,    // σ̄ = σ/R (dimensionless roughness)
    v_param: f64,      // V = σ̄·√2/(R·β·η)
) -> MKFilmResult
```

**OperatingConditions 확장** (Advanced 전용):
```rust
/// Inner raceway RMS roughness R_q [μm]. 기본 0.3 μm.
#[serde(default = "default_rq_inner")]
pub rq_inner: f64,

/// Outer raceway RMS roughness R_q [μm]. 기본 0.3 μm.
#[serde(default = "default_rq_outer")]
pub rq_outer: f64,

/// Roller surface RMS roughness R_q [μm]. 기본 0.15 μm.
#[serde(default = "default_rq_roller")]
pub rq_roller: f64,
```

- [x] `MasjediKhonsariParams`, `MKFilmResult` 구조체 정의
- [x] `compute_film_mk()` 구현 (회귀 계수 하드코딩)
- [x] σ̄→0 극한에서 D-H/D-T 공식과 수렴 검증
- [x] OperatingConditions에 R_q 필드 추가
- [x] TS 미러링 (bearing.ts)
- [x] 테스트: 매끈한 표면 (R_q→0)에서 load_fraction ≈ 0 확인
- [x] 테스트: R_q = 0.3μm에서 load_fraction > smooth 확인
- [x] 테스트: 조도 증가 시 load_fraction 단조 증가 확인

### Task 1.4: Advanced 열보정 — Murch-Wilson

Murch-Wilson (1975) 열보정:
```
φ_T = 1 − 13.2 × (p₀/E') × L_th^0.42 / [1 + 0.213 × (1 + 2.23 × SRR^0.83) × L_th^0.64]

여기서:
  L_th = η₀ × β_visc × u_m² / k_fluid
  SRR = slide-roll ratio (슬라이스별)
```

기존 Gupta φ_T는 SRR=0 가정이었으나, Murch-Wilson은 **SRR에 따른 열보정 변동**을 반영한다.

```rust
/// Murch-Wilson thermal correction factor.
///
/// More accurate than Gupta's simplified form: accounts for SRR.
/// Reference: Murch, L.E. & Wilson, W.R.D. (1975)
fn thermal_correction_murch_wilson(
    eta_0: f64, beta_visc: f64, u_m: f64, k_fluid: f64,
    srr: f64, p_hz: f64, e_star: f64,
) -> f64
```

- [x] `thermal_correction_murch_wilson()` 구현
- [x] SRR=0에서 φ_T ≈ 1.0 확인 (열효과 최소)
- [x] SRR 증가 시 φ_T 감소 확인 (Wilson 1979 공식으로 수정)

### Task 1.5: Advanced 모드 디스패치

`bearing.rs`에서 모드에 따라 분기:

```rust
// bearing.rs — compute_bearing_result() 내
let (film_distribution, film_thickness, traction) = match input.operating.lubrication_model {
    LubricationModel::Basic => {
        // 기존 경로 그대로
        let dist = lubrication::compute_film_thickness_distribution(...);
        let summary = lubrication::summarize_film_from_distribution(...);
        let trac = lubrication::compute_traction(...);
        (dist, summary, trac)
    }
    LubricationModel::Advanced => {
        let dist = lubrication::compute_film_thickness_distribution_advanced(...);
        let summary = lubrication::summarize_film_from_distribution(...); // 동일 요약
        let trac = lubrication::compute_traction_advanced(...);
        (dist, summary, trac)
    }
};
```

- [x] bearing.rs에 모드 분기 로직 추가 (Single + Dual 모드)
- [x] Advanced 함수 시그니처 정의 및 본문 구현 완료

---

## Phase 2: Eyring 트랙션 + 탄소성 조도 모델

### Task 2.1: Eyring sinh⁻¹ 트랙션 모델

현재 `μ_ehl = τ₀/p_mean` (zero-SRR 극한)을 **전체 SRR 범위** Eyring 모델로 교체:

```
전단 응력:
  τ = τ₀ × sinh⁻¹(η_eff × γ̇ / τ₀)

여기서:
  γ̇ = u_slide / h_c   (전단율)
  η_eff = η_Roelands(p_mean)  (접촉 압력에서의 점도)
  u_slide = SRR × u_roll

트랙션 계수:
  μ_ehl = τ / p_mean

포화 조건:
  |τ| ≤ τ_lim ≈ 0.1 × p_mean  (전단 강도 제한)
```

**트랙션 커브 특성**:
```
SRR ≈ 0:     μ ≈ η_eff × u_slide / (h_c × p_mean)  (선형, 뉴턴)
SRR 중간:    μ ≈ τ₀/p_mean × ln(2η_eff × γ̇/τ₀)     (로그 증가)
SRR 큰 값:   μ → τ_lim / p_mean                       (포화)
```

```rust
/// Full Eyring traction model with Roelands viscosity.
///
/// Computes the EHL traction coefficient as a function of SRR,
/// contact pressure, film thickness, and lubricant properties.
///
/// References:
///   - Johnson, K.L. & Tevaarwerk, J.L. (1977)
///   - Bair, S. & Winer, W.O. (1979)
fn eyring_traction_coefficient(
    srr: f64,           // slide-roll ratio
    u_roll: f64,        // rolling velocity [m/s]
    p_mean_pa: f64,     // mean Hertzian pressure [Pa]
    h_c_m: f64,         // central film thickness [m]
    eta_0: f64,         // ambient viscosity [Pa·s]
    z_roelands: f64,    // Roelands exponent
    tau_eyring: f64,    // Eyring stress [Pa]
) -> f64 {
    // 1. Effective viscosity at contact pressure (Roelands)
    let eta_eff = roelands_viscosity(eta_0, p_mean_pa, z_roelands);

    // 2. Shear rate
    let u_slide = srr * u_roll;
    let gamma_dot = u_slide.abs() / h_c_m.max(1e-9);

    // 3. Eyring shear stress
    let x = eta_eff * gamma_dot / tau_eyring;
    let tau = tau_eyring * x.asinh();   // τ₀ × sinh⁻¹(x)

    // 4. Limiting shear stress (typically 0.07-0.12 × p)
    let tau_lim = 0.10 * p_mean_pa;
    let tau_clamped = tau.min(tau_lim);

    // 5. Traction coefficient
    (tau_clamped / p_mean_pa).clamp(0.0, 0.15)
}
```

- [x] `eyring_traction_advanced()` 구현 (Eyring sinh⁻¹ + Roelands + 전단제한)
- [x] 테스트: SRR=0 → μ≈0, SRR→∞ → μ→τ_lim/p (포화 ≤ 0.10)
- [x] 테스트: 트랙션 커브 형상 — 단조 증가 확인
- [x] 테스트: Roelands vs Barus η_eff 차이가 트랙션에 미치는 영향

### Task 2.2: Carreau-Yasuda 전단 박화 (선택적 확장)

고전단율에서 점도 감소를 모델링. Eyring 모델과 결합:

```
η_eff(γ̇) = η_Roelands × [1 + (λ_CY × γ̇)^a]^((n-1)/a)

여기서:
  λ_CY — 완화 시간 [s] (윤활유 고유값)
  a — Yasuda 지수 (보통 2)
  n — 멱법칙 지수 (보통 0.3-0.8)
```

> **참고**: Carreau-Yasuda는 Eyring sinh⁻¹과 물리적으로 겹치는 영역이 있다.
> 실용적으로는 Eyring 모델이 베어링 트랙션 범위에서 충분하므로,
> Carreau-Yasuda는 **향후 확장 옵션**으로 두고 Phase 2에서는 구현하지 않는다.

- [x] 인터페이스만 정의 — Carreau-Yasuda는 향후 확장 옵션으로 유보
- [x] Roelands 모델이 Advanced 모드에서 자동 적용 (ViscosityModel enum 불필요)

### Task 2.3: 조도 접촉 모델 — M-K 통합 vs KE (선택적 확장)

> **설계 변경**: Masjedi-Khonsari (2015) 공식이 조도 하중 분율(F_a/F)과 접촉 면적 분율(A_a/A_H)을
> 유막두께와 함께 직접 제공하므로, **별도의 GT/KE 조도 접촉 모델이 불필요**해졌다.

**Advanced 모드 기본 동작**: M-K 공식의 `load_fraction`과 `area_fraction`을 그대로 사용하여
혼합윤활 마찰 계수를 계산:
```
μ_mixed = (1 - F_a/F) × μ_ehl + (F_a/F) × μ_boundary
```

**Kogut-Etsion (KE)는 향후 확장 옵션**으로 남겨둔다:
- M-K는 "평균적" 조도 응답을 회귀식으로 제공 → 특수 조건(극심한 경계윤활)에서 정밀도 한계 가능
- KE는 개별 조도 단위의 탄소성 역학을 모델링 → 소성 마모 예측에 유리
- 향후 Phase에서 KE를 M-K와 병행 사용하는 하이브리드 모델 검토 가능

- [x] M-K `load_fraction`을 혼합윤활 마찰 계산에 직접 적용
- [x] KE 인터페이스 — M-K가 조도 분율을 직접 제공하므로 KE 불필요 (향후 확장 옵션)
- [x] M-K vs GT 일관성 검증 (σ̄=0→0, 극단→단조 증가)

### Task 2.4: compute_traction_advanced() 통합

Advanced 트랙션 계산 함수:

```rust
/// Advanced traction computation using:
///   - Eyring sinh⁻¹ traction (SRR-dependent)
///   - Roelands viscosity (pressure-dependent)
///   - Kogut-Etsion asperity model (elasto-plastic)
///   - Murch-Wilson thermal correction (SRR-dependent)
pub fn compute_traction_advanced(
    geom: &MacroGeometry,
    material: &Material,
    operating: &OperatingConditions,
    roller_profile: &RollerProfile,
    raceway_geom: &RacewayGeometry,
    raceway_inner: &RacewayProfile,
    raceway_outer: &RacewayProfile,
    slice_geometries: &[SliceGeometry],
    roller_results: &[RollerResult],
) -> Option<TractionSummary>
```

내부 흐름:
```
1. 운동학: compute_trb_kinematics() — 기존 그대로 재사용
2. 슬라이스별 슬라이딩: compute_slice_sliding() — 기존 그대로 재사용
3. 각 롤러/슬라이스에 대해:
   a. 유막두께 + 조도 분율: Masjedi-Khonsari    ← NEW (D-H + GT 동시 대체)
   b. 열보정: Murch-Wilson (SRR 의존)           ← NEW
   c. 유효 점도: Roelands η(p)                 ← NEW
   d. 전단율: γ̇ = u_slide / h_c
   e. EHL 트랙션: Eyring sinh⁻¹(η_eff×γ̇/τ₀)   ← NEW
   f. 유효 마찰: μ = (1−F_a/F)×μ_ehl + (F_a/F)×μ_boundary  ← M-K load_fraction 사용
4. 동력 손실 합산 — 기존 구조 동일
```

- [x] `compute_traction_advanced()` 구현
- [x] 기존 `compute_traction()`과 동일 출력 형식 검증 (TractionSummary)
- [x] Basic vs Advanced 결과 비교 테스트 (매끈 표면에서 M-K ≈ D-H < 5%)

---

## Phase 3: 물리 기반 기아 + 열 모델

### Task 3.1: 물리 기반 기아 모델

현재 φ_s는 사용자가 임의로 입력하는 상수. Advanced 모드에서는 물리 기반으로 계산한다.

**Hamrock-Dowson (1981) 기아 모델**:
```
φ_s = (h_inlet / h_ff)^(3/11)

여기서:
  h_ff = 완전 충전 유막두께 (기존 D-H 계산)
  h_inlet = 입구 메니스커스 높이

입구 메니스커스 (Chevalier, 1996):
  h_inlet / R_eq = K_inlet × (η₀ × u_m / (E* × R_eq))^(2/3)

그리스 보정 (Lugt, 2013):
  φ_s,grease = φ_s,oil × f_bleed(t, T, NLGI)
  f_bleed = 기유 출혈 팩터 (시간/온도 함수)
```

간소화된 물리 기아 모델 (실용적 접근):

```rust
/// Physics-based starvation factor for Advanced mode.
///
/// For oil lubrication: Hamrock-Dowson inlet meniscus model.
/// For grease: additional base-oil bleeding correction (Lugt 2013).
///
/// References:
///   - Hamrock, B.J. & Dowson, D. (1981)
///   - Chevalier, F. (1996)
///   - Lugt, P.M. (2013), "Grease Lubrication in Rolling Bearings"
fn compute_starvation_factor_advanced(
    eta_0: f64,          // ambient viscosity [Pa·s]
    u_m: f64,            // entraining velocity [m/s]
    e_star: f64,         // combined modulus [Pa]
    r_eq: f64,           // equivalent radius [m]
    lub_type: &LubricationType,
    speed_param: f64,    // n × d_pw (speed factor) [mm·rpm]
) -> f64
```

| 조건 | φ_s 범위 | 물리적 의미 |
|------|----------|------------|
| Oil, 저속 | 0.95-1.0 | 거의 완전 충전 |
| Oil, 고속 | 0.7-0.95 | 원심력에 의한 기아 |
| Grease, 초기 | 0.6-0.8 | 채널링 효과 |
| Grease, 정상 | 0.3-0.6 | 기유 출혈만 |

- [x] `compute_starvation_factor_advanced()` 구현 (n×d_pw 기반 + 그리스 보정)
- [x] Oil vs Grease 경로 분기
- [x] 속도-기아 관계 검증 (고속에서 φ_s 감소)

### Task 3.2: Flash Temperature (Blok-Jaeger)

혼합윤활 영역에서 조도 접촉 시 국부 온도 상승:

```
ΔT_flash = μ × p_a × V_slide / (4 × k_mat × √(π × a_contact × V_slide / (2 × κ_mat)))

여기서:
  k_mat = 열전도도 [W/(m·K)] (베어링강 ≈ 46)
  κ_mat = 열확산도 [m²/s] (베어링강 ≈ 1.2e-5)
  a_contact = 조도 접촉 반경 [m]
```

Flash temperature는 **표면 손상 위험도** 평가에 사용:
- ΔT_flash > 150°C → 스미어링 위험
- ΔT_flash > 300°C → 윤활유 열화 위험

```rust
/// Blok-Jaeger flash temperature estimate at asperity contacts.
///
/// Reference: Blok, H. (1937), Jaeger, J.C. (1942)
fn flash_temperature(
    mu: f64,                // friction coefficient
    p_asperity_pa: f64,     // asperity contact pressure [Pa]
    v_slide: f64,           // sliding velocity [m/s]
    k_steel: f64,           // thermal conductivity [W/(m·K)]
    kappa_steel: f64,       // thermal diffusivity [m²/s]
    a_contact: f64,         // contact half-width [m]
) -> f64
```

- [x] `flash_temperature()` 구현 (Blok-Jaeger band source 모델)
- [x] FilmThicknessResult에 `flash_temp_c: Option<f64>` 필드 추가
- [x] 위험도 분류: Low / Medium / High / Critical (`classify_flash_temp`)
- [x] 테스트: zero sliding → 0, 속도 증가 시 증가, 위험도 분류

### Task 3.3: Advanced 유막두께 분포 계산

`compute_film_thickness_distribution_advanced()`:

```rust
/// Advanced per-slice film thickness distribution.
///
/// Enhancements over Basic:
///   - Murch-Wilson thermal correction (SRR-dependent per slice)
///   - Physics-based starvation factor
///   - Flash temperature estimation at each slice
pub fn compute_film_thickness_distribution_advanced(
    geom: &MacroGeometry,
    material: &Material,
    operating: &OperatingConditions,
    roller_profile: &RollerProfile,
    raceway_inner: &RacewayProfile,
    raceway_geom: &RacewayGeometry,
    slice_geometries: &[SliceGeometry],
    roller_results: &[RollerResult],
) -> Option<Vec<RollerFilmDistribution>>
```

핵심 차이:
- **유막두께**: Masjedi-Khonsari (조도 통합) — D-H 대체
- **조도 하중 분율**: M-K `load_fraction` — GT 대체
- φ_s: `compute_starvation_factor_advanced()` 사용
- φ_T: `thermal_correction_murch_wilson()` (슬라이스별 SRR 의존)
- 추가 출력: flash_temperature, load_fraction, area_fraction

- [x] `compute_film_thickness_distribution_advanced()` 구현
- [x] Basic과 동일한 출력 형식 유지 (RollerFilmDistribution)
- [x] Advanced 전용 진단 필드 추가 (flash_temp_c)

---

## Phase 4: UI + 문서

### Task 4.1: InputPanel — 윤활 모드 선택

```
┌─ Lubrication ─────────────────────┐
│ Model:  [Basic ▼]  /  [Advanced]  │  ← 토글/드롭다운
│                                    │
│ ── Common Parameters ──            │
│ ν₄₀: [68]  ν₁₀₀: [8]  T_op: [70] │
│ α_pv: [20]  Type: [Oil ▼]         │
│                                    │
│ ── Advanced Only ──  (Basic 시 숨김) │
│ τ_eyring: [5] MPa                  │
│ Z_roelands: [0.67]                 │
│ Hardness: [7.5] GPa                │
└────────────────────────────────────┘
```

- [x] LubricationModel 토글 UI 추가
- [x] Advanced 전용 파라미터 조건부 표시
- [x] 기본값 프리셋: 광유 / PAO / 에스터 (τ₀, Z_r 자동 설정)

### Task 4.2: 결과 표시 확장

- [x] Film Thickness 서브탭에 모델 표시 배지 ("Basic" / "Advanced")
- [x] Advanced 시 추가 정보: 수식 설명, φ_s/φ_T 소스 라벨
- [x] Traction 서브탭에 트랙션 커브 차트 추가 (μ vs SRR, Advanced만)
- [x] Mixed Lubrication 서브탭에 M-K vs GT 모델명 표시

### Task 4.3: Manual 업데이트

- [x] Manual/14_Lubrication.md — §14.3A에 Advanced 모드 전체 문서화
- [x] 각 서브 모델의 수식, 참고 문헌, 적용 범위 문서화 (§14.2.3, §14.3A, §14.6.2, §14.6A)
- [x] Basic vs Advanced 비교 표 추가 (§14.3A.4)

---

## Phase 5: 검증 및 교차 비교

### Task 5.1: Basic ↔ Advanced 교차 검증

| 조건 | 기대 결과 |
|------|-----------|
| 저속/저하중/Oil | Basic ≈ Advanced (차이 < 5%) |
| 고속/고하중 | Advanced의 φ_T가 더 낮음 (열효과) |
| 혼합윤활 (Λ<2) | KE의 γ_a > GT의 γ_a (소성 효과) |
| 고SRR | Advanced μ_ehl >> Basic μ_ehl (Eyring sinh⁻¹ 효과) |
| 그리스 | Advanced φ_s < Basic φ_s(고정) (물리 기아 효과) |

- [x] 조건별 비교 테스트 작성 (σ̄→0 수렴, 조도 단조, Roelands vs Barus, Basic vs Advanced)
- [ ] Bearinx/MESYS 공개 벤치마크 데이터 대비 검증 (향후 데이터 확보 시)
- [x] 결과를 검증 테이블로 문서화 (Manual §14.14.2)

### Task 5.2: 과도 슬라이딩 플랜과의 연계

과도 슬라이딩 플랜 ([20260309_Transient_Sliding_Analysis.md](20260309_Transient_Sliding_Analysis.md))의 Task 2.2 "EHL 견인 계수 모델"은 본 플랜의 Task 2.1 "Eyring sinh⁻¹ 트랙션"과 **동일한 모델**이다.

**연계 방안**:
- 본 플랜의 `eyring_traction_coefficient()`를 과도 솔버에서도 호출
- 과도 솔버는 Advanced 모드의 트랙션 함수를 직접 사용
- 기아 모델도 과도 해석에서 시간 의존 φ_s(t)로 확장 가능

- [x] 과도 솔버에서 Advanced 트랙션 함수 재사용 확인 (eyring_traction_advanced pub 공유)
- [x] 인터페이스 호환성 검증 (LubricationModel 분기, 144 테스트 통과)

---

## 의존성 및 Critical Path

```
Phase 1 (인프라 + 유변학 + M-K 유막)
  ├── Task 1.1 (타입/enum) ──────────────────┐
  ├── Task 1.2 (Roelands) ──────────────────┤
  ├── Task 1.3 (Masjedi-Khonsari 유막) ◀── CRITICAL PATH
  ├── Task 1.4 (Murch-Wilson 열보정) ───────┤
  └── Task 1.5 (디스패치 분기) ◀────────────┘
                                              │
Phase 2 (트랙션)                             ▼
  ├── Task 2.1 (Eyring sinh⁻¹) ◀── CRITICAL PATH
  ├── Task 2.2 (Carreau-Yasuda 인터페이스 — 선택적)
  ├── Task 2.3 (M-K 조도 분율 적용 + KE 인터페이스)
  └── Task 2.4 (통합 Advanced 트랙션) ◀─────┘
                                              │
Phase 3 (기아 + 열)                          ▼
  ├── Task 3.1 (물리 기아)
  ├── Task 3.2 (Flash temperature)
  └── Task 3.3 (Advanced 유막 분포)
                                              │
Phase 4 (UI + 문서)                          ▼
  ├── Task 4.1 (입력 UI)
  ├── Task 4.2 (결과 표시)
  └── Task 4.3 (Manual 업데이트)
                                              │
Phase 5 (검증)                               ▼
  ├── Task 5.1 (교차 검증)
  └── Task 5.2 (과도 솔버 연계)
```

**Critical Path**: Task 1.1 → Task 1.3 (M-K) → Task 2.1 → Task 2.4 → Task 3.3 → Task 5.1

---

## 설계 결정 사항

### D1: Advanced 모드 유막두께를 Masjedi-Khonsari (2015)로 교체
**선택: M-K 회귀식 채택 (Basic은 D-H 유지)**
- 이유: D-H는 매끈한 표면 가정이므로 별도 GT 조도 모델이 필요하다. M-K는 **조도 효과를 유막두께 공식에 직접 통합**하여 GT 모델과 유막 계산을 하나로 합친다.
- M-K의 σ̄→0 극한은 D-H/D-T와 수렴하므로 매끈한 표면에서도 일관된 결과를 제공한다.
- Full numerical EHL 없이도 조도→혼합윤활 전이를 연속적으로 포착할 수 있다.
- 추가 입력은 표면 조도(R_q)뿐이므로 사용자 부담이 최소화된다.
- **참고 문헌**: Masjedi, M. & Khonsari, M.M. (2015), Tribology International, 82, 228-244.

### D2: Carreau-Yasuda는 구현하지 않는다
**선택: Eyring sinh⁻¹으로 충분**
- 이유: 베어링 트랙션 범위(SRR < 5%)에서 Eyring과 Carreau-Yasuda의 차이가 작다.
- Deliverables에서 제거. 향후 필요 시 ViscosityModel enum 확장으로 대응.

### D3: 조도 접촉 모델 — M-K 통합 우선, KE는 선택적 확장
**선택: M-K load_fraction 사용, KE는 인터페이스만 확보**
- 이유: M-K가 유막두께와 함께 조도 하중 분율/면적 분율을 직접 제공하므로 별도 GT/KE 불필요.
- KE는 극심한 경계윤활이나 소성 마모 정밀 예측이 필요한 경우를 위해 향후 확장 옵션으로 남긴다.
- 사용자 추가 입력: R_q (표면 조도)만 — 경도 입력은 KE 구현 시에만 필요.

### D4: 기아 모델은 정상상태만 (시간 의존 없음)
**선택: 속도/온도 기반 φ_s 계산 (시간 비의존)**
- 이유: 시간 의존 기아(그리스 열화)는 과도 해석 범위.
- 정상상태에서는 속도-온도 조건에서의 평형 φ_s로 충분.

### D5: FilmThicknessResult 확장 방식
**선택: 기존 필드 유지 + Option 필드 추가**
- `flash_temp_c: Option<f64>` — Advanced에서만 Some
- `starvation_calculated: Option<f64>` — Advanced에서만 Some
- Basic에서는 None → UI에서 숨김

---

## Basic vs Advanced 비교 (업데이트)

| 영역 | Basic | Advanced |
|------|-------|----------|
| 유막두께 공식 | Dowson-Higginson (1977) | **Masjedi-Khonsari (2015)** — 조도 통합 |
| 압력-점도 | Barus (선형 α) | Roelands (1966) |
| 트랙션 | μ = τ₀/p_mean (상수) | Eyring sinh⁻¹ (SRR 의존) |
| 혼합윤활 조도 | Greenwood-Tripp (1970) | **M-K load_fraction** (GT 대체) |
| 열보정 | Gupta φ_T (단일값) | Murch-Wilson (SRR 의존) |
| 기아 | φ_s = 사용자 상수 | Hamrock-Dowson + Lugt 물리 모델 |
| Flash temperature | 없음 | Blok-Jaeger |

---

## 참고 문헌 (Advanced 모델)

1. **Masjedi, M. & Khonsari, M.M.** (2015). "On the effect of surface roughness in point-contact EHL: Formulas for film thickness and asperity load," Tribology International, 82, 228-244. — **Advanced 유막두께 + 조도 통합 공식**.
2. **Roelands, C.J.A.** (1966). PhD Thesis, TU Delft. — 압력-점도 관계.
3. **Johnson, K.L. & Tevaarwerk, J.L.** (1977). "A New Solution for EHL Line Contacts," Proc. R. Soc. London. — Eyring 트랙션.
4. **Bair, S. & Winer, W.O.** (1979). "A Rheological Model for EHL Contacts," J. Lubr. Tech. — 비뉴턴 유변학.
5. **Bair, S.** (2019). "High-Pressure Rheology for Quantitative Elastohydrodynamics," Elsevier. — 최신 유변학 총정리.
6. **Kogut, L. & Etsion, I.** (2002). "Elastic-Plastic Contact Analysis of a Sphere and a Rigid Flat," J. Appl. Mech. — 탄소성 조도 모델 (향후 확장).
7. **Murch, L.E. & Wilson, W.R.D.** (1975). "A Thermal EHL Inlet Zone Analysis," J. Lubr. Tech. — SRR 의존 열보정.
8. **Lugt, P.M.** (2013). "Grease Lubrication in Rolling Bearings," Wiley. — 그리스 기아 모델.
9. **Hamrock, B.J. & Dowson, D.** (1981). "Ball Bearing Lubrication," Wiley. — 기아 모델.
10. **Blok, H.** (1937). "Theoretical Study of Temperature Rise at Surfaces," Proc. IMechE. — Flash temperature.
11. **Habchi, W.** (2018). "Finite Element Modeling of EHL Contacts," Wiley. — Thermal EHL 참고.
12. **Morales-Espejel, G.E.** (2014). "Surface Life with Contaminated Lubrication," SKF Evolution. — Micro-EHL 참고.
