# Chapter 9: 5-DOF Bearing Equilibrium

## 9.1 Overview

베어링 레벨 해석(Level 1)은 외부 하중 `(F_r, F_a, M)`으로부터 내링의 5자유도 변위 `(δx, δy, δz, γx, γy)`를 구하는 비선형 평형 문제이다. 각 롤러의 접근량과 법선력을 동시에 만족시켜야 한다.

### Degrees of Freedom

| DOF | Symbol | Unit | Description |
|-----|--------|------|-------------|
| 1 | δx | μm | Radial displacement (F_r direction) |
| 2 | δy | μm | Radial displacement (perpendicular to F_r) |
| 3 | δz | μm | Axial displacement |
| 4 | γx | rad | Tilting about x-axis |
| 5 | γy | rad | Tilting about y-axis |

## 9.2 Roller Angular Positions

Z개 롤러의 원주 위치는 **레이디얼 하중 방향**을 기준으로 배치:

$$
\psi_j = \phi_{load} + j \cdot \frac{2\pi}{Z}, \quad j = 0, 1, \ldots, Z-1
$$

여기서 하중 방향 각도:

$$
\phi_{load} = \text{atan2}(F_y, F_x)
$$

- **Roller #0이 항상 레이디얼 하중 방향에 위치** (worst-case 보장)
- 베어링은 회전 요소이므로, 하중 지지 관점에서 최대 하중 롤러가 정확히 하중 방향에 있는 경우가 worst case
- 순수 축하중(F_r ≈ 0)일 때 φ_load = 0° (기본 x축 방향)
- 결과의 `load_angle_deg` 필드에 실제 적용된 하중 방향 각도 [°] 출력

## 9.3 Roller Approach

내링 변위 `(δx, δy, δz, γx, γy)`에서 각 롤러 위치 `ψ_j`에서의 강체 접근량:

### Radial Component

$$
\delta_r = \delta_x \cos\psi + \delta_y \sin\psi \quad [\mu m]
$$

### Axial Component

$$
\delta_a = \delta_z + \frac{d_{pw}}{2} \times 1000 \times (\gamma_x \sin\psi - \gamma_y \cos\psi) \quad [\mu m]
$$

`d_pw/2` [mm] × `γ` [rad] = [mm] → ×1000 → [μm]

### Combined Approach

$$
\delta_{rigid} = \delta_r \cos\alpha + \delta_a \sin\alpha - \frac{G_r}{2}
$$

여기서:
- `α`: 접촉각 [rad]
- `G_r/2`: 반경 방향 클리어런스의 절반 [μm]

`δ_rigid > 0` → 접촉 발생 → Gen1/Gen3로 해석 (dual-raceway model, Ch.5 §5.4)
`δ_rigid ≤ 0` → 비접촉 (Q_j = 0)

> **참고**: δ_rigid는 outer raceway 법선(α_o) 방향의 강체 접근량이다. Gen1/Gen3 내부에서 이 값으로부터 inner/outer 양쪽 프로파일 보정과 양쪽 Hertz 변형을 고려하여 슬라이스별 하중 q_k를 결정한다 (Ch.4 §4.4, Ch.6 §6.2 참조).

## 9.4 Force and Moment Equilibrium

### Residual Vector

$$
R_0 = \sum_j Q_j \cos\alpha \cos\psi_j - F_r \quad \text{[N]}
$$

$$
R_1 = \sum_j Q_j \cos\alpha \sin\psi_j \quad \text{[N]}
$$

$$
R_2 = \sum_j Q_j \sin\alpha - F_a \quad \text{[N]}
$$

$$
R_3 = \sum_j Q_j \frac{d_{pw}}{2} \sin\alpha \sin\psi_j - M_x \quad \text{[N·mm]}
$$

$$
R_4 = -\sum_j Q_j \frac{d_{pw}}{2} \sin\alpha \cos\psi_j - M_y \quad \text{[N·mm]}
$$

단위 변환: `F_r, F_a` [kN] → [N] (×1000), `M` [kN·m] → [N·mm] (×10⁶)

### 리브 하중 처리

리브 접촉력은 **롤러 내부 반력**으로, 베어링 전체 평형에는 포함하지 않는다 (Harris 표준). 롤러-레이스웨이 법선력 `Q_j`만 평형에 참여한다. 리브 결과는 별도 응력 평가용으로 기록한다.

## 9.5 TRB Induced Thrust and Axial Constraint Mode

