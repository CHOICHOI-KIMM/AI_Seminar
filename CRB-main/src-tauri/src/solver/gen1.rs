use crate::error::SolverError;
use crate::solver::hertz;
use crate::solver::types::*;

/// Gen1 independent slice solver (dual-raceway model).
/// Each slice is treated as an independent nonlinear spring — no beam coupling.
///
/// Given a rigid body approach `delta_rigid` [μm] along the outer raceway
/// contact normal (α_o), computes contact results for all slices using
/// dual-raceway Hertz approach:
///   δ_available = δ_rigid − Δz_outer − Δz_inner·cos(α_o−α_i)
///   solve: δ_hertz_outer(q) + δ_hertz_inner(q)·cos(α_o−α_i) = δ_available
///
/// `cos_alpha_diff`: cos(α_o − α_i), projection factor from inner to outer
///   raceway normal direction.  Pass 0.0 for legacy outer-only behavior.
///
/// Returns (slice_results, Q_total [N]).
pub fn solve_gen1_roller(
    slices: &[SliceGeometry],
    delta_rigid: f64,
    material: &Material,
    cos_alpha_diff: f64,
) -> (Vec<SliceContactResult>, f64) {
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
        // Dual-raceway available approach:
        // δ_available = δ_rigid − Δz_outer − Δz_inner·cos(α_o−α_i)
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

    (results, q_total)
}

/// Find delta_rigid [μm] that produces target total load Q_target [N].
/// Uses Newton-Raphson iteration with numerical differentiation.
///
/// `cos_alpha_diff`: cos(α_o − α_i) for dual-raceway model. Pass 0.0 for legacy.
///
/// Returns (slice_results, Q_total, delta_rigid) on convergence.
pub fn solve_gen1_for_load(
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

    // Initial guess: rough estimate based on linear stiffness assumption
    if slices.is_empty() {
        return Err(SolverError::InvalidInput("No slices provided".into()));
    }

    // Start with a moderate approach value
    let mut delta_rigid = 5.0_f64; // μm initial guess

    let perturbation = 0.01; // μm for numerical derivative

    for _iter in 0..params.max_iterations {
        let (results, q_calc) = solve_gen1_roller(slices, delta_rigid, material, cos_alpha_diff);

        let residual = q_calc - q_target;

        // Check convergence
        if (residual / q_target).abs() < params.convergence_tol {
            return Ok((results, q_calc, delta_rigid));
        }

        // Numerical derivative dQ/dδ
        let (_, q_plus) = solve_gen1_roller(slices, delta_rigid + perturbation, material, cos_alpha_diff);
        let dq_ddelta = (q_plus - q_calc) / perturbation;

        if dq_ddelta.abs() < 1e-20 {
            // Stiffness is essentially zero — increase approach significantly
            delta_rigid *= 2.0;
            continue;
        }

        let delta_new = delta_rigid - residual / dq_ddelta;

        // Clamp to positive value
        delta_rigid = delta_new.max(0.01);
    }

    Err(SolverError::ConvergenceFailure(format!(
        "Gen1 solver did not converge after {} iterations (target={:.1}N)",
        params.max_iterations, q_target
    )))
}

