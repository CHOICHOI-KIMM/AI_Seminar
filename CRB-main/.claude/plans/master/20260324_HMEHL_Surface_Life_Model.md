# HMEHL + ML Surrogate 기반 표면 피로 수명 모델

## TL;DR

### Quick Summary
Homogenized Mixed EHL (HMEHL) 솔버를 자체 개발하고, ML surrogate로 가속화하여
**GBLM 수준의 표면/표면하 분리 수명 예측**을 TRB 시스템에 통합한다.

표면하 수명은 기존 ISO 281을, 표면 수명은 HMEHL 기반 Ioannides-Harris 표면 적분으로 계산하며,
최종 수명은 weakest link 원리로 결합한다:

```
L₁₀ = [ 1/L₁₀,sub^e + 1/L₁₀,surf^e ]^(-1/e)
```

### Deliverables
1. **Rust HMEHL 솔버** — Reynolds PDE(FVM) + CG-FFT 탄성 + 조도 homogenization + 비뉴턴. rustfft/nalgebra/sprs/rayon 활용
2. **솔버 검증** — Coulon (압력), Kaneta (유막) 실험 데이터 대비 비교
3. **학습 데이터** — 20,000+ HMEHL 시뮬레이션 (LHS 샘플링, rayon 병렬)
4. **ML Surrogate** — PyTorch 학습 → ONNX 변환 → Rust ort 크레이트로 추론
5. **Ioannides-Harris 표면 수명** — 표면 응력장 → 면적 적분 → L₁₀,surf (Rust)
6. **GBLM 결합 수명** — L₁₀,sub(ISO 281) + L₁₀,surf → L₁₀,GBLM
7. **TRB 통합** — HMEHL 직접 호출(단일 접촉) 또는 ONNX surrogate(대량 반복)
8. **Surface Risk 진단** — SR = Rs/(Rs+Rss), 개선 방향 제시

### Estimated Effort
- Phase 1 (Rust HMEHL 솔버): 높음 (핵심 난이도)
- Phase 2 (솔버 검증): 중간
- Phase 3 (학습 데이터 생성): 낮음 (Rust+rayon으로 빠름)
- Phase 4 (ML surrogate): 중간 (PyTorch 학습 → ONNX 변환)
- Phase 5 (Ioannides-Harris 수명): 중간
- Phase 6 (TRB 통합): 낮음 (이미 Rust이므로 직접 통합)

---

## 아키텍처 개요

```
Phase 1-2: HMEHL Solver (Rust)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  입력: {F_N, u₁, u₂, R_eq, η₀, α, ρ₀, Rq, R_cl, H_v, E', L_contact}
         ↓
  ┌──────────────────────────────────────┐
  │  Homogenized Reynolds PDE (FVM)      │
  │  + JFO 캐비테이션                      │
  │  + Ree-Eyring 비뉴턴 점도             │
  │  + Roelands/DH 압력-점도/밀도          │
  ├──────────────────────────────────────┤
  │  CG-FFT 탄성 변형 (Polonsky-Keer)    │
  │  + Macro: Hertz 변형                  │
  │  + Micro: 조도 돌기 탄성 접촉          │
  ├──────────────────────────────────────┤
  │  Homogenization (Hansen 2011)         │
  │  + 조도 → 유효 유동 계수 A, b, C, d   │
  │  + 유효 평균 유막 h_m                  │
  │  + 평균 돌기 접촉 압력 p_con           │
  ├──────────────────────────────────────┤
  │  PID 힘 평형 + 수렴 반복               │
  └──────────────────────────────────────┘
         ↓
  출력: {p_tot(x), h(x), τ_tot(x), μ, P_loss}


Phase 3-4: ML Surrogate
━━━━━━━━━━━━━━━━━━━━━━━

  HMEHL × 20,000 (LHS) → Training Data
         ↓
  ┌──────────────────────────────────────┐
  │  Multi-modal Deep Learning            │
  │  ├ Modal 1: 조도 (Rq, R_cl, H_v, E') │
  │  ├ Modal 2: 기하 (R_eq, L_contact)    │
  │  ├ Modal 3: 운전 (F_N, u₁, u₂)       │
  │  └ Modal 4: 윤활 (η₀, α, ρ₀)         │
  │  → 중간 융합 → 출력                    │
  └──────────────────────────────────────┘
         ↓
  출력: {μ, P_loss, σ_surf_max, p_asp_mean}
  R² > 0.99 목표


Phase 5: Ioannides-Harris Surface Life
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Surrogate 출력 (σ_surf, p_asp) → 표면 응력장 재구성
         ↓
  ┌──────────────────────────────────────┐
  │  Ioannides-Harris 면적 적분           │
  │                                       │
  │  ln(1/S_surf) = B̄ · N^e ·            │
  │    ∫_A ⟨σ_s^c - σ_u,s⟩ dA           │
  │                                       │
  │  → L₁₀,surf                          │
  └──────────────────────────────────────┘


Phase 6: GBLM Integration
━━━━━━━━━━━━━━━━━━━━━━━━━

  L₁₀,sub (ISO 281, 기존)  +  L₁₀,surf (I-H, 신규)
         ↓
  L₁₀,GBLM = [ 1/L_sub^e + 1/L_surf^e ]^(-1/e)
         ↓
  Surface Risk: SR = Rs / (Rs + Rss)
```