### 9.5.1 Induced Thrust

TRB의 기본 특성: 반경 하중은 축방향 내부 추력(induced thrust)을 생성한다.

$$
F_{a,induced} = F_r \tan\alpha_o
$$

여기서 `α_o`는 외륜 접촉각이다.

### 9.5.2 Mode Selection

외부 축하중 `F_a`와 내부 추력 `F_{a,induced}`의 관계에 따라 두 가지 모드로 분기한다:

| Condition | Mode | DOF | Description |
|-----------|------|-----|-------------|
| `F_a ≥ F_{a,induced}` | **비구속 모드** (Unconstrained) | 3 (δx, δy, δz) | 표준 3×3 Newton-Raphson |
| `F_a < F_{a,induced}` | **축방향 구속 모드** (Axially Constrained) | 2 (δx, δy) | δz = 0 고정, 2×2 Newton-Raphson |

### 9.5.3 Physical Basis of Axial Constraint Mode

단열 TRB에서 `F_a < F_r·tan(α_o)`이면, 부족한 축방향 추력은 하우징(또는 대향 베어링)이 축방향 구속으로 흡수해야 한다. 이 경우:

- **δz = 0** (하우징이 축방향 변위를 구속)
- 축방향 평형은 솔버가 풀지 않으며, 하우징 반력 `R_a`를 출력으로 계산
- 반경 방향 평형 (Fx, Fy)만 2×2 Newton-Raphson으로 해석

$$
R_a = \sum_j Q_j \sin\alpha - F_a \quad \text{[N]}
$$

`R_a > 0`이면 하우징(또는 대향 베어링)이 이 축방향 반력을 지지해야 한다.

### 9.5.4 Mathematical Background

`F_a ≈ F_r·tan(α_o)` 부근에서 3×3 Jacobian이 rank-1으로 퇴화(degenerate)한다:

- 하중 영역 파라미터 `ε → 0` → 하나의 롤러만 접촉
- 이 때 `∂Fx/∂δz`와 `∂Fz/∂δz`가 비례 → Jacobian 특이
- 3×3 NR은 수렴 불가

축방향 구속 모드(δz = 0)는 이 특이점을 물리적으로 올바르게 회피한다. δz = 0에서 하중 영역은 정확히 반원(ψ₀ = 90°, 절반의 롤러가 접촉)이 되어 Jacobian이 항상 양호하다.

## 9.5A Preload Displacement Mode

### 9.5A.1 Overview

실제 TRB 장착에서는 축방향 예압을 **변위(displacement)**로 부여한다. 너트 조임 또는 심(shim) 조절로 외륜(또는 내륜)을 축방향으로 눌러 초기 접촉 상태를 만든다. FEA에서도 동일하게 외륜 고정 + 예압을 축변위로 부여하는 것이 표준 경계 조건이다.

본 솔버는 "Disp. from Force" 방식을 **두 가지 버전**으로 제공하여, 물리적 프리로드 세팅 조건의 모사와 운전 중 축하중 정밀 매칭을 모두 지원한다.

### 9.5A.2 Preload Mode 3종

| Mode | UI 표시 | Description | DOF (Phase A) | Use Case |
|------|---------|-------------|---------------|----------|
| **DisplacementFromForce** | Disp. from Force | F_a → δz 변환 후 **δz 고정**, 2×2 NR | 2 (δx, δy) | 물리적 프리로드 세팅 (기본값) |
| **DisplacementFromForceIterative** | Disp. from Force (iter.) | F_a → δz 초기값, 3×3 NR로 δz 재조정 | 2 or 3 | F_a reaction 정밀 매칭 검증 |
| **Displacement** | Displacement | 직접 δz 입력, δz 고정 2×2 NR | 2 (δx, δy) | 실측 예압량 직접 입력 |

### 9.5A.3 공통: Force-to-Displacement Conversion (1D Newton-Raphson)

`DisplacementFromForce`와 `DisplacementFromForceIterative` 모두 첫 단계로 목표 예압력 `F_a`를 등가 축변위 `δz_preload`로 변환한다.

**순수 축하중 상태 가정** (반경 하중 없이, 전 롤러 균일 접촉):

$$
F_a^{target} = Z \cdot Q(\delta_z) \cdot \sin\alpha_o
$$

여기서 단일 롤러 법선력:
$$
Q(\delta_z) = \sum_k C_k \cdot [\delta_z \cdot \sin\alpha_o - \Delta z_{total,k}]^{10/9}_+
$$