/// Gen1 split solver: independent inner/outer contact per slice.
///
/// Unlike the combined model (single q_k per slice), this computes q_outer_k
/// and q_inner_k independently. The "rigid roller" approach variable δ_o
/// determines how δ_rigid is split between inner and outer raceways.
/// A secant iteration ensures global force balance: ΣQ_outer = ΣQ_inner·cos.
///
/// No beam coupling — each slice is still independent. The split comes from
/// the roller's rigid body position between the two raceways.
///
/// `cos_alpha_diff`: cos(α_o − α_i). Must be > 0 (otherwise falls back to combined).
///
/// Returns (slice_results, Q_total_outer [N]).
pub fn solve_gen1_roller_split(
    slices: &[SliceGeometry],
    delta_rigid: f64,
    material: &Material,
    cos_alpha_diff: f64,
) -> (Vec<SliceContactResult>, f64) {
    // Fall back to combined if cos_alpha_diff ≈ 0 (legacy)
    if cos_alpha_diff.abs() < 1e-12 {
        return solve_gen1_roller(slices, delta_rigid, material, cos_alpha_diff);
    }

    let e_star_gpa = hertz::combined_elastic_modulus(
        material.e_roller, material.nu, material.e_ring, material.nu,
    );
    let e_star_mpa = e_star_gpa * 1000.0;
    let e_avg_mpa = ((material.e_roller + material.e_ring) / 2.0) * 1000.0;

    // Initial δ_o: from combined model's Hertz deformation ratio
    let mut delta_o = compute_delta_o_initial(slices, delta_rigid, e_star_mpa, cos_alpha_diff);

    // Secant iteration for force balance (very fast — no beam solve)
    let max_iters = 20;
    let tol = 1e-4;
    let mut prev_residual = f64::MAX;
    let mut prev_delta_o = delta_o;

    for _ in 0..max_iters {
        let delta_i = (delta_rigid - delta_o) / cos_alpha_diff;
        let (q_outer, q_inner) = eval_split_loads(slices, delta_o, delta_i, e_star_mpa, e_avg_mpa, material.nu);

        let force_residual = q_outer - q_inner * cos_alpha_diff;
        let force_norm = q_outer.max(q_inner * cos_alpha_diff).max(1.0);

        if (force_residual / force_norm).abs() < tol {
            break;
        }

        if prev_residual.is_finite() && (force_residual - prev_residual).abs() > 1e-20 {
            let d_delta = delta_o - prev_delta_o;
            let d_resid = force_residual - prev_residual;
            let new_delta_o = delta_o - force_residual * d_delta / d_resid;
            prev_delta_o = delta_o;
            prev_residual = force_residual;
            delta_o = new_delta_o.clamp(0.01, delta_rigid * 0.99);
        } else {
            prev_delta_o = delta_o;
            prev_residual = force_residual;
            delta_o -= force_residual / force_norm * delta_o * 0.3;
            delta_o = delta_o.clamp(0.01, delta_rigid * 0.99);
        }
    }

    // Build final results
    let delta_i = (delta_rigid - delta_o) / cos_alpha_diff;
    build_split_results(slices, delta_o, delta_i, e_star_mpa, e_avg_mpa, material.nu, cos_alpha_diff)
}

/// Compute total outer and inner loads for given split.
fn eval_split_loads(
    slices: &[SliceGeometry],
    delta_o: f64,
    delta_i: f64,
    e_star_mpa: f64,
    e_avg_mpa: f64,
    nu: f64,
) -> (f64, f64) {
    let mut q_outer_total = 0.0;
    let mut q_inner_total = 0.0;
    for s in slices {
        let h1 = s.r_roller;
        let h2 = s.r_roller * 2.0;
        let gap_outer = delta_o - s.delta_z_total_outer;
        let gap_inner = delta_i - s.delta_z_total_inner;
        let (q_o, _, _, _, _) = hertz::single_raceway_contact(gap_outer, s.r_eq_outer, e_star_mpa, e_avg_mpa, nu, h1, h2);
        let (q_i, _, _, _, _) = hertz::single_raceway_contact(gap_inner, s.r_eq_inner, e_star_mpa, e_avg_mpa, nu, h1, h2);
        q_outer_total += q_o * s.slice_width;
        q_inner_total += q_i * s.slice_width;
    }
    (q_outer_total, q_inner_total)
}

