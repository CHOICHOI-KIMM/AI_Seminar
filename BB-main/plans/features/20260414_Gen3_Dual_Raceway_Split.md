# Gen3 Dual-Raceway Split Model Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Gen3 beam solver에서 내/외륜 슬라이스 하중 q_k를 독립적으로 계산하여, 프로파일에 따라 내/외륜 접촉패치 형상이 다르게 나오도록 개선

**Architecture:** 현재 Gen3은 combined dual-Hertz 방정식으로 슬라이스당 하나의 q_k를 구함 (q_inner = q_outer·cos 고정). 개선 모델은 빔 양쪽에 독립 탄성 기초(inner/outer raceway)를 배치하여, 빔 변형 w_k가 외륜 접근량은 줄이고 내륜 접근량은 늘리는 (또는 반대) 물리를 반영. 추가 미지수 δ_split(강체 접근량의 내/외 분배)을 외부 NR 루프로 결정.

**Tech Stack:** Rust (nalgebra), existing hertz.rs/beam.rs modules

---

## Background: Why Split Matters

현재 combined 모델의 한계:
```
gap_combined_k = δ_rigid - w_k - Δz_outer - Δz_inner·cos
→ solve_q_from_dual_delta() → single q_k
→ q_inner = q_outer per slice (직렬 스프링 제약)
```

Split 모델이 달라지는 이유:
- 빔(롤러)이 노드 k에서 외륜 쪽으로 변형하면: 외륜 gap 감소(더 많은 하중), 내륜 gap 증가(더 적은 하중)
- 빔이 이 **차이력(net force)**을 내부적으로 전달 → 슬라이스 간 내/외 하중 재분배 가능
- 극단 사례: 내륜 flat + 외륜 heavy crown → 외륜은 균일 분포, 내륜은 edge loading

## Mathematical Formulation

### Variables
- `w_k` [μm]: 빔 변위 (2n DOF: w, θ per node), 기존과 동일
- `δ_o` [μm]: 외륜 강체 접근량 (NEW — 1 scalar)
- `δ_i` [μm]: 내륜 강체 접근량, `δ_i = (δ_rigid - δ_o) / cos(α_diff)`

### Gap equations (per node k)
```
gap_outer_k = δ_o - w_k - Δz_outer_k          [μm]
gap_inner_k = δ_i + w_k - Δz_inner_k          [μm]

(approximation: w projects equally onto inner/outer normals.
 Exact for CRB; good approximation for TRB with small cone angle difference.)
```

### Contact (per node k, per side)
```
if gap_outer_k > 0:
    q_outer_k = solve_q_from_delta(gap_outer_k/1000, R_eq_outer, E*)
else:
    q_outer_k = 0

if gap_inner_k > 0:
    q_inner_k = solve_q_from_delta(gap_inner_k/1000, R_eq_inner, E*)
else:
    q_inner_k = 0
```

### Beam equilibrium
```
K_beam · w/1000 = F_net

F_net[2k] = (q_outer_k - q_inner_k · cos_alpha_diff) · l_k
```

Outer contact pushes beam away from outer (positive w direction).
Inner contact pushes beam away from inner (negative w direction).

### Jacobian
```
J = K_beam/1000 + diag(K_contact_k)

K_contact_k = (k_tangent_outer_k + k_tangent_inner_k) · l_k / 1000
```

Both stiffnesses contribute positively (both resist beam deflection from equilibrium).

### Outer loop: δ_o determination
```
Residual: Σ q_outer_k · l_k - Σ q_inner_k · cos_alpha_diff · l_k = 0
(Global force balance on roller: total outer force = total inner force projected)

Solve for δ_o via Newton-Raphson (1D scalar).
Initial guess: δ_o from combined model's Hertz deformation at center slice.
```

### Verification invariant
```
δ_o + δ_i · cos_alpha_diff = δ_rigid  (always holds by construction)
Σ q_outer · l = Q_total (outer) = Q_total (inner) · cos  (at convergence)
```