**1D NR 알고리즘:**

```
FOR iter = 0, ..., max_iter:
    Q_per_roller = solve_gen1(δz·sin(α_o))
    F_a_current = Z × Q × sin(α_o)
    err = F_a_current - F_a_target

    IF |err| / F_a_target < tol: CONVERGED

    // Forward difference Jacobian
    Q' = solve_gen1((δz+h)·sin(α_o))
    dF_dδz = Z × (Q' - Q) × sin(α_o) / h

    δz -= err / dF_dδz
```

**리브 결합 모드 (Rib Coupled):** 리브 컴플라이언스가 활성화된 경우, 각 δz 평가 시 내부 고정점 반복으로 `δ_rib`을 수렴시킨다:

```
FOR each NR step:
    δ_rib = 0
    FOR rib_iter = 0, ..., 30:
        δ_eff = max(δ_rigid - δ_rib × sin(α_o), 0)
        Q = solve_gen1(δ_eff)
        Q_axial = Q × sin(α_o - α_i) / cos(α_i)
        δ_rib_new = rib_contact(Q_axial).delta_rib
        IF |δ_rib_new - δ_rib| < 0.001 μm: BREAK
        δ_rib = δ_rib_new
```

수렴 후 `δz_preload` [μm]을 메인 솔버에 전달한다. 이 값은 **반경 하중이 없는 순수 축하중 상태**에서의 등가 변위이다.

### 9.5A.4 DisplacementFromForce (Simple): δz 고정 모드

**물리적 배경:** 실제 프리로드 세팅은 반경 하중이 없는 상태에서 수행한다. 너트 토크 또는 심(shim)으로 축방향 변위를 부여하면, 이 변위는 운전 중에도 기구학적으로 유지된다. 따라서 §9.5A.3에서 구한 `δz_preload`를 그대로 고정하고 반경 평형만 푸는 것이 물리적으로 정확하다.

**알고리즘:**

1. §9.5A.3의 1D NR로 `δz_preload` 계산
2. `δz = δz_preload`로 고정
3. 2×2 NR for (δx, δy) — 반경방향 평형만 해석
4. 축방향 반력 `F_a,reaction = Σ Q_j · sin(α_o)`는 **출력** (구속 조건이 아님)

```
δz = δz_preload (fixed)
FOR iter = 0, ..., max_iter:
    R = compute_residual(δx, δy, δz, 0, 0)
    IF ||(R[0], R[1])|| / max(F_r, 1) < tol: CONVERGED

    // 2×2 Jacobian (forward difference)
    J[i,j] = (R[i](x_j + h) - R[i]) / h,  i,j ∈ {0,1}

    // Solve: J × [Δδx, Δδy] = -[R[0], R[1]]
    // Step limiting + line search
```

**특성:**
- `F_a,reaction`은 일반적으로 입력 `F_a`와 정확히 일치하지 않음 (반경 하중에 의한 하중 분포 변화 때문)
- 이것이 정상이며 물리적으로 올바른 결과임
- FEA 경계 조건(외륜 고정, 내륜 축변위 부여)과 동일한 해석 조건

### 9.5A.5 DisplacementFromForceIterative: δz 재조정 모드

**목적:** 반경 하중이 가해진 운전 상태에서도 `Σ Q_j · sin(α_o) = F_a` 축방향 평형을 정확히 만족시키고자 할 때 사용한다. 계산 결과의 F_a reaction 검증 또는 특수 해석 목적으로 제공한다.

**알고리즘:**

1. §9.5A.3의 1D NR로 `δz_preload` 계산 (초기 추정값)
2. `δz = δz_preload`를 초기값으로 설정
3. 3×3 NR for (δx, δy, δz) — **축방향 평형 포함**

```
δz = δz_preload (initial guess)

// F_a ≥ F_a,induced 인 경우: 3×3 NR (δz 자유)
IF F_a ≥ F_r·tan(α_o):
    FOR iter = 0, ..., max_iter:
        R = compute_residual(δx, δy, δz, 0, 0)
        R_scaled = [R[0]/cos(α), R[1]/cos(α), R[2]/sin(α)]
        IF ||R_scaled|| / F_scaled_total < tol: CONVERGED

        // 3×3 scaled Jacobian
        // Solve: J × [Δδx, Δδy, Δδz] = -R_scaled
        // Step limiting + line search

// F_a < F_a,induced 인 경우: simple 모드와 동일 (δz 고정, 2×2 NR)
ELSE:
    → §9.5A.4와 동일하게 동작
```

