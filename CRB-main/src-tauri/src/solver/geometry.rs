use crate::error::SolverError;
use crate::solver::types::*;

/// Compute slice geometries by dividing roller effective length into n equal segments.
pub fn compute_slices(
    macro_geom: &MacroGeometry,
    raceway_geom: &RacewayGeometry,
    roller_profile: &RollerProfile,
    raceway_profile_inner: &RacewayProfile,
    raceway_profile_outer: &RacewayProfile,
    n_slices: usize,
) -> Result<Vec<SliceGeometry>, SolverError> {
    if n_slices == 0 {
        return Err(SolverError::InvalidInput("n_slices must be > 0".into()));
    }

    // CRB: 원통 roller — r_small = r_large = d_we/2, α = 0
    let l_we = macro_geom.l_we;
    let slice_width = l_we / n_slices as f64;
    let r_uniform = macro_geom.d_we / 2.0;
    let r_small = r_uniform;
    let r_large = r_uniform;
    let d_pw = macro_geom.d_pw;
    let alpha_i_rad: f64 = 0.0;     // CRB: raceway 원통 (α_i = 0)
    let alpha_o_rad: f64 = 0.0;     // CRB: raceway 원통 (α_o = 0)
    let _alpha_12 = (alpha_o_rad - alpha_i_rad) / 2.0; // = 0 for CRB
    let alpha_m = (alpha_i_rad + alpha_o_rad) / 2.0;   // = 0 for CRB

    let slices = (0..n_slices)
        .map(|k| {
            let x_axial = (k as f64 + 0.5) * slice_width;
            let frac = x_axial / l_we;

            // Roller radius at this slice (linear taper)
            let r_roller = r_small + (r_large - r_small) * frac;

            // Roller diameter and pitch diameter at this slice (Nguyen-Schäfer Eq 1.28)
            let x_centered = x_axial - l_we / 2.0;
            let d_k = 2.0 * r_roller; // could also use d_m + 2·x·tan(α_12)
            let d_pwk = d_pw + 2.0 * x_centered * alpha_m.sin();

            // Profile corrections per contact surface [μm]
            let dz_roller = roller_profile_correction(x_axial, roller_profile, l_we);
            let dz_inner = raceway_profile_correction(x_axial, raceway_profile_inner, l_we);
            let dz_outer = raceway_profile_correction(x_axial, raceway_profile_outer, l_we);

            // Effective roller radius adjusted by profile [mm]
            // Inner contact: roller + inner raceway profile
            // Outer contact: roller + outer raceway profile
            let r_roller_eff_inner = r_roller - (dz_roller + dz_inner) / 1000.0;
            let r_roller_eff_outer = r_roller - (dz_roller + dz_outer) / 1000.0;
            let d_k_eff_inner = 2.0 * r_roller_eff_inner;
            let d_k_eff_outer = 2.0 * r_roller_eff_outer;

            // TRB orbital curvature with profile-adjusted D_k:
            //   R_eq = (D_k_eff/2)·(1 ∓ γ),  γ = D_k_eff·cos(α)/D_pwk
            let gamma_i = d_k_eff_inner * alpha_i_rad.cos() / d_pwk;
            let gamma_o = d_k_eff_outer * alpha_o_rad.cos() / d_pwk;
            let r_eq_inner = (d_k_eff_inner / 2.0) * (1.0 - gamma_i);
            let r_eq_outer = (d_k_eff_outer / 2.0) * (1.0 + gamma_o);

            // Raceway curvature radii (stored for reference)
            let r_inner_race = raceway_geom.r_i;
            let r_outer_race = raceway_geom.r_o;

            SliceGeometry {
                k,
                x_axial,
                r_roller,
                r_inner_race,
                r_outer_race,
                r_eq_inner,
                r_eq_outer,
                delta_z_total_inner: dz_roller + dz_inner,
                delta_z_total_outer: dz_roller + dz_outer,
                slice_width,
            }
        })
        .collect();

    Ok(slices)
}

/// Compute roller profile correction at axial position x [mm].
/// Returns correction in [μm].
fn roller_profile_correction(x: f64, profile: &RollerProfile, l_we: f64) -> f64 {
    let half_l = l_we / 2.0;
    let x_centered = x - half_l; // centered coordinate

    // Crown correction
    let dz_crown = crown_correction(x_centered, &profile.crown_type, half_l, profile.delta_c);

    // Dub-off correction
    let dz_dub = dub_off_correction(x, profile, l_we);

    dz_crown + dz_dub
}

