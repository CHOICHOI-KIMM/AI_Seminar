# 과도 슬라이딩 해석 및 WEC 위험도 평가

## TL;DR

### Quick Summary
풍력 메인베어링 TRB에서 동적 하중/속도 변화에 의한 **과도 슬라이딩(transient sliding)**을 해석하고, **WEC(White Etching Crack) 위험도**를 정량 평가하는 모듈을 구현한다. 정상상태 pure rolling 가정의 한계를 넘어, 롤러 관성 지연 + 견인력 한계에 의한 실제 슬라이딩을 시간 영역에서 추적한다.

### Deliverables
1. **시간 변수 하중 입력 인터페이스** — 하중 시계열(CSV/JSON) 로드 + 시각화
2. **롤러 과도 동역학 솔버** — 롤러별 1-DOF 회전 운동방정식 + EHL 견인력 한계
3. **과도 슬라이딩 분석** — 시간 영역 SRR/슬라이딩 속도/누적 에너지 추적
4. **WEC 위험도 평가** — NREL Guo(2021) 기준 + 누적 마찰 에너지 기준
5. **결과 시각화** — 시계열 차트 + 위험도 히트맵 + 통계 요약
6. **Manual 문서** — Manual/15_TransientSliding.md

### Estimated Effort
- Phase 1 (기반 구조): 중간
- Phase 2 (동역학 솔버): 높음
- Phase 3 (WEC 위험도): 중간
- Phase 4 (UI/시각화): 중간

---

## 배경 및 동기

### 문제 정의
정상상태에서 apex-aligned TRB는 kinematic sliding = 0 (이전 구현에서 확인). 그러나 실제 풍력 터빈에서는:
- **과도 하중 변화** (돌풍, 그리드 손실, 토크 반전) → 하중대 급변
- **롤러 관성 지연** → 하중대 진입 시 롤러가 충분한 속도에 도달하지 못함
- **EHL 견인력 한계** → 무부하 롤러의 감속 지연, 스키딩 발생

이로 인해 WEC/WSF 조기 파손 (베어링 설계 수명의 5~20% 시점에서 고장, 풍력 터빈 고장의 ~60%).

### 핵심 물리
```
롤러 운동방정식:
  I_roller × (dω_roller/dt) = τ_traction − τ_cage_drag − τ_viscous

견인력 제한:
  |τ_traction| ≤ μ_traction × Q_normal × r_roller
  (μ_traction은 EHL 견인 계수, 유막 두께 및 SRR의 함수)

슬라이딩 속도:
  V_slide,k = V_roller_surface,k − V_raceway_surface,k
  SRR_k = 2 × V_slide,k / (V_roller,k + V_raceway,k)

누적 손상:
  E_friction = ∫ μ × Q × |V_slide| dt   (누적 마찰 에너지)
```

### 참고문헌
- NREL/TP-5000-73286 (Vaes, Keller 2019): HSS 베어링 롤러 슬라이딩 실측
- Guo et al. (2021): 슬립-WEC 기준 모델
- Takabi & Khonsari (2014): 견인 계수-케이지 동역학
- 상세 목록: Wind_turbine/TRB_슬라이딩_손상_리서치.md (23편)

---

## 현재 아키텍처 분석

### 확장 포인트
| 현재 구성 | 한계 | 확장 방향 |
|-----------|------|-----------|
| `OperatingConditions` (단일 정상상태) | 시간 변수 하중 불가 | `TransientLoadCase` 시계열 타입 추가 |
| `BearingInput` (단일 operating) | 1-point only | `transient: Option<TransientInput>` 필드 |
| `BearingResult` (단일 snapshot) | 시간 이력 없음 | `TransientResult` 타입 + 누적 지표 |
| `TrbKinematics` (정상상태) | 롤러 관성 무시 | `RollerDynamicsState` 시간 적분 |
| `compute_traction` (정상상태 μ) | 과도 SRR 미반영 | 견인력 기반 slip 판정 |

