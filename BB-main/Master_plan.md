# CRB Contact Analysis System

**Dual-Mode Slicing: Gen1 (Independent) + Gen3 (Beam-Coupled)**

> ⚠️ **NOTE (Phase 0 직후)**: 본 문서는 [TRB-main/Master_plan.md](../TRB-main/Master_plan.md) 의 복제본 상태이며, 아래 모든 식/구조체는 **TRB 기준**으로 작성되어 있음. CRB 기준 재작성은 Phase 1 이후 단계적으로 진행. CRB ↔ TRB 차이점 요약은 [CRB_Development_Plan.md](CRB_Development_Plan.md) 참조.

---

## 1. System Overview

본 시스템은 CRB(Cylindrical Roller Bearing)의 내부 접촉 해석을 두 가지 슬라이싱 모드로 수행한다. Gen1(독립 슬라이스)은 빠른 초기 설계 검토에, Gen3(빔 기반 커플링)은 정밀 해석과 에지 응력 예측에 적합하며, 동일 입력/출력 인터페이스를 공유하여 두 결과의 직접 비교가 가능하다.

> 아래 본문 (~700+줄) 의 수식·구조체 정의는 현재 TRB 기준 그대로이며, Phase 1 단계에서 CRB 기준 (α=0, β=0, rib 옵션, 균일 D_we 등) 으로 재작성 예정.

### 1.1 Dual-Mode Comparison

| Aspect | Gen1: Independent Slice | Gen3: Beam-Coupled Slice |
|--------|------------------------|--------------------------|
| Slice interaction | None (each slice independent) | Timoshenko beam + inter-slice spring |
| Roller bending | Not considered | Fully considered (variable I_k) |
| Edge stress prediction | Poor (truncated contact only) | Accurate (pressure concentration) |
| Computation speed | Fast — O(n) per roller | Moderate — O(n²) matrix solve |
| Use case | Initial sizing, parametric sweep | Final validation, profile optimization |
| ISO 16281 compliance | Basic (Annex method) | Full (advanced method) |

### 1.2 Analysis Pipeline

```
Input Module → Pre-processor → Solver (Gen1/Gen3) → Post-processor → Monitoring
                                    ↑ mode select
```

- **Input Module**: Macro/Micro geometry, load, material
- **Pre-processor**: Slicing, profile interpolation, mode select
- **Solver**: Gen1 direct / Gen3 Beam FE + Newton-Raphson
- **Post-processor**: Stress, life, load distribution
- **Monitoring**: Dashboard, Gen1↔Gen3 comparison

---

## 2. Input Variables

### 2.1 Macro Geometry (Bearing Level)

```rust
pub struct MacroGeometry {
    pub d: f64,          // Bore diameter [mm]
    pub D: f64,          // Outer diameter [mm]
    pub T: f64,          // Bearing width [mm]
    pub alpha: f64,      // Contact angle (half-taper) [deg]
    pub Z: u32,          // Number of rollers [-]
    pub D_we_max: f64,   // Roller large-end diameter [mm]
    pub D_we_min: f64,   // Roller small-end diameter [mm]
    pub L_we: f64,       // Roller effective contact length [mm]
    pub d_pw: f64,       // Pitch circle diameter [mm]
    pub h_rib: f64,      // Rib height (large-end) [mm]
    pub alpha_rib: f64,  // Rib angle [deg]
    pub G_r: f64,        // Radial internal clearance [μm]
}
```

### 2.2 Raceway Geometry

```rust
pub struct RacewayGeometry {
    pub alpha_i: f64,    // Inner raceway taper angle [deg]
    pub alpha_o: f64,    // Outer raceway taper angle [deg]
    pub R_i: f64,        // Inner raceway transverse curvature radius [mm]
    pub R_o: f64,        // Outer raceway transverse curvature radius [mm]
    pub r_rib: f64,      // Large-end rib fillet radius [mm]
    pub d_uc: f64,       // Raceway undercut depth [mm]
    pub L_uc: f64,       // Raceway undercut axial extent [mm]
}
```

### 2.3 Micro Geometry (Profile Modification)

롤러 및 레이스웨이의 미시 형상은 접촉 압력 분포와 에지 응력에 결정적 영향을 미치며, 이 시스템의 핵심 입력이다.

