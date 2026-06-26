# EHL 솔버 수렴성 연구: 고부하(High Moes M) 조건에서의 기법 비교

## 연구 배경

Moes 파라미터 M > 5000-15000 영역에서 Roelands 점도가 η/η₀ = 10²⁶ 수준으로 극단적 stiffness를 만들어 EHL 솔버 수렴이 매우 어려워진다. 본 문서는 이 문제를 해결하는 state-of-the-art 접근법을 비교 분석한다.

### 핵심 문제: P-h₀ Coupling 실패

고부하(M > 5000)에서의 근본적 어려움:
1. Roelands 점도 η/η₀ → 10²⁶ → Poiseuille 유동 계수 ε = ρh³/(12η) → 0
2. Reynolds 방정식에서 ∂p/∂x 항이 소멸 → **압력이 film thickness에 insensitive**
3. h₀의 변화가 Reynolds residual에 거의 영향 없음 (∂F_R/∂h₀ ≈ 0)
4. P와 h₀를 동시에 결정할 수 없음 → 진동 또는 trivial solution

**TRB 프로젝트 실험 결과 (ehl2d.rs):**
- M=1628 (TRB 실제 조건): 완전 수렴, h_c=0.128μm, p/p_h=1.39
- M=14000: P-only GMRES + PID h₀에서 잔차 5760→150으로 감소 후 stall
- 원인: GMRES가 P를 변경 → 하중 이동 → PID가 h₀ 조정 → Reynolds 변동 → 반복 순환

---

## 1. Venner & Lubrecht (2000) — Multigrid (FAS) 접근법

### 핵심 구조
- **FAS (Full Approximation Scheme)** 비선형 multigrid 사용
- **Semi-system approach**: Reynolds 방정식(P)과 탄성 변형(H)을 **교번(alternating)** 으로 해석
- 압력 P는 Reynolds relaxation으로 갱신, 변형 H는 P로부터 적분으로 계산

### 수렴 핵심 기법

#### 1.1 Hybrid Relaxation
- **Gauss-Seidel relaxation**: 유동(Wedge) 지배 영역 (ε/A > 0.01)에 적용
  - under-relaxation factor: 0.5-1.0
- **Jacobi Dipole relaxation (distributive)**: 탄성 지배 영역 (ε/A < 0.01, 즉 Hertz 접촉부 내부)
  - under-relaxation factor: 0.3-0.6
  - 압력 변동을 인접 노드에 분산시켜 elastic coupling 반영
- 두 방식을 **한 그리드 레벨에서 영역별로 동시 적용**

#### 1.2 Multi-Level Multi-Integration (MLMI)
- 탄성 적분 ∫K(x-s)p(s)ds 를 O(n²) → **O(n ln n)** 으로 가속
- coarse grid에서 먼 거리 기여를 계산, fine grid에서 근거리만 보정

#### 1.3 FMG (Full Multigrid) 초기화
- 가장 coarse grid (15-32 nodes)에서 시작
- 해를 interpolation하여 fine grid로 전달 (nested iteration)
- 각 레벨에서 2V-cycle (저부하) 또는 2W-cycle (고부하) 적용

### 고부하 수렴 한계
- **M ≤ 200** 정도까지 안정적 수렴 보고 (Venner 1991 원논문 기준)
- 고부하(M > 1000)에서는 relaxation이 stall하는 현상 발생
- 근본 원인: **semi-system 접근법에서 P와 H의 교번 해석이 strongly coupled 상태에서 수렴률 저하**
- Hertz 접촉부 내부에서 ε → 0 (Reynolds Poiseuille항 소멸)으로 Reynolds 방정식의 압력 정보가 소실

### 최근 개선 (2021)
- Berzins et al. (ASME J. Tribol., 2021) "In-Depth Exploration of the Multigrid Method":
  - 새로운 pressure restriction operator로 load balance relaxation 단순화
  - V-cycle, W-cycle, F-cycle 비교, cavitation 경계 처리 개선
  - line solver 적용, relaxation factor 최적화
  - mass conservation 알고리즘으로 micro-cavitation 처리
  - **그러나 근본적인 M 한계는 여전히 존재** (semi-system의 구조적 한계)