### 의존성
- `compute_trb_kinematics` → 정상상태 "목표" 속도 계산 (기존 유지)
- `compute_film_thickness_distribution` → 슬라이스별 EHL 유막/견인계수 (기존 유지)
- `solve_bearing_equilibrium` → 각 시간 스텝의 하중 분포 (반복 호출)

---

## Phase 1: 시간 변수 하중 입력 기반 구조

### Task 1.1: 타입 정의 (types.rs)
```rust
/// 시간 변수 하중 시계열의 단일 포인트
pub struct LoadTimePoint {
    pub t_s: f64,           // 시간 [s]
    pub f_x: f64,           // 반경 하중 X [kN]
    pub f_y: f64,           // 반경 하중 Y [kN]
    pub f_a: f64,           // 축 하중 [kN]
    pub m_x: f64,           // 모멘트 X [kN·m]
    pub m_y: f64,           // 모멘트 Y [kN·m]
    pub n_rpm: f64,         // 회전속도 [rpm]
}

/// 과도 해석 입력
pub struct TransientInput {
    pub load_series: Vec<LoadTimePoint>,  // 시간-하중 시계열
    pub dt_max: f64,                       // 최대 시간 스텝 [s]
    pub enable_roller_dynamics: bool,      // 롤러 관성 모델 활성화
}

/// 과도 해석 결과 — 단일 시간 스텝
pub struct TransientSnapshot {
    pub t_s: f64,
    pub operating: OperatingConditions,
    pub equilibrium: BearingEquilibrium,
    pub roller_kinematics: Vec<RollerKinematicState>,  // 롤러별 실제 속도
    pub sliding_metrics: TransientSlidingMetrics,
}

/// 롤러별 실제 운동 상태 (관성 포함)
pub struct RollerKinematicState {
    pub j: usize,                   // 롤러 인덱스
    pub psi_deg: f64,               // 궤도 위치 [°]
    pub omega_roller_actual: f64,   // 실제 롤러 각속도 [rad/s]
    pub omega_roller_target: f64,   // 순수 구름 목표 각속도 [rad/s]
    pub slip_ratio: f64,            // (actual − target) / target
    pub u_slide_avg: f64,           // 평균 슬라이딩 속도 [m/s]
    pub tau_traction: f64,          // 작용 견인 토크 [N·m]
    pub tau_traction_max: f64,      // 최대 가용 견인 토크 [N·m]
    pub in_slip: bool,              // 슬립 판정
}

/// 과도 슬라이딩 시간 스텝 요약
pub struct TransientSlidingMetrics {
    pub n_rollers_in_slip: usize,
    pub max_slip_ratio: f64,
    pub max_slide_velocity: f64,    // [m/s]
    pub instantaneous_friction_power: f64,  // [W]
}

/// 과도 해석 최종 결과 (전체 시계열)
pub struct TransientResult {
    pub snapshots: Vec<TransientSnapshot>,
    pub damage_summary: TransientDamageSummary,
}

/// 누적 손상 지표
pub struct TransientDamageSummary {
    // 롤러별 누적 지표
    pub roller_damage: Vec<RollerDamageAccumulator>,
    // 베어링 레벨 통계
    pub total_slip_events: usize,
    pub total_slip_duration_s: f64,
    pub max_slip_ratio_overall: f64,
    pub wec_risk_index: f64,        // NREL Guo 기준 위험도 지수
}

/// 롤러별 손상 누적기
pub struct RollerDamageAccumulator {
    pub j: usize,
    pub cumulative_friction_energy_j: f64,  // ∫μQ|V_slide|dt [J]
    pub cumulative_slide_distance_m: f64,   // ∫|V_slide|dt [m]
    pub max_contact_load_during_slip_n: f64, // 슬립 중 최대 접촉 하중 [N]
    pub slip_event_count: usize,
    pub total_slip_duration_s: f64,
}
```
- [x] types.rs에 위 타입 추가
- [x] bearing.ts에 TS 미러링

