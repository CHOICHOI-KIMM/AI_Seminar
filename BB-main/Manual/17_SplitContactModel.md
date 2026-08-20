# Chapter 17: Split Contact Model — Independent Inner/Outer Raceway Loads

## 17.1 Motivation

기존 combined 모델에서는 각 슬라이스의 하중 q_k가 내/외륜에서 동일하다 (직렬 스프링):

```
δ_hertz_outer(q) + δ_hertz_inner(q·cos)·cos = δ_available
→ q_outer = q_inner (per slice)
```

이 가정은 롤러를 독립 스프링으로 취급할 때는 물리적으로 정확하지만, **롤러가 강체 또는 빔으로 결합되면** 슬라이스 간 하중 전달이 가능하므로 내/외륜 하중 분포가 독립적으로 달라질 수 있다.

### Split 모델이 중요한 경우

| 시나리오 | 효과 |
|----------|------|
| 내륜 flat + 외륜 heavy crown | 외륜: crown에 의한 포물선 분포 / 내륜: 거의 균일 |
| R_eq_inner ≠ R_eq_outer (테이퍼 기하) | 같은 접근량에서 다른 접촉폭/응력 |
| 비대칭 프로파일 (측정 프로파일) | 내/외 접촉 존(contact zone)이 다름 |

## 17.2 Mathematical Formulation

### 17.2.1 Core Concept: δ_o Split

전체 강체 접근량 `δ_rigid` [μm]를 내/외륜으로 분배:

$$
\delta_o + \delta_i \cdot \cos(\alpha_o - \alpha_i) = \delta_{rigid}
$$

여기서:
- `δ_o` [μm]: 외륜 강체 접근량
- `δ_i` = `(δ_rigid - δ_o) / cos(α_o − α_i)` [μm]: 내륜 강체 접근량

### 17.2.2 Gap Equations

**Gen1 Split** (강체 롤러, 빔 결합 없음):

```
gap_outer_k = δ_o - Δz_total_outer_k        [μm]
gap_inner_k = δ_i - Δz_total_inner_k        [μm]
```

**Gen3 Split** (빔 결합, w_k = 굽힘 변위):

```
gap_outer_k = δ_o - w_k - Δz_total_outer_k                        [μm, 외륜 법선]
gap_inner_k = δ_i + w_k · cos(α_o - α_i) - Δz_total_inner_k      [μm, 내륜 법선]
```

> w_k는 빔 횡변위로 외륜 법선 방향에 가깝다. 내륜 법선 방향으로의 투영에 cos(α_o − α_i) 팩터가 필요.
> w_k가 외륜 gap을 줄이면, 내륜 gap은 cos 투영만큼 증가한다.
> 이것이 split 모델의 핵심: 빔 변형이 내/외 접촉에 반대 방향으로, 투영 비율에 맞게 작용.

### 17.2.3 Independent Contact per Side

각 슬라이스에서 내/외 접촉을 독립적으로 계산:

```
if gap_outer_k > 0:
    q_outer_k = solve_q(gap_outer_k, R_eq_outer, E*)    [N/mm]
if gap_inner_k > 0:
    q_inner_k = solve_q(gap_inner_k, R_eq_inner, E*)    [N/mm]
```

`solve_q`는 단일 궤도면 Hertz 접촉 (Ch.5 §5.4.1), `solve_q_from_dual_delta`가 아닌 `single_raceway_contact` 사용.

### 17.2.4 Force Balance (δ_o 결정)

전역 롤러 힘 평형:

$$
\sum_k q_{outer,k} \cdot l_k = \sum_k q_{inner,k} \cdot \cos(\alpha_o - \alpha_i) \cdot l_k
$$

이 조건에서 `δ_o`를 secant method로 결정한다.

## 17.3 Gen1 Split Solver

### Algorithm

