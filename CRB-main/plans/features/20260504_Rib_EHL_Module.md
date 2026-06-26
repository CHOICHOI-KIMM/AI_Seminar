# Rib (Flange-Roller End) EHL/TEHL Module Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task.

---

## TL;DR

**Quick Summary**: 현재 rib(플랜지-롤러단) 접촉은 elliptical Hertz까지만 풀고 마찰계수는 상수(0.05~0.06)로 고정되어 있다. 보고서가 "TRB의 정체성과 직결되는 핵심 영역"이라고 강조하는 rib 영역에 EHL 유막두께·열보정·트랙션 계산을 추가해, raceway 접촉과 동일 수준의 윤활 평가를 가능케 한다.

**Deliverables**:
- `rib_contact.rs`에 Hamrock-Dowson elliptical EHL 유막두께 + Murch-Wilson 열보정 + Carreau/Eyring 트랙션 계산 추가
- `RibContactResult`에 EHL 출력 필드 7개 추가 (h_c, h_min, λ, regime, mu_ehl, T_flash, p_asperity)
- `compute_rib_ehl()` 함수 (rib 전용 EHL solver, 점접촉 회귀식 기반 — 별도 2D Reynolds solver 아님)
- 기존 `compute_rib_contact()`가 EHL 결과를 반환하도록 확장
- Manual/08_RibContact.md 및 14_Lubrication.md 업데이트
- TypeScript 타입 동기화 + RibContact 패널에 EHL 출력 표시

**Estimated Effort**: 2~3일 (medium)

**Critical Path**:
1. **선행**: `20260504_Carreau_NonNewtonian_Model.md` 머지 (Carreau 호출이 이 모듈 안에 들어감)
2. **본 플랜 구현**
3. **후행 옵션**: WEC risk 모듈에 rib flash temperature 통합

---

## Background

### Current State

`rib_contact.rs::compute_rib_contact()`:
- ✅ Hamrock-Brewe elliptical Hertz (a, b, p_max, δ_rib, k_rib)
- ✅ Spin moment 계산 (μ = 0.002 hard-coded inside)
- ❌ EHL 유막두께
- ❌ 열보정
- ❌ 트랙션 (sliding 마찰계수)
- ❌ λ ratio / regime classification
- ❌ asperity contact

`transient.rs:44`: `MU_RIB_DEFAULT = 0.05` — rib 마찰계수 상수 가정.

### Why this matters (보고서 근거)

- §1.1.2 "③ 플랜지-롤러단 접촉": **TRB 고유** 접촉 형태, 축 하중 직접 전달, 마찰·마모 주발생부
- §4.3: SRR > 1 (순수 슬라이딩) → 열효과로 유막두께 최대 40% 감소, Carreau 필수
- §5.1 / §5.2: 비뉴턴 + 열효과 **상승적 작용** → Newtonian + 등온 가정이 마찰계수·유막두께를 동시 과대 예측
- §4.4: 롤러 끝단 프로파일이 rib 접촉 응력 분포에 결정적

### Why NOT full 2D Reynolds

- CLAUDE.md "Simple is Best": 2D Reynolds + DC-FFT는 별도 대규모 작업 (별도 플랜 필요)
- Rib 접촉은 **타원형 점접촉** → Hamrock-Dowson 회귀식이 산업 표준이며 보고서 §4.3도 회귀식 기반 분석을 표준으로 인정
- 정밀 2D 해석은 후속 플랜으로 분리 (이 플랜 범위 외)

이 플랜은 **회귀식 기반 EHL + 열보정 + 비뉴턴 트랙션**을 결합한 산업 표준 접근을 채택한다.

---

## Mathematical Formulation

### 1. Speed parameters (rib-specific)

Rib 접촉의 entrainment / sliding 속도 (TRB 동역학 기반):

```
u_e = (u_roller_end + u_rib) / 2     [m/s]   (mean entrainment)
u_s = u_roller_end − u_rib            [m/s]   (sliding)
SRR = u_s / u_e
```