/// Crown correction based on crown type.
/// x_centered: distance from roller center [mm]
/// half_l: half of effective length [mm]
/// delta_c: crown drop [μm] — MASTER PARAMETER that determines the profile shape.
///
/// For each crown type, the type-specific parameter is derived from delta_c:
///   - Parabolic: c₂ = δ_c / half_l²
///   - Circular:  R_crown = half_l² / (2·δ_c/1000)  [mm]
///   - Logarithmic: A_log = δ_c / ln(1/(1-ref²)), ref=0.9
///   - Custom: uses data directly (δ_c ignored)
fn crown_correction(x_centered: f64, crown_type: &CrownType, half_l: f64, delta_c: f64) -> f64 {
    if delta_c <= 0.0 && !matches!(crown_type, CrownType::Custom { .. } | CrownType::Polynomial { .. }) {
        return 0.0;
    }
    let hl2 = half_l * half_l;

    match crown_type {
        CrownType::Logarithmic { .. } => {
            // Derive A_log from delta_c at reference position (90% of half-length)
            let ref_ratio: f64 = 0.9;
            let a_log = delta_c / (1.0 / (1.0 - ref_ratio * ref_ratio)).ln();
            let ratio = (x_centered / half_l).powi(2);
            if ratio >= 0.999 {
                return delta_c;
            }
            a_log * (1.0 / (1.0 - ratio)).ln()
        }
        CrownType::Circular { .. } => {
            // Derive R_crown from delta_c: δ_c/1000 ≈ half_l²/(2R) → R = half_l²/(2·δ_c/1000)
            let r_crown = hl2 / (2.0 * delta_c / 1000.0);
            let x2 = x_centered * x_centered;
            let r2 = r_crown * r_crown;
            if x2 >= r2 {
                return delta_c;
            }
            (r_crown - (r2 - x2).sqrt()) * 1000.0
        }
        CrownType::Parabolic { .. } => {
            // Derive c₂ from delta_c: δ_c = c₂·half_l² → c₂ = δ_c/half_l²
            let c2 = delta_c / hl2;
            c2 * x_centered * x_centered
        }
        CrownType::Custom { profile } => {
            if profile.len() < 2 {
                return 0.0;
            }
            cubic_spline_interpolate(profile, x_centered + half_l)
        }
        CrownType::Polynomial { coeffs } => {
            // Measured profile convention: negative = concave (crown shape).
            // Solver convention: positive delta_z = gap increase (crown drop).
            // Negate to convert measured → solver convention.
            let x = x_centered;
            let p1 = coeffs.first().copied().unwrap_or(0.0);
            let p2 = coeffs.get(1).copied().unwrap_or(0.0);
            let p3 = coeffs.get(2).copied().unwrap_or(0.0);
            let p4 = coeffs.get(3).copied().unwrap_or(0.0);
            let p5 = coeffs.get(4).copied().unwrap_or(0.0);
            -(((p1 * x + p2) * x + p3) * x + p4) * x - p5
        }
    }
}

/// Dub-off correction at axial position x [mm].
/// Returns additional correction in [μm].
/// CRB: dub-off 양쪽 대칭 — 단일 (delta_dub, l_dub) 로 양 끝 동일 적용 (부록 A.1).
fn dub_off_correction(x: f64, profile: &RollerProfile, l_we: f64) -> f64 {
    let mut dz = 0.0;

    // Small end dub-off (x near 0)
    if profile.l_dub > 0.0 && x < profile.l_dub {
        let ratio = 1.0 - x / profile.l_dub;
        dz += profile.delta_dub * ratio * ratio;
    }

    // Large end dub-off (x near l_we) — 대칭 적용
    let x_from_large = l_we - x;
    if profile.l_dub > 0.0 && x_from_large < profile.l_dub {
        let ratio = 1.0 - x_from_large / profile.l_dub;
        dz += profile.delta_dub * ratio * ratio;
    }

    dz
}

