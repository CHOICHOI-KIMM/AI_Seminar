use std::f64::consts::PI;

use crate::error::SolverError;
use crate::solver::hertz;
use crate::solver::lubrication::{
    classify_lambda, compute_rib_speeds, flash_temperature, greenwood_tripp_load_sharing,
    hamrock_dowson_elliptical, thermal_correction_murch_wilson, traction_coefficient,
};
use crate::solver::life::viscosity_at_temp_pub;
use crate::solver::types::*;

/// Boundary friction coefficient at the rib contact (mineral oil + EP additive).
const RIB_MU_BOUNDARY: f64 = 0.10;

/// Resolve contact height h_c on the rib face [mm].
///
/// h_c is the height above the rib base (roller large-end edge) where the
/// roller spherical end actually contacts the rib face.
///
/// - If `h_c_input` is Some(value), use the user-specified value (clamped to [0, h_rib]).
/// - If `h_c_input` is None, default to h_rib / 2 (midpoint of rib face).
///
/// Note: Liu 2023 treats h_c as a design input (nominal contact height), not
/// a computed output. The previous Eq.8 implementation was removed because the
/// formula describes geometry AT a given h_c, not a way to derive h_c.
pub fn resolve_contact_height(h_c_input: Option<f64>, h_rib: f64) -> f64 {
    match h_c_input {
        Some(v) => v.clamp(0.0, h_rib),
        None => h_rib / 2.0,
    }
}

/// Compute the radial position of the rib contact point from the bearing axis.
///
/// Two-part calculation:
/// 1. Rib base position (roller inner edge at large end):
///      r_base = d_pw/2 + (l_we/2)·sin(γ) − (d_we_max/2)·cos(γ)
///
/// 2. Contact height h_c: user input or default (h_rib / 2)
///      r_contact = r_base + h_c
///
/// Returns (r_contact, r_base, h_c) [mm].
pub fn compute_rib_contact_radius(
    d_pw: f64,
    l_we: f64,
    d_we_max: f64,
    alpha_i_deg: f64,
    alpha_o_deg: f64,
    h_c_input: Option<f64>,
    h_rib: f64,
) -> (f64, f64, f64) {
    let gamma = (alpha_i_deg + alpha_o_deg) / 2.0 * PI / 180.0;
    let r_base = d_pw / 2.0 + (l_we / 2.0) * gamma.sin() - (d_we_max / 2.0) * gamma.cos();
    let h_c = resolve_contact_height(h_c_input, h_rib);
    (r_base + h_c, r_base, h_c)
}

/// Compute circumferential curvature radius from r_contact and rib angle.
///
/// For a surface of revolution, the circumferential principal curvature radius is:
///   R_circ = r / sin(ψ)
/// where ψ is the angle between the surface normal and the rotation axis.
///
/// α_rib is measured from the RADIAL direction (e.g. 9.855°).
/// The rib face is nearly radial → the surface normal is nearly axial.
/// Normal-to-axis angle ψ = α_rib (not 90°−α_rib).
/// Therefore sin(ψ) = sin(α_rib).
///
/// Returns R_rib_circ = r_contact / sin(α_rib) [mm].
pub fn compute_rib_circ_auto(r_contact: f64, alpha_rib_deg: f64) -> f64 {
    let sin_a = (alpha_rib_deg * PI / 180.0).sin();
    if sin_a.abs() < 1e-6 {
        return 1e12;
    }
    r_contact / sin_a
}

/// Compute equivalent radii for rib contact (sphere vs. toroidal rib surface).
///
/// - r_sph: roller large-end sphere radius [mm] (convex, same in both directions)
/// - r_rib: rib fillet radius in meridional plane [mm] (concave, must be > r_sph)
/// - r_rib_circ: rib curvature radius in circumferential direction [mm] (concave).
///   Defined by axial cross-section at the contact point: R_circ = r_contact / sin(α_rib).
///
/// Both directions are convex-concave (conforming) contact:
///   R_x = 1/(1/r_sph − 1/r_rib)       (meridional)
///   R_y = 1/(1/r_sph − 1/r_rib_circ)   (circumferential)
///
/// Returns (R_x, R_y) [mm].
pub fn rib_equivalent_radii(
    r_sph: f64,
    r_rib: f64,
    r_rib_circ: f64,
) -> Result<(f64, f64), SolverError> {
    if r_sph <= 0.0 || r_rib <= 0.0 {
        return Err(SolverError::InvalidGeometry(
            "Sphere and rib radii must be positive".into(),
        ));
    }
    if r_sph >= r_rib {
        return Err(SolverError::InvalidGeometry(format!(
            "Sphere radius ({r_sph:.3} mm) must be less than rib radius ({r_rib:.3} mm)"
        )));
    }
    // Meridional plane: convex sphere in concave rib fillet
    let r_x = 1.0 / (1.0 / r_sph - 1.0 / r_rib);
    // Circumferential: convex sphere in concave rib ring
    let r_y = if r_rib_circ > r_sph && r_rib_circ < 1e10 {
        1.0 / (1.0 / r_sph - 1.0 / r_rib_circ)
    } else if r_rib_circ > 0.0 && r_rib_circ <= r_sph {
        // r_rib_circ ≤ r_sph: too conforming, cap at large value
        r_sph * 10.0
    } else {
        r_sph // r_rib_circ = ∞ → R_y = R_sph (flat limit)
    };
    Ok((r_x, r_y))
}

