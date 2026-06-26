# Van Zoelen Film Thickness Decay Model 통합

## TL;DR

### Quick Summary
Van Zoelen side flow 모델(2012 Venner/Lugt, 2026 Gao/Lugt)을 TRB 솔버에 통합하여,
현재의 **정적 starvation 상수(φ_s)** 방식을 **시간-의존적 유막 감쇠 예측**으로 확장한다.
기존 Basic/Advanced 모드 구조에 **"Decay" 옵션**을 추가하고,
슬라이스별 접촉 파라미터(a_k, b_k, p_h,k)를 Van Zoelen 식에 직접 입력하여
크라운 프로파일 → 윤활 내성 연계 분석을 가능하게 한다.

### Deliverables
1. **Van Zoelen decay 엔진** (Rust) — Eq. 25 기반 F(0) 계산 + Eq. 27 감쇠 곡선
2. **슬라이스별 감쇠 예측** — 크라운 프로파일 → b_k → F_k(0) → h_c,k(t)
3. **원주 위치별 감쇠** — 하중 분포에 따른 차별적 F(0) 평균화 (Eq. 28)
4. **스큐 보정 계수** — 실험 데이터 기반 감쇠율 보정 테이블
5. **Oil/Grease 자동 분기** — Oil: φ_s 유지 or decay, Grease: decay with base oil η₀
6. **UI 확장** — Decay 모드 토글, 운전 시간 입력, 감쇠 곡선 시각화
7. **Python 검증** — 논문 Fig. 3, 4, 6 재현 (완료)
8. **매뉴얼 업데이트**

### Estimated Effort
- Phase 1 (Core 엔진 + Python 검증): 완료
- Phase 2 (Rust 구현 + 슬라이스 통합): 중간
- Phase 3 (원주 평균 + 스큐 보정): 중간
- Phase 4 (UI + 시각화): 중간
- Phase 5 (크라운-윤활 연계 + 문서): 낮음

---

## 핵심 수식

### Van Zoelen Side Flow Model

**Eq. 27 — 중심 유막 감쇠:**
```
h_c(t) = (1/6 × ρ̄_c² × F(0) × t + h_{c,0}^{-2})^{-1/2}
```

**Eq. 25 — Side flow 파라미터 (해석적 근사, Eq.24 대비 3% 이내):**
```
F_k(0) = (2/l_t) × (p_h/b²) × (a/η₀) × π × ((0.5πα p_h)^{3/2} + 1)^{-2/3}
```

**Eq. 2 — Dowson-Higginson 압축성:**
```
ρ̄_c = ρ(p_h)/ρ_0 = (5.9×10⁸ + 1.34 p_h) / (5.9×10⁸ + p_h)
```

### 핵심 검증 결과 (Phase 1)
- Eq.24 (full integral) vs Eq.25 (해석적): **3% 차이** → Eq.25 사용 충분
- Eq.25 계산 속도: **0.9 μs/call** (Eq.24 대비 71배 빠름)
- 타원비 스케일링: k×10 → 감쇠 시간 ×100 정확히 재현
- η₀ 해석: Grease의 경우 **base oil 점도** 사용이 원본 그래프와 일치

---

## Phase 2: Rust 솔버 구현

### 2.1 새 함수 추가 (`lubrication.rs`)

```rust
// ─── Van Zoelen Film Decay ─────────────────────────────────

/// Side flow parameter F(0) for a single slice (Eq. 25)
fn van_zoelen_F0(
    p_h: f64,      // max Hertz pressure [Pa]
    a: f64,         // half-width rolling direction [m]
    b: f64,         // half-width transverse [m]
    eta_0: f64,     // viscosity [Pa·s] (oil: 오일점도, grease: base oil 점도)
    alpha: f64,     // pressure-viscosity coeff [1/Pa]
    l_t: f64,       // total track length [m]
) -> f64

/// Film thickness at time t (Eq. 27)
fn van_zoelen_h_c(
    t: f64,         // time [s]
    h_c0: f64,      // initial film thickness [m]
    F0: f64,        // side flow parameter [m⁻² s⁻¹]
    rho_bar_c: f64, // normalized density at p_h
) -> f64

/// Per-slice decay: 슬라이스별 F_k(0) 계산
fn compute_slice_decay_params(
    slice_geometries: &[SliceGeometry],
    angular_distribution: &[AngularLoadPoint],
    operating: &OperatingConditions,
    material: &Material,
    geom: &MacroGeometry,
) -> Vec<SliceDecayParams>
```