```rust
#[derive(Clone)]
pub enum CrownType {
    Logarithmic { A_log: f64 },  // Reusner log profile parameter
    Circular { R_crown: f64 },    // Crown radius [mm]
    Parabolic { c2: f64 },        // Parabolic coefficient
    Custom { profile: Vec<(f64, f64)> },  // (x_mm, dz_um) data points
}

pub struct RollerProfile {
    pub crown_type: CrownType,
    pub delta_c: f64,        // Crown drop center-to-end [μm]
    pub delta_dub_L: f64,    // Dub-off amount large end [μm]
    pub delta_dub_S: f64,    // Dub-off amount small end [μm]
    pub L_dub_L: f64,        // Dub-off length large end [mm]
    pub L_dub_S: f64,        // Dub-off length small end [mm]
    pub R_sph: f64,          // Roller large-end sphere radius [mm] (rib contact)
}

pub struct RacewayProfile {
    pub delta_rw: f64,       // Raceway crowning [μm]
    pub W_a: f64,            // Axial waviness amplitude [μm]
    pub Ra: f64,             // Surface roughness Ra [μm]
    pub custom_profile: Option<Vec<(f64, f64)>>,  // (x_mm, dz_um)
}
```

### 2.4 Material Properties

```rust
pub struct Material {
    pub E_roller: f64,   // Young's modulus roller [GPa]
    pub E_ring: f64,     // Young's modulus rings [GPa]
    pub nu: f64,         // Poisson's ratio [-]
    pub HRC: f64,        // Surface hardness [HRC]
}
```

### 2.5 Operating Conditions

```rust
pub struct OperatingConditions {
    pub F_r: f64,        // Radial load [kN]
    pub F_a: f64,        // Axial load [kN]
    pub M: f64,          // Tilting moment [kN·m]
    pub n_rpm: f64,      // Rotational speed [rpm]
    pub gamma: f64,      // Misalignment angle [arcmin]
    pub T_op: f64,       // Operating temperature [°C]
    pub nu_40: f64,      // Kinematic viscosity at 40°C [mm²/s]
    pub nu_100: f64,     // Kinematic viscosity at 100°C [mm²/s]
}
```

### 2.6 Solver Parameters

```rust
#[derive(Clone, Copy, PartialEq)]
pub enum SolverMode {
    Gen1,  // Independent slice
    Gen3,  // Beam-coupled slice
}

#[derive(Clone, Copy, PartialEq)]
pub enum RunMode {
    Single(SolverMode),    // Run one mode only
    Dual,                  // Run both, compare results
}

#[derive(Clone, Copy, PartialEq)]
pub enum BeamType {
    EulerBernoulli,
    Timoshenko,
}

pub struct SolverParams {
    pub run_mode: RunMode,           // default: Single(Gen3)
    pub n_slices: usize,             // default: 50~100
    pub beam_type: BeamType,         // default: Timoshenko (Gen3 only)
    pub convergence_tol: f64,        // default: 1e-6
    pub max_iterations: usize,       // default: 100
    pub angular_increment_deg: f64,  // default: 1.0~5.0 [deg]
}
```

### 2.7 Top-Level Input

```rust
pub struct BearingInput {
    pub macro_geom: MacroGeometry,
    pub raceway_geom: RacewayGeometry,
    pub roller_profile: RollerProfile,
    pub raceway_profile_inner: RacewayProfile,
    pub raceway_profile_outer: RacewayProfile,
    pub material: Material,
    pub operating: OperatingConditions,
    pub solver: SolverParams,
}
```

---

## 3. Calculation Method

### 3.1 Overall Calculation Flow

전체 계산은 3개 레벨의 중첩 반복으로 구성되며, Level 2(롤러 레벨)에서 Gen1과 Gen3이 분기된다. Level 1, Level 3, 그리고 전/후처리는 동일한 코드를 공유한다.

- **Level 1 (Bearing Level)**: 외부 하중(Fr, Fa, M)에 대한 내링 변위 (δx, δy, δz, γx, γy)를 구하는 5-DOF 평형 해석. 각 롤러별 법선력 Q_j를 산출. `[공통]`
- **Level 2 (Roller Level)**: 각 롤러 j에 대해 법선력 Q_j가 주어졌을 때, 롤러 축방향 하중 분포 q_k (k=1..n)를 구한다. `[Gen1/Gen3 분기점]`
- **Level 3 (Slice Level)**: 각 slice k에서 Hertz line contact 계산. 접촉 반폭 b_k, 접촉 응력 p_max_k, Weber 변형량 h_k를 산출. `[공통]`