/// Hamrock-Brewe approximation for elliptical Hertz coefficients.
/// r_x, r_y: equivalent radii [mm] (r_y / r_x >= 1)
/// Returns (k_e, F_e, E_e):
///   k_e: ellipticity ratio a/b
///   F_e: complete elliptic integral of the first kind (approx)
///   E_e: complete elliptic integral of the second kind (approx)
pub fn hertz_elliptical_coefficients(r_x: f64, r_y: f64) -> (f64, f64, f64) {
    // Ensure ratio >= 1
    let (rx, ry) = if r_y >= r_x { (r_x, r_y) } else { (r_y, r_x) };
    let ratio = ry / rx;

    let k_e = 1.0339 * ratio.powf(0.6360);
    let f_e = 1.5277 + 0.6023 * ratio.ln();
    let e_e = 1.0003 + 0.5968 / ratio;

    (k_e, f_e, e_e)
}

/// Large-end rib contact (elliptical Hertz point contact + optional EHL/TEHL).
///
/// Computes contact ellipse, max stress, and spin moment for the roller
/// spherical end face pressing against the large-end rib. When `operating` is
/// supplied and the bearing is rotating, also computes the EHL film thickness,
/// thermal-corrected lambda regime, mixed-lubrication asperity sharing, and a
/// dispatched non-Newtonian traction coefficient (Eyring or Carreau-Yasuda).
///
/// q_axial: net axial force on roller = Q × sin(α_o - α_i) / cos(α_i) [N]
pub fn compute_rib_contact(
    roller_profile: &RollerProfile,
    macro_geom: &MacroGeometry,
    raceway_geom: &RacewayGeometry,
    material: &Material,
    q_axial: f64,
    operating: Option<&OperatingConditions>,
) -> Result<RibContactResult, SolverError> {
    // No axial force → zero result
    if q_axial <= 0.0 {
        return Ok(RibContactResult {
            f_rib: 0.0,
            a_ellipse: 0.0,
            b_ellipse: 0.0,
            p_max_rib: 0.0,
            spin_moment: 0.0,
            delta_rib: 0.0,
            k_rib: 0.0,
            r_contact_mm: 0.0,
            r_rib_circ_mm: 0.0,
            h_c_mm: 0.0,
            ehl: None,
        });
    }

    let f_rib = q_axial;

    // Combined elastic modulus [GPa] → [MPa]
    let e_star_gpa =
        hertz::combined_elastic_modulus(material.e_roller, material.nu, material.e_ring, material.nu);
    let e_star_mpa = e_star_gpa * 1000.0;

    // Contact point radial position from bearing axis [mm]
    // r_contact = r_base + h_c (user input or default h_rib/2)
    let (r_contact, r_base, h_c) = compute_rib_contact_radius(
        macro_geom.d_pw,
        macro_geom.l_we,
        macro_geom.d_we_max,
        raceway_geom.alpha_i,
        raceway_geom.alpha_o,
        macro_geom.h_c,
        macro_geom.h_rib,
    );

    // Range check: r_contact must lie between rib base and rib tip
    let r_rib_tip = r_base + macro_geom.h_rib;
    if r_contact < r_base || r_contact > r_rib_tip {
        return Err(SolverError::InvalidGeometry(format!(
            "Rib contact point r_contact={r_contact:.2} mm is outside valid range \
             [{r_base:.2}, {r_rib_tip:.2}] mm (rib base to rib tip)"
        )));
    }

    // Circumferential curvature: auto-calculate or use user-specified override.
    let r_rib_circ = match raceway_geom.r_rib_circ {
        Some(rc) if rc > 0.0 => rc,
        _ => compute_rib_circ_auto(r_contact, macro_geom.alpha_rib),
    };
    let (r_x, r_y) = rib_equivalent_radii(roller_profile.r_sph, raceway_geom.r_rib, r_rib_circ)?;

    // Hamrock-Brewe coefficients
    let (k_e, f_e, e_e) = hertz_elliptical_coefficients(r_x, r_y);

    // Sum of curvatures
    let sum_rho = 1.0 / r_x + 1.0 / r_y;

    // Contact ellipse semi-axis a [mm] (Hamrock-Brewe, Johnson E* convention)
    // a = (3 * k_e^2 * E_e * F_rib / (π * E* * Σρ))^(1/3)
    // Note: coefficient is 3 (not 6) because E* = 1/[(1-ν²)/E₁ + (1-ν²)/E₂]
    // The factor 6 applies only to Harris E' = 2E*.
    let a = (3.0 * k_e * k_e * e_e * f_rib / (PI * e_star_mpa * sum_rho)).powf(1.0 / 3.0);

    // Semi-axis b [mm]
    let b = a / k_e;

    // Maximum contact stress [MPa]
    let p_max = 3.0 * f_rib / (2.0 * PI * a * b);

    // EHL/TEHL evaluation when operating conditions and rotation are available.
    let ehl = operating.and_then(|op| {
        compute_rib_ehl(
            f_rib, a, b, p_max, r_x, r_y, k_e,
            macro_geom, raceway_geom, material, op, roller_profile.sigma_roller,
        )
    });

    // Spin moment [N·mm] — use EHL effective μ if available; fallback to dry value.
    let mu = ehl.as_ref().map_or(0.002, |e| e.mu_eff);
    let spin_moment = (3.0 / 8.0) * mu * f_rib * a;

    // Hertz approach δ_rib [μm] = a² × Σρ / (2 × k_e × F_e) × 1000
    // a is in [mm], sum_rho in [1/mm] → a²×Σρ is dimensionless → multiply by 1000 for μm
    let delta_rib = a * a * sum_rho / (2.0 * k_e * f_e) * 1000.0;

    // Tangent stiffness K_rib = dF/dδ = (3/2) × F / δ [N/μm]
    let k_rib = if delta_rib > 1e-12 {
        1.5 * f_rib / delta_rib
    } else {
        0.0
    };

    Ok(RibContactResult {
        f_rib,
        a_ellipse: a,
        b_ellipse: b,
        p_max_rib: p_max,
        spin_moment,
        delta_rib,
        r_contact_mm: r_contact,
        r_rib_circ_mm: r_rib_circ,
        k_rib,
        h_c_mm: h_c,
        ehl,
    })
}

