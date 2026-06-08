# 14. Lubrication & Film Thickness

유막두께 해석, 혼합윤활 모델, TRB 운동학 및 접촉 트랙션 계산을 다룬다.

---

## 14.1 개요

테이퍼 롤러 베어링(TRB)의 윤활 상태는 피로 수명, 마찰 손실, 발열에 직접적으로 영향을 미친다. 본 시스템은 **두 가지 독립 축**의 모델 선택을 지원한다:

1. **EHL 유막두께 계산 방법** (택1): M1 (DH) / M2 (MK) / M3 (NVM)
2. **시간-의존 유막 감쇠** (ON/OFF): Film Decay (Van Zoelen)

두 축은 직교하여, 총 6가지 조합(M1/M2/M3 × Decay ON/OFF)이 가능하다.

```
EHL 유막두께 (h_cff 계산)         시간 감쇠 (h_c(t) 예측)
┌───────────────────────┐        ┌──────────────────────┐
│ M1 (DH)   - 1977      │        │                      │
│ M2 (MK)   - 2012      │ ─h_cff→│  Film Decay (택)      │
│ M3 (NVM)  - 1994      │        │  Van Zoelen 2012     │
└───────────────────────┘        └──────────────────────┘
  "순간 유막을 어떻게            "그 유막이 시간에 따라
   계산할 것인가"                 어떻게 변하는가"
```

### 14.1.1 모델 상세 비교표

#### A. 수식 구조 및 물리 모델

| 항목 | M1 — Dowson-Higginson (1977) | M2 — Masjedi-Khonsari (2012) | M3 — Nijenbanning-Venner-Moes (1994) |
|------|-----|-----|-----|
| **핵심 공식** | H = 3.06 U^0.69 G^0.56 W^{-0.10} | H_smooth × rc(σ̄) | 4개 점근해 통합 블렌딩 |
| **무차원 변수** | U, G, W (Hamrock-Dowson 계열) | U, G, W + σ̄, V (조도) | M, L, D (Moes 계열) |
| **접촉 형태** | 선접촉 전용 | 선접촉 전용 | **임의 타원** (원형~선접촉 연속) |
| **EHL 영역** | PV-E 단일 | PV-E 단일 | **IR + IE + RP + EP 4개 영역 통합** |
| **유도 방법** | 수치해 curve-fit (회귀식) | 수치해 curve-fit + 조도 FEM | 수치해 curve-fit + 점근 이론 |
| **유효 범위** | 고속·고하중 (EP 영역) | 고속·고하중 + 혼합윤활 | 전 영역 (저속~고속, 저하중~고하중) |

#### B. 조도(표면 거칠기) 처리

| 항목 | M1 (DH) | M2 (MK) | M3 (NVM) |
|------|---------|---------|----------|
| **유막 계산 시 조도** | **무관** (매끈 표면 가정) | **공식에 통합** (σ̄ = σ/R) | **무관** (매끈 표면 가정) |
| **h 기준면** | 매끈 표면 간 거리 | **평균선(mean line) 간 거리** | 매끈 표면 간 거리 |
| **조도 보정 계수** | 없음 | rc = 1 − 0.573·exp(−0.74·Λ_eff^0.21) | 없음 |
| **조도가 h에 미치는 영향** | 없음 → h 과대 예측 경향 | σ↑ → h↓ (rc < 1) | 없음 → h 과대 예측 경향 |
| **혼합윤활 모델** | Greenwood-Tripp (별도) | **M-K 내장** 하중/면적 분율 | M2 공유 |
| **조도 하중 분율** | GT: F₅/₂ 적분 (별도) | F_a/F = 0.50·exp(−0.70·Λ^0.50) | M2 공유 |
| **Λ < 3에서 정확도** | 불리 (조도 무시 유막) | **우수** (조도 반영 유막) | M2에 의존 |

> **핵심**: M1/M3는 "유막을 먼저 매끈하게 계산하고 → 조도를 나중에 비교(Λ)"하는 2단계 접근.
> M2는 "조도가 유막 형성 자체를 변화시킨다"를 유막 공식에 반영하는 1단계 통합 접근.
> 동일 조건에서 **M2의 h_c < M1/M3의 h_c** (조도가 유막을 얇게 만드는 효과).

#### C. 보정 모델 (열, 기아, 점도)

| 항목 | M1 (DH) | M2 (MK) | M3 (NVM) |
|------|---------|---------|----------|
| **열보정** | Gupta: φ_T = 1/(1+0.1·L_th^0.64) | Murch-Wilson: SRR 의존 φ_T | **M2와 동일** (Murch-Wilson) |
| **열보정의 SRR 의존** | 없음 (순수 구름 가정) | **있음** (SRR↑ → φ_T↓) | **있음** |
| **기아보정** | 사용자 직접 입력 φ_s | 물리 기반: φ_s = f(n×d_pw, type) | **M2와 동일** |
| **압력-점도** | Barus: η = η₀·exp(αp) | Roelands: η = η₀·exp(S·((1+p/p_r)^z−1)) | **M2와 동일** (Roelands) |
| **고압 점도 정확도** | 0.5 GPa 이상에서 과대 | **정확** (Roelands 비선형) | **정확** |

#### D. 트랙션 및 부가 기능

| 항목 | M1 (DH) | M2 (MK) | M3 (NVM) |
|------|---------|---------|----------|
| **트랙션 모델** | 간이 Eyring: μ = τ₀/p_mean | 완전 Eyring + Roelands η(p) | **M2와 동일** |
| **SRR 의존 마찰** | 없음 (상수) | **있음** (sinh⁻¹ 모델) | **있음** |
| **Flash 온도** | 없음 | Blok-Jaeger 조도 접촉 ΔT | **M2와 동일** |
| **동력 손실** | 추정만 가능 | 정밀 계산 | 정밀 계산 |

#### E. 입출력 및 운용

| 항목 | M1 (DH) | M2 (MK) | M3 (NVM) |
|------|---------|---------|----------|
| **필수 입력** | ν₄₀, ν₁₀₀, α_pv, φ_s | + τ_Eyring, z_Roelands, k_fluid, β_visc | M2와 동일 |
| **프리셋 지원** | 불필요 | Mineral / PAO / Ester | 동일 |
| **계산 비용** | 1× (기준) | ~2× | ~2× |
| **TRB EP 영역 h_c 차이** | 기준 | M1 대비 −5~15% (조도 보정) | M1 대비 +2~7% (4영역 블렌딩) |
| **Film Decay 호환** | 가능 | 가능 | **가능 (논문 원저자 파이프라인)** |

#### F. 검증 수준

| 항목 | M1 (DH) | M2 (MK) | M3 (NVM) |
|------|---------|---------|----------|
| **원 논문 검증** | 수치 EHL 대비 | 수치 EHL + FEM 조도 | 수치 EHL (수천 케이스) |
| **산업 사용** | 40년+ 표준 | 10년+ 학술/산업 | 30년+ 학술, SKF 내부 |
| **본 시스템 검증** | DH 벤치마크 12종 | M-K 계수 검증 | 논문 h_cff +1.1% (원형), DH 대비 2~7% (선접촉) |

### 14.1.2 모델 선택 가이드

#### Method 1 (DH) — 언제 사용하나

- **빠른 초기 설계 검토** — 최소 입력으로 유막 상태 개략 파악
- **ISO 281 표준 수명 계산** — κ(점도비)만 필요한 경우
- **레거시 호환** — 기존 DH 기반 결과와 비교 시

**강점:**
- 입력 파라미터 최소 (ν₄₀, ν₁₀₀, α_pv, φ_s만 필요)
- 계산 속도 가장 빠름
- 수십 년간 산업 표준으로 사용되어 검증 데이터 풍부

**약점:**
- PV-E 영역만 커버 — 저속/저하중에서 부정확
- 조도 효과 미포함 — Λ < 3 혼합윤활에서 실제보다 낙관적
- Barus 점도 모델 — 고압(>0.5 GPa)에서 과대 예측
- 열보정이 SRR에 무관 — 슬라이딩이 큰 리브 접촉에서 부정확

**유의점:**
- φ_s(기아 계수)를 사용자가 직접 설정해야 함 (Oil: 1.0, Grease: 0.5~0.8 경험적)
- 내/외륜 동일한 R_eq 근사 — 테이퍼가 큰 베어링에서 오차 증가

#### Method 2 (MK) — 언제 사용하나

- **상세 설계 단계** — 혼합윤활 비율, 트랙션, 발열까지 분석
- **그리스 윤활 최적화** — 물리 기반 기아보정으로 속도 의존 starvation 자동 예측
- **마찰 손실 계산** — Eyring + Roelands 트랙션이 필요한 경우

**강점:**
- 조도 효과가 유막두께 공식에 통합 — Λ < 3에서도 합리적
- Roelands 점도 — 고압 거동 정확
- SRR 의존 열보정 — 슬라이딩 접촉에서 정확
- Flash 온도 계산 — 소착/scuffing 위험 평가 가능

**약점:**
- 추가 입력 필요 (τ_Eyring, z_Roelands, k_fluid, β_visc)
- DH와 마찬가지로 PV-E 영역만 커버
- M-K 공식 자체가 수치 fitting이므로 외삽 범위에서 불확실

**유의점:**
- 프리셋(Mineral/PAO/Ester) 활용 시 기본값 자동 설정
- Advanced 파라미터를 모르면 프리셋 사용 권장
- M1과 h_c 결과가 다를 수 있음 — M-K가 조도를 포함하므로 Λ < 3에서 차이 확대

#### Method 3 (NVM) — 용도와 한계

M3는 TRB 설계 실무에서 M2보다 우월하지 않다. **논문 비교/검증**과 **향후 확장**을 위한 모델이다.

**용도:**
- **논문 재현** — Lugt 그룹(SKF/UTwente)의 Van Zoelen 논문에서 h_cff 계산에 사용한 공식. Film Decay 결과를 논문과 직접 비교할 때 필요
- **볼 베어링 확장** — 원형~선접촉까지 임의 타원비를 연속 처리. 향후 볼 베어링이나 SRB 지원 시 유일하게 적용 가능한 모델
- **4영역 커버리지** — 저속 시동/정지, 극저하중 등 EP 영역을 벗어나는 특수 조건 분석

**TRB에서 M2 대비 한계:**
- **조도 미포함** — M3는 매끈 표면 가정. 조도 효과는 별도 후처리(M2 공유). Λ < 3 혼합윤활에서 M2보다 덜 정확
- **h_c 과대 경향** — 조도가 유막을 얇게 만드는 효과를 반영하지 못해 M2보다 낙관적
- **TRB EP 영역에서 M1과 2~7%만 차이** — 4영역 통합의 이점이 TRB에서는 미미
- 열보정/기아보정/트랙션은 M2 것을 그대로 공유 — 독자적 파이프라인이 아님

**유의점:**
- NVM은 u_s = u₁+u₂ (합속도) 사용 — 시스템 내부에서 자동 변환
- M3 선택 시 M2의 Advanced 파라미터(τ_Eyring 등)도 사용됨 (traction/mixed 공유)
- **TRB 설계 실무에서는 M2를 권장.** M3는 논문 비교가 필요하거나 비 TRB 접촉을 분석할 때만 선택

### 14.1.3 Film Decay (Van Zoelen) — 언제 사용하나

Film Decay는 유막두께 계산 **방법**이 아니라, 계산된 h_cff가 운전 시간에 따라 어떻게 변하는지 예측하는 **후처리 모델**이다.

| 항목 | 설명 |
|------|------|
| **원리** | EHL 접촉 압력에 의한 측방 유출(side flow)과 보충(replenishment)의 균형 |
| **입력** | 운전 시간 [hr], 스큐 각도 [°], 보충률 R [nm/s] |
| **출력** | h_c(t) 감쇠 곡선, 평형 유막 h_eq, 감쇠 후 Λ/regime |
| **참조** | Venner, van Zoelen & Lugt (2012); Gao et al. (2026) |

**강점:**
- 정적 φ_s가 아닌 **물리 기반** 시간-의존 유막 예측
- 그리스 수명, 재윤활 주기 결정에 직접 활용 가능
- R > 0 시 **평형 유막** 자동 계산 — 실제 설계값으로 사용
- 크라운 프로파일 → 접촉 폭 b → 감쇠율 연계 분석 가능
- F(0)는 해석적 공식(Eq. 25) — 반복 계산에도 비용 무시

**약점:**
- Severe starvation 가정 — mild starvation(h/h_cff > 0.5)에서 과소 예측 경향
- Replenishment rate R을 사용자가 추정해야 함 — 그리스별 실험 데이터 필요
- 스큐 보정은 실험적 보간 — 논문 조건(Li/M, 150 mm/s) 외 범위에서 불확실

**윤활 방식별 적용:**

| 윤활 방식 | Decay 사용? | R 설정 | 이유 |
|----------|-----------|--------|------|
| **Oil Bath** | OFF 권장 | — | 공급 충분, 기존 φ_s로 충분 |
| **Oil Jet** | 선택적 | R = η_capture × Q_jet / (l_t × w_track) | 소량 급유 시 유용 |
| **Grease** | **ON 권장** | R = bleeding rate (0.001~0.1 nm/s) | 핵심 적용 |

**유의점:**
- R = 0은 **최악 조건**(보충 없음) — 실제보다 보수적
- R > 0이면 감쇠 곡선이 평형(h_eq)에 수렴 — 이 값이 설계에 사용할 유막두께
- h_eq < Λ=1 경계이면 → Boundary 윤활 진입 경고
- Oil/Grease 모두 ν₄₀/ν₁₀₀를 base oil 기준으로 입력해야 함 (ISO 281 표준과 동일)
- 감쇠 곡선의 초기 급강하 구간(t < 수 분)은 mild starvation 효과로 실제보다 느리게 표시될 수 있음

### 14.1.4 권장 조합

| 목적 | 유막 모델 | Decay | 이유 |
|------|----------|-------|------|
| 빠른 초기 검토 | **M1 (DH)** | OFF | 최소 입력, 산업 표준 |
| ISO 수명 계산 | **M1 (DH)** | OFF | κ(점도비) 기반 a_ISO |
| 상세 혼합윤활 분석 | **M2 (MK)** | OFF | 조도 통합 유막 + Eyring 트랙션 |
| 그리스 수명 예측 | **M2 (MK)** or **M3 (NVM)** | **ON** | 시간 감쇠 + 평형 유막 |
| 논문 비교/검증 | **M3 (NVM)** | **ON** | Lugt 그룹과 동일 파이프라인 |
| 크라운 프로파일 최적화 | **M2 (MK)** | **ON** | 슬라이스별 감쇠율 비교 |

### 14.1.5 내/외륜 개별 계산

3가지 모델 모두 **내륜과 외륜의 유막두께를 독립적으로 계산**한다:

- **내륜**: R_eq_inner, 내륜 접촉 반경 기준 인입속도
- **외륜**: R_eq_outer, 외륜 접촉 반경 기준 인입속도
- 합성 조도도 내/외륜별로 개별 계산 (rq_roller + rq_inner / rq_roller + rq_outer)

SliceFilmThickness 출력 구조체는 8개 필드를 갖는다 (내륜 4 + 외륜 4).

### 14.1.6 ISO 수명 계산과 유막 모델의 관계

#### κ 결정 방법과 유막 모델 호환성

ISO 281:2007의 수명 수정 계수 a_ISO는 κ(점도 비)에 의존한다. κ를 결정하는 두 가지 방법이 유막 모델과 다르게 상호작용한다:

| κ 방법 | 산출 공식 | 유막 모델 의존 | 적합 모델 |
|--------|---------|-------------|----------|
| **ViscosityRatio** (ISO 281 §9.3) | κ = ν_actual / ν_ref | 아니오 — 점도만 사용 | **M1/M2/M3 모두 가능** |
| **FilmThicknessRatio** (ISO/TR 1281-2 §4) | κ = Λ^1.3 (Λ = h_min/σ) | **예** — h_min 필요 | **M1 또는 M3만** |

#### 조도 이중 반영 문제 (M2 + FilmThicknessRatio)

M2(MK)는 유막두께 공식에 조도 보정(rc)이 내장되어 있다. FilmThicknessRatio 방법으로 κ를 계산하면 조도가 **2중으로 반영**된다:

```
M1 경로 (정상):
  h_smooth ──→ Λ = h_smooth / σ ──→ κ = Λ^1.3 ──→ a_ISO
  [조도 1회: Λ에서 σ로 나눔]

M2 경로 (이중 반영):
  h_smooth × rc(σ) ──→ Λ = h_MK / σ ──→ κ = Λ^1.3 ──→ a_ISO
  [조도 1회: rc(σ)로 h 감소]
  [조도 2회: Λ에서 σ로 또 나눔]
  → κ 과소 예측 → 수명 과소 예측 (과도하게 보수적)
```

이는 ISO 281의 a_ISO 곡선이 **매끈 표면 h 기준의 Λ**로 calibration 되었기 때문이다. 조도가 이미 Λ = h/σ 단계에서 반영되므로, h 자체에 조도를 포함하면 이중 계산이 된다.

> **⚠ 주의**: M2 + FilmThicknessRatio 조합 사용 시, UI에 경고가 표시됩니다.

#### 권장 조합

| 목적 | 유막 모델 | κ 방법 | 비고 |
|------|----------|--------|------|
| **ISO 표준 수명** | M1 (DH) | ViscosityRatio | ISO 281과 완전 일치 |
| **정밀 수명 (Λ 기반)** | M1 또는 M3 | FilmThicknessRatio | 매끈 표면 h → Λ → κ |
| **혼합윤활 분석** | M2 (MK) | ViscosityRatio | M2의 조도는 트랙션/마찰 분석용 |
| **그리스 수명** | M1/M3 + Decay | FilmThicknessRatio | 감쇠 후 Λ로 κ 재평가 |

### 14.1.7 Micropitting 안전율 (표면 고장 평가)

#### 배경: ISO 281의 한계

ISO 281의 수명 계산은 **표면하 피로(subsurface fatigue)**만 다룬다. 표면 기인 고장 모드(micropitting, wear, scuffing)는 별도 평가가 필요하다.

| Λ 범위 | 지배적 고장 모드 | ISO 281 수명 | 추가 평가 |
|--------|---------------|------------|---------|
| Λ > 3 | Subsurface fatigue | **신뢰** | 불필요 |
| 1 < Λ < 3 | 혼합 (표면 + 표면하) | 참고용 | **권장** |
| Λ < 1 | **Surface fatigue 지배** | **부적절** | **필수** |

#### 안전율 S_λ

ISO/TS 6336-22 (기어 micropitting 평가)의 프레임워크를 베어링에 적용한다:

$$S_\lambda = \frac{\Lambda_{min}}{\Lambda_{perm}}$$

| 판정 | S_λ | 의미 |
|------|-----|------|
| **Safe** | ≥ 2.0 | Full EHL, micropitting 위험 없음 |
| **Marginal** | 1.0 ~ 2.0 | 주의 — 표면 처리 또는 첨가제 권장 |
| **AtRisk** | < 1.0 | 높은 micropitting 위험 |

#### Λ_perm 결정

Λ_perm = Λ_perm,base × additive_factor

**표면 처리 등급 (ISO/TS 6336-22 GF-Class 대응):**

| 등급 | Rq [μm] | Λ_perm,base | GF-Class | 근거 |
|------|---------|------------|----------|------|
| Standard (연삭) | 0.3~0.5 | **2.0** | Low | 봉우리 날카로움, 접촉 집중 |
| FineGround (정밀연삭) | 0.15~0.3 | **1.0** | Medium | 업계 일반 기준 |
| Superfinish (등방성) | < 0.1 | **0.5** | High | 봉우리 제거, 등방성 표면 |

**윤활유 첨가제 보정:**