### Task 1.2: 하중 시계열 입출력 (transient_io.rs 신규)
- [x] CSV 파싱: `t,Fx,Fy,Fa,Mx,My,rpm` 형식
- [x] JSON 파싱: `TransientInput` serde
- [x] 보간: 비균등 시간 간격 → 균등 dt로 선형 보간
- [ ] Tauri 커맨드: `load_transient_csv`, `load_transient_json`

### Task 1.3: BearingInput 확장
- [x] `BearingInput.transient: Option<TransientInput>` 필드 추가 (serde default None)
- [x] 기존 정상상태 해석은 영향 없음 (None일 때 기존 경로)

---

## Phase 2: 롤러 과도 동역학 솔버

### Task 2.1: 롤러 관성 모멈트 계산 (geometry.rs 확장)
```rust
/// 테이퍼 롤러의 관성 모멘트 (축 중심)
/// I = (π/10) × ρ × L × (R_max^5 − R_min^5) / (R_max − R_min)
///   (원추 프러스텀 공식)
fn compute_roller_inertia(geom: &MacroGeometry, material: &Material) -> f64
```
- [x] 원추 프러스텀 관성 모멘트 공식 구현
- [x] 테스트: 원통 (R_max=R_min) 케이스에서 I = (1/2)mR² 검증

### Task 2.2: EHL 견인 계수 모델 (lubrication.rs 확장)
```rust
/// SRR-의존 견인 계수 (Eyring + 열보정)
/// μ_traction(SRR) = τ_eyring/p_mean × arcsinh(η₀×u_slide / (h_c × τ_eyring))
///                   × φ_thermal
/// 포화: SRR이 커지면 μ_traction → μ_boundary
fn compute_traction_coefficient(
    srr: f64, p_mean: f64, h_c: f64, eta_0: f64, tau_eyring: f64
) -> f64
```
- [x] Eyring 비뉴턴 견인 모델 구현
- [x] SRR 의존성: SRR ↑ → μ 먼저 증가 후 포화
- [x] 열보정: Gupta 열보정 계수 적용 (간소화)
- [ ] 테스트: SRR=0에서 μ=0, SRR→∞에서 μ→μ_boundary

### Task 2.3: 과도 동역학 시간 적분 (transient.rs 신규)

핵심 알고리즘:
```
for each time step t_n → t_{n+1}:
  1. 현재 하중 → solve_bearing_equilibrium (Gen1, 빠른 모드)
     → 롤러별 Q_j(t), 접촉/비접촉 판정

  2. 각 롤러 j에 대해:
     a. 순수 구름 목표 속도 ω_target,j 계산 (cone apex kinematics)
     b. 가용 견인 토크: τ_max = μ_traction × Q_j × r_roller
        (Q_j = 0이면 τ_max ≈ τ_viscous_drag만 존재)
     c. 필요 토크: τ_needed = I × (ω_target − ω_actual) / dt
     d. 실제 토크: τ_applied = clamp(τ_needed, −τ_max, +τ_max)
     e. 속도 업데이트: ω_actual += τ_applied × dt / I
     f. 슬립 판정: |ω_actual − ω_target| / ω_target > threshold

  3. 슬라이딩 지표 계산:
     - V_slide,j = (ω_actual − ω_target) × r_roller (at mean)
     - 슬라이스별 상세: compute_slice_sliding with ω_actual
     - 누적: E_friction += μ × Q_j × |V_slide| × dt

  4. 스냅샷 저장 (선택적 간격)
```