---

## 현재 진행 상태 (2026-03-26)

### Phase 1 상태: 부분 완료

| 구성 요소 | 상태 | 비고 |
|----------|------|------|
| Reynolds PDE 이산화 | ✅ 완료 | FVM + Distributive relaxation |
| CG-FFT 탄성 변형 | ✅ 완료 | Venner 비차원 커널 K(X) = -(2/π)ln\|X\| |
| Roelands/DH/Ree-Eyring | ✅ 완료 | 비차원화 포함 |
| Patir-Cheng 조도 flow factor | ✅ 완료 | φ_x(Λ) |
| Clarke 하중분담 | ✅ 완료 | ξ=1-erf(Λ) |
| FAS Multigrid V-cycle | ✅ 구현 완료 | FAS defect correction 포함 |
| FMG (Full Multigrid) | ✅ 구현 완료 | coarsest → finest 전파 |
| Distributive relaxation | ✅ 구현 | K_self elastic Jacobian 포함 |
| **Smooth 케이스 수렴** | ✅ 달성 | steel-glass, converged=true |
| **고하중 수렴 (TRB)** | ✅ 달성 | Line relaxation (Thomas alg.), p_max/p_h=3.03 |
| **Grubin 해석적 Fallback** | ✅ 작동 | p_max/p_h=0.98, h_min/h_c=0.75 |
| HMEHL 탭 UI | ✅ 완료 | [Run HMEHL] + Plotly 차트 |
| Tauri command 연결 | ✅ 완료 | 실제 Solve 결과 하중 전달 |

### 핵심 수정 사항 (2026-03-26 세션)

1. **Distributive relaxation**: GS diagonal에 elastic self-influence K_self 추가
   - `d_total = a_c + Λ·ρ·K_self/dX + 1.5·a_c·K_self/H` (Venner Ch.5)
   - a_c → 0 (고점도)에서도 유한한 diagonal 유지
2. **비차원 탄성 커널**: `K(X) = -(2/π)·ln|X|` (Venner 1991)
   - 차원 커널의 ln(b) 상수 오프셋 문제 해결
   - Johnson (1985) combined elastic modulus: `-4/(πE')` 계수 수정
3. **FMG**: coarsest grid에서 해 결정 → prolongation → finer grid
   - 발산 시 analytical fallback 자동 적용
4. **FAS V-cycle**: proper defect correction τ = L_coarse(ÎP) - Î(L_fine(P))
5. **h0 PID**: 음수 H0 허용 (Venner 비차원계에서 물리적으로 정당)

### 핵심 수정 사항 (2026-03-26 세션 2)

6. **Line relaxation** — Thomas algorithm tridiagonal Newton solve 구현
   - Point-wise GS 대비 극적 개선: TRB **수렴 달성**
   - Newton Jacobian = Poiseuille stencil + elastic K_self diagonal
   - Thomas O(N) 풀이로 전체 접촉 영역 동시 해
