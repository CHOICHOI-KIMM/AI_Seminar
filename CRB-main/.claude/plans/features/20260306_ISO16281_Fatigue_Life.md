# ISO 16281 피로수명 계산 (life.rs) 구현 플랜

## TL;DR

| 항목 | 내용 |
|------|------|
| **Quick Summary** | Stirling(2023) 7단계 ISO 16281 피로수명 파이프라인을 `life.rs`에 구현. 라미나별 등가하중 → 기본수명 → a_ISO 보정 → 내/외륜 합성 → 베어링 전체 수명 |
| **Deliverables** | `life.rs` 전체 구현 (7개 함수 + 헬퍼), 타입 추가 (`types.rs`), Tauri 커맨드, 테스트 15+개 |
| **Estimated Effort** | Phase 5 단일 세션 (Phase 4와 동일 규모) |
| **Dependencies** | Phase 4 완료 (bearing.rs → RollerResult with q_k 배열) |

---

## 배경: Stirling 7단계 피로수명 파이프라인

| Step | 내용 | 핵심 수식 |
|------|------|-----------|
| 1 | 라미나 등가하중 Q_ek | `Q_ek = (1/Z × Σ_j Q_kj^p)^(1/p)`, p=4(고정륜), p=4.5(회전륜) |
| 2 | 라미나 동적 부하용량 Q_ck | `Q_ck = Q_c × (l_k / L_we)^(7/9)` |
| 3 | 라미나 기본수명 L_10r,k | `L_10r,k = (Q_ck / Q_ek)^(−8/9)` (주의: ISO는 지수 역수 관계) |
| 4 | 라미나 합산 → 궤도 기본수명 | `L_10r,race = (Σ_k L_10r,k^(−8/9))^(−9/8)` |
| 5 | a_ISO 보정계수 | κ(점도비) → e_C(오염계수) → `a_ISO = f(e_C × C_u/P, κ)` |
| 6 | 내/외륜 합성 | `L_nm = (L_nm,inner^(−9/8) + L_nm,outer^(−9/8))^(−8/9)` |
| 7 | (선택) 가변하중 Miner 합산 | `L_eq = 1 / Σ(t_i / L_i)` |

---

## 구현 계획

### Task 1: 타입 확장 (`types.rs`)

- [ ] `LifeInput` 구조체 추가
  ```rust
  pub struct LifeInput {
      pub c_r: f64,           // 기본 동적 부하용량 [N]
      pub c_u: f64,           // 피로 하중한계 [N]
      pub e_c: f64,           // ISO 오염계수 (0~1, 기본 0.5)
      pub z: usize,           // 롤러 수
      pub l_we: f64,          // 유효 접촉길이 [mm]
      pub n_laminae: usize,   // 라미나 수 (= 슬라이스 수)
      pub is_inner_rotating: bool, // 내륜 회전 여부
  }
  ```
- [ ] `LaminaLifeResult` 구조체 추가
  ```rust
  pub struct LaminaLifeResult {
      pub k: usize,
      pub q_ek_inner: f64,    // 내륜 라미나 등가하중
      pub q_ek_outer: f64,    // 외륜 라미나 등가하중
      pub q_ck: f64,          // 라미나 동적 부하용량
      pub l_10r_k_inner: f64, // 내륜 라미나 기본수명 [Mrev]
      pub l_10r_k_outer: f64, // 외륜 라미나 기본수명 [Mrev]
  }
  ```
- [ ] `FatigueLifeResult` 기존 필드 확인 + `lamina_results: Vec<LaminaLifeResult>` 추가
- [ ] `OperatingConditions`에 `rpm` 필드 확인 (Mrev → hours 변환용)

### Task 2: Step 1 — 라미나 등가하중 (`life.rs`)

- [ ] `compute_lamina_equivalent_load(roller_results, z, n_laminae, is_rotating) -> Vec<(f64, f64)>`
  - 각 라미나 k에 대해 모든 롤러 j의 `q_k[j]` 수집
  - 고정륜: `p = 4.0`, 회전륜: `p = 4.5` (선접촉)
  - `Q_ek = (1/Z × Σ_j |q_kj|^p)^(1/p)`
  - 내륜/외륜 각각 계산 (TRB는 내륜 회전이 일반적)

### Task 3: Step 2 — 라미나 동적 부하용량

- [ ] `compute_lamina_capacity(c_r, l_we, n_laminae) -> Vec<f64>`
  - `l_k = l_we / n_laminae`
  - `Q_ck = Q_c × (l_k / L_we)^(7/9)`
  - `Q_c = C_r / (L_we × Z × ...)` — ISO 281 기반 단일 롤러 부하용량 역산
  - 주의: C_r에서 Q_c 역산 공식은 ISO 281 Annex에 명시

### Task 4: Step 3 — 라미나 기본수명

- [ ] `compute_lamina_basic_life(q_ek, q_ck) -> f64`
  - `L_10r,k = (Q_ck / Q_ek)^(9/2)` (선접촉: 지수 = 4.5 = 9/2)
  - Q_ek = 0이면 L_10r,k = f64::INFINITY

### Task 5: Step 4 — 궤도 기본수명 합산