- [x] `solve_transient()` 메인 루프 구현
- [x] Euler 시간 적분 (dt 적응형 옵션)
- [x] 케이지 드래그 모델 (간소화: C_VISCOUS_DRAG 상수)
- [x] 점성 드래그 모델: τ_viscous = C_drag × ω
- [x] 무부하 롤러 감속 모델 (점성 드래그로 통합)
- [x] `ProgressReporter` 연동 (시간 기반 %)
- [ ] Rayon 병렬화: 롤러별 독립 동역학 → 병렬 처리

### Task 2.4: 테스트
- [x] 정상상태 검증: 일정 하중에서 ω_actual → ω_target 수렴 확인
- [x] 하중 스텝 응답: Q=0 → Q>0 스텝에서 과도 슬라이딩 발생 후 수렴
- [ ] 하중대 회전: 반경 하중 방향 변화 시 하중대 진입/이탈 롤러의 슬립 패턴
- [ ] 에너지 보존: 입력 에너지 = 구름 에너지 + 슬라이딩 에너지 + 관성 에너지

---

## Phase 3: WEC 위험도 평가

### Task 3.1: NREL Guo(2021) 기준 구현
```rust
/// Guo 슬립-WEC 기준
/// 1. 각 시간 스텝에서 슬립 중인 롤러 식별
/// 2. 슬립 중 최대 접촉 하중 Q_max_slip 기록
/// 3. Q_max_slip의 분포 → 연간 발생 확률 산출
/// 4. 위험도 = Σ P(Q_max_slip > Q_threshold)
fn evaluate_wec_risk_guo(result: &TransientResult, annual_hours: f64) -> WecRiskAssessment
```
- [x] WecRiskAssessment 타입 정의
- [x] Q_max_slip 누적 분포 함수(CDF) 계산
- [x] 위험도 인덱스 산출 (0~1 스케일)

### Task 3.2: 누적 마찰 에너지 기준 (Argonne Lab)
```rust
/// 누적 마찰 에너지 기반 WEC 위험도
/// E_critical = 실험적 임계값 [J/mm²] (문헌값)
/// Risk = E_cumulative / E_critical
fn evaluate_wec_risk_energy(damage: &TransientDamageSummary) -> f64
```
- [x] 접촉면적당 에너지 밀도 계산
- [x] 임계값 대비 비율 산출

### Task 3.3: 스미어링 위험도 평가
```rust
/// ISO 15243 기반 스미어링 위험도
/// - 순간 SRR 이력에서 최대값 추출
/// - 슬라이딩 거리 누적
/// - 국부 온도 상승 추정 (flash temperature)
fn evaluate_smearing_risk(result: &TransientResult) -> SmearingRiskAssessment
```
- [x] Flash temperature 근사 (Blok 간소화 공식)
- [x] SRR 기반 스미어링 위험 등급 (Low/Medium/High/Critical)

### Task 3.4: 종합 위험도 대시보드 타입
```rust
pub struct TransientRiskAssessment {
    pub wec_guo: WecRiskAssessment,
    pub wec_energy: f64,
    pub smearing: SmearingRiskAssessment,
    pub overall_risk_level: RiskLevel,  // Low/Medium/High/Critical
    pub recommendations: Vec<String>,
}
```
- [x] 위험도 레벨 결정 로직
- [x] 자동 권장사항 생성 (예압 증가, 윤활유 교체, 표면 코팅 등)

---

## Phase 4: UI 및 시각화

### Task 4.1: 하중 시계열 입력 UI
- [ ] InputPanel에 "Transient" 토글 추가
- [ ] CSV/JSON 파일 로드 버튼 (tauri-plugin-dialog)
- [ ] 하중 시계열 미리보기 차트 (Plotly line chart)
- [ ] 시간 범위 선택 슬라이더

### Task 4.2: CanvasArea에 Transient 탭 추가
- [ ] 서브탭 1: **Time History** — Fx/Fy/Fa/rpm + SRR + slip events 시계열
- [ ] 서브탭 2: **Roller Dynamics** — 롤러별 ω_actual vs ω_target 시계열, 슬립 구간 하이라이트
- [ ] 서브탭 3: **Damage Map** — 롤러 위치별 누적 에너지 히트맵 (극좌표)
- [ ] 서브탭 4: **Risk Assessment** — WEC/스미어링 위험도 게이지 + 상세 통계