```
┌─────────────────────────────────────────────────────┐
│ Level 1: Bearing 5-DOF Equilibrium  [공통]           │
│   F_ext → {δx, δy, δz, γx, γy} → Q_j per roller   │
│                                                      │
│   ┌─────────────────────────────────────────┐       │
│   │ Level 2: Roller Load Distribution        │       │
│   │                                          │       │
│   │   ┌──────────┐    ┌──────────────────┐  │       │
│   │   │   Gen1   │ OR │      Gen3        │  │       │
│   │   │ δ_k =    │    │ [K_beam]{w} +    │  │       │
│   │   │ δ_rigid  │    │ f_contact(δ) =   │  │       │
│   │   │ - Δz_k   │    │ F_ext            │  │       │
│   │   └──────────┘    └──────────────────┘  │       │
│   │                                          │       │
│   │   ┌──────────────────────────────────┐  │       │
│   │   │ Level 3: Slice Hertz/Weber [공통] │  │       │
│   │   │  b_k, p_max_k, h_k per slice    │  │       │
│   │   └──────────────────────────────────┘  │       │
│   └─────────────────────────────────────────┘       │
│                                                      │
│   + Rib contact (point contact)  [공통]               │
│   + Fatigue life ISO 16281       [공통]               │
└─────────────────────────────────────────────────────┘
```

### 3.2 Common: Geometry Pre-processing

#### 3.2.1 Slicing Discretization

롤러 유효 접촉 길이 `L_we`를 `n`개 slice로 등분한다. 각 slice k (k=1..n)에서:

- 롤러 반경: `r_roller_k = r_small + (r_large - r_small) × (k-0.5)/n`
- 내륜 레이스웨이 곡률반경: `r_i_k` (테이퍼 형상에서 slice 위치별 산출)
- 외륜 레이스웨이 곡률반경: `r_o_k`
- 등가 곡률반경: `R_k = r_roller_k × r_race_k / (r_roller_k + r_race_k)`
- 프로파일 보정량: `Δz_k` (크라우닝 + dub-off + 측정 프로파일 보간)

```rust
pub struct SliceGeometry {
    pub k: usize,              // slice index (0-based)
    pub x_axial: f64,          // axial position from small end [mm]
    pub r_roller: f64,         // roller radius at this slice [mm]
    pub r_inner_race: f64,     // inner raceway curvature radius [mm]
    pub r_outer_race: f64,     // outer raceway curvature radius [mm]
    pub R_eq_inner: f64,       // equivalent radius (roller-inner) [mm]
    pub R_eq_outer: f64,       // equivalent radius (roller-outer) [mm]
    pub delta_z_total: f64,    // total profile correction [μm]
    pub slice_width: f64,      // l_k = L_we / n [mm]
}
```

#### 3.2.2 Profile Superposition

각 slice 위치에서의 총 형상 보정량:

```
Δz_total_k = Δz_roller_k + Δz_raceway_inner_k + Δz_raceway_outer_k
```

Custom profile이 있을 경우 cubic spline으로 보간하여 각 slice 중심 좌표에 매핑한다.

#### 3.2.3 Common: Slice-Level Contact (Level 3)

Gen1과 Gen3 모두 각 slice에서 동일한 Hertz/Weber 계산을 수행. 차이는 각 slice에 주어지는 접근량 `δ_k`의 결정 방법이다.

- Hertz 접촉 반폭: `b_k = √(8F_k(1-ν²)r_1k·r_2k / (π·l_k·E·(r_1k+r_2k)))`
- 최대 접촉 응력: `p_max_k = 2q_k / (π·b_k)`
- Weber 벌크 변형: `h_k = 4F_k(1-ν²)/(π·l_k·E) × [ln(2√(h_1k×h_2k)/b_k) - ν/(2(1-ν))]`
- Slice 강성 (접촉+벌크 직렬): `K_total_k = 1 / (1/K_hertz_k + 1/K_bulk_k)`

```rust
pub struct SliceContactResult {
    pub k: usize,
    pub delta_k: f64,      // approach amount [μm]
    pub q_k: f64,          // load per unit length [N/mm]
    pub b_k: f64,          // contact half-width [mm]
    pub p_max_k: f64,      // max Hertz contact stress [MPa]
    pub h_bulk_k: f64,     // Weber bulk deformation [μm]
    pub K_hertz_k: f64,    // Hertz contact stiffness [N/mm/μm]
    pub K_bulk_k: f64,     // bulk stiffness [N/mm/μm]
    pub in_contact: bool,  // δ_k > 0
}
```