여기서 (보고서 §1.1, lubrication.rs `tapered_pure_rolling`와 정합):
- `u_rib = ω_inner × r_contact` (rib 표면, 내륜 회전)
- `u_roller_end = ω_roller × r_sph + (ω_cage × r_contact)` (롤러 끝면 회전 + 공전 효과)
- 일반적으로 SRR_rib ≫ SRR_raceway (≈ 0.01~0.05) → SRR_rib > 0.5 ~ 2

### 2. Hamrock-Dowson elliptical EHL

무차원 그룹 (이미 raceway에서 사용 중인 정의):
- U = η₀ u_e / (E' R_x)
- W = F_rib / (E' R_x²)
- G = α E'
- k = a/b (Hamrock-Brewe ellipticity, 이미 계산됨)

중심·최소 유막두께:

$$H_c = 2.69\,U^{0.67}\,G^{0.53}\,W^{-0.067}\,(1 - 0.61\,e^{-0.73k})$$

$$H_{\min} = 3.63\,U^{0.68}\,G^{0.49}\,W^{-0.073}\,(1 - e^{-0.68k})$$

$$h_c = H_c \times R_x, \quad h_{\min} = H_{\min} \times R_x$$

### 3. Thermal correction (Murch-Wilson, 기존 코드 재사용)

$$L_{\rm th} = \eta_0 \,\beta_{\rm visc}\, u_e^2 / k_{\rm fluid}$$

$$\phi_T = \frac{1}{1 + 0.1\,(1 + 14.8\,|{\rm SRR}|^{0.83})\,L_{\rm th}^{0.64}}$$

$$h_{c,\rm TEHL} = \phi_T \cdot h_c, \quad h_{\min,\rm TEHL} = \phi_T \cdot h_{\min}$$

→ 기존 `lubrication.rs::wilson_thermal_correction()` 그대로 호출.

### 4. λ ratio and regime

$$\sigma_c = \sqrt{\sigma_{\rm roller}^2 + \sigma_{\rm rib}^2}$$

$$\lambda = h_{\min,\rm TEHL} / \sigma_c$$

Regime: λ < 1 (boundary), 1 ≤ λ < 3 (mixed), λ ≥ 3 (full-film) — 기존 `classify_lambda` 재사용.

### 5. Traction

선택된 traction model로 계산 (Carreau 플랜에서 추가된 분기):

```
match operating.traction_model:
  Eyring        → eyring_traction_advanced(...)        // 기존
  CarreauYasuda → carreau_traction(...)                 // 신규 (선행 플랜)
```

p_mean = F_rib / (π·a·b), γ̇ = u_s / h_c.

### 6. Mixed lubrication (혼합윤활 영역에서만)

λ < 3일 때 Greenwood-Tripp asperity 적용 (기존 `compute_mixed_lubrication` 재사용):
- f_asp = F_{5/2}(λ) / F_{5/2}(0)
- μ_eff = (1 − f_asp)·μ_ehl + f_asp·μ_boundary
- p_asperity = f_asp · p_mean

### 7. Flash temperature (Blok 모델, wec_risk.rs 재사용)

$$\Delta T_{\rm flash} = C_{\rm Blok} \cdot \mu_{\rm eff} \cdot p_{\rm mean} \cdot u_s / \sqrt{k_s\,\rho_s\,c_s\,(u_1 + u_2)/2}$$

→ 기존 `wec_risk::flash_temperature()` 호출.

### 8. Spin moment (기존 식 유지, μ만 교체)

$$M_{\rm spin} = \frac{3}{8} \mu_{\rm eff}\, F_{\rm rib}\, a$$

(현재 0.002 상수 → 새로 계산된 μ_eff로 교체. EHL/혼합 윤활 영역에 따라 spin moment가 동적으로 변함.)

---

## File Structure

```
Modified files:
  src-tauri/src/solver/rib_contact.rs   — compute_rib_ehl() 신규,
                                           compute_rib_contact() 확장 (EHL 호출)
  src-tauri/src/solver/types.rs         — RibContactResult에 EHL 출력 7 필드 추가
                                           RibKinematics struct (옵션) 추가
  src-tauri/src/solver/bearing.rs       — rib 호출부에 운전조건/속도 전달 (~ 5곳 수정)
  src-tauri/src/solver/transient.rs     — MU_RIB_DEFAULT 제거, rib EHL μ 사용
  src-tauri/src/solver/lubrication.rs   — (필요 시) elliptical HD 헬퍼 함수
                                           (기존 lubrication.rs에 line contact만 있다면 추가)

Frontend:
  src/types/bearing.ts                  — RibContactResult 타입 sync
  src/components/RibContactView/...     — EHL 결과 표시 (h_min, λ, regime, μ, T_flash)

Docs:
  Manual/08_RibContact.md               — EHL 절 추가
  Manual/14_Lubrication.md              — rib 윤활 절 추가
```

---

## Tasks

### Phase 0: Prerequisites verification (0.5h)

- [ ] **0.1** `20260504_Carreau_NonNewtonian_Model.md`이 머지되었는지 확인 (`carreau_traction()` 함수 존재)
- [ ] **0.2** `lubrication.rs`에 elliptical Hamrock-Dowson 헬퍼가 있는지 확인 (없으면 Phase 1.1에서 추가)
- [ ] **0.3** `wec_risk::flash_temperature()` 시그니처 검토 (rib에서도 호출 가능한지)

### Phase 1: Math kernels (1일)

- [ ] **1.1** `lubrication.rs`에 `hamrock_dowson_elliptical(u, g, w, k_ellipse) -> (Hc, Hmin)` 추가 (없으면)
- [ ] **1.2** `rib_contact.rs`에 `compute_rib_kinematics(macro_geom, raceway, rotation_speed, ω_roller_est) -> RibKinematics{ u_entrain, u_slide, srr }` 추가
- [ ] **1.3** `rib_contact.rs`에 `compute_rib_ehl(f_rib, r_x, r_y, k_e, kinematics, operating, material) -> RibEHLResult` 추가:
  1. U, W, G, k 무차원 계산
  2. HD elliptical (1.1 호출)
  3. Murch-Wilson 열보정 (`lubrication::wilson_thermal_correction`)
  4. λ, regime
  5. Traction (Eyring/Carreau 분기)
  6. Mixed lubrication (λ < 3일 때)
  7. Flash temp (`wec_risk::flash_temperature`)
- [ ] **1.4** `compute_rib_contact()`에서 q_axial > 0이고 운전조건이 주어졌을 때 `compute_rib_ehl()` 호출 → 결과를 `RibContactResult`에 채움

### Phase 2: Type & API integration (0.5일)

- [ ] **2.1** `RibContactResult`에 추가 필드:
  - `h_c_um: f64` (중심 유막두께)
  - `h_min_um: f64` (최소 유막두께)
  - `lambda_ratio: f64`
  - `regime: LubricationRegime` (enum: Boundary | Mixed | FullFilm)
  - `mu_eff: f64` (실제 마찰계수, 0.002 상수 대체)
  - `flash_temp_c: f64`
  - `p_asperity_mpa: f64` (혼합 윤활 시)
- [ ] **2.2** `compute_rib_contact()` 시그니처에 `Option<&OperatingConditions>` 및 `Option<&RibKinematics>` 추가 (Backward compat: None이면 EHL 스킵, 기존 동작)
- [ ] **2.3** `bearing.rs` rib 호출부 (5곳) 운전조건/kinematics 전달

### Phase 3: Replace constants (0.5일)

- [ ] **3.1** `transient.rs::MU_RIB_DEFAULT` 사용처 확인 → rib_result.mu_eff로 교체 (rib EHL 결과가 있을 때)
- [ ] **3.2** `rib_contact.rs::compute_rib_contact()` 내부 `let mu = 0.002` → `mu_eff` 사용
- [ ] **3.3** `Spin moment = (3/8) μ_eff F_rib a` 계산이 새 μ를 쓰는지 확인

### Phase 4: Tests (1일)

- [ ] **4.1** Unit: HD elliptical 점근 거동 (k=1 원형, k→∞ 라인 극한)
- [ ] **4.2** Integration: 표준 30207 베어링 1개에서 rib EHL 결과 검증
  - h_min ~ O(0.05~0.5 μm) 범위인지
  - SRR ~ 0.5~2 범위인지
  - λ < 3 (혼합/경계) regime이 일반적인지 (보고서 §1.1.2)
- [ ] **4.3** 비교: SRR=0 (순수 rolling) 한계에서 Eyring/Carreau 둘 다 raceway와 유사한 μ 산출
- [ ] **4.4** 비교: 고속 운전에서 thermal correction φ_T < 1 적용으로 h_min 감소 확인
- [ ] **4.5** Regression: rib EHL 비활성화(None 전달) 시 기존 결과 bit-level 동일

### Phase 5: Frontend & Manual (0.5일)

- [ ] **5.1** TS 타입 sync (`RibContactResult`)
- [ ] **5.2** RibContactView에 EHL 결과 카드 추가:
  - h_c, h_min (기존 raceway 표시 패턴 따라)
  - λ + regime 배지
  - μ_eff (Eyring/Carreau 표시)
  - flash_temp
- [ ] **5.3** Manual/08_RibContact.md에 "EHL 해석" 절:
  - 무차원 그룹 정의 (rib 특화)
  - SRR 계산 (rib kinematics)
  - 출력 필드 설명
  - 한계 (회귀식 기반, 2D Reynolds 미실시)
- [ ] **5.4** Manual/14_Lubrication.md "Rib 접촉 윤활" 절 추가:
  - raceway vs rib 차이 표 (SRR, regime, traction model 권장)

---

## Test Strategy

### Numerical Verification

| Level | Test | Threshold |
|-------|------|-----------|
| A | HD elliptical k=1 vs Hamrock 1981 Table | < 1% |
| A | HD elliptical k→∞ vs Dowson-Higginson line | < 5% |
| B | rib EHL @ standard 30207 vs SKF/Schaeffler 카탈로그 마찰토크 | < 15% |
| C | Eyring vs Carreau (default) at SRR=1 | μ 차이 5~30% (정성) |
| C | Thermal off vs on at u=10 m/s | h_min 감소 10~40% |

### Regression
- Phase 2.2의 backward compat: `compute_rib_contact()` (운전조건 None) 결과가 머지 전후 bit-level 동일

---

## Out of Scope

- 2D Reynolds + DC-FFT (rib 접촉의 정밀 압력 분포) — 별도 후속 플랜 (Carreau plan + 본 plan 후 검토)
- Rib roller-end 프로파일 최적화 (Lundberg log) — 별도 플랜
- Bearing current / EDM 영향 — EV 특화 후속 플랜
- ZDDP 트리보필름 화학 모델 (현재 AdditiveType factor만 유지)

---

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|-----------|
| ω_roller 추정 불확실 (cage slip 등) | SRR 부정확 → μ 부정확 | `RibKinematics`를 사용자 override 허용 (advanced input) |
| Hamrock-Dowson은 등온·완전유막 가정 → SRR>1에서 정확도 저하 | h_min 과대예측 | Carreau + thermal correction으로 부분 보완. Manual에 한계 명시. |
| `MU_RIB_DEFAULT` 제거가 transient에 영향 | 회귀 위험 | rib EHL 결과 없을 때 기존 상수 fallback 유지 |
| Frontend 타입 미동기화 | 런타임 오류 | Phase 5.1에서 cargo build → npm run build 순차 검증 |

---

## References

- 보고서 §1.1.2 (TRB 접촉 형태)
- 보고서 §4.1 (HD 회귀식)
- 보고서 §4.3 (rib TEHL, Carreau 필수성)
- 보고서 §5.2 (열보정 Murch-Wilson)
- 보고서 §6.1.3 (rib 마찰 분해, [ref_013, ref_152])
- Liu, S. (2023). *Rib contact geometry of TRB* — 기존 rib_contact.rs 인용
- Hamrock, B. J., & Dowson, D. (1981). *Ball Bearing Lubrication*. Wiley.

---

**작성일**: 2026-05-04
**의존성**: `20260504_Carreau_NonNewtonian_Model.md` (선행 필수)
**선행 조건**: Carreau 플랜 머지 + lubrication.rs에 elliptical HD 헬퍼 존재 (없으면 Phase 1.1에서 추가)
**후속 작업**:
- WEC risk 모듈에 rib flash temperature 통합
- 2D Reynolds rib solver (별도 대규모 플랜)