### 2.2 `SliceDecayParams` 구조체 (`types.rs`)

```rust
pub struct SliceDecayParams {
    pub k: usize,                   // slice index
    pub F0_inner: f64,              // F(0) inner raceway [m⁻² s⁻¹]
    pub F0_outer: f64,              // F(0) outer raceway
    pub h_cff_inner_um: f64,        // fully flooded h_c [μm]
    pub h_cff_outer_um: f64,
    pub a_inner_um: f64,            // half-width rolling dir [μm]
    pub b_inner_um: f64,            // half-width transverse [μm]
    pub a_outer_um: f64,
    pub b_outer_um: f64,
    pub p_h_inner_mpa: f64,         // max Hertz pressure [MPa]
    pub p_h_outer_mpa: f64,
}

pub struct FilmDecayResult {
    pub t_hours: f64,               // 운전 시간 [hr]
    pub h_c_inner_um: f64,          // 감쇠된 h_c inner [μm]
    pub h_c_outer_um: f64,          // 감쇠된 h_c outer [μm]
    pub starvation_ratio_inner: f64,// h_c/h_cff
    pub starvation_ratio_outer: f64,
    pub decay_rate_nm_s: f64,       // 평균 감쇠율 [nm/s]
    pub slice_decay: Vec<SliceDecayPoint>, // 슬라이스별
    pub decay_curve: Vec<(f64, f64)>,     // (t[s], h_c[nm]) 곡선 데이터
}

pub struct SliceDecayPoint {
    pub k: usize,
    pub F0: f64,
    pub h_c_um: f64,                // 시간 t에서의 h_c
    pub lambda: f64,                // h_c / σ
    pub regime: LubricationRegime,
}
```

### 2.3 l_t (총 트랙 길이) 계산

TRB 베어링의 경우:
```
l_t = 2π R_inner_raceway + 2π R_outer_raceway + Z × 2π R_roller_mean
```
여기서:
- R_inner_raceway = d_pw/2 - D_we/2 × cos(α) (내륜 접촉점 반경)
- R_outer_raceway = d_pw/2 + D_we/2 × cos(α) (외륜 접촉점 반경)
- R_roller_mean = D_we/2 (롤러 평균 반경)
- Z = 롤러 수

### 2.4 η₀ 분기 로직

```rust
let eta_0_decay = match operating.lubrication_type {
    LubricationType::Oil => {
        // Oil: 오일 자체 점도 사용
        viscosity_at_temp(operating.nu_40, operating.nu_100, operating.t_op)
            * operating.rho_oil / 1e6  // 동점도 → 동적점도
    }
    LubricationType::Grease => {
        // Grease: base oil 점도 사용 (nu_40, nu_100은 base oil 기준)
        // 현재 시스템의 nu_40/nu_100이 이미 base oil이므로 그대로 사용
        viscosity_at_temp(operating.nu_40, operating.nu_100, operating.t_op)
            * operating.rho_oil / 1e6
    }
};
```
> **참고**: 현재 시스템의 `nu_40`, `nu_100` 입력은 ISO 281 기준으로
> base oil 동점도를 의미합니다. Grease의 경우 별도 보정 없이 사용 가능합니다.

---

## Phase 3: 원주 위치 평균 + 스큐 보정

### 3.1 원주 평균 F(0) (Eq. 28)

방사 하중을 받는 베어링에서 각 롤러의 하중 Q_j는 위치에 따라 다릅니다.
F_k는 a, b, p_h에 의존하므로 원주 위치별로 달라집니다:

