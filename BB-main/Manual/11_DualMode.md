# Chapter 11: Dual-Mode Comparison

## 11.1 Concept

Dual 모드는 Gen1과 Gen3의 결과를 직접 비교하여, Gen3 사용의 필요성을 자동 판단한다. 실행 전략은 다음과 같다:

```
Step 1: Gen1 Equilibrium
  ├─ 5-DOF Newton-Raphson (fast)
  └─ 수렴된 변위 δ* 획득

Step 2: Gen3 Re-evaluation
  ├─ δ*에서 각 롤러를 Gen3로 재해석
  └─ 별도 수명 계산

Step 3: Build Comparison
  ├─ 응력, 하중, 수명 차이 비교
  └─ 자동 추천 생성
```

**핵심**: Gen3는 평형 루프 내부에서 실행하지 않고, Gen1의 수렴된 변위에서 재평가만 수행한다. 이로써 Gen3의 정밀성을 얻으면서도, 5-DOF 수렴의 계산 비용을 절약한다.

## 11.2 Re-evaluation Process

Gen1 수렴 변위 `δ* = [δx, δy, δz, γx, γy]`에서:

```
FOR each roller j (ψ_j):
    δ_rigid_j = roller_approach(δ*, ψ_j, α, d_pw, G_r)

    IF δ_rigid_j > 0:
        (slice_results, Q_j) = solve_gen3_roller(slices, δ_rigid_j, material, params)
        rib_j = compute_rib_contact(Q_j × sin(α))
    ELSE:
        Q_j = 0, no contact
```

## 11.3 Comparison Metrics

### 11.3.1 Maximum Contact Stress Difference

$$
\Delta p_{max}\% = \frac{p_{max,Gen3} - p_{max,Gen1}}{p_{max,Gen1}} \times 100
$$

여기서 `p_max = max(p_max_k)` across all rollers and slices.

### 11.3.2 Maximum Roller Load Difference

$$
\Delta Q_{max}\% = \frac{Q_{max,Gen3} - Q_{max,Gen1}}{Q_{max,Gen1}} \times 100
$$

### 11.3.3 Life Difference

$$
\Delta L_{10}\% = \frac{L_{10,Gen3} - L_{10,Gen1}}{L_{10,Gen1}} \times 100
$$

## 11.4 Recommendation Logic

다음 조건 중 하나라도 만족하면 Gen3 사용을 권장한다:

| Condition | Threshold | Rationale |
|-----------|-----------|-----------|
| `|Δp_max%| > 5%` | 5% | 유의한 응력 차이 |
| `|ΔQ_max%| > 3%` | 3% | 유의한 하중 차이 |
| Edge stress rise > 1.2 | 1.2 | 에지 응력 집중 감지 |
| Gen3 alerts > Gen1 alerts | count | 추가 경고 발견 |

### Edge Stress Rise Detection

접촉 중인 롤러에서 에지/중앙 응력 비:

$$
f_{edge} = \max_j \frac{\max(p_{max,0}, p_{max,n-1})}{p_{max,center}}
$$

`f_edge > 1.2`이면 빔 커플링 효과가 유의하다.

### Recommendation Output

| gen3_recommended | recommendation_reason |
|------------------|----------------------|
| `false` | "Gen1 is sufficient: results agree within tolerance" |
| `true` | "p_max differs by X.X%; Edge stress rise factor Y.YY detected; ..." |

## 11.5 Dual Mode Output

```
DualModeComparison {
    gen1_result: BearingResult,     // Complete Gen1 results
    gen3_result: BearingResult,     // Complete Gen3 results
    delta_p_max_pct: f64,           // Stress difference [%]
    delta_q_max_pct: f64,           // Load difference [%]
    delta_l10_pct: f64,             // Life difference [%]
    gen3_recommended: bool,         // Auto recommendation
    recommendation_reason: String,  // Human-readable reason
}
```

## 11.6 Interpretation Guidelines

### When Gen1 ≈ Gen3

- 잘 설계된 크라운 프로파일
- 낮은 하중/클리어런스 조합
- 무시할 수 있는 미정렬

→ Gen1이 충분하다. 빠른 Parametric sweep에 활용.

### When Gen3 > Gen1 (divergence)

- 공격적 크라운 프로파일 (높은 δ_c, 짧은 dub-off)
- 에지 접촉 발생 (Gen1에서 비접촉이나 Gen3에서 접촉)
- 미정렬 하에서 비대칭 하중
- 높은 하중 / 좁은 하중 영역

→ Gen3 결과를 최종 설계에 사용해야 한다.

## 11.7 Computational Cost Comparison

| Mode | Relative Cost | Description |
|------|--------------|-------------|
| Single Gen1 | 1× | Equilibrium with Gen1 only |
| Single Gen3 | 5~20× | Equilibrium with Gen3 (nested NR) |
| Dual | 2~5× | Gen1 equilibrium + Gen3 re-evaluation |

Dual 모드는 Gen3를 평형 루프에 넣지 않으므로, Single Gen3 대비 효율적이다. 다만 Gen1 변위에서의 재평가이므로, 변위가 달라질 수 있는 극한 조건에서는 차이가 있을 수 있다.