### 그리드 크기
- 225 ~ 114,689 nodes (level 5 ~ level 14)
- 도메인: X = -4.0 ~ +1.5 (Hertz 반폭 기준)

### 수렴률
- 최적 조건에서 **1 coarse grid correction cycle당 잔차 1 order 감소** (선형 수렴)
- quadratic 수렴 아님 — true Newton이 아니기 때문

---

## 2. FBNS (Fischer-Burmeister Newton-Schur) — Hansen et al. (2011, 2022)

### 핵심 아이디어
cavitation 조건을 **complementarity problem**으로 정식화:
```
p·θ = 0,   p ≥ 0,   θ ≥ 0
```
여기서 p = 압력, θ = cavity fraction.

Fischer-Burmeister 함수: φ(a,b) = √(a²+b²) - a - b = 0 ⟺ a≥0, b≥0, ab=0

이를 통해 부등식 제약을 **미분 가능한 비제약 문제**로 변환.

### 솔버 구조
1. **Reynolds + cavitation** → Fischer-Burmeister로 비제약 비선형 연립방정식 구성
2. **Newton method**로 전체 시스템 해석
3. **Schur complement**로 선형계를 효율적으로 풀어 banded 구조 활용
4. 탄성 변형은 **BEM (Boundary Element Method)** + FFT convolution으로 O(N log N) 계산

### 수렴 특성
- Newton 기반이므로 **이론적 quadratic 수렴**
- 실행시간 **O(N log N)** 스케일링 (N = grid points)
- 그러나 **high M에서의 수렴 보장에 대한 명시적 보고는 제한적**
- 주로 hydrodynamic/mixed lubrication 영역에 초점 (M < 1000 범위)