/// Raceway profile correction at axial position x [mm].
/// Returns correction in [μm].
fn raceway_profile_correction(x: f64, profile: &RacewayProfile, l_we: f64) -> f64 {
    let half_l = l_we / 2.0;
    let x_centered = x - half_l;

    // Simple parabolic crowning
    let dz_crown = if half_l > 0.0 {
        profile.delta_rw * (x_centered / half_l).powi(2)
    } else {
        0.0
    };

    // Custom profile override
    let dz_custom = match &profile.custom_profile {
        Some(data) if data.len() >= 2 => cubic_spline_interpolate(data, x),
        _ => 0.0,
    };

    // Polynomial profile (negate: measured negative = concave → solver positive = gap)
    let dz_poly = match &profile.polynomial_coeffs {
        Some(coeffs) if !coeffs.is_empty() => {
            let xc = x - half_l;
            let p1 = coeffs.first().copied().unwrap_or(0.0);
            let p2 = coeffs.get(1).copied().unwrap_or(0.0);
            let p3 = coeffs.get(2).copied().unwrap_or(0.0);
            let p4 = coeffs.get(3).copied().unwrap_or(0.0);
            let p5 = coeffs.get(4).copied().unwrap_or(0.0);
            -(((p1 * xc + p2) * xc + p3) * xc + p4) * xc - p5
        }
        _ => 0.0,
    };

    dz_crown + dz_custom + dz_poly
}

