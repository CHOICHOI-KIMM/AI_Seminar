# Chapter 6: Gen1 — Independent Slice Solver

## 6.1 Concept

Gen1 모드에서는 각 슬라이스를 **독립적인 비선형 스프링**으로 취급한다. 슬라이스 간 상호작용(빔 굽힘, 전단)은 고려하지 않는다.

```
  δ_rigid ──────────────────── (uniform rigid approach)
     │    │    │    │    │
     ↓    ↓    ↓    ↓    ↓
    ╔═╗  ╔═╗  ╔═╗  ╔═╗  ╔═╗   ← nonlinear springs (Hertz)
    ╚═╝  ╚═╝  ╚═╝  ╚═╝  ╚═╝
     │    │    │    │    │
  ───────────────────────────  (raceway, rigid)
```

**장점**: O(n) 연산, 수렴성 우수, Parametric sweep에 적합
**단점**: 에지 응력 과소/과대평가, 롤러 굽힘 효과 무시

## 6.2 Single Roller Solve

주어진 강체 접근량 `δ_rigid` [μm]에서 전 슬라이스의 접촉 해석:

### Algorithm (Dual-Raceway Model)

```
cos_α_diff = cos(α_o - α_i)

FOR each slice k = 0, 1, ..., n-1:
    δ_available_k = δ_rigid - Δz_total_outer_k
                    - Δz_total_inner_k × cos_α_diff    [μm]

    IF δ_available_k > 0:
        q_k = solve_q_dual(δ_available_k, R_eq_inner, R_eq_outer, cos_α_diff)
              [N/mm] (Dual-raceway NR, Ch.5 §5.4.2)
        b_k = hertz_half_width(q_k)      [mm]
        p_max_k = hertz_max_pressure(q_k, b_k)  [MPa]
        Q_k = q_k × l_k                  [N]
    ELSE:
        q_k = b_k = p_max_k = Q_k = 0    (no contact)

Q_total = Σ Q_k                           [N]
```

> **참고**: inner raceway 프로파일이 이제 `δ_available_k` 계산에 반영되므로, inner 프로파일 변경 시 contact path(접촉 슬라이스 분포)가 달라진다.

### Total Roller Load

$$
Q_{total} = \sum_{k=0}^{n-1} q_k \cdot l_k
$$

여기서 `l_k = L_we / n` (슬라이스 폭).

## 6.3 Newton-Raphson for Target Load

Level 2에서는 외부에서 주어진 목표 하중 `Q_target` [N]에 대해 `δ_rigid`를 역산해야 한다:

### Problem Statement

Find `δ_rigid` such that `Q_total(δ_rigid) = Q_target`

### Algorithm

```
δ_rigid ← 5.0 μm (initial guess)
Δδ = 0.01 μm (perturbation for numerical derivative)

FOR iter = 0, 1, ..., max_iterations:
    (results, Q_calc) = solve_gen1_roller(slices, δ_rigid)

    residual = Q_calc - Q_target

    IF |residual / Q_target| < tol:
        RETURN (results, Q_calc, δ_rigid)  ← converged

    // Numerical derivative dQ/dδ
    (_, Q_plus) = solve_gen1_roller(slices, δ_rigid + Δδ)
    dQ_dδ = (Q_plus - Q_calc) / Δδ

    IF |dQ_dδ| < 10⁻²⁰:
        δ_rigid ← 2 × δ_rigid             ← stiffness too low, increase
        CONTINUE

    δ_rigid ← max(0.01, δ_rigid - residual / dQ_dδ)

ERROR: convergence failure
```

### Convergence Behavior

- **단조 증가**: `δ_rigid ↑` → `Q_total ↑` (Hertz 비선형성: 지수 10/9)
- **초기 추정**: 5 μm이 일반적인 TRB에서 합리적 출발점
- **수렴 속도**: Newton-Raphson이므로 2차 수렴, 일반적으로 5~15회 반복

## 6.4 Load Distribution Characteristics

### Flat Profile (Δz = 0)

크라운이 없는 평탄 프로파일에서는 모든 슬라이스의 δ_k가 동일하므로, 하중 분포가 거의 균일하다. 단, 테이퍼로 인한 R_eq 변화 때문에 약간의 차이가 존재한다.

### Crowned Profile (Δz > 0 at edges)

크라운 프로파일에서는:
- 중앙 슬라이스: `δ_k ≈ δ_rigid` → 최대 하중
- 에지 슬라이스: `δ_k = δ_rigid - Δz` → 감소된 하중 또는 비접촉
- **Partial contact**: 큰 크라운에서 소접근량 시 → 중앙만 접촉, 에지 비접촉

```
q_k distribution:

   ┌──────────┐
   │  ────────│── flat profile (edge stress!)
   │ ╱      ╲ │
   │╱        ╲│── crowned profile (smooth)
   └──────────┘
   Small    Large
   end      end
```

## 6.5 Gen1 Limitations

1. **Edge stress**: 평탄 프로파일에서 에지 접촉 응력 집중을 정확히 예측할 수 없음
2. **Load redistribution**: 롤러 굽힘에 의한 하중 재분배를 무시
3. **Profile sensitivity**: 에지 dub-off의 효과를 과대평가하는 경향
4. **Misalignment**: 미정렬 시 프로파일 민감도가 Gen3 대비 부정확

이러한 한계는 Gen3 (beam-coupled) 모드에서 해결된다 (Chapter 7 참조).

## 6.6 Gen1 Split Contact

`use_split_contact = true`이면 Gen1에서도 내/외륜 하중을 독립 계산한다 (Chapter 17 참조).

- 강체 롤러 모델: δ_o (외륜 접근량 분배)를 secant iteration으로 결정
- 각 슬라이스에서 `gap_outer = δ_o - Δz_outer`, `gap_inner = δ_i - Δz_inner`
- q_outer_k ≠ q_inner_k (비대칭 프로파일/R_eq에서 차이 발생)
- 빔 결합 없음 → Gen1과 동일한 계산 속도