7. **dp 클램핑** — `dp_limit = 0.5·P + 1.0` (상대적) → 폭발 방지
8. **omega 스케줄** — 0.4→0.6 점진 증가, 0.8 이상은 불안정

### 다음 세션 과제

1. **극고하중 수렴** (f_n=8500N, p_h=2081MPa) — 아직 폴백 의존
   - FMG 중간 레벨 안정화 필요
   - ILU preconditioner 또는 load stepping 검토
2. **Phase 2 검증** — Coulon/Kaneta 실험 대비 압력/유막 비교
3. **Phase 3-6**: ML surrogate + 수명 모델 (솔버 안정 후)

---

## Phase 1: Rust HMEHL 솔버 개발

### 1.1 Reynolds PDE 솔버

Homogenized Reynolds 방정식 (SNU Eq.3.11):

$$0 = \nabla \cdot \left( \frac{\rho_l h_m}{12\eta} A \nabla p_0 - \rho_l h_m u_m \vec{b} (1-\theta) \right)$$

- **이산화**: Finite Volume Method (FVM) — 질량 보존 보장
- **캐비테이션**: JFO 모델 (p₀θ = 0, p₀ ≥ 0, θ ≥ 0)
- **비뉴턴**: Ree-Eyring 유효 점도 (SNU Eq.3.14)
- **압력-점도**: Roelands (Eq.3.18)
- **압력-밀도**: Dowson-Higginson (Eq.3.19)
- **솔버**: FBNS (Fischer-Burmeister Newton-Schur) — Hansen et al. [169-171]

Rust 크레이트:
- `rustfft` — FFT (탄성 변형 convolution)
- `nalgebra` — 밀집 행렬/벡터 (이미 프로젝트에 있음)
- `sprs` — 희소 행렬 (이미 프로젝트에 있음)
- `rayon` — 병렬 (학습 데이터 생성, 이미 프로젝트에 있음)
- `rand` + `statrs` — LHS 샘플링

구현 구조:
```rust
pub struct HMEHLSolver {
    nx: usize,          // 격자 수 (256 typical)
    grid: Vec<f64>,     // FVM 격자
    kernel: Vec<f64>,   // FFT 커널 (사전 계산)
}

impl HMEHLSolver {
    pub fn solve(&self, params: &ContactParams) -> HMEHLResult {
        // 1. 초기 추정: Hertz 압력/변형
        // 2. Homogenization factors 계산 (조도 → A, b, C, d)
        // 3. Reynolds PDE 풀이 (FBNS)
        // 4. 탄성 변형 갱신 (CG-FFT)
        // 5. 힘 평형 확인 → PID로 h₀ 조정
        // 6. 수렴까지 반복
    }
}

// 학습 데이터 대량 생성: rayon 병렬
pub fn generate_training_data(n_samples: usize) -> Vec<HMEHLResult> {
    let params = lhs_sample(n_samples);
    params.par_iter().map(|p| solver.solve(p)).collect()
}
```

**장점**: Rust로 구현하면 솔버를 surrogate 없이도 **직접 호출** 가능.
단일 접촉 정밀 분석 시 ML 근사 없이 정확한 HMEHL 결과를 얻을 수 있다.
surrogate는 반복 계산(360° 원주 × 10 슬라이스 = 3600 호출)의 가속용.

### 1.2 CG-FFT 탄성 변형 솔버

Polonsky-Keer (1999) 알고리즘:

$$h_{el}(\xi) = \sum \frac{2}{\pi E'} K(\xi - \xi_R) \cdot p(\xi_R)$$

- **매크로**: 전체 접촉 영역 탄성 변형 (Eq.3.8)
- **마이크로**: 조도 돌기별 탄성 변형 (Eq.3.5)
- **핵심**: FFT convolution으로 O(N²) → O(N log N) 가속
- **구현**: `numpy.fft` 사용, 커널 함수 K 사전 계산

### 1.3 Homogenization (Hansen 2011)

조도를 직접 Reynolds에 넣지 않고, **유효 유동 계수**로 변환:

- 마이크로 도메인에서 조도-접촉 문제를 미리 풀어 → A, b̃, C, d̃ 계수 산출
- 매크로 Reynolds에 A, b̃를 대입 → 매끈 표면처럼 효율적으로 풀 수 있음
- 조도의 통계적 매개변수 (Rq, 상관길이, skewness, kurtosis)로 표면 생성

### 1.4 전단응력 및 동력손실

Poiseuille + Couette 흐름 전단응력 (SNU Eq.3.20):

$$\vec{\tau}_{fl} = -\frac{h_m}{2} C \nabla p + \frac{\eta}{h_m}(-6u_m\vec{d} + u_s)(1-\theta)$$

돌기 접촉 전단 (Eq.3.21):

$$\vec{\tau}_{con} = C_{f,b} \cdot p_{con}$$

동력손실 (Eq.3.23):

$$P(x) = a \int_0^{h_m} \vec{\tau}_{tot} \frac{\partial u}{\partial z} dz$$

---

## Phase 2: 솔버 검증

### 2.1 압력 분포 검증 — Coulon et al.

- 볼 위 표면 패임(dent) → 압력 피크 재현
- 순수 미끄럼, 두 속도 조건 (25, 75 mm/s)
- 비교: 압력 프로파일 형태 및 피크 위치/크기

### 2.2 유막두께 검증 — Kaneta et al.

- 볼 위 표면 파형(wave) → 유막 두께 분포 재현
- 순수 미끄럼, 두 속도 조건 (21.6, 98 mm/s)
- 비교: 중심선 유막 프로파일

### 2.3 동력손실 검증 — Petry-Johnson et al.

- 실제 기어 쌍 동력손실 시험
- 다양한 속도/토크 조건
- 비교: 평균 동력손실 [W]

### 2.4 TRB 자체 검증

- M1(DH)/M2(MK) 결과와 HMEHL 비교
- 동일 입력에서 유막두께, 마찰계수 비교
- 기대: HMEHL ≈ M2 (조도 포함 시), HMEHL > M1 (조도 없을 시)

---

## Phase 3: 학습 데이터 생성

### 3.1 설계 변수 범위 (TRB 조건)

SNU Table 3.3 기반, TRB 운전 범위로 조정:

| 변수 | 최소 | 최대 | 단위 |
|------|------|------|------|
| 수직 하중 F_N | 10 | 2000 | N |
| 표면 속도 u₁ | 0.1 | 30 | m/s |
| 표면 속도 u₂ | 0.1 | 30 | m/s |
| 밀도 ρ₀ | 750 | 1000 | kg/m³ |
| 점도 η₀ | 5e-3 | 5e-1 | Pa·s |
| 압력-점도 계수 α | 5e-9 | 30e-9 | Pa⁻¹ |
| 조도 Rq | 0.05 | 5.0 | μm |
| 상관 길이 R_cl | 1 | 100 | μm |
| 경도 H_v | 200 | 800 | HV |
| 유효 반경 R_eq | 2 | 50 | mm |
| 접촉 길이 L | 5 | 50 | mm |
| 축소 영률 E' | 100 | 230 | GPa |

### 3.2 샘플링

- **Latin Hypercube Sampling (LHS)** — 12차원 공간 균일 커버
- **20,000 ~ 30,000 샘플** (SNU: 20,000으로 R² > 0.99)
- 병렬 실행: Rust rayon, 샘플당 ~0.1-1초 → 총 ~30분-6시간

### 3.3 출력 변수

| 출력 | 단위 | 용도 |
|------|------|------|
| 마찰 계수 μ | - | 트랙션/동력손실 |
| 동력 손실 P_loss | W | 발열 |
| 최대 표면 응력 σ_surf_max | MPa | 표면 피로 |
| 평균 돌기 압력 p_asp_mean | MPa | 하중 분담 |
| 유효 유막두께 h_eff | μm | Λ 비 |

---

## Phase 4: ML Surrogate

### 4.1 아키텍처 (SNU 방식 적용)

```
Modal 1 (조도):     [Rq, R_cl, H_v, E'] → Dense(128) → Dense(128)
Modal 2 (기하):     [R_eq, L]            → Dense(128) → Dense(128)
Modal 3 (운전):     [F_N, u₁, u₂]       → Dense(128) → Dense(128)
Modal 4 (윤활):     [η₀, α, ρ₀]         → Dense(128) → Dense(128)
                           ↓
                    Concat Fusion Layer
                           ↓
                    Dense(64) → Dense(64)
                           ↓
                    Output: [μ, P_loss, σ_surf_max, p_asp_mean, h_eff]
```

- **프레임워크**: PyTorch
- **활성함수**: ReLU
- **옵티마이저**: Adam (lr=1e-3 → cosine decay)
- **데이터 분할**: Train 70% / Val 15% / Test 15%
- **정규화**: [-1, 1] (Eq.3.24)
- **목표**: R² > 0.99 (Test set)

### 4.2 ONNX 변환

```python
torch.onnx.export(model, dummy_input, "hmehl_surrogate.onnx")
```

→ Rust에서 `ort` (ONNX Runtime) 크레이트로 로드

---

## Phase 5: Ioannides-Harris 표면 수명

### 5.1 표면 피로 적분

Ioannides-Harris (1985) 확장:

$$\ln\frac{1}{S_{surf}} = \bar{B} \cdot N^e \cdot \int_A \left\langle \sigma_s^c - \sigma_{u,s} \right\rangle dA$$

- **σ_s**: HMEHL surrogate에서 예측한 표면 최대 응력
- **σ_{u,s}**: 표면층 피로 한계 (재료 의존, ~200-400 MPa for bearing steel)
- **c**: 응력 지수 (≈ 31/3, Lundberg-Palmgren과 동일)
- **e**: Weibull 기울기 (10/9)
- **⟨ ⟩**: Macaulay bracket (피로 한계 이하면 기여 0)
- **A**: 접촉 면적
- **B̄**: 표면 피로 상수 (시험 데이터로 교정)

### 5.2 교정 전략

B̄ (표면 피로 상수)는 시험 데이터가 필요하다. 교정 방법:

1. **기존 a_SKF 곡선에서 역산**: κ가 낮을 때 a_SKF 감소 → 표면 기여분 분리
2. **공개 베어링 시험 데이터**: 표면 고장이 확인된 시험 결과에서 B̄ 피팅
3. **GBLM 논문 예시 재현**: Morales-Espejel (2015) 논문의 Figure에서 B̄ 역산

> **주의**: B̄ 교정 없이는 L₁₀,surf의 절대값이 의미 없음.
> 초기에는 상대 비교(조건 A vs B)에 활용하고, 교정 데이터 확보 후 절대값 신뢰.

### 5.3 간소화 옵션

Full 면적 적분 대신, surrogate 출력(σ_surf_max, p_asp_mean)을 직접 사용:

$$L_{10,surf} \propto \left(\frac{\sigma_{u,s}}{\sigma_{surf,max}}\right)^c$$

이 형태는 S-N 곡선과 동일한 구조이며, B̄ 교정이 더 단순해진다.

---

## Phase 6: TRB 시스템 통합

### 6.1 Rust 통합 구조

```rust
// ONNX 모델 로드 (앱 시작 시 1회)
let session = ort::Session::builder()
    .with_model("hmehl_surrogate.onnx")?;

// Per-slice 호출
fn predict_surface_stress(
    params: &SliceContactParams,
    session: &ort::Session,
) -> SurfaceStressResult {
    let input = normalize_input(params);
    let output = session.run(input)?;
    denormalize_output(output)
}

// GBLM 결합
fn compute_gblm_life(
    l10_sub: f64,   // ISO 281
    l10_surf: f64,  // I-H surface
    e: f64,         // Weibull slope (10/9)
) -> f64 {
    (l10_sub.powf(-e) + l10_surf.powf(-e)).powf(-1.0/e)
}

// Surface Risk
fn surface_risk(l10_sub: f64, l10_surf: f64) -> f64 {
    let rs = 1.0 / l10_surf;
    let rss = 1.0 / l10_sub;
    rs / (rs + rss)
}
```

### 6.2 UI 확장

- **수명 탭**: ISO 281 수명 + GBLM 수명 병렬 표시
- **Surface Risk 게이지**: SR = 0 (표면하 지배) ~ 1 (표면 지배)
- **개선 가이드**: SR > 0.5이면 "표면 처리/윤활 개선 필요", SR < 0.5이면 "베어링 크기 증가 필요"
- **민감도 차트**: Rq, κ, SRR 변화에 따른 L₁₀ 변화

### 6.3 ONNX 모델 배포

- `hmehl_surrogate.onnx` 파일을 앱 번들에 포함
- Tauri build 시 자동 포함 (`tauri.conf.json` resources)
- 모델 크기: ~1 MB (17,000 파라미터 × fp32)

---

## 의존성 및 Critical Path

```
P1 (HMEHL 솔버) ──→ P2 (검증) ──→ P3 (학습 데이터)
                                        ↓
                                   P4 (ML surrogate)
                                        ↓
                         P5 (I-H 수명) ←─┘
                              ↓
                         P6 (TRB 통합)
```

- P1이 전체 프로젝트의 **Critical Path** — HMEHL 솔버 품질이 모든 것을 결정
- P3는 P2 완료 후 rayon 병렬 실행 (멀티코어 CPU, Rust native)
- P4는 P3 데이터에만 의존
- P5는 P4의 surrogate 출력 구조에 의존
- P6는 P4(ONNX) + P5(수명 엔진)에 의존

---

## 리스크 및 완화 전략

| 리스크 | 영향 | 완화 |
|--------|------|------|
| HMEHL 수렴 실패 | P1 지연 | FBNS 솔버는 robust, SNU/Hansen 검증 완료 |
| 학습 데이터 불충분 | R² < 0.99 | 데이터 추가 생성 (30,000→50,000) |
| B̄ 교정 데이터 부재 | L₁₀,surf 절대값 부정확 | 상대 비교 먼저 활용, 시험 데이터 확보 후 교정 |
| ONNX 추론 속도 | 실시간 UI 응답 | 모델 경량화, 배치 추론 |
| 기어 → 베어링 적용 차이 | 정확도 한계 | TRB 전용 검증 데이터 세트 구축 |

---

## 참고 문헌

### HMEHL 솔버
- Hansen, E. et al. (2011). "A New Film Parameter for the Homogenization of Rough Surfaces"
- Polonsky, I.A. & Keer, L.M. (1999). "A numerical method for solving rough contact problems based on the multi-level multi-summation and conjugate gradient techniques"
- Akchurin, A. et al. (2015). "On a model for the prediction of the friction coefficient in mixed lubrication"

### ML Surrogate
- SNU 논문: HMEHL + Multi-modal DL 기반 기어 부하 동력손실
- Marian, M. et al. (2022). "Predicting EHL film thickness parameters by machine learning approaches"

### 표면 피로 수명
- Ioannides, E. & Harris, T.A. (1985). "A New Fatigue Life Model for Rolling Bearings"
- Morales-Espejel, G.E. & Gabelli, A. (2015). "A Model for Rolling Bearing Life with Surface and Subsurface Survival"
- Morales-Espejel, G.E. & Brizmer, V. (2011). "Micropitting modelling in rolling-sliding contacts: application to rolling bearings"

### 트랙션/마찰
- Arana, A. et al. (2019). "Partial EHL friction coefficient model to predict power losses in cylindrical gears"
- Clarke, A. et al. (2016). ECR-based asperity load sharing measurements

---

## 현재 시스템과의 관계

```
현재 TRB 시스템 (완성)
├── ISO 281 수명 (L₁₀,sub) ← 유지
├── M1/M2/M3 유막두께 ← 유지 (빠른 설계 검토용)
├── Film Decay (Van Zoelen) ← 유지
├── S_λ micropitting 안전율 ← 유지 (간이 스크리닝)
└── τ_E 자동추정 (Arana) ← 유지

+ 신규 HMEHL/GBLM 모듈 (추가)
  ├── HMEHL ML surrogate → 정밀 마찰/표면응력
  ├── L₁₀,surf (I-H 표면 적분)
  ├── L₁₀,GBLM (결합 수명)
  └── Surface Risk 진단
```

기존 기능은 모두 유지되며, HMEHL/GBLM은 **고급 분석 옵션**으로 추가된다.
사용자는 "빠른 검토(M1+S_λ)"와 "정밀 분석(HMEHL+GBLM)"을 선택할 수 있다.
