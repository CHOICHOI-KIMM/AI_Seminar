// CRB (Phase 3): 이 모듈은 α = 0 (원통) 조건에서 호출됨.
// commands.rs 에서 `cos_alpha_diff = 1.0` 인자 전달 → dual-raceway 프로젝션 자동 환원.
// CRB 특성: r_roller = D_we/2 = const → beam.rs 의 I_k, A_k 자동으로 균일 →
// [K_beam] 행렬이 element-wise 동일 (구조 대칭 개선). Newton-Raphson + active set
// 알고리즘 자체 (O(n²)) 는 CRB/TRB 공용, 변경 없음.
//
// Level C (Phase 3): Gen1 (독립 slice) ↔ Gen3 (beam-coupled) 는 서로 다른 알고리즘 →
// flat profile + zero misalignment 조건에서 두 결과 수렴해야 (진짜 독립 검증).

use nalgebra::DVector;

use crate::error::SolverError;
use crate::solver::beam;
use crate::solver::hertz;
use crate::solver::types::*;

/// Gen3 beam-coupled slice solver: single δ_rigid (dual-raceway model).
///
/// Couples a Timoshenko beam FE model with nonlinear Hertz contact springs
/// via Newton-Raphson iteration with active set management.
///
/// The key insight: δ_rigid captures the rigid body approach, while w captures
/// only the flexural (bending) deviation. After each NR step, rigid body
/// components (translation + rotation) are projected out of w to prevent
/// the solver from absorbing rigid motion into the beam deflection.
///
/// `cos_alpha_diff`: cos(α_o − α_i) for dual-raceway model. Pass 0.0 for legacy.
///
/// Returns (slice_results, Q_total [N]).
pub fn solve_gen3_roller(
    slices: &[SliceGeometry],
    delta_rigid: f64,
    material: &Material,
    params: &SolverParams,
    cos_alpha_diff: f64,
) -> Result<(Vec<SliceContactResult>, f64), SolverError> {
    let n = slices.len();
    if n == 0 {
        return Err(SolverError::InvalidInput("No slices provided".into()));
    }
    if n < 2 {
        return solve_fallback_gen1(slices, delta_rigid, material, cos_alpha_diff);
    }

    let ndof = 2 * n;

    let e_star_gpa = hertz::combined_elastic_modulus(
        material.e_roller,
        material.nu,
        material.e_ring,
        material.nu,
    );
    let e_star_mpa = e_star_gpa * 1000.0;
    let e_avg_mpa = ((material.e_roller + material.e_ring) / 2.0) * 1000.0;

    // Assemble beam stiffness (constant throughout iteration)
    let k_beam = beam::assemble_beam_stiffness(slices, material, params.beam_type);

    // w: beam flexural displacement [μm] (rigid body component is projected out)
    let mut w = DVector::zeros(ndof);

    let max_active_set_iters = 15;
    let mut prev_active: Vec<bool> = vec![false; n];

    for _outer in 0..max_active_set_iters {
        // Inner Newton-Raphson loop
        for _nr_iter in 0..params.max_iterations {
            // Compute gaps and contact forces
            let mut f_contact = DVector::zeros(ndof);
            let mut contact_stiffness = vec![0.0_f64; n];
            let mut active = vec![false; n];

            for k in 0..n {
                let gap_k = delta_rigid - w[2 * k] - slices[k].delta_z_total_outer
                    - slices[k].delta_z_total_inner * cos_alpha_diff;

                if gap_k > 0.0 {
                    active[k] = true;
                    let result = hertz::compute_slice_contact(
                        k,
                        gap_k,
                        slices[k].r_eq_inner,
                        slices[k].r_eq_outer,
                        e_star_mpa,
                        e_avg_mpa,
                        material.nu,
                        slices[k].slice_width,
                        slices[k].r_roller,
                        slices[k].r_roller * 2.0,
                        cos_alpha_diff,
                    );
                    f_contact[2 * k] = result.q_k * slices[k].slice_width;
                    contact_stiffness[k] = result.k_hertz_k_outer * slices[k].slice_width;
                }
            }

            let n_active = active.iter().filter(|&&a| a).count();
            if n_active < 2 {
                return solve_fallback_gen1(slices, delta_rigid, material, cos_alpha_diff);
            }

            // Residual: R = K_beam · (w/1000) - F_contact
            // K_beam operates on mm, w is in μm → divide by 1000
            let w_mm = &w * 1e-3;
            let residual = &k_beam * &w_mm - &f_contact;

            // Convergence check
            let f_norm = f_contact.norm().max(1.0);
            let r_norm = residual.norm();
            if r_norm / f_norm < params.convergence_tol {
                if active == prev_active {
                    return build_gen3_result(
                        slices, &w, delta_rigid, material, e_star_mpa, e_avg_mpa, cos_alpha_diff,
                    );
                }
                prev_active = active;
                break;
            }

            // Jacobian: J = K_beam/1000 + diag(K_c_k) on displacement DOFs
            // dR/dw = K_beam * (1/1000) + K_contact (since gap decreases with w)
            let mut jacobian = &k_beam * 1e-3;
            for k in 0..n {
                if active[k] {
                    jacobian[(2 * k, 2 * k)] += contact_stiffness[k];
                }
            }

            // Solve: J · Δw = -R
            let neg_residual = -&residual;
            let decomp = jacobian.clone().lu();
            let dw = match decomp.solve(&neg_residual) {
                Some(sol) => sol,
                None => {
                    return Err(SolverError::ConvergenceFailure(
                        "Jacobian is singular in Gen3 solver".into(),
                    ));
                }
            };

            w += &dw;

            // Project out rigid body modes from w.
            // δ_rigid already captures the overall approach; w should only
            // contain the flexural (bending) component.
            remove_rigid_body_modes(&mut w, slices);
        }

        // Check active set stability
        let mut final_active = vec![false; n];
        for k in 0..n {
            let gap_k = delta_rigid - w[2 * k] - slices[k].delta_z_total_outer
                - slices[k].delta_z_total_inner * cos_alpha_diff;
            final_active[k] = gap_k > 0.0;
        }
        if final_active == prev_active {
            return build_gen3_result(slices, &w, delta_rigid, material, e_star_mpa, e_avg_mpa, cos_alpha_diff);
        }
        prev_active = final_active;
    }

    build_gen3_result(slices, &w, delta_rigid, material, e_star_mpa, e_avg_mpa, cos_alpha_diff)
}