---

### 3.3 Gen1: Independent Slice Method

Gen1에서는 각 slice가 완전히 독립적인 비선형 스프링으로 작동한다. 롤러 굽힘 변형과 slice 간 상호작용을 무시한다.

#### 3.3.1 Approach per Slice

각 slice k의 접근량은 rigid body 변위에서 프로파일 보정만 차감:

```
δ_k = δ_rigid(k) - Δz_total_k
```

`δ_rigid(k)`는 내링 변위와 롤러 위치 ψ, slice 축방향 위치에서 기하학적으로 결정된다.

#### 3.3.2 Load-Displacement Relation

```
if δ_k > 0:  q_k = C_k × δ_k^(10/9)   // Palmgren line contact exponent
if δ_k ≤ 0:  q_k = 0                    // no contact
```

`C_k`는 slice 위치별 Hertz 접촉 상수로 등가 곡률반경 `R_k`와 재료 물성, slice 폭의 함수. TRB에서는 소단↔대단 간 `C_k`가 달라 비균일 하중 분포를 부분적으로 반영.

#### 3.3.3 Roller Total Load

```
Q_j = Σ(q_k × l_k)  for k = 1..n
```

이 값이 Level 1 베어링 평형에서 요구하는 `Q_j`와 일치하도록 `δ_rigid`를 조정.

#### 3.3.4 Gen1 Limitations

- 롤러 굽힘 무시 → misalignment 시 에지 응력을 과소평가
- Slice 간 연속성 없음 → 프로파일 불연속점에서 비물리적 하중 분포
- Weber 벌크 변형의 slice 간 커플링 무시 → 접촉 폭 전이구간 부정확
- **장점**: 계산 속도 빠름, 구현 단순, 파라메트릭 스터디에 적합

---

### 3.4 Gen3: Beam-Coupled Slice Method

Gen3에서는 롤러를 Timoshenko beam으로 모델링하고, 각 slice 절점에 비선형 접촉 스프링을 부착하여 롤러 변형과 접촉을 커플링한다.

#### 3.4.1 Roller Beam Element

각 slice를 하나의 beam 절점으로 취급. 요소 강성 행렬 포함 요소:

- Bending stiffness: `EI_k` (단면2차모멘트는 slice 위치의 롤러 반경으로 결정)
- Shear stiffness: `GA_s_k` (κ = 10/9 for circular section, Timoshenko)
- Section properties: `A_k = π × r_roller_k²`, `I_k = π/4 × r_roller_k⁴`

글로벌 강성 행렬 `[K_beam]` 크기: `2n × 2n` (n개 절점, 절점당 2 DOF: 변위 `w_k`, 회전 `θ_k`). 테이퍼 롤러는 소단→대단으로 갈수록 단면이 커지므로 `EI_k`가 위치에 따라 비균일.

```rust
/// Timoshenko beam element stiffness matrix (4×4 per element)
/// Connects node k to node k+1
pub fn beam_element_stiffness(
    E: f64,          // Young's modulus
    I_k: f64,        // second moment of area at element center
    A_k: f64,        // cross-section area
    G: f64,          // shear modulus = E / (2(1+ν))
    kappa: f64,      // shear correction factor (10/9 for circle)
    L_e: f64,        // element length = L_we / n
) -> nalgebra::Matrix4<f64> { ... }

/// Assemble global beam stiffness matrix (2n × 2n, banded/sparse)
pub fn assemble_beam_stiffness(
    slices: &[SliceGeometry],
    material: &Material,
    beam_type: BeamType,
) -> sprs::CsMat<f64> { ... }
```

#### 3.4.2 Contact Stiffness at Each Slice

각 slice k의 접촉 강성은 비선형 (Hertz line contact):

```
q_k = f(δ_k, R_k, E', l_k)
K_c_k = dq_k/dδ_k    // tangent stiffness (linearized)
```

비선형 접촉 스프링이 각 beam 절점에 부착 → 전체 시스템 강성 행렬.

#### 3.4.3 Bulk (Sub-surface) Deformation

Weber 수식에 의한 벌크 변형 강성이 Hertz 접촉 강성과 직렬로 작용. sear length `h_ik`는 slice 위치별 롤러 반경에 따라 달라지므로 벌크 강성도 위치 종속.

