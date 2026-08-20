# Chapter 7: Gen3 — Beam-Coupled Slice Solver

## 7.1 Concept

Gen3 모드에서는 롤러를 **Timoshenko 빔**으로 모델링하고, 각 슬라이스에 비선형 Hertz 접촉 스프링을 연결한다. 빔의 굽힘/전단 변형이 슬라이스 간 하중을 재분배하여, 에지 응력을 정밀하게 예측한다.

```
  δ_rigid ──────────────────── (rigid approach)
     │    │    │    │    │
     ↓    ↓    ↓    ↓    ↓
    ╔═╗──╔═╗──╔═╗──╔═╗──╔═╗   ← Timoshenko beam elements (coupled)
    ╚═╝  ╚═╝  ╚═╝  ╚═╝  ╚═╝
     │    │    │    │    │
    ╔═╗  ╔═╗  ╔═╗  ╔═╗  ╔═╗   ← Hertz contact springs (nonlinear)
    ╚═╝  ╚═╝  ╚═╝  ╚═╝  ╚═╝
     │    │    │    │    │
  ───────────────────────────  (raceway)
```

## 7.2 Timoshenko Beam Element

### 7.2.1 Section Properties

원형 단면에서:

$$
I = \frac{\pi}{4} R_{roller}^4, \quad A = \pi R_{roller}^2
$$

테이퍼 롤러이므로 각 요소에서 양단 반경의 평균 사용: `R_avg = (R_k + R_{k+1}) / 2`

### 7.2.2 Element Stiffness Matrix (4×4)

2절점 × 2 DOF (w: 횡변위, θ: 회전):

$$
\mathbf{K}_e = \frac{EI}{L^3(1+\Phi)} \begin{bmatrix}
12 & 6L & -12 & 6L \\
6L & (4+\Phi)L^2 & -6L & (2-\Phi)L^2 \\
-12 & -6L & 12 & -6L \\
6L & (2-\Phi)L^2 & -6L & (4+\Phi)L^2
\end{bmatrix}
$$

여기서:
- `L = l_k`: 요소 길이 (슬라이스 폭) [mm]
- `E`: 롤러 Young's modulus [MPa]
- `I`: 단면 2차 모멘트 [mm⁴]
- `Φ`: Timoshenko 전단 변형 파라미터

### 7.2.3 Shear Parameter Φ

$$
\Phi = \frac{12 E I}{\kappa G A L^2}
$$

여기서:
- `G = E / [2(1+ν)]`: 전단 탄성 계수 [MPa]
- `κ = 10 / (9 + 10ν)`: Timoshenko 전단 보정 계수 (원형 단면)
- **Euler-Bernoulli**: `Φ = 0` (전단 변형 무시)

**물리적 의미**: `Φ`가 클수록 전단 변형의 기여가 크다. TRB 롤러는 L/D ≈ 1.5~2.5로 stocky beam이므로 전단 효과가 무시할 수 없다.

### 7.2.4 Shear vs Bending

| L/D Ratio | Shear Effect | Recommended Model |
|-----------|-------------|-------------------|
| > 10 | < 0.5% | Euler-Bernoulli |
| 3~10 | 1~5% | Either |
| < 3 | > 5% | Timoshenko |

TRB 롤러 (L/D ≈ 1.5~2.5): **Timoshenko 필수**

## 7.3 Global Stiffness Assembly

n 슬라이스 → n 절점 → (n-1) 요소 → 2n × 2n 전역 강성행렬.

DOF 순서: `[w₀, θ₀, w₁, θ₁, ..., w_{n-1}, θ_{n-1}]`

경계 조건: **Free-free** (양단 자유). 접촉 스프링이 구속을 제공한다.

```
K_global = Assembly of K_e for elem = 0, ..., n-2

DOF mapping: element e → global DOFs [2e, 2e+1, 2e+2, 2e+3]

K_global[(2e+i, 2e+j)] += K_e[(i, j)]  for i,j = 0..3
```

## 7.4 Newton-Raphson with Active Set

### 7.4.1 Equilibrium Equation

빔 내력과 접촉력의 평형:

$$
\mathbf{K}_{beam} \cdot \frac{\mathbf{w}}{1000} = \mathbf{f}_{contact}
$$

여기서:
- `w` [μm]: 빔 굽힘 변위 벡터 (강체 성분 제거됨)
- `K_beam` [N/mm]: 빔 강성행렬 (mm 단위 운영)
- `f_contact` [N]: 접촉력 벡터

### 7.4.2 Gap and Contact Force (Dual-Raceway)