**특성:**
- `F_a,reaction`이 입력 `F_a`와 정밀하게 일치 (수렴 허용치 내)
- `δz`가 초기 추정값에서 미세 조정됨 (보통 ±0.1~0.5 μm 차이)
- `F_a < F_a,induced`일 때는 Simple 모드와 동일하게 δz 고정

### 9.5A.6 Displacement: 직접 변위 입력

사용자가 `δz_preload` [μm]을 직접 입력한다. 1D NR 변환 과정 없이 즉시 2×2 NR로 진행한다.

- 실측 프리로드 변위가 있는 경우
- 파라메트릭 스터디 (δz를 직접 스윕하며 영향 분석)
- 타 소프트웨어의 변위 출력 값을 입력으로 사용

### 9.5A.7 Three Modes Comparison

| 항목 | DisplacementFromForce | DisplacementFromForceIterative | Displacement |
|------|----------------------|-------------------------------|-------------|
| 입력 | F_a [kN] | F_a [kN] | δz [μm] |
| δz 결정 | 1D NR (순수 축하중) | 1D NR → 3×3 NR 재조정 | 사용자 직접 입력 |
| Phase A DOF | 2 (δx, δy) | 3 (δx, δy, δz) or 2 | 2 (δx, δy) |
| δz 고정 여부 | 항상 고정 | F_a ≥ F_a,induced: 자유 | 항상 고정 |
| F_a,reaction | 출력 (F_a 입력과 차이 가능) | F_a 입력과 정밀 일치 | 출력 (독립) |
| FEA 대응 | ◎ (동일 경계조건) | ○ (힘 평형 강제) | ◎ (직접 변위) |
| 물리적 의미 | 실제 장착 → 운전 과정 | 운전 중 축력 정밀 제어 | 실측값 직접 적용 |
| 주 용도 | **기본 해석** (권장) | 검증용, F_a 매칭 필요 시 | 실측/파라메트릭 |

### 9.5A.8 F_a,reaction 해석 가이드

`DisplacementFromForce` 모드에서 `F_a,reaction ≠ F_a,input`이 되는 것은 정상이다.

**차이 발생 원인:**
- `δz_preload`는 순수 축하중(F_r = 0) 상태에서 계산됨
- 실제 운전 시 반경 하중에 의해 하중 분포가 변화
- 접촉 롤러 수, 각 롤러의 접촉각 변화가 축방향 반력에 영향

**해석 기준:**

| F_a,reaction vs F_a,input | 의미 |
|---------------------------|------|
| 거의 일치 (< 1% 차이) | 반경 하중 영향이 작음 (F_r << F_a) |
| F_a,reaction > F_a,input | 반경 하중이 추가 축력 유발 (induced thrust) |
| F_a,reaction < F_a,input | 반경 하중에 의한 하중 재분배로 축력 감소 |

`DisplacementFromForceIterative` 모드와 비교하면, 동일 입력 조건에서의 δz 차이를 통해 반경 하중의 축방향 영향을 정량적으로 파악할 수 있다.

## 9.6 Scaled Newton-Raphson (Phase A)

### 9.6.1 Ill-Conditioning Problem

표준 `(δx, δz)` Newton-Raphson은 `F_a ≈ F_r tan(α)` 부근에서 Jacobian이 악조건화(ill-conditioned)된다:

$$
\text{cond}(\mathbf{J}) \approx \cot^2(\alpha) \cdot \text{cond}(\mathbf{M})
$$

α = 12°이면 `cot²(12°) ≈ 23.2`로 조건수가 약 20배 증가한다.

### 9.6.2 Contact-Line Approach Space

해결책: 접촉선 접근량 공간 `(A, B)`에서 풀어 조건수를 개선한다:

$$
A = \delta_x \cos\alpha, \quad B = \delta_z \sin\alpha
$$

이 좌표계에서:
$$
\delta_{rigid}(\psi) = A \cos\psi + B
$$

**Jacobian M**: `(A, B)` 공간에서

$$
\mathbf{M} \approx \begin{bmatrix} \sum k \cos^2\psi & \sum k \cos\psi \\ \sum k \cos\psi & \sum k \end{bmatrix}
$$

`cond(M) ≈ 2~3`으로 자연스럽게 양호하다.

### 9.6.3 Algorithm (Phase A: Force Only)