### FBNS-EHL 확장 (Hansen 2022)
- 탄성 변형을 BEM으로 추가하여 full EHL로 확장
- Roelands 및 Dowson-Higginson 관계식 내장
- homogenized roughness 지원 (Patir-Cheng)
- MATLAB 구현 공개: [GitHub - ErikHansenGit/EHL](https://github.com/ErikHansenGit/EHL)

### 한계점
- **Semi-system 성격**: P는 Newton으로, H는 BEM으로 별도 계산 후 coupling
- 고부하에서 η/η₀ → ∞ 에 의한 Jacobian ill-conditioning 처리 미흡
- M > 5000 영역에 대한 benchmark 없음

---

## 3. Habchi (2008, 2018) — Full-System Finite Element 접근법

### 핵심 혁신: **Full-System Coupling**

Reynolds 방정식 + 탄성 방정식 + 하중 평형을 **동시에** 하나의 비선형 시스템으로 구성:

```
F(P, U, h₀) = [Reynolds residual; Elasticity residual; Load balance] = 0
```

Newton-Raphson으로 **전체 자코비안** ∂F/∂(P,U,h₀) 를 한 번에 풀음.

### ★ 핵심 구분: 두 가지 Full-System 접근법

Habchi의 연구에서 가장 중요하게 이해해야 할 점은 **elastic deformation 계산 방식이 두 가지**라는 것이다:

#### 접근법 A: 3D Elastic Body Volume Mesh (Habchi 2008 원저)

**Habchi의 원래 접근법**은 탄성체를 3D 유한요소 체적 메쉬(volume mesh)로 이산화한다:
- 접촉 표면 아래의 반무한체(half-space)를 **3D Navier-Lame 탄성 방정식**으로 풀음
- 표면에서는 Reynolds 방정식, 체적 내부에서는 linear elasticity
- 두 방정식을 **동일한 FE 프레임워크에서 monolithic하게** 결합
- 2차 Lagrange 요소 사용 (표면: 1D/2D Reynolds, 체적: 2D/3D elasticity)

**P-h₀ coupling이 자연스럽게 해결되는 이유:**
- 체적 내부의 elastic DOF들이 surface pressure P와 surface displacement U를 물리적으로 연결
- 접촉부 내부에서 ε→0이 되어 Reynolds가 trivial해져도, **탄성 방정식이 여전히 P를 constrain**
- h₀는 표면 변위의 적분적 결과로 자동 결정됨 (별도의 h₀ 미지수 불필요)
- Jacobian에서 ∂(elasticity)/∂P 블록이 **접촉부 전체에서 nonzero** → P-h coupling 유지

**단점**: subsurface volume을 이산화해야 하므로 **DOF 수가 매우 큼** (수만~수십만)

#### 접근법 B: Differential Deflection (Hughes/Evans 2000, Habchi 발전)

탄성 적분을 2차 미분 형태로 변환하여 surface-only 해석:

```
원래: H(X) = H₀ + X²/2 - (2/π) ∫ P(S) ln|X-S| dS    [적분 형태 → dense Jacobian]

변환: d²H/dX² = 1 - (2/π) × d/dX [∫ P'(S)/(X-S) dS]   [미분 형태 → banded Jacobian]
```

Evans & Hughes (2000)가 개발한 핵심 아이디어:
- 반무한체 위의 압력 하중에 의한 변형의 Laplacian을 적절한 구적식으로 평가
- 이 differential deflection의 효과는 **극도로 국소화(localized)** — 직접 적분 대비
- Hertz 압력 분포로 검증됨 (line contact + point contact)

**이 변환의 효과:**
- Jacobian이 **banded (tri-diagonal 근방)** 구조가 됨
- O(N) 메모리, O(N) 솔브 (banded LU 또는 direct solver)
- **물리적으로 동일한 해**, 수학적 재정식화만 다름
- Volume mesh 불필요 → DOF가 surface node 수와 같음

**P-h₀ coupling 처리:**
- d²H/dX²에서 H₀ 항은 미분 시 소멸 (d²H₀/dX² = 0)
- 대신 **경계조건**으로 H₀를 결정: H(X_inlet) = 지정값 또는 load balance
- Coupled Newton에서 h₀를 N+1번째 미지수로 추가, load balance를 추가 방정식으로

### 수렴 성능
- **Quadratic convergence** (true Newton-Raphson)
- 전형적으로 **5-10 Newton iterations**으로 수렴 (semi-system 대비 10-100배 적은 iteration)
- **고부하(수 GPa) 접촉에서도 안정적 수렴** — stabilized FE formulation 사용
- Habchi (2008): "stabilizing terms extending the method for high loads up to several Gigapascals"

### Stabilization for High Loads (SUPG)

Reynolds 방정식이 고부하에서 **convection-dominated** 형태가 됨:
- Poiseuille항 (∂/∂x[ρh³/(12η) · ∂p/∂x]) → 0 (η → ∞이므로) → **diffusion 소멸**
- Couette항 (∂/∂x[ρhu_m]) 만 남음 → **순수 convection (advection)**
- 표준 Galerkin FEM은 convection-dominated 문제에서 spurious oscillation 발생

**SUPG (Streamline Upwind Petrov-Galerkin) 적용:**
```
약형식에 stabilization 항 추가:
∫ W·R dΩ + Σ_e ∫_Ωe τ(v·∇W)·R dΩ = 0

여기서:
  W = test function (weight)
  R = Reynolds residual
  v = advection velocity (Couette flow direction)
  τ = stabilization parameter (element Peclet number 기반)
```

- τ는 local Peclet number Pe = |v|h_e/(2D)에 의존 (D = diffusion ≈ ε)
- Pe → ∞ (고부하)에서 τ → h_e/(2|v|) (full upwind limit)
- **정확도**: O(h^{p+1/2}) convection-dominated, O(h^{p+1}) diffusion-dominated
- GLS (Galerkin/Least-Squares)가 SUPG보다 우수할 수 있음 (reactive term도 안정화)

Habchi et al. (2012) "Stabilized fully-coupled finite elements for EHL problems" 에서 상세히 다룸.

### Model Order Reduction (MOR) — 3D Body 접근법의 효율화

3D elastic body의 DOF를 줄이는 기법:

#### Static Condensation with Splitting (SCS)
- **핵심**: 체적 내부 노드를 **Guyan condensation**으로 제거, 표면 노드에만 효과 집중
- 수학적으로 **정확(exact)** — 근사 오차 없음
- 단, condensation 후 행렬이 dense → **Splitting 알고리즘**으로 sparse 패턴 유지
- 속도 향상: **3배~15배** (조건에 따라)

#### EHL-Basis Reduction
- 탄성 DOF를 30개 미만으로 축소 가능
- Schur-complement 기반 — 표면 자유도에 내부 효과를 투영
- 열 EHL에서 line contact 10-20% 감소, circular contact 50%+ 감소

### 그리드 크기
- FE 특성상 variable unstructured meshing 가능
- 접촉부 고밀도, 입출구 저밀도 → 실질 자유도 감소 (256-512 등가)
- Model order reduction (MOR) 적용 시 더욱 축소 가능

---

## 4. Hughes/Evans/Snidle — Differential Deflection 독자 발전

Hughes, Elcoate, Evans (2000)가 개발한 differential deflection 기반 coupled solver:

### 핵심 기여
- **Evans & Hughes**: 반무한체 위 압력 하중에 의한 변형의 Laplacian에 대한 구적식(quadrature) 유도
- Line contact 및 point contact 모두 적용
- Hertz 압력 분포로 유효성 검증
- **핵심 발견**: differential deflection의 영향이 직접 적분 대비 **극도로 국소화(localized)**

### 구현 세부
```
Line contact differential deflection:
d²δ/dx² = -(2/πE') × [P_{i+1} - 2P_i + P_{i-1}] / Δx² + correction terms

이 식은 tri-diagonal 구조 → banded Jacobian 자연스럽게 형성
```

- Damped Newton procedure 사용 (line search)
- 접촉부 내에서도 elasticity가 P를 constrain
- Holmes, Evans, Hughes, Snidle (2003): transient point contact으로 확장

### Tribonet MATLAB 구현
- Tribonet.org에서 line contact EHL solver 공개
- 완전 coupled, differential deflection 기반
- Newton solver로 빠른 수렴
- **"The most robust and fast way to solve EHL systems is to use a fully coupled approach and Newton's scheme"** (Tribonet 인용)

---

## 5. Jacobian-free Newton-Krylov (JFNK) — Bujurke & Kantli (2017-2024)

### 핵심 아이디어
Jacobian을 명시적으로 구성하지 않고, matrix-vector product만으로 Newton 시스템을 풀음:

```
J·δx ≈ [F(x + ε·v) - F(x)] / ε    (finite difference Jacobian-vector product)
```

GMRES (Krylov subspace)로 선형계를 반복적으로 풀음.

### Wavelet-based Preconditioner
- 웨이블릿 기반 전처리로 GMRES 수렴 가속
- Dense non-symmetric 시스템을 효율적으로 처리
- **장점**: 조밀한 그리드에서도 안정적, 큰 time step 허용

### 적용 영역
- 주로 transient EHL + surface asperities (거칠기 효과)
- Grease 윤활, non-Newtonian 효과 포함
- Newton-GMRES for thermal EHL (2024, J. Inst. Eng.)

### 한계
- 고부하(M > 5000)에서의 수렴성에 대한 명시적 보고는 부족
- Preconditioner 성능이 문제 stiffness에 의존
- Jacobian-free이므로 P-h₀ coupling의 근본적 ill-conditioning을 직접 해결하지는 못함

---

## 6. 핵심 비교: Coupled vs Decoupled

| 항목 | Semi-System (Multigrid/FBNS) | Full-System (Differential Deflection) | Full-System (3D Body FE) |
|------|-----|-----|-----|
| P-H 연립 방식 | 교번 (alternating) | **동시 (simultaneous)** | **동시 (simultaneous)** |
| 탄성 계산 | 적분/BEM | **d²H/dX² 미분 형태** | **3D Navier-Lame FE** |
| Jacobian 구조 | 없음 (relaxation) | **Sparse banded** | **Sparse (MOR 후)** |
| 수렴 차수 | Linear (multigrid) | **Quadratic** | **Quadratic** |
| Iteration 수 | 50-500+ | **5-10** | **5-10** |
| 고부하 안정성 | M > 1000에서 stall | **수 GPa까지 안정** | **수 GPa까지 안정** |
| 그리드 확장성 | O(N log N) | **O(N)** | O(N²~N³) → MOR로 개선 |
| 구현 복잡도 | 중간 | **중간** | 높음 |
| P-h₀ coupling | 본질적으로 분리 | BC + load balance | **물리적으로 자연 해결** |

### 왜 Full-System이 고부하에서 유리한가?

고부하(M > 5000)에서의 물리:
1. Hertz 접촉부 내부: ε = ρh³/(12η) → 0 → Reynolds 방정식에서 **P 정보 소실**
2. Semi-system에서는 P를 Reynolds로만 갱신 → 접촉부 내부에서 P 갱신 불가
3. Full-system에서는 elasticity 방정식이 P 정보를 직접 제공 → **접촉부 내부에서도 P가 Hertz로 수렴**

핵심 통찰: **고부하 EHL에서 접촉부 내부 압력은 Reynolds가 아닌 탄성 방정식이 결정한다.**
Full-system은 이를 자연스럽게 반영하고, semi-system은 구조적으로 이를 놓친다.

---

## 7. Roelands 점도 특이성 처리

### 문제
Roelands: η = η₀ · exp(S(-1 + (1+p/p_r)^Z))

pH = 2 GPa, α = 20 GPa⁻¹ 일 때 η/η₀ ≈ 10²⁶ → exp(60)

### 각 솔버의 처리 방법

#### Reduced Pressure (Ertel-Grubin)
```
q = ∫₀ᵖ (η₀/η(s)) ds
```
Barus 모델(η=η₀e^αp)에서는 q = (1-e^{-αp})/α → p→∞에서 q→1/α (유한값)
Roelands에서는 유사하게 q가 bounded → Reynolds를 q에 대해 풀면 stiffness 제거

**그러나**: Reduced pressure 변환은 Couette 항이 있을 때 완전하지 않음.
Ertel-Grubin 근사의 기본 가정: 접촉부 내부 압력 ≈ Hertz → inlet 영역에서만 Reynolds를 풀면 됨.
이 근사는 M이 매우 클 때 오히려 정확해짐 (접촉부가 거의 dry contact에 가까워지므로).

#### Viscosity Capping (현재 TRB 구현)
```rust
eta_0 * exp_arg.min(60.0).exp()   // cap at exp(60)
```
물리적 근거: ε = ρh³/(12ηΛ) → 0이면 η = 10⁶이든 10²⁶이든 결과 동일.
장점: 구현 간단, Jacobian 수치 안정성 확보.
한계: 천이 영역에서 미세한 차이 발생 가능.

#### Full-System FE (Habchi)
- P와 H가 동시에 풀리므로, η→∞ 인 접촉부에서 Reynolds는 자동으로 trivial
- 탄성 방정식이 접촉부 P를 결정 → **특이성이 문제되지 않음**
- SUPG stabilization이 convection-dominated 영역의 수치 진동 방지

---

## 8. Continuation / Parameter Stepping 기법

### 8.1 Load Stepping (F 연속)
- F_target을 F_start → F_target까지 점진적 증가
- 이전 해를 다음 단계 초기값으로 사용
- **단점**: Hertz 파라미터(b, p_h)가 매 단계 변화 → 비차원 압력 전달이 불완전

### 8.2 α-Continuation (현재 TRB 구현) ★ 추천
- 압력-점도 계수 α를 0.03α → 1.0α까지 점진적 증가
- **장점**: Hertz 파라미터(b, p_h)가 α에 무관하게 일정 → 비차원 P가 완벽 전달
- 물리적 의미: "뉴턴 유체 → 실제 piezoviscous 유체"로의 연속 변형
- 적응적 step size: 성공 시 증가, 실패 시 절반으로 축소

### 8.3 Homotopy (학문적)
```
F(x, λ) = λ·F_target(x) + (1-λ)·F_easy(x) = 0
```
λ=0 (쉬운 문제) → λ=1 (목표 문제)까지 predictor-corrector로 경로 추적.
arc-length continuation으로 turning point도 통과 가능.

### 8.4 Inlet Zone Optimization
- Habchi & Issa (2022, Tribology Letters): inlet computation zone 최적화
- 도메인 크기가 수렴에 미치는 영향 분석
- 고부하에서 inlet이 매우 좁아지므로 적절한 domain sizing이 critical

---

## 9. 최근 발전 (2020-2026)

### 9.1 Physics-Informed Neural Networks (PINNs) for EHL
- **HD-PINN** (2024, Lubricants): Reynolds 방정식 + cavitation을 PINN으로 풀음
  - Sliding/squeezing motion, transient cavitation 모델링
  - 학습 후 ms 단위 예측 (vs. 수초~수분 기존 solver)
- **Weighted PINN** (2025, Friction): Hertz-like 접촉에서 elastic response 계산
  - Linear elasticity를 neural network으로 변환
- **한계**: 고부하 EHL(M > 5000)에 대한 PINN 적용은 아직 미보고
  - 비매끄러운 경계조건(solid contact)에서 neural operator 퇴화 현상 보고
  - EHL의 multi-scale 특성(inlet vs contact zone)이 학습 어려움

### 9.2 Fully Coupled Finite Line Contact (2024)
- "A fully coupled FE model for mixed EHL of finite line contacts" (Tribology International, 2024)
- Monolithic 시스템으로 모든 governing equation 통합
- Weakly coupled 대비 **5% 높은 정확도, 50% 높은 효율**
- Logarithmic profiling으로 최소 막두께 30배 증가, 최대 asperity 압력 64.6% 감소
- Heavy-load, low-velocity 조건에서 효과적

### 9.3 Non-iterative EHL Approach (2024)
- Viscoelastic 기판에 대한 non-iterative visco-elasto-hydrodynamic 해석
- 각 time step에서 **linear matrix equation 1회 풀이**로 해 구함
- 반복 수렴 자체를 제거 → 본질적으로 안정적
- **단, viscoelastic 특수 조건에 한정** — 일반 steel contact에는 직접 적용 불가

### 9.4 Stable FE for Reynolds Equation (Leonhartsberger, 2025)
- TU Wien 학위논문: Reynolds 방정식의 안정적 유한요소법
- SUPG 포함한 여러 stabilization 기법 비교 분석
- EHL 고부하 조건에서의 pressure-density 관계 포함

### 9.5 Integrated Finite Volume Framework for TEHL (2022)
- 열-탄성유체역학 윤활을 위한 통합 유한체적법 프레임워크
- Reynolds + energy 방정식 동시 해석
- Newton-Raphson 기반 coupled scheme

---

## 10. 솔버별 M 파라미터 달성 범위

정확한 수치를 명시한 논문은 드물지만, 문헌 종합으로 추정:

| 솔버 유형 | M 달성 범위 | 근거 |
|-----------|------------|------|
| Multigrid (Venner 1991) | M ≤ ~200 | Venner thesis, stall 보고 |
| Multigrid + 개선 (2021) | M ≤ ~500-1000 | 개선된 relaxation/cycle로도 한계 |
| FBNS (Hansen 2022) | M ≤ ~1000 | 주로 HD/mixed lubrication 초점 |
| Differential Deflection + Newton | M ≤ ~5000-10000 (추정) | Coupled Newton의 장점, 문헌 직접 보고 부족 |
| Habchi 3D Body FE | **M > 10000, 수 GPa** | "high loads up to several GPa" 명시 |
| TRB ehl2d.rs (현재) | M ≤ ~3000 | GMRES P-only + PID h₀, M=14000에서 stall |

### 실제 응용에서의 M 범위 (참고)
- 일반 볼 베어링: M ≈ 50-500
- 원통/원추 롤러 베어링: M ≈ 500-5000
- 기어 접촉 (저속 고부하): M ≈ 2000-20000
- 극한 조건 (캠-팔로워, 중하중 기어): M ≈ 10000-50000+

---

## 11. 현재 TRB 구현 분석 및 개선 방향

### 현재 구현 (ehl2d.rs)
- **GMRES + α-continuation** (M > 3000)
- Roelands cap at exp(60)
- FFT elastic deformation (surface-only, integral form)
- 128-256 grid
- **P-h₀ 분리**: P는 GMRES, h₀는 PID controller → M > 5000에서 oscillation

### ★ 핵심 진단: 왜 현재 구현이 M > 5000에서 실패하는가

1. **Surface-only integral form** 사용 → Jacobian이 dense 또는 FFT로 implicit
2. **P와 h₀가 분리** (P: GMRES, h₀: PID) → semi-system 성격
3. Reynolds에서 ε→0이면 ∂F_R/∂h₀ ≈ 0 → h₀ 미결정
4. **탄성 방정식이 P를 직접 constrain하지 않음** → 접촉부 내부에서 P 갱신 불가

### 개선 가능 방향 (우선순위 순)

#### ★★★ 1순위: Differential Deflection + Coupled Newton (가장 현실적)
**이것이 가장 효과적인 다음 단계이다.**

구현 계획:
1. 탄성 적분 `H(X) = H₀ + X²/2 - (2/π)∫P ln|X-S|dS` 를 2차 미분 형태로 변환
2. `d²H/dX² = 1 - (2/π) × finite_diff(P)` → tri-diagonal band
3. P, h₀를 하나의 Newton 시스템으로 결합: `[Reynolds(N); LoadBalance(1)]` 크기 (N+1)
4. Jacobian: Reynolds의 ∂F_R/∂P (banded) + ∂F_R/∂h₀ (column) + differential deflection terms
5. Damped Newton with line search
6. SUPG stabilization 추가 (optional but recommended)

예상 효과: M ≤ 5000-10000 달성 가능

#### ★★ 2순위: SUPG Stabilization
- Reynolds FE 이산화에 SUPG 추가
- local Peclet number 기반 τ 계산
- convection-dominated 영역의 oscillation 제거
- Differential deflection과 결합하면 효과 극대화

#### ★ 3순위: 3D Elastic Body FE (장기)
- 가장 근본적이지만 구현 복잡도 높음
- Navier-Lame 3D FE + surface Reynolds coupling
- MOR (Static Condensation) 필수
- M > 10000 이상에서 필요할 때만 고려

---

## 12. 공학적 대안: M이 극단적일 때

전체 수치 EHL이 불가능하거나 불필요한 경우의 대안:

### 12.1 Dowson-Higginson / Hamrock-Dowson 경험식
```
Line contact central:  h_c = 3.06 R U^0.69 G^0.56 W^-0.10
Line contact minimum:  h_min = 2.65 R U^0.70 G^0.54 W^-0.13
```
- 장점: 즉시 계산, M 범위 무관
- 단점: profile 효과, misalignment 불포함, 점근 고부하에서 부정확

### 12.2 Moes 필름 두께 식 (개선)
- Moes (2000)가 모든 M, L 범위를 커버하는 단일 식 제안
- PR (piezoviscous-rigid), PE (piezoviscous-elastic), IR, IE 4개 점근 영역 합성
- 고부하 극한에서 정확도 향상

### 12.3 Ertel-Grubin 입구 해석
- 접촉부 내부 = Hertz (가정), inlet만 Reynolds로 풀어 film thickness 결정
- **M이 극히 클 때 오히려 정확**: 접촉부가 거의 dry contact
- 해석적/반해석적 풀이 가능 → iteration 불필요

### 12.4 Hybrid Approach (추천 실용 전략)
1. Moes 경험식으로 h_c, h_min 추정
2. 수치 EHL로 pressure profile만 계산 (Hertz 초기값, 소수 iteration)
3. Inlet/outlet 상세는 Ertel-Grubin으로 보정
4. Life/fatigue 계산에는 Hertz + correction factor 사용

---

## 참고 문헌

### 핵심 논문
1. Venner, C.H. (1991). "Multilevel solution of the EHL line and point contact problems." Ph.D. thesis, University of Twente.
2. Venner, C.H. & Lubrecht, A.A. (2000). "Multi-Level Methods in Lubrication." Elsevier.
3. Hughes, T.G., Elcoate, C.D., Evans, H.P. (2000). "Coupled Solution of the EHL Line Contact Problem Using a Differential Deflection Method." Proc. IMechE Part C, 214, 585-598.
4. Habchi, W., Eyheramendy, D., Vergne, P., Morales-Espejel, G. (2008). "A Full-System Approach of the Elastohydrodynamic Line/Point Contact Problem." ASME J. Tribol., 130(2), 021501.
5. Habchi, W. (2018). "Finite Element Modeling of Elastohydrodynamic Lubrication Problems." Wiley. ISBN: 978-1-119-22512-6.
6. Hansen, E. et al. (2022). "An EHL Extension of the Unsteady FBNS Algorithm." Tribology Letters, 70:81.
7. Habchi, W. et al. (2012). "Stabilized fully-coupled finite elements for EHL problems." Conference paper, TU Delft.

### MOR 및 효율화
8. Habchi, W. & Issa, J. (2017). "An Exact and General Model Order Reduction Technique for the FE Solution of EHL Problems." ASME J. Tribol., 139(5), 051501.
9. Habchi, W. & Issa, J. (2019). "A Schur-complement model-order-reduction technique for the FE solution of transient EHL problems." Advances in Engineering Software, 127, 28-37.
10. Habchi, W. & Issa, J. (2023). "Exact Model Order Reduction for the Full-System Finite Element Solution of Thermal EHL Problems." Lubricants, 11(2), 61.

### JFNK 및 최신 기법
11. Bujurke, N.M. & Kantli, M.H. (2020). "Jacobian-free Newton-Krylov subspace method with wavelet-based preconditioner for analysis of transient EHL problems with surface asperities." Appl. Math. Mech., 41(6), 881-898.
12. Kantli, M.H. et al. (2024). "Newton-GMRES Method for Thermal EHL of Line Contact Problems." J. Inst. Eng. (India): Series C.

### 최근 논문 (2020-2026)
13. Berzins et al. (2021). "In-Depth Exploration of the Multigrid Method to Simulate EHL Line Lubrications." ASME J. Tribol., 143(12), 121602.
14. Habchi, W. & Issa, J. (2022). "An Inlet Computation Zone Optimization for EHL Line Contacts." Tribology Letters.
15. "A fully coupled finite element solution for the mixed EHL of finite line contacts." (2024). Tribology International.
16. Leonhartsberger, C. (2025). "Stable finite element methods for the Reynolds equation." TU Wien dissertation.

### PINN 관련
17. "Extrapolation of cavitation and hydrodynamic pressure in lubricated contacts: a PINN approach." (2025). AMSES.
18. "Weighted PINN for obtaining elastic responses under Hertzian-like contact." (2025). Friction.
19. "Physics-Informed Neural Networks for the Reynolds Equation with Transient Cavitation Modeling." (2024). Lubricants, 12(11), 365.

### 기초 참고
20. Woloszynski, T. et al. (2015). "Efficient Solution to the Cavitation Problem in Hydrodynamic Lubrication." Tribology Letters, 58:18.
21. Morales-Espejel, G.E. & Wemekamp, A.W. (2008). "Ertel-Grubin methods in EHL - a review." Proc. IMechE Part J.

## 온라인 소스

- [Tribonet EHL Line Contact Solver](https://www.tribonet.org/cmdownloads/line-contact-ehl-solver/)
- [Hansen EHL-FBNS GitHub](https://github.com/ErikHansenGit/EHL)
- [Habchi Full-System Approach (Semantic Scholar)](https://www.semanticscholar.org/paper/A-full-system-finite-element-approach-to-problems-Habchi/5944dbf81d957eab64c75582e12d393ba8930fa1)
- [ASME In-Depth Multigrid Exploration](https://asmedigitalcollection.asme.org/tribology/article-abstract/143/12/121602/1103237)
- [Habchi Book (Wiley)](https://onlinelibrary.wiley.com/doi/book/10.1002/9781119225133)
- [Stabilized FE for Reynolds (TU Wien, 2025)](https://repositum.tuwien.at/bitstream/20.500.12708/221407/1/Leonhartsberger%20Christoph%20-%202025%20-%20Stable%20finite%20element%20methods%20for%20the...pdf)
- [INSA Lyon EHL Introduction](https://moodle.insa-lyon.fr/pluginfile.php/125723/mod_resource/content/1/ehl28.pdf)
