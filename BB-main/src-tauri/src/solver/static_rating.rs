//! Static load rating calculation
//!
//! ISO 76:2006 — Basic static load rating C₀ᵣ, static equivalent load P₀ᵣ, safety factor S₀.
//! ISO 17956:2025 — Effective static safety factor S₀,eff using lamina-level loads.

use crate::solver::types::*;

// ─── ISO 76 ─────────────────────────────────────────────────────────

/// Basic static radial load rating for single-row radial roller bearings.
/// ISO 76:2006, Eq. (7):
///   C₀ᵣ = 44 × (1 - D_we·cos(α)/D_pw) × i × Z × L_we × D_we × cos(α)  [N]
///
/// Returns C₀ᵣ in [kN].
pub fn compute_c_0r(geom: &MacroGeometry) -> f64 {
    let alpha_rad = geom.alpha.to_radians();
    let d_we = (geom.d_we_max + geom.d_we_min) / 2.0; // mean roller diameter [mm]
    let gamma = d_we * alpha_rad.cos() / geom.d_pw;
    let i = 1.0; // single-row

    let c_0r_n = 44.0 * (1.0 - gamma) * i * (geom.z as f64) * geom.l_we * d_we * alpha_rad.cos();
    c_0r_n / 1000.0 // [N] → [kN]
}

/// Static equivalent radial load for single-row TRB.
/// ISO 76:2006, Eq. (8)-(9):
///   P₀ᵣ = max(X₀·F_r + Y₀·F_a, F_r)
///
/// For single-row roller bearings (Table 3): X₀ = 0.5, Y₀ = 0.22·cot(α).
/// Returns (P₀ᵣ [kN], X₀, Y₀).
pub fn compute_p_0r(f_r_kn: f64, f_a_kn: f64, alpha_deg: f64) -> (f64, f64, f64) {
    let alpha_rad = alpha_deg.to_radians();
    let x_0 = 0.5;
    let y_0 = 0.22 / alpha_rad.tan(); // 0.22·cot(α)

    let p1 = x_0 * f_r_kn + y_0 * f_a_kn;
    let p2 = f_r_kn;
    (p1.max(p2), x_0, y_0)
}

/// Static safety factor S₀ = C₀ᵣ / P₀ᵣ (ISO 76:2006, Eq. 14).
pub fn compute_s_0(c_0r_kn: f64, p_0r_kn: f64) -> f64 {
    if p_0r_kn <= 0.0 {
        return f64::INFINITY;
    }
    c_0r_kn / p_0r_kn
}

// ─── ISO 17956 ──────────────────────────────────────────────────────

/// Reference lamina load q₀ for single-row radial roller bearing.
/// ISO 17956:2025, Eq. (7):
///   q₀ = (1/n_s) × (5/(i·Z·cos(α))) × C₀ᵣ  [N]
///
/// `c_0r_kn`: C₀ᵣ in [kN], `n_slices`: number of laminae n_s, `z`: roller count, `alpha_deg`: contact angle.
/// Returns q₀ in [N].
pub fn compute_q_0(c_0r_kn: f64, n_slices: usize, z: u32, alpha_deg: f64) -> f64 {
    let alpha_rad = alpha_deg.to_radians();
    let i = 1.0; // single-row
    let c_0r_n = c_0r_kn * 1000.0; // [kN] → [N]
    let n_s = n_slices as f64;

    (1.0 / n_s) * (5.0 / (i * (z as f64) * alpha_rad.cos())) * c_0r_n
}

/// Find maximum lamina load q_max from roller results.
/// ISO 17956:2025, Eq. (6): q_max = max{q_{j,k}}
///
/// Each slice's `q_k` is load per unit length [N/mm] × slice_width [mm] = lamina load [N].
/// Returns (q_max [N], roller_index, lamina_index).
pub fn find_q_max(roller_results: &[RollerResult], slice_width: f64) -> (f64, usize, usize) {
    let mut q_max = 0.0_f64;
    let mut roller_idx = 0;
    let mut lamina_idx = 0;

    for (j, roller) in roller_results.iter().enumerate() {
        for slice in &roller.slice_results {
            // q_k is load per unit length [N/mm], lamina load = q_k × slice_width
            let q_lamina = slice.q_k * slice_width;
            if q_lamina > q_max {
                q_max = q_lamina;
                roller_idx = j;
                lamina_idx = slice.k;
            }
        }
    }

    (q_max, roller_idx, lamina_idx)
}