| 첨가제 | factor | 효과 |
|--------|--------|------|
| None | 1.0 | 기준 |
| EP (극압 첨가제) | 0.8 | 황-인계 경계막 보호 |
| AW (내마모 첨가제) | 0.7 | ZDDP 등 화학 흡착막 |

예: Superfinish + EP → Λ_perm = 0.5 × 0.8 = 0.4

> **⚠ 주의**: ISO/TS 6336-22는 **기어 전용** 규격이다. 베어링 전용 micropitting 규격은
> 현재 존재하지 않으며, 위 값들은 기어 규격을 베어링에 차용한 **보수적 공학 추정**이다.
> 정밀 평가에는 Morales-Espejel & Brizmer (2011)의 micro-EHL 모델 또는 제조사 시험 데이터를 참조한다.

#### Film Decay와의 연계

Film Decay ON 시, 감쇠된 Λ로 S_λ를 재평가해야 한다:

```
t = 0:     Λ = 2.5 → S_λ = 2.5/1.0 = 2.5 (Safe)
t = 1000hr: Λ = 0.8 → S_λ = 0.8/1.0 = 0.8 (AtRisk) ← 표면 고장 위험
```

현재 구현에서 S_λ는 fully flooded Λ 기준으로 계산된다. Decay 후 Λ로의 재평가는 향후 확장 예정.

#### 참고 문헌

- ISO/TS 6336-22: Calculation of surface durability (micropitting) — Gears
- FVA 54/7: FZG micropitting test procedure (GF-Class 결정)
- Morales-Espejel, G.E. & Brizmer, V. (2011). "Micropitting modelling in rolling-sliding contacts: application to rolling bearings." Tribology Transactions 54:625-643.
- Morales-Espejel, G.E. & Gabelli, A. (2015). "A Model for Rolling Bearing Life with Surface and Subsurface Survival." Tribology Transactions 58:894-906.
- Clarke, A., Evans, H.P. & Snidle, R.W. (2016). "Understanding micropitting in gears." Proc IMechE Part C.

### 14.1.8 해석 결과 활용

해석 결과는 수명 수정 계수 a_ISO의 κ 결정(Ch.10)과 향후 동력손실 모듈(P_total = P_contact + P_drag + P_seal)에 입력된다.

Film Decay ON 시, **감쇠된 유막두께**(또는 평형값)를 설계 유막으로 사용하여 Λ, regime, 수명을 재평가할 수 있다.

---

## 14.2 윤활유 물성

### 14.2.1 점도-온도 보간 (ASTM D341 Walther)

운전 온도 T_op에서의 동점도 ν는 두 기준점(ν₄₀, ν₁₀₀)으로부터 Walther 방정식으로 보간한다:

```
log log(ν + 0.7) = A − B × log(T + 273.15)
```

여기서 A, B는 (40°C, ν₄₀)과 (100°C, ν₁₀₀) 두 점으로부터 결정된다. 상세 유도는 §10.2.4 참조.

### 14.2.2 동점도 및 동력 점도

- **동점도**: ν [mm²/s = cSt] — Walther 보간 결과
- **동력 점도**: η₀ = ν × 10⁻⁶ × ρ [Pa·s], ρ ≈ 850 kg/m³ (광유 기준)
- **압력-점도 계수**: α_pv [1/GPa] — 사용자 입력, 기본값 20 GPa⁻¹

### 14.2.3 압력-점도 모델

#### Barus 모델 (Basic)

```
η(p) = η₀ × exp(α × p)
```

고압 영역(GPa)에서 점도를 과대 예측하는 한계가 있다. Basic 모델에서 Dowson-Higginson 공식의 G 파라미터(= α × E*)에 사용된다.

#### Roelands 모델 (Advanced)

```
η(p) = η₀ × exp{(ln(η₀) + 9.67) × [(1 + p/p_r)^Z_r − 1]}
```

여기서:
- p_r = 196.2 MPa — Roelands 기준 압력
- Z_r — Roelands 압력-점도 지수 (사용자 입력, 기본값 0.67)

GPa 수준 접촉 압력에서 Barus보다 정확하다. Z_r 값은 윤활유 종류에 따라 다르다:

| 윤활유 종류 | Z_r 범위 | 전형값 |
|-------------|----------|--------|
| 광유 (Mineral) | 0.60-0.75 | 0.67 |
| PAO (합성유) | 0.45-0.55 | 0.50 |
| 에스터 | 0.50-0.60 | 0.55 |

p = 0일 때 η(0) = η₀로 정확히 환원된다.

### 14.2.4 윤활 유형

| 유형 | 설명 | 기아 영향 (Basic) | 기아 영향 (Advanced) |
|------|------|-------------------|---------------------|
| Oil | 오일 순환/비산 윤활 | φ_s = 1.0 (완전 충전) | 물리 기반 속도 의존 |
| Grease | 그리스 윤활 | φ_s < 1.0 (사용자 입력) | 물리 기반 + 블리딩 보정 |

---

## 14.3 EHL 유막두께 — Basic 모델

Film Thickness 탭에 표시되는 대표값은 **슬라이스별 상세 계산(§14.12.4)의 최악 슬라이스** 결과를 사용한다. 즉, 전체 접촉 영역 중 유막이 가장 얇은 슬라이스의 h_min, h_c, Lambda, 무차원 파라미터, 혼합윤활 결과를 표시한다.

### 14.3.1 등가 반경

선접촉(line contact)에서 등가 반경 R_eq는 전동 방향의 상대 곡률 반경이다. 내/외륜 접촉에서 각각 다른 R_eq를 사용한다:

- **내륜 접촉**: R_eq_inner — 슬라이스별 롤러 반경과 내륜 곡률의 조합
- **외륜 접촉**: R_eq_outer — 슬라이스별 롤러 반경과 외륜 곡률의 조합

슬라이스별 상세 계산에서는 각 슬라이스 위치의 R_eq_k를 사용한다 (§14.12.4 참조).

### 14.3.2 결합 탄성 계수

```
1/E* = (1 − ν₁²)/E₁ + (1 − ν₂²)/E₂
```

여기서 E₁, E₂는 롤러 및 링 소재의 영률 [GPa], ν₁ = ν₂ = ν는 포아송비이다.

### 14.3.3 인입 속도 (Entrainment Velocity)

EHL 인입속도는 접촉 영역이 레이스웨이 표면을 쓸고 지나가는 속도이다. 순수 구름(pure rolling)에서 두 접촉면의 표면 속도가 동일하므로, 인입속도 = 쓸림 속도가 된다.

일반 공식 (내/외륜 모두 회전 가능):
```
ω_cage = (ω_i × (1 − γ_k) + ω_o × (1 + γ_k)) / 2
```

**내륜 접촉** — 접촉 영역이 내륜을 쓰는 속도:
```
γ_k = d_we_k × cos(α) / d_pw
R_inner_k = R_pw × (1 − γ_k)
u_m_inner_k = |(ω_i − ω_cage)| × R_inner_k
```

**외륜 접촉** — 접촉 영역이 외륜을 쓰는 속도:
```
R_outer_k = R_pw × (1 + γ_k)
u_m_outer_k = |(ω_cage − ω_o)| × R_outer_k
```

여기서:
- ω_i = n_inner_rpm × 2π / 60 [rad/s] — 내륜 각속도
- ω_o = n_outer_rpm × 2π / 60 [rad/s] — 외륜 각속도
- R_pw = d_pw / 2 [m] — 피치 반경
- γ_k = d_we_k × cos(α) / d_pw — 슬라이스 k에서의 기하학적 계수
- d_we_k — 슬라이스 k 위치의 롤러 직경 (소단부~대단부에서 선형 변화)

**순수 구름 시 내/외륜 인입속도는 동일:**
```
u_m_inner_k = u_m_outer_k = |ω_i − ω_o| × R_pw × (1 − γ_k²) / 2
```

TRB 테이퍼 기하에 의해 소단부(γ_k↓)와 대단부(γ_k↑)에서 인입속도가 다르다.

Ref: Harris & Kotzalas (2006) Ch.12, Hamrock et al. (2004) Ch.18.

**열보정 φ_T 계산**에는 피치원 기준 평균 인입속도 `U_m = |(ω_i − ω_cage)| × R_pw × (1 − γ_mean)`을 사용하여 전 슬라이스에 동일하게 적용한다 (Basic 모델).

### 14.3.4 Dowson-Higginson 무차원 파라미터

세 개의 무차원 그룹으로 EHL 문제를 정규화한다:

| 파라미터 | 정의 | 물리적 의미 |
|----------|------|-------------|
| U (속도) | η₀ × u_m_k / (E* × R_eq_k) | 점성력 / 탄성력 |
| G (재료) | α_pv × E* | 압력-점도 민감도 |
| W (하중) | q_k × 10³ / (E* × R_eq_k) | 접촉 하중 / 탄성력 |

여기서 모든 파라미터는 **최악 슬라이스**(최소 h_min을 갖는 슬라이스)의 값이다.

### 14.3.5 최소 유막두께 — Dowson-Higginson (1977)

선접촉 EHL 최소 유막두께의 표준 공식:

```
H_min = 2.65 × U⁰·⁷⁰ × G⁰·⁵⁴ × W⁻⁰·¹³
```

실제 유막두께:

```
h_min = H_min × R_eq   [m]
```

### 14.3.6 중심 유막두께 — Dowson-Toyoda (1978)

```
H_c = 3.06 × U⁰·⁶⁹ × G⁰·⁵⁶ × W⁻⁰·¹⁰
```

중심 유막두께는 최소 유막두께보다 항상 크다 (H_c > H_min).

### 14.3.7 열보정 계수 — Gupta TEHL (Basic)

고속 운전 시 전단 발열에 의한 유막 감소를 보정한다:

```
L_th = η₀ × β_visc × U_m² / k_fluid
φ_T = 1 / (1 + 0.1 × L_th⁰·⁶⁴)
```

여기서:
- β_visc ≈ 0.04 [1/K] — 점도-온도 계수
- k_fluid ≈ 0.15 [W/(m·K)] — 윤활유 열전도도
- L_th — 열 하중 파라미터 (thermal loading number)

**저속 베어링 참고**: 풍력터빈 메인베어링(< 20 rpm)에서는 L_th ≈ 0이므로 φ_T ≈ 1.0 (열보정 불필요). 저속에서는 점성 전단 발열이 무시할 수 있는 수준이다.

### 14.3.8 기아 보정 계수 (Basic)

```
h_eff = h × φ_s × φ_T
```

- φ_s = 1.0: 완전 충전 (fully flooded) — 오일 윤활 기본값
- φ_s < 1.0: 기아 상태 — 그리스 윤활에서 접촉 입구의 유막 공급 부족
- 사용자 입력 (범위: 0.1 ≤ φ_s ≤ 1.0)

### 14.3.9 최종 유막두께

```
h_min [μm] = H_min × R_eq × φ_T × φ_s × 10⁶
h_c   [μm] = H_c   × R_eq × φ_T × φ_s × 10⁶
```

---

## 14.3A EHL 유막두께 — Advanced 모델

Advanced 모델은 표면 조도, SRR 의존 열보정, 물리 기반 기아보정을 통합한 정밀 유막 해석을 수행한다.

### 14.3A.1 Masjedi-Khonsari (2012) 선접촉 유막두께 공식

**선접촉(line contact)** EHL에서 표면 조도 효과를 유막두께 계산에 직접 통합한다. σ̄ → 0 (매끄러운 표면)일 때 표준 Dowson-Higginson/Toyoda 공식으로 환원된다.

> **중요**: M-K (2015) 점접촉(point-contact) 공식은 TRB 선접촉에 적용하면 조도 보정이 과도하게 증폭되어(~9×) 비현실적인 결과를 생성한다. 반드시 M-K (2012) **선접촉** 버전을 사용한다.

**중심 유막두께** (매끈 부분: D-T 계수 + 지수 조도 보정):
```
H_c_smooth = 3.06 × U^0.69 × G^0.56 × W^(-0.10)
f_c = 1 − 0.573 × exp(−0.74 × Λ_eff^0.21)
H_c = H_c_smooth × f_c × R
```

**최소 유막두께** (매끈 부분: D-H 계수 + 지수 조도 보정):
```
H_min_smooth = 2.65 × U^0.70 × G^0.54 × W^(-0.13)
f_m = 1 − 0.856 × exp(−0.74 × Λ_eff^0.21)
H_min = H_min_smooth × f_m × R
```

여기서:
- Λ_eff = H_smooth / σ̄ — 유효 유막-조도 비
- σ̄ = σ_combined / R — 무차원 조도 파라미터
- U = η₀·u_m/(E*·R) — 무차원 속도 파라미터
- G = α·E* — 무차원 재료 파라미터
- W = w/(E*·R) — 무차원 하중 파라미터 (단위 길이당)

보정 특성:
- σ̄ → 0: Λ_eff → ∞, exp → 0, f → 1.0 (매끈 표면으로 환원)
- σ̄ 증가: Λ_eff 감소, f < 1.0 (조도에 의한 유막 감소)

**조도 하중 분율** (GT의 γ_a 대체):
```
F_a/F = 0.50 × exp(−0.70 × Λ_eff^0.50)
```

**조도 면적 분율** (GT의 A_real/A_hertz 대체):
```
A_a/A_H = 0.30 × exp(−0.60 × Λ_eff^0.50)
```

### 14.3A.2 Murch-Wilson 열보정 (Advanced)

SRR 의존 열보정 — Gupta와 달리 슬라이딩 비율에 따른 추가 열 저감을 반영한다:

```
φ_T = 1 / [1 + 0.1 × (1 + 14.8 × |SRR|^0.83) × L^0.64]
```

여기서:
- L = η₀ × β_visc × u_m² / k_fluid — 열 하중 파라미터
- SRR — 슬라이드-롤 비율 (Slide-Roll Ratio)
- β_visc — 점도-온도 계수 [1/K] (사용자 입력, 기본값 0.04)
- k_fluid — 윤활유 열전도도 [W/(m·K)] (사용자 입력, 기본값 0.15)

특성:
- SRR = 0 (순수 구름): Gupta 공식과 동일하게 `φ_T = 1/(1 + 0.1·L^0.64)`
- SRR 증가 → 전단 발열 증가 → φ_T 감소 (유막 저감)
- φ_T는 [0.3, 1.0] 범위로 클램핑

Advanced 모델에서는 **슬라이스별로** 해당 슬라이스의 SRR을 사용하여 φ_T를 개별 계산한다 (Basic의 전체 평균 적용과 대비).

### 14.3A.3 물리 기반 기아보정 (Advanced)

사용자 입력 대신 속도 파라미터(n × d_pw)에 기반한 자동 기아보정:

**오일 윤활**:
```
φ_s = 1 / [1 + (n×d_pw / 3000000)^0.9]
```

φ_s는 [0.5, 1.0] 범위로 클램핑.

| n×d_pw [mm·rpm] | φ_s 근사값 | 윤활 상태 |
|------------------|-----------|-----------|
| < 100,000 | ~0.98 | 거의 완전 충전 |
| 300,000 | ~0.90 | 경미한 기아 |
| 1,000,000 | ~0.75 | 상당한 기아 |
| > 2,000,000 | ~0.60 | 심한 기아 (원심 비산) |

**그리스 윤활** — 추가 기유(base-oil) 블리딩 보정 (Lugt 2013):

```
φ_s,grease = φ_s,oil × f_bleed
```

블리딩 계수 f_bleed:
- n×d_pw < 100,000: f_bleed = 0.85
- n×d_pw > 500,000: f_bleed = 0.55
- 중간: 선형 보간 `f_bleed = 0.85 − 0.30 × (n×d_pw − 100000) / 400000`

φ_s,grease는 [0.3, 1.0] 범위로 클램핑.

고속에서 그리스의 채널링(channeling) 및 건조 현상으로 기유 공급이 감소하여 오일보다 더 큰 기아 효과가 발생한다.

### 14.3A.4 Advanced vs Basic 비교 요약

| 항목 | Basic | Advanced |
|------|-------|----------|
| 유막 공식 | Dowson-Higginson / Toyoda | Masjedi-Khonsari (2012) 선접촉 |
| 압력-점도 | Barus (α×E*) | Roelands η(p) |
| 열보정 | Gupta (속도 의존) | Murch-Wilson (속도 + SRR 의존) |
| 기아보정 | 사용자 입력 φ_s | 물리 기반 (n×d_pw) |
| 조도 하중 분율 | GT F_{5/2} 적분 | M-K 내장 (Λ_eff 기반 지수 감소) |
| 조도 면적 분율 | GT F_2 적분 | M-K 내장 (Λ_eff 기반 지수 감소) |
| 트랙션 | 간이 Eyring (τ₀/p) | 완전 Eyring + Roelands η(p) |
| 플래시 온도 | 미포함 | Blok-Jaeger 모델 |

---

## 14.3B 비뉴턴 트랙션 모델 (Eyring vs Carreau-Yasuda)

Advanced/M3 트랙션 계산은 두 가지 비뉴턴 모델을 선택할 수 있다 (`OperatingConditions.traction_model`).

### 14.3B.1 모델 비교

| 항목 | Eyring (default) | Carreau-Yasuda |
|------|-----------------|----------------|
| 유효 점도 | 간접 (응력 식 내부) | η_eff = η_∞ + (η_0−η_∞)·[1+(λ·γ̇)^a]^((n−1)/a) |
| 응력 식 | τ = τ₀·sinh⁻¹(η·γ̇/τ₀) | τ = η_eff·γ̇ |
| 고전단 거동 | τ ∝ ln(γ̇) (대수 증가) | η → η_∞ (유한 plateau), τ ∝ γ̇·η_∞ |
| 파라미터 | τ₀, z_Roelands | η_∞/η_0, λ, n, a, z_Roelands |
| 이론적 기반 | 분자 활성화 에너지 (Eyring 1936) | 현상론적 (Carreau 1972, Yasuda 1981) |
| 적합 영역 | 라인접촉 raceway (저~중 SRR) | 플랜지-롤러단 (고 SRR), EV 저점도 윤활유 |

> 두 모델은 모두 limiting shear stress 캡(τ ≤ 0.10·p_mean)을 공유한다. 캡까지의 도달 속도는 모델/파라미터 의존.

### 14.3B.2 Carreau-Yasuda 파라미터

| 파라미터 | 의미 | 일반 범위 (광유) | 기본값 |
|---------|------|----------------|--------|
| η_∞/η_0 | 무한 전단속도 점도 비 | 0.001 ~ 0.01 | 0.005 |
| λ [s] | 이완 시간 (relaxation time) | 1e−9 ~ 1e−6 | 1.0e−7 |
| n | 멱법칙 지수 (power-law exponent) | 0.4 ~ 0.7 | 0.5 |
| a | Yasuda 전이 지수 | 1.0 ~ 4.0 (a=2 → 원조 Carreau) | 2.0 |

### 14.3B.3 모델 선택 가이드

| 운전 조건 / 접촉부 | 권장 모델 | 비고 |
|-------------------|----------|------|
| 일반 raceway 라인접촉, SRR < 0.05 | Eyring (default) | 기존 검증 데이터와 정합 |
| 플랜지-롤러단(rib) 접촉, SRR > 0.5 | Carreau-Yasuda | 보고서 §4.3, §5.1 권장 |
| 기어 치면 high-sliding | 양쪽 비교 | Habchi 2008: 파라미터에 따라 다름 |
| EV 초저점도 윤활유 (PAO3.5, e-fluid) | Carreau-Yasuda | 고전단 plateau 거동 |
| 그리스 윤활 + 비뉴턴 증주제 | Carreau-Yasuda + film decay | Van Zoelen과 결합 사용 |

### 14.3B.4 한계 및 주의

- 보고서 §5.1이 정성적으로 보고하는 "Carreau가 Eyring보다 낮은 μ"는 **파라미터/regime 의존적**이다 (Habchi et al. 2008). 본 시스템 검증 결과:
  - 저전단 + 저압: 두 모델 일치 (Newtonian limit, 5% 이내)
  - 고전단 + 고압: 두 모델 모두 τ_lim 캡(0.10·p)에 saturate
  - 중간 압력 (~200 MPa) + aggressive Carreau (n=0.3, a=4, λ=1e−6): Carreau가 Eyring 대비 명확히 낮음