#### 3.4.4 Coupled System Equation

```
[K_beam]{w} + {f_contact(δ)} = {F_external}
```

- `{w}`: 각 slice 절점의 변위/회전 벡터
- `{δ_k}`: 접촉 접근량 = `δ_rigid_k - w_k - Δz_total_k`
- `{f_contact}`: 비선형 Hertz 접촉력 + Weber 벌크 변형력
- Complementarity 조건: `δ_k ≥ 0` → contact, `δ_k < 0` → no contact (`q_k = 0`)

**Gen1과의 핵심 차이**: Gen1에서는 `[K_beam]{w}` 항이 0이므로 `δ_k = δ_rigid_k - Δz_total_k`로 단순화.

#### 3.4.5 Newton-Raphson Solution

Jacobian:

```
[J] = [K_beam] + diag(K_c_k + K_bulk_k)  // 접촉 slice만
```

수렴 판정: `||Δw||/||w|| < ε` and `||residual|| < ε_abs`

접촉/비접촉 경계가 변하면 active set을 갱신하고 재반복.

```rust
pub struct Gen3Solver {
    pub K_beam: sprs::CsMat<f64>,    // global beam stiffness (sparse)
    pub active_set: Vec<bool>,        // contact/no-contact per slice
    pub w: nalgebra::DVector<f64>,    // displacement/rotation vector
    pub residual: nalgebra::DVector<f64>,
}

impl Gen3Solver {
    pub fn solve(
        &mut self,
        slices: &[SliceGeometry],
        delta_rigid: &[f64],
        params: &SolverParams,
    ) -> Vec<SliceContactResult> { ... }
}
```

---

### 3.5 Common: Rib Contact (Large-end)

롤러 대단면-내륜 리브 접촉은 별도의 Hertzian point contact (타원 접촉)으로 해석.

- 입력: 롤러 대단 구면 반경 `R_sph`, 리브 형상 (`R_rib`, `α_rib`)
- 리브 접촉력: 롤러 축방향 평형에서 산출 (`Q × sinα`에 대한 반력)
- 접촉 타원 반경 `a`, `b` 및 최대 접촉 응력 `p_max_rib` 계산
- 마찰 발열 및 스핀 모멘트 산출 (고속 시 중요)

```rust
pub struct RibContactResult {
    pub F_rib: f64,        // rib contact force [N]
    pub a_ellipse: f64,    // contact ellipse semi-axis a [mm]
    pub b_ellipse: f64,    // contact ellipse semi-axis b [mm]
    pub p_max_rib: f64,    // max contact stress [MPa]
    pub spin_moment: f64,  // spin moment [N·mm]
}
```

### 3.6 Common: Bearing-Level Load Distribution

전체 베어링의 5-DOF 평형 해석 (Gen1, Gen3 공통 프레임워크, Level 2 솔버만 교체):

- 내링 변위 벡터: `{δx, δy, δz, γx, γy}`
- 각 롤러 위치 `ψ_j` (j=1..Z)에서 기하학적 간섭량 산출
- Gen1: 각 롤러의 `q_k`를 독립 slice 방식으로 직접 산출
- Gen3: 각 롤러의 `q_k`를 beam-coupled 시스템으로 산출
- 전체 평형: `ΣF_j = F_external`, Newton-Raphson 수렴
- Dual mode 시: Gen1 → Gen3 순서로 수행, Gen1 결과를 Gen3 초기값으로 활용하여 수렴 가속

```rust
pub struct BearingEquilibrium {
    pub displacement: [f64; 5],  // [δx, δy, δz, γx, γy]
    pub roller_loads: Vec<f64>,  // Q_j per roller [N]
    pub roller_results: Vec<RollerResult>,
}

pub struct RollerResult {
    pub psi_deg: f64,                        // angular position [deg]
    pub Q_normal: f64,                       // normal force [N]
    pub slice_results: Vec<SliceContactResult>,
    pub rib_result: RibContactResult,
}
```

### 3.7 Common: Fatigue Life Calculation

ISO 16281에 따른 수정 기준 수명 계산:

- 각 slice별 등가 동하중 `q_ek` 산출
- Lamina life: `L_10_k = (Q_c_k / q_ek)^(10/3)` for line contact
- Roller life: `L_10_roller = [Σ(1/L_10_k)^e]^(-1/e)`, Lundberg-Palmgren 합성
- Bearing life: `L_10_bearing` from all rollers, inner/outer ring 분리 계산
- 수정 수명: `L_nm = a_ISO × L_10` (a_ISO는 κ, e_C/P, contamination 함수)