```
Scaled residual: R'_r = R[0] / cos(α),  R'_a = R[2] / sin(α)
Scaled total:    F'_total = √((F_r/cosα)² + (F_a/sinα)²)

FOR iter = 0, ..., max_iter:
    Compute residual R[0..4]

    IF ||(R'_r, R'_a)|| / F'_total < tol: BREAK

    // Perturb A: δx += h/cos(α)
    M[0,0] = (R'_r(A+h) - R'_r) / h
    M[0,1] = (R'_r(B+h) - R'_r) / h
    M[1,0] = (R'_a(A+h) - R'_a) / h
    M[1,1] = (R'_a(B+h) - R'_a) / h

    // Solve M × [ΔA, ΔB] = -[R'_r, R'_a]
    (ΔA, ΔB) = M⁻¹ × [-R'_r, -R'_a]

    // Convert to physical: Δδx = ΔA/cos(α), Δδz = ΔB/sin(α)
    // Step limiting: ||step|| ≤ 5 μm

    // Backtracking line search (scaled residual norm)
    α_ls = 1.0
    WHILE scaled_residual_new ≥ scaled_residual_old:
        α_ls *= 0.5
```

### 9.6.4 Constrained Mode: 2×2 NR (δx, δy)

축방향 구속 모드에서는 δz = 0으로 고정하고, (δx, δy)에 대한 2×2 Newton-Raphson을 수행한다.

```
Residual: R_r = [R[0], R[1]]  (radial only)

FOR iter = 0, ..., max_iter:
    Compute residual R[0..4] with δz = 0

    IF ||R_r|| / max(F_r, 1) < tol: BREAK

    // 2×2 Jacobian by forward difference
    J[0,0] = (R[0](δx+h) - R[0]) / h
    J[0,1] = (R[0](δy+h) - R[0]) / h
    J[1,0] = (R[1](δx+h) - R[1]) / h
    J[1,1] = (R[1](δy+h) - R[1]) / h

    // Solve 2×2: J × [Δδx, Δδy] = -R_r
    // Step limiting: ||step|| ≤ max_step

    // Backtracking line search (radial residual norm)
    α_ls = 1.0
    WHILE radial_residual_new ≥ radial_residual_old:
        α_ls *= 0.5
```

반경 방향 수렴 후, 하우징 반력은 다음과 같이 계산한다:

$$
R_a = \sum_j Q_j \sin\alpha - F_a
$$

## 9.7 Moment Equilibrium (Phase B) — Block-Decomposed Solver

외부 모멘트 M ≠ 0 또는 외부 기울어짐 γ ≠ 0인 경우에만 수행. `γx, γy`를 2-DOF Newton-Raphson으로 풀되, 각 반복 후 force equilibrium을 재수렴시킨다.

> **참고**: Phase B는 외부 모멘트/기울어짐이 없으면 비활성화된다. TRB에서 접촉각에 의한 자연 기울어짐(natural tilting)은 축-베어링 시스템 모델에서만 정확히 계산 가능하다 (§9.11 참조).

### Algorithm

```
FOR iter = 0, ..., max_iter:
    // 2×2 moment Jacobian (central difference)
    J_m[i,j] = (R[3+i](γ+h_j) - R[3+i](γ-h_j)) / (2h)

    // Solve: J_m × Δγ = -R_m
    // Line search for γ update

    // Re-converge force equilibrium (1-DOF sequential)
    Sequential δx, δz correction (20 inner iterations)
```

## 9.8 Initial Guess

$$
\delta_x = \frac{F_r}{K_{radial}} \in [0.5, 50] \quad [\mu m]
$$

$$
\delta_z = \frac{F_a - F_r \tan\alpha}{K_{axial}} \in [-20, 50] \quad [\mu m]
$$

여기서:
- `K_radial ≈ (Z/2) × 500 × cos(α)` [N/μm]
- `K_axial ≈ Z × 500 × sin(α)` [N/μm]
- 500 N/μm: 단일 롤러의 근사 강성

## 9.9 Convergence Criteria

### Unconstrained Mode (3-DOF)

| Criterion | Formula | Tolerance |
|-----------|---------|-----------|
| Force | ‖(R₀, R₁, R₂)‖ / F_total | < tol × 10 |
| Moment | ‖(R₃, R₄)‖ / M_norm | < tol × 10 (M ≠ 0) |

### Axially Constrained Mode (2-DOF)