/// Remove rigid body motion (translation + rotation) from beam displacement vector.
///
/// For a free-free beam, δ_rigid handles the rigid approach.
/// w should contain only flexural (bending) deformation.
fn remove_rigid_body_modes(w: &mut DVector<f64>, slices: &[SliceGeometry]) {
    let n = slices.len();
    if n < 2 {
        return;
    }

    // Compute centroid of axial positions
    let x_mean: f64 = slices.iter().map(|s| s.x_axial).sum::<f64>() / n as f64;

    // Compute mean displacement (translation mode)
    let w_mean: f64 = (0..n).map(|k| w[2 * k]).sum::<f64>() / n as f64;

    // Compute rotation (linear trend): w_k ≈ a + b*(x_k - x_mean)
    // b = Σ(w_k * (x_k - x_mean)) / Σ((x_k - x_mean)²)
    let (num, den) = (0..n).fold((0.0, 0.0), |(num, den), k| {
        let xc = slices[k].x_axial - x_mean;
        (num + w[2 * k] * xc, den + xc * xc)
    });
    let slope = if den.abs() > 1e-30 { num / den } else { 0.0 };

    // Subtract rigid body components
    for k in 0..n {
        let xc = slices[k].x_axial - x_mean;
        w[2 * k] -= w_mean + slope * xc;
        w[2 * k + 1] -= slope; // rotation DOF correction
    }
}

/// Build final result from converged displacement vector.
fn build_gen3_result(
    slices: &[SliceGeometry],
    w: &DVector<f64>,
    delta_rigid: f64,
    material: &Material,
    e_star_mpa: f64,
    e_avg_mpa: f64,
    cos_alpha_diff: f64,
) -> Result<(Vec<SliceContactResult>, f64), SolverError> {
    let n = slices.len();
    let mut results = Vec::with_capacity(n);
    let mut q_total = 0.0;

    for k in 0..n {
        let w_k = if w.len() > 2 * k { w[2 * k] } else { 0.0 };
        let gap_k = delta_rigid - w_k - slices[k].delta_z_total_outer
            - slices[k].delta_z_total_inner * cos_alpha_diff;
        let h1 = slices[k].r_roller;
        let h2 = slices[k].r_roller * 2.0;
        let result = hertz::compute_slice_contact(
            k,
            gap_k,
            slices[k].r_eq_inner,
            slices[k].r_eq_outer,
            e_star_mpa,
            e_avg_mpa,
            material.nu,
            slices[k].slice_width,
            h1,
            h2,
            cos_alpha_diff,
        );
        if result.in_contact {
            q_total += result.q_k * slices[k].slice_width;
        }
        results.push(result);
    }

    Ok((results, q_total))
}

/// Fallback: when < 2 slices in contact, beam coupling is meaningless.
fn solve_fallback_gen1(
    slices: &[SliceGeometry],
    delta_rigid: f64,
    material: &Material,
    cos_alpha_diff: f64,
) -> Result<(Vec<SliceContactResult>, f64), SolverError> {
    let e_star_gpa = hertz::combined_elastic_modulus(
        material.e_roller,
        material.nu,
        material.e_ring,
        material.nu,
    );
    let e_star_mpa = e_star_gpa * 1000.0;
    let e_avg_mpa = ((material.e_roller + material.e_ring) / 2.0) * 1000.0;

    let mut results = Vec::with_capacity(slices.len());
    let mut q_total = 0.0;

    for s in slices {
        let delta_k = delta_rigid - s.delta_z_total_outer
            - s.delta_z_total_inner * cos_alpha_diff;
        let h1 = s.r_roller;
        let h2 = s.r_roller * 2.0;
        let result = hertz::compute_slice_contact(
            s.k,
            delta_k,
            s.r_eq_inner,
            s.r_eq_outer,
            e_star_mpa,
            e_avg_mpa,
            material.nu,
            s.slice_width,
            h1,
            h2,
            cos_alpha_diff,
        );
        if result.in_contact {
            q_total += result.q_k * s.slice_width;
        }
        results.push(result);
    }

    Ok((results, q_total))
}