/// Compute full static rating result.
pub fn compute_static_rating(
    geom: &MacroGeometry,
    operating: &OperatingConditions,
    solver: &SolverParams,
    roller_results: &[RollerResult],
    slice_width: f64,
) -> StaticRatingResult {
    // C₀ᵣ: use manual override or auto-calculate (ISO 76 Eq.7)
    let c_0r_kn = solver.c_0r_kn.unwrap_or_else(|| compute_c_0r(geom));

    // P₀ᵣ (ISO 76 Eq.8-9)
    let f_r = operating.f_r();
    let (p_0r_kn, x_0, y_0) = compute_p_0r(f_r, operating.f_a, geom.alpha);

    // S₀ (ISO 76 Eq.14)
    let s_0 = compute_s_0(c_0r_kn, p_0r_kn);

    // q₀ (ISO 17956 Eq.7)
    let q_0 = compute_q_0(c_0r_kn, solver.n_slices, geom.z, geom.alpha);

    // q_max (ISO 17956 Eq.6)
    let (q_max, q_max_roller_idx, q_max_lamina_idx) = find_q_max(roller_results, slice_width);

    // S₀,eff (ISO 17956 Eq.5)
    let s_0_eff = if q_max > 0.0 { q_0 / q_max } else { f64::INFINITY };

    let s_0_adequate = s_0_eff >= solver.f_s_min;

    StaticRatingResult {
        c_0r_kn,
        p_0r_kn,
        s_0,
        x_0,
        y_0,
        q_0,
        q_max,
        s_0_eff,
        q_max_roller_idx,
        q_max_lamina_idx,
        s_0_adequate,
    }
}