```rust
pub struct FatigueLifeResult {
    pub L_10_basic: f64,          // basic rating life [10⁶ rev]
    pub L_nm_hours: f64,          // modified reference life [hours]
    pub L_10_inner: f64,          // inner ring life [10⁶ rev]
    pub L_10_outer: f64,          // outer ring life [10⁶ rev]
    pub weakest_lamina: usize,    // slice index with minimum life
    pub a_iso: f64,               // life modification factor
    pub kappa: f64,               // viscosity ratio
}
```

---

## 4. Result Monitoring Dashboard

### 4.1 Primary Output Variables

```rust
pub struct BearingResult {
    pub mode: SolverMode,
    pub equilibrium: BearingEquilibrium,
    pub life: FatigueLifeResult,
    pub alerts: Vec<Alert>,
}

pub struct Alert {
    pub level: AlertLevel,    // Info, Warning, Critical
    pub category: String,     // "EdgeStress", "RibStress", "Life", etc.
    pub message: String,
    pub value: f64,
    pub threshold: f64,
}
```

| Category | Variable | Unit | Alert Criteria |
|----------|----------|------|---------------|
| **Contact Stress** | | | |
| Max Hertz stress | p_max | MPa | > 4000 MPa (steel) |
| Edge stress ratio | p_edge/p_center | [-] | > 1.5 (excessive edge loading) |
| Rib contact stress | p_rib | MPa | > 1500 MPa |
| **Load Distribution** | | | |
| Roller load (max) | Q_max | kN | Q_max/Q_mean > 5 |
| Load zone extent | ψ_load | [°] | < 120° |
| Slice load uniformity | q_max/q_min | [-] | > 3.0 in loaded roller |
| **Deformation** | | | |
| Max roller deflection | w_max | μm | > 10 μm (Gen3 only) |
| Ring tilt | γ_eff | arcmin | > permissible misalignment |
| **Fatigue Life** | | | |
| Basic rating life L10 | L_10 | 10⁶ rev | < target life |
| Modified reference life | L_nm | hours | < required service life |
| Weakest lamina location | k_min_life | slice # | Near edge → edge loading |

### 4.2 Visualization Outputs

#### 4.2.1 Roller-Level Plots (per roller position ψ)

- **Plot A**: Slice load distribution `q_k` vs. axial position — 실선(계산), 점선(Hertz 이론 비교)
- **Plot B**: Contact pressure `p_max_k` vs. axial position — 에지 응력 집중 확인
- **Plot C**: Roller deflection `w_k` vs. axial position — beam bending shape (Gen3 only)
- **Plot D**: Contact half-width `b_k` vs. axial position
- **Plot E**: Profile overlay — 입력 프로파일 + 변형 후 프로파일 비교

#### 4.2.2 Bearing-Level Plots

- **Plot F**: Roller load `Q_j` vs. angular position ψ (polar plot) — load zone 시각화
- **Plot G**: Fatigue life distribution `L_10_j` vs. angular position
- **Plot H**: Inner ring displacement orbit (δx, δz plane)

#### 4.2.3 Contour Maps

- **Map I**: Contact pressure field `p(ψ, x_axial)` — 2D heatmap
- **Map J**: Sub-surface von Mises stress `σ_vM(depth, x_axial)` — 최대 하중 롤러에 대해

#### 4.2.4 Gen1 vs Gen3 Comparison (Dual Mode)

Dual mode 실행 시 자동 생성:

- **Plot K**: Slice load overlay `q_k` (Gen1 vs Gen3) — 빔 효과 정량화
- **Plot L**: Edge stress ratio comparison — Gen1 과소평가 정도 시각화
- **Plot M**: Bearing load distribution `Q_j` overlay (Gen1 vs Gen3) — polar plot 중첩
- **Plot N**: Life comparison `L_10` (Gen1 vs Gen3) — bar chart
- **Summary table**: Δp_max, ΔQ_max, ΔL_10 등 주요 차이 자동 산출

→ Gen1 근사의 유효성을 정량 평가하고, Gen3가 필요한 조건(misalignment, 비대칭 프로파일 등)을 자동 식별.