/// Initial δ_o guess from combined model's Hertz deformation.
fn compute_delta_o_initial(
    slices: &[SliceGeometry],
    delta_rigid: f64,
    e_star_mpa: f64,
    cos_alpha_diff: f64,
) -> f64 {
    let mut sum_delta_o = 0.0;
    let mut sum_weight = 0.0;
    for s in slices {
        let gap = delta_rigid - s.delta_z_total_outer - s.delta_z_total_inner * cos_alpha_diff;
        if gap <= 0.0 { continue; }
        let q = hertz::solve_q_from_dual_delta(gap / 1000.0, s.r_eq_inner, s.r_eq_outer, e_star_mpa, cos_alpha_diff);
        if q > 0.0 {
            let delta_hertz_outer = hertz::hertz_approach(q, s.r_eq_outer, e_star_mpa) * 1000.0;
            let weight = q * s.slice_width;
            sum_delta_o += delta_hertz_outer * weight;
            sum_weight += weight;
        }
    }
    if sum_weight > 0.0 { sum_delta_o / sum_weight } else { delta_rigid * 0.5 }
}

/// Build final split results.
fn build_split_results(
    slices: &[SliceGeometry],
    delta_o: f64,
    delta_i: f64,
    e_star_mpa: f64,
    e_avg_mpa: f64,
    nu: f64,
    cos_alpha_diff: f64,
) -> (Vec<SliceContactResult>, f64) {
    let mut results = Vec::with_capacity(slices.len());
    let mut q_total_outer = 0.0;

    for s in slices {
        let h1 = s.r_roller;
        let h2 = s.r_roller * 2.0;

        let gap_outer = delta_o - s.delta_z_total_outer;
        let gap_inner = delta_i - s.delta_z_total_inner;

        let (q_o, b_o, p_o, hb_o, kh_o) =
            hertz::single_raceway_contact(gap_outer, s.r_eq_outer, e_star_mpa, e_avg_mpa, nu, h1, h2);
        let (q_i, b_i, p_i, hb_i, kh_i) =
            hertz::single_raceway_contact(gap_inner, s.r_eq_inner, e_star_mpa, e_avg_mpa, nu, h1, h2);

        let in_contact = q_o > 0.0 || q_i > 0.0;
        if q_o > 0.0 { q_total_outer += q_o * s.slice_width; }

        // Combined slice stiffness along outer normal (Hertz-only 2-spring series):
        //   1/k = 1/k_hertz_outer + cos²/k_hertz_inner
        let cos_sq = cos_alpha_diff * cos_alpha_diff;
        let mut inv_k = 0.0;
        if kh_o > 0.0 { inv_k += 1.0 / kh_o; }
        if kh_i > 0.0 && cos_sq > 0.0 { inv_k += cos_sq / kh_i; }
        let k_combined = if inv_k > 0.0 { 1.0 / inv_k } else { 0.0 };

        results.push(SliceContactResult {
            k: s.k,
            delta_k: gap_outer.max(0.0),
            q_k: q_o,
            q_k_outer: q_o,
            q_k_inner: q_i,
            b_k: b_i,
            p_max_k: p_i,
            h_bulk_k: hb_i,
            k_hertz_k: kh_i,
            b_k_outer: b_o,
            p_max_k_outer: p_o,
            h_bulk_k_outer: hb_o,
            k_hertz_k_outer: kh_o,
            k_combined_k: k_combined,
            in_contact,
        });
    }

    (results, q_total_outer)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Create test slices with parabolic crown profile.
    /// `crown_um`: peak-to-edge crown drop [μm]. 0 = flat profile.
    fn make_test_slices(n: usize, crown_um: f64) -> (Vec<SliceGeometry>, Material) {
        let l_we = 15.0;
        let slice_width = l_we / n as f64;
        let slices: Vec<SliceGeometry> = (0..n)
            .map(|k| {
                let x = (k as f64 + 0.5) * slice_width;
                let frac = x / l_we;
                let r_roller = 4.0 + frac; // 4.0–5.0 mm taper
                let r_race = 200.0;
                let x_centered = x - l_we / 2.0;
                // Parabolic: Δz = crown_um * (2·x_centered / l_we)²
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
        let material = Material::default();
        (slices, material)
    }

    #[test]
    fn test_flat_profile_uniform_load() {
        let (slices, mat) = make_test_slices(20, 0.0);
        let (results, q_total) = solve_gen1_roller(&slices, 5.0, &mat, 0.0);

        // All slices in contact (flat profile, positive approach)
        assert!(results.iter().all(|r| r.in_contact));
        assert!(q_total > 0.0);

        // Load distribution should be relatively uniform (within 30% of mean)
        let q_values: Vec<f64> = results.iter().map(|r| r.q_k).collect();
        let q_mean = q_values.iter().sum::<f64>() / q_values.len() as f64;
        for q in &q_values {
            assert!(
                (*q - q_mean).abs() / q_mean < 0.30,
                "q_k={q:.1} deviates >30% from mean={q_mean:.1}"
            );
        }
    }

    #[test]
    fn test_crowned_profile_reduced_edge() {
        let (slices, mat) = make_test_slices(20, 2.0);
        let (results, _) = solve_gen1_roller(&slices, 5.0, &mat, 0.0);

        // Center slices (indices ~9,10) should have higher q_k than edge slices (0, 19)
        let q_center = results[10].q_k;
        let q_edge_small = results[0].q_k;
        let q_edge_large = results[19].q_k;

        assert!(
            q_center > q_edge_small,
            "Center q_k={q_center:.1} should > edge q_k={q_edge_small:.1}"
        );
        assert!(
            q_center > q_edge_large,
            "Center q_k={q_center:.1} should > edge q_k={q_edge_large:.1}"
        );
    }

    #[test]
    fn test_zero_approach_no_contact() {
        // With crown > 0, delta_rigid = 0 means delta_k ≤ 0 for all slices
        let (slices, mat) = make_test_slices(20, 5.0);
        let (results, q_total) = solve_gen1_roller(&slices, 0.0, &mat, 0.0);

        assert!(results.iter().all(|r| !r.in_contact));
        assert!((q_total - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_partial_contact() {
        // Large crown (10μm), small approach (3μm) → only center slices contact
        let (slices, mat) = make_test_slices(20, 10.0);
        let (results, q_total) = solve_gen1_roller(&slices, 3.0, &mat, 0.0);

        let n_contact = results.iter().filter(|r| r.in_contact).count();
        let n_no_contact = results.len() - n_contact;

        // Some slices should be in contact (center), some not (edges)
        assert!(n_contact > 0, "At least some center slices should be in contact");
        assert!(n_no_contact > 0, "Edge slices should not be in contact");
        assert!(q_total > 0.0);
    }

    #[test]
    fn test_total_load_summation() {
        let (slices, mat) = make_test_slices(20, 1.0);
        let (results, q_total) = solve_gen1_roller(&slices, 5.0, &mat, 0.0);

        // Manually sum q_k * l_k
        let q_sum: f64 = results
            .iter()
            .zip(slices.iter())
            .filter(|(r, _)| r.in_contact)
            .map(|(r, s)| r.q_k * s.slice_width)
            .sum();

        assert!(
            (q_total - q_sum).abs() < 1e-6,
            "Q_total={q_total:.6} should equal manual sum={q_sum:.6}"
        );
    }

    #[test]
    fn test_solve_for_target_load() {
        let (slices, mat) = make_test_slices(50, 2.0);
        let params = SolverParams::default(); // tol=1e-6, max_iter=100

        let q_target = 5000.0; // N
        let result = solve_gen1_for_load(&slices, q_target, &mat, &params, 0.0);
        assert!(result.is_ok(), "Should converge: {:?}", result.err());

        let (_, q_calc, delta_rigid) = result.unwrap();
        let rel_err = (q_calc - q_target).abs() / q_target;
        assert!(
            rel_err < 1e-4,
            "Q_calc={q_calc:.2}N vs target={q_target:.0}N, rel_err={rel_err:.2e}"
        );
        assert!(delta_rigid > 0.0, "delta_rigid should be positive");
    }

    #[test]
    fn test_load_increases_with_approach() {
        let (slices, mat) = make_test_slices(20, 1.0);

        let deltas = [1.0, 5.0, 10.0, 20.0];
        let loads: Vec<f64> = deltas
            .iter()
            .map(|&d| solve_gen1_roller(&slices, d, &mat, 0.0).1)
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

    // ─── Gen1 Split Tests ───

    fn make_asymmetric_slices(n: usize) -> (Vec<SliceGeometry>, Material) {
        let l_we = 15.0;
        let slice_width = l_we / n as f64;
        let d_pw = 70.0;
        let d_we_min = 8.5;
        let d_we_max = 10.0;
        let alpha: f64 = 12.0_f64.to_radians();
        let r_race = 200.0;
        let slices: Vec<SliceGeometry> = (0..n)
            .map(|k| {
                let x = (k as f64 + 0.5) * slice_width;
                let frac = x / l_we;
                let d_roller = d_we_min + (d_we_max - d_we_min) * frac;
                let r_roller = d_roller / 2.0;
                let gamma = d_roller * alpha.cos() / d_pw;
                let r_eq_inner = (d_roller / 2.0) * (1.0 - gamma);
                let r_eq_outer = (d_roller / 2.0) * (1.0 + gamma);
                let x_centered = x - l_we / 2.0;
                SliceGeometry {
                    k, x_axial: x, r_roller,
                    r_inner_race: r_race, r_outer_race: r_race,
                    r_eq_inner, r_eq_outer,
                    delta_z_total_inner: 0.0,   // flat inner
                    delta_z_total_outer: 3.0 * (2.0 * x_centered / l_we).powi(2), // crowned outer
                    slice_width,
                }
            })
            .collect();
        (slices, Material::default())
    }

    #[test]
    fn test_gen1_split_basic() {
        let (slices, mat) = make_asymmetric_slices(30);
        let cos_alpha_diff = 0.99;
        let (results, q_total) = solve_gen1_roller_split(&slices, 10.0, &mat, cos_alpha_diff);
        assert!(q_total > 0.0);
        assert!(results.iter().any(|r| r.q_k_outer > 0.0));
        assert!(results.iter().any(|r| r.q_k_inner > 0.0));
    }

    #[test]
    fn test_gen1_split_force_balance() {
        let (slices, mat) = make_asymmetric_slices(30);
        let cos_alpha_diff = 0.99;
        let (results, q_total_outer) = solve_gen1_roller_split(&slices, 10.0, &mat, cos_alpha_diff);
        let q_total_inner: f64 = results.iter().map(|r| r.q_k_inner * slices[r.k].slice_width).sum();
        let err = (q_total_outer - q_total_inner * cos_alpha_diff).abs() / q_total_outer.max(1.0);
        assert!(err < 0.02, "Force balance err={:.3}%", err * 100.0);
    }

    #[test]
    fn test_gen1_split_different_distributions() {
        let (slices, mat) = make_asymmetric_slices(30);
        let cos_alpha_diff = 0.99;
        let (results, _) = solve_gen1_roller_split(&slices, 10.0, &mat, cos_alpha_diff);

        let ratios: Vec<f64> = results.iter()
            .filter(|r| r.q_k_outer > 1.0 && r.q_k_inner > 1.0)
            .map(|r| r.q_k_outer / r.q_k_inner)
            .collect();

        if ratios.len() >= 3 {
            let r_max = ratios.iter().cloned().fold(f64::MIN, f64::max);
            let r_min = ratios.iter().cloned().fold(f64::MAX, f64::min);
            assert!(r_max - r_min > 0.01,
                "q_outer/q_inner should vary: min={r_min:.4}, max={r_max:.4}");
        }
    }

    #[test]
    fn test_gen1_split_vs_combined_total() {
        let (slices, mat) = make_test_slices(30, 3.0);
        let cos_alpha_diff = 0.99;
        let (_, q_combined) = solve_gen1_roller(&slices, 10.0, &mat, cos_alpha_diff);
        let (_, q_split) = solve_gen1_roller_split(&slices, 10.0, &mat, cos_alpha_diff);
        let rel = (q_split - q_combined).abs() / q_combined.max(1.0);
        assert!(rel < 0.15, "Split Q={q_split:.1} vs combined Q={q_combined:.1}, diff={:.1}%", rel * 100.0);
    }
}