/// Natural cubic spline interpolation using Thomas algorithm.
/// data_points: sorted (x, y) pairs
/// x: interpolation point
pub fn cubic_spline_interpolate(data_points: &[(f64, f64)], x: f64) -> f64 {
    let n = data_points.len();
    if n == 0 {
        return 0.0;
    }
    if n == 1 {
        return data_points[0].1;
    }

    // Clamp to data range
    if x <= data_points[0].0 {
        return data_points[0].1;
    }
    if x >= data_points[n - 1].0 {
        return data_points[n - 1].1;
    }

    if n == 2 {
        // Linear interpolation
        let (x0, y0) = data_points[0];
        let (x1, y1) = data_points[1];
        let t = (x - x0) / (x1 - x0);
        return y0 + t * (y1 - y0);
    }

    // Compute intervals
    let m = n - 1; // number of intervals
    let h: Vec<f64> = (0..m).map(|i| data_points[i + 1].0 - data_points[i].0).collect();

    // Build tridiagonal system for second derivatives (natural spline: S''(0) = S''(n) = 0)
    let mut alpha = vec![0.0; m + 1];
    for i in 1..m {
        alpha[i] = (3.0 / h[i]) * (data_points[i + 1].1 - data_points[i].1)
            - (3.0 / h[i - 1]) * (data_points[i].1 - data_points[i - 1].1);
    }

    // Thomas algorithm (solve tridiagonal)
    let mut c = vec![0.0; n];
    let mut l = vec![1.0; n];
    let mut mu = vec![0.0; n];
    let mut z = vec![0.0; n];

    for i in 1..m {
        l[i] = 2.0 * (data_points[i + 1].0 - data_points[i - 1].0) - h[i - 1] * mu[i - 1];
        mu[i] = h[i] / l[i];
        z[i] = (alpha[i] - h[i - 1] * z[i - 1]) / l[i];
    }

    // Back substitution
    for j in (0..m).rev() {
        c[j] = z[j] - mu[j] * c[j + 1];
    }

    // Compute b and d coefficients
    let b: Vec<f64> = (0..m)
        .map(|i| {
            (data_points[i + 1].1 - data_points[i].1) / h[i]
                - h[i] * (c[i + 1] + 2.0 * c[i]) / 3.0
        })
        .collect();
    let d: Vec<f64> = (0..m).map(|i| (c[i + 1] - c[i]) / (3.0 * h[i])).collect();

    // Find interval
    let mut idx = 0;
    for i in 0..m {
        if x >= data_points[i].0 && x <= data_points[i + 1].0 {
            idx = i;
            break;
        }
    }

    // Evaluate spline
    let dx = x - data_points[idx].0;
    data_points[idx].1 + b[idx] * dx + c[idx] * dx * dx + d[idx] * dx * dx * dx
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_geometry() -> (MacroGeometry, RacewayGeometry, RollerProfile, RacewayProfile) {
        // CRB 테스트 지오메트리 (Phase 1: NU 시리즈 유사 파라미터)
        let macro_geom = MacroGeometry {
            d: 50.0,
            outer_diameter: 90.0,
            t: 20.0,
            z: 20,
            d_we: 9.0,       // 균일 원통 roller diameter
            l_we: 15.0,
            d_pw: 70.0,
            g_r: 10.0,
        };
        let raceway_geom = RacewayGeometry {
            r_i: 1.0e9,      // 원통 raceway — transverse 곡률 무한대 근사
            r_o: 1.0e9,
            d_uc: 0.0,
            l_uc: 0.0,
        };
        let roller_profile = RollerProfile {
            crown_type: CrownType::Parabolic { c2: 0.01 },
            delta_c: 5.0,
            delta_dub: 10.0,  // 양쪽 대칭
            l_dub: 2.0,
            sigma_roller: 0.15,
        };
        let raceway_profile = RacewayProfile {
            delta_rw: 0.0,
            w_a: 0.0,
            ra: 0.3,
            custom_profile: None,
            polynomial_coeffs: None,
        };
        (macro_geom, raceway_geom, roller_profile, raceway_profile)
    }

    #[test]
    fn test_uniform_slicing() {
        let (mg, rg, rp, rwp) = make_test_geometry();
        let slices = compute_slices(&mg, &rg, &rp, &rwp, &rwp, 10).unwrap();

        assert_eq!(slices.len(), 10);
        assert!((slices[0].slice_width - 1.5).abs() < 1e-10); // 15 / 10
        assert!((slices[0].x_axial - 0.75).abs() < 1e-10);    // center of first slice
        assert!((slices[9].x_axial - 14.25).abs() < 1e-10);   // center of last slice
    }

    #[test]
    fn test_uniform_radius_crb() {
        // CRB: 모든 slice 의 roller 반경이 균일 (= d_we/2)
        let (mg, rg, rp, rwp) = make_test_geometry();
        let slices = compute_slices(&mg, &rg, &rp, &rwp, &rwp, 10).unwrap();
        let r_expected = mg.d_we / 2.0;
        for s in &slices {
            assert!((s.r_roller - r_expected).abs() < 1e-10);
        }
    }

    #[test]
    fn test_equivalent_radius() {
        let (mg, rg, rp, rwp) = make_test_geometry();
        let slices = compute_slices(&mg, &rg, &rp, &rwp, &rwp, 10).unwrap();

        for s in &slices {
            // R_eq = r1*r2 / (r1+r2) should be positive and < min(r1, r2)
            assert!(s.r_eq_inner > 0.0);
            assert!(s.r_eq_inner < s.r_roller.min(s.r_inner_race));
        }
    }

    #[test]
    fn test_cubic_spline_linear_data() {
        // Spline through linear data should reproduce linear function
        let data = vec![(0.0, 0.0), (5.0, 10.0), (10.0, 20.0)];
        let y = cubic_spline_interpolate(&data, 2.5);
        assert!((y - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_cubic_spline_quadratic_data() {
        // Spline through quadratic y=x² at x=0,1,2,3,4
        let data: Vec<(f64, f64)> = (0..=4).map(|x| (x as f64, (x * x) as f64)).collect();
        let y = cubic_spline_interpolate(&data, 1.5);
        assert!((y - 2.25).abs() < 0.1); // should be close to 2.25
    }

    #[test]
    fn test_cubic_spline_boundary_clamp() {
        let data = vec![(1.0, 5.0), (3.0, 7.0), (5.0, 3.0)];
        assert!((cubic_spline_interpolate(&data, 0.0) - 5.0).abs() < 1e-10); // clamp low
        assert!((cubic_spline_interpolate(&data, 6.0) - 3.0).abs() < 1e-10); // clamp high
    }

    #[test]
    fn test_parabolic_crown() {
        // δ_c is master: Parabolic Δz = (δ_c/half_l²) * x²
        let delta_c = 5.0; // μm
        let half_l = 7.5;
        // At edge (x = half_l): should return δ_c
        let dz = crown_correction(half_l, &CrownType::Parabolic { c2: 0.0 }, half_l, delta_c);
        assert!((dz - delta_c).abs() < 1e-10);

        // Center should be zero
        let dz_center = crown_correction(0.0, &CrownType::Parabolic { c2: 0.0 }, half_l, delta_c);
        assert!(dz_center.abs() < 1e-10);

        // Mid-point (x = half_l/2): should be δ_c/4
        let dz_mid = crown_correction(half_l / 2.0, &CrownType::Parabolic { c2: 0.0 }, half_l, delta_c);
        assert!((dz_mid - delta_c / 4.0).abs() < 1e-10);

        // δ_c = 0 → flat profile
        let dz_flat = crown_correction(half_l, &CrownType::Parabolic { c2: 0.0 }, half_l, 0.0);
        assert!(dz_flat.abs() < 1e-10);
    }

    #[test]
    fn test_dub_off_crb_symmetric() {
        // CRB: dub-off 대칭 — 양 끝에서 같은 값
        let profile = RollerProfile {
            crown_type: CrownType::Parabolic { c2: 0.0 },
            delta_c: 0.0,
            delta_dub: 15.0,
            l_dub: 2.5,
            sigma_roller: 0.15,
        };
        let l_we = 15.0;

        // At x=0 (small end edge): full dub-off
        let dz_small = dub_off_correction(0.0, &profile, l_we);
        assert!((dz_small - 15.0).abs() < 1e-10);

        // At x=l_we (large end edge): 대칭 → same value
        let dz_large = dub_off_correction(l_we, &profile, l_we);
        assert!((dz_large - 15.0).abs() < 1e-10);

        // At center: no dub-off
        let dz_center = dub_off_correction(l_we / 2.0, &profile, l_we);
        assert!(dz_center.abs() < 1e-10);
    }

    #[test]
    fn test_zero_slices_error() {
        let (mg, rg, rp, rwp) = make_test_geometry();
        assert!(compute_slices(&mg, &rg, &rp, &rwp, &rwp, 0).is_err());
    }

    #[test]
    fn test_roller_inertia_cylinder() {
        // Cylindrical roller (R_max = R_min) → I = (1/2)mR²
        let rho = 7850.0; // kg/m³
        let r = 5.0;      // mm → 0.005 m
        let l = 10.0;     // mm → 0.010 m
        let i = compute_roller_inertia(r, r, l, rho);
        let r_m = r * 1e-3;
        let l_m = l * 1e-3;
        let mass = rho * std::f64::consts::PI * r_m * r_m * l_m;
        let i_expected = 0.5 * mass * r_m * r_m;
        assert!((i - i_expected).abs() / i_expected < 1e-6,
            "Cylinder: I={:.6e}, expected={:.6e}", i, i_expected);
    }

    #[test]
    fn test_roller_inertia_tapered() {
        // Tapered roller: R_max > R_min, I should be between cylinder limits
        let rho = 7850.0;
        let r_min = 4.0;  // mm
        let r_max = 5.0;  // mm
        let l = 10.0;     // mm
        let i = compute_roller_inertia(r_max, r_min, l, rho);
        let i_small = compute_roller_inertia(r_min, r_min, l, rho);
        let i_large = compute_roller_inertia(r_max, r_max, l, rho);
        assert!(i > i_small && i < i_large,
            "Tapered I={:.6e} should be between {:.6e} and {:.6e}", i, i_small, i_large);
    }
}

/// Compute moment of inertia for a tapered (frustum) roller about its spin axis.
///
/// Uses the exact frustum formula:
///   I = (π/10) × ρ × L × (R_max⁵ − R_min⁵) / (R_max − R_min)
/// For cylinder (R_max = R_min): I = (1/2) × m × R²
///
/// # Arguments
/// * `r_max_mm` — Large-end radius [mm]
/// * `r_min_mm` — Small-end radius [mm]
/// * `l_mm` — Roller effective length [mm]
/// * `rho_kg_m3` — Material density [kg/m³] (bearing steel ~7850)
///
/// # Returns
/// Moment of inertia [kg·m²]
pub fn compute_roller_inertia(r_max_mm: f64, r_min_mm: f64, l_mm: f64, rho_kg_m3: f64) -> f64 {
    let r_max = r_max_mm * 1e-3; // [m]
    let r_min = r_min_mm * 1e-3;
    let l = l_mm * 1e-3;

    if (r_max - r_min).abs() < 1e-12 {
        // Cylindrical roller: I = (1/2) × ρ × π × R⁴ × L
        0.5 * rho_kg_m3 * std::f64::consts::PI * r_max.powi(4) * l
    } else {
        // Frustum: I = (π/10) × ρ × L × (R_max⁵ − R_min⁵) / (R_max − R_min)
        std::f64::consts::PI / 10.0 * rho_kg_m3 * l
            * (r_max.powi(5) - r_min.powi(5)) / (r_max - r_min)
    }
}