```rust
pub struct DualModeComparison {
    pub gen1_result: BearingResult,
    pub gen3_result: BearingResult,
    pub delta_p_max_pct: f64,       // (gen3-gen1)/gen3 × 100
    pub delta_Q_max_pct: f64,
    pub delta_L10_pct: f64,
    pub gen3_recommended: bool,     // true if difference > threshold
    pub recommendation_reason: String,
}
```

### 4.3 Design Optimization Feedback

| Issue Detected | Root Cause | Recommended Action |
|---------------|------------|-------------------|
| Edge stress > threshold | Insufficient crowning or misalignment | Increase crown drop Δ_c or log profile A |
| Center stress dominant | Excessive crowning | Reduce crown drop, check alignment |
| Low load zone (<120°) | Excessive clearance or light load | Reduce G_r or increase preload |
| High rib stress | Excessive axial load or poor geometry | Review R_sph, r_rib, α_rib |
| Roller bending > 10μm | Moment load or misalignment | Check shaft deflection, alignment |
| Life < target | Multiple possible causes | Iterate on profile + clearance + load |

---

## 5. Implementation Roadmap

### 5.1 Development Phases

| Phase | Scope | Key Deliverable | Duration |
|-------|-------|----------------|----------|
| **Phase 1** | Python prototype: Geometry + Gen1 solver | Hertz/Weber per slice, Gen1 solver, MESYS/MASTA 검증 | 4 weeks |
| **Phase 2** | Python prototype: Gen3 beam-coupled solver | Timoshenko beam, N-R solver, FEA 교차검증 | 5 weeks |
| **Phase 3** | Rust porting: Gen1/Gen3 solver core | nalgebra/sprs 기반 솔버, Python golden test 통과 | 5 weeks |
| **Phase 4** | Tauri app: UI + 3D rendering + charts | React frontend, Three.js 베어링뷰, Plotly 대시보드 | 5 weeks |
| **Phase 5** | Bearing-level equilibrium + rib contact | 5-DOF solver (Rust), dual-mode, MASTA sidecar | 4 weeks |
| **Phase 6** | Life calc + monitoring + report | ISO 16281 life, Gen1↔Gen3 compare, PDF report | 3 weeks |
| **Phase 7** | Validation + optimization + polish | Full benchmark, UX refinement, documentation | 4 weeks |

**Total: ~30 weeks**

### 5.2 Validation Strategy

- **Level A**: 단일 slice Hertz 해석 → 해석해와 비교 (상대오차 < 0.1%) `[Gen1, Gen3 공통]`
- **Level B**: 단일 롤러 Gen3 beam-contact → 상용 FEA (ANSYS/ABAQUS) contact 해석과 비교 (< 3%)
- **Level C**: Gen1 ↔ Gen3 교차 검증 → 정렬 상태에서 두 결과 수렴 확인 (misalignment=0, flat profile)
- **Level D**: 전체 베어링 하중 분포 → Bearinx/MESYS/MASTA 결과와 비교 (< 5%)
- **Level E**: 실험 검증 → strain gauge/변위 센서 측정 데이터와 비교

### 5.3 Technology Stack

| Layer | Technology | Role |
|-------|-----------|------|
| Desktop shell | Tauri 2.0 | App framework, Rust backend hosting |
| Solver core | Rust (nalgebra + sprs) | Gen1/Gen3 solver, beam FE, N-R iteration |
| Frontend framework | React + TypeScript | UI components, state management |
| 3D rendering | Three.js (r128+) | Bearing cross-section, roller contact zone viz. |
| Interactive charts | Plotly.js | Load distribution, stress, life plots |
| MASTA bridge | Tauri sidecar (.NET) | Geometry import via JSON/CSV exchange |
| Profile I/O | serde (Rust) + CSV/JSON | Measured profile import, spline interp. |
| Report generation | HTML → PDF (printpdf crate) | Automated report with embedded plots |
| Prototyping | Python (NumPy/SciPy) | Algorithm validation before Rust porting |

#### 5.3.1 Rust Solver Architecture

Rust 솔버는 Tauri의 command system으로 프론트엔드와 통신. IPC 오버헤드 없이 Rust 함수를 직접 호출하며, 계산 결과를 serde로 직렬화하여 WebView에 전달.

- `nalgebra`: Dense matrix 연산 (slice-level Hertz, 소형 행렬)
- `sprs`: Sparse matrix (beam 글로벌 강성 행렬 2n×2n, banded structure 활용)
- `rayon`: 병렬 처리 (롤러별 독립 계산을 multi-thread로 분배)
- `serde + serde_json`: 솔버 입출력 직렬화, 프론트엔드 통신