- [ ] `compose_race_life(lamina_lives: &[f64]) -> f64`
  - `L_10r = (Σ_k L_10r,k^(−8/9))^(−9/8)`
  - Weibull 합성 (직렬 시스템 신뢰성)
  - 무한수명 라미나는 합산에서 제외

### Task 6: Step 5 — a_ISO 보정계수

- [ ] `compute_kappa(nu_40, nu_100, t_op, rpm, d_pw) -> f64`
  - 운동점도 → 작동온도 점도 (Walther 식 또는 ASTM D341 근사)
  - 기준점도 `nu_1 = f(n, d_pw)` (ISO 281 Figure)
  - `κ = ν / ν_1`
- [ ] `compute_a_iso(kappa, e_c, c_u, p_ks) -> f64`
  - 3구간 보간:
    - κ < 0.4: 하한 (보수적)
    - 0.4 ≤ κ ≤ 4: ISO 281 Table/curve
    - κ > 4: 상한 (포화)
  - `x = e_C × C_u / P_ks`
  - ISO 281 Table 6~8 또는 Stirling Fig 5.3 curve-fit

### Task 7: Step 6 — 내/외륜 합성 + 최종 수명

- [ ] `compute_bearing_life(life_input, roller_results, operating) -> FatigueLifeResult`
  - Step 1~5 조합
  - `L_nm,race = a_1 × a_ISO × L_10r,race`
  - `a_1 = 1.0` (90% 신뢰도)
  - `L_nm = (L_nm,inner^(−9/8) + L_nm,outer^(−9/8))^(−8/9)`
  - Mrev → hours: `L_hours = L_Mrev × 10^6 / (60 × rpm)`
  - `weakest_lamina`: L_10r,k가 최소인 라미나 인덱스

### Task 8: Step 7 — 가변하중 수명 (선택, 차후 구현 가능)

- [ ] `compute_variable_load_life(load_spectrum: &[(f64, f64)]) -> f64`
  - Miner 선형 누적: `L_eq = 1 / Σ(t_i / L_i)`
  - 풍력터빈 등 하중 스펙트럼 적용
  - **Phase 5에서는 stub으로 남기고 Phase 6+에서 구현 가능**

### Task 9: Tauri 커맨드 연결

- [ ] `commands.rs`에 `compute_fatigue_life` 커맨드 추가
  - 입력: `LifeInput` + `BearingResult` (또는 `Vec<RollerResult>`) + `OperatingConditions`
  - 출력: `FatigueLifeResult`

### Task 10: 테스트

- [ ] **단위 테스트** (life.rs 내부)
  - 라미나 등가하중: 균일 하중 → Q_ek = q_uniform (p 무관)
  - 라미나 등가하중: 한 롤러만 하중 → Q_ek = (q/Z)^(1/p) × Z^(1/p)
  - 라미나 부하용량: 단일 라미나(n=1) → Q_ck = Q_c
  - 궤도 수명 합성: 모든 라미나 동일 → L_race = L_k
  - 궤도 수명 합성: 하나가 0이면 전체 0
  - a_ISO: κ=1, 청정 조건 → a_ISO ≈ 1.0
  - a_ISO: κ→0 → a_ISO → 하한
  - 내/외륜 합성: 동일하면 L_total = L_race × 2^(−8/9)... 아님, Weibull 합성
- [ ] **통합 테스트**
  - 균일 롤러 하중 + 플랫 프로파일 → 알려진 L_10 비교
  - crowned 프로파일 → 엣지 라미나 수명 < 중앙 라미나 수명
  - bearing.rs 결과 → life.rs 파이프라인 end-to-end
- [ ] **Level D 검증**
  - Bearinx/MESYS/MASTA 레퍼런스값과 <5% 이내 비교 (데이터 확보 시)

---

## 의존성 및 Critical Path

```
types.rs (Task 1)
    ↓
Step 1~4 (Task 2~5) ← 병렬 구현 가능 (순서대로 호출되지만 독립 함수)
    ↓
Step 5 (Task 6) ← OperatingConditions 점도 데이터 필요
    ↓
Step 6 (Task 7) ← 모든 Step 결합
    ↓
커맨드 연결 (Task 9) + 테스트 (Task 10)
```

## 참고 자료

- **Stirling(2023)** Ch.5.3: 7단계 파이프라인 원본
- **ISO 16281:2008**: 수정 기준수명, 라미나 방법
- **ISO 281:2007**: a_ISO 보정계수, Table 6~8
- **Master_plan.md** Phase 5 섹션

## 주의사항

1. **지수 부호**: ISO 선접촉 수명-하중 지수는 `9/2 = 4.5` (볼 베어링은 3). 라미나 합성 지수는 `−8/9` / `−9/8`.
2. **Q_c 역산**: C_r(카탈로그)에서 단일 접촉 Q_c를 역산하는 과정이 핵심. ISO 281 Annex 참조.
3. **응력 집중 계수**: Stirling은 edge stress riser f_s를 적용하지만, 초기 구현에서는 f_s=1 (미적용)로 시작하고 차후 추가.
4. **점도 모델**: Walther 식 구현이 필요. `nu_40`, `nu_100`에서 임의 온도 점도 보간.