### Task 4.3: ResultsCard 확장
- [ ] Transient 모드 결과 요약 카드
- [ ] 핵심 지표: max SRR, slip events, cumulative energy, WEC risk level
- [ ] 위험도 색상 배지

---

## Phase 5: 문서 및 검증

### Task 5.1: Manual/15_TransientSliding.md 작성
- [ ] §15.1 개요 (왜 과도 해석이 필요한가)
- [ ] §15.2 롤러 동역학 모델 (운동방정식, 견인력 한계)
- [ ] §15.3 시간 적분 알고리즘
- [ ] §15.4 WEC 위험도 평가 (Guo 기준 + 에너지 기준)
- [ ] §15.5 스미어링 위험도 (flash temperature)
- [ ] §15.6 입출력 사양
- [ ] §15.7 검증 전략

### Task 5.2: 검증
- [ ] Level A: 정상상태 수렴 (과도 솔버가 정상상태와 일치)
- [ ] Level B: NREL GRC 실측 데이터 대비 cage slip 비교
- [ ] Level C: 문헌의 과도 슬립 패턴 정성적 비교

---

## 의존성 및 Critical Path

```
Phase 1 (기반 구조)
  ├── Task 1.1 (타입) ──────────────┐
  ├── Task 1.2 (I/O) ──────────────┤
  └── Task 1.3 (BearingInput 확장) ─┘
                                     │
Phase 2 (동역학 솔버)               ▼
  ├── Task 2.1 (관성 모멘트) ───────┐
  ├── Task 2.2 (견인 계수) ─────────┤
  └── Task 2.3 (시간 적분) ◀────────┘  ← CRITICAL PATH
       └── Task 2.4 (테스트)
                                     │
Phase 3 (위험도 평가)               ▼
  ├── Task 3.1 (Guo WEC 기준)
  ├── Task 3.2 (에너지 기준)
  ├── Task 3.3 (스미어링)
  └── Task 3.4 (종합 대시보드)
                                     │
Phase 4 (UI)                        ▼
  ├── Task 4.1 (입력 UI)
  ├── Task 4.2 (Transient 탭)
  └── Task 4.3 (결과 카드)
                                     │
Phase 5 (문서/검증)                 ▼
```

**Critical Path**: Task 1.1 → Task 2.1 + 2.2 → Task 2.3 → Task 3.x → Task 4.x

---

## 설계 결정 사항

### D1: 시간 적분 방법
**선택: Forward Euler (1차)**
- 이유: 롤러 동역학은 stiff하지 않음 (관성 시간상수 ~10-100ms, dt ~1ms)
- 향후: Runge-Kutta 4차 옵션 추가 가능

### D2: 하중 분포 계산 빈도
**선택: 매 시간 스텝마다 Gen1 평형**
- Gen1은 O(Z×n) → ms 단위 → 10k 스텝도 ~10초
- Gen3는 너무 느림 (O(Z×n²) × NR iterations)
- 옵션: 하중 변화 < threshold일 때 이전 결과 재사용

### D3: 케이지 모델 복잡도
**선택: 단순 1-DOF (케이지 강체 회전만)**
- 6-DOF 케이지 동역학은 Phase 2+ 범위를 넘어섬
- 케이지 속도 = 하중 롤러 속도의 가중 평균으로 근사
- 향후: 6-DOF 확장 가능

### D4: 신규 파일 vs 기존 파일 확장
**선택: transient.rs 신규 모듈**
- 기존 bearing.rs / lubrication.rs는 정상상태 유지
- transient.rs가 기존 모듈을 호출하는 orchestrator 역할