| Criterion | Formula | Tolerance |
|-----------|---------|-----------|
| Force (radial only) | ‖(R₀, R₁)‖ / max(F_r, 1) | < tol × 10 |
| Moment | ‖(R₃, R₄)‖ / M_norm | < tol × 10 (M ≠ 0) |

축방향 잔차 `R₂`는 구속 모드에서 확인하지 않는다 (하우징 반력으로 흡수).

`tol`은 solver.convergence_tol (기본값: 1e-4).

## 9.10 Alert Generation

수렴 후 진단 경고를 생성한다:

| Condition | Level | Description |
|-----------|-------|-------------|
| Q_max / Q_mean > 5 | Warning | 불균등 하중 분포 |
| Load zone < 120° | Warning | 좁은 하중 영역 |
| p_max_rib > 1500 MPa | Warning | 과대 리브 응력 |
| No rollers in contact | Critical | 전체 비접촉 |
| Axially constrained | Info | 축방향 구속 모드 활성, 하우징 반력 R_a 출력 |

## 9.11 5-DOF Full NR Solver (Alternative)

### 9.11.1 개요

`solve_bearing_equilibrium_5dof()`는 block-decomposed solver의 대안으로, nalgebra LM-damped Newton-Raphson을 사용하여 force equilibrium을 동시에 풀어 강건한 수렴을 제공한다.

**주요 특징:**
- Block solver 결과를 warm-start로 사용
- Levenberg-Marquardt damping으로 ill-conditioned Jacobian 안정화
- 중심차분(central difference) 수치 Jacobian
- DOF/잔차 독립 스케일링
- Line search + step limiting

### 9.11.2 활성 DOF

| 조건 | 활성 DOF | 비고 |
|------|----------|------|
| F_a ≥ F_a,induced | δx, δy, δz | 3-DOF force equilibrium |
| F_a < F_a,induced | δx, δy | 2-DOF radial (δz 고정) |

**γx, γy는 항상 비활성**: 독립 베어링 모델에서 모멘트 잔차(My)는 접촉각 커플링에 의한 자연 기울어짐 반력이다. γy에 대한 My 감도(∂My/∂γy)가 극히 낮아 수렴 불가. 축 모델과 연계 시 γ DOF를 활성화할 수 있다.

### 9.11.3 스케일링

변위와 잔차의 크기 차이를 정규화:

| DOF | 스케일 계수 d_scale | 물리적 의미 |
|-----|-------------------|------------|
| δx, δy, δz [μm] | 1.0 | 이미 μm 단위 |
| γx, γy [rad] | 1000 | 1e-3 rad ≈ 0.06° 기준 |

| 잔차 | 스케일 계수 r_scale | 물리적 의미 |
|------|-------------------|------------|
| Fx, Fy, Fz [N] | 1 / F_total | 전체 하중 대비 |
| Mx, My [N·mm] | 1 / M_norm | 모멘트 기준 대비 |

### 9.11.4 Levenberg-Marquardt Damping

$$
(\mathbf{J}^T \mathbf{J} + \lambda \mathbf{I}) \Delta \mathbf{x} = -\mathbf{J}^T \mathbf{r}
$$

- λ = diag_max(J^TJ) × 1e-6 (minimal damping)
- nalgebra LU decomposition으로 풀이
- 야코비안이 near-singular일 때 안정성 보장

### 9.11.5 모멘트 반력 출력

5-DOF 솔버 수렴 후, 모멘트 잔차는 자연 기울어짐 반력으로 해석:

$$
M_{y,reaction} = -\sum_j Q_j \frac{d_{pw}}{2} \sin\alpha \cos\psi_j
$$

이 값은 축(shaft)이 베어링에 가하는 구속 모멘트에 해당하며, 축-베어링 시스템 해석 시 입력값으로 사용된다.

### 9.11.6 Block Solver와의 비교

| 항목 | Block Solver (기본) | 5-DOF Full NR |
|------|-------------------|---------------|
| 구조 | Phase A → Phase B | 동시 풀이 |
| 모멘트 | 외부 M ≠ 0일 때만 | 반력으로 출력 |
| δz 처리 | 모드에 따라 2/3-DOF | 항상 자유 (≥ induced) |
| 수렴성 | 양호 | LM damping으로 강건 |
| 속도 | 빠름 | 약간 느림 (SVD 오버헤드) |
| 용도 | 일반 해석 | MASTA/FE 비교, 정밀 해석 |