슬라이스 k에서의 간극 (dual-raceway model):

$$
gap_k = \delta_{rigid} - w_{2k} - \Delta z_{total,outer,k} - \Delta z_{total,inner,k} \cdot \cos(\alpha_o - \alpha_i)
$$

`gap_k > 0`이면 접촉 → Dual-raceway Hertz 접촉력 및 강성 계산 (Ch.5 §5.4.1).

### 7.4.3 Residual

$$
\mathbf{R} = \mathbf{K}_{beam} \cdot \frac{\mathbf{w}}{1000} - \mathbf{f}_{contact}(\mathbf{w})
$$

### 7.4.4 Jacobian

$$
\mathbf{J} = \frac{\mathbf{K}_{beam}}{1000} + \text{diag}(K_{contact,k})
$$

여기서 `K_contact,k = K_hertz_k × l_k` (접촉 중인 슬라이스만).

### 7.4.5 Complete Algorithm

```
OUTER LOOP (active set iteration, max 15):

  INNER LOOP (Newton-Raphson, max max_iterations):
    FOR each slice k:
      gap_k = δ_rigid - w[2k] - Δz_total_outer_k
              - Δz_total_inner_k × cos(α_o - α_i)
      IF gap_k > 0:
        active[k] = true
        (q_k, b_k, p_max_k, K_hertz_k) = compute_slice_contact(gap_k)
        f_contact[2k] = q_k × l_k
        K_c[k] = K_hertz_k × l_k

    // Check active set minimum
    IF n_active < 2: fallback to Gen1

    // Residual
    R = K_beam × (w/1000) - f_contact

    // Convergence check
    IF ||R|| / max(||f_contact||, 1) < tol:
      IF active_set unchanged: RETURN result
      ELSE: update active_set, BREAK to outer

    // Jacobian
    J = K_beam / 1000
    FOR k in active: J[2k, 2k] += K_c[k]

    // Solve
    Δw = J⁻¹ × (-R)    (LU decomposition)
    w += Δw

    // Remove rigid body modes
    remove_rigid_body_modes(w, slices)
```

## 7.5 Rigid Body Mode Projection

핵심: `δ_rigid`가 이미 강체 접근량을 담당하므로, `w`에는 순수 굽힘 변형만 포함되어야 한다.

매 NR 반복 후 `w`에서 강체 병진 + 회전 성분을 제거한다:

### Algorithm

1. 축 위치 평균: `x̄ = mean(x_k)`
2. 변위 평균 (병진): `w̄ = mean(w_{2k})`
3. 기울기 (회전): `β = Σ[w_{2k} × (x_k - x̄)] / Σ[(x_k - x̄)²]`
4. 보정:
   - `w_{2k} -= w̄ + β(x_k - x̄)` (변위 DOF)
   - `w_{2k+1} -= β` (회전 DOF)

## 7.6 Gen3 for Target Load

Gen1과 동일한 구조로, `δ_rigid`를 Newton-Raphson으로 탐색하되, 내부에서 `solve_gen3_roller`를 호출:

$$
Q_{total}(\delta_{rigid}) = Q_{target}
$$

각 반복에서 Gen3 롤러 솔버가 완전히 수렴해야 하므로, 이중 중첩 Newton-Raphson이 된다.

## 7.7 Gen1 vs Gen3 Comparison

동일한 `δ_rigid`에서:

| Metric | Gen1 | Gen3 |
|--------|------|------|
| Edge q_k | Underestimated | Accurate (beam redistributes) |
| Center q_k | Overestimated | Reduced (load spreads) |
| Q_total | Same or similar | Same or similar |
| p_max_k range | Wider | Narrower (smoother) |

**Level C Validation**: 평탄 프로파일(Δz=0)에서 Gen1 ≈ Gen3 (차이 < 3%).

## 7.8 Gen3 Split Contact

`use_split_contact = true`이면 Gen3에서 내/외륜 하중을 독립 계산한다 (Chapter 17 참조).

- 빔 양쪽에 독립 탄성 기초: `f_net = (q_outer - q_inner·cos) × l_k`
- w_k가 외륜 gap 감소 시 내륜 gap 증가 (반대 방향 작용)
- δ_o 결정: combined 모델에서 초기값 → secant 보정 (≤8 beam solve)
- 야코비안: `J = K_beam/1000 + diag((K_outer + K_inner) × l_k)`

**검증**: 비대칭 프로파일에서 q_outer/q_inner 비율이 0.61~1.18로 변동 (combined에서는 일정).