```
INPUT: slices, δ_rigid, material, cos_α_diff

1. Initial guess: δ_o from combined model (Hertz deformation ratio)
   - 각 loaded 슬라이스에서 combined Hertz → 외륜 변형량의 하중 가중평균

2. Secant iteration (max 20회):
   δ_i = (δ_rigid - δ_o) / cos_α_diff
   
   FOR each slice k:
     gap_outer_k = δ_o - Δz_outer_k
     gap_inner_k = δ_i - Δz_inner_k
     q_outer_k = hertz(gap_outer_k, R_eq_outer)
     q_inner_k = hertz(gap_inner_k, R_eq_inner)
   
   Q_outer = Σ q_outer_k × l_k
   Q_inner = Σ q_inner_k × l_k
   
   force_residual = Q_outer - Q_inner × cos_α_diff
   IF converged: BREAK
   
   Secant update: δ_o ← δ_o - residual × Δδ / Δresidual
   Clamp: δ_o ∈ [0.01, 0.99 × δ_rigid]

OUTPUT: (slice_results, Q_total_outer)
```

**성능**: O(n × iters), 빔 없으므로 LU 불필요. Gen1 combined과 거의 동일한 속도.

## 17.4 Gen3 Split Solver

### Algorithm

```
INPUT: slices, δ_rigid, material, params, cos_α_diff

1. Initial δ_o from combined model (same as Gen1 split)

2. Secant refinement (max 8회):
   δ_i = (δ_rigid - δ_o) / cos_α_diff

   3. Beam NR (inner loop):
      FOR each NR iteration:
        FOR each slice k:
          gap_outer = δ_o - w_k - Δz_outer_k
          gap_inner = δ_i + w_k·cos(α_diff) - Δz_inner_k
          q_outer_k = hertz(gap_outer, R_eq_outer)
          q_inner_k = hertz(gap_inner, R_eq_inner)
          
          f_net[2k] = (q_outer_k - q_inner_k × cos) × l_k
          K_contact[k] = (K_outer + K_inner·cos²(α_diff)) × l_k
        
        R = K_beam × w/1000 - f_net
        J = K_beam/1000 + diag(K_contact)
        Δw = J⁻¹ × (-R)
        w += Δw
        remove_rigid_body_modes(w)
   
   4. Force balance check:
      IF |Q_outer - Q_inner × cos| / Q_max < tol: CONVERGED
      ELSE: secant update δ_o

OUTPUT: (slice_results, Q_total_outer)
```

### Beam Equation 차이점 (Combined vs Split)

| | Combined | Split |
|--|---------|-------|
| 접촉력 | `f[2k] = q_k × l_k` | `f[2k] = (q_outer - q_inner·cos) × l_k` |
| 강성 | `K_combined × l_k` | `(K_outer + K_inner·cos²) × l_k` |
| Gap | 단일 `gap_k` | 별도 `gap_outer`, `gap_inner` |
| w 영향 | gap 감소 | outer 감소, inner 증가 (반대!) |

## 17.5 Bearing Solver Integration

### Architecture

```
┌─────────────────────────────────────────┐
│ Bearing 5-DOF NR Loop                   │
│   compute_residual() → 항상 Gen1 (빠름) │
│   → δ_rigid per roller → Q_total       │
└─────────────┬───────────────────────────┘
              │ 수렴 후
              ▼
┌─────────────────────────────────────────┐
│ Final Re-evaluation (1회)               │
│ ┌─ Gen1 mode + split:                  │
│ │  reevaluate_with_gen1_split()        │
│ ├─ Gen3 mode + split:                  │
│ │  reevaluate_with_gen3() → split      │
│ └─ split OFF:                          │
│    기존 combined 결과 사용              │
└─────────────┬───────────────────────────┘
              │
              ▼
┌─────────────────────────────────────────┐
│ Angular Distribution 구축               │
│ - gen1 envelope (180점, 빠름)           │
│ - split 시: roller_results에서 보간     │
│   → slice_p_max, slice_q_k 대체        │
└─────────────────────────────────────────┘
```

> **설계 근거**: Q_total은 combined/split에서 동일하므로 (force balance), 평형 변위는 gen1으로 충분히 정확하다. Split은 **per-slice 내/외 분포**만 변경한다.