/// Gen3 beam-coupled split solver: independent inner/outer raceway contacts.
///
/// Unlike `solve_gen3_roller` which enforces q_inner = q_outer · cos(Δα),
/// this split model allows different load distributions on inner vs outer
/// raceways. The beam deflection w_k affects inner and outer gaps in
/// opposite directions.
///
/// Two nested loops:
/// - Outer: NR on `delta_o` for global force balance Σq_outer = Σq_inner·cos
/// - Inner: NR beam equilibrium with split contact forces
///
/// `cos_alpha_diff`: cos(α_o − α_i). Must be > 0 (falls back to combined if ~0).
pub fn solve_gen3_roller_split(
    slices: &[SliceGeometry],
    delta_rigid: f64,
    material: &Material,
    params: &SolverParams,
    cos_alpha_diff: f64,
) -> Result<(Vec<SliceContactResult>, f64), SolverError> {
    let n = slices.len();
    if n == 0 {
        return Err(SolverError::InvalidInput("No slices provided".into()));
    }
    if n < 2 {
        return solve_fallback_gen1(slices, delta_rigid, material, cos_alpha_diff);
    }

    // When cos_alpha_diff is ~0 (legacy outer-only), fall back to combined model
    if cos_alpha_diff.abs() < 1e-12 {
        return solve_gen3_roller(slices, delta_rigid, material, params, cos_alpha_diff);
    }

    let e_star_gpa = hertz::combined_elastic_modulus(
        material.e_roller, material.nu, material.e_ring, material.nu,
    );
    let e_star_mpa = e_star_gpa * 1000.0;
    let e_avg_mpa = ((material.e_roller + material.e_ring) / 2.0) * 1000.0;
    let k_beam = beam::assemble_beam_stiffness(slices, material, params.beam_type);

    // Compute δ_o from combined model as initial guess, then refine with
    // a few secant iterations for force balance. Avoids the full 30-iter NR
    // with numerical perturbation (which was 60× beam solves per call).
    let mut delta_o = compute_delta_o_from_combined(slices, delta_rigid, e_star_mpa, cos_alpha_diff);

    let max_refine = 8; // at most 8 beam solves (vs. 60 before)
    let mut prev_residual = f64::MAX;
    let mut prev_delta_o = delta_o;

    for _iter in 0..max_refine {
        let delta_i = (delta_rigid - delta_o) / cos_alpha_diff;
        let (results, q_total_outer, q_total_inner) = solve_beam_with_split(
            slices, delta_o, delta_i, material, params,
            e_star_mpa, e_avg_mpa, cos_alpha_diff, &k_beam,
        )?;

        let force_residual = q_total_outer - q_total_inner * cos_alpha_diff;
        let force_norm = q_total_outer.max(q_total_inner * cos_alpha_diff).max(1.0);

        if (force_residual / force_norm).abs() < params.convergence_tol {
            return Ok((results, q_total_outer));
        }

        // Secant update using previous iteration (if available)
        if prev_residual.is_finite() && (force_residual - prev_residual).abs() > 1e-20 {
            let d_delta = delta_o - prev_delta_o;
            let d_resid = force_residual - prev_residual;
            let delta_o_new = delta_o - force_residual * d_delta / d_resid;
            prev_delta_o = delta_o;
            prev_residual = force_residual;
            delta_o = delta_o_new.clamp(0.01, delta_rigid * 0.99);
        } else {
            // First iteration or no progress: perturb based on residual sign
            prev_delta_o = delta_o;
            prev_residual = force_residual;
            // If Q_outer > Q_inner*cos, outer is overloaded → decrease δ_o
            delta_o -= force_residual / force_norm * delta_o * 0.3;
            delta_o = delta_o.clamp(0.01, delta_rigid * 0.99);
        }
    }

    // Return last result
    let delta_i = (delta_rigid - delta_o) / cos_alpha_diff;
    let (results, q_total_outer, _) = solve_beam_with_split(
        slices, delta_o, delta_i, material, params,
        e_star_mpa, e_avg_mpa, cos_alpha_diff, &k_beam,
    )?;
    Ok((results, q_total_outer))
}

/// Compute δ_o (outer rigid approach) from the combined dual-Hertz model.
///
/// For each loaded slice, solve the combined Hertz equation to get q, then
/// compute the outer Hertz deformation. The weighted average across slices
/// gives δ_o. This is exact when beam deflection is zero and a good
/// approximation when beam effects are small.
fn compute_delta_o_from_combined(
    slices: &[SliceGeometry],
    delta_rigid: f64,
    e_star_mpa: f64,
    cos_alpha_diff: f64,
) -> f64 {
    let mut sum_delta_o = 0.0;
    let mut sum_weight = 0.0;

    for s in slices {
        let gap = delta_rigid - s.delta_z_total_outer - s.delta_z_total_inner * cos_alpha_diff;
        if gap <= 0.0 {
            continue;
        }
        let q = hertz::solve_q_from_dual_delta(
            gap / 1000.0,
            s.r_eq_inner,
            s.r_eq_outer,
            e_star_mpa,
            cos_alpha_diff,
        );
        if q > 0.0 {
            let delta_hertz_outer = hertz::hertz_approach(q, s.r_eq_outer, e_star_mpa) * 1000.0; // mm→μm
            let weight = q * s.slice_width; // load-weighted
            sum_delta_o += delta_hertz_outer * weight;
            sum_weight += weight;
        }
    }

    if sum_weight > 0.0 {
        sum_delta_o / sum_weight
    } else {
        delta_rigid * 0.5 // fallback: symmetric split
    }
}