---

## File Structure

```
Modified files:
  src-tauri/src/solver/types.rs      — SliceContactResult에 q_k_outer, q_k_inner 추가
  src-tauri/src/solver/hertz.rs      — compute_slice_contact_split() 신규 함수
  src-tauri/src/solver/gen3.rs       — 핵심 변경: split NR loop + δ_o outer loop
  src/types/bearing.ts               — TypeScript 타입 동기화
```

---

## Task 1: SliceContactResult에 q_k_outer / q_k_inner 필드 추가

**Files:**
- Modify: `src-tauri/src/solver/types.rs:830-850`
- Modify: `src/types/bearing.ts:179-195`

- [ ] **Step 1: Rust struct에 필드 추가**

`types.rs`의 SliceContactResult에 추가:

```rust
pub struct SliceContactResult {
    pub k: usize,
    pub delta_k: f64,          // approach amount [μm] (combined, for backward compat)
    pub q_k: f64,              // load per unit length [N/mm] (outer normal, backward compat)
    pub q_k_outer: f64,        // outer raceway line load [N/mm] (NEW)
    pub q_k_inner: f64,        // inner raceway line load [N/mm] (NEW)
    // ... existing inner/outer contact fields unchanged ...
}
```

기존 `q_k`는 backward compatibility를 위해 유지. 기존 combined 모델에서는 `q_k_outer = q_k`, `q_k_inner = q_k * cos_alpha_diff`.

- [ ] **Step 2: compute_slice_contact()에서 새 필드 초기화**

`hertz.rs`의 `compute_slice_contact()` 함수 내 SliceContactResult 반환부에서:

```rust
// No-contact case (delta_k <= 0):
q_k_outer: 0.0,
q_k_inner: 0.0,

// Contact case:
q_k_outer: q_k,               // combined model: outer = q_k
q_k_inner: q_k_inner,         // combined model: inner = q_k * cos
```

- [ ] **Step 3: TypeScript 타입 동기화**

`src/types/bearing.ts`의 SliceContactResult에 추가:

```typescript
export interface SliceContactResult {
  // ... existing ...
  q_k_outer: number;    // NEW
  q_k_inner: number;    // NEW
}
```

- [ ] **Step 4: cargo test로 기존 테스트 통과 확인**