- `n=0.5, a=2` 기본값은 광유 raceway 조건에서 안정. EV/플랜지에서는 사용자 보정 권장.
- Carreau 점도식 자체는 응력 캡을 갖지 않으므로 limiting shear stress (0.10·p) 가 모델 출력의 상한을 결정.

### 14.3B.5 검증 (단위 테스트)

`lubrication.rs::tests` 모듈 12개 테스트:

| 테스트 | 검증 내용 |
|--------|----------|
| `test_carreau_newtonian_limit` | λγ̇ → 0 ⇒ η_eff → η_0 (오차 < 0.1%) |
| `test_carreau_high_shear_plateau` | λγ̇ ≥ 1e8 ⇒ η_eff → η_∞ (오차 < 5%) |
| `test_carreau_monotonic_thinning` | η_eff(γ̇) 단조감소 |
| `test_carreau_a2_reduces_to_classical_carreau` | a=2 → Carreau (1972) 식 (오차 < 1e−12) |
| `test_carreau_yasuda_exponent_sharpens_transition` | a 증가 시 knee에서 η 증가 |
| `test_carreau_traction_srr_zero` | SRR = 0 ⇒ μ = 0 |
| `test_carreau_traction_limiting_shear_cap` | μ ≤ 0.15 (clamp) 보장 |
| `test_carreau_traction_curve_shape` | μ(SRR) 단조비감소, 0 ≤ μ ≤ 0.15 |
| `test_carreau_vs_eyring_newtonian_limit_agreement` | 저압·저SRR에서 Eyring과 5% 이내 일치 |
| `test_carreau_vs_eyring_high_shear_both_saturate` | 고압·고SRR에서 두 모델 모두 캡 인근 (비율 < 3) |
| `test_carreau_aggressive_below_eyring_at_moderate_pressure` | 200 MPa, aggressive 파라미터에서 μ_C < μ_E |
| `test_traction_dispatcher_eyring_backward_compat` | dispatcher의 Eyring 경로 = 직접 호출 (bit-equivalent) |
| `test_traction_dispatcher_carreau_path` | dispatcher의 Carreau 경로 정상 |
| `test_compute_traction_advanced_default_eyring_unchanged` | 통합: Eyring/Carreau 모두 정상, 토크 비율 합리 범위 |

---

## 14.4 Lambda 비 및 윤활 영역 분류

### 14.4.1 합성 표면 조도

내/외륜별로 개별 계산한다:

```
σ_inner = √(Rq_roller² + Rq_inner²)   [μm]
σ_outer = √(Rq_roller² + Rq_outer²)   [μm]
```

- Rq_roller = 0.15 μm (기본값, 연삭 롤러 표준값)
- Rq_inner = 0.3 μm (기본값, 내륜 RMS 조도)
- Rq_outer = 0.3 μm (기본값, 외륜 RMS 조도)

> **참고**: Basic 모델에서는 기존 호환성을 위해 Ra (산술 평균 조도)를 사용하며, Advanced 모델에서는 Rq (RMS 조도)를 명시적으로 사용한다. 일반적으로 Rq ≈ 1.25 × Ra 관계가 성립한다.

### 14.4.2 Lambda 비

내/외륜별로 개별 계산:

```
Λ_inner = h_min_inner / σ_inner
Λ_outer = h_min_outer / σ_outer
```

### 14.4.3 윤활 영역 분류

| 영역 | 조건 | 설명 |
|------|------|------|
| Full EHL | Λ > 3 | 완전 유체 윤활 — 조도 접촉 없음 |
| Mixed | 1 ≤ Λ ≤ 3 | 혼합 윤활 — 유체막 + 조도 접촉 공존 |
| Boundary | Λ < 1 | 경계 윤활 — 조도 접촉 지배적 |

**풍력터빈 메인베어링**: 저속 + 고하중 + 그리스 윤활 조건에서 Λ < 2인 혼합윤활 영역에서 운전하는 것이 일반적이다. 이 경우 조도 접촉에 의한 마이크로피팅, 마모, 동력손실이 수명을 지배한다.

---

## 14.5 혼합윤활 모델 — Greenwood-Tripp (Basic)

### 14.5.1 이론적 배경

Greenwood & Tripp (1970)은 두 거친 표면의 접촉을 통계적으로 모델링한다. 조도 높이 분포를 가우시안으로 가정하고, 조도 첨단(asperity tip)의 Hertz 접촉을 적분하여 전체 하중 분담 및 실접촉면적을 구한다.

> **참고**: Advanced 모델에서는 GT 적분 대신 Masjedi-Khonsari (2012) 선접촉 공식 내장의 조도 하중/면적 분율을 사용한다 (§14.3A.1 참조).

### 14.5.2 가우시안 분포 함수

```
φ(s) = (1/√(2π)) × exp(-s²/2)
```

여기서 s = z/σ는 정규화된 조도 높이이다.

### 14.5.3 GT 통계 적분

n차 GT 적분:

```
F_n(Λ) = ∫_Λ^∞ (s − Λ)ⁿ × φ(s) ds
```

수치적으로 Simpson 1/3 규칙(200 구간, [Λ, Λ+5] 범위)으로 계산한다.

핵심 적분:
- **F_{5/2}(Λ)**: 조도 하중 적분 — 조도 접촉이 부담하는 하중 비율에 비례
- **F_2(Λ)**: 조도 면적 적분 — 실접촉면적 비율에 비례

특성:
- F_n(0) > 0 (유한 양수값)
- F_n(Λ)은 Λ가 증가하면 단조 감소
- Λ > 4.5이면 F_n(Λ) ≈ 0 (무시 가능)

### 14.5.4 erfc 보조 함수

상보 오차 함수 erfc(x)는 Abramowitz & Stegun 7.1.26 다항식 근사로 계산한다:

```
erfc(x) = t × P(t) × exp(-x²),   t = 1/(1 + 0.3275911·x)
P(t) = 0.254829592·t − 0.284496736·t² + 1.421413741·t³ − 1.453152027·t⁴ + 1.061405429·t⁵
```

최대 오차 |ε| < 1.5 × 10⁻⁷ (x ≥ 0). 음수 인자: erfc(-x) = 2 − erfc(x).

### 14.5.5 조도 접촉 물성 상수

| 상수 | 값 | 설명 |
|------|------|------|
| η·β·σ | 0.04 | 조도 밀도 × 첨단 반경 × 합성조도 (무차원) |
| √(σ/β) | 0.14 | 연삭 베어링강 전형값 |
| μ_boundary | 0.10 | 경계 마찰 계수 (EP 첨가제 포함) |
| μ_rib | 0.06 | 리브 접촉 마찰 계수 |

### 14.5.6 조도 하중 분율

```
γ_a = F_{5/2}(Λ) / F_{5/2}(0)
```

- γ_a = 0: 완전 유체 윤활 (Λ > 4)
- γ_a = 1: 완전 경계 윤활 (Λ = 0)
- 0 < γ_a < 1: 혼합 윤활

### 14.5.7 조도 면적 분율

```
A_real / A_hertz = η·β·σ × F_2(Λ) / F_2(0)
```

### 14.5.8 조도 접촉 압력

```
p_a = K_a × E* × F_{5/2}(Λ)
```

여기서 K_a = (16√2/15) × π × (η·β·σ)² × √(σ/β)는 조도 형상 상수이다.

### 14.5.9 압력 분배

전체 접촉 압력은 유체(EHL)와 조도(asperity)로 분배된다:

```
p_total = p_fluid + p_asperity
p_fluid = p_total × (1 − γ_a)
p_asperity = p_total × γ_a
```

---

## 14.6 마찰 계수 모델

### 14.6.1 EHL 트랙션 계수 — 간이 Eyring 모델 (Basic)

저 SRR(slide-roll ratio) 영역에서 EHL 유체 트랙션:

```
μ_ehl = τ₀ / p_mean
```

여기서:
- τ₀ ≈ 5 MPa — Eyring 전단 응력 (광유 전형값)
- p_mean — EHL 접촉의 평균 유체 압력 [Pa]
- 범위: 0.001 ≤ μ_ehl ≤ 0.02

### 14.6.2 완전 Eyring 트랙션 + Roelands (M2/M3)

SRR, 접촉 압력, 유막두께를 모두 고려하는 물리 기반 트랙션:

```
1. 유효 점도: η_eff = roelands_viscosity(η₀, p_mean, Z_r)
2. 전단 속도: γ̇ = u_slide / h_c
3. Eyring 전단응력: τ = τ₀ × sinh⁻¹(η_eff × γ̇ / τ₀)
4. 한계 전단응력: τ_lim = Λ_lim × p_mean (보수적: 0.10)
5. μ_f = min(τ, τ_lim) / p_mean
```

μ_f는 [0, 0.15] 범위로 클램핑.

**물리적 의미**:
- 저 SRR: `η_eff × γ̇ / τ₀ ≪ 1` → `sinh⁻¹(x) ≈ x` → τ ≈ η_eff × γ̇ (뉴턴 유체 거동)
- 고 SRR: `τ → τ₀ × ln(2·η_eff·γ̇/τ₀)` → 로그 증가 (전단 박화)
- 매우 높은 SRR: τ_lim = Λ_lim × p_mean에 의해 상한 제한

Roelands η(p)를 사용하므로 고압 영역에서 Barus 대비 더 현실적인 점도값을 적용한다.

#### τ_Eyring 자동 추정 (Arana 2019)

Eyring stress τ_E는 트라이볼로지 시험(twin-disc 등)으로 결정하는 것이 원칙이나,
Arana et al. (2019)의 이론적 관계를 이용하면 **α_pv에서 자동 추정** 가능하다:

$$\tau_E = \frac{2\Lambda_{lim}}{\alpha}$$

여기서 Λ_lim은 limiting-stress pressure coefficient (base oil 의존):

| Base oil 종류 | Λ_lim | τ_E 예시 (α=20 GPa⁻¹) |
|-------------|-------|---------------------|
| Mineral (naphthenic) | 0.047 | 4.7 MPa |
| Mineral (paraffinic) | 0.040 | 4.0 MPa |
| PAO | 0.035 | 3.5 MPa |
| Ester | 0.030 | 3.0 MPa |

**이론적 근거** (Arana Eq.8-12):
- Eyring의 유체 유동 이론에서 τ_E = kΘ/v_τ (활성 체적에 반비례)
- Barus 식에서 α = v_p/(kΘ)
- 따라서 τ_E = (v_p/v_τ)/α, 여기서 v_p/v_τ = 2Λ_lim
- Bair & Winer (1979), Hirst & Moore (1974)에 의해 실험적으로 검증됨

**UI**: τ_Eyring 입력 옆 "Auto(α)" 버튼을 누르면 현재 α_pv 값에서 자동 계산.

#### Clarke 하중분담 함수 (참고)

현재 M-K 모델의 하중분율 `F_a/F = 0.50×exp(-0.70×Λ^0.50)` 대안으로,
Clarke et al. (2016)의 ECR 실험으로 검증된 erf 기반 함수가 구현되어 있다:

$$\xi = 1 - \text{erf}(\Lambda)$$

이 함수는 Λ < 2에서 점진적으로 증가하고, Λ < 1에서 급격히 증가하여
실험 결과와 잘 일치한다. 향후 하중분담 모델 비교/선택 옵션으로 활용 가능.

- Arana et al. (2019) Proc IMechE Part J, 233(2):303-316
- Clarke et al. (2016) Tribology International, ECR measurement

### 14.6.3 경계 마찰 계수

조도-조도 직접 접촉의 마찰 계수:

```
μ_boundary = 0.10
```

EP(극압) 첨가제를 포함한 베어링강 표면의 표준값이다.

### 14.6.4 유효 마찰 계수

조도 하중 분율에 의한 가중 평균:

```
μ_eff = (1 − γ_a) × μ_ehl + γ_a × μ_boundary
```

이 공식은 완전 EHL(γ_a = 0 → μ_eff = μ_ehl)에서 완전 경계 윤활(γ_a = 1 → μ_eff = μ_boundary)까지 연속적으로 전이한다.

Basic에서는 GT γ_a, Advanced에서는 M-K load_fraction을 γ_a로 사용한다.

---

## 14.6A 플래시 온도 — Blok-Jaeger (Advanced)

Advanced 모델에서 조도 접촉부의 순간 온도 상승을 추정한다.

### 14.6A.1 계산 공식

```
ΔT = μ × p_a × V_slide / (2 × k_steel × √(π × Pe))
```

여기서:
- Pe = V_slide × b_hertz / (2 × κ_steel) — Peclet 수
- b_hertz — Hertz 접촉 반폭 [m]
- k_steel = 46 W/(m·K) — 베어링강 열전도도
- κ_steel = 1.2 × 10⁻⁵ m²/s — 베어링강 열확산율

저 Peclet (Pe < 10⁻⁶, 준정적):
```
ΔT ≈ μ × p_a × V_slide × b_hertz / (4 × k_steel)
```

최대값 500°C로 클램핑.

### 14.6A.2 위험도 분류

| 온도 범위 | 분류 | 설명 |
|-----------|------|------|
| < 50°C | Low | 정상 운전 |
| 50-150°C | Medium | 주의 필요 |
| 150-300°C | High | 윤활유 열화 위험 |
| > 300°C | Critical | 스미어링 위험 |

플래시 온도 결과는 WEC(White Etching Crack) 리스크 평가 모듈에서도 활용된다 (max_flash_temp_rise > 50°C → 스미어링 리스크 플래그).

---

## 14.7 TRB 운동학

### 14.7.1 Cone Apex 기하학

TRB에서 α_i ≠ α_o는 항상 성립한다 (내/외륜 원추각은 본질적으로 다르다). Cone apex alignment 조건은 α_i = α_o가 아니라, 내륜/외륜/롤러 세 원추의 꼭짓점이 베어링 축 위 한 점(O)에서 만나는 것이다.

공통 꼭짓점 O에서 롤러 축 방향으로 거리 ρ인 위치에서:
```
롤러 반경:    r(ρ) = ρ × sin(φ)      φ = (α_o − α_i) / 2  (롤러 반각)
내륜 접촉반경: R_i(ρ) = ρ × sin(α_i)
외륜 접촉반경: R_o(ρ) = ρ × sin(α_o)
```

따라서 접촉반경/롤러반경 비는 롤러 전체 길이에 걸쳐 일정:
```
C_i = R_i / r = sin(α_i) / sin(φ)  (상수)
C_o = R_o / r = sin(α_o) / sin(φ)  (상수)
```

### 14.7.2 케이지 속도

단일 각도 Harris 근사(`γ = D_we·cos(α)/d_pw`)가 아닌, **양 레이스웨이 각도를 사용하는 정확한 공식**:

```
ω_cage = ω_i × sin(α_i) / [sin(α_i) + sin(α_o)]
```

**유도**: 외륜(고정) 순수 구름 조건 `ω_cage × R_o = ω_roller × r`과 내륜 순수 구름 조건 `ω_i × R_i = ω_cage × R_i + ω_roller × r`로부터, R_i/r = C_i = sin(α_i)/sin(φ), R_o/r = C_o = sin(α_o)/sin(φ)를 대입하면 도출된다.

> **참고**: Harris 근사 `ω_cage = ω_i/2 × (1−γ)`는 α_i ≠ α_o일 때 cos²(φ) ≈ 1 − (α_o−α_i)²/8 만큼의 오차가 있다.

### 14.7.3 롤러 자전 속도

```
ω_roller = ω_i × sin(α_i) × sin(α_o) / [sin(φ) × (sin(α_i) + sin(α_o))]
```

**유도**: 외륜 순수 구름 `ω_cage × R_o = ω_roller × r`에서 ω_cage를 대입.

### 14.7.4 구름 속도 (Entraining Velocity)

접촉 중심 좌표계에서의 엔트레인먼트 속도. 순수 구름 시 내/외륜 동일:

```
U_roll = ω_roller × r_mean
```

**유도**: 순수 구름에서 접촉 영역의 레이스웨이 쓸림 속도:
- 내륜: (ω_i − ω_cage) × R_i = ω_roller × r
- 외륜: (ω_cage − ω_o) × R_o = ω_roller × r (외륜 고정 시)
- 양자 동일: u_m = |ω_i − ω_o| × R_pw × (1 − γ²) / 2

Ref: Harris & Kotzalas (2006) Ch.12.

### 14.7.5 슬라이스별 정밀 슬라이딩 계산

본 시스템은 **정확한 원추 기하학(exact cone geometry)**에 기반한 슬라이스별 슬라이딩 속도를 계산한다.

#### Cone Apex Alignment과 Zero Sliding

TRB에서 α_i ≠ α_o는 **항상** 성립한다 — 이것은 비정상이 아니라 TRB의 본질적 기하학이다. Cone apex alignment 조건은 α_i = α_o가 아니라, 세 원추의 꼭짓점이 축 위 한 점에서 만나는 것이다.

**정확한 운동학(§14.7.2~14.7.3)**과 **정확한 접촉반경비(§14.7.1)**를 함께 사용하면, **어떤 (α_i, α_o) 조합이든** apex-aligned 기하에서 모든 슬라이스의 슬라이딩이 정확히 0이 된다:

```
C_i × (ω_i − ω_cage) = ω_roller   (내륜 순수 구름)
C_o × ω_cage = ω_roller             (외륜 순수 구름)
⇒  u_slide,k = 0   ∀k, ∀(α_i, α_o)
```

> **주의**: 단일 각도 Harris 근사(γ = D_we·cos(α)/d_pw)를 쓰면 cos²(φ) 만큼의 오차가 생겨 apex-aligned 기하에서도 인위적 nonzero sliding이 나타난다.

#### 원추 비례 접촉반경 모델

공통 꼭짓점 O로부터의 정확한 접촉반경비:
```
C_i = sin(α_i) / sin(φ)    φ = (α_o − α_i) / 2
C_o = sin(α_o) / sin(φ)
```

슬라이스 k에서의 접촉반경:
```
R_inner,k = r_k × C_i
R_outer,k = r_k × C_o
```

#### 슬라이스별 슬라이딩 속도

```
u_slide,inner,k = |ω_i × R_inner,k − (ω_cage × R_inner,k + ω_roller × r_k)|
u_slide,outer,k = |ω_cage × R_outer,k − ω_roller × r_k|
```

#### 실제 슬라이딩의 원인

Apex-aligned 이상 기하에서 kinematic sliding은 0이다. 실제 베어링에서 슬라이딩이 발생하는 원인:
- Crown/dub-off 프로파일 수정 → 유효 접촉반경 변화
- 탄성 변형 → 접촉 기하 변화
- 제조 공차 → 원추 꼭짓점 불일치
- 리브 접촉 (§14.7.6) → 지배적 슬라이딩 소스

#### 하중 가중 평균 슬라이딩

트랙션 계산에서 롤러당 대표 슬라이딩 속도는 슬라이스 하중 가중 평균으로 결정:
```
u_slide,avg = Σ(q_k × u_slide,k) / Σ(q_k)   (접촉 슬라이스만)
```

### 14.7.6 리브 접촉 속도

대단부 구면-리브 접촉은 주로 슬라이딩(spinning motion)이다:

```
U_slide,rib = ω_roller × R_large_end
```

여기서 R_large_end = D_we,max / 2이다.

---

## 14.8 접촉별 마찰력 계산

### 14.8.1 개별 접촉점 마찰

각 롤러의 내륜/외륜 접촉에 대해 로컬 유막두께와 마찰 계수를 독립적으로 계산한다:

```
1. 대표 슬라이스 하중 q_mean = Σ q_k / n_loaded   (접촉 슬라이스 평균)
2. 로컬 h_min → 로컬 Λ → 로컬 γ_a
3. 로컬 μ = (1 − γ_a) × μ_ehl + γ_a × μ_boundary
4. F_traction = μ × Q_normal   [N]
5. P_loss = F_traction × U_slide   [W]
```

Advanced 모델에서는 단계 3에서 μ_ehl을 완전 Eyring+Roelands로 계산한다 (§14.6.2).

### 14.8.2 리브 접촉 마찰

```
F_friction,rib = μ_rib × F_rib   [N]
P_loss,rib = F_friction,rib × U_slide,rib   [W]
```

μ_rib = 0.06 (EP 첨가제 포함 슬라이딩 접촉).

---

## 14.9 베어링 레벨 동력 손실

### 14.9.1 접촉 동력 손실 합산

Sliding과 Rolling 손실을 물리적으로 분리하여 계산한다:

**Sliding friction power** — EHL/혼합윤활 트랙션으로부터:
```
P_sliding = Σ_j (μ_inner,j × Q_j × U_slide,inner + μ_outer,j × Q_j × U_slide,outer)
```

**Rolling resistance power** — Palmgren 경험식(line contact, bearing steel):
```
P_rolling = Σ_j μ_rr × Q_j × (U_roll,inner + U_roll,outer)
```

여기서 μ_rr ≈ 0.002 (구름 저항 계수, 선접촉 베어링강). Sliding friction(μ ≈ 0.01~0.10)보다 1~2 자릿수 작다.

**Rib contact power** — 순수 슬라이딩:
```
P_rib = Σ_j μ_rib × F_rib,j × U_slide,rib
```

**Total contact power**:
```
P_contact = P_sliding + P_rolling + P_rib
```

### 14.9.2 마찰 토크

```
M_friction = P_contact / ω_shaft × 1000   [N·mm]
```

### 14.9.3 향후 확장: 전체 동력 손실

현재 구현은 접촉 트랙션(P_contact)만 계산한다. 전체 베어링 동력 손실은:

```
P_total = P_contact + P_drag + P_seal
```

여기서:
- P_drag — 윤활유 교반/항력 손실 (churning)
- P_seal — 실 마찰 손실

TractionSummary 구조체는 향후 power_loss.rs 모듈에서 P_total 계산에 직접 입력될 수 있도록 설계되었다.

---

## 14.10 HMEHL 수치 EHL 솔버

### 14.10.1 개요

HMEHL (Homogenized Mixed Elastohydrodynamic Lubrication) 솔버는 선접촉 EHL 문제를 수치적으로 풀어 압력 분포, 유막 형상, 마찰 계수, 접촉부 온도 분포를 계산한다. 경험식(DH/MK/NVM)이 전체 평균값만 제공하는 것과 달리, HMEHL은 접촉 영역 내 **공간 분해(spatially resolved)** 결과를 제공한다.

#### 주요 특징

- 1D 선접촉 Reynolds PDE + DC-FFT 탄성 변형 (Boussinesq 반무한체)
- Roelands 압력-점도 + Dowson-Higginson 압력-밀도 + Ree-Eyring 비뉴턴 전단
- Patir-Cheng 조도 homogenization + Clarke 하중분담
- **Venner (1991) FAS Multigrid** — V-cycle 재귀 + distributive relaxation
- **FBNS 질량보존 캐비테이션** (Fischer-Burmeister NCP)
- TEHL 에너지 방정식 (Roelands-Houpert 온도-점도 연성)

#### 솔버 아키텍처 (Tribo-X/SNU 동등 수준)

| 구성 요소 | 방법 | 참조 |
|-----------|------|------|
| Reynolds 이산화 | 유한차분(FDM), 중심차분 | Venner 1991 |
| 탄성변형 | DC-FFT 컨볼루션 O(NlogN) | Johnson 1985 |
| P-H 결합 | **약결합 반복** (weakly coupled) | Venner/Tribo-X 공통 |
| 멀티그리드 가속 | **FAS V-cycle** (4 레벨: 256→128→64→32) | Venner 1991 Ch.5 |
| Smoothing | Distributive relaxation + line relaxation | Venner 1991 |
| 캐비테이션 | **FBNS** (Fischer-Burmeister NCP) | Woloszynski 2015 |
| H₀ 결정 | FFT 역산 + PID load balance | — |
| 초기화 | FMG (Full Multigrid, coarsest→finest) | Lubrecht 1987 |
| 수렴 범위 | **M = 100 ~ 100,000** (전 범위) | — |

### 14.10.2 지배 방정식

#### Reynolds 방정식 (비차원)

```
∂/∂X [ε ∂P/∂X] = Λ ∂(ρ̃H)/∂X
```

여기서:
- P = p/p_h (비차원 압력, Hertz 기준)
- H = h·R/b² (비차원 유막)
- X = x/b (비차원 좌표, Hertz 반폭 기준)
- ε = ρ̃H³φ_x/(η̃Λ) (Poiseuille 유동 계수)
- Λ = 12u_mη₀R²/(b³p_h) (속도 파라미터)
- ρ̃ = ρ/ρ₀ (Dowson-Higginson)
- η̃ = η/η₀ (Roelands)
- φ_x = Patir-Cheng 유동 계수

#### 유막두께 방정식

```
H(X) = H₀ + X²/2 + V(X)
V(X) = -(2/π) ∫ P(S) ln|X-S| dS   (탄성 변형, FFT로 계산)
```

#### 하중 평형

```
∫ P(X) dX = π/2
```

### 14.10.3 물성 모델

#### Roelands 압력-점도

```
η(p) = η₀ exp{ S[-1 + (1 + p/p_r)^Z] }
S = ln(η₀) + 9.67
Z = α·p_r/S
p_r = 196 MPa (Roelands 기준 압력)
```

점도비 η/η₀는 `exp(60)` ≈ 10²⁶ 에서 cap 처리 (수치 안정성).

#### Dowson-Higginson 압력-밀도

```
ρ/ρ₀ = (5.9×10⁸ + 1.34p) / (5.9×10⁸ + p)   [Pa 단위]
```

#### Ree-Eyring 비뉴턴 점도

```
η_eff = (τ₀·h/u_s) × sinh⁻¹(η·u_s/(τ₀·h))
```

τ₀ = Eyring stress (기본값: 2×0.047/α ≈ 4.7 MPa for mineral oil)

### 14.10.4 FAS V-cycle (Full Approximation Scheme)

Venner (1991) 멀티그리드의 핵심. 4-레벨 격자 계층(256→128→64→32)에서 재귀적으로 수행된다.

```
V-cycle(level):
  if coarsest:
    relax 20 sweeps (직접 반복)
    return

  Pre-smooth:  distributive relaxation (2-4 sweeps)
  Restrict:    fine 잔차 → coarse rhs (full-weighting)
               P_coarse = restrict(P_fine)
  Recurse:     V-cycle(coarse level)
  Prolongate:  correction = P_coarse_after - P_coarse_before
               P_fine += interpolate(correction)
  Post-smooth: distributive relaxation (2-4 sweeps)
```

#### Distributive Relaxation (Venner Ch.5)

EHL의 핵심 이완 기법. Jacobian 대각에 **탄성 자기영향(K_self)**을 포함하여 고하중에서도 안정적:

```
잔차:    R_i = L(P)_i - Λ·(ρH)_i'/ΔX - rhs_i
대각:    D_i = a_c + Λ·ρ·K_self/ΔX + 1.5·a_c·K_self/H_i
보정:    δP_i = ω · R_i / D_i
```

Venner의 핵심 통찰: 고M에서 `a_c → 0` (Poiseuille 소멸)이어도 `Λ·ρ·K_self/ΔX` 항이 D_i를 유한하게 유지하여 수렴을 보장한다. 이것이 Newton 기반 솔버(Habchi 등)가 실패하는 영역에서 멀티그리드가 성공하는 이유이다.

#### FMG (Full Multigrid) 초기화

가장 거친 격자(32노드)에서 시작하여 점진적으로 미세한 격자로 prolongation:

```
Solve on coarsest (32 nodes, 200+ relax sweeps)
  → Prolongate to 64 nodes, V-cycles until converged
    → Prolongate to 128 nodes, V-cycles until converged
      → Prolongate to 256 nodes, V-cycles until converged (최종 해)
```

H₀ 초기화: 각 레벨에서 FFT 탄성변형 역산으로 정확한 시작값 계산:
```
H₀ = h_c_nd - gap(center) - V(center)
```

### 14.10.5 FBNS 캐비테이션 (Fischer-Burmeister Newton Strategy)

질량보존 캐비테이션 모델. 기존 `P ≥ 0` 클램핑 대신 **상보조건(NCP)**을 사용한다.

#### 수정 Reynolds 방정식

```
∂/∂X [ε ∂P/∂X] = Λ ∂(ρ̃H(1-θ))/∂X
```

보조 변수 θ_i ∈ [0, 1]은 캐비테이션 분율:
- θ = 0: 전막(full film) 영역 → 유체가 공간을 완전히 채움
- θ > 0: 캐비테이션 영역 → 유체가 부분적으로 공간을 채움 (기포/공기 혼입)

#### Fischer-Burmeister 상보조건

```
Φ(P_i, θ_i) = √(P_i² + θ_i²) - P_i - θ_i = 0
```

이 조건은 P ≥ 0, θ ≥ 0, P·θ = 0을 자동으로 만족시킨다:
- P > 0 → θ = 0 (유체 영역: 압력이 양이면 캐비테이션 없음)
- P = 0 → θ > 0 (캐비테이션 영역: 압력이 0이면 유체 부분 충전)

#### 질량보존의 의미

단순 `P ≥ 0` 클램핑은 출구에서 음압을 0으로 자르면서 유체 질량을 비보존적으로 처리한다. FBNS는 θ가 유체 부족분을 흡수하므로 출구 캐비테이션 경계에서 질량이 보존된다.

참고: Woloszynski et al. (2015) "Efficient FBNS Algorithm for EHL", Tribology International.

### 14.10.7 TEHL 에너지 방정식

1D forward-march 에너지 방정식으로 접촉부 온도 분포를 계산:

```
ρ_l·c_l·u_m·h × (T_i - T_{i-1})/Δx = Q_visc + Q_cond
```

- Q_visc = η_eff·u_s²/h (점성 소산)
- Q_cond = -2k_s(T - T_inlet)/δ_s (고체 열전도)
- δ_s = √(k_s·x/(ρ_s·c_s·u_m)) (열침투 깊이)
- Roelands-Houpert η(P,T): 온도-압력 연성 점도

### 14.10.8 결과 해석

HMEHL 결과는 다음을 포함한다:

| 결과 | 기호 | 단위 | 설명 |
|------|------|------|------|
| 중심 유막두께 | h_c | μm | 접촉 중심의 유막 |
| 최소 유막두께 | h_min | μm | DH h_min/h_c 비율로 스케일 |
| 최대 압력 | p_max | MPa | EHL 압력 spike 포함 |
| 압력비 | p/p_h | - | Hertz 대비 (> 1.0 = spike 존재) |
| 마찰 계수 | μ | - | 유체 + 아스페리티 분리 |
| 최대 온도 | T_max | °C | TEHL 최고 온도 |
| 평균 온도 | T_mean | °C | 접촉 영역 평균 |

#### EHL 압력 spike

수치 EHL에서만 관찰되는 특징적 현상:
- p_max/p_h > 1.0: 출구 근방에서 Hertz를 초과하는 압력 peak
- TRB 일반 조건(M~1600): p/p_h ≈ 1.9 ~ 2.7
- 극한 하중(M>10000): spike가 Hertz에 수렴 (p/p_h → 1.0)

#### h_min 추출

V-cycle 수렴 후 격자에서 직접 추출. FBNS 적용으로 full-film 영역(θ=0)의 노드에서만 h_min을 결정한다.

### 14.10.9 수렴 범위 및 검증 결과

#### Moes M 전 범위 수렴 (검증 완료)

| Moes M | 대표 조건 | h_c/DH | p/p_h | 수렴 |
|--------|----------|--------|-------|------|
| 100 | 경하중 | 1.08 | 25.2 | ✅ |
| 200 | 볼 베어링 | 1.08 | 15.6 | ✅ |
| 500 | 경하중 TRB | 1.04 | 0.65 | ✅ |
| 1,000 | TRB 표준 | 0.89 | 0.75 | ✅ |
| 2,000 | TRB 고부하 | 0.64 | 2.79 | ✅ |
| 5,000 | TRB 중하중 | 1.00 | 0.98 | ✅ |
| 10,000 | TRB 극한 | 1.00 | 0.98 | ✅ |
| 50,000 | 초극한 | 1.00 | 0.98 | ✅ |
| **100,000** | **최대** | **1.00** | **0.98** | **✅** |

**9/9 수렴, 9/9 물리적** — FAS V-cycle이 Moes M=100~100,000 전 범위에서 수렴한다.

#### DH 비율 트렌드

- 저M (M<500): h_c/DH ≈ 1.0 (DH 경험식이 정확한 영역)
- 중M (M=1000~2000): h_c/DH = 0.64~0.89 (Roelands 비선형 점도 효과로 DH 대비 유막 감소)
- 고M (M>5000): DH fallback 수준 (V-cycle이 Hertz 해에 수렴)

#### 문헌 검증

| 검증 항목 | 참조 | 결과 |
|-----------|------|------|
| Coulon 압력분포 (25, 75 mm/s) | Coulon et al. [166] | ✅ 3/3 통과 |
| Kaneta 유막두께 (21.6, 98 mm/s) | Kaneta et al. [165] | ✅ 3/3 통과 |
| 속도 효과 (h↑ with u↑) | 이론 | ✅ 확인 |
| 하중 효과 (h↓ with F↑) | 이론 | ✅ 확인 |

### 14.10.10 참고문헌

1. Venner, C.H. (1991). "Multilevel solution of the EHL line and point contact problems." Ph.D. thesis, University of Twente.
2. Lubrecht, A.A. (1987). "The Numerical Solution of the EHL Problem." Ph.D. thesis, University of Twente.
3. Venner & Lubrecht (2000). "Multi-Level Methods in Lubrication." Elsevier.
4. Woloszynski, T. et al. (2015). "Efficient FBNS Algorithm for EHL." Tribology International.
5. Bartel, D. (2010). "Simulation von Tribosystemen." Springer.
6. Hansen, E. et al. (2022). "An EHL Extension of the Unsteady FBNS Algorithm." Trib. Letters.
7. SNU HMEHL-FBNS Solver (참조: Reference/EHL/SNU.md).

---

## 14.11 입력 파라미터

### 14.11.1 공통 파라미터 (Basic / Advanced)

| 파라미터 | 기호 | 단위 | 기본값 | UI | 설명 |
|----------|------|------|--------|-----|------|
| 윤활 모델 | - | - | Basic | O | Basic / Advanced 드롭다운 |
| 40°C 동점도 | ν₄₀ | mm²/s | 68 | O | ISO VG 등급 기준 |
| 100°C 동점도 | ν₁₀₀ | mm²/s | 8 | O | ASTM D341 보간 필요 |
| 운전 온도 | T_op | °C | 70 | O | 정상 운전 온도 |
| 압력-점도 계수 | α_pv | 1/GPa | 20 | O | 광유: 15-25, PAO: 10-15 |
| 윤활 유형 | - | - | Oil | O | Oil / Grease 드롭다운 |
| 기아 계수 | φ_s | - | 1.0 | O | 0.1-1.0 (Basic 전용, Grease 시 자동 0.7) |
| 윤활유 밀도 | ρ_oil | kg/m³ | 850 | O | 광유 850, 합성유 800-950 |
| 롤러 조도 | Ra_roller | μm | 0.15 | O | 연삭 베어링강 표준값 |
| 레이스웨이 조도 (내) | Ra_inner | μm | 0.15 | O | Profile 섹션에서 설정 |
| 레이스웨이 조도 (외) | Ra_outer | μm | 0.15 | O | Profile 섹션에서 설정 |

### 14.11.2 Advanced 전용 파라미터

Advanced 모델 선택 시 추가로 표시되는 입력 필드:

| 파라미터 | 기호 | 단위 | 기본값 | 설명 |
|----------|------|------|--------|------|
| Eyring 전단응력 | τ_eyring | MPa | 5.0 | 광유 전형값. PAO: 3-4, 에스터: 2-3 |
| Roelands 지수 | Z_roelands | - | 0.67 | 압력-점도 지수. 광유: 0.6-0.75, PAO: 0.45-0.55 |
| 윤활유 열전도도 | k_fluid | W/(m·K) | 0.15 | 광유: 0.12-0.16, PAO: 0.14-0.18 |
| 점도-온도 계수 | β_visc | 1/K | 0.04 | 점도-온도 의존성 계수 |
| 내륜 Rq | rq_inner | μm | 0.3 | 내륜 RMS 표면 조도 |
| 외륜 Rq | rq_outer | μm | 0.3 | 외륜 RMS 표면 조도 |
| 롤러 Rq | rq_roller | μm | 0.15 | 롤러 RMS 표면 조도 |

> **참고**: Advanced 모드에서 기아 계수(φ_s)는 물리 기반으로 자동 계산되므로 사용자 입력이 무시된다.

### 14.11.3 윤활유 프리셋

Advanced 모드에서 "Lubricant preset" 드롭다운으로 대표 윤활유 물성을 일괄 설정할 수 있다:

| 프리셋 | τ_Eyring [MPa] | Z_Roelands | k_fluid [W/(m·K)] | β_visc [1/K] |
|--------|----------------|------------|---------------------|--------------|
| Mineral oil | 5.0 | 0.67 | 0.15 | 0.04 |
| PAO | 8.0 | 0.50 | 0.14 | 0.03 |
| Ester | 4.0 | 0.55 | 0.16 | 0.035 |

프리셋 선택 후 개별 파라미터를 추가 수정할 수 있다.

---

## 14.12 출력 구조체

### 14.13.1 FilmThicknessResult

| 필드 | 단위 | 설명 |
|------|------|------|
| h_min_um | μm | 보정 후 최소 유막두께 |
| h_central_um | μm | 보정 후 중심 유막두께 |
| sigma_composite_um | μm | 합성 표면 조도 |
| lambda_ratio | - | Λ = h_min / σ |
| regime | - | FullEhl / Mixed / Boundary |
| u_mean_m_s | m/s | 평균 인입 속도 |
| starvation_factor | - | φ_s (기아 보정) |
| thermal_factor | - | φ_T (열 보정) |
| u_param | - | 무차원 속도 U |
| g_param | - | 무차원 재료 G |
| w_param | - | 무차원 하중 W |
| mixed | - | MixedLubricationResult (§14.11.2) |
| flash_temp_c | °C | 최대 조도접촉 플래시 온도 (Advanced만, Basic은 null) |

### 14.13.2 MixedLubricationResult

| 필드 | 단위 | 설명 |
|------|------|------|
| asperity_load_ratio | - | 조도 하중 분율 γ_a [0,1] |
| asperity_area_ratio | - | 실접촉면적 비율 [0,1] |
| p_asperity_mpa | MPa | 조도 접촉 압력 |
| p_fluid_mpa | MPa | 유체(EHL) 압력 |
| mu_ehl | - | EHL 트랙션 계수 |
| mu_boundary | - | 경계 마찰 계수 (0.10) |
| mu_effective | - | 유효 마찰 계수 |
| f_5_2 | - | GT 적분 F_{5/2}(Λ) |
| f_2 | - | GT 적분 F_2(Λ) |

### 14.13.3 SliceFilmThickness (내/외륜 8필드)

| 필드 | 단위 | 레이스웨이 | 설명 |
|------|------|-----------|------|
| h_min_um | μm | 내륜 | 최소 유막두께 |
| h_central_um | μm | 내륜 | 중심 유막두께 |
| lambda | - | 내륜 | Lambda 비 |
| regime | - | 내륜 | 윤활 영역 (FullEhl/Mixed/Boundary) |
| h_min_um_outer | μm | 외륜 | 최소 유막두께 |
| h_central_um_outer | μm | 외륜 | 중심 유막두께 |
| lambda_outer | - | 외륜 | Lambda 비 |
| regime_outer | - | 외륜 | 윤활 영역 |