```rust
#[tauri::command]
async fn solve_bearing(input: BearingInput) -> Result<BearingResult, String> {
    let result = match input.solver.run_mode {
        RunMode::Single(SolverMode::Gen1) => solve_gen1(&input),
        RunMode::Single(SolverMode::Gen3) => solve_gen3(&input),
        RunMode::Dual => {
            let gen1 = solve_gen1(&input);
            let gen3 = solve_gen3_with_initial(&input, &gen1);  // Gen1 결과를 초기값으로
            create_dual_comparison(gen1, gen3)
        }
    };
    result.map_err(|e| e.to_string())
}
```

#### 5.3.2 WebView Rendering Pipeline

3개 레이어:

- **Three.js 3D View**: 베어링 단면도, 롤러 위치별 접촉 영역 색상맵, 변형 형상 애니메이션. OrbitControls로 자유 회전/줌.
- **Plotly Interactive Charts**: slice 하중 분포, 접촉 응력, 수명 분포 등. Hover로 slice별 상세값 표시, Gen1↔Gen3 토글 비교.
- **Contour/Heatmap**: Plotly heatmap으로 `p(ψ, x_axial)` 압력 필드, sub-surface 응력장 시각화.

#### 5.3.3 MASTA Integration Strategy

Loose coupling으로 구현. MASTA에서 베어링 내부 형상 데이터를 JSON/CSV로 export하는 C# sidecar 프로세스를 Tauri가 관리. Eyeshot 3D 렌더링은 Three.js로 대체하여 .NET 의존성 제거.

#### 5.3.4 Development Strategy: Python → Rust Porting

- **Stage A (Prototype)**: Python으로 Gen1/Gen3 solver를 먼저 구현, MESYS/MASTA 결과와 교차 검증.
- **Stage B (Production)**: 검증 완료된 알고리즘을 Rust로 포팅. nalgebra API가 NumPy와 유사하여 1:1 대응 가능.
- **테스트 전략**: Python reference 결과를 golden test로 보존하여 Rust 포팅 시 bit-level 일치 검증.

---

## 6. Project Structure (Reference)

```
trb-contact-analysis/
├── Cargo.toml                    # Rust workspace
├── src-tauri/
│   ├── src/
│   │   ├── main.rs               # Tauri entry
│   │   ├── commands.rs           # Tauri commands (solve_bearing, etc.)
│   │   ├── solver/
│   │   │   ├── mod.rs
│   │   │   ├── geometry.rs       # SliceGeometry, profile interpolation
│   │   │   ├── hertz.rs          # Hertz contact, Weber bulk deformation
│   │   │   ├── gen1.rs           # Independent slice solver
│   │   │   ├── gen3.rs           # Beam-coupled solver
│   │   │   ├── beam.rs           # Timoshenko beam FE
│   │   │   ├── rib_contact.rs    # Large-end rib point contact
│   │   │   ├── bearing.rs        # 5-DOF equilibrium
│   │   │   ├── life.rs           # ISO 16281 fatigue life
│   │   │   └── types.rs          # All struct/enum definitions
│   │   └── sidecar/
│   │       └── masta_bridge.rs   # MASTA sidecar management
│   └── Cargo.toml
├── src/                          # React frontend
│   ├── App.tsx
│   ├── components/
│   │   ├── InputPanel/           # Geometry, load, profile inputs
│   │   ├── BearingView3D/        # Three.js 3D bearing visualization
│   │   ├── ResultCharts/         # Plotly charts (load, stress, life)
│   │   ├── ContourMap/           # Pressure/stress heatmaps
│   │   ├── ComparisonView/       # Gen1 vs Gen3 dual-mode comparison
│   │   └── AlertPanel/           # Warning/alert display
│   ├── hooks/
│   │   └── useSolver.ts          # Tauri command invocation
│   └── types/
│       └── bearing.ts            # TypeScript type definitions (mirror Rust)
├── python-prototype/             # Phase 1-2 prototype
│   ├── gen1_solver.py
│   ├── gen3_solver.py
│   ├── hertz_contact.py
│   ├── beam_fe.py
│   ├── tests/
│   │   └── golden_tests/         # Reference results for Rust porting
│   └── validation/
│       └── mesys_comparison.py
└── README.md
```