```rust
/// 원주 평균 F_k(0) — 하중 분포를 따라 F 평균화
fn compute_circumferential_avg_F0(
    angular_distribution: &[AngularLoadPoint],
    slice_geometries: &[SliceGeometry],
    // ...
) -> Vec<f64>  // 슬라이스별 평균 F(0)
{
    // Eq. 28: F̄_k(y) = (1/2π) ∫₀²π F_k(y, Ψ) dΨ
    // angular_distribution이 이미 1° 간격으로 있으므로 직접 수치 평균
    for slice_k in 0..n_slices {
        let sum_F0: f64 = angular_distribution.iter()
            .filter(|pt| pt.slice_q_k[k] > 0.0)  // 접촉 중인 위치만
            .map(|pt| {
                let (a, b, p_h) = hertz_from_q(pt.slice_q_k[k], ...);
                van_zoelen_F0(p_h, a, b, eta_0, alpha, l_t)
            })
            .sum();
        F0_avg[k] = sum_F0 / n_loaded_positions;
    }
}
```

### 3.2 하중 영역별 차별적 감쇠 (논문 결론 반영)

| 영역 | 하중 | p_h | F(0) | 기아 수준 | 실제 감쇠 |
|------|------|-----|------|---------|----------|
| 최대 하중 (0°) | Q_max | 높음 | 높음 (빠른 side flow) | Severe | 모델과 잘 일치 |
| 중간 하중 (45°) | Q_mid | 중간 | 중간 | Moderate | 약간 빠름 |
| 비하중 (180°) | Q≈0 | 낮음 | 낮음 | Mild | **모델보다 실제로 더 빠름** |

> **설계 결정**: Phase 3에서는 Eq. 28 원주 평균을 먼저 구현하고,
> mild starvation 보정은 향후 확장으로 남깁니다.

### 3.3 스큐 보정 계수

논문 Table 3-6의 실험 데이터를 보간 테이블로 구현합니다:

```rust
/// 스큐 보정 계수: 감쇠율에 곱하는 factor
/// skew > 0 (원심력과 같은 방향): factor < 1 (감쇠 느려짐)
/// skew < 0 (원심력과 반대):      factor > 1 (감쇠 빨라짐)
fn skew_correction_factor(
    skew_angle_deg: f64,     // 스큐 각도 [°]
    speed_param: f64,         // 속도 파라미터 (n×d_pw 등)
) -> f64
{
    // Table 3 기반 보간:
    // ±2° → ḣ_ave 비율: +2°=0.67, +1°=0.76, 0°=1.0, -1°=1.06, -2°=1.12
    // (150 mm/s 기준, 0°에 대한 비율)
}
```

> **주의**: 스큐 각도 입력은 현재 UI에 없으므로 Phase 4에서 추가합니다.
> 기본값 = 0° (보정 없음).

---

## Phase 4: UI + 시각화

### 4.1 InputPanel 변경

**Lubrication Model 섹션에 Decay 옵션 추가:**

```
SOLVER > Lubrication Model
┌─────────────────────────────────────────┐
│ Model: [Basic ▼] [Advanced ▼]           │
│                                         │
│ ☐ Film Decay (Van Zoelen)              │ ← 새로 추가
│   ├ Operating time: [___] hours         │
│   ├ Skew angle:     [___] °  (0=none)   │
│   └ ⓘ Grease: uses base oil viscosity  │
│     Oil: uses oil viscosity             │
│                                         │
│ φ_s (starvation): [0.70]  ← Decay ON시 │
│   자동 계산으로 전환 (읽기 전용)           │
└─────────────────────────────────────────┘
```

### 4.2 LubricationView 변경

**새 탭/섹션: "Film Decay"**

1. **감쇠 곡선 차트** (Plotly)
   - X: 시간 [hours], Y: h_c [μm]
   - 내륜/외륜 별도 곡선
   - 현재 운전 시간 위치에 마커

2. **슬라이스별 감쇠율 바 차트**
   - X: slice position, Y: F(0) or decay rate
   - 크라운 프로파일에 따른 변화 시각화