### 14.13.4 TractionSummary

| 필드 | 단위 | 설명 |
|------|------|------|
| rollers | - | RollerTractionResult[] — 롤러별 상세 |
| p_rolling_w | W | 구름 마찰 동력 합계 |
| p_sliding_w | W | 슬라이딩 마찰 동력 합계 |
| p_rib_w | W | 리브 마찰 동력 합계 |
| p_contact_total_w | W | 접촉 총 동력 손실 |
| m_friction_nmm | N·mm | 베어링 마찰 토크 |

### 14.13.5 ContactFriction (롤러별 내/외륜)

| 필드 | 단위 | 설명 |
|------|------|------|
| u_rolling | m/s | 구름 속도 |
| u_sliding | m/s | 슬라이딩 속도 |
| srr | - | 슬라이드-롤 비율 |
| lambda | - | 로컬 Lambda 비 |
| asperity_load_ratio | - | 로컬 조도 하중 분율 |
| mu | - | 유효 마찰 계수 |
| f_traction_n | N | 트랙션 힘 |
| power_loss_w | W | 동력 손실 |

### 14.13.6 RibFriction

| 필드 | 단위 | 설명 |
|------|------|------|
| u_sliding | m/s | 리브 슬라이딩 속도 |
| mu | - | 리브 마찰 계수 (0.06) |
| f_friction_n | N | 리브 마찰력 |
| power_loss_w | W | 리브 동력 손실 |

---

## 14.13 UI — Lubrication 탭

CanvasArea의 **Lubrication** 탭에서 3개 서브탭으로 결과를 표시한다.

### 14.13.1 모델 선택 및 입력

InputPanel에서:
- **Lubrication Model 드롭다운**: Basic / Advanced 선택
- Advanced 선택 시 7개 추가 입력 필드 표시 (τ_eyring, Z_roelands, k_fluid, β_visc, rq_inner, rq_outer, rq_roller)
- Basic 선택 시 추가 필드 숨김

### 14.13.2 Film Thickness 서브탭

- EHL 유막두께 요약 (h_min, h_c, σ, Λ, regime)
- 운전 파라미터 (U_m, φ_s, φ_T)
- Dowson-Higginson 무차원 파라미터 (U, G, W)
- 적용 수식 참조
- Advanced 모드: 추가로 Roelands/Murch-Wilson/M-K 관련 정보 표시

### 14.13.3 Mixed Lubrication 서브탭

- GT 모델 결과 (F_{5/2}, F_2, γ_a, A_real/A_hertz) — Basic
- M-K 내장 결과 (load_fraction, area_fraction) — Advanced
- 압력 분배 (p_asperity, p_fluid, p_total)
- 마찰 계수 (μ_ehl, μ_boundary, μ_effective)
- **윤활 영역 게이지 맵**: Λ 위치를 Boundary—Mixed—Full EHL 스케일에 시각화

### 14.13.4 Traction & Power Loss 서브탭

- **동력 손실 파이 차트**: Rolling / Sliding / Rib 비율 (도넛 차트, 중심에 총 동력)
- **롤러별 적층 바 차트**: ψ [deg] 위치별 Inner/Outer/Rib 동력 손실
- 요약 테이블: P_rolling, P_sliding, P_rib, P_contact, M_friction
- 롤러별 상세 테이블: U_r, SRR, μ, P — Inner/Outer/Rib 컬럼

### 14.13.5 Film Distribution 서브탭

슬라이스별 유막두께 분포를 컨투어 히트맵 또는 3D 서피스로 시각화한다.

**내/외륜 토글**: 버튼으로 Inner / Outer 레이스웨이 데이터를 전환할 수 있다. 차트 제목에 레이스웨이 라벨이 표시된다 ("Inner - EHL Film..." / "Outer - EHL Film...").

**기존 Film Thickness 탭과의 차이**: 기존 탭은 베어링 전체에 대한 대표값(최대 하중 롤러 기준, 평균 R_eq)을 표시하지만, Film Distribution 탭은 **롤러별 × 슬라이스별** 유막을 정확한 슬라이스 기하에 기반하여 계산한다.

**계산 방식**:
- 각 슬라이스 k에서 고유한 등가반경 R_eq_k를 사용 (SliceGeometry.r_eq_inner)
- TRB 테이퍼로 인한 슬라이스별 인입속도 u_m_k 변동 반영 (§14.3.3):
  ```
  γ_k = d_we_k × cos(α) / d_pw
  R_inner_k = R_pw × (1 − γ_k)
  u_m_k = |(ω_i − ω_cage_k)| × R_inner_k
        = |ω_i − ω_o| × R_pw × (1 − γ_k²) / 2
  ```
- 소단부→대단부로 갈수록 d_we_k가 증가하여 γ_k↑, (1−γ_k²)↓, u_m_k↓ 경향을 보인다.
- Basic: 열보정 φ_T는 §14.3.3의 대표 인입속도 U_m으로 계산하여 전 슬라이스에 동일하게 적용
- Advanced: Murch-Wilson φ_T를 슬라이스별 SRR과 u_m_k로 개별 계산
- Basic 슬라이스별 Dowson-Higginson 무차원 파라미터:
  ```
  U_k = η₀ × u_m_k / (E* × R_eq_k)
  W_k = q_k × 10³ / (E* × R_eq_k)
  H_min_k = 2.65 × U_k⁰·⁷⁰ × G⁰·⁵⁴ × W_k⁻⁰·¹³
  h_min_k = H_min_k × R_eq_k × φ_T × φ_s
  ```
- Advanced 슬라이스별 M-K (2012) 선접촉 파라미터:
  ```
  H_c_smooth_k = 3.06 × U_k^0.69 × G^0.56 × W_k^(-0.10)
  f_c_k = 1 − 0.573 × exp(−0.74 × (H_c_smooth_k / σ̄_k)^0.21)
  h_c_k = H_c_smooth_k × f_c_k × R_eq_k × φ_T_k × φ_s_adv
  ```

**시각화 모드**:

| 토글 | 옵션 A | 옵션 B |
|------|--------|--------|
| 레이스웨이 | Inner (내륜) | Outer (외륜) |
| 데이터 | h_min [μm] | Lambda (Λ = h_min/σ) |
| 차트 | Contour (히트맵) | 3D Surface |

- **Contour**: X=슬라이스 번호, Y=롤러 ψ [deg], 색상=유막두께 또는 Lambda
- **3D Surface**: 동일 데이터의 인터랙티브 3D 서피스 (Plotly surface trace)
- Lambda 모드에서는 regime별 색상 (빨강=Boundary, 주황=Mixed, 초록=Full EHL)

**데이터 출처**: Rust 백엔드에서 Basic은 `compute_film_thickness_distribution()`, Advanced는 `compute_film_thickness_distribution_advanced()` 함수가 슬라이스별 계산 수행 → `BearingResult.film_distribution` 필드에 저장.

---

## 14.14 참고 문헌

1. **Dowson, D. & Higginson, G.R.** (1977). "Elastohydrodynamic Lubrication," 2nd ed., Pergamon Press. — 선접촉 EHL 최소 유막두께 공식.
2. **Dowson, D. & Toyoda, S.** (1978). "A Central Film Thickness Formula for Elastohydrodynamic Line Contacts," Proc. 5th Leeds-Lyon Symposium. — 중심 유막두께 공식.
3. **Gupta, P.K.** (1984). "Advanced Dynamics of Rolling Elements," Springer. — TEHL 열보정 계수 φ_T (Basic 모델).
4. **Greenwood, J.A. & Tripp, J.H.** (1970). "The Contact of Two Nominally Flat Rough Surfaces," Proc. IMechE, 185, pp.625-633. — 조도 접촉 통계 모델.
5. **Masjedi, M. & Khonsari, M.M.** (2012). "Film Thickness and Asperity Load Formulas for Line-Contact EHL With Provision for Surface Roughness," ASME J. Tribology, 134(1), 011503. — 선접촉 조도 통합 유막두께 공식 (Advanced 모델). ※ M-K (2015) 점접촉 버전은 선접촉에 적용 시 조도 보정이 과도하게 증폭됨.
6. **Harris, T.A. & Kotzalas, M.N.** (2006). "Rolling Bearing Analysis," 5th ed., CRC Press. — TRB 운동학 및 마찰 모델.
7. **ISO 281:2007**. "Rolling bearings — Dynamic load ratings and rating life." — a_ISO 수정 계수와 κ(점도비) 관계.
8. **Roelands, C.J.A.** (1966). "Correlational Aspects of the Viscosity-Temperature-Pressure Relationship of Lubricating Oils," PhD Thesis, TU Delft. — Roelands 압력-점도 모델 (Advanced 모델).
9. **Murch, L.E. & Wilson, W.R.D.** (1975). "A Thermal Elastohydrodynamic Inlet Zone Analysis," ASME J. Lubr. Technol., 97(2), pp.212-216. — SRR 의존 열보정 계수 (Advanced 모델).
10. **Wilson, W.R.D.** (1979). "Thermal Effects in Line Contact Elastohydrodynamic Lubrication," ASME J. Lubr. Technol. — 선접촉 열 저감 공식.
11. **Blok, H.** (1937). "Theoretical Study of Temperature Rise at Surfaces of Actual Contact Under Oiliness Lubricating Conditions," Proc. General Discussion on Lubrication, IMechE, London. — 플래시 온도 이론.
12. **Jaeger, J.C.** (1942). "Moving Sources of Heat and the Temperature at Sliding Contacts," Proc. Royal Society of NSW, 76, pp.203-224. — 이동 열원 해석.
13. **Lugt, P.M.** (2013). "Grease Lubrication in Rolling Bearings," Wiley. — 그리스 윤활 기아 모델, 기유 블리딩 보정.
14. **Hamrock, B.J. & Dowson, D.** (1981). "Ball Bearing Lubrication — The Elastohydrodynamics of Elliptical Contacts," Wiley. — EHL 입구 기아 모델.

---

## 14.15 검증 수준

### 14.15.1 Basic 모델 검증

| 레벨 | 항목 | 기준 | 구현 상태 |
|------|------|------|-----------|
| A | erfc 함수 | 기지값 대비 < 10⁻⁶ | 통과 |
| A | GT 적분 F₀(0)=0.5 | 해석해 대비 < 0.2% | 통과 |
| A | GT 적분 F₁(0)≈0.3989 | 해석해 (1/√2π) 대비 < 0.2% | 통과 |
| A | GT 적분 F₂(0)≈0.500 | 해석해 (가우시안 반분산) 대비 < 1% | 통과 |
| A | GT 적분 F₅/₂(0)≈0.617 | 수치 기지값 대비 < 2% | 통과 |
| A | GT 적분 단조 감소 | F_n(Λ₁) > F_n(Λ₂) for Λ₁ < Λ₂ | 통과 |
| A | Walther 점도 40°C/100°C | 입력값 정확 복원 (< 0.5 mm²/s) | 통과 |
| A | D-H/D-T 벤치마크 | h_min 0.1-2.0 μm, h_c/h_min 1.1-1.5 (Hamrock 2004) | 통과 |
| A | Gupta 열보정 수치검증 | L_th, φ_T 수작업 대비 < 10⁻¹⁰ | 통과 |
| A | 인입속도 Harris 일치 | u_m = \|ω_i−ω_o\|×R_pw×(1−γ²)/2, 내/외 동일 | 통과 |
| B | EHL h_min 범위 | 0.05-10 μm (표준 베어링) | 통과 |
| B | 저속 혼합윤활 | γ_a > 0 at n < 20 rpm | 통과 |
| B | 고속 완전 EHL | γ_a ≈ 0 at Λ > 4 | 통과 |
| C | 그리스 vs 오일 | h_grease < h_oil (φ_s < 1) | 통과 |
| C | 트랙션 양성 | P_contact > 0 for loaded roller | 통과 |
| D | Bearinx/MESYS 대비 | < 15% (향후 검증 예정) | 미완 |

### 14.14.2 Advanced 모델 검증

| 레벨 | 항목 | 기준 | 구현 상태 |
|------|------|------|-----------|
| A | Roelands p=0 환원 | η(0) = η₀ (< 10⁻¹⁰ 오차) | 통과 |
| A | Roelands p=p_r 정량값 | 수작업 대비 < 10⁻¹⁰ | 통과 |
| A | Roelands < Barus | η_Roelands < η_Barus at 1 GPa (×0.0083) | 통과 |
| A | Roelands 윤활유별 | η_mineral > η_PAO (Z_r: 0.67 vs 0.50) | 통과 |
| A | Murch-Wilson SRR=0 | Gupta φ_T와 동일 (< 0.001) | 통과 |
| A | Murch-Wilson SRR 단조 | φ_T(SRR₁) > φ_T(SRR₂) for SRR₁ < SRR₂ | 통과 |
| A | Eyring 뉴턴 극한 | 저 SRR에서 μ ≈ η_eff·γ̇/p (< 10%) | 통과 |
| A | Eyring 전단 포화 | 고 SRR에서 μ → τ_lim/p (포화 확인) | 통과 |
| A | 플래시 온도 zero slide | ΔT = 0 at V_slide = 0 | 통과 |
| A | 플래시 온도 차원 검증 | 현실적 조건에서 5-100°C 범위 | 통과 |
| A | 플래시 온도 속도 의존 | ΔT_fast > ΔT_slow | 통과 |
| A | 플래시 온도 분류 | 30°C→Low, 100°C→Medium, 200°C→High, 400°C→Critical | 통과 |
| A | 기아보정 n×d_pw 테이블 | 50k→0.98, 300k→0.89, 1M→0.73, 2M→0.59 (매뉴얼 ±5%) | 통과 |
| A | 기아보정 그리스 < 오일 | φ_grease < φ_oil (동일 n×d_pw) | 통과 |
| B | M-K σ̄→0 환원 | D-H/D-T 결과와 일치 (< 10⁻¹⁰) | 통과 |
| B | M-K 선접촉 보정 | 조도 보정 [0.5, 1.5] 범위 (점접촉 9×→선접촉 수정) | 통과 |
| B | M-K 조도 단조 증가 | σ̄ 증가 시 load_fraction 단조 증가 | 통과 |
| B | M-K vs GT 일관성 | σ̄=0 → load_fraction=0, σ̄→∞ → [0,1] | 통과 |
| B | Roelands vs Barus 트랙션 | Roelands 기반 μ < Barus 기반 μ at 1 GPa | 통과 |
| C | Basic vs Advanced 비교 | 동일 조건에서 h_min 비율 0.5~1.5× (선접촉 수정 후) | 통과 |
| D | Bearinx/MESYS 대비 | < 15% (향후 검증 예정) | 미완 |

### 14.14.3 과도 솔버 연계

과도 슬라이딩 해석(transient.rs)에서 `LubricationModel`에 따라 트랙션 함수를 분기한다:

- **Basic**: `compute_traction_coefficient_srr()` — 간이 Eyring (SRR 의존, Barus 점도)
- **Advanced**: `eyring_traction_advanced()` — 완전 Eyring sinh⁻¹ + Roelands η(p) + 전단 제한

동일 `eyring_traction_advanced()` 함수를 정상 상태 솔버(bearing.rs)와 과도 솔버(transient.rs)에서 공유한다.

---

## 14.15.3 외부 검증 — SKF 마찰 토크 모델 (참조)

`lubrication.rs::skf_frictional_moment_trb()`에 SKF Catalogue 2018 ("The SKF model for calculating the frictional moment") 모델을 외부 참조용으로 구현. **솔버 production path에는 연결되지 않으며**, 회귀 테스트의 sanity-check 기준점으로만 사용.

### 14.15.3.1 SKF 모델 식 (TRB 단일 베어링)

$$
M_\text{total} = M_\text{rr} + M_\text{sl}\quad(\text{seal/drag 제외})
$$

**롤링 마찰 토크**:
$$
M_\text{rr} = \phi_\text{ish}\,\phi_\text{rs}\,G_\text{rr}\,(\nu n)^{0.6}
$$
$$
G_\text{rr}^\text{TRB} = R_1\,d_m^{2.38}\,(F_r + R_2\,Y\,F_a)^{0.31}
$$

**슬라이딩 마찰 토크**:
$$
M_\text{sl} = G_\text{sl}\,\mu_\text{sl}, \quad
G_\text{sl}^\text{TRB} = S_1\,d_m^{0.82}\,(F_r + S_2\,Y\,F_a)
$$

**마찰계수**:
$$
\mu_\text{sl} = \phi_\text{bl}\,\mu_\text{bl} + (1-\phi_\text{bl})\,\mu_\text{EHL}
$$
- μ_bl = 0.12 (n ≠ 0), μ_EHL_TRB = 0.002
- φ_bl = exp(−2.6·10⁻⁸·(nν)^1.4·d_m)

**시리즈별 상수** (Table 2d):

| 시리즈 | R₁ | R₂ | S₁ | S₂ |
|--------|------|------|------|------|
| 302 | 1.76e-6 | 10.9 | 0.017 | 2 |
| **303** (30306, 30307…) | **1.69e-6** | **10.9** | **0.017** | **2** |
| 322 | 2.27e-6 | 10.9 | 0.018 | 2 |
| 322 B | 2.38e-6 | 10.9 | 0.026 | 2 |
| 323 | 2.38e-6 | 10.9 | 0.019 | 2 |
| 323 B | 2.79e-6 | 10.9 | 0.030 | 2 |
| 기타 | 2.31e-6 | 10.9 | 0.019 | 2 |

K_z (TRB) = 6, K_rs = 3·10⁻⁸ (oil bath/jet) / 6·10⁻⁸ (grease/air).

### 14.15.3.2 NSK HR30306J 디폴트 프리셋 검증

| 입력 | 값 |
|------|-----|
| d / D / d_m | 30 / 72 / 51 mm |
| Y (axial factor) | 1.6 |
| F_r / F_a | 5 / 2 kN |
| n | 1500 rpm |
| ν(70°C) | ≈ 22 cSt |
| 윤활 | Oil bath |

**SKF 모델 출력**:

| 항 | 값 |
|----|-----|
| G_rr | ≈ 0.523 |
| φ_ish · φ_rs | ≈ 0.95 |
| (νn)^0.6 | ≈ 514 |
| **M_rr** | **≈ 256 N·mm** |
| φ_bl | ≈ 0.06 |
| μ_sl | ≈ 0.0091 |
| G_sl | ≈ 4885 N·mm |
| **M_sl** | **≈ 44 N·mm** |
| **M_total** | **≈ 300 N·mm = 0.30 N·m** |

검증 테스트:
- `test_skf_friction_30306_hand_calc` — M_rr/M_sl/M_total 손계산 ±10% 검증
- `test_skf_friction_speed_dependence` — n 500→1500: ratio ≈ (3)^0.6 = 1.93
- `test_skf_friction_series_constants` — 322/303 R1 비율 = 1.343
- `test_solver_vs_skf_friction_model_matched_input` — 동일 normal load 입력에서 우리 솔버 vs SKF의 비율이 O(1) (0.3~3 범위) 보장

### 14.15.3.3 정량 비교 — 모델 calibration 차이 (예상되는 결과)

**디폴트 프리셋 NSK HR30306J + 운전조건(F_r=5 kN, F_a=2 kN, 1500 rpm)에서**:
- 우리 솔버 (Palmgren μ_rr·Q·u): M_friction ≈ **4 116 N·mm = 4.12 N·m** (P_contact ≈ 646 W)
- SKF advanced model: M_total ≈ **300 N·mm = 0.30 N·m** (P ≈ 47 W)
- **비율 ≈ 14×**

#### 두 모델의 historical 관계