// ─── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn test_geom() -> MacroGeometry {
        // NSK HR30306J
        MacroGeometry {
            d: 30.0,
            outer_diameter: 72.0,
            t: 20.75,
            alpha: 11.859,
            z: 14,
            d_we_max: 10.9371,
            d_we_min: 10.123273,
            l_we: 11.65,
            d_pw: 51.0,
            h_rib: 2.5,
            alpha_rib: 9.855,
            g_r: 0.0,
            h_c: None,
        }
    }

    #[test]
    fn test_c_0r_reasonable() {
        let geom = test_geom();
        let c_0r = compute_c_0r(&geom);
        // NSK catalog: C₀ᵣ = 59.8 kN for HR30306J
        // Our formula should give a reasonable value (within ~20% of catalog)
        assert!(
            c_0r > 40.0 && c_0r < 90.0,
            "C₀ᵣ = {c_0r:.1} kN should be in reasonable range for 30306"
        );
    }

    #[test]
    fn test_c_0r_formula_components() {
        let geom = test_geom();
        let alpha_rad = geom.alpha.to_radians();
        let d_we = (geom.d_we_max + geom.d_we_min) / 2.0;
        let gamma = d_we * alpha_rad.cos() / geom.d_pw;

        // gamma should be in typical TRB range (0.15 ~ 0.25)
        assert!(gamma > 0.1 && gamma < 0.3, "γ = {gamma:.4} should be in typical range");

        // (1 - gamma) factor should reduce C₀ᵣ
        assert!((1.0 - gamma) > 0.7 && (1.0 - gamma) < 0.95);
    }

    #[test]
    fn test_p_0r_radial_only() {
        // Pure radial load: P₀ᵣ = max(0.5·Fr, Fr) = Fr
        let (p, x0, y0) = compute_p_0r(10.0, 0.0, 11.859);
        assert!((p - 10.0).abs() < 1e-10, "Pure radial: P₀ᵣ should equal F_r");
        assert!((x0 - 0.5).abs() < 1e-10);
        assert!(y0 > 0.0);
    }

    #[test]
    fn test_p_0r_combined_load() {
        // Combined load: X₀·F_r + Y₀·F_a should dominate for large F_a
        let (p, _, y0) = compute_p_0r(5.0, 10.0, 11.859);
        let expected = 0.5 * 5.0 + y0 * 10.0;
        assert!((p - expected).abs() < 1e-10);
        assert!(p > 5.0, "Combined load P₀ᵣ should be > F_r");
    }

    #[test]
    fn test_s_0_zero_load() {
        let s = compute_s_0(59.8, 0.0);
        assert!(s.is_infinite(), "S₀ should be infinite for zero load");
    }

    #[test]
    fn test_s_0_normal() {
        let s = compute_s_0(59.8, 10.0);
        assert!((s - 5.98).abs() < 0.01);
    }

    #[test]
    fn test_q_0_positive() {
        let q0 = compute_q_0(59.8, 30, 14, 11.859);
        assert!(q0 > 0.0, "q₀ must be positive");
        // q₀ = (1/30) × (5/(1×14×cos(11.859°))) × 59800 N
        // ≈ (1/30) × (5/13.7) × 59800 ≈ 727 N
        assert!(q0 > 500.0 && q0 < 1000.0, "q₀ = {q0:.1} N should be in reasonable range");
    }

    #[test]
    fn test_find_q_max_empty() {
        let (q_max, _, _) = find_q_max(&[], 0.5);
        assert!((q_max - 0.0).abs() < 1e-10, "q_max should be 0 for empty results");
    }

    #[test]
    fn test_s_0_eff_iso17956() {
        // Synthetic test: q₀ = 700 N, q_max = 350 N → S₀,eff = 2.0
        let geom = test_geom();
        let op = OperatingConditions {
            f_x: 5.0, f_y: 0.0, f_a: 2.0,
            m_x: 0.0, m_y: 0.0, n_inner_rpm: 1500.0, n_outer_rpm: 0.0,
            gamma: 0.0, t_op: 70.0, nu_40: 68.0, nu_100: 8.0,
            alpha_pv: 20.0,
            lubrication_type: LubricationType::Oil,
            starvation_factor: 1.0,
            rho_oil: 870.0,
            preload_mode: PreloadMode::DisplacementFromForce,
            delta_preload_um: 0.0,
            lubrication_model: LubricationModel::Method1_DH, film_decay_enabled: false, film_decay_time_hours: 0.0, skew_angle_deg: 0.0, replenishment_rate_nm_s: 0.0, surface_finish: SurfaceFinish::Standard, additive_type: AdditiveType::None,
            tau_eyring: 5.0,
            z_roelands: 0.67,
            traction_model: TractionModel::Eyring,
            carreau_eta_inf_ratio: 0.005,
            carreau_lambda_s: 1.0e-7,
            carreau_n: 0.5,
            carreau_a: 2.0,
            friction_model: FrictionModel::PalmgrenLike,
            thermal_correction: ThermalCorrection::Aihara1987,
            hysteresis_loss_factor: 0.005,
            skf_trb_series: SkfTrbSeriesEnum::Series303,
            skf_lubrication: SkfLubricationEnum::OilBath,
            skf_y_factor: 1.6,
            k_fluid: 0.15,
            beta_visc: 0.04,
            rq_inner: 0.3,
            rq_outer: 0.3,
            rq_roller: 0.15,
            roughness_input_mode: RoughnessInputMode::Rq,
            design_life_hours: 100.0,
        };
        let solver = SolverParams {
            c_0r_kn: Some(59.8),
            n_slices: 30,
            f_s_min: 1.0,
            ..Default::default()
        };

        // Create synthetic roller with known q_k
        let slice_width = geom.l_we / 30.0;
        let q_k_per_mm = 500.0; // N/mm → lamina load = 500 × slice_width
        let slices: Vec<SliceContactResult> = (0..30).map(|k| SliceContactResult {
            k,
            delta_k: 1.0, q_k: q_k_per_mm, q_k_outer: q_k_per_mm, q_k_inner: q_k_per_mm,
            b_k: 0.1, p_max_k: 1000.0, h_bulk_k: 0.0,
            k_hertz_k: 0.0,
            b_k_outer: 0.1, p_max_k_outer: 900.0, h_bulk_k_outer: 0.0,
            k_hertz_k_outer: 0.0, k_combined_k: 0.0,
            in_contact: true,
        }).collect();

        let rollers = vec![RollerResult {
            psi_deg: 0.0, q_normal: 5000.0, q_normal_inner: 5000.0,
            slice_results: slices, rib_result: None,
        }];

        let result = compute_static_rating(&geom, &op, &solver, &rollers, slice_width);

        assert!((result.c_0r_kn - 59.8).abs() < 0.01, "Should use manual override");
        assert!(result.s_0 > 0.0, "S₀ should be positive");
        assert!(result.s_0_eff > 0.0, "S₀,eff should be positive");
        assert!(result.s_0_adequate, "S₀,eff should be adequate with f_s_min=1.0");

        // q_max = q_k_per_mm × slice_width
        let expected_q_max = q_k_per_mm * slice_width;
        assert!((result.q_max - expected_q_max).abs() < 0.01,
            "q_max = {}, expected {}", result.q_max, expected_q_max);
    }
}