3. **Starvation ratio 게이지**
   - h_c(t) / h_cff 비율 표시
   - Severe / Moderate / Mild 영역 색상 구분

### 4.3 TypeScript 타입 추가 (`bearing.ts`)

```typescript
export interface FilmDecayResult {
  t_hours: number;
  h_c_inner_um: number;
  h_c_outer_um: number;
  starvation_ratio_inner: number;
  starvation_ratio_outer: number;
  decay_rate_nm_s: number;
  slice_decay: SliceDecayPoint[];
  decay_curve: [number, number][];  // [t_s, h_c_nm]
}

export interface SliceDecayPoint {
  k: number;
  F0: number;
  h_c_um: number;
  lambda: number;
  regime: LubricationRegime;
}
```

---

## Phase 5: 크라운-윤활 연계 분석

### 5.1 크라운 프로파일 → 접촉 폭 b_k → F_k(0)

현재 Gen3 솔버의 `SliceGeometry`에서 슬라이스별 프로파일 보정 Δz_k가 있습니다.
이것이 접촉 형상을 결정하고, 따라서 유막 감쇠율을 결정합니다:

```
Δz_k (프로파일) → δ_k (접근량) → q_k (선하중) → b_k (접촉 반폭)
                                                → p_h,k (최대 압력)
                                                → F_k(0) (감쇠 파라미터)
```

| 크라운 타입 | b_k 분포 | F_k(0) 분포 | 윤활 내성 |
|------------|----------|------------|----------|
| Flat | 균일 (최대) | 균일 (최소) | 최상 — 에지 응력 문제 |
| Logarithmic | 약간 감소 | 약간 증가 | 좋음 |
| Full circular | 크게 감소 | 크게 증가 | 불리 |
| Partial crown | 중앙 균일, 끝단 감소 | 끝단에서 급증 | 끝단 취약 |

### 5.2 설계 트레이드오프 시각화

UI에서 크라운 프로파일별 비교 차트:
- 상단: 접촉 응력 분포 p_h(k)
- 하단: 유막 감쇠율 F(0)(k)
- → 에지 응력 vs 윤활 내성의 트레이드오프를 직관적으로 보여줌

---

## 의존성 및 Critical Path

```
Phase 1 (완료) ──→ Phase 2 ──→ Phase 3 ──→ Phase 5
                      │              │
                      └──→ Phase 4 ──┘
```

- Phase 2는 Phase 1의 Python 검증 결과에 의존 (완료)
- Phase 3은 Phase 2의 core 엔진에 의존
- Phase 4는 Phase 2/3의 데이터 구조에 의존
- Phase 5는 Phase 3의 슬라이스별 결과에 의존

---

## 기존 코드 변경 범위

| 파일 | 변경 내용 | 영향 |
|------|----------|------|
| `types.rs` | `FilmDecayResult`, `SliceDecayParams` 추가 | 기존 구조 영향 없음 |
| `lubrication.rs` | `van_zoelen_*` 함수 추가 | 기존 함수 수정 없음 |
| `bearing.rs` | decay 결과 수집 로직 추가 | 기존 흐름에 optional 추가 |
| `bearing.ts` | TS 타입 미러 추가 | 기존 타입 영향 없음 |
| `InputPanel` | Decay 토글 + 시간 입력 추가 | 기존 UI 레이아웃 유지 |
| `LubricationView` | Decay 탭/섹션 추가 | 기존 뷰 영향 없음 |

> **원칙**: 기존 Basic/Advanced 모드는 일절 수정하지 않습니다.
> Decay는 **독립 옵션**으로 어떤 모드에서든 ON/OFF 가능합니다.

---

## 참고 논문

1. Venner, van Zoelen, Lugt (2012) "Thin layer flow and film decay modeling for grease lubricated rolling bearings" Tribology International 47:175-187
2. Gao, van Zoelen, Osara, Meeuwenoord, Pasaribu, Lugt (2026) "Film thickness decay in grease lubricated wide elliptical contacts" Tribology International
3. Van Zoelen (2009) PhD Thesis, University of Twente — 원본 이론 도출