/// Rib EHL/TEHL evaluation (research report §4.3 reference path).
///
/// Hamrock-Dowson elliptical regression + Murch-Wilson φ_T + Greenwood-Tripp
/// (Clarke variant) asperity sharing + non-Newtonian traction (Eyring or
/// Carreau-Yasuda via `traction_coefficient`) + Blok-Jaeger flash temperature.
///
/// Inputs are dimensional (mm for ellipse semi-axes, MPa for p_max).
/// Returns None when speed is zero or essential geometry/materials are degenerate.
fn compute_rib_ehl(
    f_rib_n: f64,
    a_ellipse_mm: f64,
    b_ellipse_mm: f64,
    p_max_mpa: f64,
    r_x_mm: f64,
    r_y_mm: f64,
    k_e: f64,
    macro_geom: &MacroGeometry,
    raceway_geom: &RacewayGeometry,
    material: &Material,
    operating: &OperatingConditions,
    sigma_roller_um: f64,
) -> Option<RibEhlResult> {
    if operating.n_rpm() < 1e-6 || f_rib_n < 1e-6 {
        return None;
    }

    // Speeds (pure-sliding limit at the rib face)
    let (u_entrain, u_slide, srr) = compute_rib_speeds(macro_geom, raceway_geom, operating);
    if u_entrain < 1e-10 {
        return None;
    }

    // Dynamic viscosity at operating temperature [Pa·s]
    let nu_op = viscosity_at_temp_pub(operating.nu_40, operating.nu_100, operating.t_op);
    let eta_0 = nu_op * 1e-6 * operating.rho_oil;
    let alpha_pv = operating.alpha_pv * 1e-9; // 1/GPa → 1/Pa

    // Effective elastic modulus (Hamrock-Dowson E') — twice Johnson's E*
    let e_star_gpa = hertz::combined_elastic_modulus(
        material.e_roller, material.nu, material.e_ring, material.nu,
    );
    let e_star_pa = e_star_gpa * 1e9;
    let e_prime_pa = 2.0 * e_star_pa; // Hamrock-Dowson convention

    // Convert geometry to SI [m]
    let r_x_m = r_x_mm * 1e-3;
    let _r_y_m = r_y_mm * 1e-3;

    // Dimensionless groups (Hamrock-Dowson ball-bearing convention)
    let u_param = eta_0 * u_entrain / (e_prime_pa * r_x_m);
    let g_param = alpha_pv * e_prime_pa;
    let w_param = f_rib_n / (e_prime_pa * r_x_m * r_x_m);

    // Hamrock-Dowson elliptical (k must be ≥ 1 — clamp inside helper)
    let k_clamped = k_e.max(1.0);
    let (h_c_dim, h_min_dim) = hamrock_dowson_elliptical(u_param, g_param, w_param, k_clamped);
    let h_c_iso_m = h_c_dim * r_x_m;
    let h_min_iso_m = h_min_dim * r_x_m;

    // Murch-Wilson thermal correction
    let p_mean_pa = (p_max_mpa * 1e6 * 0.8).max(1.0); // p̄ ≈ (2/3)p_max for spherical pressure dist.
    let phi_t = thermal_correction_murch_wilson(
        eta_0, operating.beta_visc, u_entrain, operating.k_fluid,
        srr, p_mean_pa, e_prime_pa,
    );
    let phi_s = operating.starvation_factor.clamp(0.1, 1.0);
    let h_c_m = h_c_iso_m * phi_t * phi_s;
    let h_min_m = h_min_iso_m * phi_t * phi_s;
    let h_c_um = h_c_m * 1e6;
    let h_min_um = h_min_m * 1e6;

    // Composite roughness — rib face uses inner-raceway Rq as a proxy
    // (rib is part of the inner ring; same finishing process)
    let sigma_rib_um = operating.rq_inner_eff();
    let sigma_c_um = (sigma_roller_um.powi(2) + sigma_rib_um.powi(2)).sqrt().max(1e-3);
    let lambda = h_min_um / sigma_c_um;
    let regime = classify_lambda(lambda);

    // Fluid-EHL traction (dispatcher: Eyring or Carreau-Yasuda)
    let mu_ehl = if h_c_m > 1e-10 {
        traction_coefficient(operating, srr, u_entrain, p_mean_pa, h_c_m, eta_0)
    } else {
        RIB_MU_BOUNDARY
    };

    // Asperity sharing — Greenwood-Tripp 1970 F_{5/2} statistical integral
    // (unified across raceway and rib contacts; supersedes prior Clarke 1-erf
    //  closed-form, which is retained as `clarke_load_sharing` for back-compat).
    let f_a = greenwood_tripp_load_sharing(lambda).clamp(0.0, 1.0);
    let mu_eff = (1.0 - f_a) * mu_ehl + f_a * RIB_MU_BOUNDARY;
    let p_asperity_mpa = f_a * p_mean_pa * 1e-6;

    // Blok-Jaeger flash temperature — use ellipse semi-minor as contact scale
    let b_min_m = (a_ellipse_mm.min(b_ellipse_mm)) * 1e-3;
    let p_asp_pa = (f_a * p_mean_pa).max(0.0);
    let flash_dt = if p_asp_pa > 1.0 && u_slide > 1e-6 {
        flash_temperature(mu_eff, p_asp_pa, u_slide, b_min_m)
    } else {
        0.0
    };

    Some(RibEhlResult {
        h_c_um, h_min_um,
        sigma_composite_um: sigma_c_um,
        lambda_ratio: lambda,
        regime,
        mu_eff, mu_ehl,
        asperity_load_ratio: f_a,
        p_asperity_mpa,
        flash_temp_c: flash_dt,
        srr,
        u_entrain_m_s: u_entrain,
        u_slide_m_s: u_slide,
        thermal_factor: phi_t,
        u_param, g_param, w_param,
        k_ellipse: k_clamped,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_material() -> Material {
        Material::default() // E=210 GPa, nu=0.3
    }

    fn test_roller_profile(r_sph: f64) -> RollerProfile {
        RollerProfile {
            crown_type: CrownType::Logarithmic { a_log: 0.0002 },
            delta_c: 5.0,
            delta_dub_l: 3.0,
            delta_dub_s: 3.0,
            l_dub_l: 2.0,
            l_dub_s: 2.0,
            r_sph,
            sigma_roller: 0.15,
        }
    }

    fn test_raceway_geom(r_rib: f64) -> RacewayGeometry {
        RacewayGeometry {
            alpha_i: 12.0,
            alpha_o: 12.0,
            r_i: 200.0,
            r_o: 200.0,
            r_rib,
            r_rib_circ: None, // rib face is flat in circumferential direction
            d_uc: 0.0,
            l_uc: 0.0,
        }
    }

    fn test_macro_geom() -> MacroGeometry {
        MacroGeometry {
            d: 50.0,
            outer_diameter: 90.0,
            t: 20.0,
            alpha: 12.0,
            z: 20,
            d_we_max: 10.0,
            d_we_min: 8.5,
            l_we: 15.0,
            d_pw: 70.0,
            h_rib: 3.0,
            alpha_rib: 10.0,
            g_r: 0.0,
            h_c: None,
        }
    }

    #[test]
    fn test_rib_contact_basic() {
        let profile = test_roller_profile(50.0);
        let raceway = test_raceway_geom(1500.0);
        let macro_g = test_macro_geom();
        let mat = test_material();

        let result = compute_rib_contact(&profile, &macro_g, &raceway, &mat, 500.0, None);
        assert!(result.is_ok());
        let r = result.unwrap();

        assert!(r.a_ellipse > 0.0, "a should be positive: {}", r.a_ellipse);
        assert!(r.b_ellipse > 0.0, "b should be positive: {}", r.b_ellipse);
        assert!(r.p_max_rib > 0.0, "p_max should be positive: {}", r.p_max_rib);
        assert!(r.spin_moment > 0.0, "spin moment should be positive");
        // Reasonable range check: p_max for 500N on small ellipse → hundreds to thousands of MPa
        assert!(
            r.p_max_rib > 100.0 && r.p_max_rib < 10000.0,
            "p_max_rib={:.1} MPa out of expected range",
            r.p_max_rib
        );
    }

    #[test]
    fn test_rib_contact_zero_load() {
        let profile = test_roller_profile(50.0);
        let raceway = test_raceway_geom(1500.0);
        let macro_g = test_macro_geom();
        let mat = test_material();

        let r = compute_rib_contact(&profile, &macro_g, &raceway, &mat, 0.0, None).unwrap();
        assert_eq!(r.f_rib, 0.0);
        assert_eq!(r.a_ellipse, 0.0);
        assert_eq!(r.b_ellipse, 0.0);
        assert_eq!(r.p_max_rib, 0.0);
        assert_eq!(r.spin_moment, 0.0);
    }

    #[test]
    fn test_rib_contact_load_scaling() {
        let profile = test_roller_profile(50.0);
        let raceway = test_raceway_geom(1500.0);
        let macro_g = test_macro_geom();
        let mat = test_material();

        let r1 = compute_rib_contact(&profile, &macro_g, &raceway, &mat, 200.0, None).unwrap();
        let r2 = compute_rib_contact(&profile, &macro_g, &raceway, &mat, 800.0, None).unwrap();

        // Hertz: p_max ~ F^(1/3) for point contact, so higher load → higher stress
        assert!(
            r2.p_max_rib > r1.p_max_rib,
            "Higher load should give higher p_max: {:.1} vs {:.1}",
            r2.p_max_rib,
            r1.p_max_rib
        );
        assert!(r2.a_ellipse > r1.a_ellipse, "Higher load → larger contact ellipse");
    }

    #[test]
    fn test_rib_contact_invalid_geometry() {
        // r_sph >= r_rib → should fail
        let profile = test_roller_profile(2000.0); // r_sph = 2000
        let raceway = test_raceway_geom(1500.0); // r_rib = 1500 < r_sph
        let macro_g = test_macro_geom();
        let mat = test_material();

        let result = compute_rib_contact(&profile, &macro_g, &raceway, &mat, 500.0, None);
        assert!(result.is_err(), "Should fail when r_sph >= r_rib");
    }

    #[test]
    fn test_rib_circ_auto_vs_manual() {
        // Auto-calculated circumferential curvature vs manual override
        let profile = test_roller_profile(50.0);
        let macro_g = test_macro_geom();
        let mat = test_material();

        // Case 1: Auto (r_rib_circ = None → auto from d_pw/2 / sin(alpha_rib))
        let raceway_auto = test_raceway_geom(1500.0);
        let r_auto = compute_rib_contact(&profile, &macro_g, &raceway_auto, &mat, 500.0, None).unwrap();

        // Case 2: Manual with smaller circumferential radius (more conforming) → lower p_max
        let mut raceway_manual = test_raceway_geom(1500.0);
        raceway_manual.r_rib_circ = Some(60.0); // concave, closer to r_sph=50 → more conforming
        let r_manual = compute_rib_contact(&profile, &macro_g, &raceway_manual, &mat, 500.0, None).unwrap();

        // Auto should give finite r_rib_circ → valid result
        assert!(r_auto.p_max_rib > 0.0, "Auto rib contact should produce valid result");

        // Manual 60mm is more conforming than auto (~201mm), so R_y is larger → lower p_max
        assert!(
            r_manual.p_max_rib < r_auto.p_max_rib,
            "Manual 60mm circ (more conforming) should give lower p_max: {:.1} vs {:.1}",
            r_manual.p_max_rib, r_auto.p_max_rib
        );
    }

    #[test]
    fn test_compute_rib_contact_radius() {
        let d_pw = 70.0;
        let l_we = 15.0;
        let d_we_max = 10.0;
        let alpha_i = 12.0;
        let alpha_o = 12.0;
        let h_rib = 3.0;

        // h_c = None → default h_rib/2 = 1.5
        let (r_contact, r_base, h_c) = compute_rib_contact_radius(
            d_pw, l_we, d_we_max, alpha_i, alpha_o, None, h_rib,
        );
        let gamma = 12.0_f64.to_radians();
        let expected_base = d_pw / 2.0 + (l_we / 2.0) * gamma.sin() - (d_we_max / 2.0) * gamma.cos();
        assert!((r_base - expected_base).abs() < 1e-10, "r_base: {r_base:.3} vs {expected_base:.3}");
        assert!((h_c - 1.5).abs() < 1e-10, "h_c should be h_rib/2=1.5: h_c={h_c:.6}");
        assert!((r_contact - (r_base + 1.5)).abs() < 1e-10);

        // h_c = Some(1.0) → explicit value
        let (r_c2, _, h_c2) = compute_rib_contact_radius(
            d_pw, l_we, d_we_max, alpha_i, alpha_o, Some(1.0), h_rib,
        );
        assert!((h_c2 - 1.0).abs() < 1e-10);
        assert!((r_c2 - (r_base + 1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_resolve_contact_height_auto() {
        // None → h_rib / 2
        assert!((resolve_contact_height(None, 6.0) - 3.0).abs() < 1e-10);
        assert!((resolve_contact_height(None, 3.0) - 1.5).abs() < 1e-10);
    }

    #[test]
    fn test_resolve_contact_height_explicit() {
        // Some(v) → clamped to [0, h_rib]
        assert!((resolve_contact_height(Some(2.0), 6.0) - 2.0).abs() < 1e-10);
        assert!((resolve_contact_height(Some(-1.0), 6.0) - 0.0).abs() < 1e-10);
        assert!((resolve_contact_height(Some(10.0), 6.0) - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_compute_rib_circ_auto() {
        // R_rib_circ = r_contact / sin(α_rib)
        // α_rib is from radial → normal-to-axis angle ψ = α_rib
        let r_contact = 21.0;
        let alpha_rib = 10.0;
        let r = compute_rib_circ_auto(r_contact, alpha_rib);
        let expected = 21.0 / 10.0_f64.to_radians().sin();
        assert!((r - expected).abs() < 1e-6, "Auto calc: {r:.3} vs expected {expected:.3}");
        // For small α_rib (~10°), R_circ >> r_contact
        assert!(r > r_contact * 5.0, "R_rib_circ should be >> r_contact for small α_rib: {r:.1}");
    }

    #[test]
    fn test_rib_equivalent_radii_with_circ() {
        // Both directions: convex-concave (conforming) → subtraction
        // R_x = 1/(1/r_sph - 1/r_rib), R_y = 1/(1/r_sph - 1/r_rib_circ)
        let (r_x, r_y) = rib_equivalent_radii(50.0, 1500.0, 120.0).unwrap();
        let expected_r_x = 1.0 / (1.0 / 50.0 - 1.0 / 1500.0);
        let expected_r_y = 1.0 / (1.0 / 50.0 - 1.0 / 120.0);
        assert!((r_x - expected_r_x).abs() < 1e-10);
        assert!((r_y - expected_r_y).abs() < 1e-10);
        assert!(r_y > 50.0, "R_y with concave circ should be > R_sph (conforming)");

        // With very large circ → R_y ≈ R_sph (flat limit)
        let (_, r_y_flat) = rib_equivalent_radii(50.0, 1500.0, 1e12).unwrap();
        assert!((r_y_flat - 50.0).abs() < 0.1, "Large circ should give R_y ≈ R_sph");

        // More conforming (r_rib_circ closer to r_sph) → larger R_y
        let (_, r_y_conform) = rib_equivalent_radii(50.0, 1500.0, 60.0).unwrap();
        assert!(r_y_conform > r_y, "More conforming → larger R_y: {r_y_conform:.1} vs {r_y:.1}");
    }

    // ─── Rib EHL/TEHL Tests ─────────────────────────────────────────

    /// Asymmetric-cone fixture (α_i ≠ α_o) so `compute_trb_kinematics` can
    /// resolve a non-zero roller spin (the dry-fixture α_i=α_o=12° collapses
    /// the cone apex and yields ω_roller = 0 by construction).
    fn ehl_raceway_geom() -> RacewayGeometry {
        RacewayGeometry {
            alpha_i: 8.0,
            alpha_o: 12.0,
            r_i: 200.0,
            r_o: 200.0,
            r_rib: 1500.0,
            r_rib_circ: None,
            d_uc: 0.0,
            l_uc: 0.0,
        }
    }

    fn ehl_macro_geom() -> MacroGeometry {
        MacroGeometry {
            d: 50.0, outer_diameter: 90.0, t: 20.0,
            alpha: 10.0, // mean of α_i and α_o
            z: 20,
            d_we_max: 10.0, d_we_min: 8.5, l_we: 15.0, d_pw: 70.0,
            h_rib: 3.0, alpha_rib: 10.0, g_r: 0.0, h_c: None,
        }
    }

    /// Build a representative `OperatingConditions` (NSK HR30306J-like) for EHL tests.
    fn test_op_rib(rpm: f64) -> OperatingConditions {
        OperatingConditions {
            f_x: 10.0, f_y: 0.0, f_a: 5.0, m_x: 0.0, m_y: 0.0,
            n_inner_rpm: rpm, n_outer_rpm: 0.0,
            gamma: 0.0, t_op: 70.0, nu_40: 68.0, nu_100: 8.0,
            alpha_pv: 20.0, lubrication_type: LubricationType::Oil,
            starvation_factor: 1.0, rho_oil: 870.0,
            preload_mode: PreloadMode::DisplacementFromForce,
            delta_preload_um: 0.0, design_life_hours: 100.0,
            lubrication_model: LubricationModel::Method1_DH,
            film_decay_enabled: false, film_decay_time_hours: 0.0,
            skew_angle_deg: 0.0, replenishment_rate_nm_s: 0.0,
            surface_finish: SurfaceFinish::Standard, additive_type: AdditiveType::None,
            tau_eyring: 5.0, z_roelands: 0.67,
            traction_model: TractionModel::Eyring,
            carreau_eta_inf_ratio: 0.005, carreau_lambda_s: 1.0e-7,
            carreau_n: 0.5, carreau_a: 2.0,
            friction_model: FrictionModel::PalmgrenLike,
            thermal_correction: ThermalCorrection::Aihara1987,
            hysteresis_loss_factor: 0.005,
            skf_trb_series: SkfTrbSeriesEnum::Series303,
            skf_lubrication: SkfLubricationEnum::OilBath, skf_y_factor: 1.6,
            k_fluid: 0.15, beta_visc: 0.04,
            rq_inner: 0.3, rq_outer: 0.3, rq_roller: 0.15,
            roughness_input_mode: RoughnessInputMode::Rq,
        }
    }

    /// Backward-compat: `compute_rib_contact(.., None)` produces identical Hertz
    /// results (a, b, p_max, δ) compared with the previous (pre-EHL) signature
    /// — and rib.ehl is None.
    #[test]
    fn test_rib_ehl_none_when_operating_omitted() {
        let profile = test_roller_profile(50.0);
        let raceway = test_raceway_geom(1500.0);
        let macro_g = test_macro_geom();
        let mat = test_material();

        let r = compute_rib_contact(&profile, &macro_g, &raceway, &mat, 500.0, None).unwrap();
        assert!(r.ehl.is_none(), "EHL must be None when operating is None");
        // Spin moment should fall back to dry μ = 0.002
        let mu_implied = r.spin_moment / ((3.0 / 8.0) * r.f_rib * r.a_ellipse);
        assert!((mu_implied - 0.002).abs() < 1e-9,
            "Dry path μ should be 0.002, got {mu_implied}");
    }

    /// EHL is None when bearing is static (n_rpm = 0).
    #[test]
    fn test_rib_ehl_none_when_static() {
        let profile = test_roller_profile(50.0);
        let raceway = ehl_raceway_geom();
        let macro_g = ehl_macro_geom();
        let mat = test_material();
        let op = test_op_rib(0.0);

        let r = compute_rib_contact(&profile, &macro_g, &raceway, &mat, 500.0, Some(&op)).unwrap();
        assert!(r.ehl.is_none(), "EHL must be None at zero speed");
    }

    /// EHL is None when no axial load (q_axial = 0 → early return).
    #[test]
    fn test_rib_ehl_none_when_no_axial_load() {
        let profile = test_roller_profile(50.0);
        let raceway = ehl_raceway_geom();
        let macro_g = ehl_macro_geom();
        let mat = test_material();
        let op = test_op_rib(1000.0);

        let r = compute_rib_contact(&profile, &macro_g, &raceway, &mat, 0.0, Some(&op)).unwrap();
        assert!(r.ehl.is_none(), "EHL must be None at zero axial load");
        assert_eq!(r.f_rib, 0.0);
    }

    /// Loaded + rotating bearing produces a populated EHL block with positive
    /// film thickness, finite Λ, valid regime, and bounded effective μ.
    #[test]
    fn test_rib_ehl_loaded_rotating() {
        let profile = test_roller_profile(50.0);
        let raceway = ehl_raceway_geom();
        let macro_g = ehl_macro_geom();
        let mat = test_material();
        let op = test_op_rib(2000.0);

        let r = compute_rib_contact(&profile, &macro_g, &raceway, &mat, 500.0, Some(&op)).unwrap();
        let ehl = r.ehl.expect("EHL must be Some for loaded + rotating");

        assert!(ehl.h_c_um > 0.0, "h_c must be > 0: {}", ehl.h_c_um);
        assert!(ehl.h_min_um > 0.0, "h_min must be > 0: {}", ehl.h_min_um);
        assert!(ehl.h_min_um <= ehl.h_c_um, "h_min ≤ h_c");
        assert!(ehl.lambda_ratio > 0.0, "Λ must be > 0");
        assert!(ehl.mu_eff > 0.0 && ehl.mu_eff < 0.20,
            "μ_eff out of [0, 0.2]: {}", ehl.mu_eff);
        // SRR is the pure-sliding limit ≈ 2 (rib face stationary in inner-rotating frame)
        assert!((ehl.srr - 2.0).abs() < 0.01,
            "SRR should be ~2 at rib (pure sliding), got {}", ehl.srr);
        assert!(ehl.thermal_factor > 0.0 && ehl.thermal_factor <= 1.0,
            "φ_T must be in (0, 1]: {}", ehl.thermal_factor);
        assert!(ehl.u_param > 0.0 && ehl.g_param > 0.0 && ehl.w_param > 0.0,
            "Dimensionless groups must be positive");
        assert!(ehl.k_ellipse >= 1.0, "k must be ≥ 1");
    }

    /// EHL μ_eff replaces the 0.002 dry constant in spin moment, so spin
    /// moment under EHL is meaningfully different (typically larger) than in
    /// the dry path.
    #[test]
    fn test_rib_ehl_changes_spin_moment() {
        let profile = test_roller_profile(50.0);
        let raceway = ehl_raceway_geom();
        let macro_g = ehl_macro_geom();
        let mat = test_material();
        let op = test_op_rib(2000.0);

        let dry = compute_rib_contact(&profile, &macro_g, &raceway, &mat, 500.0, None).unwrap();
        let wet = compute_rib_contact(&profile, &macro_g, &raceway, &mat, 500.0, Some(&op)).unwrap();

        // Same Hertz contact (only μ differs)
        assert!((dry.a_ellipse - wet.a_ellipse).abs() < 1e-9);
        assert!((dry.b_ellipse - wet.b_ellipse).abs() < 1e-9);
        assert!((dry.p_max_rib - wet.p_max_rib).abs() < 1e-9);
        // Spin moments differ — wet path uses EHL μ_eff
        assert!((dry.spin_moment - wet.spin_moment).abs() > 1e-6,
            "Spin moment should change with EHL: dry={:.4} vs wet={:.4}",
            dry.spin_moment, wet.spin_moment);
    }

    /// Higher rotational speed → larger U → larger HD film thickness h_c.
    /// Murch-Wilson φ_T also reduces h_c at very high speed; this test stays
    /// in a regime where the U^0.67 dependence dominates.
    #[test]
    fn test_rib_ehl_speed_increases_film_thickness() {
        let profile = test_roller_profile(50.0);
        let raceway = ehl_raceway_geom();
        let macro_g = ehl_macro_geom();
        let mat = test_material();

        let r_low = compute_rib_contact(
            &profile, &macro_g, &raceway, &mat, 500.0,
            Some(&test_op_rib(500.0)),
        ).unwrap();
        let r_high = compute_rib_contact(
            &profile, &macro_g, &raceway, &mat, 500.0,
            Some(&test_op_rib(3000.0)),
        ).unwrap();

        let h_low = r_low.ehl.as_ref().unwrap().h_c_um;
        let h_high = r_high.ehl.as_ref().unwrap().h_c_um;
        assert!(h_high > h_low,
            "h_c should grow with speed in mild regime: {h_low:.3} → {h_high:.3} μm");
    }

    /// Eyring vs Carreau-Yasuda traction at the rib produces *different* μ_eff.
    /// Spin moment changes accordingly. p_max and h_min are unchanged (only
    /// the traction model differs).
    #[test]
    fn test_rib_ehl_eyring_vs_carreau() {
        let profile = test_roller_profile(50.0);
        let raceway = ehl_raceway_geom();
        let macro_g = ehl_macro_geom();
        let mat = test_material();

        let mut op_e = test_op_rib(2000.0);
        op_e.traction_model = TractionModel::Eyring;
        let mut op_c = test_op_rib(2000.0);
        op_c.traction_model = TractionModel::CarreauYasuda;

        let r_e = compute_rib_contact(&profile, &macro_g, &raceway, &mat, 500.0, Some(&op_e)).unwrap();
        let r_c = compute_rib_contact(&profile, &macro_g, &raceway, &mat, 500.0, Some(&op_c)).unwrap();

        let ehl_e = r_e.ehl.as_ref().unwrap();
        let ehl_c = r_c.ehl.as_ref().unwrap();

        // Hertz quantities and film identical (depend only on speed/load/geometry)
        assert!((r_e.p_max_rib - r_c.p_max_rib).abs() < 1e-9);
        assert!((ehl_e.h_min_um - ehl_c.h_min_um).abs() < 1e-9);
        // Traction differs measurably
        let mu_diff = (ehl_e.mu_eff - ehl_c.mu_eff).abs();
        assert!(mu_diff > 1e-6,
            "Eyring vs Carreau μ should differ: {} vs {}", ehl_e.mu_eff, ehl_c.mu_eff);
        // Both are in physically reasonable range
        for mu in [ehl_e.mu_eff, ehl_c.mu_eff] {
            assert!(mu > 0.0 && mu < 0.15, "μ out of range: {mu}");
        }
    }

    /// Higher rib roughness (Rq_inner_eff) → smaller Λ → mixed/boundary regime
    /// and larger asperity load fraction.
    #[test]
    fn test_rib_ehl_rough_surface_increases_asperity() {
        let profile = test_roller_profile(50.0);
        let raceway = ehl_raceway_geom();
        let macro_g = ehl_macro_geom();
        let mat = test_material();

        let mut op_smooth = test_op_rib(2000.0);
        op_smooth.rq_inner = 0.05; // very smooth
        op_smooth.rq_roller = 0.05;
        let mut op_rough = test_op_rib(2000.0);
        op_rough.rq_inner = 0.6; // 12× rougher
        op_rough.rq_roller = 0.6;

        let r_s = compute_rib_contact(&profile, &macro_g, &raceway, &mat, 500.0, Some(&op_smooth)).unwrap();
        let r_r = compute_rib_contact(&profile, &macro_g, &raceway, &mat, 500.0, Some(&op_rough)).unwrap();

        let ehl_s = r_s.ehl.as_ref().unwrap();
        let ehl_r = r_r.ehl.as_ref().unwrap();

        assert!(ehl_s.lambda_ratio > ehl_r.lambda_ratio,
            "Smooth Λ ({}) must exceed rough Λ ({})", ehl_s.lambda_ratio, ehl_r.lambda_ratio);
        assert!(ehl_r.asperity_load_ratio >= ehl_s.asperity_load_ratio,
            "Rough surface must carry at least as much asperity load");
    }

    /// Higher load → higher p_max but typically smaller h (W^(−0.073) is mild).
    /// Verify h_min stays positive across a load range.
    #[test]
    fn test_rib_ehl_load_sweep_consistency() {
        let profile = test_roller_profile(50.0);
        let raceway = ehl_raceway_geom();
        let macro_g = ehl_macro_geom();
        let mat = test_material();
        let op = test_op_rib(2000.0);

        let mut prev_p = 0.0;
        for &q in &[100.0, 300.0, 600.0, 1000.0_f64] {
            let r = compute_rib_contact(&profile, &macro_g, &raceway, &mat, q, Some(&op)).unwrap();
            let ehl = r.ehl.as_ref().expect("EHL must be Some");
            assert!(ehl.h_min_um > 0.0, "h_min positive @ q={q}: {}", ehl.h_min_um);
            assert!(r.p_max_rib > prev_p, "p_max monotonic in load");
            prev_p = r.p_max_rib;
        }
    }

    /// Hamrock-Dowson elliptical helper: verify k=1 (circular) and k→∞ trends
    /// from the dispatcher (re-imported via super module).
    #[test]
    fn test_hamrock_dowson_elliptical_trends() {
        use crate::solver::lubrication::hamrock_dowson_elliptical;
        let u = 1e-11; let g = 5000.0; let w = 1e-4;

        let (h_c_circ, h_min_circ) = hamrock_dowson_elliptical(u, g, w, 1.0);
        let (h_c_long, h_min_long) = hamrock_dowson_elliptical(u, g, w, 20.0);

        // Larger ellipticity (k>>1) → larger H factors (1 − exp(−0.68k)) → 1
        assert!(h_c_long > h_c_circ, "H_c grows with k: {h_c_circ} → {h_c_long}");
        assert!(h_min_long > h_min_circ, "H_min grows with k: {h_min_circ} → {h_min_long}");

        // Ratio stays in expected range (H_min/H_c ≈ 0.5 ~ 0.8 for typical EHL)
        for (hc, hm, label) in [(h_c_circ, h_min_circ, "circular"), (h_c_long, h_min_long, "slender")] {
            let ratio = hm / hc;
            assert!(ratio > 0.3 && ratio < 1.0, "{label}: H_min/H_c={ratio:.3} out of expected range");
        }
    }

    /// Static / non-loaded edge case: zero u, zero w should return zero film.
    #[test]
    fn test_hamrock_dowson_elliptical_zero_inputs() {
        use crate::solver::lubrication::hamrock_dowson_elliptical;
        assert_eq!(hamrock_dowson_elliptical(0.0, 5000.0, 1e-4, 2.0), (0.0, 0.0));
        assert_eq!(hamrock_dowson_elliptical(1e-11, 0.0, 1e-4, 2.0), (0.0, 0.0));
        assert_eq!(hamrock_dowson_elliptical(1e-11, 5000.0, 0.0, 2.0), (0.0, 0.0));
    }
}