Run: `cd d:/SW/TRB/src-tauri && cargo test`
Expected: 모든 기존 테스트 PASS (backward compatible 변경)

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/solver/types.rs src-tauri/src/solver/hertz.rs src/types/bearing.ts
git commit -m "feat: add q_k_outer/q_k_inner fields to SliceContactResult"
```

---

## Task 2: hertz.rs에 단일 궤도면 접촉 계산 헬퍼 추가

**Files:**
- Modify: `src-tauri/src/solver/hertz.rs`

현재 `solve_q_from_delta()`는 private. Gen3 split에서 각 면의 q를 독립적으로 구해야 하므로 public으로 노출 + 결과 구조체 헬퍼 추가.

- [ ] **Step 1: solve_q_from_delta를 pub으로 변경**

```rust
// 기존: fn solve_q_from_delta(...)
// 변경:
pub fn solve_q_from_delta(delta: f64, r_eq: f64, e_star: f64) -> f64 {
```

- [ ] **Step 2: 단일면 접촉 결과 계산 함수 추가**

```rust
/// Compute contact results for a SINGLE raceway given approach delta [μm].
/// Returns (q, b, p_max, h_bulk, k_hertz, k_bulk).
/// Returns all zeros if delta <= 0 (no contact).
pub fn single_raceway_contact(
    delta_um: f64,
    r_eq: f64,
    e_star: f64,
    e: f64,
    nu: f64,
    h1: f64,
    h2: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    if delta_um <= 0.0 {
        return (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    let delta_mm = delta_um / 1000.0;
    let q = solve_q_from_delta(delta_mm, r_eq, e_star);
    let b = hertz_half_width(q, r_eq, e_star);
    let p_max = hertz_max_pressure(q, b);
    let h_bulk = weber_bulk_deformation(q, e, nu, b, h1, h2);
    let dq = 0.001 * q.max(1.0);
    let k_hertz = tangent_stiffness(q, dq, r_eq, e_star);
    let k_bulk = if h_bulk > 0.0 { q / h_bulk } else { 0.0 };
    (q, b, p_max, h_bulk, k_hertz, k_bulk)
}
```

- [ ] **Step 3: 테스트 추가**

```rust
#[test]
fn test_single_raceway_contact_consistency() {
    let e_star_gpa = combined_elastic_modulus(210.0, 0.3, 210.0, 0.3);
    let e_star_mpa = e_star_gpa * 1000.0;
    let delta_um = 5.0;
    let r_eq = 5.0;

    let (q, b, p_max, _, _, _) = single_raceway_contact(
        delta_um, r_eq, e_star_mpa, 210000.0, 0.3, 5.0, 10.0,
    );

    // Verify round-trip: q → delta → q
    let delta_recovered = hertz_approach(q, r_eq, e_star_mpa) * 1000.0; // mm→μm
    assert!((delta_recovered - delta_um).abs() / delta_um < 0.01);
    assert!(b > 0.0);
    assert!(p_max > 0.0);
}

#[test]
fn test_single_raceway_no_contact() {
    let e_star = 115000.0;
    let (q, b, p_max, _, _, _) = single_raceway_contact(-1.0, 5.0, e_star, 210000.0, 0.3, 5.0, 10.0);
    assert_eq!(q, 0.0);
    assert_eq!(b, 0.0);
    assert_eq!(p_max, 0.0);
}
```

- [ ] **Step 4: cargo test 통과 확인**

Run: `cd d:/SW/TRB/src-tauri && cargo test hertz`
Expected: 기존 + 신규 테스트 모두 PASS

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/solver/hertz.rs
git commit -m "feat: add single_raceway_contact helper for split model"
```

---

## Task 3: Gen3 solver에 split 모드 구현 (핵심)

**Files:**
- Modify: `src-tauri/src/solver/gen3.rs`

이 Task가 핵심. 새 함수 `solve_gen3_roller_split()`를 추가하되 기존 `solve_gen3_roller()`는 그대로 유지 (backward compat).

- [ ] **Step 1: solve_gen3_roller_split 함수 시그니처**

```rust
/// Gen3 beam-coupled solver with independent inner/outer contact.
///
/// Unlike `solve_gen3_roller` (combined dual-Hertz per slice),
/// this solver computes q_outer_k and q_inner_k independently at each node.
/// The beam carries the NET force difference between inner and outer.
/// An additional scalar δ_o (outer rigid approach) is solved via
/// outer Newton-Raphson loop to satisfy global roller force balance.
///
/// Returns (slice_results, Q_total_outer [N]).
pub fn solve_gen3_roller_split(
    slices: &[SliceGeometry],
    delta_rigid: f64,
    material: &Material,
    params: &SolverParams,
    cos_alpha_diff: f64,
) -> Result<(Vec<SliceContactResult>, f64), SolverError> {
```

- [ ] **Step 2: δ_o 초기값 계산**

```rust
    // Initial guess for δ_o: use combined model's outer Hertz deformation at center slice
    let center = slices.len() / 2;
    let gap_center = delta_rigid - slices[center].delta_z_total_outer
        - slices[center].delta_z_total_inner * cos_alpha_diff;
    // Solve combined to get nominal split
    let q_nom = hertz::solve_q_from_dual_delta(
        gap_center.max(0.0) / 1000.0,
        slices[center].r_eq_inner, slices[center].r_eq_outer,
        e_star_mpa, cos_alpha_diff,
    );
    let mut delta_o = hertz::hertz_approach(q_nom, slices[center].r_eq_outer, e_star_mpa) * 1000.0;
    // δ_o is in μm, represents outer rigid approach
```

- [ ] **Step 3: 외부 NR 루프 (δ_o 결정)**

```rust
    let max_split_iters = 20;
    let split_tol = 1e-4; // relative tolerance for force balance

    for _split_iter in 0..max_split_iters {
        let delta_i = if cos_alpha_diff.abs() > 1e-12 {
            (delta_rigid - delta_o) / cos_alpha_diff
        } else {
            0.0
        };

        // Inner beam NR loop (same structure as current, but with split gaps)
        let (results, q_total_outer, q_total_inner) =
            solve_beam_with_split(slices, delta_o, delta_i, material, params,
                                  e_star_mpa, e_avg_mpa, cos_alpha_diff, &k_beam)?;

        // Check global force balance: Q_outer = Q_inner · cos
        let force_residual = q_total_outer - q_total_inner * cos_alpha_diff;
        let force_norm = q_total_outer.max(q_total_inner * cos_alpha_diff).max(1.0);

        if (force_residual / force_norm).abs() < split_tol {
            return Ok((results, q_total_outer));
        }

        // Numerical derivative: perturb δ_o
        let h = 0.01; // μm
        let (_, q_out_p, q_in_p) =
            solve_beam_with_split(slices, delta_o + h, delta_i - h / cos_alpha_diff,
                                  material, params, e_star_mpa, e_avg_mpa,
                                  cos_alpha_diff, &k_beam)?;
        let f_plus = q_out_p - q_in_p * cos_alpha_diff;
        let df = (f_plus - force_residual) / h;

        if df.abs() > 1e-20 {
            let delta_o_new = delta_o - force_residual / df;
            delta_o = delta_o_new.clamp(0.01, delta_rigid - 0.01);
        }
    }
```

- [ ] **Step 4: 내부 빔 NR 함수 (solve_beam_with_split)**

```rust
/// Inner beam NR loop with split inner/outer contacts.
/// Returns (slice_results, Q_total_outer, Q_total_inner).
fn solve_beam_with_split(
    slices: &[SliceGeometry],
    delta_o: f64,           // outer rigid approach [μm]
    delta_i: f64,           // inner rigid approach [μm]
    material: &Material,
    params: &SolverParams,
    e_star_mpa: f64,
    e_avg_mpa: f64,
    cos_alpha_diff: f64,
    k_beam: &DMatrix<f64>,
) -> Result<(Vec<SliceContactResult>, f64, f64), SolverError> {
    let n = slices.len();
    let ndof = 2 * n;
    let mut w = DVector::zeros(ndof);
    let h1_h2 = |s: &SliceGeometry| (s.r_roller, s.r_roller * 2.0);

    for _nr_iter in 0..params.max_iterations {
        let mut f_net = DVector::zeros(ndof);
        let mut contact_stiffness = vec![0.0_f64; n];

        for k in 0..n {
            let (h1, h2) = h1_h2(&slices[k]);

            // Outer gap and contact
            let gap_outer = delta_o - w[2 * k] - slices[k].delta_z_total_outer;
            let (q_o, _, _, _, k_h_o, _) = hertz::single_raceway_contact(
                gap_outer, slices[k].r_eq_outer, e_star_mpa, e_avg_mpa, material.nu, h1, h2,
            );

            // Inner gap and contact
            let gap_inner = delta_i + w[2 * k] - slices[k].delta_z_inner;
            let (q_i, _, _, _, k_h_i, _) = hertz::single_raceway_contact(
                gap_inner, slices[k].r_eq_inner, e_star_mpa, e_avg_mpa, material.nu, h1, h2,
            );

            // Net force on beam node: outer pushes +w, inner pushes -w
            f_net[2 * k] = (q_o - q_i * cos_alpha_diff) * slices[k].slice_width;

            // Both stiffnesses resist deflection (positive diagonal)
            contact_stiffness[k] = (k_h_o + k_h_i) * slices[k].slice_width;
        }

        // Residual: R = K_beam · w/1000 - F_net
        let w_mm = &w * 1e-3;
        let residual = k_beam * &w_mm - &f_net;

        // Convergence check
        let f_norm = f_net.norm().max(1.0);
        if residual.norm() / f_norm < params.convergence_tol {
            return build_split_result(slices, &w, delta_o, delta_i,
                                      material, e_star_mpa, e_avg_mpa, cos_alpha_diff);
        }

        // Jacobian: J = K_beam/1000 + diag(K_contact)
        let mut jacobian = k_beam * 1e-3;
        for k in 0..n {
            jacobian[(2 * k, 2 * k)] += contact_stiffness[k];
        }

        // Newton step
        let dw = jacobian.clone().lu().solve(&(-&residual))
            .ok_or_else(|| SolverError::ConvergenceFailure("Singular Jacobian in split solver".into()))?;
        w += &dw;

        remove_rigid_body_modes(&mut w, slices);
    }

    build_split_result(slices, &w, delta_o, delta_i,
                       material, e_star_mpa, e_avg_mpa, cos_alpha_diff)
}
```

- [ ] **Step 5: build_split_result 함수**

```rust
/// Build final results with independent inner/outer q values.
fn build_split_result(
    slices: &[SliceGeometry],
    w: &DVector<f64>,
    delta_o: f64,
    delta_i: f64,
    material: &Material,
    e_star_mpa: f64,
    e_avg_mpa: f64,
    cos_alpha_diff: f64,
) -> Result<(Vec<SliceContactResult>, f64, f64), SolverError> {
    let n = slices.len();
    let mut results = Vec::with_capacity(n);
    let mut q_total_outer = 0.0;
    let mut q_total_inner = 0.0;

    for k in 0..n {
        let w_k = if w.len() > 2 * k { w[2 * k] } else { 0.0 };
        let (h1, h2) = (slices[k].r_roller, slices[k].r_roller * 2.0);

        let gap_outer = delta_o - w_k - slices[k].delta_z_total_outer;
        let (q_o, b_o, p_o, hb_o, kh_o, kb_o) = hertz::single_raceway_contact(
            gap_outer, slices[k].r_eq_outer, e_star_mpa, e_avg_mpa, material.nu, h1, h2,
        );

        let gap_inner = delta_i + w_k - slices[k].delta_z_total_inner;
        let (q_i, b_i, p_i, hb_i, kh_i, kb_i) = hertz::single_raceway_contact(
            gap_inner, slices[k].r_eq_inner, e_star_mpa, e_avg_mpa, material.nu, h1, h2,
        );

        let in_contact = q_o > 0.0 || q_i > 0.0;
        if q_o > 0.0 { q_total_outer += q_o * slices[k].slice_width; }
        if q_i > 0.0 { q_total_inner += q_i * slices[k].slice_width; }

        // Combined stiffness for backward compat
        let mut inv_k = 0.0;
        if kh_i > 0.0 { inv_k += 1.0 / kh_i; }
        if kb_i > 0.0 { inv_k += 1.0 / kb_i; }
        if kh_o > 0.0 { inv_k += 1.0 / kh_o; }
        if kb_o > 0.0 { inv_k += 1.0 / kb_o; }
        let k_combined = if inv_k > 0.0 { 1.0 / inv_k } else { 0.0 };

        results.push(SliceContactResult {
            k,
            delta_k: gap_outer + gap_inner * cos_alpha_diff, // combined for compat
            q_k: q_o,              // backward compat: outer load
            q_k_outer: q_o,        // NEW: independent outer
            q_k_inner: q_i,        // NEW: independent inner
            b_k: b_i,
            p_max_k: p_i,
            h_bulk_k: hb_i,
            k_hertz_k: kh_i,
            k_bulk_k: kb_i,
            b_k_outer: b_o,
            p_max_k_outer: p_o,
            h_bulk_k_outer: hb_o,
            k_hertz_k_outer: kh_o,
            k_bulk_k_outer: kb_o,
            k_combined_k: k_combined,
            in_contact,
        });
    }

    Ok((results, q_total_outer, q_total_inner))
}
```

- [ ] **Step 6: cargo test 통과 확인**

Run: `cd d:/SW/TRB/src-tauri && cargo test gen3`
Expected: 기존 테스트 PASS (기존 함수 미변경), 신규 함수는 Task 4에서 테스트

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/solver/gen3.rs
git commit -m "feat: Gen3 split solver — independent inner/outer q_k per slice"
```

---

## Task 4: Split 모델 검증 테스트

**Files:**
- Modify: `src-tauri/src/solver/gen3.rs` (tests module)

- [ ] **Step 1: 대칭 프로파일 테스트 — split ≈ combined**

프로파일이 내/외 동일하고 R_eq도 동일하면, split 모델과 combined 모델의 결과가 일치해야 함.

```rust
#[test]
fn test_split_symmetric_matches_combined() {
    let (slices, mat) = make_test_slices(20, 3.0);
    let params = SolverParams::default();

    let (combined_results, q_combined) =
        solve_gen3_roller(&slices, 8.0, &mat, &params, 0.0).unwrap();
    let (split_results, q_split) =
        solve_gen3_roller_split(&slices, 8.0, &mat, &params, 0.0).unwrap();

    let rel_diff = (q_split - q_combined).abs() / q_combined;
    assert!(
        rel_diff < 0.05,
        "Symmetric: split Q={q_split:.1} vs combined Q={q_combined:.1}, diff={:.2}%",
        rel_diff * 100.0
    );

    // Per-slice loads should be close
    for k in 0..slices.len() {
        if combined_results[k].in_contact {
            let q_diff = (split_results[k].q_k_outer - combined_results[k].q_k).abs();
            let q_ref = combined_results[k].q_k.max(1.0);
            assert!(
                q_diff / q_ref < 0.1,
                "Slice {k}: split q_o={:.2} vs combined q={:.2}",
                split_results[k].q_k_outer, combined_results[k].q_k
            );
        }
    }
}
```

- [ ] **Step 2: 비대칭 프로파일 테스트 — 내/외 분포 차이 검증**

내륜 flat + 외륜 heavy crown → 외륜 q가 더 균일하고 내륜 q가 edge-loaded.

```rust
fn make_asymmetric_slices(n: usize) -> (Vec<SliceGeometry>, Material) {
    let l_we = 15.0;
    let slice_width = l_we / n as f64;
    let slices: Vec<SliceGeometry> = (0..n)
        .map(|k| {
            let x = (k as f64 + 0.5) * slice_width;
            let frac = x / l_we;
            let r_roller = 4.0 + frac;
            let r_race_inner = 100.0; // larger inner raceway radius
            let r_race_outer = 200.0; // different outer raceway radius
            let x_centered = x - l_we / 2.0;
            let dz_outer = 5.0 * (2.0 * x_centered / l_we).powi(2); // heavy outer crown
            let dz_inner = 0.0; // flat inner
            SliceGeometry {
                k,
                x_axial: x,
                r_roller,
                r_inner_race: r_race_inner,
                r_outer_race: r_race_outer,
                r_eq_inner: (r_roller * r_race_inner) / (r_roller + r_race_inner),
                r_eq_outer: (r_roller * r_race_outer) / (r_roller + r_race_outer),
                delta_z_total_inner: dz_inner,
                delta_z_total_outer: dz_outer,
                slice_width,
            }
        })
        .collect();
    (slices, Material::default())
}

#[test]
fn test_split_asymmetric_profiles() {
    let (slices, mat) = make_asymmetric_slices(20);
    let params = SolverParams::default();

    let (results, _q_total) =
        solve_gen3_roller_split(&slices, 10.0, &mat, &params, 0.0).unwrap();

    // Collect outer and inner loads for slices in contact
    let q_outer: Vec<f64> = results.iter()
        .filter(|r| r.q_k_outer > 0.0)
        .map(|r| r.q_k_outer)
        .collect();
    let q_inner: Vec<f64> = results.iter()
        .filter(|r| r.q_k_inner > 0.0)
        .map(|r| r.q_k_inner)
        .collect();

    if !q_outer.is_empty() && !q_inner.is_empty() {
        let outer_ratio = q_outer.iter().cloned().fold(f64::MIN, f64::max)
            / q_outer.iter().cloned().fold(f64::MAX, f64::min);
        let inner_ratio = q_inner.iter().cloned().fold(f64::MIN, f64::max)
            / q_inner.iter().cloned().fold(f64::MAX, f64::min);

        // Outer (crowned) should be more uniform than inner (flat)
        assert!(
            outer_ratio < inner_ratio * 1.5,
            "Outer ratio={outer_ratio:.2} should be lower than inner ratio={inner_ratio:.2}"
        );
    }

    // Inner and outer should have different distributions
    // (not just scaled versions of each other)
    let n_contact = results.iter().filter(|r| r.q_k_outer > 0.0 && r.q_k_inner > 0.0).count();
    if n_contact >= 3 {
        let ratios: Vec<f64> = results.iter()
            .filter(|r| r.q_k_outer > 0.0 && r.q_k_inner > 0.0)
            .map(|r| r.q_k_outer / r.q_k_inner.max(1e-10))
            .collect();
        let ratio_variation = ratios.iter().cloned().fold(f64::MIN, f64::max)
            - ratios.iter().cloned().fold(f64::MAX, f64::min);
        assert!(
            ratio_variation > 0.01,
            "q_outer/q_inner ratio should vary across slices, variation={ratio_variation:.4}"
        );
    }
}
```

- [ ] **Step 3: 총 하중 보존 테스트**

```rust
#[test]
fn test_split_force_balance() {
    let (slices, mat) = make_asymmetric_slices(20);
    let params = SolverParams::default();

    let (results, q_total_outer) =
        solve_gen3_roller_split(&slices, 10.0, &mat, &params, 0.0).unwrap();

    let q_total_inner: f64 = results.iter()
        .map(|r| r.q_k_inner * slices[r.k].slice_width)
        .sum();

    // cos_alpha_diff = 0.0 (legacy) → Q_outer should equal Q_inner
    let rel_diff = (q_total_outer - q_total_inner).abs() / q_total_outer.max(1.0);
    assert!(
        rel_diff < 0.01,
        "Force balance: Q_outer={q_total_outer:.1} vs Q_inner={q_total_inner:.1}, diff={:.3}%",
        rel_diff * 100.0
    );
}
```

- [ ] **Step 4: cargo test 전체 통과 확인**

Run: `cd d:/SW/TRB/src-tauri && cargo test`
Expected: 모든 테스트 PASS

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/solver/gen3.rs
git commit -m "test: Gen3 split model — symmetric, asymmetric, force balance tests"
```

---

## Task 5: Gen3 split 모드를 bearing solver에 통합

**Files:**
- Modify: `src-tauri/src/solver/bearing.rs:131-173`
- Modify: `src-tauri/src/solver/types.rs` (SolverParams에 split_mode 플래그)

- [ ] **Step 1: SolverParams에 split 모드 플래그 추가**

```rust
// types.rs SolverParams:
#[serde(default)]
pub use_split_contact: bool,  // true = Gen3 split model, false = combined (default)
```

- [ ] **Step 2: bearing.rs에서 split 모드 분기**

```rust
// bearing.rs, 롤러 솔버 호출부:
let (sr, qn) = if input.solver.use_split_contact {
    gen3::solve_gen3_roller_split(slices, delta_eff, &input.material, &input.solver, cos_alpha_diff)?
} else {
    gen1::solve_gen1_roller(slices, delta_eff, &input.material, cos_alpha_diff)
};
```

- [ ] **Step 3: TypeScript SolverParams 동기화**

```typescript
// bearing.ts:
export interface SolverParams {
  // ... existing ...
  use_split_contact?: boolean;  // NEW
}
```

- [ ] **Step 4: cargo test + npm run build 통과 확인**

Run: `cd d:/SW/TRB/src-tauri && cargo test`
Run: `cd d:/SW/TRB && npm run build`
Expected: 모두 PASS

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/solver/types.rs src-tauri/src/solver/bearing.rs src/types/bearing.ts
git commit -m "feat: integrate Gen3 split model into bearing solver via use_split_contact flag"
```

---

## Task 6: 프론트엔드 차트에 내/외 q_k 분리 표시

**Files:**
- Modify: `src/components/charts/LoadDistChart.tsx`

- [ ] **Step 1: LoadDistChart에서 q_k_outer/q_k_inner 표시**

기존 `q_k` 단일 trace → `q_k_outer` (실선) + `q_k_inner` (점선) 두 trace로 분리 표시.
`q_k_outer`와 `q_k_inner`가 동일하면 (combined 모드) 단일 trace로 fallback.

```tsx
// 차이가 있는지 확인
const hasSplitData = sliceResults.some(s =>
  Math.abs(s.q_k_outer - s.q_k_inner) > 0.01 * Math.max(s.q_k_outer, s.q_k_inner, 1)
);

if (hasSplitData) {
  // Two traces: outer (solid) + inner (dashed)
  traces.push({
    x: xValues,
    y: sliceResults.map(s => s.q_k_outer),
    name: 'q_k outer',
    line: { dash: 'solid' },
  });
  traces.push({
    x: xValues,
    y: sliceResults.map(s => s.q_k_inner),
    name: 'q_k inner',
    line: { dash: 'dash' },
  });
} else {
  // Single trace (backward compat)
  traces.push({
    x: xValues,
    y: sliceResults.map(s => s.q_k),
    name: 'q_k',
  });
}
```

- [ ] **Step 2: 테이블에 q_k_outer / q_k_inner 열 추가**

기존 `q_k` 열 옆에 `q_outer` / `q_inner` 열 추가 (split 모드일 때만 표시).

- [ ] **Step 3: npm run build 통과 확인**

Run: `cd d:/SW/TRB && npm run build`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add src/components/charts/LoadDistChart.tsx
git commit -m "feat: display split q_k_outer/q_k_inner in load distribution chart"
```

---

## Verification Checklist

- [ ] `cargo test` — 전체 통과
- [ ] `cargo clippy` — 경고 없음
- [ ] `npm run build` — TypeScript + Vite 빌드 성공
- [ ] Level C: flat profile + symmetric R_eq → split ≈ combined (< 5% 차이)
- [ ] Asymmetric profile → 내/외 q 분포 차이 확인 (ratio variation > 0)
- [ ] Force balance: Σq_outer = Σq_inner · cos (< 1% 오차)

---

## Approximations & Future Work

1. **w projection 근사**: 빔 변위 w가 내/외 contact normal에 동일하게 투영된다고 가정. TRB에서 cone angle 차이가 크면 (α_o - α_i > 5°) 보정 필요.
2. **Active set**: 현재 구현은 active set 관리를 단순화. 내/외 contact zone이 다를 수 있으므로 outer_active[k] ≠ inner_active[k] 지원 필요.
3. **Gen1 확장**: Gen1에 강체 롤러 모델(Gen2) 적용하여 내/외 분리. 이는 별도 플랜으로.
4. **Bearing-level 영향**: split 모델이 총 하중 Q_total에 미치는 영향은 작을 수 있으나, p_max 분포와 수명 계산에는 유의미한 차이.