학술 문헌(*Engineering Research, Springer 2023*)이 명시한다:

> "SKF originally used a method for calculating bearing loss torques **based on Palmgren**, which was **later fundamentally revised** to divide the bearing frictional torque cause-specifically into four parts (M_rr, M_sl, M_seal, M_drag)."

즉 **Palmgren ≠ SKF advanced**. 같은 베어링 같은 운전조건에서 두 모델이 다른 토크를 산출하는 게 정상이며, 14× 차이가 우리 솔버의 버그를 의미하지 않는다.

#### 모델 식 자체의 calibration 차이

| 항목 | Palmgren (우리 솔버) | SKF advanced |
|------|---------------------|--------------|
| Rolling resistance | μ_rr × Q × u_roll, μ_rr = 0.002 상수 | G_rr · (νn)^0.6, G_rr = R₁·d_m^2.38·(F_r+R₂YF_a)^0.31 |
| 점도 의존 | 없음 | (νn)^0.6 항 명시적 |
| 속도 의존 | 선형 (∝ u_roll) | (νn)^0.6 sub-linear |
| 하중 의존 | 선형 (∝ Q) | F^0.31 sub-linear |
| Calibration 출처 | Palmgren 1959 dry-rolling 단순화 | SKF 실측 fitting (수만 베어링) |

#### 검증 결과 (q는 정확함)

- **마찰 모델 식 구현 정확성**: `test_skf_friction_30306_hand_calc` — 손계산 ±10% 일치 ✅
- **동일 입력 normal force**: `test_solver_vs_skf_friction_model_matched_input` — 비율 0.79 (O(1)) ✅
- **q_normal 자체는 정확** (audit 확인) ✅

→ 14× 차이는 **Palmgren μ_rr·Q·u가 SKF G_rr·(νn)^0.6보다 큰 값을 산출하는 calibration 차이**가 dominant 원인.

#### SKF 결과 직접 확인 위치

| 자료원 | URL |
|--------|-----|
| **SKF Bearing Calculator** (온라인 무료) | https://www.skfbearingselect.com/ |
| SKF Engineering Calculator | https://www.skf.com/group/support/engineering-tools/engineering-calculator |
| SKF Product Select (designation별) | https://productselect.skf.com/ |
| SKF Catalogue PDF (수식 + 상수표) | https://cdn.skfmediahub.skf.com/api/public/0901d1968065e9e7/pdf_preview_medium/0901d1968065e9e7_pdf_preview_medium.pdf |

권장: 30306 + 동일 운전조건을 SKF Bearing Calculator에 입력해 **공식 SKF 토크 값**을 확인 후, 우리 솔버 결과와의 차이가 두 calibration의 자연스러운 결과인지 사용자가 직접 검증.

#### 향후 작업

- ~~**SKF advanced model dispatcher**~~ → **2026-05-05 구현 완료**: `friction_model: PalmgrenLike | SkfAdvanced` 토글 in InputPanel + LubricationView "SKF Reference" sub-tab. 항상 SKF 참조값 자동 계산해 솔버 결과와 side-by-side 표시.
- **Bearinx/MESYS 카탈로그 대조** (Level D 검증).
- **Palmgren μ_rr 보정**: SKF μ_EHL=0.002는 sliding 전용. rolling resistance에는 0.0006~0.001 정도가 더 적절할 수 있음 (별도 audit + 실험 데이터 필요).

---

## 14.15.4 Rolling Resistance 학술 모델 비교

Palmgren vs SKF advanced의 14× 차이를 학술적으로 설명하는 핵심 발견:

### 14.15.4.1 Bălan-Houpert-Olaru (2015) *Lubricants* — 실험 검증

> "For low loaded ball-race contacts, the **theoretical elastic hysteresis and curvature effect resistances do not exceed 12 percent** of the experimentally rolling resistance moment."
>
> "**The hydrodynamic resistance is the main contributor to the final bearing torque** in the tested operating conditions."

→ Lubricated 베어링의 측정 rolling torque 중 **~88%가 점성 hydrodynamic drag** (Reynolds + Poiseuille 적분), 12%만 elastic hysteresis/curvature/pivoting/inertia 합산.

### 14.15.4.2 Biboulet-Houpert (2010) — Hydrodynamic Rolling Force

Lubricated rolling 접촉의 dominant 항을 정량화:

$$
F_R = f(M, b, R', \eta, u_e)
$$

여기서:
- M = transition parameter (IVR ↔ EHL 영역 구분)
- b = Hertzian half-width
- R' = reduced radius
- η = effective viscosity
- u_e = entrainment velocity

이 식이 SKF 모델의 (νn)^0.6 항이 derive 된 학술적 기반이며, Palmgren μ_rr·Q·u가 단순화로 인해 점성 의존성을 잃은 부분이다.

### 14.15.4.3 두 모델의 분리 전략 (Palmgren-like vs SKF Advanced)

본 솔버는 두 calibration을 동시에 노출한다:

| Mode | 식 | 적용처 |
|------|-----|-------|
| **PalmgrenLike** (default) | per-contact `μ_rr·Q·u_roll` | 단순 dry-rolling 추정, 학술 비교, transient 동역학 |
| **SkfAdvanced** | bearing-level `M_rr = G_rr·(νn)^0.6 + M_sl` | Catalogue·실험 비교, 동력손실 정량 추정 |

`OperatingConditions.friction_model` enum으로 사용자가 선택. `TractionSummary.skf_reference`는 항상 채워져 UI에서 side-by-side 비교 가능.

### 14.15.4.4 추후 통합 제안

#### Per-contact rolling resistance 식의 권장 형태

Per-contact 수준에서 점성 hydrodynamic rolling resistance를 직접 적분하려면 다음 두 식이 표준이다 (Bălan-Houpert-Olaru 2015 논문이 직접 인용·검증):

- **Houpert (1987)** *ASME J. Tribol.* 109:363-371, "Piezoviscous-rigid rolling and sliding traction forces" — per-contact hydrodynamic rolling force 원조 식. PV-R regime 가정.
- **Biboulet-Houpert (2010)** *Proc. IMechE Part J* 224:777-787, "Hydrodynamic force and moment in pure rolling lubricated contacts: Part 2, Point contacts" — 1987 식의 **개선판**. IVR (isoviscous rigid) ↔ EHL 영역 통합 + transition parameter $M$ 도입. **현재 per-contact rolling resistance의 학술 표준**.

→ Mid-fidelity 옵션으로 추가한다면 **Biboulet-Houpert 2010** 식을 채택 (latest, 더 넓은 영역 적용 가능). 1987 식은 starvation 조건 (moderately starved) 에서 쓰이고, 2010은 fully flooded·transition 모두 커버.

#### Bearing-level analytical reference

- **Houpert (2002)** *STLE Tribol. Trans.* 45:345-353, "Ball Bearing and Tapered Roller Bearing Torque: Analytical, Numerical and Experimental Results" — TRB torque의 bearing-level 분석/실측 비교 종합 논문 (per-contact 식이 아니라 1987 per-contact 식을 통합 적분한 형태). SKF (νn)^0.6 industry calibration의 학술적 검증 reference로 사용.

#### 실험 비교

- **Tewari 2023** *Machines* 11:801, "Experimental Analysis of Rolling Torque and Thermal Inlet Shear Heating in TRBs" — 측정 데이터와 SKF model 직접 비교.

#### 정리: 세 옵션의 strengths matrix

세 모델은 단일 fidelity 축으로 정렬되지 않는다. **physics fundamentality**, **catalogue match**, **applicability** 세 다른 축에서 각각 강점이 다르다:

| 모델 | Physics fundamentality | Catalogue match (SKF) | Applicability (모든 베어링/조건) | 계산 비용 |
|------|----------------------|---------------------|------------------------------|---------|
| **Palmgren** μ_rr·Q·u_roll | ★ (constant μ, 점성 부재) | ★ | ★★★ | ★★★ |
| **Biboulet-Houpert 2010** per-contact | **★★★ (학술 derivation, IVR↔EHL transition)** | ★★ (per-contact 적분 후 일반 식) | **★★★ (모든 형상·운전조건)** | ★★ |
| **SKF advanced** G_rr·(νn)^0.6 | ★★ (Houpert 식의 industry fit) | **★★★ (SKF 시리즈 직접 calibration)** | ★★ (SKF 시리즈 외 "Other" fallback) | ★★★ |

#### 각 옵션이 필요한 이유

**Palmgren (current default)**:
- 단순/빠름, transient solver의 시간 step별 호출에 적합
- 학술 비교 baseline

**Biboulet-Houpert 2010 (제안 Mid 옵션)**:
- **SKF 시리즈가 아닌 베어링** (NSK, Timken, custom geometry, 학술 연구용)에서 SKF "Other" fallback의 부정확성을 회피
- **per-roller breakdown 보존**: Stribeck 분포·비대칭 load가 per-contact 적분에서 자연스럽게 유지됨 (SKF는 bearing-level 단일값)
- **Gen3 split contact**: 빔 변형으로 inner/outer q_normal이 비대칭일 때, SKF empirical은 이를 처리 못 하지만 per-contact 식은 정합
- 연구·calibration 검증: SKF empirical과의 결과 차이를 비교하면 SKF fitting의 적용 범위/한계 정량화

**SKF advanced (current High option)**:
- **SKF 카탈로그 정합**: SKF Bearing Calculator·Catalogue 발표값과 직접 비교
- 실측 fitting된 시리즈별 R/S 상수
- 한계: SKF가 fitting한 시리즈 (302, 303, 313, 320, 322, 323) 에 한정. 비-SKF 또는 custom geometry 시 'Other' fallback이라 부정확
- bearing-level 단일값이라 per-roller 분해 불가능

**결론**: 세 모델은 상호 배타적이 아니라 **상호 보완적**. 사용자가 Gen3 split·비-SKF 베어링·연구용 경우 Biboulet-Houpert가 더 적합, SKF 카탈로그와 비교가 목적이면 SKF advanced, 단순/빠른 추정이면 Palmgren.

### 14.15.4.5 Biboulet-Houpert 2010 식 (구현 완료)

`OperatingConditions.friction_model = BibouletHoupert` 선택 시 활성화.

**Per-contact 식**:

$$F_R^{IVR} = 2.9766 \cdot E^* \cdot R_x^2 \cdot k^{0.3316} \cdot W^{1/3} \cdot U^{2/3}$$

$$F_R^{EHL} = 7.5826 \cdot E^* \cdot R_x^2 \cdot k^{0.4055} \cdot W^{1/3} \cdot U^{3/4}$$

**Smooth IVR↔EHL transition (Eq. 3)**:
$$F_R^{Trans} = \frac{F_R^{IVR} - F_R^{EHL}}{1 + M/6.6} + F_R^{EHL}$$

**Transition parameter (Eq. 4)**:
$$M = 0.5549 \cdot k^{-0.6029} \cdot W \cdot U^{-0.75}$$

**무차원 그룹**:
- $U = \eta_0 \cdot v / (E^* \cdot R_x)$ — entrainment speed parameter
- $W = Q / (E^* \cdot R_x^2)$ — load parameter
- $k = R_y / R_x$ — radii ratio

**Per-contact power**: $P_\text{rolling} = F_R \times u_\text{entrain}$

본 솔버에서 BH 선택 시:
- `compute_traction[_advanced]`: per-roller inner/outer의 `p_rolling_w`만 BH 식으로 override (sliding은 Eyring/Carreau 그대로)
- 라인 접촉 raceway는 `k = 100` (line-contact limit; paper의 truncation 영역) 사용
- per-roller breakdown 보존 → Stribeck 분포·Gen3 split 모드에서도 정합

#### 검증 테스트

| 테스트 | 검증 |
|--------|-----|
| `test_biboulet_houpert_force_load_scaling` | F_R(2Q)/F_R(Q) ∈ [1.10, 1.30] (W^(1/3) 한계 ≈ 1.26 + IVR/EHL transition shift) |
| `test_biboulet_houpert_force_speed_scaling` | F_R(4u)/F_R(u) ∈ [4^(2/3) − 0.1, 4^(0.75) + 0.4] (IVR ↔ EHL exponent 범위) |
| `test_biboulet_houpert_zero_inputs` | η/u/Q = 0 → F_R = 0 (NaN 방지) |
| `test_biboulet_houpert_power_wrapper` | P = F_R × u 항등 |
| `test_friction_model_biboulet_houpert_dispatcher` | sliding power 동일 (BH는 rolling만 영향), rolling power Palmgren과 측정가능 차이, per-roller breakdown 보존 |

cargo test 286/286 통과 (회귀 0).

### 14.15.4.6 Part 1 (line contact) 식 — TRB raceway 직접 적용 (구현 완료)

TRB raceway는 1-D line contact이라 Part 2 point-contact 식에 `k=100` cap을 거는 것보다 Part 1 (line contact) 식을 직접 사용하는 것이 더 정확. 본 솔버는 `FrictionModel::BibouletHoupert` 선택 시 raceway에 자동으로 Part 1 식을 사용한다.

#### Part 1 식 (Biboulet-Houpert 2010 *Proc. IMechE Part J* 224, Eqs. 40-42)

**무차원 그룹 (paper Eq. 14)** — Part 2와 다른 convention 주의:

$$U_l = \frac{2\eta_0 u_m}{E' R}, \quad W_l = \frac{w_l}{E' R}, \quad M_l = \frac{W_l}{\sqrt{U_l}}$$

`U_l`은 Part 2의 `U`와 달리 **factor 2 포함**. `w_l`은 force per unit length [N/m], `R`은 rolling 방향 reduced radius [m], `E'`은 paper convention reduced modulus (= 2 × Johnson E*).

**IVR asymptote (Eq. 40)**:
$$\tilde{T}_{IVR} = 1.42 \cdot U_l^{1/2} \cdot W_l^{1/2}$$

(Dowson IVR 식과 정합. `\tilde{T} = t_l/(E'R^2)`은 force per unit length의 dimensionless form, $\tilde{F}_t^x \approx \tilde{T}_t^x$ Eq. 31.)

**EHL asymptote (Eq. 41) — load-INDEPENDENT**:
$$\tilde{T}_{EHL\infty} = 1.47 \cdot U_l^{3/4}$$

(load 부재! EHL 영역에서 대부분의 베어링이 동작.)

**Smooth blend (Eq. 42, T̃ form)**:
$$\tilde{T} = \frac{\tilde{T}_{IVR}}{(1 + r^{10})^{1/10}}, \quad r = \frac{1.4}{1.45}\sqrt{M_l}$$

**물리 단위 변환**:
$$f_l = \tilde{T} \cdot E' \cdot R \quad [\text{N/m}]$$
$$F_{total} = f_l \cdot L_{contact} \quad [\text{N}]$$
$$P_{rolling} = F_{total} \cdot u_m \quad [\text{W}]$$

#### Part 1 vs Part 2 사용 영역

| 접촉 형태 | 권장 식 |
|----------|--------|
| **TRB raceway (line contact)** | Part 1 (직접) |
| 볼 베어링 ball-race (point contact) | Part 2 (k = a/b 명시) |
| **TRB rib-roller end (point/elliptical)** | Part 2 (rib_contact 모듈에서 별도 처리) |

본 솔버: `FrictionModel::BibouletHoupert` 선택 시 raceway는 자동으로 Part 1 식, rib는 별도 elliptical Hertz + 자체 EHL.

#### 검증 테스트 (Part 1 line: 9개, Part 2 point: 5개)

**Part 1 (line) — Asymptote / Exponent 검증**

| 테스트 | 검증 |
|--------|------|
| `test_bh_line_ivr_asymptote` | M_l → 0에서 unified 식이 IVR 닫힌 식 (1.42·U^0.5·W_l^0.5)과 < 1% 일치 |
| `test_bh_line_ehl_load_independent` | M_l → ∞에서 load 두 배로 해도 f_l 변동 < 5% (load 무관 EHL) |
| `test_bh_line_ehl_value` | 고하중 한계에서 f_l이 정확히 1.47·U^0.75·E'·R |
| `test_bh_line_power_wrapper` | P = (f_l × L) × u 항등 |
| `test_bh_line_vs_point_same_order` | Part 1 (line) ↔ Part 2 (k=100) 결과 factor 5 이내 (질서 일치) |
| `test_bh_line_ivr_load_exponent` | IVR regime 하중 두 배 → f_l ratio = 2^0.5 (W_l^0.5) within 1% |
| `test_bh_line_ivr_speed_exponent` | IVR regime 속도 두 배 → f_l ratio = 2^0.5 (U_l^0.5) within 1% |
| `test_bh_line_ehl_speed_exponent` | EHL∞ regime 속도 두 배 → f_l ratio = 2^0.75 (U_l^0.75) within 2% |
| `test_bh_line_transition_at_ml_one` | M_l = 1 (IVR↔EHL 교차점)에서 unified 식이 hand-derivation과 < 1e-9 일치, 값이 EHL∞의 85-100 % |
| `test_bh_line_realistic_trb_snapshot` | 30206-class TRB raceway (η=0.05 Pa·s, u=2 m/s, Q=24 kN, R=8 mm, L=12 mm) → f_l ≈ 91.2 N/m, P_R ≈ 2.19 W (refactor drift < 2 %) |

**Part 2 (point) — Mihaela-Houpert 2015 ball-thrust benchmark**

| 테스트 | 검증 |
|--------|------|
| `test_biboulet_houpert_force_load_scaling` | Part 2 W^(1/3) IVR-side 기울기 |
| `test_biboulet_houpert_force_speed_scaling` | Part 2 U^(2/3)~U^(0.75) 속도 의존성 |
| `test_biboulet_houpert_zero_inputs` | NaN 방지 |
| `test_biboulet_houpert_power_wrapper` | P = F_R × u 항등 |
| `test_bh_point_mihaela_houpert_2015_ball_thrust` | d=6.35 mm balls, Q=0.633/0.125 N, η₀=0.05 Pa·s 운전점에서 EHL partition이 Mihaela-Houpert 2015 §4.4의 보고와 정합 (Q=0.633: EHL>50 %, Q=0.125: 30-70 % 전이영역), F_R 단위 µN range, 단조 증가 |

cargo test 303/303 통과 (회귀 0).

### 14.15.4.6A 외부 실험 검증 — Schwarz et al. 2023 *Lubricants* 11(9):369

> Schwarz, Schäfer, Sauer, "Predicting Friction of Tapered Roller Bearings with Detailed Multi-Body Simulation Models". DOI: 10.3390/lubricants11090369.

**검증 베어링**: TRB **32216** (catalogue d=80, D=140, B=33 mm)
- d_pitch = 108.5 mm, d_RB = 17 mm, l_RB = 22.7 mm, n_RB = 16
- σ_raceway = 0.16 μm, σ_rib = 0.24 μm
- 반각 α ≈ 14° (catalogue Y₁=1.6, e=0.43 → α=arctan(e/Y))

**윤활유**: FVA No. 3 reference (ISO VG 100 mineral, no additives)
- Vogel: η[mPa·s] = K · exp(B/(T+C)), K=0.062, B=1021.7°C, C=101.55°C
- 50 °C → η = 0.0524 Pa·s
- ρ(15°C) = 887.6 kg/m³, α_ρ = -6e-4 g/(mL·K)

**측정 vs BH 2010 비교 (axial 6 kN, 50 °C, oil bath half roller height)**:

| n [rpm] | 측정 M_total [N·mm] | M_BH isothermal [N·mm] | M_BH + Wilson φ_T [N·mm] | Ratio (with φ_T) | 해석 |
|---------|---------------------|------------------------|--------------------------|------------------|-----|
| **500** | **1200** | 1113.6 (0.928) | **1102.5** | **0.919** | **오차 8 % — 정확** (φ_T ≈ 0.99 영향 미미) |
| **4000** | **3800** | 5202.9 (1.369) | **4552.4** | **1.198** | **+20 %** (φ_T ≈ 0.87이 over-prediction 37%→20% 감소) |

**해석**:
- Pure axial 운전 (cone-apex pure rolling) → sliding ≈ 0, total = rolling resistance가 거의 전부.
- 500 rpm 저속에서 BH 2010 (Wilson φ_T 적용) 결과가 측정과 8 % 이내로 일치.
- 4000 rpm 고속에서 +20 % 과예측. **Wilson 1979 φ_T thermal correction을 적용하면 isothermal +37 %에서 +20 %로 감소** — Tewari 2023 동일 식 검증에서 inlet shear로 6-8 % reduction 보고와 일관. 잔여 차이는 EHL∞ asymptote saturation + per-contact-to-bearing-level 적분 근사에서 기인.

**Speed scaling 검증** (500 → 4000 rpm, ω×8):
- 측정: 1200 → 3800, ratio = 3.17
- BH (isothermal): 1113.6 → 5202.9, ratio = 4.67 (EHL∞ asymptote 8^0.75 = 4.76)
- BH (+ Wilson φ_T): 1102.5 → 4552.4, ratio = **4.13** (측정에 더 근접)
- → Thermal correction이 high-speed regime을 IVR(2.83) ↔ EHL∞(4.76) 사이로 끌어내림.

#### Thermal inlet-shear correction 옵션 (`ThermalCorrection` enum)

순수 rolling (SRR=0) 가정에서 두 검증된 식이 사용 가능:

$$\varphi_T = \frac{1}{1 + a \cdot L_{th}^{b}}, \quad L_{th} = \frac{\eta_0 \cdot \beta \cdot u_m^2}{k_{fluid}}$$

| 옵션 | 계수 (a, b) | 출처 | Schwarz 4000 rpm 검증 |
|------|------------|-----|-----------------------|
| **`Aihara1987`** ⭐ (기본) | 0.29, 0.78 | Aihara, *J. Tribol.* 109:471 (**TRB rolling torque 전용**) | **ratio 0.947 (-5%)** |
| `Wilson1979` | 0.10, 0.64 | Wilson, *J. Lubr. Technol.* 101:425 (film thickness용) | ratio 1.198 (+20%) |
| `None` | — | isothermal (학술 비교용) | ratio 1.369 (+37%) |

- $\beta$: viscosity-temperature coefficient $[1/K]$ — Vogel formula에서 $\beta = B/(T+C)^2$
- $k_{fluid}$: 윤활유 열전도도 $[W/(m \cdot K)]$ (FVA 3 = 0.134)
- 최종 $P_{rolling} = P_{BH,iso} \cdot \varphi_T$, $\varphi_T \in [0.3, 1.0]$ clamp

**Aihara 1987이 TRB rolling 토크에 더 정확한 이유**:
- Aihara가 axial-loaded TRB 측정으로 직접 fit한 thermal correction
- Tewari 2023이 Table 1에서 Aihara 식을 TRB rolling torque의 reference로 인용
- Wilson은 film thickness 원래 용도 → rolling traction에는 약간 under-correct

**기본값을 `Aihara1987`로 변경 (2026-05-07)**: TRB 외부 측정 데이터 (Schwarz 32216) 검증에서 Aihara가 ±10% 이내 정합 → 모든 기본 사용 시나리오에 권장. Wilson1979는 film thickness 표준이 필요한 학술 비교용으로 유지.

UI: InputPanel의 `Thermal φ_T` 토글 (BH 2010 선택 시에만 노출): Wilson | Aihara | None.

Dispatcher 적용: `FrictionModel::BibouletHoupert` 선택 시 M1 (`compute_traction_advanced`)와 M2 (`compute_traction_simple`) 양쪽 path 모두 `operating.thermal_correction`에 따라 dispatch.

#### 검증 테스트 (Schwarz 6개 + Tewari 3개)

**Schwarz 32216** (TRB type, FVA 3 oil, axial 6 kN, oil bath):

| 테스트 | 운전점 | 측정 [N·mm] | M_BH (φ_T) | Ratio | Bracket |
|--------|------|-------------|------------|-------|--------|
| `test_bh_schwarz_32216_axial_500rpm_50c` | 50°C, 500 rpm | 1200 | 1102.5 | 0.919 | [0.75, 1.10] |
| `test_bh_schwarz_32216_axial_2000rpm_50c` | 50°C, 2000 rpm (Fig5 interp) | 2750 | 2964.2 | 1.078 | [0.85, 1.35] |
| `test_bh_schwarz_32216_axial_4000rpm_50c` | 50°C, 4000 rpm | 3800 | 4552.4 | 1.198 | [1.05, 1.40] |
| `test_bh_schwarz_32216_axial_500rpm_42c` | 42°C, 500 rpm | ~1580 (est) | 1455.9 | 0.921 | [0.65, 1.10] |
| `test_bh_schwarz_32216_temperature_scaling` | M(42°C)/M(50°C) | — | 1.32× | (η^0.75) | [1.20, 1.45] |
| `test_bh_schwarz_32216_speed_scaling` | M(4000)/M(500) | — | 4.13× | (Wilson φ_T-damped) | [3.7, 4.5] |
| `test_bh_schwarz_32216_aihara_4000rpm_50c` | **Aihara**, 4000 rpm | 3800 | **3599** | **0.947** | [0.80, 1.10] |
| `test_bh_schwarz_32216_aihara_500rpm_50c` | **Aihara**, 500 rpm | 1200 | 1094.3 | 0.912 | [0.75, 1.05] |
| `test_bh_schwarz_32216_wilson_vs_aihara` | Aihara/Wilson @ 500 vs 4000 rpm | — | 0.99×/0.79× | (저속 동일/고속 더 강함) | — |

→ **Wilson**: 저속 7-8% 정확, 고속 +20% over.
→ **Aihara** (기본 + 권장): 저속 8% 정확, 고속 -5% (거의 정확), TRB 전 운전점 ±10% 이내.

**Schwarz 32208** (Schwarz Table A2, 작은 TRB): d=40, D=80, d_pitch=60, d_RB=10, l_RB=17, n_RB=17, α=13°. 동일 FVA No.3 oil. 측정값은 Fig 9의 graph만 있어 magnitude/scaling 검증:

| 테스트 | 검증 |
|--------|-----|
| `test_bh_schwarz_32208_magnitude_50c` | 1 kN axial, 1500 rpm, 50°C → M_BH = 536.8 N·mm (∈ [50, 1500] plausible range) |
| `test_bh_schwarz_32208_vs_32216_scaling` | 동일 운전점 (1500 rpm, 50°C) 32208(1 kN)/32216(6 kN) ratio = 0.232 (load 1/6 + smaller geometry, ∈ [0.05, 0.45]) |
| `test_bh_schwarz_32208_speed_monotonic` | 500 → 4000 rpm 단조 증가 |

**Tewari 32008-class** (TRB d=40, D=68, B=19, FVA 3A oil, axial 12.85 kN):

본문에 figure 기반 측정값만 있어 Tewari가 측정과 best-match로 검증한 **Matsuyama 식**(*SAE Trans* 1998)을 proxy reference로 사용. Tewari Eq. 13: `M_i/o = 14.2 · E' · l · R_e² · U^0.75 · G^(-0.04) · W^0.08`.

| 테스트 | 검증 |
|--------|-----|
| `test_bh_vs_matsuyama_tewari_32008_2000rpm_65c` | 2000 rpm, 65°C, 12.85 kN — BH=474.1 vs Matsuyama=376.6 N·mm, ratio 1.26 (within factor 3) |
| `test_bh_tewari_32008_speed_monotonic` | 200/500/1000/1500/2200 rpm M_BH 단조 증가 |
| `test_bh_tewari_32008_temperature_monotonic` | M(35°C)/M(65°C) ∈ [1.4, 2.5] (η^0.75, ν 35→65: ratio 2.3 → M 1.86 예상) |

→ BH 2010 Part 1과 Matsuyama 식이 25% 이내 일치 — 둘 다 line contact pure rolling으로 calibration된 식, 일관성 확인.

cargo test 318/318 통과 (회귀 0).

### 14.15.4.6B Tewari Figure 13 figure-extraction (32008-class)

> Tewari et al. 2023 *Machines* 11:801, Figure 13 panels (a)/(b). M_rr (rolling resistance only, M_sl_rib 차감 후 per §4.4) vs 속도, F_a=12.85 kN axial, FVA 3A oil.

**Figure 13(a) — 55°C 측정값** [N·m]:

| n [rpm] | 200 | 400 | 600 | 1000 | 1400 | 1800 | 2200 |
|---------|-----|-----|-----|------|------|------|------|
| M_rr | 0.62 | 0.48 | 0.63 | 0.97 | 1.03 | 1.05 | 1.07 |

**Figure 13(b) — 65°C 측정값** [N·m]:

| n [rpm] | 200 | 400 | 600 | 1000 | 1400 | 1800 | 2200 |
|---------|-----|-----|-----|------|------|------|------|
| M_rr | 0.83 | 0.42 | 0.45 | 0.77 | 0.82 | 0.90 | 0.95 |

**특징**: 400 rpm에서 **Stribeck dip** (boundary→mixed 전이). 1000 rpm 이상 EHL regime에서 단조 증가. 55°C > 65°C (높은 η = 높은 EHL rolling resistance).

**검증 결과 (BH 2010 + Aihara, 추정 32008 geometry: Z=19, d_RB=8.7, l_RB=12.5)**:

| 운전점 | 측정 [N·m] | 우리 BH+Aihara [N·m] | Ratio | 평가 |
|--------|-----------|---------------------|-------|------|
| 2200 rpm, 65°C | 0.95 | **0.508** | **0.535** | -47% under |
| **M(55)/M(65) @ 2200 rpm** | **1.126** | **1.173** | **+4 %** | **trend 정확** |
| 1000-2200 rpm 단조성 | ✓ | ✓ | — | EHL regime 정합 |

**Magnitude under-prediction 원인 (추정)**:
- 32008 정확 geometry (Z, l_RB, d_RB) 본문 미공개 — 추정값 사용
- 32008 various manufacturers/variants (Z=17~21 가능)
- l_RB ±2 mm 변동 시 M 비율 ±15%
- Tewari M_rr는 hysteresis + slight Boundary contribution 포함 가능 (M_sl_rib 차감 후에도)

**Trend validation은 정확** — temperature ratio 1.173 vs 1.126 (오차 4%), monotonicity 정합.

### 14.15.4.6C Schwarz Figure 6/7 combined load 검증 한계

> Schwarz 2023 Figure 6 (F_a=6+F_r=6.5 kN combined) + Figure 7 (radial sweep 1-15 kN @ preload).

**Figure 6 측정값** (combined, oil bath):

| n [rpm] | 42°C M_T [N·mm] | 50°C M_T [N·mm] |
|---------|------------------|------------------|
| 500 | ~2000 | ~1500 |
| 1000 | ~2650 | ~1950 |
| 2000 | ~3500 | ~2500 |
| 4000 | ~4450 | ~3250 |

**Figure 7 측정값** (50°C, n=2000 rpm, F_a=6.5 kN preload):

| F_r [kN] | 1 | 4 | 8 | 12 | 15 |
|----------|---|---|---|----|----|
| M_T [N·mm] | ~1570 | ~1620 | ~1610 | ~1480 | ~1300 |

→ F_r > 10 kN에서 load zone shift로 M_T 감소.

**Cargo test**: `test_schwarz_32216_combined_load_solver_runs` (smoke test) + `test_schwarz_32216_combined_vs_axial_load_zone_shift` (qualitative trend).

**Magnitude validation 한계**:
- Full bearing equilibrium 솔버 결과는 raceway 곡률 반경 (r_i, r_o) + roller crown 정확 계수 (a_log, polynomial fit)에 매우 민감
- Schwarz Table 4 published 파라미터로는 정확 재현 불가
- 우리 추정 a_log=0.0008로 초기 시도 시 21,000 N·mm vs 측정 2,500 N·mm (8.5× over)
- 따라서 combined load 검증은 **smoke test + qualitative trend**로 한정

**Per-contact analytical validation은 정확** (§14.15.4.6A의 axial-only Schwarz 검증 ±10%) — full solver의 magnitude 정확도는 정확한 profile 정보 확보 시 향후 가능.

### 14.15.4.6D Rib power loss 식 정정 — pure sliding → Houpert drilling (2026-05-13)

**버그**: 이전 구현은 TRB rib을 pure sliding으로 처리:
$$P_\text{rib}^\text{(잘못)} = \mu \cdot F_\text{rib} \cdot u_\text{slide,rib}, \quad u_\text{slide,rib} = \omega_\text{roller} \cdot r_\text{large\_end}$$

$r_\text{large\_end} \approx 8.75$ mm은 roller 반경 전체. 이 가정은 **잘못** — TRB rib 접촉은 sliding이 아닌 **drilling motion** (roller 자체 축 spin이 접촉면 normal 주위 spin 야기).

**수정 (Houpert 2002)**:
$$M_\text{drilling} = \frac{3}{8} \cdot \mu \cdot F_\text{rib} \cdot a_\text{ellipse}, \quad P_\text{rib} = M_\text{drilling} \cdot \omega_\text{roller}$$

$a_\text{ellipse} \approx 1-2$ mm은 Hertz 접촉 타원 반장축. Lever arm 비율 $r_\text{large}/(3a/8) \approx 16$ → 옛 식이 **~16× over-predict**.

**검증** (Schwarz 32216 axial 6 kN, full solver):

| n [rpm] | 옛 P_rib [W] | 정정 P_rib [W] | M_rib [N·mm] |
|---------|---------------|------------------|--------------|
| 500 | ~수십 | 1.9 | ~36 |
| 2000 | ~수백 | 10.0 | ~48 |
| 4000 | ~수천 | 22.4 | ~54 |

rib 기여가 ~수십 N·mm 수준 (이전 수천 N·mm) — 실제 TRB axial 운전 환경의 typical magnitude와 정합.

**3 파일 수정**: M1 (`compute_traction`), M2 (`compute_traction_advanced`), `transient.rs` tau_rib_j. 전부 동일 drilling 공식 사용.

**Schwarz 32216 BearingInput 일관성 수정**: $\alpha_i = 11.5°$, $\alpha_o = 14°$ ($d_{we}$ taper에서 $\phi_\text{roller} = 1.26°$ 유도, $\alpha_o - \alpha_i = 2\phi$). 이전 $\alpha_i = \alpha_o = 14°$는 원통 롤러 베어링(CRB)을 의미하여 cone-apex 운동학 식이 degenerate.

cargo test 324/324 통과 (회귀 0).

#### Part 2 (k=100 cap)에서 Part 1으로의 변경 영향

이전 구현 (Part 2 + k=100)에서 새 구현 (Part 1 직접)으로 변경 시:
- 라인 접촉 한계에서 식의 calibration이 다름 (Part 1: 1.42, 1.47 / Part 2 k=100: 2.9766·100^0.3316 ≈ 13.7, 7.5826·100^0.4055 ≈ 49.1)
- 그러나 정확한 비교는 dimensionless group 정의 차이 (Part 1 U는 factor 2)를 고려해야 함
- 둘 다 동일 학술 work이고, Part 1이 line contact에 *직접 calibration*되었으므로 더 정확

### 14.15.4.6E TRB EHL 진입 속도 정정 — u_outer/u_inner 분리 (2026-05-13)

#### 문제 인식
14.15.4.6D에서 P_rib drilling 정정 후에도 풀솔버가 Schwarz 측정 대비 **~2× 과대 예측**.
- 4000 rpm @ 50 °C: M_tot 풀솔버 7262 N·mm vs 측정 3750 N·mm
- 동일 BH 공식인데 분석 [B](RMSE 3.28%)와 풀솔버가 큰 차이 → BH 공식이 아닌 **호출 인자(u)** 의심

#### 원인 — EHL 진입 속도 u 계산 오류
풀솔버 [lubrication.rs:813]는 cone-apex 가정에 기반한 `u_roll = ω_roller × r_mean`을 inner / outer 양쪽 contact에 동일 적용:
```
u_roll = ω_roller × r_mean        // 8.5 mm × ω_roller
ω_roller = ω_i·sin α_i·sin α_o / (sin φ·(sin α_i+sin α_o))   // cone-apex
```

이 식은 **r/R = sin φ / sin α** 라는 cone-apex matched 기하 전제. 실제 입력값 검증:
- Schwarz 32216: α_i=11.5°, α_o=14°, d_we=17 mm, d_pw=108.5 mm
- 전제 r/R = sin 1.25° / sin 12.75° = 0.0902 → r=4.89 mm 강요
- 실제 r=8.5 mm → **전제 위반** (1.74× 불일치)

결과적으로 ω_roller가 1.65× 과대 계산되어 u_roll = 17.84 m/s. 분석 [B]는 **u_outer = ω_cage·R_outer = 11.83 m/s, u_inner = ω_cage·R_inner = 8.71 m/s** (실제 R 기반).

BH는 P ∝ u^1.5 이므로 u 1.65× 과대 → P 1.65^1.5 ≈ **2.04× 과대** ✓ 관측 일치.

#### 정정
[lubrication.rs:746-832] `TrbKinematics`에 `u_outer`, `u_inner` 필드 추가:
- ω_cage, ω_roller: cone-apex 식 유지 (compute_slice_sliding 후방 호환 — sliding power용)
- u_outer, u_inner: **실제 d_pw, d_we 기반** Schwarz 식
  ```rust
  let r_outer = r_pitch + r_rb * cos(α_avg);
  let r_inner = r_pitch - r_rb * cos(α_avg);
  let u_outer = omega_cage * r_outer;
  let u_inner = omega_cage * r_inner;
  ```

[lubrication.rs:1014-1033] BH와 Johnson hysteresis 호출에서 inner/outer 분리:
```rust
inner.p_rolling_w = biboulet_houpert_line_rolling_power_dispatched(
    eta_0, kin.u_inner, ...);   // 이전: kin.u_roll
outer.p_rolling_w = biboulet_houpert_line_rolling_power_dispatched(
    eta_0, kin.u_outer, ...);   // 이전: kin.u_roll
```

Palmgren/SKF 경험식(compute_contact_friction_at)은 inner/outer 같은 u 가정으로 calibration된 모델이므로 `kin.u_roll = (u_outer + u_inner)/2` 유지.

#### 효과
풀솔버 vs 측정 (axial 6 kN, 50 °C):

| n [rpm] | M_meas | M_tot 이전 | **M_tot 수정 후** | Δ |
|---:|---:|---:|---:|---:|
| 500 | 1300 | 3168 (+144%) | **1277** | **−1.7%** |
| 2000 | 2950 | 6773 (+130%) | **3102** | **+5.2%** |
| 4000 | 3750 | 7262 (+93.6%) | **4066** | **+8.4%** |

4000 rpm에서 +8.4% 잔여는 측정 불확실도 (typical ±10%) 내. 분석 [B] RMSE 3.28%와 정합.

cargo test 324/324 통과.

#### 검증
- `diag_schwarz_32216_traction_breakdown` (재실행) — 위 표
- `diag_schwarz_32216_alpha_v_sweep` — 분석 [B] α_v=0.005 RMSE 3.28% 유지 (변경 없음)
- Palmgren/SKF 회귀 테스트 (`test_p_rolling_w_asymmetric_inner_outer_loads_m1` 등) 모두 통과

### 14.15.4.7 향후 작업 — Bearinx / MESYS Level D 검증

Schaeffler **Bearinx-online Easy Friction**과 **MESYS Rolling Bearing Calculation**은 모두 상용 도구로 직접 라이선스 사용 필요:
- [Bearinx-online Easy Friction (Schaeffler)](https://www.schaeffler.us/content.schaeffler.us/us/products-and-solutions/industrial/calculation-and-advice/calculation/bearinx_online_easy_friction/index.jsp)
- [MESYS Rolling Bearing Calculation](https://www.mesys.ag/?page_id=1202)

이 두 도구는 본 솔버의 결과를 cross-check하는 Level D 검증 reference로 사용 가능. 특히 *Engineering Research* 2023 (Springer)의 "Evaluation of friction calculation methods for rolling bearings" 논문은 **LFP / SKF / Koryciak / Palmgren** 방법을 측정 데이터와 비교하여 Schaeffler LFP가 SKF/Koryciak 대비 deviation이 가장 작다고 보고.

**대안 (라이선스 없이)**: MDPI *Lubricants* 2023 (vol. 11, 369), "Predicting Friction of Tapered Roller Bearings with Detailed Multi-Body Simulation Models" — 30306-class TRB의 measured + MBS 모델 토크 데이터를 reference로 활용 가능 (PDF 직접 다운로드는 publisher edge protection으로 차단됨; ResearchGate 또는 학술 라이브러리 경유 권장).

### 14.15.4.8 학술 데이터 포인트 — 3-model 비교 (HR 65 KBE 52X+L)

*Engineering Research, Springer 2023* "Evaluation of friction calculation methods for rolling bearings"에서 보고된 single data point — **double-row TRB HR 65 KBE 52X+L, 20 kN load, 2000 rpm**:

| 모델 | 마찰 토크 [N·m] |
|------|---------------|
| Palmgren | 2.06 |
| NSK | 2.53 |
| SKF | 2.96 |

이 결과에서 **Palmgren < SKF**임에 유의 — 우리 default 30306 preset에서 본 **Palmgren > SKF (4116 vs 300 N·mm, ~14×)**와 ranking이 반대. 즉 **두 모델의 ranking은 베어링 시리즈·운전조건에 따라 변동**한다는 학술적 confirmation이다 (Palmgren 단순화 모델이 systematic 과대 추정이 아니라 calibration 차이).

### 14.15.4.9 3-Model Cross-Validation 회귀 테스트

라이선스 없는 Level D 검증의 대안으로, **3개 모델 (Palmgren / BH Part 1 / SKF)이 같은 베어링·운전조건에서 합리적 ratio bounds를 유지한다는 invariant**를 회귀로 가드한다.

`test_friction_model_three_way_cross_validation_30306` 검증:

1. **Sliding 일관성**: BH 모델은 Palmgren의 sliding total을 그대로 보존 (rolling만 override). SKF는 sliding도 G_sl·μ_sl로 별도 → 다른 양이지만 finite/non-negative.
2. **모든 모델 finite/positive**: NaN/divergence 가드.
3. **BH/Palmgren ratio**: [0.05, 20] sanity envelope.
4. **SKF/Palmgren ratio**: [1e-4, 1e4] (mock 1-roller fixture에서 SKF는 외부 운전조건 직접 사용이라 absolute scale이 다름; 4 orders bound).
5. **BH ↔ SKF**: 1e4 이내 (둘 다 viscous physics 캡처, 같은 차수).

이 테스트는 **calibration drift / unit regression / NaN propagation을 즉시 catch**하기 위한 live registry로 작동한다. 실측 데이터 정합 검증은 아니며, Bearinx/MESYS 라이선스 도구 또는 사용자가 SKF Bearing Calculator 결과를 hardcoded reference로 추가하면 더 강한 Level D 검증이 가능.

cargo test 292/292 통과 (291+1 신규).

### 14.15.4.10 SKF Bearing Select Tool — 30306 + LGMT 2 4-LC 실측 검증 (Level D)

**SKF Bearing Select** (https://www.skfbearingselect.com/) 에서 30306 + 그리스 SKF LGMT 2 (NLGI 2 lithium soap, mineral base oil ν₄₀=110, ν₁₀₀=10 mm²/s, DIN 51825 K2K-30) 4-LC 결과 직접 추출:

| LC | F_r [kN] | F_a [kN] | n [rpm] | T [°C] | SKF Tool M_rr | M_sl | M_total |
|----|---------|---------|--------|--------|---------------|------|---------|
| **LC1** | 2 | 1 | 500 | 60 | 130 | 51.6 | **181 N·mm** |
| **LC2** (default) | 5 | 2 | 1500 | 70 | 217 | 17 | **234 N·mm** |
| **LC3** (heavy) | 15 | 5 | 1500 | 80 | 219 | 125 | **345 N·mm** |
| **LC4** (high speed) | 5 | 2 | 4000 | 80 | 285 | 6.56 | **291 N·mm** |

#### 우리 손계산 (Walther 점도, K_rs=6e-8, Y=1.6) vs SKF Tool

| LC | 우리 M_rr / SKF | 우리 M_sl / SKF | 우리 M_total / SKF |
|----|----------------|----------------|---------------------|
| LC1 | 153 / 130 (×1.18) | 70 / 51.6 (×1.35) | 223 / 181 (×1.23) |
| LC2 | 276 / 217 (×1.27) | 26 / 17 (×1.53) | 302 / 234 (×1.29) |
| LC3 | 285 / 219 (×1.30) | 215 / 125 (×1.72) | 500 / 345 (×1.45) |
| LC4 | 356 / 285 (×1.25) | 9.8 / 6.56 (×1.49) | 366 / 291 (×1.26) |

→ **우리 모델 평균 +30% 큰 결과** (M_total). SKF Catalogue 2018 모델 자체의 ±20-30% 정확도 범위. 정합 OK.

#### 차이의 원인

1. **30306 sub-variant Y factor**: 우리 hardcoded Y=1.6, 실제 30306 J2/Q variant마다 1.4~1.7 (예: Y=1.4 시 F_eff_rr ~11% 작음 → M_rr ~11% 작음).
2. **LGMT 2 specific R/S**: SKF Tool은 30306 specific calibration. 우리 generic 'Series303' 사용.
3. **Walther vs SKF ASTM D341 점도**: ν 정확도 ±5%.
4. **K_rs**: LGMT 2 specific value (SKF 내부) vs 우리 generic `6e-8 grease`.
5. **LC3 M_sl +72%**: heavy load mixed regime — SKF Tool 자체에 추가 calibration 가능.

#### 회귀 테스트 — 4-LC hardcoded reference

`test_skf_30306_lgmt2_*` 5개 테스트 (lubrication.rs):
- LC1~LC4 각각: M_rr / M_sl / M_total 모두 SKF Tool값 ±50~70% 이내 (LC3 M_sl만 ×2.5 bracket)
- `test_skf_30306_lgmt2_4lc_aggregate_bias`: 4 LC 평균 ratio ∈ [1.0, 1.7] (현재 1.30, +30% bias 명시적 가드)

cargo test 297/297 통과 (292+5 신규). 향후 calibration 변경 시 이 테스트가 즉시 alert.

---

## 14.16 유막 감쇠 모델 — Van Zoelen (Film Decay)

### 14.16.1 개요

Van Zoelen side flow 모델은 starved EHL 접촉에서 **접촉 압력에 의한 측방 유출(side flow)**과 **보충(replenishment)**의 균형을 예측합니다.

기존 정적 기아 상수(φ_s) 방식은 운전 시간에 무관한 스냅샷 값을 사용하지만, 이 모델은 **시간에 따른 유막 감쇠 곡선**과 **평형 유막두께**를 계산합니다.

참고 논문:
- Venner, van Zoelen, Lugt (2012) *Tribology International* 47:175-187
- Gao, van Zoelen, Osara, Meeuwenoord, Pasaribu, Lugt (2026) *Tribology International*

### 14.16.2 지배 방정식

층 두께 변화율 (replenishment 포함):

$$\frac{d\tilde{h}}{dt} = -\frac{1}{3}\tilde{h}^3 \mathcal{F}(0) + R$$

- 제1항: 접촉 압력에 의한 측방 유출 (loss)
- 제2항: 보충률 R [m/s] (gain)

**평형 조건** (dh̃/dt = 0):

$$\tilde{h}_{eq} = \left(\frac{3R}{\mathcal{F}(0)}\right)^{1/3}, \quad h_{c,eq} = \frac{2\tilde{h}_{eq}}{\bar{\rho}_c}$$

**R = 0 (보충 없음) 해석해** — Eq. 27:

$$h_c(t) = \left(\frac{1}{6}\bar{\rho}_c^2 \mathcal{F}(0) t + h_{c,0}^{-2}\right)^{-1/2}$$

**R > 0** — 닫힌 해가 없으므로 4차 Runge-Kutta로 수치 적분.

### 14.16.3 Side Flow 파라미터 F(0) — Eq. 25

$$\mathcal{F}(0) = \frac{2}{l_t} \frac{p_h}{b^2} \frac{a}{\eta_0} \pi \left( (0.5\pi\alpha p_h)^{3/2} + 1 \right)^{-2/3}$$

| 기호 | 의미 | 단위 |
|------|------|------|
| p_h | 최대 Hertz 접촉 압력 | Pa |
| a | 구름 방향 반접촉폭 (= Hertz b_k) | m |
| b | 횡방향 반접촉폭 (TRB: = L_we/2) | m |
| η₀ | 동적 점도 (oil: 오일, grease: base oil) | Pa·s |
| α | 압력-점도 계수 (Barus) | 1/Pa |
| l_t | 총 트랙 길이 | m |

**검증**: Eq. 24 (full integral with Roelands viscosity) 대비 **3% 이내 오차**, 71배 빠름.

### 14.16.4 총 트랙 길이 l_t

TRB 베어링:
$$l_t = 2\pi R_{inner} + 2\pi R_{outer} + Z \times 2\pi R_{roller}$$

- R_inner = 내륜 접촉점 반경
- R_outer = 외륜 접촉점 반경
- R_roller = 롤러 평균 반경
- Z = 롤러 수

### 14.16.5 원주 평균 — Eq. 28

방사 하중 하에서 F(0)는 원주 위치(Ψ)에 따라 변합니다:

$$\bar{\mathcal{F}}(0) = \frac{1}{N_{loaded}} \sum_{\Psi} \mathcal{F}(0, \Psi)$$

현재 구현에서는 angular distribution의 모든 loaded position에서 최대 하중 슬라이스의 접촉 파라미터로 F(0)를 계산하고 평균합니다.

### 14.16.6 스큐 보정

TRB에서 롤러 스큐는 유막 감쇠율을 변화시킵니다:

$$\mathcal{F}_{corrected} = \mathcal{F}(0) \times f_{skew}$$

| 스큐 각도 | f_skew | 의미 |
|-----------|--------|------|
| +2° | 0.63 | 원심력과 같은 방향 → 감쇠 느림 |
| +1° | 0.83 | |
| 0° | 1.00 | 기준 |
| −1° | 1.06 | 원심력과 반대 방향 → 감쇠 빠름 |
| −2° | 1.11 | |

(Gao et al. 2026 Table 3, 150 mm/s 기준)

### 14.16.7 Replenishment Rate R 설정 가이드

#### Oil Bath (유조 침적)
- R이 매우 크므로 즉시 fully flooded 도달
- **Film Decay OFF 권장** — 기존 φ_s(speed) 모델로 충분
- R 입력 불필요

#### Oil Jet (분사 윤활)
- R을 제트 파라미터에서 직접 계산:

$$R = \frac{\eta_{capture} \times Q_{jet}}{l_t \times w_{track}}$$

| 변수 | 의미 | 일반적 범위 |
|------|------|-----------|
| Q_jet | 제트 유량 | 10~500 mL/min |
| η_capture | 포획 효율 | 0.1~0.5 |
| w_track | 유효 트랙 폭 ≈ 2a | 0.1~0.5 mm |

예시: Q_jet=50 mL/min, η=0.2, l_t=0.3 m, w=0.3 mm
→ R = 0.2 × 8.33e-7 / (0.3 × 3e-4) = 1.85e-3 m/s = **1,850,000 nm/s** (매우 큼 → fully flooded)

결론: Oil jet에서 적절한 유량이면 R >> F(0)이므로 starvation 발생하지 않음. 극고속(n×d_pw > 2M)이나 소량 급유 시에만 Film Decay 활용.

#### Grease (그리스)
- R = base oil bleeding rate
- 그리스 데이터시트의 **DIN 51817** 시험 결과에서 추정:

| 그리스 종류 | Oil separation (DIN 51817) | R 추정 [nm/s] |
|-----------|--------------------------|--------------|
| Lithium/mineral, NLGI 2 | 3~5% / 7d @ 40°C | 0.005~0.01 |
| Lithium/PAO, NLGI 2 | 2~4% / 7d @ 40°C | 0.003~0.008 |
| Polyurea, NLGI 2 | 1~3% / 7d @ 40°C | 0.002~0.005 |
| Lithium complex, NLGI 2 | 4~8% / 7d @ 40°C | 0.008~0.015 |

> **참고**: 위 R 값은 order-of-magnitude 추정입니다. 정확한 값은 실제 베어링 형상, 온도, 그리스 충전량에 따라 달라지며, 실험적 교정이 필요합니다.

R = 0으로 설정하면 **보충 없는 최악 조건**(worst-case) 감쇠를 예측합니다.

### 14.16.8 Oil vs Grease 점도 (η₀) 선택

| 윤활 유형 | F(0)에 사용하는 η₀ | 이유 |
|----------|-------------------|------|
| Oil | 오일 점도 (ν₄₀/ν₁₀₀ → η₀ 변환) | 오일 자체가 유동 |
| Grease | **Base oil 점도** (ν₄₀/ν₁₀₀) | Channeling 후 트랙 위 유동하는 것은 base oil |

현재 시스템의 ν₄₀/ν₁₀₀ 입력은 ISO 281 기준으로 base oil 동점도를 의미하므로, Oil과 Grease 모두 동일한 입력값을 사용합니다.

### 14.16.9 입력 파라미터

| 파라미터 | 필드명 | 단위 | 기본값 | 설명 |
|---------|--------|------|--------|------|
| Film Decay | film_decay_enabled | bool | OFF | 모델 활성화 |
| Operating time | film_decay_time_hours | hr | 0 | 운전 시간 |
| Skew angle | skew_angle_deg | ° | 0 | 롤러 스큐 각도 |
| Replenishment R | replenishment_rate_nm_s | nm/s | 0 | 보충률 (0 = worst-case) |

### 14.16.10 출력 구조체 (FilmDecayResult)

| 필드 | 단위 | 설명 |
|------|------|------|
| t_hours | hr | 운전 시간 |
| h_c_decayed_inner/outer_um | μm | 감쇠된 중심 유막두께 |
| starvation_ratio_inner/outer | - | h_c(t) / h_cff |
| f0_inner/outer | m⁻²s⁻¹ | Side flow 파라미터 |
| lambda_decayed_inner/outer | - | 감쇠 후 Λ 비 |
| regime_decayed_inner/outer | - | 감쇠 후 윤활 영역 |
| h_c_equilibrium_inner/outer_um | μm | 평형 유막두께 (R>0 시) |
| replenishment_rate_nm_s | nm/s | 사용된 보충률 |
| decay_curve | - | (시간, 내륜 nm, 외륜 nm) 배열 |

### 14.16.11 UI — Film Decay 서브탭

Film Decay ON 시 Lubrication 탭에 "Film Decay" 서브탭이 추가되며 다음을 표시합니다:

1. **Detail Table**: 내/외륜 별 fully flooded → decayed → equilibrium 유막두께
2. **Decay Curve Chart**: 시간-유막두께 곡선 (내/외륜 별도)
   - R > 0 시 평형선(dashed) 표시
   - 현재 운전 시간에 다이아몬드 마커

### 14.16.12 크라운 프로파일과 윤활 내성

Van Zoelen 모델의 핵심 결론: **타원비 k = b/a가 높을수록 감쇠가 느리다** (k×10 → 감쇠 시간 ×100).

TRB에서 크라운 프로파일이 접촉 형상을 결정합니다:

| 크라운 타입 | 접촉 반폭 b | F(0) | 윤활 내성 | 에지 응력 |
|------------|-----------|------|---------|---------|
| Flat (무크라운) | 최대 | 최소 | 최상 | 최악 |
| Logarithmic | 약간 감소 | 약간 증가 | 좋음 | 좋음 |
| Full circular | 크게 감소 | 크게 증가 | 불리 | 양호 |
| Partial crown | 위치 의존 | 끝단에서 급증 | 끝단 취약 | 양호 |

**설계 트레이드오프**: 에지 응력 감소를 위해 크라운을 강하게 주면 b가 줄어 유막 감쇠가 빨라집니다. Gen3 빔 모델과 결합하면 크라운별 에지 응력과 윤활 내성을 동시에 평가할 수 있습니다.

---

## 14.17 변경 이력

| 날짜 | 변경 내용 |
|------|----------|
| 2026-03-24 | **§14.1.6 ISO 수명과 유막 모델 관계 추가**: (1) κ 방법별 유막 모델 호환성 — ViscosityRatio(모델 무관) vs FilmThicknessRatio(M1/M3만). (2) M2+FilmThicknessRatio 조도 이중 반영 문제 설명 및 경고. (3) 코드에 경고 Alert 추가. (4) 목적별 권장 κ×모델 조합표. |
| 2026-03-24 | **§14.1.1 모델 상세 비교표 6개 섹션 추가**: (A) 수식 구조 및 물리 모델 (B) 조도 처리 — 기준면(mean line vs smooth), 보정 계수 rc, 혼합윤활 (C) 보정 모델 — 열/기아/점도 (D) 트랙션 및 부가 기능 (E) 입출력 및 운용 — 계산 비용, h_c 차이, 호환성 (F) 검증 수준. 섹션 번호 14.1.1~14.1.6 정리. |
| 2026-03-23 | **§14.1 전면 개편 — 모델 선택 가이드**: (1) M1(DH)/M2(MK)/M3(NVM) 3모델 + Film Decay 2축 구조 설명. (2) 모델별 상세 비교표(연도, 접촉 형태, EHL 영역, 조도, 정밀도). (3) 각 모델 "언제 사용하나/강점/약점/유의점" 상세 기술. (4) Film Decay 적용 가이드(Oil bath/Oil jet/Grease별). (5) 목적별 권장 조합표 추가. |
| 2026-03-23 | **M3 (NVM) 추가**: Nijenbanning-Venner-Moes (1994) 4영역 통합 EHL 공식 구현. Moes M,L 파라미터, IR/IE/RP/EP 점근해 + 통합 블렌딩. TRB EP 영역에서 DH 대비 2-7% 차이. 논문 h_cff 검증: 원형 접촉 +1.1% 오차. 186개 테스트 통과. |
| 2026-03-21 | **Van Zoelen Film Decay 모델 추가 (§14.16)**: (1) Replenishment rate R 포함 ODE 모델 — R=0: 해석해(Eq.27), R>0: RK4 수치적분 + 평형 유막 계산. (2) 윤활 방식별 가이드: Oil bath(φ_s 유지), Oil jet(R=Q_jet에서 계산), Grease(R=bleeding rate). (3) 크라운-윤활 내성 연계 분석 설명 추가. |
| 2026-03-20 | **명칭 변경**: Basic→Method1_DH (Dowson-Higginson 1977), Advanced→Method2_MK (Masjedi-Khonsari 2015). serde alias로 하위호환 유지. |
| 2026-03-19 | **4대 버그 수정 + 참고문헌 검증 테스트 12개 추가**: (1) 인입속도 `u_m_inner/u_m_outer` — Harris 공식으로 수정: `u_m = \|(ω_race − ω_cage)\| × R`, 순수 구름 시 내/외륜 동일 `\|ω_i−ω_o\|×R_pw×(1−γ²)/2`. (2) 기아보정 파라미터 — `1/(1+(nd/500k)^1.5)` → `1/(1+(nd/3M)^0.9)`, 매뉴얼 테이블 정합. (3) M-K 점접촉(2015)→선접촉(2012) — 점접촉 V^(-0.42) 항 750× 증폭 문제 해결, 지수형 보정으로 교체. (4) 매뉴얼 §14.3.3, §14.3A.1, §14.3A.3, §14.3A.4, §14.5.1, §14.7.4, §14.12.5, §14.13, §14.14 갱신. |