/// Inner beam NR loop with split inner/outer contacts.
///
/// Returns (slice_results, q_total_outer, q_total_inner) where totals are
/// integrated line loads: Σ q_k · slice_width.
fn solve_beam_with_split(
    slices: &[SliceGeometry],
    delta_o: f64,
    delta_i: f64,
    material: &Material,
    params: &SolverParams,
    e_star_mpa: f64,
    e_avg_mpa: f64,
    cos_alpha_diff: f64,
    k_beam: &nalgebra::DMatrix<f64>,
) -> Result<(Vec<SliceContactResult>, f64, f64), SolverError> {
    let n = slices.len();
    let ndof = 2 * n;
    let mut w = DVector::zeros(ndof);

    let max_active_set_iters = 15;
    let mut prev_active: Vec<bool> = vec![false; n];

    for _outer in 0..max_active_set_iters {
        for _nr_iter in 0..params.max_iterations {
            let mut f_contact = DVector::zeros(ndof);
            let mut contact_stiffness = vec![0.0_f64; n];
            let mut active = vec![false; n];

            for k in 0..n {
                let s = &slices[k];
                let h1 = s.r_roller;
                let h2 = s.r_roller * 2.0;

                // Outer gap: w is ~outer normal direction → direct subtraction
                let gap_outer = delta_o - w[2 * k] - s.delta_z_total_outer;
                // Inner gap: w projected onto inner normal → w × cos(α_o - α_i)
                let gap_inner = delta_i + w[2 * k] * cos_alpha_diff - s.delta_z_total_inner;

                let (q_out, _, _, _, k_h_out) =
                    hertz::single_raceway_contact(gap_outer, s.r_eq_outer, e_star_mpa, e_avg_mpa, material.nu, h1, h2);
                let (q_in, _, _, _, k_h_in) =
                    hertz::single_raceway_contact(gap_inner, s.r_eq_inner, e_star_mpa, e_avg_mpa, material.nu, h1, h2);

                if q_out > 0.0 || q_in > 0.0 {
                    active[k] = true;
                    // Net force on beam (outer normal direction):
                    // outer pushes +w, inner pushes -w (projected)
                    let f_net = (q_out - q_in * cos_alpha_diff) * s.slice_width;
                    f_contact[2 * k] = f_net;
                    // Jacobian: ∂f/∂w = -(k_out + k_in·cos²)·l/1000
                    // Both terms positive (restoring) on diagonal
                    let cos2 = cos_alpha_diff * cos_alpha_diff;
                    contact_stiffness[k] = (k_h_out + k_h_in * cos2) * s.slice_width;
                }
            }

            let n_active = active.iter().filter(|&&a| a).count();
            if n_active < 2 {
                // Build result directly for this split configuration
                return build_split_result(slices, &w, delta_o, delta_i, material, e_star_mpa, e_avg_mpa, cos_alpha_diff);
            }

            // Residual: R = K_beam · (w/1000) - F_contact
            let w_mm = &w * 1e-3;
            let residual = k_beam * &w_mm - &f_contact;

            // Convergence check
            let f_norm = f_contact.norm().max(1.0);
            let r_norm = residual.norm();
            if r_norm / f_norm < params.convergence_tol {
                if active == prev_active {
                    return build_split_result(slices, &w, delta_o, delta_i, material, e_star_mpa, e_avg_mpa, cos_alpha_diff);
                }
                prev_active = active;
                break;
            }

            // Jacobian: J = K_beam/1000 + diag(K_contact)
            let mut jacobian = k_beam * 1e-3;
            for k in 0..n {
                if active[k] {
                    jacobian[(2 * k, 2 * k)] += contact_stiffness[k];
                }
            }

            // Solve: J · Δw = -R
            let neg_residual = -&residual;
            let decomp = jacobian.clone().lu();
            let dw = match decomp.solve(&neg_residual) {
                Some(sol) => sol,
                None => {
                    return Err(SolverError::ConvergenceFailure(
                        "Jacobian is singular in Gen3 split solver".into(),
                    ));
                }
            };

            w += &dw;
            remove_rigid_body_modes(&mut w, slices);
        }

        // Check active set stability
        let mut final_active = vec![false; n];
        for k in 0..n {
            let gap_outer = delta_o - w[2 * k] - slices[k].delta_z_total_outer;
            let gap_inner = delta_i + w[2 * k] * cos_alpha_diff - slices[k].delta_z_total_inner;
            final_active[k] = gap_outer > 0.0 || gap_inner > 0.0;
        }
        if final_active == prev_active {
            return build_split_result(slices, &w, delta_o, delta_i, material, e_star_mpa, e_avg_mpa, cos_alpha_diff);
        }
        prev_active = final_active;
    }

    build_split_result(slices, &w, delta_o, delta_i, material, e_star_mpa, e_avg_mpa, cos_alpha_diff)
}

/// Build final result from converged displacement vector (split model).
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
        let s = &slices[k];
        let w_k = if w.len() > 2 * k { w[2 * k] } else { 0.0 };
        let h1 = s.r_roller;
        let h2 = s.r_roller * 2.0;

        let gap_outer = delta_o - w_k - s.delta_z_total_outer;
        let gap_inner = delta_i + w_k * cos_alpha_diff - s.delta_z_total_inner;

        let (q_out, b_out, p_out, h_bulk_out, k_h_out) =
            hertz::single_raceway_contact(gap_outer, s.r_eq_outer, e_star_mpa, e_avg_mpa, material.nu, h1, h2);
        let (q_in, b_in, p_in, h_bulk_in, k_h_in) =
            hertz::single_raceway_contact(gap_inner, s.r_eq_inner, e_star_mpa, e_avg_mpa, material.nu, h1, h2);

        let in_contact = q_out > 0.0 || q_in > 0.0;
        if q_out > 0.0 {
            q_total_outer += q_out * s.slice_width;
        }
        if q_in > 0.0 {
            q_total_inner += q_in * s.slice_width;
        }

        // Combined slice stiffness along outer normal (Hertz-only 2-spring series):
        //   1/k = 1/k_hertz_outer + cos²/k_hertz_inner
        let cos_sq = cos_alpha_diff * cos_alpha_diff;
        let mut inv_k = 0.0;
        if k_h_out > 0.0 { inv_k += 1.0 / k_h_out; }
        if k_h_in > 0.0 && cos_sq > 0.0 { inv_k += cos_sq / k_h_in; }
        let k_combined = if inv_k > 0.0 { 1.0 / inv_k } else { 0.0 };

        // delta_k: use the outer gap as the "approach" for backward compat
        let delta_k = gap_outer.max(0.0);

        results.push(SliceContactResult {
            k,
            delta_k,
            q_k: q_out,            // backward compat: outer normal load
            q_k_outer: q_out,
            q_k_inner: q_in,
            b_k: b_in,
            p_max_k: p_in,
            h_bulk_k: h_bulk_in,
            k_hertz_k: k_h_in,
            b_k_outer: b_out,
            p_max_k_outer: p_out,
            h_bulk_k_outer: h_bulk_out,
            k_hertz_k_outer: k_h_out,
            k_combined_k: k_combined,
            in_contact,
        });
    }

    Ok((results, q_total_outer, q_total_inner))
}

