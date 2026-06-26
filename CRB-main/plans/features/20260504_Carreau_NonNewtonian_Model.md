# Carreau / Carreau-Yasuda 비뉴턴 모델 추가 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task.

---

## TL;DR

**Quick Summary**: 현재 트랙션/마찰 계산은 Eyring sinh⁻¹ 모델만 지원한다. 보고서가 "TRB 플랜지-롤러단 TEHL은 Carreau가 필수"라고 명시하므로 Carreau-Yasuda 모델을 병행 구현해 사용자가 두 모델을 선택할 수 있도록 한다.

**Deliverables**:
- `lubrication.rs`에 `carreau_yasuda_viscosity()`, `carreau_traction()` 함수 추가
- `OperatingConditions`에 `traction_model: TractionModel` enum (Eyring | CarreauYasuda) 및 Carreau 파라미터 4종 (η_∞, λ_relax, n, a) 추가
- 기존 Eyring 호출부를 분기 처리 (rib EHL 모듈에서도 동일 분기 재사용)
- Manual/14_Lubrication.md에 두 모델 비교/선택 가이드 추가
- 단위 테스트: 저전단/고전단 점근 거동 + 보고서 사례값 비교

**Estimated Effort**: 0.5~1일 (small; 함수 2개 + enum 1개 + 분기 3곳)

**Critical Path**: Rib EHL Plan(20260504_Rib_EHL_Module.md)이 이 모델을 호출하므로, **이 플랜이 Rib EHL보다 먼저 머지되어야 한다**.

---

## Background

### Why Carreau

보고서 §5.1 핵심 발견:
- TRB 플랜지-롤러단 접촉의 SRR > 1 (순수 슬라이딩) 조건에서 Eyring과 Carreau는 다른 마찰계수를 예측
- Eyring: 고전단에서 선형 전단희박화 (τ ∝ ln γ̇)
- Carreau: 고전단에서 유한 점도 η_∞로 수렴 → 저점도/고슬라이딩 영역에서 실험값과 더 일치
- 두 모델은 **상호 배타적이 아니라 보완적** — 사용자가 운전 조건에 따라 선택

### Carreau-Yasuda 식

$$\eta(\dot\gamma) = \eta_\infty + (\eta_0 - \eta_\infty)\,\bigl[1 + (\lambda \dot\gamma)^a\bigr]^{(n-1)/a}$$

| 파라미터 | 의미 | 일반 범위 (광유 기준) | Default |
|---------|------|--------------------|---------|
| η₀ | 영전단속도 점도 | Roelands(p,T) 출력 | 기존 흐름 사용 |
| η_∞ | 무한전단속도 점도 | 0.001 ~ 0.01 × η₀ | 0.005 × η₀ |
| λ_relax | 이완 시간 [s] | 1e−9 ~ 1e−6 | 1e−7 |
| n | 멱지수 | 0.4 ~ 0.7 | 0.5 |
| a | 전이 폭 (Yasuda) | 1.0 ~ 3.0 (a=2 → 원조 Carreau) | 2.0 |

### Why NOT a separate module

- 새 추상화 도입 금지 (CLAUDE.md "Simple is Best")
- Carreau는 Eyring과 동일한 위치(traction 계산)에 들어가는 단일 점도 식 — 함수 1~2개로 충분
- 따라서 `lubrication.rs`에 인라인 추가, 모듈 분리 안 함

---

## Mathematical Formulation

### Effective viscosity

전단 속도 γ̇ = u_s / h (slide speed / film thickness):

$$\dot\gamma = \frac{|u_s|}{h_c}$$

Carreau-Yasuda 유효 점도:

$$\eta_{\rm eff} = \eta_\infty + (\eta_0 - \eta_\infty)\,\bigl[1 + (\lambda \dot\gamma)^a\bigr]^{(n-1)/a}$$

### Traction stress

$$\tau = \eta_{\rm eff}\,\dot\gamma$$

마찰계수:

$$\mu = \frac{\tau}{p_{\rm mean}}$$

Limiting shear stress 적용: $\mu \le \Lambda_{\rm lim}$ (기존 Eyring 모델과 동일 cap).

### Comparison with Eyring (sanity check)

저전단 극한 (λ γ̇ ≪ 1):
- Carreau: η ≈ η₀ → τ ≈ η₀ γ̇ (Newtonian)
- Eyring: τ ≈ η₀ γ̇ (Newtonian, sinh⁻¹(x) ≈ x for small x) ✓ 일치

고전단 극한 (λ γ̇ ≫ 1):
- Carreau: η → η_∞ → τ ≈ η_∞ γ̇ (선형, 유한 점도)
- Eyring: τ ≈ τ₀ ln(2 η₀ γ̇ / τ₀) (대수)
- → 두 모델 발산 (이게 보고서가 말하는 차이)

---

## File Structure