### Angular Distribution 보간

Split 활성 시, angular_distribution의 보간 점(롤러 사이)에서:

1. 인접 두 loaded roller를 찾는다 (`find_bracketing_rollers`, 원형 보간)
2. 두 roller의 slice_p_max, slice_q_k를 선형 보간 (파라미터 t)
3. gen1 Q envelope로 스케일링: `scale = Q_gen1(ψ) / Q_interp` (1.5배 상한)

이 방식으로 추가 Rust 계산 없이 고해상도 컨투어를 유지한다.

## 17.6 Result Structure

### SliceContactResult 필드

```rust
q_k: f64,           // outer normal load [N/mm] (backward compat)
q_k_outer: f64,     // outer raceway line load [N/mm] (independent)
q_k_inner: f64,     // inner raceway line load [N/mm] (independent)

// Inner raceway contact (from q_k_inner):
b_k, p_max_k, h_bulk_k, k_hertz_k

// Outer raceway contact (from q_k_outer):
b_k_outer, p_max_k_outer, h_bulk_k_outer, k_hertz_k_outer

// Combined slice stiffness along outer normal (Hertz-only 2-spring series):
k_combined_k  // 1/k = 1/k_hertz_k_outer + cos²(α_o−α_i)/k_hertz_k
```

> **Backward compatibility**: `q_k = q_k_outer` (외륜 법선 하중). Combined 모델에서는 `q_k_inner = q_k × cos(α_o - α_i)`.

## 17.7 Validation

### Test Results (TRB-realistic geometry, asymmetric profiles)

내륜 flat, 외륜 3μm parabolic crown, 30 슬라이스, cos_α_diff = 0.99:

| 검증 항목 | 결과 |
|-----------|------|
| Q_split vs Q_combined | 0.9% 차이 |
| Force balance | 0.00% 오차 |
| q_outer/q_inner 비율 | 0.61 ~ 1.18 (변동폭 0.57) |
| Combined 비율 CV | 0.000 (일정) |
| Split 비율 CV | 0.181 (변동) |
| 접촉폭 정합성 | b_k ↔ q_k < 1% |

### 물리적 검증

```
Slice  0 (edge):   q_outer=104, q_inner=168 → ratio=0.62
Slice 14 (center): q_outer=208, q_inner=177 → ratio=1.18
Slice 29 (edge):   q_outer=101, q_inner=166 → ratio=0.61

외륜 (crowned): CV=0.197 (변동 큼, crown에 의한 재분배)
내륜 (flat):    CV=0.020 (거의 균일, 빔 강성에 의한 평탄화)
```

## 17.8 UI

### 설정

InputPanel → Solver 섹션 → **Split contact** ON/OFF 토글

- Gen1 + Split ON → Gen1 split (빠름)
- Gen3 + Split ON → Gen3 split (re-evaluation에서 적용)
- Split OFF → 기존 combined 모델

### 표시

- **LoadDistChart**: Slice Detail 테이블에서 `q_o` / `q_i` 별도 열
- **StressContourChart**: Overview에서 내/외 p_max 컨투어 (보간된 고해상도)
- **RollerDetailChart**: 개별 롤러의 내/외 분리된 접촉 결과

## 17.9 Limitations & Future Work

1. **w 투영 근사**: 빔 변위 w가 내/외 contact normal에 동일하게 투영된다고 가정. TRB cone angle 차이 > 5°에서는 보정 필요.
2. **롤러 기울기 (tilt)**: 현재 δ_o만 (병진 1 DOF). 롤러 기울기 θ를 추가하면 2-DOF 강체 모델 (Gen2) 가능.
3. **단일면 접촉**: 극단 비대칭에서 한쪽만 접촉하는 슬라이스 발생 가능. `single_raceway_contact`는 gap ≤ 0에서 0을 반환하므로 처리됨.
4. **직렬 강성**: split에서도 `k_combined`는 4성분 직렬 합성. 한쪽만 접촉 시 의미가 달라지나, 분석 목적에는 허용 가능.