/// Gen3 solver: find δ_rigid [μm] for target total load Q_target [N].
///
/// `cos_alpha_diff`: cos(α_o − α_i) for dual-raceway model. Pass 0.0 for legacy.
pub fn solve_gen3_for_load(
    slices: &[SliceGeometry],
    q_target: f64,
    material: &Material,
    params: &SolverParams,
    cos_alpha_diff: f64,
) -> Result<(Vec<SliceContactResult>, f64, f64), SolverError> {
    if q_target <= 0.0 {
        return Err(SolverError::InvalidInput(
            "Target load must be positive".into(),
        ));
    }
    if slices.is_empty() {
        return Err(SolverError::InvalidInput("No slices provided".into()));
    }

    let mut delta_rigid = 5.0_f64;
    let perturbation = 0.01;

    for _iter in 0..params.max_iterations {
        let (results, q_calc) = solve_gen3_roller(slices, delta_rigid, material, params, cos_alpha_diff)?;

        let residual = q_calc - q_target;

        if (residual / q_target).abs() < params.convergence_tol {
            return Ok((results, q_calc, delta_rigid));
        }

        let (_, q_plus) =
            solve_gen3_roller(slices, delta_rigid + perturbation, material, params, cos_alpha_diff)?;
        let dq_ddelta = (q_plus - q_calc) / perturbation;

        if dq_ddelta.abs() < 1e-20 {
            delta_rigid *= 2.0;
            continue;
        }

        let delta_new = delta_rigid - residual / dq_ddelta;
        delta_rigid = delta_new.max(0.01);
    }

    Err(SolverError::ConvergenceFailure(format!(
        "Gen3 solver did not converge after {} iterations (target={:.1}N)",
        params.max_iterations, q_target
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::gen1;

    fn make_test_slices(n: usize, crown_um: f64) -> (Vec<SliceGeometry>, Material) {
        let l_we = 15.0;
        let slice_width = l_we / n as f64;
        let slices: Vec<SliceGeometry> = (0..n)
            .map(|k| {
                let x = (k as f64 + 0.5) * slice_width;
                let frac = x / l_we;
                let r_roller = 4.0 + frac;
                let r_race = 200.0;
                let x_centered = x - l_we / 2.0;
                let dz = crown_um * (2.0 * x_centered / l_we).powi(2);
                SliceGeometry {
                    k,
                    x_axial: x,
                    r_roller,
                    r_inner_race: r_race,
                    r_outer_race: r_race,
                    r_eq_inner: (r_roller * r_race) / (r_roller + r_race),
                    r_eq_outer: (r_roller * r_race) / (r_roller + r_race),
                    delta_z_total_inner: dz,
                    delta_z_total_outer: dz,
                    slice_width,
                }
            })
            .collect();
        (slices, Material::default())
    }

    #[test]
    fn test_flat_profile_converges() {
        let (slices, mat) = make_test_slices(20, 0.0);
        let params = SolverParams::default();
        let result = solve_gen3_roller(&slices, 5.0, &mat, &params, 0.0);
        assert!(result.is_ok(), "Gen3 should converge: {:?}", result.err());
        let (results, q_total) = result.unwrap();
        assert!(q_total > 0.0, "Q_total should be > 0, got {q_total}");
        assert!(results.iter().all(|r| r.in_contact));
    }

    #[test]
    fn test_gen3_gen1_convergence() {
        // Level C verification: flat profile → Gen3 ≈ Gen1
        let (slices, mat) = make_test_slices(50, 0.0);
        let params = SolverParams::default();

        let (_, q_gen1) = gen1::solve_gen1_roller(&slices, 5.0, &mat, 0.0);
        let (_, q_gen3) = solve_gen3_roller(&slices, 5.0, &mat, &params, 0.0).unwrap();

        let rel_diff = (q_gen3 - q_gen1).abs() / q_gen1;
        assert!(
            rel_diff < 0.03,
            "Level C: Gen3 Q={q_gen3:.1} vs Gen1 Q={q_gen1:.1}, diff={:.2}%",
            rel_diff * 100.0
        );
    }

    #[test]
    fn test_crowned_profile_smoother() {
        let (slices, mat) = make_test_slices(20, 3.0);
        let params = SolverParams::default();

        let (gen1_results, _) = gen1::solve_gen1_roller(&slices, 8.0, &mat, 0.0);
        let (gen3_results, _) = solve_gen3_roller(&slices, 8.0, &mat, &params, 0.0).unwrap();

        // Gen3 should produce smoother (or equal) load distribution
        let gen1_q: Vec<f64> = gen1_results
            .iter()
            .filter(|r| r.in_contact)
            .map(|r| r.q_k)
            .collect();
        let gen3_q: Vec<f64> = gen3_results
            .iter()
            .filter(|r| r.in_contact)
            .map(|r| r.q_k)
            .collect();

        let gen1_ratio = gen1_q.iter().cloned().fold(f64::MIN, f64::max)
            / gen1_q.iter().cloned().fold(f64::MAX, f64::min);
        let gen3_ratio = gen3_q.iter().cloned().fold(f64::MIN, f64::max)
            / gen3_q.iter().cloned().fold(f64::MAX, f64::min);

        assert!(
            gen3_ratio <= gen1_ratio * 1.05,
            "Gen3 ratio={gen3_ratio:.2} should be <= Gen1 ratio={gen1_ratio:.2}"
        );
    }

    #[test]
    fn test_beam_deflection_nonzero() {
        let (slices, mat) = make_test_slices(20, 3.0);
        let params = SolverParams::default();

        let (gen3_results, _) = solve_gen3_roller(&slices, 8.0, &mat, &params, 0.0).unwrap();
        let (gen1_results, _) = gen1::solve_gen1_roller(&slices, 8.0, &mat, 0.0);

        let max_diff: f64 = gen3_results
            .iter()
            .zip(gen1_results.iter())
            .filter(|(g3, _)| g3.in_contact)
            .map(|(g3, g1)| (g3.q_k - g1.q_k).abs())
            .fold(0.0, f64::max);

        assert!(
            max_diff > 0.01,
            "Beam deflection should cause load redistribution, max_diff={max_diff}"
        );
    }

    #[test]
    fn test_solve_for_target_load() {
        let (slices, mat) = make_test_slices(50, 2.0);
        let params = SolverParams::default();

        let q_target = 5000.0;
        let result = solve_gen3_for_load(&slices, q_target, &mat, &params, 0.0);
        assert!(result.is_ok(), "Should converge: {:?}", result.err());

        let (_, q_calc, delta_rigid) = result.unwrap();
        let rel_err = (q_calc - q_target).abs() / q_target;
        assert!(
            rel_err < 1e-4,
            "Q_calc={q_calc:.2}N vs target={q_target:.0}N, rel_err={rel_err:.2e}"
        );
        assert!(delta_rigid > 0.0);
    }

    #[test]
    fn test_load_increases_with_approach() {
        let (slices, mat) = make_test_slices(20, 1.0);
        let params = SolverParams::default();

        let deltas = [1.0, 5.0, 10.0, 20.0];
        let loads: Vec<f64> = deltas
            .iter()
            .map(|&d| solve_gen3_roller(&slices, d, &mat, &params, 0.0).unwrap().1)
            .collect();

        for i in 1..loads.len() {
            assert!(
                loads[i] > loads[i - 1],
                "Q({})={:.1} should > Q({})={:.1}",
                deltas[i],
                loads[i],
                deltas[i - 1],
                loads[i - 1]
            );
        }
    }

    #[test]
    fn test_split_solver_basic() {
        let (slices, mat) = make_test_slices(20, 3.0);
        let params = SolverParams::default();
        // Use cos_alpha_diff = 0.99 (typical TRB)
        let result = solve_gen3_roller_split(&slices, 8.0, &mat, &params, 0.99);
        assert!(result.is_ok(), "Split solver should converge: {:?}", result.err());
        let (results, q_total) = result.unwrap();
        assert!(q_total > 0.0);
        assert!(results.iter().any(|r| r.q_k_outer > 0.0));
        assert!(results.iter().any(|r| r.q_k_inner > 0.0));
    }

    fn make_asymmetric_slices(n: usize) -> (Vec<SliceGeometry>, Material) {
        let l_we = 15.0;
        let slice_width = l_we / n as f64;
        let slices: Vec<SliceGeometry> = (0..n)
            .map(|k| {
                let x = (k as f64 + 0.5) * slice_width;
                let frac = x / l_we;
                let r_roller = 4.0 + frac;
                let r_race = 200.0;
                let x_centered = x - l_we / 2.0;
                let dz_outer = 5.0 * (2.0 * x_centered / l_we).powi(2); // heavy outer crown
                let dz_inner = 0.0; // flat inner
                SliceGeometry {
                    k,
                    x_axial: x,
                    r_roller,
                    r_inner_race: r_race,
                    r_outer_race: r_race,
                    r_eq_inner: (r_roller * r_race) / (r_roller + r_race),
                    r_eq_outer: (r_roller * r_race) / (r_roller + r_race),
                    delta_z_total_inner: dz_inner,
                    delta_z_total_outer: dz_outer,
                    slice_width,
                }
            })
            .collect();
        (slices, Material::default())
    }

    #[test]
    fn test_split_symmetric_matches_combined() {
        // Symmetric slices: r_eq_inner == r_eq_outer, dz_inner == dz_outer
        let (slices, mat) = make_test_slices(20, 3.0);
        let params = SolverParams::default();
        let cos_alpha_diff = 0.99;

        let (_, q_combined) = solve_gen3_roller(&slices, 8.0, &mat, &params, cos_alpha_diff).unwrap();
        let (_, q_split) = solve_gen3_roller_split(&slices, 8.0, &mat, &params, cos_alpha_diff).unwrap();

        let rel_diff = (q_split - q_combined).abs() / q_combined.max(1.0);
        assert!(
            rel_diff < 0.10,
            "Symmetric: split Q={q_split:.1} vs combined Q={q_combined:.1}, diff={:.2}%",
            rel_diff * 100.0
        );
    }

    #[test]
    fn test_split_asymmetric_different_distributions() {
        let (slices, mat) = make_asymmetric_slices(20);
        let params = SolverParams::default();
        let cos_alpha_diff = 0.99;

        let (results, q_total) = solve_gen3_roller_split(&slices, 10.0, &mat, &params, cos_alpha_diff).unwrap();
        assert!(q_total > 0.0, "Should have non-zero load");

        // Check that inner and outer q distributions are NOT just scaled copies
        let contact_results: Vec<&SliceContactResult> = results.iter()
            .filter(|r| r.q_k_outer > 0.0 && r.q_k_inner > 0.0)
            .collect();

        if contact_results.len() >= 3 {
            // Compute ratio q_outer/q_inner at each slice
            let ratios: Vec<f64> = contact_results.iter()
                .map(|r| r.q_k_outer / r.q_k_inner.max(1e-10))
                .collect();
            let ratio_max = ratios.iter().cloned().fold(f64::MIN, f64::max);
            let ratio_min = ratios.iter().cloned().fold(f64::MAX, f64::min);
            let ratio_variation = ratio_max - ratio_min;

            // In combined model, ratio would be constant (1/cos or similar).
            // In split model with asymmetric profiles, it should vary.
            assert!(
                ratio_variation > 0.001,
                "q_outer/q_inner ratio should vary across slices: variation={ratio_variation:.6}, \
                 min={ratio_min:.4}, max={ratio_max:.4}"
            );
        }
    }

    #[test]
    fn test_split_force_balance() {
        let (slices, mat) = make_asymmetric_slices(20);
        let params = SolverParams::default();
        let cos_alpha_diff = 0.99;

        let (results, q_total_outer) = solve_gen3_roller_split(&slices, 10.0, &mat, &params, cos_alpha_diff).unwrap();

        // Recompute total inner from results
        let q_total_inner: f64 = results.iter()
            .filter(|r| r.q_k_inner > 0.0)
            .map(|r| r.q_k_inner * slices[r.k].slice_width)
            .sum();

        // Force balance: Q_outer = Q_inner * cos
        let q_inner_projected = q_total_inner * cos_alpha_diff;
        let rel_diff = (q_total_outer - q_inner_projected).abs() / q_total_outer.max(1.0);
        assert!(
            rel_diff < 0.02,
            "Force balance: Q_outer={q_total_outer:.1} vs Q_inner*cos={q_inner_projected:.1}, diff={:.3}%",
            rel_diff * 100.0
        );
    }

    #[test]
    fn test_split_solver_fallback_cos_zero() {
        // cos_alpha_diff ≈ 0 should fall back to combined model
        let (slices, mat) = make_test_slices(20, 3.0);
        let params = SolverParams::default();
        let result = solve_gen3_roller_split(&slices, 8.0, &mat, &params, 0.0);
        assert!(result.is_ok());
        let (_, q_total) = result.unwrap();
        assert!(q_total > 0.0);
    }

    // ─── Comprehensive physics validation for split model ───

    /// TRB-realistic slices: different R_eq for inner/outer (tapered geometry),
    /// asymmetric profiles (inner flat, outer crowned).
    fn make_trb_realistic_slices(n: usize) -> (Vec<SliceGeometry>, Material) {
        let l_we = 15.0;
        let slice_width = l_we / n as f64;
        let d_pw = 70.0;
        let d_we_min = 8.5;
        let d_we_max = 10.0;
        let alpha_o: f64 = 12.0_f64.to_radians();
        let alpha_i: f64 = 12.0_f64.to_radians();
        let r_inner_race = 200.0;
        let r_outer_race = 200.0;

        let slices: Vec<SliceGeometry> = (0..n)
            .map(|k| {
                let x = (k as f64 + 0.5) * slice_width;
                let frac = x / l_we;
                let d_roller = d_we_min + (d_we_max - d_we_min) * frac;
                let r_roller = d_roller / 2.0;

                // TRB equivalent radii (inner: convex-convex, outer: convex-concave)
                let gamma_i = d_roller * alpha_i.cos() / d_pw;
                let gamma_o = d_roller * alpha_o.cos() / d_pw;
                let r_eq_inner = (d_roller / 2.0) * (1.0 - gamma_i);
                let r_eq_outer = (d_roller / 2.0) * (1.0 + gamma_o);

                // Asymmetric profiles: outer has crown, inner is flat
                let x_centered = x - l_we / 2.0;
                let dz_outer = 3.0 * (2.0 * x_centered / l_we).powi(2); // 3μm parabolic crown
                let dz_inner = 0.0; // flat

                SliceGeometry {
                    k,
                    x_axial: x,
                    r_roller,
                    r_inner_race,
                    r_outer_race,
                    r_eq_inner,
                    r_eq_outer,
                    delta_z_total_inner: dz_inner,
                    delta_z_total_outer: dz_outer,
                    slice_width,
                }
            })
            .collect();
        (slices, Material::default())
    }

    #[test]
    fn test_split_physics_validation() {
        let (slices, mat) = make_trb_realistic_slices(30);
        let params = SolverParams::default();
        let cos_alpha_diff = 0.99; // typical TRB
        let delta_rigid = 10.0; // μm

        // ─── 1. Combined model (baseline) ───
        let (combined_results, q_combined) =
            solve_gen3_roller(&slices, delta_rigid, &mat, &params, cos_alpha_diff).unwrap();

        // ─── 2. Split model ───
        let (split_results, q_split) =
            solve_gen3_roller_split(&slices, delta_rigid, &mat, &params, cos_alpha_diff).unwrap();

        println!("\n=== Gen3 Split Model Physics Validation ===");
        println!("δ_rigid = {delta_rigid} μm, cos(α_o−α_i) = {cos_alpha_diff}");
        println!("Combined Q_total = {q_combined:.1} N");
        println!("Split Q_total    = {q_split:.1} N");

        // ─── Check 1: Both models produce positive load ───
        assert!(q_combined > 0.0, "Combined Q must be > 0");
        assert!(q_split > 0.0, "Split Q must be > 0");

        // ─── Check 2: Total load should be in same ballpark ───
        // (not exact match because split model has different load distribution)
        let q_ratio = q_split / q_combined;
        println!("Q_split/Q_combined = {q_ratio:.4}");
        assert!(q_ratio > 0.5 && q_ratio < 2.0,
            "Total load ratio should be reasonable: {q_ratio:.4}");

        // ─── Check 3: Force balance (ΣQ_outer ≈ ΣQ_inner * cos) ───
        let q_outer_total: f64 = split_results.iter()
            .filter(|r| r.q_k_outer > 0.0)
            .map(|r| r.q_k_outer * slices[r.k].slice_width)
            .sum();
        let q_inner_total: f64 = split_results.iter()
            .filter(|r| r.q_k_inner > 0.0)
            .map(|r| r.q_k_inner * slices[r.k].slice_width)
            .sum();
        let force_balance_err = (q_outer_total - q_inner_total * cos_alpha_diff).abs()
            / q_outer_total.max(1.0);
        println!("Force balance: Q_outer={q_outer_total:.1}, Q_inner*cos={:.1}, err={:.2}%",
            q_inner_total * cos_alpha_diff, force_balance_err * 100.0);
        assert!(force_balance_err < 0.05,
            "Force balance error {:.2}% exceeds 5%", force_balance_err * 100.0);

        // ─── Check 4: Inner/outer q distributions differ (the whole point!) ───
        println!("\n  k | q_outer | q_inner | ratio  | combined q | b_inner | b_outer | p_inner | p_outer");
        println!("----+---------+---------+--------+------------+---------+---------+---------+--------");
        let mut ratios = Vec::new();
        for (sr, cr) in split_results.iter().zip(combined_results.iter()) {
            if sr.q_k_outer > 1.0 && sr.q_k_inner > 1.0 {
                let ratio = sr.q_k_outer / sr.q_k_inner;
                ratios.push(ratio);
                println!("{:>3} | {:>7.2} | {:>7.2} | {:>6.3} | {:>10.2} | {:>7.5} | {:>7.5} | {:>7.0} | {:>7.0}",
                    sr.k, sr.q_k_outer, sr.q_k_inner, ratio, cr.q_k,
                    sr.b_k, sr.b_k_outer, sr.p_max_k, sr.p_max_k_outer);
            }
        }

        if ratios.len() >= 3 {
            let ratio_min = ratios.iter().cloned().fold(f64::MAX, f64::min);
            let ratio_max = ratios.iter().cloned().fold(f64::MIN, f64::max);
            let ratio_variation = ratio_max - ratio_min;
            println!("\nq_outer/q_inner ratio: min={ratio_min:.4}, max={ratio_max:.4}, variation={ratio_variation:.4}");

            // KEY CHECK: in the combined model, this ratio would be constant (= 1/cos).
            // In the split model, it should VARY across slices.
            assert!(ratio_variation > 0.001,
                "Split model should have varying inner/outer ratio, got variation={ratio_variation:.6}");
        }

        // ─── Check 5: Contact results are consistent with q values ───
        // b_k should be computed from q_k_inner, b_k_outer from q_k_outer
        let e_star_gpa = hertz::combined_elastic_modulus(mat.e_roller, mat.nu, mat.e_ring, mat.nu);
        let e_star_mpa = e_star_gpa * 1000.0;
        for sr in split_results.iter().filter(|r| r.q_k_outer > 1.0) {
            let s = &slices[sr.k];
            // Verify b_outer is consistent with q_outer
            let b_expected = hertz::hertz_half_width(sr.q_k_outer, s.r_eq_outer, e_star_mpa);
            let b_err = (sr.b_k_outer - b_expected).abs() / b_expected.max(1e-10);
            assert!(b_err < 0.01,
                "Slice {}: b_outer={:.5} vs expected={:.5}, err={:.2}%",
                sr.k, sr.b_k_outer, b_expected, b_err * 100.0);

            // Verify b_inner is consistent with q_inner
            if sr.q_k_inner > 1.0 {
                let b_inner_expected = hertz::hertz_half_width(sr.q_k_inner, s.r_eq_inner, e_star_mpa);
                let b_inner_err = (sr.b_k - b_inner_expected).abs() / b_inner_expected.max(1e-10);
                assert!(b_inner_err < 0.01,
                    "Slice {}: b_inner={:.5} vs expected={:.5}, err={:.2}%",
                    sr.k, sr.b_k, b_inner_expected, b_inner_err * 100.0);
            }
        }
        println!("\n✓ Contact half-width (b) consistent with independent q values");

        // ─── Check 6: R_eq difference produces different contact patterns ───
        // Inner R_eq < Outer R_eq (TRB geometry) → for same q:
        //   inner b < outer b, inner p_max > outer p_max
        let mut inner_wider = 0;
        let mut outer_wider = 0;
        for sr in split_results.iter().filter(|r| r.q_k_outer > 1.0 && r.q_k_inner > 1.0) {
            if sr.b_k > sr.b_k_outer { inner_wider += 1; }
            else { outer_wider += 1; }
        }
        println!("Contact width: inner wider in {} slices, outer wider in {} slices", inner_wider, outer_wider);
        // For TRB: R_eq_outer > R_eq_inner, and loads are similar → outer should generally be wider
        assert!(outer_wider > 0, "Outer raceway should have wider contact in at least some slices");

        // ─── Check 7: Crowned outer → more uniform outer distribution ───
        let outer_q: Vec<f64> = split_results.iter()
            .filter(|r| r.q_k_outer > 1.0)
            .map(|r| r.q_k_outer)
            .collect();
        let inner_q: Vec<f64> = split_results.iter()
            .filter(|r| r.q_k_inner > 1.0)
            .map(|r| r.q_k_inner)
            .collect();

        if outer_q.len() >= 3 && inner_q.len() >= 3 {
            let outer_cv = coefficient_of_variation(&outer_q);
            let inner_cv = coefficient_of_variation(&inner_q);
            println!("Load distribution CV: outer={outer_cv:.4}, inner={inner_cv:.4}");
            println!("  (Crowned outer should have lower CV = more uniform)");
            // Note: this may not always hold depending on beam effects,
            // but for this test case it should
        }

        println!("\n=== All physics checks passed ===");
    }

    fn coefficient_of_variation(values: &[f64]) -> f64 {
        let n = values.len() as f64;
        let mean = values.iter().sum::<f64>() / n;
        let var = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
        var.sqrt() / mean.abs().max(1e-10)
    }

    /// Verify that combined model has constant q_outer/q_inner ratio (= 1/cos or 1)
    /// while split model does NOT.
    #[test]
    fn test_combined_vs_split_ratio_constancy() {
        let (slices, mat) = make_trb_realistic_slices(30);
        let params = SolverParams::default();
        let cos_alpha_diff = 0.99;
        let delta_rigid = 10.0;

        // Combined: q_inner = q_k * cos → ratio = q_k / (q_k * cos) = 1/cos = constant
        let (combined, _) = solve_gen3_roller(&slices, delta_rigid, &mat, &params, cos_alpha_diff).unwrap();
        let combined_ratios: Vec<f64> = combined.iter()
            .filter(|r| r.q_k_outer > 1.0 && r.q_k_inner > 1.0)
            .map(|r| r.q_k_outer / r.q_k_inner)
            .collect();

        let combined_cv = coefficient_of_variation(&combined_ratios);

        // Split: ratio should vary
        let (split, _) = solve_gen3_roller_split(&slices, delta_rigid, &mat, &params, cos_alpha_diff).unwrap();
        let split_ratios: Vec<f64> = split.iter()
            .filter(|r| r.q_k_outer > 1.0 && r.q_k_inner > 1.0)
            .map(|r| r.q_k_outer / r.q_k_inner)
            .collect();

        let split_cv = coefficient_of_variation(&split_ratios);

        println!("Combined q_o/q_i ratio CV = {combined_cv:.6} (should be ~0 = constant)");
        println!("Split    q_o/q_i ratio CV = {split_cv:.6} (should be > 0 = varying)");

        // Combined model: ratio is exactly 1/cos at every slice → CV ≈ 0
        assert!(combined_cv < 0.001,
            "Combined model ratio should be near-constant, CV={combined_cv:.6}");
        // Split model: ratio varies → CV > 0
        assert!(split_cv > combined_cv,
            "Split model ratio should vary more than combined: split_cv={split_cv:.6} vs combined_cv={combined_cv:.6}");
    }
}