```
Modified files:
  src-tauri/src/solver/types.rs           — TractionModel enum + Carreau 파라미터 4종
  src-tauri/src/solver/lubrication.rs     — carreau_yasuda_viscosity(), carreau_traction()
                                            기존 traction 함수 분기 (eyring_traction_advanced 등 3곳)
  src/types/bearing.ts                     — TypeScript 타입 동기화
  src/components/InputPanel/LubricationPanel.tsx (또는 동등 파일) — 선택 UI 추가
  Manual/14_Lubrication.md                — 두 모델 비교 섹션 추가
```

---

## Tasks

### Phase 1: Type System (0.5h)

- [ ] **1.1** `types.rs`에 `TractionModel { Eyring, CarreauYasuda }` enum 추가 (default: Eyring; 기존 동작 유지)
- [ ] **1.2** `OperatingConditions`에 다음 필드 추가:
  - `traction_model: TractionModel` (default Eyring)
  - `carreau_eta_inf_ratio: f64` (default 0.005, η_∞ = ratio × η₀)
  - `carreau_lambda_s: f64` (default 1e-7) — relaxation time [s]
  - `carreau_n: f64` (default 0.5)
  - `carreau_a: f64` (default 2.0)
- [ ] **1.3** Default 값에서 모든 기존 `OperatingConditions::default()` 호출이 그대로 동작하는지 확인 (Eyring fallback)

### Phase 2: Core Functions (1.5h)

- [ ] **2.1** `lubrication.rs`에 `carreau_yasuda_viscosity(eta_0, eta_inf, lambda, n, a, gamma_dot) -> f64` 추가 (~10 lines, pure 함수)
- [ ] **2.2** `lubrication.rs`에 `carreau_traction(eta_0, u_s, h_c, p_mean, params, lambda_lim) -> f64` 추가 (마찰계수 반환, η_∞ ratio · η₀ 자동 계산)
- [ ] **2.3** 기존 `eyring_traction_advanced()` 옆에 배치, 같은 시그니처 (호환 가능하게)

### Phase 3: Integration (1h)

- [ ] **3.1** `compute_friction()` 호출부 (lubrication.rs ~2220, ~2354, ~2460 라인 근처) 3곳에서 `match traction_model` 분기 처리
- [ ] **3.2** HMEHL `compute_friction()` (hmehl.rs ~1112) 분기 — Carreau 선택 시 점도 식만 교체, 잔차 계산 동일
- [ ] **3.3** 기존 Eyring 동작이 default로 유지되는지 회귀 테스트 1개 추가

### Phase 4: Tests (1h)

- [ ] **4.1** Newtonian 극한 테스트: λ γ̇ → 0에서 Carreau ≡ Eyring 1차항 (5% 이내)
- [ ] **4.2** 고전단 plateau 테스트: λ γ̇ ≫ 1에서 η_eff → η_∞ (3% 이내)
- [ ] **4.3** 보고서 §5.1 그림 정성 비교: 동일 SRR 곡선에서 Carreau가 Eyring보다 낮은 마찰 (TRB rib 조건)
- [ ] **4.4** 한 슬라이스에서 두 모델 모두 호출, p_max·h_min은 동일하고 μ만 차이나는지 확인

### Phase 5: Frontend & Manual (1h)

- [ ] **5.1** TS 타입 sync (`bearing.ts`): TractionModel enum + 4 fields
- [ ] **5.2** Lubrication 입력 패널: `traction_model` dropdown + 조건부 Carreau 4 파라미터 표시 (Carreau 선택 시만 펼침)
- [ ] **5.3** Manual/14_Lubrication.md에 "비뉴턴 트랙션 모델 선택" 절 추가:
  - 두 모델 식 병기
  - 선택 가이드 표 (저속/고속, SRR, rib vs raceway)
  - Default 파라미터 출처 (보고서 §5.1)

---

## Test Strategy

### Unit
- `carreau_yasuda_viscosity`: 분석해 (저/고 전단 극한)
- `carreau_traction`: η_0 → η_eff → τ → μ 체인

### Integration
- 슬라이스 1개 + 기존 운전조건에서 `model = Eyring` vs `model = CarreauYasuda(default)` 호출 시 ① h_min·p_max 동일, ② μ 차이는 5~30% 범위 (운전조건 의존)

### Regression
- 기존 default 입력 (Eyring) 결과가 머지 전후 bit-level 동일

---

## Out of Scope

- Cross 모델, Power-law 모델 추가 (보고서 우선순위 낮음)
- 압력 의존 Carreau 파라미터 (현재는 η₀만 압력 의존, 다른 4개는 상수)
- 비뉴턴 모델별 자동 파라미터 추정 (현재는 사용자 입력)

---

## References

- 보고서 §5.1 비뉴턴 모델 (수식, 비교표, TRB 적용 권고)
- 보고서 §4.3 플랜지-롤러단 TEHL (Carreau 필요성)
- Bair, S. (2007). *High-Pressure Rheology for Quantitative Elastohydrodynamics* (이완시간 λ 범위)

---

**작성일**: 2026-05-04
**의존성**: 없음 (독립)
**선행 조건**: 없음
**후속 작업**: `20260504_Rib_EHL_Module.md` (이 모델을 호출)
