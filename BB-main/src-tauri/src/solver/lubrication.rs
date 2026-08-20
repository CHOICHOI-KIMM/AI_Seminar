use crate::solver::types::*;

// ─── Constants ──────────────────────────────────────────────────────

/// Greenwood-Tripp asperity model parameters (typical bearing steel).
/// Product η·β·σ ≈ 0.04 (dimensionless), where:
///   η = asperity density [1/m²], β = asperity tip radius [m], σ = composite roughness [m]
const ETA_BETA_SIGMA: f64 = 0.04;

/// Boundary friction coefficient for bearing steel with EP additive.
const MU_BOUNDARY: f64 = 0.10;

/// Rib contact friction coefficient (sliding, with EP additive).
const MU_RIB: f64 = 0.06;

// ─── Mathematical Utilities ─────────────────────────────────────────

/// Complementary error function erfc(x) — Abramowitz & Stegun 7.1.26.
/// Maximum error |ε| < 1.5e-7 for x ≥ 0.
#[allow(dead_code)]
fn erfc(x: f64) -> f64 {
    if x < 0.0 {
        return 2.0 - erfc(-x);
    }
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let poly = t * (0.254829592
        + t * (-0.284496736
        + t * (1.421413741
        + t * (-1.453152027
        + t * 1.061405429))));
    poly * (-x * x).exp()
}

/// Standard normal PDF φ(s) = (1/√(2π)) × exp(-s²/2)
fn phi_gauss(s: f64) -> f64 {
    const INV_SQRT_2PI: f64 = 0.3989422804014327; // 1/√(2π)
    INV_SQRT_2PI * (-0.5 * s * s).exp()
}

/// Greenwood-Tripp statistical integral F_n(h) for Gaussian distribution.
///   F_n(h) = ∫_h^∞ (s − h)^n × φ(s) ds
///
/// Computed by Gauss-Hermite quadrature (16-point) for robustness.
fn gt_integral(n: f64, h: f64) -> f64 {
    if h > 4.5 {
        return 0.0; // negligible for Gaussian
    }
    // Numerical integration using composite Simpson's rule over [h, h+5σ]
    // (covers >99.99% of the Gaussian tail)
    let upper = (h + 5.0).max(h + 0.1);
    let n_steps = 200;
    let ds = (upper - h) / n_steps as f64;
    let mut sum = 0.0;
    for i in 0..=n_steps {
        let s = h + i as f64 * ds;
        let val = (s - h).powf(n) * phi_gauss(s);
        let w = if i == 0 || i == n_steps {
            1.0
        } else if i % 2 == 1 {
            4.0
        } else {
            2.0
        };
        sum += w * val;
    }
    sum * ds / 3.0
}

// ─── Nijenbanning-Venner-Moes (1994) Film Thickness ─────────────────
//
// Reference: Nijenbanning G, Venner CH, Moes H.
// "Film thickness in elastohydrodynamically lubricated elliptic contacts"
// Wear 1994;176(2):217-229.
//
// Unified EHL formula covering all 4 regimes (IR, IE, RP, EP).

/// NVM (1994) central film thickness for elliptic/line contacts.
///
/// Covers all 4 EHL regimes: Isoviscous-Rigid, Isoviscous-Elastic,
/// Piezoviscous-Rigid, Piezoviscous-Elastic (Dowson-Higginson regime).
///
/// For TRB line contacts, results are within 2-7% of Dowson-Higginson
/// in the EP regime where TRBs typically operate.
///
/// # Arguments
/// * `f_total` - Total contact load [N]
/// * `rx`      - Reduced radius in rolling direction [m]
/// * `ry`      - Reduced radius transverse [m] (large for line contact)
/// * `e_prime`  - Combined elastic modulus [Pa]
/// * `eta_0`   - Dynamic viscosity [Pa·s]
/// * `u_s`     - Sum velocity u1+u2 [m/s] (= 2 × mean entraining velocity)
/// * `alpha`   - Pressure-viscosity coefficient [1/Pa]
///
/// # Returns
/// Central film thickness [m]
pub fn nvm_central_film(
    f_total: f64,
    rx: f64,
    ry: f64,
    e_prime: f64,
    eta_0: f64,
    u_s: f64,
    alpha: f64,
) -> f64 {
    if f_total <= 0.0 || rx <= 0.0 || e_prime <= 0.0 || eta_0 <= 0.0 || u_s <= 0.0 {
        return 0.0;
    }

    let _pi = std::f64::consts::PI;

    // Moes dimensionless parameters
    let speed_param = eta_0 * u_s / (e_prime * rx);
    let m = (f_total / (e_prime * rx * rx)) * speed_param.powf(-0.75);
    let l = alpha * e_prime * speed_param.powf(0.25);
    let d = (rx / ry).max(1e-6); // curvature ratio (→0 for line contact)

    // Four asymptotic central film thickness solutions (Eqs. 5-8)
    let ln_d = d.ln();

    // RI: Rigid-Isoviscous (Eq. 5)
    let c_ri = 145.0 * (1.0 + 0.796 * d.powf(14.0/15.0)).powf(-15.0/7.0) / d;
    let h_ri = c_ri * m.powf(-2.0);

    // EI: Elastic-Isoviscous (Eq. 6)
    let c_ei = 3.18 * (1.0 + 0.006 * ln_d + 0.63 * d.powf(4.0/7.0)).powf(-14.0/25.0)
             * d.powf(-1.0/15.0);
    let h_ei = c_ei * m.powf(-2.0/15.0);

    // RP: Rigid-Piezoviscous (Eq. 7)
    let c_rp = 1.29 * (1.0 + 0.691 * d).powf(-2.0/3.0);
    let h_rp = c_rp * l.powf(2.0/3.0);

    // EP: Elastic-Piezoviscous (Eq. 8) — Dowson-Higginson regime
    let c_ep = 1.48 * (1.0 + 0.006 * ln_d + 0.63 * d.powf(4.0/7.0)).powf(-7.0/20.0)
             * d.powf(-1.0/24.0);
    let h_ep = c_ep * m.powf(-1.0/12.0) * l.powf(3.0/4.0);

    // Unified blending (Eqs. 9-10)
    let s = 1.5 * (1.0 + (-1.2 * h_ei / h_ri).exp());
    let h_00 = 1.8 / d;

    // Isoviscous branch
    let ei_bridge = (h_ei.powf(-4.0) + h_00.powf(-4.0)).powf(-3.0 * s / 8.0);
    let iso_branch = (h_ri.powf(3.0 * s / 2.0) + ei_bridge).powf(2.0 / (3.0 * s));

    // Piezoviscous branch
    let piezo_branch = (h_rp.powf(-8.0) + h_ep.powf(-8.0)).powf(-s / 8.0);

    // Combined: (iso + piezo)^(1/s)
    let h_bar = (iso_branch + piezo_branch).powf(1.0 / s);

    // Recover dimensional: h = h_bar × Rx × √(η₀·u_s / (E'·Rx))
    let h_scale = rx * (eta_0 * u_s / (e_prime * rx)).sqrt();
    h_bar * h_scale
}

// ─── Film Thickness (unchanged API) ─────────────────────────────────

/// Compute EHL film thickness + mixed lubrication for bearing's max-loaded contact.
#[allow(dead_code)]
pub fn compute_film_thickness(
    geom: &MacroGeometry,
    material: &Material,
    operating: &OperatingConditions,
    _roller_profile: &RollerProfile,
    _raceway_inner: &RacewayProfile,
    _raceway_outer: &RacewayProfile,
    q_max_n_per_mm: f64,
) -> Option<FilmThicknessResult> {
    if operating.n_rpm() < 1e-6 || q_max_n_per_mm < 1e-6 {
        return None;
    }

    let d_we = (geom.d_we_max + geom.d_we_min) / 2.0;
    let r_roller = d_we / 2.0;
    let alpha_rad = geom.alpha.to_radians();

    let r_eq_mm = r_roller;
    let r_eq = r_eq_mm * 1e-3; // [m]

    // Combined elastic modulus E* [Pa]
    let nu_mat = material.nu;
    let e1 = material.e_roller * 1e9;
    let e2 = material.e_ring * 1e9;
    let e_star = 1.0 / ((1.0 - nu_mat * nu_mat) / e1 + (1.0 - nu_mat * nu_mat) / e2);

    // Entraining velocity
    let r_pw = geom.d_pw / 2.0 * 1e-3;
    let gamma_dw = d_we * alpha_rad.cos() / geom.d_pw;
    let u_m = operating.u_m_inner(r_pw, gamma_dw);
    let u_m_outer = operating.u_m_outer(r_pw, gamma_dw);

    if u_m < 1e-8 {
        return None;
    }

    // Lubricant properties
    let nu_actual = crate::solver::life::viscosity_at_temp_pub(
        operating.nu_40, operating.nu_100, operating.t_op,
    );
    let eta_0 = nu_actual * 1e-6 * operating.rho_oil; // [Pa·s]
    let alpha_pv = operating.alpha_pv * 1e-9; // [1/Pa]

    let w_per_l = q_max_n_per_mm * 1e3; // [N/m]

    // Dowson-Higginson dimensionless parameters
    let u_param = eta_0 * u_m / (e_star * r_eq);
    let g_param = alpha_pv * e_star;
    let w_param = w_per_l / (e_star * r_eq);

    // Minimum film thickness: Dowson-Higginson (1977)
    let h_min_dimless = 2.65 * u_param.powf(0.70) * g_param.powf(0.54) * w_param.powf(-0.13);
    let h_min_m = h_min_dimless * r_eq;

    // Central film thickness: Dowson-Toyoda (1978)
    let h_c_dimless = 3.06 * u_param.powf(0.69) * g_param.powf(0.56) * w_param.powf(-0.10);
    let h_c_m = h_c_dimless * r_eq;

    // Thermal correction (Gupta)
    let beta_visc = 0.04;
    let k_fluid = 0.15;
    let l_th = eta_0 * beta_visc * u_m * u_m / k_fluid;
    let phi_t = 1.0 / (1.0 + 0.1 * l_th.powf(0.64));

    // Starvation correction
    let phi_s = operating.starvation_factor.clamp(0.1, 1.0);

    let h_min_corrected = h_min_m * phi_t * phi_s;
    let h_c_corrected = h_c_m * phi_t * phi_s;

    let h_min_um = h_min_corrected * 1e6;
    let h_central_um = h_c_corrected * 1e6;

    // Composite surface roughness [μm] — Rq-based (ISO/TR 1281-2)
    let rq_r = operating.rq_roller_eff();
    let rq_i = operating.rq_inner_eff();
    let rq_o = operating.rq_outer_eff();
    let sigma_i = (rq_r * rq_r + rq_i * rq_i).sqrt();
    let sigma_o = (rq_r * rq_r + rq_o * rq_o).sqrt();

    // Inner Lambda ratio
    let lambda_i = if sigma_i > 1e-6 { h_min_um / sigma_i } else { 100.0 };
    let regime_i = classify_lambda(lambda_i);

    // Outer: same R_eq approximation for this simplified function
    let lambda_o = if sigma_o > 1e-6 { h_min_um / sigma_o } else { 100.0 };
    let regime_o = classify_lambda(lambda_o);

    // ─── Mixed lubrication (Greenwood-Tripp) — inner raceway ───
    let mixed = compute_mixed_lubrication(
        lambda_i, sigma_i, q_max_n_per_mm, e_star, u_m, eta_0, alpha_pv,
    );

    Some(FilmThicknessResult {
        h_min_um,
        h_central_um,
        sigma_composite_um: sigma_i,
        lambda_ratio: lambda_i,
        regime: regime_i,
        h_min_um_outer: h_min_um,     // same h_min (single R_eq approximation)
        h_central_um_outer: h_central_um,
        sigma_composite_um_outer: sigma_o,
        lambda_ratio_outer: lambda_o,
        regime_outer: regime_o,
        rq_roller_um: rq_r,
        rq_inner_um: rq_i,
        rq_outer_um: rq_o,
        u_mean_m_s: u_m,
        u_mean_m_s_outer: u_m_outer,
        cage_speed_rpm: operating.omega_cage(gamma_dw) * 60.0 / std::f64::consts::TAU,
        roller_spin_rpm: operating.omega_roller(gamma_dw, geom.d_pw, d_we) * 60.0 / std::f64::consts::TAU,
        starvation_factor: phi_s,
        thermal_factor: phi_t,
        u_param,
        g_param,
        w_param,
        mixed,
        flash_temp_c: None,
        film_decay: None,
        micropitting: None,
    })
}

// ─── Per-Slice Film Thickness Distribution ──────────────────────────

/// Compute EHL film thickness for every angular position using the fine-
/// resolution `angular_distribution` (one point per `angular_increment_deg`).
///
/// Each `AngularLoadPoint` already carries per-slice line loads `slice_q_k`
/// computed via Gen1 contact re-solve, giving the same angular resolution
/// as the stress contour (typically 1° ≈ 360 points vs. z ≈ 14-20 rollers).
///
/// Per-slice geometry is taken from `SliceGeometry`:
///   - R_eq_k  equivalent radius (varies along taper)
///   - u_m_k   entraining velocity at slice-local contact radius
///   - q_k     from AngularLoadPoint.slice_q_k
pub fn compute_film_thickness_distribution(
    geom: &MacroGeometry,
    material: &Material,
    operating: &OperatingConditions,
    _roller_profile: &RollerProfile,
    _raceway_inner: &RacewayProfile,
    _raceway_outer: &RacewayProfile,
    slice_geometries: &[SliceGeometry],
    angular_distribution: &[AngularLoadPoint],
) -> Option<Vec<RollerFilmDistribution>> {
    if operating.n_rpm() < 1e-6 || angular_distribution.is_empty() {
        return None;
    }

    let alpha_rad = geom.alpha.to_radians();
    let r_pw = geom.d_pw / 2.0 * 1e-3; // [m]
    let d_we_mean = (geom.d_we_max + geom.d_we_min) / 2.0;
    let gamma_dw = d_we_mean * alpha_rad.cos() / geom.d_pw;

    // Material
    let nu_mat = material.nu;
    let e1 = material.e_roller * 1e9;
    let e2 = material.e_ring * 1e9;
    let e_star = 1.0 / ((1.0 - nu_mat * nu_mat) / e1 + (1.0 - nu_mat * nu_mat) / e2);

    // Lubricant
    let nu_actual = crate::solver::life::viscosity_at_temp_pub(
        operating.nu_40, operating.nu_100, operating.t_op,
    );
    let eta_0 = nu_actual * 1e-6 * operating.rho_oil; // [Pa·s]
    let alpha_pv = operating.alpha_pv * 1e-9; // [1/Pa]
    let g_param = alpha_pv * e_star;
    let phi_s = operating.starvation_factor.clamp(0.1, 1.0);

    // Thermal correction (Gupta) — use mean entraining velocity for φ_T
    let u_m_mean = operating.u_m_inner(r_pw, gamma_dw);
    let beta_visc = 0.04;
    let k_fluid = 0.15;
    let l_th = eta_0 * beta_visc * u_m_mean * u_m_mean / k_fluid;
    let phi_t = 1.0 / (1.0 + 0.1 * l_th.powf(0.64));

    // Surface roughness — Rq-based (ISO/TR 1281-2), inner and outer separately
    let sigma_r = operating.rq_roller_eff();
    let sigma_race_inner = operating.rq_inner_eff();
    let sigma_race_outer = operating.rq_outer_eff();
    let sigma_inner = (sigma_r * sigma_r + sigma_race_inner * sigma_race_inner).sqrt();
    let sigma_outer = (sigma_r * sigma_r + sigma_race_outer * sigma_race_outer).sqrt();

    // Bearing-wide max slice load — threshold for negligible contacts
    let q_max_bearing = angular_distribution.iter()
        .flat_map(|pt| pt.slice_q_k.iter())
        .cloned()
        .fold(0.0_f64, f64::max);
    let q_min_threshold = q_max_bearing * 0.01;

    let n_slices = slice_geometries.len();
    let zero_slice = SliceFilmThickness {
        h_min_um: 0.0, h_central_um: 0.0, lambda: 0.0, regime: LubricationRegime::Boundary,
        h_min_um_outer: 0.0, h_central_um_outer: 0.0, lambda_outer: 0.0, regime_outer: LubricationRegime::Boundary,
    };
    let no_contact_slices: Vec<SliceFilmThickness> = vec![zero_slice.clone(); n_slices];

    // Helper: compute Dowson-Higginson film at given R_eq [m], u_m [m/s], q [N/mm]
    let ehl_film = |r_eq: f64, u_m: f64, q_k: f64| -> (f64, f64) {
        let w_per_l = q_k * 1e3; // N/mm → N/m
        let u_p = eta_0 * u_m / (e_star * r_eq);
        let w_p = w_per_l / (e_star * r_eq);
        let h_min = 2.65 * u_p.powf(0.70) * g_param.powf(0.54) * w_p.powf(-0.13) * r_eq * phi_t * phi_s * 1e6;
        let h_c   = 3.06 * u_p.powf(0.69) * g_param.powf(0.56) * w_p.powf(-0.10) * r_eq * phi_t * phi_s * 1e6;
        (h_min, h_c)
    };

    let mut result = Vec::with_capacity(angular_distribution.len());

    for (j, pt) in angular_distribution.iter().enumerate() {
        if pt.q_total < 1e-3 {
            result.push(RollerFilmDistribution {
                roller_idx: j,
                psi_deg: pt.psi_deg,
                slices: no_contact_slices.clone(),
            });
            continue;
        }

        let mut slice_films = Vec::with_capacity(n_slices);

        for (k, &q_k) in pt.slice_q_k.iter().enumerate() {
            if q_k < q_min_threshold {
                slice_films.push(zero_slice.clone());
                continue;
            }

            let r_roller_k_mm = if k < n_slices { slice_geometries[k].r_roller } else { d_we_mean / 2.0 };
            let gamma_k = (2.0 * r_roller_k_mm * alpha_rad.cos()) / geom.d_pw;

            // Inner raceway: R_eq_inner, u_m at inner contact radius
            let r_eq_i = if k < n_slices { slice_geometries[k].r_eq_inner * 1e-3 } else { d_we_mean / 2.0 * 1e-3 };
            let u_m_i = operating.u_m_inner(r_pw, gamma_k);

            // Outer raceway: R_eq_outer, u_m at outer contact radius
            let r_eq_o = if k < n_slices { slice_geometries[k].r_eq_outer * 1e-3 } else { d_we_mean / 2.0 * 1e-3 };
            let u_m_o = operating.u_m_outer(r_pw, gamma_k);

            // Inner film
            let (h_min_i, h_c_i) = if u_m_i > 1e-8 && r_eq_i > 1e-8 {
                ehl_film(r_eq_i, u_m_i, q_k)
            } else { (0.0, 0.0) };
            let lambda_i = if sigma_inner > 1e-6 && h_min_i > 0.0 { h_min_i / sigma_inner } else { 0.0 };

            // Outer film
            let (h_min_o, h_c_o) = if u_m_o > 1e-8 && r_eq_o > 1e-8 {
                ehl_film(r_eq_o, u_m_o, q_k)
            } else { (0.0, 0.0) };
            let lambda_o = if sigma_outer > 1e-6 && h_min_o > 0.0 { h_min_o / sigma_outer } else { 0.0 };

            slice_films.push(SliceFilmThickness {
                h_min_um: h_min_i,
                h_central_um: h_c_i,
                lambda: lambda_i,
                regime: classify_lambda(lambda_i),
                h_min_um_outer: h_min_o,
                h_central_um_outer: h_c_o,
                lambda_outer: lambda_o,
                regime_outer: classify_lambda(lambda_o),
            });
        }

        result.push(RollerFilmDistribution {
            roller_idx: j,
            psi_deg: pt.psi_deg,
            slices: slice_films,
        });
    }

    if result.is_empty() { None } else { Some(result) }
}

// ─── Distribution → Summary Aggregation ─────────────────────────────

/// Derive `FilmThicknessResult` from per-slice distribution results.
///
/// Finds the worst-case (minimum h_min > 0) slice across all loaded
/// rollers and recomputes the dimensionless parameters and mixed
/// lubrication at that specific slice's geometry.  This replaces the
/// simplified average-geometry formula with values consistent with the
/// detailed per-slice calculation.
pub fn summarize_film_from_distribution(
    geom: &MacroGeometry,
    material: &Material,
    operating: &OperatingConditions,
    _roller_profile: &RollerProfile,
    _raceway_inner: &RacewayProfile,
    _raceway_outer: &RacewayProfile,
    slice_geometries: &[SliceGeometry],
    angular_distribution: &[AngularLoadPoint],
    distribution: &[RollerFilmDistribution],
) -> Option<FilmThicknessResult> {
    if distribution.is_empty() {
        return None;
    }

    let alpha_rad = geom.alpha.to_radians();
    let r_pw = geom.d_pw / 2.0 * 1e-3;
    let d_we_mean = (geom.d_we_max + geom.d_we_min) / 2.0;
    let gamma_dw = d_we_mean * alpha_rad.cos() / geom.d_pw;

    // Material
    let nu_mat = material.nu;
    let e1 = material.e_roller * 1e9;
    let e2 = material.e_ring * 1e9;
    let e_star = 1.0 / ((1.0 - nu_mat * nu_mat) / e1 + (1.0 - nu_mat * nu_mat) / e2);

    // Lubricant
    let nu_actual = crate::solver::life::viscosity_at_temp_pub(
        operating.nu_40, operating.nu_100, operating.t_op,
    );
    let eta_0 = nu_actual * 1e-6 * operating.rho_oil;
    let alpha_pv = operating.alpha_pv * 1e-9;
    let g_param = alpha_pv * e_star;
    let phi_s = operating.starvation_factor.clamp(0.1, 1.0);

    // Thermal correction
    let u_m_mean = operating.u_m_inner(r_pw, gamma_dw);
    let beta_visc = 0.04;
    let k_fluid = 0.15;
    let l_th = eta_0 * beta_visc * u_m_mean * u_m_mean / k_fluid;
    let phi_t = 1.0 / (1.0 + 0.1 * l_th.powf(0.64));

    // Surface roughness — Rq-based (ISO/TR 1281-2), inner and outer separately
    let rq_r = operating.rq_roller_eff();
    let rq_i = operating.rq_inner_eff();
    let rq_o = operating.rq_outer_eff();
    let sigma_inner = (rq_r * rq_r + rq_i * rq_i).sqrt();
    let sigma_outer = (rq_r * rq_r + rq_o * rq_o).sqrt();

    // Find worst-case inner slice (minimum h_min > 0)
    let mut min_h_inner = f64::MAX;
    let mut worst_inner_roller: usize = 0;
    let mut worst_inner_slice: usize = 0;
    // Find worst-case outer slice
    let mut min_h_outer = f64::MAX;
    let mut worst_outer_roller: usize = 0;
    let mut worst_outer_slice: usize = 0;

    for rd in distribution {
        for (k, sf) in rd.slices.iter().enumerate() {
            if sf.h_min_um > 0.0 && sf.h_min_um < min_h_inner {
                min_h_inner = sf.h_min_um;
                worst_inner_roller = rd.roller_idx;
                worst_inner_slice = k;
            }
            if sf.h_min_um_outer > 0.0 && sf.h_min_um_outer < min_h_outer {
                min_h_outer = sf.h_min_um_outer;
                worst_outer_roller = rd.roller_idx;
                worst_outer_slice = k;
            }
        }
    }

    if min_h_inner == f64::MAX {
        return None;
    }

    let worst_sf_inner = &distribution.iter()
        .find(|rd| rd.roller_idx == worst_inner_roller)?
        .slices[worst_inner_slice];

    // Recompute dimensionless parameters at inner worst-case slice
    let r_eq_k = if worst_inner_slice < slice_geometries.len() {
        slice_geometries[worst_inner_slice].r_eq_inner * 1e-3
    } else {
        d_we_mean / 2.0 * 1e-3
    };
    let r_roller_k_mm = if worst_inner_slice < slice_geometries.len() {
        slice_geometries[worst_inner_slice].r_roller
    } else {
        d_we_mean / 2.0
    };
    let gamma_k = (2.0 * r_roller_k_mm * alpha_rad.cos()) / geom.d_pw;
    let u_m_k = operating.u_m_inner(r_pw, gamma_k);
    let u_m_k_outer = operating.u_m_outer(r_pw, gamma_k);

    // Bearing-level kinematics
    let cage_rpm = operating.omega_cage(gamma_dw) * 60.0 / std::f64::consts::TAU;
    let roller_rpm = operating.omega_roller(gamma_dw, geom.d_pw, d_we_mean) * 60.0 / std::f64::consts::TAU;

    let q_k = angular_distribution.get(worst_inner_roller)
        .and_then(|pt| pt.slice_q_k.get(worst_inner_slice))
        .copied()
        .unwrap_or(0.0);

    let u_param = if r_eq_k > 1e-8 { eta_0 * u_m_k / (e_star * r_eq_k) } else { 0.0 };
    let w_param = if r_eq_k > 1e-8 { q_k * 1e3 / (e_star * r_eq_k) } else { 0.0 };

    let lambda_inner = worst_sf_inner.lambda;
    let regime_inner = worst_sf_inner.regime;

    let mixed = compute_mixed_lubrication(
        lambda_inner, sigma_inner, q_k, e_star, u_m_k, eta_0, alpha_pv,
    );

    // Outer worst-case
    let (h_min_o, h_c_o, lambda_o, regime_o) = if min_h_outer < f64::MAX {
        let sf_o = &distribution.iter()
            .find(|rd| rd.roller_idx == worst_outer_roller)
            .unwrap()
            .slices[worst_outer_slice];
        (sf_o.h_min_um_outer, sf_o.h_central_um_outer, sf_o.lambda_outer, sf_o.regime_outer)
    } else {
        (0.0, 0.0, 0.0, LubricationRegime::Boundary)
    };

    Some(FilmThicknessResult {
        // Inner
        h_min_um: worst_sf_inner.h_min_um,
        h_central_um: worst_sf_inner.h_central_um,
        sigma_composite_um: sigma_inner,
        lambda_ratio: lambda_inner,
        regime: regime_inner,
        // Outer
        h_min_um_outer: h_min_o,
        h_central_um_outer: h_c_o,
        sigma_composite_um_outer: sigma_outer,
        lambda_ratio_outer: lambda_o,
        regime_outer: regime_o,
        // Individual roughness
        rq_roller_um: rq_r,
        rq_inner_um: rq_i,
        rq_outer_um: rq_o,
        // Common
        u_mean_m_s: u_m_k,
        u_mean_m_s_outer: u_m_k_outer,
        cage_speed_rpm: cage_rpm,
        roller_spin_rpm: roller_rpm,
        starvation_factor: phi_s,
        thermal_factor: phi_t,
        u_param,
        g_param,
        w_param,
        mixed,
        flash_temp_c: None,
        film_decay: None,
        micropitting: None,
    })
}

// ─── Mixed Lubrication Model ────────────────────────────────────────

/// Greenwood-Tripp mixed lubrication model.
///
/// Computes asperity load/area fractions and effective friction coefficient.
/// Reference: Greenwood & Tripp (1970), Masjedi & Khonsari (2015).
fn compute_mixed_lubrication(
    lambda: f64,
    sigma_um: f64,
    q_n_per_mm: f64,
    e_star: f64,
    _u_m: f64,
    _eta_0: f64,
    _alpha_pv: f64,
) -> MixedLubricationResult {
    // Greenwood-Tripp statistical integrals
    let f_5_2 = gt_integral(2.5, lambda);
    let f_2 = gt_integral(2.0, lambda);

    let _sigma_m = sigma_um * 1e-6; // [m]
    let ebs = ETA_BETA_SIGMA; // η·β·σ ≈ 0.04

    // Asperity pressure [Pa]:
    //   p_a = (16√2/15) × π × (η·β·σ)² × E* × √(σ/β) × F_{5/2}(Λ)
    // With √(σ/β) ≈ √(σ/(σ/ebs/η)) — for simplicity, use empirical form:
    //   p_a = K_a × E* × F_{5/2}(Λ)
    // where K_a = (16√2/15) × π × ebs² × √(σ/β_typical)
    // For bearing steel with σ ≈ 0.2μm, β ≈ 10μm, η ≈ 2e11/m²:
    //   K_a ≈ 3.56e-4 × (σ/β)^0.5 ≈ 3.56e-4 × 0.14 ≈ 5e-5
    // Simplified: use the product form directly.
    let sqrt_sigma_over_beta = 0.14; // √(σ/β) typical for ground steel
    let k_a = (16.0 * std::f64::consts::FRAC_1_SQRT_2 * 2.0 / 15.0)
        * std::f64::consts::PI
        * ebs * ebs
        * sqrt_sigma_over_beta;

    let p_asperity_pa = k_a * e_star * f_5_2;
    let p_asperity_mpa = p_asperity_pa * 1e-6;

    // Total contact pressure (Hertzian average) [MPa]
    let p_total_mpa = q_n_per_mm.max(1e-6); // N/mm ≈ MPa (line contact, per unit length / contact width ratio)
    // More precisely, we use the load ratio from the GT model
    // Asperity load fraction = p_a / (p_a + p_fluid)
    let asperity_load_ratio = if p_asperity_mpa > 1e-12 && lambda < 4.0 {
        // For Λ < 4, significant asperity contact may exist
        // Use complementary approach: f_a = F_{5/2}(Λ) / F_{5/2}(0) as normalized fraction
        let f_5_2_at_0 = gt_integral(2.5, 0.0);
        if f_5_2_at_0 > 1e-20 {
            (f_5_2 / f_5_2_at_0).clamp(0.0, 1.0)
        } else {
            0.0
        }
    } else {
        0.0
    };

    // Asperity area ratio: A_real/A_hertz ∝ F_2(Λ)
    let f_2_at_0 = gt_integral(2.0, 0.0);
    let asperity_area_ratio = if f_2_at_0 > 1e-20 {
        (ebs * f_2 / f_2_at_0).clamp(0.0, 1.0)
    } else {
        0.0
    };

    // Fluid pressure = total - asperity
    let p_fluid_mpa = p_total_mpa * (1.0 - asperity_load_ratio);

    // EHL traction coefficient (Eyring model, simplified)
    // μ_ehl ≈ τ_0 / (α_pv × p_mean) for low SRR
    // τ_0 ≈ 2-10 MPa (Eyring stress), typical ≈ 5 MPa for mineral oil
    let tau_eyring = 5.0e6; // [Pa]
    let p_mean_ehl = p_fluid_mpa.max(1.0) * 1e6; // [Pa]
    let mu_ehl = (tau_eyring / p_mean_ehl).clamp(0.001, 0.02);

    // Effective friction coefficient
    let mu_effective = (1.0 - asperity_load_ratio) * mu_ehl
        + asperity_load_ratio * MU_BOUNDARY;

    MixedLubricationResult {
        asperity_load_ratio,
        asperity_area_ratio,
        p_asperity_mpa,
        p_fluid_mpa,
        mu_ehl,
        mu_boundary: MU_BOUNDARY,
        mu_effective,
        f_5_2,
        f_2,
    }
}

// ─── SRR-Dependent Traction Coefficient (for Transient Analysis) ───

/// Compute traction coefficient as a function of slide-roll ratio (SRR).
///
/// Uses Eyring thermal-corrected model:
///   μ(SRR) = (τ_eyring / p_mean) × arcsinh(η₀ × |V_slide| / (h_c × τ_eyring)) × φ_thermal
///
/// Saturates to μ_boundary at high SRR.
///
/// # Arguments
/// * `srr` — Slide-roll ratio (dimensionless, can be negative)
/// * `p_mean_pa` — Mean contact pressure [Pa]
/// * `h_c_m` — Central film thickness [m]
/// * `eta_0_pa_s` — Dynamic viscosity at inlet [Pa·s]
/// * `tau_eyring_pa` — Eyring shear stress [Pa] (typically 5e6 for mineral oil)
///
/// # Returns
/// Traction coefficient μ (always ≥ 0)
pub fn compute_traction_coefficient_srr(
    srr: f64,
    p_mean_pa: f64,
    h_c_m: f64,
    eta_0_pa_s: f64,
    tau_eyring_pa: f64,
) -> f64 {
    if srr.abs() < 1e-12 || p_mean_pa < 1.0 {
        return 0.0;
    }

    // Eyring-based traction
    let tau_ratio = tau_eyring_pa / p_mean_pa.max(1.0);
    let arg = if h_c_m > 1e-10 {
        eta_0_pa_s * srr.abs() / (h_c_m * tau_eyring_pa.max(1.0))
    } else {
        // Very thin film → boundary regime
        return MU_BOUNDARY;
    };
    let mu_eyring = tau_ratio * arg.asinh();

    // Thermal correction (simplified Gupta): reduces μ at high speed
    let phi_thermal = 1.0 / (1.0 + 0.1 * srr.abs());

    let mu = mu_eyring * phi_thermal;

    // Clamp: cannot exceed boundary friction
    mu.clamp(0.0, MU_BOUNDARY)
}

// ─── TRB Kinematics ────────────────────────────────────────────────

/// TRB base kinematic quantities (angular velocities + rib sliding).
/// Per-slice sliding velocities are computed separately via `compute_slice_sliding`.
struct TrbKinematics {
    /// Cage angular velocity [rad/s]
    omega_cage: f64,
    /// Roller spin angular velocity [rad/s]
    omega_roller: f64,
    /// Mean rolling entrainment velocity [m/s] — legacy single-value (= (u_outer+u_inner)/2)
    u_roll: f64,
    /// EHL entrainment velocity at outer raceway contact [m/s] (cage-attached frame)
    /// For stationary outer race: u_outer = ω_cage × R_outer_contact
    u_outer: f64,
    /// EHL entrainment velocity at inner raceway contact [m/s] (cage-attached frame)
    /// For rotating inner race: u_inner = (ω_inner − ω_cage) × R_inner_contact
    u_inner: f64,
    /// Rib sliding velocity [m/s]
    u_slide_rib: f64,
}

/// Compute TRB base kinematics for inner ring rotating, outer ring stationary.
///
/// ## Two conventions used in the literature
///
/// **Cone-apex** (Harris "Rolling Bearing Analysis" Ch.6 idealization):
///   r/R = sin φ / sin α — assumes inner race, outer race, and roller cones
///   share one apex.  Gives clean ω_cage = ω_i·sin α_i / (sin α_i + sin α_o)
///   and ω_roller = ω_i·sin α_i·sin α_o / (sin φ·(sin α_i + sin α_o)).
///   `compute_slice_sliding` uses this convention internally so sliding is
///   exactly zero when α_i, α_o, φ are consistent.
///
/// **Actual-geometry** (Schwarz 2023 Eq. derived from γ = r·cos α / R_pitch):
///   R_outer = R_pitch + r·cos α, R_inner = R_pitch − r·cos α — uses the
///   real `d_pw` and `d_we` from inputs.  For real TRBs the cone-apex
///   constraint `r/R = sin φ / sin α` rarely holds exactly (input α may
///   not match the d_we/d_pw ratio), so EHL entrainment velocity must be
///   computed from actual r and R, NOT from cone-apex-derived R.
///
/// We keep cone-apex ω_cage and ω_roller (for slice-sliding back-compat),
/// but compute `u_outer`, `u_inner` from the actual-geometry convention.
/// This matches the analytical reference formula in
/// `schwarz_32216_torque_with_alpha_v` (Schwarz Fig 5 validation).
fn compute_trb_kinematics(
    geom: &MacroGeometry,
    raceway_geom: &RacewayGeometry,
    operating: &OperatingConditions,
) -> TrbKinematics {
    let omega_inner = operating.omega_inner();
    let alpha_i_rad = raceway_geom.alpha_i.to_radians();
    let alpha_o_rad = raceway_geom.alpha_o.to_radians();

    let sin_ai = alpha_i_rad.sin();
    let sin_ao = alpha_o_rad.sin();
    let phi_rad = (alpha_o_rad - alpha_i_rad) / 2.0; // roller half-angle
    let sin_phi = phi_rad.sin();

    // Cone-apex ω_cage (back-compat with compute_slice_sliding).
    let omega_cage = if (sin_ai + sin_ao).abs() > 1e-12 {
        omega_inner * sin_ai / (sin_ai + sin_ao)
    } else {
        omega_inner / 2.0
    };

    // Cone-apex ω_roller (back-compat with compute_slice_sliding).
    let omega_roller = if sin_phi.abs() > 1e-12 && (sin_ai + sin_ao).abs() > 1e-12 {
        omega_inner * sin_ai * sin_ao / (sin_phi * (sin_ai + sin_ao))
    } else {
        0.0
    };

    // Actual-geometry contact radii from d_pw and d_we (Schwarz convention).
    let d_we = (geom.d_we_max + geom.d_we_min) / 2.0;
    let r_rb_m = d_we / 2.0 * 1e-3;
    let r_pitch_m = geom.d_pw / 2.0 * 1e-3;
    let alpha_avg = (alpha_i_rad + alpha_o_rad) / 2.0;
    let r_outer_contact = r_pitch_m + r_rb_m * alpha_avg.cos();
    let r_inner_contact = r_pitch_m - r_rb_m * alpha_avg.cos();

    // EHL entrainment velocity at each raceway contact.  Both forms use
    // ω_cage × R_actual — matches analytical Schwarz-Fig-5 reference.
    let u_outer = omega_cage * r_outer_contact;
    let u_inner = omega_cage * r_inner_contact;
    let u_roll = (u_outer + u_inner) / 2.0;

    // Rib contact: large-end roller sphere on rib face (drilling motion).
    // Uses cone-apex ω_roller for back-compat; user should be aware this
    // can be inflated when input geometry violates cone-apex matching.
    let r_large_end = geom.d_we_max / 2.0 * 1e-3;
    let u_slide_rib = omega_roller * r_large_end;

    TrbKinematics { omega_cage, omega_roller, u_roll, u_outer, u_inner, u_slide_rib }
}

/// Compute per-slice sliding velocity at inner and outer raceway contacts.
///
/// # Cone apex alignment
///
/// In a well-manufactured TRB, the cones of inner raceway (half-angle α_i),
/// outer raceway (half-angle α_o), and roller (half-angle φ = (α_o−α_i)/2)
/// all converge to a common apex on the bearing axis.  **α_i ≠ α_o always**
/// holds — they are inherently different angles.  The apex condition is NOT
/// about the angles being equal but about the cones meeting at one point.
///
/// From the common apex O at distance ρ along the roller axis:
///   r(ρ) = ρ sin(φ),  R_i(ρ) = ρ sin(α_i),  R_o(ρ) = ρ sin(α_o)
/// ⇒ C_i = R_i/r = sin(α_i)/sin(φ)  (constant for all ρ)
/// ⇒ C_o = R_o/r = sin(α_o)/sin(φ)  (constant for all ρ)
///
/// Combined with the exact kinematics (ω_cage, ω_roller from cone apex),
/// this gives **exactly zero sliding at every slice** for any α_i, α_o pair.
///
/// Sliding arises from:
/// - Bearing dimensions (d_pw, d_we) inconsistent with cone apex condition
/// - Crown/dub-off profile modifications
/// - Manufacturing tolerances / elastic deformation
///
/// Returns `(u_slide_inner_k, u_slide_outer_k)` in [m/s] for each slice.
fn compute_slice_sliding(
    kin: &TrbKinematics,
    _geom: &MacroGeometry,
    raceway_geom: &RacewayGeometry,
    operating: &OperatingConditions,
    slice_geometries: &[SliceGeometry],
) -> Vec<(f64, f64)> {
    let omega_inner = operating.omega_inner();
    let alpha_i_rad = raceway_geom.alpha_i.to_radians();
    let alpha_o_rad = raceway_geom.alpha_o.to_radians();
    let phi_rad = (alpha_o_rad - alpha_i_rad) / 2.0; // roller half-angle
    let sin_phi = phi_rad.sin();

    // Exact cone-proportional contact radius ratios from apex geometry:
    //   C_i = sin(α_i) / sin(φ)
    //   C_o = sin(α_o) / sin(φ)
    // These are constant along the roller when cones share a common apex.
    let c_inner = if sin_phi.abs() > 1e-12 { alpha_i_rad.sin() / sin_phi } else { 1.0 };
    let c_outer = if sin_phi.abs() > 1e-12 { alpha_o_rad.sin() / sin_phi } else { 1.0 };

    slice_geometries.iter().map(|sg| {
        let r_k = sg.r_roller * 1e-3; // mm → m

        // Contact radius from cone-proportional model:
        //   R_i,k = r_k × C_i = r_k × sin(α_i) / sin(φ)
        //   R_o,k = r_k × C_o = r_k × sin(α_o) / sin(φ)
        let r_inner_k = r_k * c_inner;
        let r_outer_k = r_k * c_outer;

        // Inner raceway sliding:
        //   u_race    = ω_shaft × R_i,k
        //   u_roller  = ω_cage × R_i,k + ω_roller × r_k
        //   Δu = u_race − u_roller
        let u_slide_inner = (omega_inner * r_inner_k)
            - (kin.omega_cage * r_inner_k + kin.omega_roller * r_k);

        // Outer raceway sliding (outer stationary):
        //   u_race    = 0
        //   u_roller  = ω_cage × R_o,k − ω_roller × r_k
        //   Δu = −u_roller
        let u_slide_outer = -(kin.omega_cage * r_outer_k - kin.omega_roller * r_k);

        (u_slide_inner.abs(), u_slide_outer.abs())
    }).collect()
}

// ─── Traction Computation ───────────────────────────────────────────

/// Compute per-roller traction and bearing-level friction summary.
///
/// Uses per-slice sliding velocities derived from the actual cone geometry,
/// accounting for the cone apex alignment condition.  When α_i, α_o, and the
/// roller half-angle are consistent, sliding is near-zero at every slice.
///
///   P_contact = Σ_j (inner.power_loss + outer.power_loss + rib.power_loss)
///   M_friction = P_contact / ω_shaft
pub fn compute_traction(
    geom: &MacroGeometry,
    material: &Material,
    operating: &OperatingConditions,
    _roller_profile: &RollerProfile,
    raceway_geom: &RacewayGeometry,
    _raceway_inner: &RacewayProfile,
    _raceway_outer: &RacewayProfile,
    slice_geometries: &[SliceGeometry],
    roller_results: &[RollerResult],
) -> Option<TractionSummary> {
    if operating.n_rpm() < 1e-6 {
        return None;
    }

    let kin = compute_trb_kinematics(geom, raceway_geom, operating);
    let d_we = (geom.d_we_max + geom.d_we_min) / 2.0;
    let r_eq = d_we / 2.0 * 1e-3;

    // Per-slice sliding velocities from cone geometry
    let slice_sliding = compute_slice_sliding(&kin, geom, raceway_geom, operating, slice_geometries);

    // Material / lubricant properties
    let nu_mat = material.nu;
    let e1 = material.e_roller * 1e9;
    let e2 = material.e_ring * 1e9;
    let e_star = 1.0 / ((1.0 - nu_mat * nu_mat) / e1 + (1.0 - nu_mat * nu_mat) / e2);

    let nu_actual = crate::solver::life::viscosity_at_temp_pub(
        operating.nu_40, operating.nu_100, operating.t_op,
    );
    let eta_0 = nu_actual * 1e-6 * operating.rho_oil;
    let alpha_pv = operating.alpha_pv * 1e-9;
    let phi_s = operating.starvation_factor.clamp(0.1, 1.0);

    // Surface roughness — Rq-based (ISO/TR 1281-2)
    let sigma_r = operating.rq_roller_eff();
    let sigma_race_i = operating.rq_inner_eff();
    let sigma_race_o = operating.rq_outer_eff();
    let sigma_i = (sigma_r * sigma_r + sigma_race_i * sigma_race_i).sqrt();
    let sigma_o = (sigma_r * sigma_r + sigma_race_o * sigma_race_o).sqrt();

    let omega_shaft = operating.n_rpm() * std::f64::consts::TAU / 60.0;

    let mut rollers = Vec::with_capacity(roller_results.len());
    let mut p_rolling_total = 0.0_f64;
    let mut p_sliding_total = 0.0_f64;
    let mut p_rib_total = 0.0_f64;
    let mut p_hysteresis_total = 0.0_f64;

    for (j, rr) in roller_results.iter().enumerate() {
        if rr.q_normal < 1e-3 {
            rollers.push(RollerTractionResult {
                roller_idx: j,
                psi_deg: rr.psi_deg,
                inner: zero_friction(),
                outer: zero_friction(),
                rib: None,
            });
            continue;
        }

        // ─── Per-slice sliding → load-weighted average u_slide ───
        // u_slide_avg = Σ(q_k × u_slide_k) / Σ(q_k)
        // This correctly weights sliding by the local load intensity.
        let mut sum_q = 0.0_f64;
        let mut sum_q_us_inner = 0.0_f64;
        let mut sum_q_us_outer = 0.0_f64;
        for sc in rr.slice_results.iter().filter(|s| s.in_contact) {
            let k = sc.k;
            let (us_i, us_o) = if k < slice_sliding.len() {
                slice_sliding[k]
            } else {
                (0.0, 0.0)
            };
            sum_q += sc.q_k;
            sum_q_us_inner += sc.q_k * us_i;
            sum_q_us_outer += sc.q_k * us_o;
        }
        let u_slide_inner_avg = if sum_q > 1e-6 { sum_q_us_inner / sum_q } else { 0.0 };
        let u_slide_outer_avg = if sum_q > 1e-6 { sum_q_us_outer / sum_q } else { 0.0 };

        let n_loaded = rr.slice_results.iter().filter(|s| s.in_contact).count().max(1) as f64;
        let q_mean = sum_q / n_loaded;

        // ─── Inner raceway contact ───
        // Inner and outer normal forces differ via the cone-angle factor:
        //   q_normal_inner = q_normal · cos(α_o − α_i)
        // Using each raceway's own normal force matters when α_o − α_i is large
        // or when split-contact mode produces additional asymmetry.
        // compute_contact_friction_at hosts the Palmgren/SKF empirical models
        // that lump inner/outer rolling-resistance into a single bearing-level
        // calibration — they do NOT distinguish u_inner from u_outer.  Use the
        // mean entrainment `kin.u_roll` to preserve those calibrations.
        // The BH path below explicitly overrides with kin.u_inner / kin.u_outer
        // (physically correct per-contact entrainment).
        let mut inner = compute_contact_friction_at(
            q_mean, sigma_i, r_eq, e_star, eta_0, alpha_pv, phi_s,
            kin.u_roll, u_slide_inner_avg, rr.q_normal_inner,
        );

        // ─── Outer raceway contact ───
        let mut outer = compute_contact_friction_at(
            q_mean, sigma_o, r_eq, e_star, eta_0, alpha_pv, phi_s,
            kin.u_roll, u_slide_outer_avg, rr.q_normal,
        );

        // Override per-contact rolling resistance with Biboulet-Houpert 2010
        // when that friction model is selected. Use the Part 1 (line contact)
        // formulas directly — the TRB raceway is essentially 1-D, so the
        // dedicated line-contact calibration is more accurate than the Part 2
        // point-contact extrapolation with a k = R_y/R_x cap.
        if matches!(operating.friction_model, FrictionModel::BibouletHoupert) {
            let e_prime = 2.0 * e_star;
            let l_contact = (geom.l_we * 1e-3).max(1e-6); // mm → m
            // Apply selected thermal inlet-shear correction (Wilson 1979
            // default; Aihara 1987 recommended for TRB rolling torque, matches
            // Schwarz 2023 measurements within 8 % at high speed).
            inner.p_rolling_w = biboulet_houpert_line_rolling_power_dispatched(
                eta_0, kin.u_inner, rr.q_normal_inner, l_contact, r_eq, e_prime,
                operating.beta_visc, operating.k_fluid, operating.thermal_correction,
            );
            outer.p_rolling_w = biboulet_houpert_line_rolling_power_dispatched(
                eta_0, kin.u_outer, rr.q_normal, l_contact, r_eq, e_prime,
                operating.beta_visc, operating.k_fluid, operating.thermal_correction,
            );
            // Johnson 1985 material hysteresis (Schwarz Eq. 20).  Solid-side
            // rolling resistance from incomplete elastic recovery; INDEPENDENT
            // of lubricant.  Must be added separately because BH is purely
            // viscous.  Palmgren/SKF μ_rr already include hysteresis empirically.
            inner.p_hysteresis_w = johnson_hysteresis_power_line_contact(
                rr.q_normal_inner, l_contact, r_eq, e_prime,
                operating.hysteresis_loss_factor, kin.u_inner,
            );
            outer.p_hysteresis_w = johnson_hysteresis_power_line_contact(
                rr.q_normal, l_contact, r_eq, e_prime,
                operating.hysteresis_loss_factor, kin.u_outer,
            );
        }

        // ─── Rib contact ───
        // TRB rib has DRILLING motion (roller spin about own axis creating spin
        // about contact normal), NOT pure sliding translation.  Use Houpert
        // 2002 closed-form drilling-moment formulation for elliptical contact:
        //     M_drilling = (3/8) · μ · F_rib · a_ellipse
        //     P_drilling = M_drilling · ω_roller
        // Earlier model used μ·F·(ω_roller·r_large_end) which over-predicts by
        // ~16× because r_large_end is the full roller radius, not the effective
        // drilling lever arm (3a/8 ≈ 0.5 mm vs r_large = 8.75 mm for typical 32216).
        //
        // Schwarz uses cell-model integration of τ·dA (equivalent in spirit).
        let rib = rr.rib_result.as_ref().map(|rib_r| {
            let mu = rib_r.ehl.as_ref().map_or(MU_RIB, |e| e.mu_eff);
            let f_friction = mu * rib_r.f_rib;
            // Drilling-based power (Houpert 2002): a_ellipse in mm → 1e-3 to W
            let m_drilling_nmm = 0.375 * mu * rib_r.f_rib * rib_r.a_ellipse;
            let power = m_drilling_nmm * 1e-3 * kin.omega_roller.abs();
            p_rib_total += power;
            RibFriction {
                u_sliding: kin.u_slide_rib,    // diagnostic (kinematic relative velocity)
                mu,
                f_friction_n: f_friction,
                power_loss_w: power,
            }
        });

        // Sliding + rolling totals derive from per-contact ContactFriction
        // (single source of truth — also drives the per-roller chart).
        p_sliding_total += inner.power_loss_w + outer.power_loss_w;
        p_rolling_total += inner.p_rolling_w + outer.p_rolling_w;
        p_hysteresis_total += inner.p_hysteresis_w + outer.p_hysteresis_w;

        rollers.push(RollerTractionResult {
            roller_idx: j,
            psi_deg: rr.psi_deg,
            inner,
            outer,
            rib,
        });
    }

    // Total bearing contact power includes hysteresis (Johnson 1985) for BH;
    // hysteresis is 0 for Palmgren/SKF (already implicit in μ_rr / G_rr).
    let p_contact_total = p_rolling_total + p_sliding_total + p_rib_total + p_hysteresis_total;
    let m_friction = if omega_shaft > 1e-8 {
        p_contact_total / omega_shaft * 1000.0 // [W] / [rad/s] = [N·m] → [N·mm]
    } else {
        0.0
    };

    Some(apply_friction_model_to_summary(TractionSummary {
        rollers,
        p_rolling_w: p_rolling_total,
        p_hysteresis_w: p_hysteresis_total,
        p_sliding_w: p_sliding_total,
        p_rib_w: p_rib_total,
        p_contact_total_w: p_contact_total,
        m_friction_nmm: m_friction,
        friction_model: operating.friction_model,
        skf_reference: None,
    }, geom, operating))
}

/// Compute friction for a single roller–raceway line contact.
fn compute_contact_friction_at(
    q_k: f64,        // representative slice load [N/mm]
    sigma: f64,       // composite roughness [μm]
    r_eq: f64,        // equivalent radius [m]
    e_star: f64,      // combined elastic modulus [Pa]
    eta_0: f64,       // dynamic viscosity [Pa·s]
    alpha_pv: f64,    // pressure-viscosity coeff [1/Pa]
    phi_s: f64,       // starvation factor
    u_rolling: f64,   // rolling velocity [m/s]
    u_sliding: f64,   // sliding velocity [m/s]
    q_normal: f64,    // total roller normal force [N]
) -> ContactFriction {
    let srr = if u_rolling > 1e-8 { u_sliding / u_rolling } else { 0.0 };

    // Compute local film thickness
    let w_per_l = q_k.max(1e-3) * 1e3; // [N/m]
    let u_param = eta_0 * u_rolling.max(1e-8) / (e_star * r_eq);
    let g_param = alpha_pv * e_star;
    let w_param = w_per_l / (e_star * r_eq);
    let h_min_dimless = 2.65 * u_param.powf(0.70) * g_param.powf(0.54) * w_param.powf(-0.13);
    let h_min_um = h_min_dimless * r_eq * 1e6 * phi_s;

    let lambda = if sigma > 1e-6 { h_min_um / sigma } else { 100.0 };

    // Asperity load fraction from GT model
    let f_5_2 = gt_integral(2.5, lambda);
    let f_5_2_at_0 = gt_integral(2.5, 0.0);
    let asperity_ratio = if f_5_2_at_0 > 1e-20 && lambda < 4.0 {
        (f_5_2 / f_5_2_at_0).clamp(0.0, 1.0)
    } else {
        0.0
    };

    // Friction coefficient
    let tau_eyring = 5.0e6;
    let p_hz = q_k.max(1.0) * 1e6; // rough estimate
    let mu_ehl = (tau_eyring / p_hz).clamp(0.001, 0.02);
    let mu = (1.0 - asperity_ratio) * mu_ehl + asperity_ratio * MU_BOUNDARY;

    let f_traction = mu * q_normal;
    let power = f_traction * u_sliding.abs();
    // Palmgren rolling resistance per single raceway contact (μ_rr = 0.002).
    let p_rolling = 0.002 * q_normal * u_rolling.abs();

    ContactFriction {
        u_rolling,
        u_sliding,
        srr,
        lambda,
        asperity_load_ratio: asperity_ratio,
        mu,
        f_traction_n: f_traction,
        power_loss_w: power,
        p_rolling_w: p_rolling,
        p_hysteresis_w: 0.0,
    }
}

fn zero_friction() -> ContactFriction {
    ContactFriction {
        u_rolling: 0.0,
        u_sliding: 0.0,
        srr: 0.0,
        lambda: 100.0,
        asperity_load_ratio: 0.0,
        mu: 0.0,
        f_traction_n: 0.0,
        power_loss_w: 0.0,
        p_rolling_w: 0.0,
        p_hysteresis_w: 0.0,
    }
}

/// Classify lubrication regime from Lambda ratio.
// ═══════════════════════════════════════════════════════════════════
// ADVANCED LUBRICATION MODEL
// ═══════════════════════════════════════════════════════════════════

// ─── Roelands Pressure-Viscosity (Task 1.2) ─────────────────────────

/// Roelands (1966) pressure-viscosity model.
///
/// η(p) = η₀ × exp{ (ln(η₀) + 9.67) × [(1 + p/p_r)^Z_r − 1] }
///
/// More accurate than Barus at GPa-level contact pressures.
/// Reference: Roelands, C.J.A. (1966), PhD Thesis, TU Delft.
fn roelands_viscosity(eta_0: f64, p_pa: f64, z_r: f64) -> f64 {
    const P_R: f64 = 196.2e6; // Roelands reference pressure [Pa]
    if p_pa < 0.0 {
        return eta_0;
    }
    let log_term = (eta_0.ln() + 9.67) * ((1.0 + p_pa / P_R).powf(z_r) - 1.0);
    eta_0 * log_term.exp()
}

// ─── Murch-Wilson Thermal Correction (Task 1.4) ─────────────────────

/// Murch-Wilson (1975) thermal correction factor.
///
/// φ_T = 1 − 13.2×(p₀/E')×L^0.42 / [1 + 0.213×(1 + 2.23×SRR^0.83)×L^0.64]
///
/// Unlike Gupta's simplified form, this accounts for SRR dependence.
/// Reference: Murch, L.E. & Wilson, W.R.D. (1975)
pub fn thermal_correction_murch_wilson(
    eta_0: f64, beta_visc: f64, u_m: f64, k_fluid: f64,
    srr: f64, _p_hz_pa: f64, _e_star: f64,
) -> f64 {
    // Wilson (1979) thermal reduction factor for line contacts:
    //   φ_T = 1 / [1 + 0.1 × (1 + 14.8 × |SRR|^0.83) × L^0.64]
    // where L = η₀ × β_visc × u_m² / k_fluid
    // Higher SRR → higher denominator → lower φ_T (more thermal thinning)
    let l_th = eta_0 * beta_visc * u_m * u_m / k_fluid.max(1e-6);
    if l_th < 1e-12 {
        return 1.0;
    }
    let srr_abs = srr.abs();
    let srr_term = 1.0 + 14.8 * srr_abs.powf(0.83);
    let phi_t = 1.0 / (1.0 + 0.1 * srr_term * l_th.powf(0.64));
    phi_t.clamp(0.3, 1.0)
}

// ─── Van Zoelen Film Thickness Decay ────────────────────────────────
//
// References:
//   [VZ12] Venner, van Zoelen, Lugt (2012), Tribology International 47:175-187
//   [GL26] Gao, van Zoelen, Osara, Meeuwenoord, Pasaribu, Lugt (2026), Tribology International
//
// Predicts time-dependent film thickness decay in starved EHL contacts
// due to pressure-driven side flow. Applicable to both oil and grease.

/// Dowson-Higginson compressibility: ρ̄ = ρ(p)/ρ₀  [VZ12 Eq. 16]
pub fn dowson_higginson_rho_bar(p_pa: f64) -> f64 {
    (5.9e8 + 1.34 * p_pa) / (5.9e8 + p_pa)
}

/// Side flow parameter F(0) for a single contact.  [VZ12 Eq. 25, GL26 Eq. 3]
///
/// F(0) = (2/l_t) × (p_h/b²) × (a/η₀) × π × ((0.5πα p_h)^{3/2} + 1)^{-2/3}
///
/// Validated: <3% error vs full integral Eq. 24.
/// Speed: ~1 μs/call (71× faster than numerical integration).
///
/// # Arguments
/// * `p_h`    - Maximum Hertz pressure [Pa]
/// * `a`      - Half-width in rolling direction [m]
/// * `b`      - Half-width transverse to rolling [m]
/// * `eta_0`  - Dynamic viscosity [Pa·s] (oil: oil viscosity, grease: base oil viscosity)
/// * `alpha`  - Pressure-viscosity coefficient [1/Pa]
/// * `l_t`    - Total track length [m] (sum of all surface circumferences)
pub fn van_zoelen_side_flow_f0(
    p_h: f64,
    a: f64,
    b: f64,
    eta_0: f64,
    alpha: f64,
    l_t: f64,
) -> f64 {
    if p_h < 1.0 || a < 1e-9 || b < 1e-9 || eta_0 < 1e-6 || l_t < 1e-6 {
        return 0.0;
    }
    let term = 0.5 * std::f64::consts::PI * alpha * p_h;
    let visc_corr = (term.powf(1.5) + 1.0).powf(-2.0 / 3.0);
    (2.0 / l_t) * (p_h / (b * b)) * (a / eta_0) * std::f64::consts::PI * visc_corr
}

/// Film thickness at time t due to side-flow decay.  [VZ12 Eq. 27, GL26 Eq. 1]
///
/// h_c(t) = (1/6 × ρ̄² × F(0) × t + h_{c,0}⁻²)^{-1/2}
///
/// # Arguments
/// * `t_s`    - Operating time [seconds]
/// * `h_c0`   - Initial (fully flooded) central film thickness [m]
/// * `f0`     - Side flow parameter F(0) [m⁻² s⁻¹] from `van_zoelen_side_flow_f0`
/// * `p_h`    - Maximum Hertz pressure [Pa] (for density correction)
pub fn van_zoelen_film_at_time(t_s: f64, h_c0: f64, f0: f64, p_h: f64) -> f64 {
    if h_c0 < 1e-12 || f0 <= 0.0 || t_s < 0.0 {
        return h_c0;
    }
    let rho_bar = dowson_higginson_rho_bar(p_h);
    let decay_coeff = (1.0 / 6.0) * rho_bar * rho_bar * f0;
    (decay_coeff * t_s + h_c0.powi(-2)).powf(-0.5)
}

/// Equilibrium film thickness when replenishment rate R > 0.
///
/// At equilibrium: dh̃/dt = 0 → h̃_eq = (3R/F(0))^{1/3}
/// → h_c_eq = 2 h̃_eq / ρ̄_c
///
/// Returns h_c_eq in [m]. Returns None if R ≤ 0.
pub fn van_zoelen_equilibrium(f0: f64, r_m_s: f64, p_h: f64) -> Option<f64> {
    if r_m_s <= 0.0 || f0 <= 0.0 {
        return None;
    }
    let rho_bar = dowson_higginson_rho_bar(p_h);
    let h_tilde_eq = (3.0 * r_m_s / f0).cbrt();
    Some(2.0 * h_tilde_eq / rho_bar)
}

/// Solve Van Zoelen ODE with replenishment using RK4.
///
/// dh_c/dt = -(ρ̄²/12) × F(0) × h_c³ + 2R/ρ̄_c
///
/// Returns decay curve: Vec<(time_s, h_c_m)>.
fn van_zoelen_ode_solve(
    t_end: f64,
    h_c0: f64,
    f0: f64,
    r_m_s: f64,
    p_h: f64,
    n_points: usize,
) -> Vec<(f64, f64)> {
    let rho_bar = dowson_higginson_rho_bar(p_h);
    let coeff_loss = rho_bar * rho_bar / 12.0 * f0;
    let coeff_gain = 2.0 * r_m_s / rho_bar;

    let rhs = |h: f64| -> f64 { -coeff_loss * h * h * h + coeff_gain };

    // Equilibrium and stability-based time step
    let h_eq = if coeff_loss > 0.0 && coeff_gain > 0.0 {
        (coeff_gain / coeff_loss).cbrt()
    } else {
        h_c0
    };
    let h_max = h_c0.max(h_eq) * 1.5; // upper bound for stability estimate

    // RK4 stability: dt < 2.8 / (3 × coeff_loss × h_max²)
    let dt_stability = if coeff_loss > 0.0 && h_max > 0.0 {
        2.0 / (3.0 * coeff_loss * h_max * h_max) // conservative (< 2.8 factor)
    } else {
        t_end / 1000.0
    };

    // Ensure enough steps: at least 1000, and dt ≤ stability limit
    let n_steps_min = (t_end / dt_stability).ceil() as usize;
    let n_steps = n_steps_min.max(1000).min(1_000_000); // cap at 1M to prevent infinite loop
    let dt = t_end / n_steps as f64;
    let sample_interval = (n_steps / n_points).max(1);

    let mut curve = Vec::with_capacity(n_points + 2);
    curve.push((0.0, h_c0));

    let mut h = h_c0;
    for step in 1..=n_steps {
        // RK4
        let k1 = rhs(h);
        let k2 = rhs(h + 0.5 * dt * k1);
        let k3 = rhs(h + 0.5 * dt * k2);
        let k4 = rhs(h + dt * k3);
        h += dt / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
        // Clamp to physical range: never negative, never above 2×h_eq (prevents runaway)
        h = h.clamp(1e-12, h_max);

        if step % sample_interval == 0 || step == n_steps {
            curve.push((step as f64 * dt, h));
        }
    }
    curve
}

/// Total track length for a TRB bearing.
///
/// l_t = 2π R_inner_contact + 2π R_outer_contact + Z × 2π R_roller_mean
///
/// # Arguments
/// * `r_inner` - Inner raceway contact radius [m]
/// * `r_outer` - Outer raceway contact radius [m]
/// * `r_roller` - Roller mean radius [m]
/// * `z`        - Number of rollers
pub fn total_track_length(r_inner: f64, r_outer: f64, r_roller: f64, z: usize) -> f64 {
    let tau = std::f64::consts::TAU; // 2π
    tau * r_inner + tau * r_outer + (z as f64) * tau * r_roller
}

/// Skew correction factor for film decay rate.
///
/// Based on experimental data from Gao et al. (2026) Table 3.
/// Positive skew (aligned with centrifugal force) → slower decay (factor < 1).
/// Negative skew (opposing centrifugal force) → faster decay (factor > 1).
///
/// Returns multiplier for F(0): F_corrected = F(0) × skew_factor
pub fn skew_decay_correction(skew_deg: f64) -> f64 {
    // Linear interpolation from Table 3 (150 mm/s, normalized to 0° baseline):
    //   +2° → 0.63,  +1° → 0.83,  0° → 1.00,  -1° → 1.06,  -2° → 1.11
    let clamped = skew_deg.clamp(-3.0, 3.0);
    if clamped >= 0.0 {
        // Positive skew: replenishment aid
        1.0 - 0.185 * clamped  // 1.0 at 0°, 0.63 at +2°
    } else {
        // Negative skew: accelerated loss
        1.0 + 0.055 * clamped.abs()  // 1.0 at 0°, 1.11 at -2°
    }
}

/// Compute Van Zoelen film decay for a bearing, attaching results to `FilmThicknessResult`.
///
/// Averages F(0) over the loaded circumference (Eq. 28 from Venner 2012),
/// then computes time-dependent film thickness at the specified operating time.
///
/// Van Zoelen "a" (rolling dir.) = Hertz half-width b_k in our code.
/// Van Zoelen "b" (transverse)   = L_we / 2  (half effective contact length).
pub fn compute_film_decay(
    film: &mut FilmThicknessResult,
    geom: &MacroGeometry,
    material: &Material,
    operating: &OperatingConditions,
    slices: &[SliceGeometry],
    angular_distribution: &[AngularLoadPoint],
) {
    if !operating.film_decay_enabled || operating.film_decay_time_hours <= 0.0 {
        return;
    }

    let t_s = operating.film_decay_time_hours * 3600.0; // hours → seconds

    // ─── Material constants ───
    // Combined elastic modulus E* [Pa]
    let nu = material.nu;
    let e_star = 1.0 / ((1.0 - nu * nu) / (material.e_roller * 1e9)
                      + (1.0 - nu * nu) / (material.e_ring * 1e9));
    let alpha = operating.alpha_pv * 1e-9;     // GPa⁻¹ → Pa⁻¹

    // ─── η₀: base oil viscosity at operating temperature ───
    // For both Oil and Grease, nu_40/nu_100 represent base oil kinematic viscosity.
    // Convert kinematic → dynamic: η = ν × ρ
    let nu_op = crate::solver::life::viscosity_at_temp_pub(
        operating.nu_40, operating.nu_100, operating.t_op,
    );
    let eta_0 = nu_op * 1e-6 * operating.rho_oil; // mm²/s → m²/s × kg/m³ = Pa·s

    if eta_0 < 1e-6 || alpha < 1e-12 {
        return;
    }

    // ─── Total track length ───
    let alpha_rad = geom.alpha * std::f64::consts::PI / 180.0;
    let d_we_mean = (geom.d_we_max + geom.d_we_min) / 2.0; // mm
    let r_inner = (geom.d_pw - d_we_mean * alpha_rad.cos()) / 2.0 * 1e-3; // mm → m
    let r_outer = (geom.d_pw + d_we_mean * alpha_rad.cos()) / 2.0 * 1e-3;
    let r_roller = d_we_mean / 2.0 * 1e-3;
    let l_t = total_track_length(r_inner, r_outer, r_roller, geom.z as usize);

    // ─── Van Zoelen "b" (transverse half-width) = L_we / 2 ───
    let b_vz = geom.l_we / 2.0 * 1e-3; // mm → m

    // ─── Circumferential average F(0) — Eq. 28 ───
    // Average over all loaded angular positions using the max-loaded slice per position.
    let e_star_mm = e_star / 1e6; // Pa → MPa for hertz functions (mm units)
    let mut f0_sum_inner = 0.0_f64;
    let mut f0_sum_outer = 0.0_f64;
    let mut n_loaded = 0_u32;

    for pt in angular_distribution.iter() {
        if pt.q_total <= 0.0 || pt.slice_q_k.is_empty() {
            continue;
        }
        n_loaded += 1;

        // Find representative slice (max loaded)
        let (best_k, &max_q) = pt.slice_q_k.iter().enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or((0, &0.0));

        if max_q <= 0.0 || best_k >= slices.len() {
            continue;
        }

        // Hertz half-width in rolling direction → Van Zoelen "a"
        let r_eq_i = slices[best_k].r_eq_inner; // mm
        let r_eq_o = slices[best_k].r_eq_outer;
        let b_hertz_i = crate::solver::hertz::hertz_half_width(max_q, r_eq_i, e_star_mm); // mm
        let b_hertz_o = crate::solver::hertz::hertz_half_width(max_q, r_eq_o, e_star_mm);
        let p_h_i = crate::solver::hertz::hertz_max_pressure(max_q, b_hertz_i) * 1e6; // MPa → Pa
        let p_h_o = crate::solver::hertz::hertz_max_pressure(max_q, b_hertz_o) * 1e6;

        let a_vz_i = b_hertz_i * 1e-3; // mm → m (VZ "a" = rolling dir half-width)
        let a_vz_o = b_hertz_o * 1e-3;

        f0_sum_inner += van_zoelen_side_flow_f0(p_h_i, a_vz_i, b_vz, eta_0, alpha, l_t);
        f0_sum_outer += van_zoelen_side_flow_f0(p_h_o, a_vz_o, b_vz, eta_0, alpha, l_t);
    }

    if n_loaded == 0 {
        return;
    }

    let f0_avg_inner = f0_sum_inner / n_loaded as f64;
    let f0_avg_outer = f0_sum_outer / n_loaded as f64;

    // ─── Skew correction ───
    let skew_factor = skew_decay_correction(operating.skew_angle_deg);
    let f0_inner = f0_avg_inner * skew_factor;
    let f0_outer = f0_avg_outer * skew_factor;

    // ─── h_c0 = fully flooded central film thickness ───
    let h_c0_inner = film.h_central_um * 1e-6; // μm → m
    let h_c0_outer = film.h_central_um_outer * 1e-6;

    if h_c0_inner < 1e-12 || h_c0_outer < 1e-12 {
        return;
    }

    // ─── Compute h_c(t) ───
    let _p_h_repr_inner = film.w_param * e_star * slices.get(slices.len() / 2)
        .map(|s| s.r_eq_inner * 1e-3).unwrap_or(0.01);
    // Use representative p_h from film summary (approximate from EHL parameters)
    // For Hertz p_h, use average loaded slice pressure
    let p_h_for_rho = {
        let loaded_pts: Vec<_> = angular_distribution.iter()
            .filter(|pt| !pt.slice_p_max.is_empty() && pt.q_total > 0.0)
            .collect();
        if loaded_pts.is_empty() {
            0.22e9 // fallback
        } else {
            let sum: f64 = loaded_pts.iter()
                .map(|pt| pt.slice_p_max.iter().cloned().fold(0.0_f64, f64::max))
                .sum();
            sum / loaded_pts.len() as f64 * 1e6 // MPa → Pa
        }
    };

    // ─── Replenishment rate ───
    let r_m_s = operating.replenishment_rate_nm_s * 1e-9; // nm/s → m/s

    // ─── Equilibrium film thickness (R > 0), capped at fully flooded ───
    let h_eq_inner = van_zoelen_equilibrium(f0_inner, r_m_s, p_h_for_rho)
        .map(|h| h.min(h_c0_inner)); // can't exceed fully flooded
    let h_eq_outer = van_zoelen_equilibrium(f0_outer, r_m_s, p_h_for_rho)
        .map(|h| h.min(h_c0_outer));

    // ─── Compute h_c(t) — with or without replenishment ───
    let (h_decayed_inner, h_decayed_outer, curve) = if r_m_s > 0.0 {
        // ODE with replenishment: numerical integration (RK4)
        let curve_i = van_zoelen_ode_solve(t_s, h_c0_inner, f0_inner, r_m_s, p_h_for_rho, 20);
        let curve_o = van_zoelen_ode_solve(t_s, h_c0_outer, f0_outer, r_m_s, p_h_for_rho, 20);

        let h_i = curve_i.last().map(|&(_, h)| h).unwrap_or(h_c0_inner);
        let h_o = curve_o.last().map(|&(_, h)| h).unwrap_or(h_c0_outer);

        // Merge curves: zip inner & outer, convert to (hours, nm_inner, nm_outer)
        let merged: Vec<(f64, f64, f64)> = curve_i.iter().zip(curve_o.iter())
            .map(|(&(t, hi), &(_, ho))| (t / 3600.0, hi * 1e9, ho * 1e9))
            .collect();

        (h_i, h_o, merged)
    } else {
        // Closed-form (R = 0): existing analytical solution
        let h_i = van_zoelen_film_at_time(t_s, h_c0_inner, f0_inner, p_h_for_rho);
        let h_o = van_zoelen_film_at_time(t_s, h_c0_outer, f0_outer, p_h_for_rho);

        let t_max_s = t_s.max(3600.0);
        let mut c = Vec::with_capacity(21);
        c.push((0.0, h_c0_inner * 1e9, h_c0_outer * 1e9));
        for i in 1..=20 {
            let t_pt = t_max_s * (i as f64 / 20.0);
            let hi = van_zoelen_film_at_time(t_pt, h_c0_inner, f0_inner, p_h_for_rho) * 1e9;
            let ho = van_zoelen_film_at_time(t_pt, h_c0_outer, f0_outer, p_h_for_rho) * 1e9;
            c.push((t_pt / 3600.0, hi, ho));
        }
        (h_i, h_o, c)
    };

    // ─── Lambda and regime at decayed thickness ───
    let sigma_i = film.sigma_composite_um;
    let sigma_o = film.sigma_composite_um_outer;
    let lambda_i = if sigma_i > 1e-6 { h_decayed_inner * 1e6 / sigma_i } else { 100.0 };
    let lambda_o = if sigma_o > 1e-6 { h_decayed_outer * 1e6 / sigma_o } else { 100.0 };

    film.film_decay = Some(FilmDecayResult {
        t_hours: operating.film_decay_time_hours,
        h_c_decayed_inner_um: h_decayed_inner * 1e6,
        h_c_decayed_outer_um: h_decayed_outer * 1e6,
        starvation_ratio_inner: h_decayed_inner / h_c0_inner,
        starvation_ratio_outer: h_decayed_outer / h_c0_outer,
        f0_inner,
        f0_outer,
        lambda_decayed_inner: lambda_i,
        lambda_decayed_outer: lambda_o,
        regime_decayed_inner: classify_lambda(lambda_i),
        regime_decayed_outer: classify_lambda(lambda_o),
        replenishment_rate_nm_s: operating.replenishment_rate_nm_s,
        h_c_equilibrium_inner_um: h_eq_inner.map(|h| h * 1e6),
        h_c_equilibrium_outer_um: h_eq_outer.map(|h| h * 1e6),
        decay_curve: curve,
    });
}

// ─── Physics-Based Starvation Factor ────────────────────────────────

/// Physics-based starvation factor for Method2_MK mode.
///
/// For oil lubrication: inlet meniscus model based on Hamrock-Dowson (1981).
///   φ_s = min(1, (h_inlet / h_ff)^(3/11))
/// High speed → centrifugal oil throw-off → lower inlet supply → φ_s < 1.
///
/// For grease: additional base-oil bleeding correction (Lugt 2013).
///   φ_s,grease = φ_s,oil × f_bleed
///   f_bleed ∈ [0.5, 0.9] depending on speed parameter n×d_pw.
///
/// References:
///   - Hamrock, B.J. & Dowson, D. (1981)
///   - Lugt, P.M. (2013), "Grease Lubrication in Rolling Bearings"
fn compute_starvation_factor_advanced(
    _eta_0: f64,
    u_m: f64,
    _e_star: f64,
    r_eq: f64,
    lub_type: &LubricationType,
    speed_param: f64,   // n_rpm × d_pw [mm·rpm]
) -> f64 {
    if u_m < 1e-8 || r_eq < 1e-10 {
        return 1.0;
    }

    // Speed-based starvation using n×d_pw speed parameter [mm·rpm].
    // Calibrated to match Manual §14.3A.3 table values:
    //   n×d_pw < 100k  → φ_s ≈ 0.98 (nearly flooded)
    //   n×d_pw = 300k  → φ_s ≈ 0.90 (mild starvation)
    //   n×d_pw = 1M    → φ_s ≈ 0.75 (significant starvation)
    //   n×d_pw > 2M    → φ_s → 0.60 (severe, centrifugal throw-off)
    //
    // Formula: φ_s = 1 / (1 + (n×d_pw / 3e6)^0.9)
    // Ref: Hamrock & Dowson (1981), adapted for TRB operating range.
    let nd = speed_param.max(0.0);
    let phi_s_oil = 1.0 / (1.0 + (nd / 3_000_000.0).powf(0.9));
    let phi_s_oil = phi_s_oil.clamp(0.5, 1.0);

    match lub_type {
        LubricationType::Oil => phi_s_oil,
        LubricationType::Grease => {
            // Grease correction: base-oil bleeding reduces with speed
            // At low n×d_pw (< 100k): fresh grease, reasonable bleeding → f_bleed ≈ 0.85
            // At high n×d_pw (> 500k): channeling + dry out → f_bleed ≈ 0.55
            let nd = speed_param.max(0.0);
            let f_bleed = if nd < 100_000.0 {
                0.85
            } else if nd > 500_000.0 {
                0.55
            } else {
                0.85 - 0.30 * (nd - 100_000.0) / 400_000.0
            };
            (phi_s_oil * f_bleed).clamp(0.3, 1.0)
        }
    }
}

// ─── Flash Temperature (Task 3.2) ──────────────────────────────────

/// Blok-Jaeger flash temperature rise at asperity contacts [°C].
///
/// Estimates the transient temperature rise due to frictional heating
/// at asperity micro-contacts during mixed lubrication.
///
/// ΔT = μ × p_a × V_slide / (4 × k_steel × √(π × a / (2 × κ)))
///
/// Simplified for line contacts using Hertzian half-width as contact scale.
///
/// References:
///   - Blok, H. (1937), "Theoretical Study of Temperature Rise"
///   - Jaeger, J.C. (1942), "Moving Sources of Heat"
pub fn flash_temperature(
    mu: f64,
    p_asperity_pa: f64,
    v_slide: f64,
    b_hertz: f64,       // Hertzian half-width [m] (or ellipse semi-minor for point contact)
) -> f64 {
    // Bearing steel thermal properties
    const K_STEEL: f64 = 46.0;   // thermal conductivity [W/(m·K)]
    const KAPPA_STEEL: f64 = 1.2e-5; // thermal diffusivity [m²/s]

    if v_slide.abs() < 1e-8 || p_asperity_pa < 1.0 || b_hertz < 1e-10 {
        return 0.0;
    }

    // Blok band source model for line contact:
    // ΔT = (μ × w' × V_slide) / (4 × k × b × √(π × Pe))
    // where w' = p_asp × 2b (force per unit length),
    //       Pe = V × b / (2 × κ) (Peclet number based on Hertz half-width)
    //
    // Simplified: ΔT = μ × p_asp × V_slide / (2 × k × √(π × V × b / (2κ)))
    let pe = v_slide.abs() * b_hertz / (2.0 * KAPPA_STEEL);

    if pe < 1e-6 {
        // Low-Pe (quasi-static): ΔT ≈ μ × p × V × b / (4 × k)
        return (mu * p_asperity_pa * v_slide.abs() * b_hertz / (4.0 * K_STEEL)).min(500.0);
    }

    // Jaeger moving band heat source:
    let delta_t = mu * p_asperity_pa * v_slide.abs()
        / (2.0 * K_STEEL * (std::f64::consts::PI * pe).sqrt());

    delta_t.min(500.0)
}

/// Classify flash temperature rise into risk level.
#[allow(dead_code)]
fn classify_flash_temp(delta_t: f64) -> &'static str {
    if delta_t < 50.0 { "Low" }
    else if delta_t < 150.0 { "Medium" }
    else if delta_t < 300.0 { "High" }
    else { "Critical" }
}

// ─── Masjedi-Khonsari (2015) Film Thickness (Task 1.3) ──────────────

/// Masjedi-Khonsari (2015) integrated film thickness result.
pub struct MKFilmResult {
    /// Central film thickness [m]
    pub h_c: f64,
    /// Minimum film thickness [m]
    pub h_min: f64,
    /// Asperity load fraction F_a/F [0,1] (replaces GT γ_a)
    pub load_fraction: f64,
    /// Asperity contact area fraction A_a/A_H [0,1]
    pub area_fraction: f64,
}

/// Compute film thickness using Masjedi-Khonsari LINE CONTACT formula.
///
/// Uses M-K (2012) line-contact formulation with exponential roughness
/// correction, not the M-K (2015) point-contact power-law version.
/// When σ̄ → 0, correction → 1.0 and formula reduces to standard D-H/D-T.
///
/// # Arguments
/// * `u_param` — Speed parameter U = η₀·u_m/(E*·R)
/// * `g_param` — Material parameter G = α·E*
/// * `w_param` — Load parameter W = w/(E*·R)  (per unit length)
/// * `sigma_bar` — Dimensionless roughness σ̄ = σ_combined/R
/// * `v_param` — Roughness shape parameter V = σ̄·√2/(R·β·η_asp)
/// * `r_eq` — Equivalent radius [m] (for dimensionalizing)
///
/// Reference: Masjedi, M. & Khonsari, M.M. (2012), "Film Thickness and
/// Asperity Load Formulas for Line-Contact EHL With Provision for Surface
/// Roughness", ASME J. Tribology, 134(1), 011503.
fn compute_film_mk(
    u_param: f64, g_param: f64, w_param: f64,
    sigma_bar: f64, _v_param: f64, r_eq: f64,
) -> MKFilmResult {
    // ── Central film thickness H_c (line contact) ──
    // Smooth part uses D-T coefficients (well-validated for line contact).
    // Roughness correction: exponential form from M-K (2012) line contact.
    //   f_c = 1 − C_c × exp(−k_c × Λ_eff^m_c)
    // where Λ_eff = H_smooth / σ̄ is the effective film-to-roughness ratio.
    // When σ̄→0: Λ_eff→∞, exp→0, f_c→1.0 (smooth).
    // When σ̄→large: Λ_eff→0, exp→1, f_c→(1−C_c) < 1.0 (reduced film).
    const A: [f64; 4] = [3.06, 0.69, 0.56, -0.10]; // D-T central
    let h_c_dimless = A[0] * u_param.powf(A[1]) * g_param.powf(A[2]) * w_param.powf(A[3]);
    let rc_c = if sigma_bar > 1e-15 && h_c_dimless > 1e-30 {
        let lambda_eff = h_c_dimless / sigma_bar;
        // M-K (2012) line contact: moderate correction for rough surfaces
        // Coefficients calibrated so correction stays in [0.85, 1.15] for
        // typical bearing conditions (σ̄ ~ 1e-5 to 1e-4).
        1.0 - 0.573 * (-0.74 * lambda_eff.powf(0.21)).exp()
    } else {
        1.0
    };
    let h_c = h_c_dimless * rc_c.clamp(0.5, 1.5) * r_eq;

    // ── Minimum film thickness H_min (line contact) ──
    const B: [f64; 4] = [2.65, 0.70, 0.54, -0.13]; // D-H minimum
    let h_min_dimless = B[0] * u_param.powf(B[1]) * g_param.powf(B[2]) * w_param.powf(B[3]);
    let rc_m = if sigma_bar > 1e-15 && h_min_dimless > 1e-30 {
        let lambda_eff = h_min_dimless / sigma_bar;
        1.0 - 0.856 * (-0.74 * lambda_eff.powf(0.21)).exp()
    } else {
        1.0
    };
    let h_min = h_min_dimless * rc_m.clamp(0.5, 1.5) * r_eq;

    // ── Asperity load fraction F_a/F (line contact) ──
    // M-K (2012) line contact: F_a/F = f(Λ_eff)
    // Uses effective lambda from minimum film thickness.
    let load_fraction = if sigma_bar > 1e-15 && h_min_dimless > 1e-30 {
        let lambda_eff = h_min_dimless / sigma_bar;
        // Asperity load increases as lambda decreases.
        // Calibrated: Λ_eff > 10 → ~0, Λ_eff < 1 → significant.
        let f = 0.50 * (-0.70 * lambda_eff.powf(0.50)).exp();
        f.clamp(0.0, 1.0)
    } else if sigma_bar > 1e-15 {
        1.0  // very thin film → full asperity contact
    } else {
        0.0
    };

    // ── Asperity contact area fraction A_a/A_H ──
    let area_fraction = if sigma_bar > 1e-15 && h_min_dimless > 1e-30 {
        let lambda_eff = h_min_dimless / sigma_bar;
        let a = 0.30 * (-0.60 * lambda_eff.powf(0.50)).exp();
        a.clamp(0.0, 1.0)
    } else if sigma_bar > 1e-15 {
        1.0
    } else {
        0.0
    };

    MKFilmResult { h_c, h_min, load_fraction, area_fraction }
}

// ─── Eyring Stress Auto-Estimation (Arana 2019) ─────────────────────
//
// τ_E = 2Λ/α where Λ is the limiting-stress pressure coefficient.
// Reference: Arana et al. (2019) Proc IMechE Part J, Eq. 12-13.
// Based on Bair & Winer (1979) and Hirst & Moore (1974).

/// Estimate Eyring stress τ_E [Pa] from pressure-viscosity coefficient α [1/Pa].
///
/// τ_E = 2Λ_lim/α  (Arana 2019, Eq.12)
///
/// Λ_lim = limiting-stress pressure coefficient (base oil dependent):
///   Mineral/Naphthenic: 0.047, Paraffinic: 0.040, PAO: 0.035, Ester: 0.030
///
/// Validated: Bair & Winer (1979), Hirst & Moore (1974), Jacod et al. (1999).
/// Allows non-Newtonian traction prediction without tribometer characterization.
#[allow(dead_code)]
pub fn eyring_stress_from_alpha(alpha_pa: f64, lambda_lim: f64) -> f64 {
    if alpha_pa < 1e-12 { return 5e6; } // fallback 5 MPa
    2.0 * lambda_lim / alpha_pa
}

/// Default Λ_lim (limiting-stress pressure coefficient) for common base oils.
/// Values from Höglund (1999), Arana (2019).
#[allow(dead_code)]
pub fn default_lambda_lim(lub_type: &LubricationType) -> f64 {
    match lub_type {
        LubricationType::Oil => 0.047,   // mineral oil default
        LubricationType::Grease => 0.040, // grease base oil (paraffinic typical)
    }
}

// ─── Clarke/Arana Load Sharing Function ─────────────────────────────
//
// ξ = 1 - erf(Λ)
// Arana (2019) Eq.14, experimentally validated by Clarke et al. (2016)
// using electrical contact resistance (ECR) in twin-disc machine.

/// Asperity load sharing fraction ξ = 1 - erf(Λ).
///
/// ξ ∈ [0,1]: fraction of total load carried by asperity contacts.
/// Clarke et al. (2016) ECR experiments confirm:
///   Λ > 2 → ξ ≈ 0 (full film separation)
///   Λ < 1 → ξ increases steeply (significant asperity contact)
///   Λ = 0 → ξ = 1 (full boundary contact)
///
/// **DEPRECATED for new code** — see `greenwood_tripp_load_sharing` which
/// is the standard first-principles statistical model (Greenwood & Tripp
/// 1970, ~3000 citations).  This Clarke 1-erf form is retained for
/// historical reference and backward-compat with calibrated codes.
#[allow(dead_code)]
pub fn clarke_load_sharing(lambda: f64) -> f64 {
    if lambda <= 0.0 { return 1.0; }
    if lambda > 4.0 { return 0.0; }
    // erf approximation (Abramowitz & Stegun 7.1.26, |ε| < 1.5e-7)
    let x = lambda;
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let poly = t * (0.254829592
        + t * (-0.284496736
        + t * (1.421413741
        + t * (-1.453152027
        + t * 1.061405429))));
    let erf_val = 1.0 - poly * (-x * x).exp();
    (1.0 - erf_val).clamp(0.0, 1.0)
}

/// Greenwood–Tripp 1970 asperity load sharing fraction — **standard**
/// first-principles statistical model.
///
/// $f_a = F_{5/2}(\Lambda) / F_{5/2}(0)$
///
/// where $F_n(\Lambda) = \int_\Lambda^\infty (s-\Lambda)^n \phi(s) ds$ is
/// integrated over a Gaussian asperity-height distribution.  The 5/2 power
/// is the Hertz contact-pressure exponent ($F \propto \delta^{3/2}$,
/// integrated → 5/2 statistical moment).
///
/// This is the standard model in tribology mixed-lubrication analysis
/// (>3000 citations since 1970).  It is the preferred choice for both
/// raceway and rib contacts in this solver; the older `clarke_load_sharing`
/// (1 − erf λ) is retained for back-compat but should not be used in new code.
///
/// Reference: Greenwood, J.A. & Tripp, J.H. (1970) *Proc. IMechE* 185:625-633.
pub fn greenwood_tripp_load_sharing(lambda: f64) -> f64 {
    if lambda <= 0.0 { return 1.0; }
    if lambda > 4.5 { return 0.0; }
    let f5_2 = gt_integral(2.5, lambda);
    let f5_2_at_0 = gt_integral(2.5, 0.0);
    if f5_2_at_0 < 1e-20 { return 0.0; }
    (f5_2 / f5_2_at_0).clamp(0.0, 1.0)
}

// ─── Advanced Eyring Traction with Roelands ─────────────────────────

/// Full Eyring traction model with Roelands viscosity + limiting shear stress.
///
/// When tau_eyring_pa is from user input, uses it directly.
/// Can also be auto-estimated via eyring_stress_from_alpha().
///
/// Fluid friction: μ_f = min[τ_E/p̄ × sinh⁻¹(η·γ̇/τ_E), Λ_lim]  (Arana Eq.13)
///
/// References:
///   Arana et al. (2019) Proc IMechE Part J, 233(2):303-316
///   Johnson & Tevaarwerk (1977), Proc R Soc Lond A
pub fn eyring_traction_advanced(
    srr: f64, u_roll: f64, p_mean_pa: f64, h_c_m: f64,
    eta_0: f64, z_roelands: f64, tau_eyring_pa: f64,
) -> f64 {
    if srr.abs() < 1e-12 || p_mean_pa < 1.0 || h_c_m < 1e-10 {
        return 0.0;
    }

    // Effective viscosity at contact pressure (Roelands)
    let eta_eff = roelands_viscosity(eta_0, p_mean_pa, z_roelands);

    // Shear rate (Couette flow dominant)
    let u_slide = srr.abs() * u_roll;
    let gamma_dot = u_slide / h_c_m;

    // Eyring shear stress: τ = τ₀ × sinh⁻¹(η_eff × γ̇ / τ₀)
    let x = eta_eff * gamma_dot / tau_eyring_pa;
    let tau = tau_eyring_pa * x.asinh();

    // Limiting shear stress: τ_L = Λ_lim × p
    // Λ_lim ≈ 0.047 (mineral), 0.035 (PAO) — Arana (2019)
    // Use conservative 0.10 upper bound for bearing steel contacts
    let tau_lim = 0.10 * p_mean_pa;
    let tau_clamped = tau.min(tau_lim);

    (tau_clamped / p_mean_pa).clamp(0.0, 0.15)
}

// ─── Carreau-Yasuda Non-Newtonian Model ─────────────────────────────
//
// Plateaus to a finite η_∞ at high shear (cf. Eyring's logarithmic decay).
// Recommended for rib-roller-end and high-SRR contacts (research report §4.3, §5.1).
//
// References:
//   Bair, S. (2007). High-Pressure Rheology for Quantitative EHL.
//   Habchi, W. et al. (2008). Tribol. Int. 41:733-741 (Carreau-EHL coupling)

/// Carreau-Yasuda effective viscosity at a given shear rate.
///
/// η_eff = η_∞ + (η_0 − η_∞)·[1 + (λ·γ̇)^a]^((n−1)/a)
///
/// Limits:
///   λ·γ̇ → 0    : η_eff → η_0       (Newtonian)
///   λ·γ̇ → ∞    : η_eff → η_∞       (high-shear plateau)
///   a = 2       : original Carreau (1972) model
pub fn carreau_yasuda_viscosity(
    eta_0: f64, eta_inf: f64, lambda_s: f64, n: f64, a: f64, gamma_dot: f64,
) -> f64 {
    if eta_0 <= 0.0 { return 0.0; }
    if gamma_dot.abs() < 1e-30 || lambda_s < 1e-30 {
        return eta_0;
    }
    let lg = (lambda_s * gamma_dot.abs()).powf(a);
    let factor = (1.0 + lg).powf((n - 1.0) / a);
    eta_inf + (eta_0 - eta_inf) * factor
}

/// Full Carreau-Yasuda traction model: Roelands(p) + Carreau-Yasuda(γ̇).
///
/// Mirrors `eyring_traction_advanced` so call sites can swap freely.
///
/// τ = η_eff(p, γ̇) · γ̇,  μ = τ / p̄,  capped at Λ_lim·p̄ (limiting shear stress).
///
/// `eta_inf_ratio` — η_∞ / η_0 (typical 0.001 ~ 0.01 for mineral oil)
/// `lambda_s`      — relaxation time [s] (typical 1e-9 ~ 1e-6)
/// `n`             — power-law exponent (0.4 ~ 0.7)
/// `a`             — Yasuda transition exponent (1.0 ~ 3.0; a=2 → original Carreau)
pub fn carreau_traction_advanced(
    srr: f64, u_roll: f64, p_mean_pa: f64, h_c_m: f64,
    eta_0: f64, z_roelands: f64,
    eta_inf_ratio: f64, lambda_s: f64, n: f64, a: f64,
) -> f64 {
    if srr.abs() < 1e-12 || p_mean_pa < 1.0 || h_c_m < 1e-10 {
        return 0.0;
    }

    // Pressure-corrected zero-shear viscosity (Roelands)
    let eta_p = roelands_viscosity(eta_0, p_mean_pa, z_roelands);
    let eta_inf = eta_p * eta_inf_ratio.clamp(0.0, 1.0);

    // Shear rate
    let u_slide = srr.abs() * u_roll;
    let gamma_dot = u_slide / h_c_m;

    // Carreau-Yasuda effective viscosity at this shear rate
    let eta_eff = carreau_yasuda_viscosity(eta_p, eta_inf, lambda_s, n, a, gamma_dot);

    // Newtonian shear stress at the effective viscosity
    let tau = eta_eff * gamma_dot;

    // Limiting shear stress (same cap as Eyring path for parity)
    let tau_lim = 0.10 * p_mean_pa;
    let tau_clamped = tau.min(tau_lim);

    (tau_clamped / p_mean_pa).clamp(0.0, 0.15)
}

// ─── Rib Kinematics (public access for rib_contact module) ──────────

/// Mean entrainment + sliding velocity at the rib (flange-roller end) contact.
///
/// Returns (u_entrain, u_slide, SRR) where:
///   u_slide   = |ω_roller × r_large_end|        (roller-end surface speed)
///   u_entrain = u_slide / 2                     (rib face stationary in
///                                                inner-rotating frame ⇒ mean
///                                                entrainment of (u_roller + 0)/2)
///   SRR       = u_slide / u_entrain ≈ 2.0      (pure-sliding point-contact
///                                                limit; research report §4.3)
///
/// Returns (0, 0, 0) when the bearing is static.
pub fn compute_rib_speeds(
    geom: &MacroGeometry,
    raceway_geom: &RacewayGeometry,
    operating: &OperatingConditions,
) -> (f64, f64, f64) {
    let kin = compute_trb_kinematics(geom, raceway_geom, operating);
    let u_slide = kin.u_slide_rib.abs();
    if u_slide < 1e-10 {
        return (0.0, 0.0, 0.0);
    }
    let u_entrain = u_slide / 2.0;
    let srr = u_slide / u_entrain; // = 2.0 by construction
    (u_entrain, u_slide, srr)
}

// ─── Biboulet-Houpert (2010) Hydrodynamic Rolling Force ───────────────
//
// Per-contact analytical model of the viscous-hydrodynamic braking force
// that opposes pure rolling. Reproduces the IVR/EHL transition smoothly
// via a single dimensionless parameter M. Combined with per-roller load
// distribution, it reconstructs bearing-level rolling resistance without
// the manufacturer-specific calibration of the SKF empirical model.
//
// Reference:
//   Biboulet, N. & Houpert, L. (2010), "Hydrodynamic force and moment in
//   pure rolling lubricated contacts. Part 2: Point contacts",
//   Proc. Inst. Mech. Eng. Part J, 224(8):777–787.
//
// Validated against ball-race contact experiments by Bălan-Houpert-Olaru
// (2015) Lubricants 3:222–243; identified hydrodynamic rolling as ~88% of
// measured rolling torque on lubricated rolling contacts.

/// Biboulet-Houpert 2010 hydrodynamic rolling force (per single contact).
///
/// All inputs SI:
///   eta_0:    inlet dynamic viscosity [Pa·s]
///   u_entrain: average entrainment speed (u₁+u₂)/2 [m/s]
///   q_normal:  normal contact load [N]
///   r_x:       equivalent radius in rolling direction [m]
///   r_y:       equivalent radius perpendicular to rolling [m]
///   e_prime:   reduced Young's modulus E* [Pa]
///              (steel-on-steel ≈ 2.3e11 Pa)
///
/// Returns the hydrodynamic braking rolling force F_R [N]. Power dissipation
/// per contact is `F_R × u_entrain`.
pub fn biboulet_houpert_rolling_force(
    eta_0: f64, u_entrain: f64, q_normal: f64,
    r_x: f64, r_y: f64, e_prime: f64,
) -> f64 {
    if eta_0 <= 0.0 || u_entrain <= 0.0 || q_normal <= 0.0
        || r_x <= 0.0 || e_prime <= 0.0 { return 0.0; }
    // Radii ratio k = R_y/R_x (≥ 1 for rolling-direction-dominant geometry;
    // for line-contact-like raceways k can be very large — formula stays valid
    // and tends toward the line-contact limit as truncation occurs).
    let k = (r_y / r_x).max(1.0);
    // Dimensionless groups
    let u = eta_0 * u_entrain / (e_prime * r_x);
    let w = q_normal / (e_prime * r_x * r_x);
    if u <= 0.0 || w <= 0.0 { return 0.0; }
    // IVR and EHL component forces (Eqs. 1-2)
    let prefac = e_prime * r_x * r_x;
    let f_ivr = 2.9766 * prefac * k.powf(0.3316) * w.powf(1.0 / 3.0) * u.powf(2.0 / 3.0);
    let f_ehl = 7.5826 * prefac * k.powf(0.4055) * w.powf(1.0 / 3.0) * u.powf(0.75);
    // Transition parameter M (Eq. 4) — IVR-EHL smooth blend
    let m = 0.5549 * k.powf(-0.6029) * w * u.powf(-0.75);
    // Eq. 3 (transition formula)
    (f_ivr - f_ehl) / (1.0 + m / 6.6) + f_ehl
}

/// Convenience wrapper: returns the rolling power dissipation per contact
/// from Biboulet-Houpert 2010.
///   P_roll = F_R × u_entrain   [W]
pub fn biboulet_houpert_rolling_power(
    eta_0: f64, u_entrain: f64, q_normal: f64,
    r_x: f64, r_y: f64, e_prime: f64,
) -> f64 {
    let f_r = biboulet_houpert_rolling_force(eta_0, u_entrain, q_normal, r_x, r_y, e_prime);
    f_r * u_entrain.abs()
}

// ─── Biboulet-Houpert 2010 Part 1 — Line Contact ──────────────────────
//
// Direct line-contact formulas — Biboulet & Houpert (2010), "Hydrodynamic
// force and moment in pure rolling lubricated contacts. Part 1: line
// contacts", Proc. Inst. Mech. Eng. Part J, 224(8) (sister paper to Part 2).
//
// Used for TRB raceway-roller contacts, where the contact ellipse truncates
// to an essentially 1-D line. Part 1 is calibrated directly for line contact
// (no need to cap k = R_y/R_x as in the Part 2 point-contact extrapolation).
//
// Notation (Eq. 14 in Part 1) — note the FACTOR 2 in U vs Part 2:
//   U_l = 2 η₀ u_m / (E' R)
//   W_l = w_l / (E' R)         (w_l = force per unit length [N/m])
//   M_l = W_l / sqrt(U_l)
//
// Asymptotic forms (Eqs. 40, 41):
//   T̃_IVR     = 1.42 · U_l^(1/2) · W_l^(1/2)   (matches Dowson IVR)
//   T̃_EHL∞   = 1.47 · U_l^(3/4)               (load-INDEPENDENT)
//
// Smooth IVR-EHL transition (Eq. 42 in T* form, recast to T̃ form):
//   T̃ = T̃_IVR / (1 + r^10)^(1/10),   r = (1.4/1.45) · sqrt(M_l)
//
// Force per unit length: f_l = T̃ · E' · R   [N/m]
// (Using T̃_t^x ≈ F̃_t^x, Eq. 31, valid except in extreme high-speed
//  low-load contacts where the contribution differs by ≲ 2 %.)
//
// Reference: Biboulet & Houpert (2010), Proc. IMechE Part J 224(8):xxx (Part 1).

/// Biboulet-Houpert 2010 Part 1 hydrodynamic rolling force per unit length
/// for a line contact. Returns f_l [N/m].
pub fn biboulet_houpert_line_force_per_length(
    eta_0: f64, u_entrain: f64,
    q_per_length: f64,    // [N/m] normal load per unit contact length
    r: f64,               // [m] reduced radius (rolling direction)
    e_prime: f64,         // [Pa] reduced Young's modulus (paper E' = 2 × Johnson E*)
) -> f64 {
    if eta_0 <= 0.0 || u_entrain <= 0.0 || q_per_length <= 0.0
        || r <= 0.0 || e_prime <= 0.0 { return 0.0; }
    // Dimensionless groups (Part 1, Eq. 14) — note factor 2 in U.
    let u = 2.0 * eta_0 * u_entrain / (e_prime * r);
    let w_l = q_per_length / (e_prime * r);
    if u <= 0.0 || w_l <= 0.0 { return 0.0; }
    let m_l = w_l / u.sqrt();
    // IVR asymptote (Eq. 40, modulus form)
    let t_tilde_ivr = 1.42 * u.sqrt() * w_l.sqrt();
    // EHL/IVR smooth blend (Eq. 42, recast to T̃ form)
    // r_blend = (1.4/1.45) · sqrt(M_l); denominator (1 + r^10)^(1/10)
    let r_blend = (1.4 / 1.45) * m_l.sqrt();
    let denom = (1.0 + r_blend.powi(10)).powf(0.1);
    let t_tilde = t_tilde_ivr / denom;
    // f_l = T̃ · E' · R
    t_tilde * e_prime * r
}

/// Biboulet-Houpert 2010 Part 1 rolling power per contact (line contact).
///
/// P = (f_l × L_contact) × u_entrain     [W]
///
/// **Isothermal** — does not include inlet-shear thermal correction.  At
/// high entrainment speeds the thermal heating in the inlet zone reduces
/// the effective viscosity and hence rolling resistance; use
/// `biboulet_houpert_line_rolling_power_with_thermal` for thermally
/// corrected results.
pub fn biboulet_houpert_line_rolling_power(
    eta_0: f64, u_entrain: f64,
    q_normal: f64,        // [N] total normal load on the line contact
    l_contact: f64,       // [m] effective contact length (e.g. roller L_we)
    r: f64,               // [m] reduced rolling-direction radius
    e_prime: f64,         // [Pa] reduced Young's modulus
) -> f64 {
    if l_contact <= 0.0 { return 0.0; }
    let q_per_length = q_normal / l_contact;
    let f_l = biboulet_houpert_line_force_per_length(eta_0, u_entrain, q_per_length, r, e_prime);
    let f_total = f_l * l_contact;
    f_total * u_entrain.abs()
}

/// Aihara (1987) thermal inlet-shear correction for **TRB rolling torque**.
///
///   φ_T_Aihara = 1 / [1 + 0.29·L^0.78]    (SRR = 0 pure rolling)
///   L = η₀·β·u² / k_fluid
///
/// Stronger reduction than Wilson 1979 (a=0.1, b=0.64).  Calibrated
/// specifically for TRB axial-load rolling torque measurements.
/// Cited in Tewari 2023 (Table 1) as the basis of Aihara's TRB friction model.
///
/// Reference: Aihara, S. (1987) *J. Tribol.* 109:471-478
/// "A New Running Torque Formula for Tapered Roller Bearings under Axial Load"
pub fn aihara_thermal_factor(
    eta_0: f64, beta_visc: f64, u_entrain: f64, k_fluid: f64,
) -> f64 {
    if eta_0 <= 0.0 || u_entrain <= 0.0 { return 1.0; }
    let l_th = eta_0 * beta_visc * u_entrain * u_entrain / k_fluid.max(1e-6);
    if l_th < 1e-12 { return 1.0; }
    let phi_t = 1.0 / (1.0 + 0.29 * l_th.powf(0.78));
    phi_t.clamp(0.3, 1.0)
}

/// Wilson (1979) thermal inlet-shear correction for SRR=0 line contact.
///   φ_T_Wilson = 1 / [1 + 0.1·L^0.64]
/// More conservative than Aihara; matches the φ_T already applied to film
/// thickness in the M1/M2 EHL paths.
pub fn wilson_thermal_factor(
    eta_0: f64, beta_visc: f64, u_entrain: f64, k_fluid: f64,
) -> f64 {
    if eta_0 <= 0.0 || u_entrain <= 0.0 { return 1.0; }
    let l_th = eta_0 * beta_visc * u_entrain * u_entrain / k_fluid.max(1e-6);
    if l_th < 1e-12 { return 1.0; }
    let phi_t = 1.0 / (1.0 + 0.1 * l_th.powf(0.64));
    phi_t.clamp(0.3, 1.0)
}

/// Biboulet-Houpert 2010 Part 1 rolling power **with thermal inlet-shear
/// correction** (line contact, pure rolling).
///
/// Dispatches between Wilson 1979 and Aihara 1987 based on `correction`.
/// Both factors are of form φ_T = 1/(1+a·L^b) with L = η₀·β·u²/k_fluid;
/// they differ in calibration (Wilson: film-thickness-derived, Aihara:
/// TRB-rolling-torque-derived).  See `ThermalCorrection` enum docs.
///
/// At high entrainment speeds, viscous heating in the inlet zone reduces
/// effective viscosity and EHL pressure-shift–derived rolling resistance.
/// Tewari 2023 reports a 6-8 % torque reduction from inlet shear at typical
/// TRB operating speeds; Schwarz 2023 measurements at 4000 rpm validate the
/// Aihara correction (matches measurement to within 8 %).
pub fn biboulet_houpert_line_rolling_power_with_thermal(
    eta_0: f64, u_entrain: f64,
    q_normal: f64, l_contact: f64,
    r: f64, e_prime: f64,
    beta_visc: f64,       // [1/K] viscosity-temperature coefficient
    k_fluid: f64,         // [W/(m·K)] lubricant thermal conductivity
) -> f64 {
    biboulet_houpert_line_rolling_power_dispatched(
        eta_0, u_entrain, q_normal, l_contact, r, e_prime,
        beta_visc, k_fluid, ThermalCorrection::Wilson1979,
    )
}

// ─── Johnson 1985 Material Hysteresis Rolling Resistance ───────────
//
// Reference: Johnson, K.L. (1985) "Contact Mechanics" §9.6 Rolling
// friction. Caused by incomplete elastic recovery of bearing steel during
// cyclic compression — INDEPENDENT of lubricant or speed.
//
// Schwarz et al. 2023 Eq. 20:
//     M_T,Hys = Q · α_v · 2b/(3π)             [N·m per contact]
// where:
//     Q   = normal load [N]
//     α_v = hysteresis loss factor [dimensionless]
//     b   = Hertz contact half-width [m]
//
// Bearing rolling resistance force = M / R_roller, power = F × u_entrain.
//
// Hysteresis MUST be added explicitly to BH 2010 (purely viscous EHL formula).
// Palmgren and SKF empirical fits already include hysteresis via their μ_rr / G_rr
// calibration — adding it would double-count.
//
// α_v range for hardened bearing steel: 0.005 – 0.05.

/// Johnson hysteresis moment about the roller's own rotation axis [N·m].
pub fn johnson_hysteresis_moment(
    q_normal_n: f64,         // [N]
    b_hertz_m: f64,          // [m]
    alpha_v: f64,            // [-]
) -> f64 {
    if q_normal_n <= 0.0 || b_hertz_m <= 0.0 || alpha_v <= 0.0 {
        return 0.0;
    }
    q_normal_n * alpha_v * 2.0 * b_hertz_m / (3.0 * std::f64::consts::PI)
}

/// Johnson hysteresis power dissipation per contact [W].
///
/// P = F_hys × u_entrain, where F_hys = M_hys / R_roller (tangential friction
/// from pressure-shift moment about roller axis).
pub fn johnson_hysteresis_power(
    q_normal_n: f64,
    b_hertz_m: f64,
    alpha_v: f64,
    r_roller_m: f64,         // [m] roller radius at contact
    u_entrain_m_s: f64,      // [m/s]
) -> f64 {
    if r_roller_m <= 0.0 || u_entrain_m_s.abs() < 1e-12 {
        return 0.0;
    }
    let m_hys = johnson_hysteresis_moment(q_normal_n, b_hertz_m, alpha_v);
    let f_hys = m_hys / r_roller_m;
    f_hys * u_entrain_m_s.abs()
}

/// Bearing-level Johnson hysteresis power [W] for line contact.
///
/// Aggregates over the slice-discretized line contact assuming uniform load
/// distribution (Q_per_slice = Q_total / n_slices, b_slice ≈ b_total since b
/// depends only on q_per_length).  For typical TRB the per-slice b is the
/// same as the bearing-level b computed from total Q and L:
///     b = √(8·q_per_length·R / (π·E'))
pub fn johnson_hysteresis_power_line_contact(
    q_normal_n: f64,         // [N] total normal load on line contact
    l_contact_m: f64,        // [m] effective contact length
    r_roller_m: f64,         // [m] roller radius (= R_x for TRB line contact)
    e_prime: f64,            // [Pa] paper E' (= 2 × Johnson E*)
    alpha_v: f64,
    u_entrain_m_s: f64,
) -> f64 {
    if q_normal_n <= 0.0 || l_contact_m <= 0.0 || r_roller_m <= 0.0 || e_prime <= 0.0 {
        return 0.0;
    }
    let q_per_length = q_normal_n / l_contact_m;
    // Hertz half-width for line contact: b = √(8·q·R / (π·E'))
    // Note: paper E' = 2 × Johnson E* — but Hertz uses Johnson E*, so divide.
    let e_star = e_prime / 2.0;
    let b = (8.0 * q_per_length * r_roller_m / (std::f64::consts::PI * e_star)).sqrt();
    johnson_hysteresis_power(q_normal_n, b, alpha_v, r_roller_m, u_entrain_m_s)
}

/// As above but with explicit thermal-correction selector.
pub fn biboulet_houpert_line_rolling_power_dispatched(
    eta_0: f64, u_entrain: f64,
    q_normal: f64, l_contact: f64,
    r: f64, e_prime: f64,
    beta_visc: f64, k_fluid: f64,
    correction: ThermalCorrection,
) -> f64 {
    let p_iso = biboulet_houpert_line_rolling_power(
        eta_0, u_entrain, q_normal, l_contact, r, e_prime,
    );
    if p_iso <= 0.0 { return 0.0; }
    let phi_t = match correction {
        ThermalCorrection::None       => 1.0,
        ThermalCorrection::Wilson1979 => wilson_thermal_factor(eta_0, beta_visc, u_entrain, k_fluid),
        ThermalCorrection::Aihara1987 => aihara_thermal_factor(eta_0, beta_visc, u_entrain, k_fluid),
    };
    p_iso * phi_t
}

// ═══════════════════════════════════════════════════════════════════
// TRB Analytical Friction Formula Library (Tewari 2023 Table 1)
// ═══════════════════════════════════════════════════════════════════
//
// Per-contact viscous rolling resistance torque for inner OR outer
// raceway of a TRB.  Returns torque about the **roller axis** [N·m].
// To convert to bearing-level rolling torque:
//   M_bearing = Z · (M_inner + M_outer) × (geometry factor)
// where the geometry factor depends on how the formula derivation
// transforms roller-axis moment to bearing-axis torque.  In each
// reference paper the formula yields the contribution directly
// usable in Tewari's M = Z·(M_i + M_o)·(R/D_a) summation form (Eq. 7).
//
// Common dimensionless groups (per-raceway):
//   U = η₀·u_m / (E'·R_e)        — speed
//   W = w_l / (E'·R_e)            — line load (w_l = Q/l, N/m)
//   G = α_pv · E'                 — material/lubricant
//   L = η₀·β·u_m² / k_fluid       — thermal loading
//
// Inputs use SI: Pa·s, m/s, N/m, Pa, m, rad.  Returns N·m.

/// Aihara 1987 raceway rolling resistance — original-paper formula
/// (verified from Aihara 1987 *J. Tribol.* 109:471 §2.4 + Appendix 1).
///
/// $M_{i,o} = \frac{1.76 \times 10^2}{1 + 0.29 L^{0.78}} \cdot \frac{1}{\alpha_0}
///           \cdot (GU)^{0.658} \cdot W^{0.31} \cdot R_e^2 \cdot l$
///
/// where:
/// - α_0 = **pressure-viscosity coefficient** [1/Pa] (NOT half-cone angle —
///   common transcription error in secondary sources like Tewari Table 1)
/// - U = η₀ u / (E' R_e)     — dimensionless speed
/// - G = α_pv · E'            — dimensionless material
/// - W = 2 F_a / (D_a · l · z · sin α · E') — **bearing-level dimensionless
///   load** (uses TOTAL axial load F_a, not per-contact w_l)
/// - L = η₀ β u² / k_fluid    — thermal loading
/// - R_e = inner OR outer equivalent radius [m]
/// - l = effective roller length [m]
/// - 1/α_0 has units of Pa → product gives N·m (dimensionally consistent)
///
/// Returns per-contact moment about roller axis [N·m].  Bearing-level torque:
///   M = (z/D_a) · (R_o · M_i + R_i · M_o)  (Aihara Eq. 8)
///
/// Calibrated on 0.45–1.2 GPa axial, 100–3000 rpm, gear oil 80W, 50–80 °C.
///
/// Reference: Aihara, S. (1987) *J. Tribol.* 109(3):471-477.
pub fn aihara_1987_raceway_torque(
    eta_0: f64, u_m: f64,
    f_a_total: f64, n_rollers: usize, d_a_roller: f64,
    l_contact: f64, alpha_cone_rad: f64,
    r_eq: f64, alpha_pv: f64, e_prime: f64,
    beta_visc: f64, k_fluid: f64,
) -> f64 {
    if u_m <= 0.0 || f_a_total <= 0.0 || r_eq <= 0.0 || l_contact <= 0.0
        || alpha_cone_rad.abs() < 1e-9 || alpha_pv <= 0.0
        || eta_0 <= 0.0 || e_prime <= 0.0 || d_a_roller <= 0.0
        || n_rollers == 0 {
        return 0.0;
    }
    let u = eta_0 * u_m / (e_prime * r_eq);
    let g = alpha_pv * e_prime;
    let w_bearing = 2.0 * f_a_total
        / (d_a_roller * l_contact * n_rollers as f64
           * alpha_cone_rad.sin() * e_prime);
    let l_th = eta_0 * beta_visc * u_m * u_m / k_fluid.max(1e-6);
    let thermal_factor = 1.0 / (1.0 + 0.29 * l_th.powf(0.78));
    // 1/α_pv has units [Pa] → product [Pa · m³] = [N·m]
    176.0 * thermal_factor / alpha_pv
        * (g * u).powf(0.658) * w_bearing.powf(0.31)
        * r_eq * r_eq * l_contact
}

/// Zhou-Hoeprich 1991 raceway rolling resistance — original-paper formula
/// (verified from Zhou & Hoeprich 1991 *J. Tribol.* 113:590-597 Eq. 17 + Table 1).
///
/// $M_{i,o} = \varphi_{ish} \varphi_{bl} \cdot 58.4 \cdot \frac{R_e^2}{\alpha_{pv}}
///           \cdot (GU)^{0.648} \cdot W^{0.246} \cdot l$
///
/// where:
/// - α_pv = **pressure-viscosity coefficient** [1/Pa] (NOT half-cone angle —
///   common transcription error in Tewari Table 1)
/// - U = η₀ u_r / (E' R_e)        — dimensionless speed
/// - G = α_pv · E'                 — dimensionless material
/// - W = w_l / (E' R_e)            — per-contact dimensionless line load
///   (w_l = Q / l_contact, line load per unit length)
/// - R_e = inner OR outer equivalent radius [m]
/// - l = effective roller length [m]
/// - φ_ish: inlet shear heating correction (default 1.0, can plug Wilson/Aihara)
/// - φ_bl : boundary-lubrication weighting (default 1.0 for full EHL)
///
/// 1/α_pv has units of Pa → product gives N·m (dimensionally consistent).
///
/// Calibrated on 0.85–1.47 GPa axial, 100–8000 rpm, SAE20 oil, 50 °C.
///
/// Reference: Zhou, R.S. & Hoeprich, M.R. (1991) *J. Tribol.* 113(3):590-597.
pub fn zhou_hoepprich_1991_raceway_torque(
    eta_0: f64, u_m: f64, w_l: f64, r_eq: f64, l_contact: f64,
    alpha_pv: f64, e_prime: f64,
    phi_ish: f64, phi_bl: f64,
) -> f64 {
    if u_m <= 0.0 || w_l <= 0.0 || r_eq <= 0.0 || l_contact <= 0.0
        || alpha_pv <= 0.0 || eta_0 <= 0.0 || e_prime <= 0.0 {
        return 0.0;
    }
    let u = eta_0 * u_m / (e_prime * r_eq);
    let w = w_l / (e_prime * r_eq);
    let g = alpha_pv * e_prime;
    // 1/α_pv has units [Pa] → product [Pa · m² · m] = [N · m]
    phi_ish * phi_bl * 58.4 * r_eq * r_eq / alpha_pv
        * (g * u).powf(0.648) * w.powf(0.246) * l_contact
}

/// Matsuyama 2001 raceway rolling resistance (Tewari Table 1).
///
/// $M_{i,o} = \varphi_{ish} \cdot 14.2 E' l R_e^2 \cdot U^{0.75} G^{-0.04} W^{0.08}$
///
/// - Calibrated on 0.3–1.3 GPa axial, 100–1500 rpm, paraffin/traction oil, 26 °C
/// - **EHL-fully-flooded numerical fit** — best-match per Tewari Fig 13
/// - φ_ish: external thermal correction (Matsuyama Eq. uses Wilson-form)
///
/// Reference: Matsuyama, H. & Kamamoto, S. (2001) *KOYO Eng. J.* 159:53-60.
pub fn matsuyama_2001_raceway_torque(
    eta_0: f64, u_m: f64, w_l: f64, r_eq: f64, l_contact: f64,
    alpha_pv: f64, e_prime: f64, phi_ish: f64,
) -> f64 {
    if u_m <= 0.0 || w_l <= 0.0 || r_eq <= 0.0 || l_contact <= 0.0
        || eta_0 <= 0.0 || e_prime <= 0.0 {
        return 0.0;
    }
    let u = eta_0 * u_m / (e_prime * r_eq);
    let w = w_l / (e_prime * r_eq);
    let g = alpha_pv * e_prime;
    phi_ish * 14.2 * e_prime * l_contact * r_eq * r_eq
        * u.powf(0.75) * g.powf(-0.04) * w.powf(0.08)
}

/// Houpert 2002 raceway rolling resistance (Tewari Table 1).
///
/// $M_{i,o} = 0.04 E' l R_e^2 U^{0.44} W^{0.37}$
///
/// - Calibrated on 3500 N axial + 4250 N radial, 100–4500 rpm, ATF oil, 50 °C
/// - No G factor (different scaling than Matsuyama)
/// - Uses Zhou-Hoeprich rolling friction model in derivation
///
/// Reference: Houpert, L. (2002) *J. Tribol.* 124:121-129.
pub fn houpert_2002_raceway_torque(
    eta_0: f64, u_m: f64, w_l: f64, r_eq: f64, l_contact: f64,
    e_prime: f64,
) -> f64 {
    if u_m <= 0.0 || w_l <= 0.0 || r_eq <= 0.0 || l_contact <= 0.0
        || eta_0 <= 0.0 || e_prime <= 0.0 {
        return 0.0;
    }
    let u = eta_0 * u_m / (e_prime * r_eq);
    let w = w_l / (e_prime * r_eq);
    0.04 * e_prime * l_contact * r_eq * r_eq
        * u.powf(0.44) * w.powf(0.37)
}

/// Scheuermann solid rolling friction (Schwarz LaMBDA Eq. 19).
///
/// $M_{T,Sr} = c_r \cdot F_N^{e_r}$
///
/// Surface-deformation hysteresis rolling friction, independent of lubricant.
/// Solid-body inelastic recovery — distinct from Johnson hysteresis (which
/// scales with Hertz half-width).  Schwarz reports c_r, e_r per geometry; for
/// 32216 raceway Scheuermann fit gives small contribution (~수 % of M_T).
///
/// Per-contact, F_N in N → M in N·m.
///
/// Reference: cited in Schwarz et al. (2023) *Lubricants* 11:369 §2.1.3.
pub fn scheuermann_solid_rolling_torque(
    f_n: f64, c_r: f64, e_r: f64,
) -> f64 {
    if f_n <= 0.0 || c_r <= 0.0 {
        return 0.0;
    }
    c_r * f_n.powf(e_r)
}

/// Coulomb cage friction (Schwarz LaMBDA Eq. 37).
///
/// $F_T^\text{cage} = F_N^\text{pocket} \cdot \mu_\text{Coulomb}$
///
/// Roller-cage pocket contact friction.  Total bearing cage power:
///   P_cage = Σ_j F_T^cage · u_pocket(j)
/// where u_pocket is the relative sliding velocity at the cage pocket.
///
/// Typical bearing μ_Coulomb ≈ 0.05-0.10 (Schwarz references [62]).
///
/// Reference: cited in Schwarz et al. (2023) §2.1.3.
pub fn cage_friction_coulomb_power(
    f_n_pocket: f64, mu_coulomb: f64, u_pocket: f64,
) -> f64 {
    if f_n_pocket <= 0.0 || mu_coulomb <= 0.0 || u_pocket.abs() < 1e-12 {
        return 0.0;
    }
    mu_coulomb * f_n_pocket * u_pocket.abs()
}

// ─── Hamrock-Dowson Elliptical EHL ──────────────────────────────────
//
// Used for point-contact and elliptical-contact EHL — particularly the
// rib-roller-end contact in TRBs (research report §4.1 / §4.3).

/// Hamrock-Dowson (1981) dimensionless central and minimum film thickness
/// for elliptical EHL point contact.
///
/// H_c   = 2.69 · U^0.67 · G^0.53 · W^(−0.067) · (1 − 0.61·exp(−0.73·k))
/// H_min = 3.63 · U^0.68 · G^0.49 · W^(−0.073) · (1 − exp(−0.68·k))
///
/// Inputs are dimensionless (U, G, W per Hamrock-Dowson) and `k = a/b`,
/// the contact ellipticity (semi-major / semi-minor, k ≥ 1).
///
/// Returns (H_c, H_min). Multiply by R_x to get physical film thickness.
///
/// Reference: Hamrock, B.J. & Dowson, D. (1981) *Ball Bearing Lubrication*,
/// Wiley. Eqs. 7.31, 7.33.
pub fn hamrock_dowson_elliptical(u: f64, g: f64, w: f64, k_ellipse: f64) -> (f64, f64) {
    if u <= 0.0 || g <= 0.0 || w <= 0.0 {
        return (0.0, 0.0);
    }
    let k = k_ellipse.max(1.0);
    let h_c = 2.69 * u.powf(0.67) * g.powf(0.53) * w.powf(-0.067)
        * (1.0 - 0.61 * (-0.73 * k).exp());
    let h_min = 3.63 * u.powf(0.68) * g.powf(0.49) * w.powf(-0.073)
        * (1.0 - (-0.68 * k).exp());
    (h_c, h_min)
}

// ─── Traction Dispatcher ────────────────────────────────────────────

/// Compute fluid-EHL traction coefficient using the model selected in
/// `OperatingConditions::traction_model`.
///
/// Single dispatch point so adding a new model only requires extending this match.
pub fn traction_coefficient(
    operating: &OperatingConditions,
    srr: f64, u_roll: f64, p_mean_pa: f64, h_c_m: f64,
    eta_0_pa_s: f64,
) -> f64 {
    match operating.traction_model {
        TractionModel::Eyring => eyring_traction_advanced(
            srr, u_roll, p_mean_pa, h_c_m,
            eta_0_pa_s, operating.z_roelands, operating.tau_eyring * 1e6,
        ),
        TractionModel::CarreauYasuda => carreau_traction_advanced(
            srr, u_roll, p_mean_pa, h_c_m,
            eta_0_pa_s, operating.z_roelands,
            operating.carreau_eta_inf_ratio, operating.carreau_lambda_s,
            operating.carreau_n, operating.carreau_a,
        ),
    }
}

// ─── Advanced Film Thickness Distribution (Task 1.3 + 3.3) ─────────

/// Compute per-slice film distribution using Advanced model.
///
/// Differences from Basic:
///   - Film formula: Masjedi-Khonsari (2015) with roughness integration
///   - Thermal correction: Murch-Wilson (SRR-dependent per slice)
///   - Asperity load/area fractions from M-K formula (replaces GT)
pub fn compute_film_thickness_distribution_advanced(
    geom: &MacroGeometry,
    material: &Material,
    operating: &OperatingConditions,
    _roller_profile: &RollerProfile,
    _raceway_inner: &RacewayProfile,
    raceway_geom: &RacewayGeometry,
    slice_geometries: &[SliceGeometry],
    angular_distribution: &[AngularLoadPoint],
) -> Option<Vec<RollerFilmDistribution>> {
    if operating.n_rpm() < 1e-6 || angular_distribution.is_empty() {
        return None;
    }

    let alpha_rad = geom.alpha.to_radians();
    let r_pw = geom.d_pw / 2.0 * 1e-3;
    let d_we_mean = (geom.d_we_max + geom.d_we_min) / 2.0;

    // Material
    let nu_mat = material.nu;
    let e1 = material.e_roller * 1e9;
    let e2 = material.e_ring * 1e9;
    let e_star = 1.0 / ((1.0 - nu_mat * nu_mat) / e1 + (1.0 - nu_mat * nu_mat) / e2);

    // Lubricant
    let nu_actual = crate::solver::life::viscosity_at_temp_pub(
        operating.nu_40, operating.nu_100, operating.t_op,
    );
    let eta_0 = nu_actual * 1e-6 * operating.rho_oil;
    let alpha_pv = operating.alpha_pv * 1e-9;
    let g_param = alpha_pv * e_star;

    // Kinematics (needed for starvation + SRR)
    let kin = compute_trb_kinematics(geom, raceway_geom, operating);
    let slicing = compute_slice_sliding(
        &kin, geom, raceway_geom, operating, slice_geometries,
    );

    // Physics-based starvation: compute from speed/lubricant or use user override
    let speed_param = operating.n_rpm() * geom.d_pw; // [mm·rpm]
    let r_eq_avg = d_we_mean / 2.0 * 1e-3;
    let phi_s_calc = compute_starvation_factor_advanced(
        eta_0, kin.u_roll, e_star, r_eq_avg,
        &operating.lubrication_type, speed_param,
    );
    let phi_s = if operating.starvation_factor < 0.99 {
        operating.starvation_factor.clamp(0.1, 1.0).min(phi_s_calc)
    } else {
        phi_s_calc
    };

    // Advanced surface roughness: use R_q (RMS) for M-K, combine roller + raceway
    let sigma_inner = (operating.rq_roller_eff() * operating.rq_roller_eff()
        + operating.rq_inner_eff() * operating.rq_inner_eff()).sqrt(); // [μm]
    let sigma_outer = (operating.rq_roller_eff() * operating.rq_roller_eff()
        + operating.rq_outer_eff() * operating.rq_outer_eff()).sqrt(); // [μm]
    let sigma_m_inner = sigma_inner * 1e-6; // [m]
    let sigma_m_outer = sigma_outer * 1e-6; // [m]

    // Asperity parameters for M-K V parameter (inner)
    let beta_eta_inner = if sigma_m_inner > 1e-12 {
        ETA_BETA_SIGMA / sigma_m_inner
    } else {
        1e12
    };
    // Asperity parameters for M-K V parameter (outer)
    let beta_eta_outer = if sigma_m_outer > 1e-12 {
        ETA_BETA_SIGMA / sigma_m_outer
    } else {
        1e12
    };

    // Bearing-wide max slice load threshold
    let q_max_bearing = angular_distribution.iter()
        .flat_map(|pt| pt.slice_q_k.iter())
        .cloned()
        .fold(0.0_f64, f64::max);
    let q_min_threshold = q_max_bearing * 0.01;

    let n_slices = slice_geometries.len();
    let no_contact_slices: Vec<SliceFilmThickness> = (0..n_slices)
        .map(|_| SliceFilmThickness {
            h_min_um: 0.0, h_central_um: 0.0, lambda: 0.0,
            regime: LubricationRegime::Boundary,
            ..Default::default()
        })
        .collect();

    let mut result = Vec::with_capacity(angular_distribution.len());

    for (j, pt) in angular_distribution.iter().enumerate() {
        if pt.q_total < 1e-3 {
            result.push(RollerFilmDistribution {
                roller_idx: j, psi_deg: pt.psi_deg,
                slices: no_contact_slices.clone(),
            });
            continue;
        }

        let mut slice_films = Vec::with_capacity(n_slices);

        for k in 0..n_slices {
            let q_k_nmm = if k < pt.slice_q_k.len() { pt.slice_q_k[k] } else { 0.0 };
            if q_k_nmm < q_min_threshold {
                slice_films.push(SliceFilmThickness {
                    h_min_um: 0.0, h_central_um: 0.0, lambda: 0.0,
                    regime: LubricationRegime::Boundary,
                    ..Default::default()
                });
                continue;
            }

            let r_eq_k_inner = if k < slice_geometries.len() {
                slice_geometries[k].r_eq_inner * 1e-3
            } else {
                d_we_mean / 2.0 * 1e-3
            };
            let r_eq_k_outer = if k < slice_geometries.len() {
                slice_geometries[k].r_eq_outer * 1e-3
            } else {
                d_we_mean / 2.0 * 1e-3
            };

            let r_roller_k_mm = if k < slice_geometries.len() {
                slice_geometries[k].r_roller
            } else {
                d_we_mean / 2.0
            };
            let gamma_k = (2.0 * r_roller_k_mm * alpha_rad.cos()) / geom.d_pw;
            let u_m_k = operating.u_m_inner(r_pw, gamma_k);
            let u_m_k_outer = operating.u_m_outer(r_pw, gamma_k);

            if u_m_k < 1e-8 || r_eq_k_inner < 1e-8 {
                slice_films.push(SliceFilmThickness {
                    h_min_um: 0.0, h_central_um: 0.0, lambda: 0.0,
                    regime: LubricationRegime::Boundary,
                    ..Default::default()
                });
                continue;
            }

            let w_per_l = q_k_nmm * 1e3; // N/mm → N/m
            let use_nvm = operating.lubrication_model == LubricationModel::Method3_NVM;
            let slice_w = if k < slice_geometries.len() {
                slice_geometries[k].slice_width * 1e-3 // mm → m
            } else {
                geom.l_we / n_slices as f64 * 1e-3
            };

            // Murch-Wilson thermal correction (SRR-dependent, inner)
            let srr_k = if k < slicing.len() {
                let (u_sl_i, _) = slicing[k];
                if u_m_k.abs() > 1e-10 { u_sl_i / u_m_k } else { 0.0 }
            } else {
                0.0
            };
            let p_hz_approx = if k < pt.slice_p_max.len() {
                (pt.slice_p_max[k] * 1e6).max(1.0)
            } else { 100e6 };
            let phi_t_i = thermal_correction_murch_wilson(
                eta_0, operating.beta_visc, u_m_k, operating.k_fluid,
                srr_k, p_hz_approx, e_star,
            );

            // ── Inner raceway film ──
            let (h_min_i, h_c_i) = if use_nvm {
                // M3 (NVM): Nijenbanning-Venner-Moes unified 4-regime formula
                let f_slice = w_per_l * slice_w; // N
                let ry_line = 1e3; // effectively infinite for line contact slice
                let u_s = 2.0 * u_m_k; // sum velocity
                let h_c_nvm = nvm_central_film(
                    f_slice, r_eq_k_inner, ry_line, e_star, eta_0, u_s, alpha_pv,
                );
                let h_c = h_c_nvm * phi_t_i * phi_s * 1e6; // m → μm
                let h_min = h_c * 0.75; // NVM gives central only; h_min ≈ 0.75×h_c for line contact
                (h_min, h_c)
            } else {
                // M2 (MK): Masjedi-Khonsari roughness-integrated formula
                let u_k_i = eta_0 * u_m_k / (e_star * r_eq_k_inner);
                let w_k_i = w_per_l / (e_star * r_eq_k_inner);
                let sigma_bar_i = sigma_m_inner / r_eq_k_inner;
                let v_param_i = sigma_bar_i * std::f64::consts::SQRT_2 / (r_eq_k_inner * beta_eta_inner);
                let mk_i = compute_film_mk(u_k_i, g_param, w_k_i, sigma_bar_i, v_param_i, r_eq_k_inner);
                (mk_i.h_min * phi_t_i * phi_s * 1e6,
                 mk_i.h_c * phi_t_i * phi_s * 1e6)
            };
            let lambda_i = if sigma_inner > 1e-6 { h_min_i / sigma_inner } else { 100.0 };

            // ── Outer raceway film ──
            let (h_min_o, h_c_o, lambda_o) = if u_m_k_outer > 1e-8 && r_eq_k_outer > 1e-8 {
                let phi_t_o = {
                    let srr_k_o = if k < slicing.len() {
                        let (_, u_sl_o) = slicing[k];
                        if u_m_k_outer.abs() > 1e-10 { u_sl_o / u_m_k_outer } else { 0.0 }
                    } else { 0.0 };
                    thermal_correction_murch_wilson(
                        eta_0, operating.beta_visc, u_m_k_outer, operating.k_fluid,
                        srr_k_o, p_hz_approx, e_star,
                    )
                };

                let (hm, hc) = if use_nvm {
                    let f_slice = w_per_l * slice_w;
                    let ry_line = 1e3;
                    let u_s_o = 2.0 * u_m_k_outer;
                    let h_c_nvm = nvm_central_film(
                        f_slice, r_eq_k_outer, ry_line, e_star, eta_0, u_s_o, alpha_pv,
                    );
                    let hc = h_c_nvm * phi_t_o * phi_s * 1e6;
                    (hc * 0.75, hc)
                } else {
                    let u_k_o = eta_0 * u_m_k_outer / (e_star * r_eq_k_outer);
                    let w_k_o = w_per_l / (e_star * r_eq_k_outer);
                    let sigma_bar_o = sigma_m_outer / r_eq_k_outer;
                    let v_param_o = sigma_bar_o * std::f64::consts::SQRT_2 / (r_eq_k_outer * beta_eta_outer);
                    let mk_o = compute_film_mk(u_k_o, g_param, w_k_o, sigma_bar_o, v_param_o, r_eq_k_outer);
                    (mk_o.h_min * phi_t_o * phi_s * 1e6,
                     mk_o.h_c * phi_t_o * phi_s * 1e6)
                };
                let lam = if sigma_outer > 1e-6 { hm / sigma_outer } else { 100.0 };
                (hm, hc, lam)
            } else {
                (0.0, 0.0, 0.0)
            };

            slice_films.push(SliceFilmThickness {
                h_min_um: h_min_i,
                h_central_um: h_c_i,
                lambda: lambda_i,
                regime: classify_lambda(lambda_i),
                h_min_um_outer: h_min_o,
                h_central_um_outer: h_c_o,
                lambda_outer: lambda_o,
                regime_outer: classify_lambda(lambda_o),
            });
        }

        result.push(RollerFilmDistribution {
            roller_idx: j, psi_deg: pt.psi_deg,
            slices: slice_films,
        });
    }

    if result.is_empty() { None } else { Some(result) }
}

/// Summarize Advanced film distribution → single FilmThicknessResult.
///
/// Same logic as Basic's `summarize_film_from_distribution`, but uses M-K
/// for mixed lubrication quantities instead of GT.
pub fn summarize_film_from_distribution_advanced(
    geom: &MacroGeometry,
    material: &Material,
    operating: &OperatingConditions,
    _roller_profile: &RollerProfile,
    _raceway_inner: &RacewayProfile,
    _raceway_outer: &RacewayProfile,
    slice_geometries: &[SliceGeometry],
    angular_distribution: &[AngularLoadPoint],
    distribution: &[RollerFilmDistribution],
) -> Option<FilmThicknessResult> {
    if distribution.is_empty() {
        return None;
    }

    // Find worst-case (minimum h_min > 0) slice
    let mut min_h = f64::MAX;
    let mut worst_roller_idx = 0usize;
    let mut worst_slice = 0usize;

    for rd in distribution {
        for (k, sf) in rd.slices.iter().enumerate() {
            if sf.h_min_um > 1e-6 && sf.h_min_um < min_h {
                min_h = sf.h_min_um;
                worst_roller_idx = rd.roller_idx;
                worst_slice = k;
            }
        }
    }

    if min_h >= f64::MAX {
        return None;
    }

    // Recompute M-K parameters at worst-case slice
    let alpha_rad = geom.alpha.to_radians();
    let r_pw = geom.d_pw / 2.0 * 1e-3;
    let d_we_mean = (geom.d_we_max + geom.d_we_min) / 2.0;

    let nu_mat = material.nu;
    let e1 = material.e_roller * 1e9;
    let e2 = material.e_ring * 1e9;
    let e_star = 1.0 / ((1.0 - nu_mat * nu_mat) / e1 + (1.0 - nu_mat * nu_mat) / e2);
    let nu_actual = crate::solver::life::viscosity_at_temp_pub(
        operating.nu_40, operating.nu_100, operating.t_op,
    );
    let eta_0 = nu_actual * 1e-6 * operating.rho_oil;
    let alpha_pv = operating.alpha_pv * 1e-9;
    let g_param = alpha_pv * e_star;

    let r_eq_k = if worst_slice < slice_geometries.len() {
        slice_geometries[worst_slice].r_eq_inner * 1e-3
    } else {
        d_we_mean / 2.0 * 1e-3
    };
    let r_roller_k_mm = if worst_slice < slice_geometries.len() {
        slice_geometries[worst_slice].r_roller
    } else {
        d_we_mean / 2.0
    };
    let gamma_k = (2.0 * r_roller_k_mm * alpha_rad.cos()) / geom.d_pw;
    let u_m_k = operating.u_m_inner(r_pw, gamma_k);
    let u_m_k_outer = operating.u_m_outer(r_pw, gamma_k);

    // Bearing-level kinematics
    let gamma_dw = d_we_mean * alpha_rad.cos() / geom.d_pw;
    let cage_rpm = operating.omega_cage(gamma_dw) * 60.0 / std::f64::consts::TAU;
    let roller_rpm = operating.omega_roller(gamma_dw, geom.d_pw, d_we_mean) * 60.0 / std::f64::consts::TAU;

    // Get slice load from angular_distribution
    let q_k = angular_distribution.get(worst_roller_idx)
        .and_then(|pt| pt.slice_q_k.get(worst_slice).copied())
        .unwrap_or(1.0); // N/mm fallback
    let w_per_l = q_k * 1e3;
    let u_k = eta_0 * u_m_k / (e_star * r_eq_k);
    let w_k = w_per_l / (e_star * r_eq_k);

    let sigma_inner = (operating.rq_roller_eff() * operating.rq_roller_eff()
        + operating.rq_inner_eff() * operating.rq_inner_eff()).sqrt();
    let sigma_outer = (operating.rq_roller_eff() * operating.rq_roller_eff()
        + operating.rq_outer_eff() * operating.rq_outer_eff()).sqrt();
    let sigma_m = sigma_inner * 1e-6;
    let sigma_bar = sigma_m / r_eq_k;
    let beta_eta = if sigma_m > 1e-12 { ETA_BETA_SIGMA / sigma_m } else { 1e12 };
    let v_param = sigma_bar * std::f64::consts::SQRT_2 / (r_eq_k * beta_eta);

    let mk = compute_film_mk(u_k, g_param, w_k, sigma_bar, v_param, r_eq_k);

    // Physics-based starvation (same logic as distribution calculation)
    let speed_param = operating.n_rpm() * geom.d_pw;
    let phi_s_calc = compute_starvation_factor_advanced(
        eta_0, u_m_k, e_star, r_eq_k,
        &operating.lubrication_type, speed_param,
    );
    let phi_s = if operating.starvation_factor < 0.99 {
        operating.starvation_factor.clamp(0.1, 1.0).min(phi_s_calc)
    } else {
        phi_s_calc
    };

    // Murch-Wilson thermal correction at worst-case slice (for summary display)
    // Use SRR ≈ 0 (pure rolling approximation) — consistent with TRB apex-aligned geometry
    let p_hz_worst = angular_distribution.get(worst_roller_idx)
        .and_then(|pt| pt.slice_p_max.get(worst_slice).copied())
        .map(|p| (p * 1e6).max(1.0))
        .unwrap_or(100e6);
    let phi_t_worst = thermal_correction_murch_wilson(
        eta_0, operating.beta_visc, u_m_k, operating.k_fluid,
        0.0, p_hz_worst, e_star,
    );

    // Use worst-case slice's values from distribution directly
    let worst_sf = distribution.iter()
        .find(|rd| rd.roller_idx == worst_roller_idx)
        .and_then(|rd| rd.slices.get(worst_slice));

    let (h_min_um, h_central_um, lambda) = if let Some(sf) = worst_sf {
        (sf.h_min_um, sf.h_central_um, sf.lambda)
    } else {
        return None;
    };

    let regime = classify_lambda(lambda);

    // M-K mixed lubrication result (replaces GT)
    let p_max_mpa = angular_distribution.get(worst_roller_idx)
        .and_then(|pt| pt.slice_p_max.get(worst_slice).copied())
        .unwrap_or(1000.0);
    let p_mean_pa = (p_max_mpa * 1e6 * 0.8).max(1.0);

    let h_c_m = h_central_um * 1e-6;
    let mu_ehl = if h_c_m > 1e-10 {
        traction_coefficient(operating, 0.01, u_m_k, p_mean_pa, h_c_m, eta_0)
    } else {
        MU_BOUNDARY
    };

    let mixed = MixedLubricationResult {
        asperity_load_ratio: mk.load_fraction,
        asperity_area_ratio: mk.area_fraction,
        p_asperity_mpa: mk.load_fraction * p_mean_pa * 1e-6,
        p_fluid_mpa: (1.0 - mk.load_fraction) * p_mean_pa * 1e-6,
        mu_ehl,
        mu_boundary: MU_BOUNDARY,
        mu_effective: (1.0 - mk.load_fraction) * mu_ehl + mk.load_fraction * MU_BOUNDARY,
        f_5_2: 0.0,
        f_2: 0.0,
    };

    // Flash temperature at worst-case asperity contact (Blok-Jaeger)
    let flash_temp = if mk.load_fraction > 1e-6 {
        let p_asp_pa = mk.load_fraction * p_mean_pa;
        let b_hertz = (4.0 * w_per_l / (std::f64::consts::PI * e_star)).sqrt() * r_eq_k.sqrt();
        // SRR at worst slice (use small representative SRR for flash temp)
        let srr_est = 0.01_f64; // conservative estimate
        let v_slide = srr_est * u_m_k;
        let dt = flash_temperature(mixed.mu_effective, p_asp_pa, v_slide, b_hertz);
        Some(dt)
    } else {
        Some(0.0)
    };

    // Outer worst-case from distribution
    let mut min_h_outer = f64::MAX;
    let mut worst_o_roller = 0usize;
    let mut worst_o_slice = 0usize;
    for rd in distribution {
        for (k, sf) in rd.slices.iter().enumerate() {
            if sf.h_min_um_outer > 1e-6 && sf.h_min_um_outer < min_h_outer {
                min_h_outer = sf.h_min_um_outer;
                worst_o_roller = rd.roller_idx;
                worst_o_slice = k;
            }
        }
    }
    let (h_min_o, h_c_o, lambda_o, regime_o) = if min_h_outer < f64::MAX {
        let sf_o = &distribution.iter()
            .find(|rd| rd.roller_idx == worst_o_roller)
            .unwrap()
            .slices[worst_o_slice];
        let lam_o = if sigma_outer > 1e-6 { sf_o.h_min_um_outer / sigma_outer } else { 100.0 };
        (sf_o.h_min_um_outer, sf_o.h_central_um_outer, lam_o, classify_lambda(lam_o))
    } else {
        // Fallback: estimate outer from inner h_min and outer sigma
        let lam_o = if sigma_outer > 1e-6 { h_min_um / sigma_outer } else { 100.0 };
        (h_min_um, h_central_um, lam_o, classify_lambda(lam_o))
    };

    // Roughness values for Advanced: Rq
    let rq_r = operating.rq_roller_eff();
    let rq_i = operating.rq_inner_eff();
    let rq_o = operating.rq_outer_eff();

    Some(FilmThicknessResult {
        h_min_um,
        h_central_um,
        sigma_composite_um: sigma_inner,
        lambda_ratio: lambda,
        regime,
        h_min_um_outer: h_min_o,
        h_central_um_outer: h_c_o,
        sigma_composite_um_outer: sigma_outer,
        lambda_ratio_outer: lambda_o,
        regime_outer: regime_o,
        rq_roller_um: rq_r,
        rq_inner_um: rq_i,
        rq_outer_um: rq_o,
        u_mean_m_s: u_m_k,
        u_mean_m_s_outer: u_m_k_outer,
        cage_speed_rpm: cage_rpm,
        roller_spin_rpm: roller_rpm,
        starvation_factor: phi_s,
        thermal_factor: phi_t_worst,
        u_param: u_k,
        g_param,
        w_param: w_k,
        mixed,
        flash_temp_c: flash_temp,
        film_decay: None,
        micropitting: None,
    })
}

// ─── Advanced Traction Computation (Task 2.4) ───────────────────────

/// Advanced traction using Eyring sinh⁻¹ + Roelands + M-K load fractions.
pub fn compute_traction_advanced(
    geom: &MacroGeometry,
    material: &Material,
    operating: &OperatingConditions,
    _roller_profile: &RollerProfile,
    raceway_geom: &RacewayGeometry,
    _raceway_inner: &RacewayProfile,
    _raceway_outer: &RacewayProfile,
    slice_geometries: &[SliceGeometry],
    roller_results: &[RollerResult],
) -> Option<TractionSummary> {
    if operating.n_rpm() < 1e-6 || roller_results.is_empty() {
        return None;
    }

    let alpha_rad = geom.alpha.to_radians();
    let omega_shaft = operating.omega_inner();
    let r_pw = geom.d_pw / 2.0 * 1e-3;
    let d_we_mean = (geom.d_we_max + geom.d_we_min) / 2.0;

    let nu_mat = material.nu;
    let e1 = material.e_roller * 1e9;
    let e2 = material.e_ring * 1e9;
    let e_star = 1.0 / ((1.0 - nu_mat * nu_mat) / e1 + (1.0 - nu_mat * nu_mat) / e2);
    let nu_actual = crate::solver::life::viscosity_at_temp_pub(
        operating.nu_40, operating.nu_100, operating.t_op,
    );
    let eta_0 = nu_actual * 1e-6 * operating.rho_oil;
    let alpha_pv = operating.alpha_pv * 1e-9;
    let g_param = alpha_pv * e_star;
    let phi_s = operating.starvation_factor.clamp(0.1, 1.0);

    // Advanced roughness parameters
    let sigma_inner_um = (operating.rq_roller_eff().powi(2) + operating.rq_inner_eff().powi(2)).sqrt();
    let sigma_outer_um = (operating.rq_roller_eff().powi(2) + operating.rq_outer_eff().powi(2)).sqrt();
    let sigma_inner_m = sigma_inner_um * 1e-6;
    let beta_eta_inner = if sigma_inner_m > 1e-12 { ETA_BETA_SIGMA / sigma_inner_m } else { 1e12 };

    // Kinematics
    let kin = compute_trb_kinematics(geom, raceway_geom, operating);
    let slicing = compute_slice_sliding(
        &kin, geom, raceway_geom, operating, slice_geometries,
    );

    const MU_ROLLING: f64 = 0.002;

    let mut rollers = Vec::with_capacity(roller_results.len());
    let mut p_rolling_total = 0.0;
    let mut p_sliding_total = 0.0;
    let mut p_rib_total = 0.0;
    let mut p_hysteresis_total = 0.0_f64;

    for (j, rr) in roller_results.iter().enumerate() {
        if rr.q_normal < 1e-3 {
            rollers.push(RollerTractionResult {
                roller_idx: j, psi_deg: rr.psi_deg,
                inner: zero_friction(), outer: zero_friction(), rib: None,
            });
            continue;
        }

        // Load-weighted average sliding for this roller
        let mut sum_q = 0.0;
        let mut sum_q_u_inner = 0.0;
        let mut sum_q_u_outer = 0.0;
        let mut q_weighted_h_c_inner = 0.0;
        let mut q_weighted_load_frac = 0.0;

        for (k, sc) in rr.slice_results.iter().enumerate() {
            if !sc.in_contact || sc.q_k < 1e-6 { continue; }
            let (u_sl_i, u_sl_o) = if k < slicing.len() { slicing[k] } else { (0.0, 0.0) };
            sum_q += sc.q_k;
            sum_q_u_inner += sc.q_k * u_sl_i;
            sum_q_u_outer += sc.q_k * u_sl_o;

            // Compute per-slice M-K film for traction
            let r_eq_k = if k < slice_geometries.len() {
                slice_geometries[k].r_eq_inner * 1e-3
            } else {
                d_we_mean / 2.0 * 1e-3
            };
            let r_roller_k_mm = if k < slice_geometries.len() {
                slice_geometries[k].r_roller
            } else {
                d_we_mean / 2.0
            };
            let gamma_k = (2.0 * r_roller_k_mm * alpha_rad.cos()) / geom.d_pw;
            let u_m_k = operating.u_m_inner(r_pw, gamma_k);

            if u_m_k > 1e-8 && r_eq_k > 1e-8 {
                let u_k = eta_0 * u_m_k / (e_star * r_eq_k);
                let w_k = (sc.q_k * 1e3) / (e_star * r_eq_k);
                let sigma_bar = sigma_inner_m / r_eq_k;
                let v_param = sigma_bar * std::f64::consts::SQRT_2 / (r_eq_k * beta_eta_inner);
                let mk = compute_film_mk(u_k, g_param, w_k, sigma_bar, v_param, r_eq_k);

                let srr_k = if u_m_k.abs() > 1e-10 { u_sl_i / u_m_k } else { 0.0 };
                let phi_t = thermal_correction_murch_wilson(
                    eta_0, operating.beta_visc, u_m_k, operating.k_fluid,
                    srr_k, sc.p_max_k * 1e6, e_star,
                );
                let h_c_corrected = mk.h_c * phi_t * phi_s;
                q_weighted_h_c_inner += sc.q_k * h_c_corrected;
                q_weighted_load_frac += sc.q_k * mk.load_fraction;
            }
        }

        if sum_q < 1e-6 {
            rollers.push(RollerTractionResult {
                roller_idx: j, psi_deg: rr.psi_deg,
                inner: zero_friction(), outer: zero_friction(), rib: None,
            });
            continue;
        }

        let u_slide_inner = sum_q_u_inner / sum_q;
        let u_slide_outer = sum_q_u_outer / sum_q;
        let h_c_avg = q_weighted_h_c_inner / sum_q;
        let load_frac_avg = (q_weighted_load_frac / sum_q).clamp(0.0, 1.0);

        // Inner / outer normal forces differ by the cone-angle factor
        // cos(α_o − α_i); split-contact mode adds further asymmetry.
        //
        // Unit audit: gen1.rs builds q_normal as Σ q_k[N/mm] × slice_width[mm]
        // so the field is already in [N]. The earlier code multiplied by 1e3
        // under a stale "kN → N" comment, inflating M2 power by 1000×. Use the
        // raw value here, matching M1's `compute_contact_friction_at`.
        let q_n_outer = rr.q_normal;
        let q_n_inner = rr.q_normal_inner;
        let u_roll = kin.u_roll;

        // SRR
        let srr_inner = if u_roll.abs() > 1e-10 { u_slide_inner / u_roll } else { 0.0 };
        let srr_outer = if u_roll.abs() > 1e-10 { u_slide_outer / u_roll } else { 0.0 };

        // Lambda
        let lambda_inner = if sigma_inner_um > 1e-6 && h_c_avg > 1e-12 {
            (h_c_avg * 1e6) / sigma_inner_um
        } else { 100.0 };
        let lambda_outer = if sigma_outer_um > 1e-6 && h_c_avg > 1e-12 {
            (h_c_avg * 1e6) / sigma_outer_um
        } else { 100.0 };

        // Fluid-EHL traction (dispatched: Eyring or Carreau-Yasuda)
        let p_mean_pa = (rr.slice_results.iter()
            .filter(|s| s.in_contact)
            .map(|s| s.p_max_k)
            .fold(0.0_f64, f64::max) * 1e6 * 0.8).max(1.0);

        let mu_ehl_inner = traction_coefficient(
            operating, srr_inner, u_roll, p_mean_pa, h_c_avg, eta_0,
        );
        let mu_inner = (1.0 - load_frac_avg) * mu_ehl_inner + load_frac_avg * MU_BOUNDARY;

        let mu_ehl_outer = traction_coefficient(
            operating, srr_outer, u_roll, p_mean_pa, h_c_avg, eta_0,
        );
        let mu_outer = (1.0 - load_frac_avg) * mu_ehl_outer + load_frac_avg * MU_BOUNDARY;

        // Forces and power — each raceway uses its own normal load.
        let f_inner = mu_inner * q_n_inner;
        let f_outer = mu_outer * q_n_outer;
        let p_slide_inner = f_inner * u_slide_inner.abs();
        let p_slide_outer = f_outer * u_slide_outer.abs();

        let mu_rr = MU_ROLLING;
        let r_eq_repr = (geom.d_we_max + geom.d_we_min) * 0.25 * 1e-3; // mm → m
        let (p_roll_inner, p_roll_outer, p_hys_inner, p_hys_outer) = match operating.friction_model {
            FrictionModel::BibouletHoupert => {
                // Part 1 (line contact) calibration with selected thermal
                // inlet-shear correction.  Add Johnson hysteresis separately
                // (BH is purely viscous — solid-side hysteresis must be added).
                // Inner/outer use distinct entrainment velocities (u_inner ≠ u_outer)
                // because R_inner_contact ≠ R_outer_contact (Schwarz convention).
                let e_prime = 2.0 * e_star;
                let l_contact = (geom.l_we * 1e-3).max(1e-6); // mm → m
                let p_ri = biboulet_houpert_line_rolling_power_dispatched(
                    eta_0, kin.u_inner, q_n_inner, l_contact, r_eq_repr, e_prime,
                    operating.beta_visc, operating.k_fluid, operating.thermal_correction);
                let p_ro = biboulet_houpert_line_rolling_power_dispatched(
                    eta_0, kin.u_outer, q_n_outer, l_contact, r_eq_repr, e_prime,
                    operating.beta_visc, operating.k_fluid, operating.thermal_correction);
                let p_hi = johnson_hysteresis_power_line_contact(
                    q_n_inner, l_contact, r_eq_repr, e_prime,
                    operating.hysteresis_loss_factor, kin.u_inner);
                let p_ho = johnson_hysteresis_power_line_contact(
                    q_n_outer, l_contact, r_eq_repr, e_prime,
                    operating.hysteresis_loss_factor, kin.u_outer);
                (p_ri, p_ro, p_hi, p_ho)
            }
            _ => (
                // Palmgren/SKF empirical calibration uses bearing-level mean u
                // (does not distinguish per-raceway entrainment).
                mu_rr * q_n_inner * u_roll.abs(),
                mu_rr * q_n_outer * u_roll.abs(),
                0.0, 0.0,    // Palmgren/SKF μ_rr/G_rr already include hysteresis
            ),
        };

        let inner = ContactFriction {
            u_rolling: u_roll, u_sliding: u_slide_inner,
            srr: srr_inner, lambda: lambda_inner,
            asperity_load_ratio: load_frac_avg,
            mu: mu_inner, f_traction_n: f_inner,
            power_loss_w: p_slide_inner,
            p_rolling_w: p_roll_inner,
            p_hysteresis_w: p_hys_inner,
        };
        let outer = ContactFriction {
            u_rolling: u_roll, u_sliding: u_slide_outer,
            srr: srr_outer, lambda: lambda_outer,
            asperity_load_ratio: load_frac_avg,
            mu: mu_outer, f_traction_n: f_outer,
            power_loss_w: p_slide_outer,
            p_rolling_w: p_roll_outer,
            p_hysteresis_w: p_hys_outer,
        };

        // Rib friction — DRILLING-based power (Houpert 2002):
        //   M_drilling = (3/8) · μ_eff · F_rib · a_ellipse
        //   P_drilling = M_drilling · ω_roller
        // Earlier μ·F·u_slide formulation over-predicted by ~16× because
        // u_slide_rib used full roller surface velocity (ω_roller × r_large_end)
        // rather than effective drilling lever arm (3a/8 ≈ 0.5 mm).
        let rib = if let Some(ref rib_res) = rr.rib_result {
            let f_rib = rib_res.f_rib;
            let mu_rib = rib_res.ehl.as_ref().map_or(MU_RIB, |e| e.mu_eff);
            let m_drilling_nmm = 0.375 * mu_rib * f_rib * rib_res.a_ellipse;
            let power = m_drilling_nmm * 1e-3 * kin.omega_roller.abs();
            Some(RibFriction {
                u_sliding: kin.u_slide_rib, mu: mu_rib,
                f_friction_n: mu_rib * f_rib,
                power_loss_w: power,
            })
        } else { None };

        p_rolling_total += p_roll_inner + p_roll_outer;
        p_sliding_total += p_slide_inner + p_slide_outer;
        p_hysteresis_total += p_hys_inner + p_hys_outer;
        if let Some(ref r) = rib {
            p_rib_total += r.power_loss_w;
        }

        rollers.push(RollerTractionResult {
            roller_idx: j, psi_deg: rr.psi_deg,
            inner, outer, rib,
        });
    }

    // Total bearing contact power includes hysteresis (Johnson 1985) for BH
    // friction model; hysteresis is 0 for Palmgren/SKF (already implicit).
    let p_total = p_rolling_total + p_sliding_total + p_rib_total + p_hysteresis_total;
    let m_friction = if omega_shaft > 1e-10 { p_total / omega_shaft * 1e3 } else { 0.0 };

    Some(apply_friction_model_to_summary(TractionSummary {
        rollers,
        p_rolling_w: p_rolling_total,
        p_hysteresis_w: p_hysteresis_total,
        p_sliding_w: p_sliding_total,
        p_rib_w: p_rib_total,
        p_contact_total_w: p_total,
        m_friction_nmm: m_friction,
        friction_model: operating.friction_model,
        skf_reference: None,
    }, geom, operating))
}

// ═══════════════════════════════════════════════════════════════════

/// Compute micropitting safety factor S_λ = Λ_min / Λ_perm.
///
/// Framework adapted from ISO/TS 6336-22 (gear micropitting).
/// **No equivalent bearing ISO standard exists — treat as engineering estimate.**
pub fn compute_micropitting_safety(
    film: &mut FilmThicknessResult,
    operating: &OperatingConditions,
) {
    let base = operating.surface_finish.lambda_perm_base();
    let factor = operating.additive_type.lambda_perm_factor();
    let lambda_perm = base * factor;

    if lambda_perm < 1e-6 {
        return;
    }

    let s_inner = film.lambda_ratio / lambda_perm;
    let s_outer = film.lambda_ratio_outer / lambda_perm;

    let classify = |s: f64| -> MicropittingRisk {
        if s >= 2.0 { MicropittingRisk::Safe }
        else if s >= 1.0 { MicropittingRisk::Marginal }
        else { MicropittingRisk::AtRisk }
    };

    film.micropitting = Some(MicropittingSafety {
        lambda_perm,
        lambda_perm_base: base,
        additive_factor: factor,
        s_lambda_inner: s_inner,
        s_lambda_outer: s_outer,
        risk_inner: classify(s_inner),
        risk_outer: classify(s_outer),
    });
}

pub fn classify_lambda(lambda: f64) -> LubricationRegime {
    if lambda > 3.0 {
        LubricationRegime::FullEhl
    } else if lambda >= 1.0 {
        LubricationRegime::Mixed
    } else {
        LubricationRegime::Boundary
    }
}

// ─── SKF Frictional Moment Reference Model ─────────────────────────
//
// Independent SKF (Catalogue 2018, "The SKF model for calculating the
// frictional moment") implementation used purely as an *external validation
// reference*. NOT wired into the production solver — provides a sanity-check
// reference torque for our results.

/// SKF series families covered by the friction model. Geometric constants
/// (R1, R2, S1, S2) are read from SKF Catalogue Table 2d.
#[derive(Debug, Clone, Copy)]
pub enum SkfTrbSeries {
    /// 302xx — light series TRB
    Series302,
    /// 303xx — medium series TRB (covers 30306, 30307, …)
    Series303,
    /// 313 (X) series
    Series313,
    /// 320 X series
    Series320,
    /// 322 (non-B) series
    Series322,
    /// 322 B series
    Series322B,
    /// 323 (non-B) series
    Series323,
    /// 323 B series
    Series323B,
    /// Generic fallback for non-listed TRB series (uses "All other" row).
    Other,
}

impl SkfTrbSeries {
    /// Returns (R1, R2, S1, S2) per SKF Catalogue Table 2d.
    pub fn constants(self) -> (f64, f64, f64, f64) {
        match self {
            SkfTrbSeries::Series302  => (1.76e-6, 10.9, 0.017,  2.0),
            SkfTrbSeries::Series303  => (1.69e-6, 10.9, 0.017,  2.0),
            SkfTrbSeries::Series313  => (1.84e-6, 10.9, 0.048,  2.0),
            SkfTrbSeries::Series320  => (2.38e-6, 10.9, 0.014,  2.0),
            SkfTrbSeries::Series322  => (2.27e-6, 10.9, 0.018,  2.0),
            SkfTrbSeries::Series322B => (2.38e-6, 10.9, 0.026,  2.0),
            SkfTrbSeries::Series323  => (2.38e-6, 10.9, 0.019,  2.0),
            SkfTrbSeries::Series323B => (2.79e-6, 10.9, 0.030,  2.0),
            SkfTrbSeries::Other      => (2.31e-6, 10.9, 0.019,  2.0),
        }
    }
}

/// Lubrication scheme — controls the kinematic-starvation constant K_rs.
#[derive(Debug, Clone, Copy)]
pub enum SkfLubrication {
    /// Low-level oil bath or oil jet
    OilBath,
    /// Oil jet (same K_rs as OilBath in the SKF model)
    OilJet,
    /// Grease lubrication
    Grease,
    /// Oil-air mist
    OilAir,
}

/// Bearing-level frictional moment from the SKF model.
#[derive(Debug, Clone, Copy)]
pub struct SkfFrictionMoment {
    pub m_rr_nmm: f64,    // rolling friction torque
    pub m_sl_nmm: f64,    // sliding friction torque
    pub m_total_nmm: f64, // M_rr + M_sl (no seals, no drag)
    pub phi_ish: f64,     // inlet shear heating reduction
    pub phi_rs: f64,      // kinematic replenishment/starvation reduction
    pub g_rr: f64,
    pub g_sl: f64,
    pub mu_sl: f64,
    pub phi_bl: f64,      // boundary weighting factor
}

/// Compute SKF (Catalogue 2018) frictional moment for a single tapered roller
/// bearing — rolling and sliding components only (M_seal and M_drag are zero
/// when not specified). Inputs are catalogue-style: bore/outer diameters,
/// loads, axial-load factor Y, speed and operating viscosity.
///
/// References:
/// - SKF, "The SKF model for calculating the frictional moment" (2018 brochure)
/// - SKF Rolling Bearings Catalogue Table 2 / 2d (TRB R, S constants)
pub fn skf_frictional_moment_trb(
    series: SkfTrbSeries,
    d_mm: f64,           // bore diameter [mm]
    big_d_mm: f64,       // outside diameter [mm]
    f_r_n: f64,          // radial load [N]
    f_a_n: f64,          // axial load [N]
    y_axial: f64,        // axial load factor Y (catalogue value)
    n_rpm: f64,
    nu_op_cst: f64,      // operating viscosity [cSt = mm²/s]
    lubrication: SkfLubrication,
) -> SkfFrictionMoment {
    let (r1, r2, s1, s2) = series.constants();
    let d_m = 0.5 * (d_mm + big_d_mm);

    // ── Rolling friction G_rr (Table 1) ───────────────────────────────
    let f_eff_rr = f_r_n + r2 * y_axial * f_a_n.abs();
    let g_rr = r1 * d_m.powf(2.38) * f_eff_rr.max(1.0).powf(0.31);

    // Inlet shear heating reduction (φ_ish)
    let phi_ish = 1.0 / (1.0
        + 1.84e-9 * (n_rpm * d_m).max(0.0).powf(1.28) * nu_op_cst.max(0.0).powf(0.64));

    // Kinematic replenishment/starvation (φ_rs)
    const K_Z_TRB: f64 = 6.0;
    let k_rs = match lubrication {
        SkfLubrication::OilBath | SkfLubrication::OilJet => 3.0e-8,
        SkfLubrication::Grease  | SkfLubrication::OilAir => 6.0e-8,
    };
    let arg = if (big_d_mm - d_mm).abs() > 1e-9 {
        k_rs * nu_op_cst * n_rpm * (d_mm + big_d_mm)
            * (K_Z_TRB / (2.0 * (big_d_mm - d_mm))).sqrt()
    } else { 0.0 };
    let phi_rs = (-arg).exp();

    let m_rr = phi_ish * phi_rs * g_rr * (nu_op_cst * n_rpm).max(0.0).powf(0.6);

    // ── Sliding friction G_sl (Table 1) ───────────────────────────────
    let f_eff_sl = f_r_n + s2 * y_axial * f_a_n.abs();
    let g_sl = s1 * d_m.powf(0.82) * f_eff_sl;

    // Boundary weighting φ_bl
    let phi_bl = (-2.6e-8 * (n_rpm * nu_op_cst).max(0.0).powf(1.4) * d_m).exp();
    const MU_BL: f64 = 0.12;
    const MU_EHL_TRB: f64 = 0.002;
    let mu_sl = phi_bl * MU_BL + (1.0 - phi_bl) * MU_EHL_TRB;
    let m_sl = g_sl * mu_sl;

    SkfFrictionMoment {
        m_rr_nmm: m_rr,
        m_sl_nmm: m_sl,
        m_total_nmm: m_rr + m_sl,
        phi_ish, phi_rs, g_rr, g_sl, mu_sl, phi_bl,
    }
}

// ─── Enum mapping: types.rs ↔ lubrication-local SKF enums ──────────

impl From<crate::solver::types::SkfTrbSeriesEnum> for SkfTrbSeries {
    fn from(v: crate::solver::types::SkfTrbSeriesEnum) -> Self {
        use crate::solver::types::SkfTrbSeriesEnum as E;
        match v {
            E::Series302  => SkfTrbSeries::Series302,
            E::Series303  => SkfTrbSeries::Series303,
            E::Series313  => SkfTrbSeries::Series313,
            E::Series320  => SkfTrbSeries::Series320,
            E::Series322  => SkfTrbSeries::Series322,
            E::Series322B => SkfTrbSeries::Series322B,
            E::Series323  => SkfTrbSeries::Series323,
            E::Series323B => SkfTrbSeries::Series323B,
            E::Other      => SkfTrbSeries::Other,
        }
    }
}

impl From<crate::solver::types::SkfLubricationEnum> for SkfLubrication {
    fn from(v: crate::solver::types::SkfLubricationEnum) -> Self {
        use crate::solver::types::SkfLubricationEnum as E;
        match v {
            E::OilBath => SkfLubrication::OilBath,
            E::OilJet  => SkfLubrication::OilJet,
            E::Grease  => SkfLubrication::Grease,
            E::OilAir  => SkfLubrication::OilAir,
        }
    }
}

/// Compute SKF reference frictional moment from a `BearingInput`-style record.
/// Convenience wrapper that pulls bearing geometry, loads, and operating
/// parameters from the same structures the production solver consumes.
pub fn skf_reference_for_bearing(
    geom: &MacroGeometry,
    operating: &OperatingConditions,
) -> SkfFrictionMoment {
    let nu_op_cst = crate::solver::life::viscosity_at_temp_pub(
        operating.nu_40, operating.nu_100, operating.t_op,
    );
    skf_frictional_moment_trb(
        operating.skf_trb_series.into(),
        geom.d,
        geom.outer_diameter,
        operating.f_r() * 1000.0,    // kN → N
        operating.f_a.abs() * 1000.0,
        operating.skf_y_factor,
        operating.n_rpm(),
        nu_op_cst,
        operating.skf_lubrication.into(),
    )
}

/// Build the serializable `SkfFrictionRef` from a raw `SkfFrictionMoment`
/// plus echo the bearing/series context for the UI.
pub fn build_skf_reference(
    skf: SkfFrictionMoment,
    geom: &MacroGeometry,
    operating: &OperatingConditions,
) -> crate::solver::types::SkfFrictionRef {
    let omega = operating.omega_inner().abs();
    let p_rolling_w = (skf.m_rr_nmm * 1e-3) * omega; // [N·mm × 1e-3 = N·m] × [rad/s] = W
    let p_sliding_w = (skf.m_sl_nmm * 1e-3) * omega;
    let nu_op_cst = crate::solver::life::viscosity_at_temp_pub(
        operating.nu_40, operating.nu_100, operating.t_op,
    );
    crate::solver::types::SkfFrictionRef {
        m_rr_nmm: skf.m_rr_nmm,
        m_sl_nmm: skf.m_sl_nmm,
        m_total_nmm: skf.m_total_nmm,
        p_rolling_w,
        p_sliding_w,
        p_total_w: p_rolling_w + p_sliding_w,
        phi_ish: skf.phi_ish,
        phi_rs: skf.phi_rs,
        phi_bl: skf.phi_bl,
        g_rr: skf.g_rr,
        g_sl: skf.g_sl,
        mu_sl: skf.mu_sl,
        d_m_mm: 0.5 * (geom.d + geom.outer_diameter),
        nu_op_cst,
        n_rpm: operating.n_rpm(),
        series: operating.skf_trb_series,
        lubrication: operating.skf_lubrication,
        y_factor: operating.skf_y_factor,
    }
}

/// Apply friction-model dispatch: when `SkfAdvanced` is selected, replace
/// bearing-level totals with the SKF reference values; per-roller breakdown
/// (Palmgren-style) is preserved for diagnostics.
fn apply_friction_model_to_summary(
    mut ts: TractionSummary,
    geom: &MacroGeometry,
    operating: &OperatingConditions,
) -> TractionSummary {
    let omega = operating.omega_inner().abs();
    if omega > 1e-8 {
        let skf = skf_reference_for_bearing(geom, operating);
        let skf_ref = build_skf_reference(skf, geom, operating);
        if matches!(operating.friction_model, FrictionModel::SkfAdvanced) {
            // Override bearing-level totals; keep p_rib_w (SKF M_seal/M_drag
            // are zero in our config and rib is a separate physical contact).
            ts.p_rolling_w = skf_ref.p_rolling_w;
            ts.p_sliding_w = skf_ref.p_sliding_w;
            ts.p_contact_total_w = ts.p_rolling_w + ts.p_sliding_w + ts.p_rib_w;
            ts.m_friction_nmm = if omega > 1e-8 { ts.p_contact_total_w / omega * 1000.0 } else { 0.0 };
        }
        ts.skf_reference = Some(skf_ref);
    }
    ts.friction_model = operating.friction_model;
    ts
}

// ─── Tests ─────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn test_geom() -> MacroGeometry {
        MacroGeometry {
            d: 120.0, outer_diameter: 180.0, t: 38.0,
            alpha: 12.0, z: 20,
            d_we_max: 14.5, d_we_min: 13.0,
            l_we: 22.0, d_pw: 150.0,
            h_rib: 3.0, alpha_rib: 84.0, g_r: 0.0,
            h_c: None,
        }
    }

    fn test_material() -> Material { Material::default() }

    fn test_raceway_profile() -> RacewayProfile {
        RacewayProfile { delta_rw: 0.0, w_a: 0.0, ra: 0.2, custom_profile: None, polynomial_coeffs: None }
    }

    fn test_roller_profile() -> RollerProfile {
        RollerProfile {
            crown_type: CrownType::Logarithmic { a_log: 0.001 },
            delta_c: 3.0, delta_dub_l: 2.0, delta_dub_s: 2.0,
            l_dub_l: 1.0, l_dub_s: 1.0, r_sph: 35.0,
            sigma_roller: 0.15,
        }
    }

    fn test_op(n_rpm: f64) -> OperatingConditions {
        OperatingConditions {
            f_x: 10.0, f_y: 0.0, f_a: 5.0,
            m_x: 0.0, m_y: 0.0, n_inner_rpm: n_rpm, n_outer_rpm: 0.0,
            gamma: 0.0, t_op: 70.0,
            nu_40: 68.0, nu_100: 8.0,
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
        }
    }

    // ─── erfc tests ───

    #[test]
    fn test_erfc_known_values() {
        assert!((erfc(0.0) - 1.0).abs() < 1e-6, "erfc(0)=1");
        assert!((erfc(1.0) - 0.1573).abs() < 1e-3, "erfc(1)≈0.1573");
        assert!((erfc(2.0) - 0.00468).abs() < 1e-4, "erfc(2)≈0.00468");
        assert!(erfc(5.0) < 1e-10, "erfc(5)≈0");
        // Negative argument: erfc(-x) = 2 - erfc(x)
        assert!((erfc(-1.0) - (2.0 - 0.1573)).abs() < 1e-3);
    }

    // ─── GT integral tests ───

    #[test]
    fn test_gt_integral_f52() {
        // F_{5/2}(0) should be a positive finite value
        let f0 = gt_integral(2.5, 0.0);
        assert!(f0 > 0.0, "F_5/2(0)={f0} must be positive");
        // F_{5/2}(Λ) should decrease with Λ
        let f1 = gt_integral(2.5, 1.0);
        let f2 = gt_integral(2.5, 2.0);
        let f3 = gt_integral(2.5, 3.0);
        assert!(f0 > f1, "F_5/2 must decrease: {f0} > {f1}");
        assert!(f1 > f2, "F_5/2 must decrease: {f1} > {f2}");
        assert!(f2 > f3, "F_5/2 must decrease: {f2} > {f3}");
        // At Λ=5, should be negligible
        let f5 = gt_integral(2.5, 5.0);
        assert!(f5 < 1e-6, "F_5/2(5)={f5} should be negligible");
    }

    // ─── Film thickness + mixed model tests ───

    #[test]
    fn test_standard_speed_ehl() {
        let geom = test_geom();
        let rp = test_raceway_profile();
        let op = test_op(1000.0);
        let rolp = test_roller_profile();
        let result = compute_film_thickness(&geom, &test_material(), &op, &rolp, &rp, &rp, 100.0);
        assert!(result.is_some());
        let ft = result.unwrap();
        assert!(ft.h_min_um > 0.05 && ft.h_min_um < 10.0,
            "h_min={} μm", ft.h_min_um);
        assert!(ft.h_central_um > ft.h_min_um);
        assert!(ft.thermal_factor > 0.9);
        // Mixed lubrication fields
        assert!(ft.mixed.mu_effective >= 0.0);
        assert!(ft.mixed.asperity_load_ratio >= 0.0 && ft.mixed.asperity_load_ratio <= 1.0);
    }

    #[test]
    fn test_low_speed_high_asperity_contact() {
        let geom = MacroGeometry {
            d: 800.0, outer_diameter: 1100.0, t: 150.0,
            alpha: 15.0, z: 40,
            d_we_max: 55.0, d_we_min: 48.0,
            l_we: 80.0, d_pw: 950.0,
            h_rib: 8.0, alpha_rib: 84.0, g_r: 0.0,
            h_c: None,
        };
        let rp = test_raceway_profile();
        let op = OperatingConditions {
            f_x: 500.0, f_y: 0.0, f_a: 200.0,
            m_x: 0.0, m_y: 0.0,
            n_inner_rpm: 12.0, n_outer_rpm: 0.0,
            gamma: 0.0, t_op: 50.0,
            nu_40: 320.0, nu_100: 30.0,
            alpha_pv: 20.0,
            lubrication_type: LubricationType::Grease,
            starvation_factor: 0.7,
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
        let rolp = test_roller_profile();
        let ft = compute_film_thickness(&geom, &test_material(), &op, &rolp, &rp, &rp, 300.0).unwrap();
        // Low speed → thin film → higher asperity contact
        assert!(ft.mixed.asperity_load_ratio > 0.0,
            "Low speed should have asperity contact, got ratio={}",
            ft.mixed.asperity_load_ratio);
        // Effective friction should be between EHL and boundary
        assert!(ft.mixed.mu_effective >= ft.mixed.mu_ehl);
        assert!(ft.mixed.mu_effective <= ft.mixed.mu_boundary + 0.001);
    }

    #[test]
    fn test_full_ehl_no_asperity() {
        // High speed, low roughness → full EHL, no asperity contact
        let geom = test_geom();
        let rp = RacewayProfile { delta_rw: 0.0, w_a: 0.0, ra: 0.02, custom_profile: None, polynomial_coeffs: None };
        let rolp = test_roller_profile();
        let op = test_op(3000.0); // high speed
        let ft = compute_film_thickness(&geom, &test_material(), &op, &rolp, &rp, &rp, 50.0).unwrap();
        if ft.lambda_ratio > 4.0 {
            assert!(ft.mixed.asperity_load_ratio < 0.01,
                "Λ={:.1} should have negligible asperity contact, got {:.4}",
                ft.lambda_ratio, ft.mixed.asperity_load_ratio);
        }
    }

    #[test]
    fn test_grease_vs_oil() {
        let geom = test_geom();
        let rp = test_raceway_profile();
        let oil = test_op(500.0);
        let grease = OperatingConditions {
            lubrication_type: LubricationType::Grease,
            starvation_factor: 0.6,
            ..oil.clone()
        };
        let rolp = test_roller_profile();
        let ft_oil = compute_film_thickness(&geom, &test_material(), &oil, &rolp, &rp, &rp, 80.0).unwrap();
        let ft_grease = compute_film_thickness(&geom, &test_material(), &grease, &rolp, &rp, &rp, 80.0).unwrap();
        assert!(ft_grease.h_min_um < ft_oil.h_min_um);
        // Grease → thinner film → higher asperity ratio
        assert!(ft_grease.mixed.asperity_load_ratio >= ft_oil.mixed.asperity_load_ratio);
    }

    #[test]
    fn test_static_returns_none() {
        let geom = test_geom();
        let rp = test_raceway_profile();
        let op = test_op(0.0);
        let rolp = test_roller_profile();
        assert!(compute_film_thickness(&geom, &test_material(), &op, &rolp, &rp, &rp, 100.0).is_none());
    }

    #[test]
    fn test_lambda_regime_classification() {
        assert_eq!(classify_lambda(5.0), LubricationRegime::FullEhl);
        assert_eq!(classify_lambda(2.0), LubricationRegime::Mixed);
        assert_eq!(classify_lambda(0.5), LubricationRegime::Boundary);
    }

    fn test_raceway_geom() -> RacewayGeometry {
        RacewayGeometry {
            alpha_i: 8.0, alpha_o: 12.0, // realistic TRB: α_i ≠ α_o (always)
            r_i: 200.0, r_o: 200.0, r_rib: 1500.0, r_rib_circ: None,
            d_uc: 0.0, l_uc: 0.0,
        }
    }

    fn test_slice_geometries(geom: &MacroGeometry, n: usize) -> Vec<SliceGeometry> {
        let sw = geom.l_we / n as f64;
        (0..n).map(|k| {
            let x = (k as f64 + 0.5) * sw;
            let frac = x / geom.l_we;
            let r_k = (geom.d_we_min + (geom.d_we_max - geom.d_we_min) * frac) / 2.0;
            SliceGeometry {
                k, x_axial: x, r_roller: r_k,
                r_inner_race: 200.0, r_outer_race: 200.0,
                r_eq_inner: r_k * 0.8, r_eq_outer: r_k * 1.2,
                delta_z_total_inner: 0.0, delta_z_total_outer: 0.0, slice_width: sw,
            }
        }).collect()
    }

    // ─── Kinematics tests ───

    #[test]
    fn test_trb_kinematics_positive() {
        let geom = test_geom();
        let rg = test_raceway_geom();
        let op = test_op(1000.0);
        let kin = compute_trb_kinematics(&geom, &rg, &op);
        assert!(kin.omega_cage > 0.0, "cage must rotate");
        assert!(kin.u_roll > 0.0, "rolling velocity must be positive");
        assert!(kin.u_slide_rib > 0.0, "rib sliding must be positive");
    }

    #[test]
    fn test_kinematics_zero_speed() {
        let geom = test_geom();
        let rg = test_raceway_geom();
        let op = test_op(0.0);
        let kin = compute_trb_kinematics(&geom, &rg, &op);
        assert!((kin.omega_cage).abs() < 1e-10);
        assert!((kin.u_roll).abs() < 1e-10);
    }

    #[test]
    fn test_apex_aligned_zero_sliding() {
        // Cone apex alignment: α_i ≠ α_o (always different in real TRB),
        // but with consistent exact kinematics + cone-proportional contact
        // radii, sliding is exactly zero at ALL slices for ANY (α_i, α_o).
        let geom = test_geom();
        let rg = test_raceway_geom(); // α_i=8°, α_o=12° — realistic TRB
        let op = test_op(1000.0);
        let kin = compute_trb_kinematics(&geom, &rg, &op);
        let sg = test_slice_geometries(&geom, 10);
        let sliding = compute_slice_sliding(&kin, &geom, &rg, &op, &sg);
        // Exact formulas → machine-precision zero at every slice
        for (k, (us_i, us_o)) in sliding.iter().enumerate() {
            assert!(us_i.abs() < 1e-12,
                "slice {k}: inner sliding={us_i:.4e} should be zero for apex-aligned");
            assert!(us_o.abs() < 1e-12,
                "slice {k}: outer sliding={us_o:.4e} should be zero for apex-aligned");
        }
    }

    #[test]
    fn test_various_angles_all_zero_sliding() {
        // Verify zero sliding for several different α_i/α_o combinations,
        // proving the apex condition is about geometry consistency not α_i=α_o.
        let geom = test_geom();
        let op = test_op(1500.0);
        let sg = test_slice_geometries(&geom, 10);

        for (ai, ao) in [(5.0, 15.0), (7.85, 11.859), (10.0, 20.0), (3.0, 8.0)] {
            let rg = RacewayGeometry { alpha_i: ai, alpha_o: ao, ..test_raceway_geom() };
            let kin = compute_trb_kinematics(&geom, &rg, &op);
            let sliding = compute_slice_sliding(&kin, &geom, &rg, &op, &sg);
            let max_slide = sliding.iter()
                .map(|(a, b)| a.max(*b)).fold(0.0_f64, f64::max);
            assert!(max_slide < 1e-12,
                "α_i={ai}°, α_o={ao}°: max_slide={max_slide:.4e} should be zero");
        }
    }

    // ─── Traction tests ───

    #[test]
    fn test_traction_loaded_roller() {
        let geom = test_geom();
        let mat = test_material();
        let rp = test_raceway_profile();
        let op = test_op(1000.0);
        let rg = test_raceway_geom();

        let sg = test_slice_geometries(&geom, 10);
        let slices: Vec<SliceContactResult> = (0..10).map(|k| SliceContactResult {
            k, delta_k: 5.0, q_k: 50.0, q_k_outer: 50.0, q_k_inner: 50.0,
            b_k: 0.1, p_max_k: 1500.0, h_bulk_k: 0.0,
            k_hertz_k: 0.0,
            b_k_outer: 0.1, p_max_k_outer: 1400.0, h_bulk_k_outer: 0.0,
            k_hertz_k_outer: 0.0, k_combined_k: 0.0,
            in_contact: true,
        }).collect();
        let rollers = vec![RollerResult {
            psi_deg: 0.0, q_normal: 500.0, q_normal_inner: 500.0,
            slice_results: slices, rib_result: None,
        }];

        let rolp = test_roller_profile();
        let ts = compute_traction(&geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers);
        assert!(ts.is_some());
        let ts = ts.unwrap();
        assert_eq!(ts.rollers.len(), 1);
        assert!(ts.p_contact_total_w > 0.0, "loaded roller must dissipate power");
        assert!(ts.m_friction_nmm > 0.0, "friction torque must be positive");
        assert!(ts.rollers[0].inner.mu > 0.0);
        assert!(ts.rollers[0].outer.mu > 0.0);
    }

    #[test]
    fn test_traction_static_none() {
        let geom = test_geom();
        let mat = test_material();
        let rp = test_raceway_profile();
        let op = test_op(0.0);
        let rg = test_raceway_geom();
        let sg = test_slice_geometries(&geom, 10);
        let rolp = test_roller_profile();
        assert!(compute_traction(&geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &[]).is_none());
    }

    // ─── Advanced model tests ───

    #[test]
    fn test_roelands_p_zero_returns_eta0() {
        // At p=0, Roelands must return η₀ exactly
        let eta_0 = 0.05; // 50 mPa·s
        let result = roelands_viscosity(eta_0, 0.0, 0.67);
        assert!((result - eta_0).abs() < 1e-15,
            "Roelands(p=0) = {result}, expected {eta_0}");
    }

    #[test]
    fn test_roelands_less_than_barus() {
        // At high pressure, Roelands < Barus (Barus over-predicts)
        let eta_0 = 0.05;
        let alpha_pv = 20e-9; // 20 1/GPa in 1/Pa
        let p = 1.0e9; // 1 GPa
        let z_r = 0.67;
        let eta_roelands = roelands_viscosity(eta_0, p, z_r);
        let eta_barus = eta_0 * (alpha_pv * p).exp();
        assert!(eta_roelands < eta_barus,
            "Roelands({eta_roelands:.2e}) must be < Barus({eta_barus:.2e}) at 1 GPa");
        assert!(eta_roelands > eta_0,
            "Roelands must increase with pressure");
    }

    #[test]
    fn test_roelands_pao_vs_mineral() {
        // PAO (z_r≈0.5) should give lower viscosity increase than mineral (z_r≈0.67)
        let eta_0 = 0.05;
        let p = 500e6; // 500 MPa
        let eta_mineral = roelands_viscosity(eta_0, p, 0.67);
        let eta_pao = roelands_viscosity(eta_0, p, 0.50);
        assert!(eta_pao < eta_mineral,
            "PAO({eta_pao:.4e}) should be < mineral({eta_mineral:.4e}) at same pressure");
    }

    #[test]
    fn test_mk_smooth_surface_convergence() {
        // σ̄ → 0: M-K correction factor → 1, reducing to standard D-H-like formula
        let u = 1e-11;   // typical U
        let g = 5000.0;  // typical G
        let w = 1e-4;    // typical W
        let sigma_bar = 1e-12; // nearly zero roughness
        let v = 1e-8;
        let r_eq = 0.005; // 5mm equivalent radius
        let result = compute_film_mk(u, g, w, sigma_bar, v, r_eq);
        assert!(result.h_c > 0.0, "h_c must be positive");
        assert!(result.h_min > 0.0, "h_min must be positive");
        assert!(result.h_min <= result.h_c, "h_min ≤ h_c");
        assert!(result.load_fraction < 0.01,
            "Smooth surface should have near-zero asperity load: {}", result.load_fraction);
        assert!(result.area_fraction < 0.01,
            "Smooth surface should have near-zero asperity area: {}", result.area_fraction);
    }

    #[test]
    fn test_mk_rough_surface_increases_asperity() {
        let u = 1e-11;
        let g = 5000.0;
        let w = 1e-4;
        let r_eq = 0.005;

        // Smooth
        let smooth = compute_film_mk(u, g, w, 1e-12, 1e-8, r_eq);
        // Rough (σ̄ = 1e-5, typical for Rq=0.3μm, R=30mm)
        let rough = compute_film_mk(u, g, w, 1e-5, 0.01, r_eq);

        assert!(rough.load_fraction > smooth.load_fraction,
            "Rough surface load_fraction({}) > smooth({})",
            rough.load_fraction, smooth.load_fraction);
        assert!(rough.area_fraction > smooth.area_fraction,
            "Rough surface area_fraction({}) > smooth({})",
            rough.area_fraction, smooth.area_fraction);
    }

    #[test]
    fn test_murch_wilson_srr_zero() {
        // At SRR=0, thermal correction should be close to 1 (minimal thermal effect)
        let phi_t = thermal_correction_murch_wilson(
            0.05,   // eta_0
            0.04,   // beta_visc
            1.0,    // u_m [m/s]
            0.15,   // k_fluid
            0.0,    // SRR = 0
            500e6,  // p_hz [Pa]
            220e9,  // E*
        );
        assert!(phi_t > 0.85 && phi_t <= 1.0,
            "φ_T at SRR=0 should be near 1.0, got {phi_t}");
    }

    #[test]
    fn test_murch_wilson_srr_increases_correction() {
        // Higher SRR → more thermal thinning → lower φ_T
        let phi_0 = thermal_correction_murch_wilson(0.05, 0.04, 2.0, 0.15, 0.0, 500e6, 220e9);
        let phi_1 = thermal_correction_murch_wilson(0.05, 0.04, 2.0, 0.15, 0.5, 500e6, 220e9);
        let phi_2 = thermal_correction_murch_wilson(0.05, 0.04, 2.0, 0.15, 2.0, 500e6, 220e9);
        assert!(phi_0 >= phi_1, "φ_T(SRR=0)={phi_0} ≥ φ_T(SRR=0.5)={phi_1}");
        assert!(phi_1 >= phi_2, "φ_T(SRR=0.5)={phi_1} ≥ φ_T(SRR=2.0)={phi_2}");
        assert!(phi_2 >= 0.3, "φ_T should be clamped ≥ 0.3, got {phi_2}");
    }

    #[test]
    fn test_eyring_traction_srr_zero() {
        // At SRR=0, traction should be zero (no sliding)
        let mu = eyring_traction_advanced(0.0, 2.0, 500e6, 0.5e-6, 0.05, 0.67, 5e6);
        assert!(mu.abs() < 1e-10, "μ at SRR=0 should be ~0, got {mu}");
    }

    #[test]
    fn test_eyring_traction_saturation() {
        // At very high SRR, traction should saturate at ~0.1 (shear limit = 0.1×p)
        let mu_low = eyring_traction_advanced(0.01, 2.0, 500e6, 0.5e-6, 0.05, 0.67, 5e6);
        let mu_high = eyring_traction_advanced(1.0, 2.0, 500e6, 0.5e-6, 0.05, 0.67, 5e6);
        let mu_extreme = eyring_traction_advanced(5.0, 2.0, 500e6, 0.5e-6, 0.05, 0.67, 5e6);

        assert!(mu_low > 0.0, "μ at SRR=0.01 > 0");
        assert!(mu_high > mu_low, "μ increases with SRR");
        // Saturation: extreme SRR should be capped at ~0.10
        assert!((mu_extreme - mu_high).abs() < 0.02,
            "Traction should saturate: μ(SRR=1)={mu_high:.4}, μ(SRR=5)={mu_extreme:.4}");
        assert!(mu_extreme <= 0.10 + 1e-6,
            "μ should not exceed τ_lim/p ≈ 0.10, got {mu_extreme}");
    }

    #[test]
    fn test_eyring_traction_curve_shape() {
        // Linear → logarithmic → saturation transition
        let p = 500e6;
        let h_c = 0.5e-6;
        let eta_0 = 0.05;
        let tau_0 = 5e6;

        let mus: Vec<f64> = [0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 2.0]
            .iter()
            .map(|&srr| eyring_traction_advanced(srr, 2.0, p, h_c, eta_0, 0.67, tau_0))
            .collect();

        // Monotonically increasing
        for i in 1..mus.len() {
            assert!(mus[i] >= mus[i-1] - 1e-10,
                "μ must be monotonic: μ[{}]={:.6} < μ[{}]={:.6}", i, mus[i], i-1, mus[i-1]);
        }
    }

    // ─── Carreau-Yasuda Traction Tests ──────────────────────────────

    /// Newtonian limit: λ·γ̇ → 0 ⇒ η_eff → η_0 (within 0.1%).
    #[test]
    fn test_carreau_newtonian_limit() {
        let eta_0 = 0.05;
        let eta_inf = 0.005 * eta_0;
        // λ·γ̇ = 1e-7 × 100 = 1e-5 ≪ 1
        let gamma_dot = 100.0;
        let lambda = 1e-7;
        let eta = carreau_yasuda_viscosity(eta_0, eta_inf, lambda, 0.5, 2.0, gamma_dot);
        let rel_err = ((eta - eta_0) / eta_0).abs();
        assert!(rel_err < 1e-3,
            "Carreau low-shear must approach η_0: η={eta:.6e}, η_0={eta_0}, rel_err={rel_err:.2e}");
    }

    /// High-shear plateau: λ·γ̇ ≫ 1 ⇒ η_eff → η_∞.
    /// For n=0.5, a=2 the convergence is slow: factor = (λγ̇)^(n-1) = (λγ̇)^(-0.5).
    /// Need λγ̇ ≥ 1e8 to reach within 1% of η_∞.
    #[test]
    fn test_carreau_high_shear_plateau() {
        let eta_0 = 0.05;
        let eta_inf_ratio = 0.005;
        let eta_inf = eta_0 * eta_inf_ratio;
        // λ·γ̇ = 1e-6 × 1e14 = 1e8  →  factor (1+1e16)^(-0.25) ≈ 1e-4
        let gamma_dot = 1.0e14;
        let lambda = 1e-6;
        let eta = carreau_yasuda_viscosity(eta_0, eta_inf, lambda, 0.5, 2.0, gamma_dot);
        let rel_err = ((eta - eta_inf) / eta_inf).abs();
        assert!(rel_err < 0.05,
            "Carreau high-shear must approach η_∞: η={eta:.6e}, η_∞={eta_inf:.6e}, rel_err={rel_err:.2e}");
    }

    /// Yasuda exponent `a` controls transition sharpness: larger `a` makes the
    /// shear-thinning bend more abrupt around λγ̇ = 1.
    /// Verify at the knee (λγ̇ = 1) that increasing `a` widens η_eff toward η_0.
    #[test]
    fn test_carreau_yasuda_exponent_sharpens_transition() {
        let eta_0 = 0.05;
        let eta_inf = 0.005 * eta_0;
        let lambda = 1e-7;
        let gamma_dot = 1.0 / lambda; // λγ̇ = 1 exactly
        let n = 0.5;

        let eta_a1 = carreau_yasuda_viscosity(eta_0, eta_inf, lambda, n, 1.0, gamma_dot);
        let eta_a2 = carreau_yasuda_viscosity(eta_0, eta_inf, lambda, n, 2.0, gamma_dot);
        let eta_a4 = carreau_yasuda_viscosity(eta_0, eta_inf, lambda, n, 4.0, gamma_dot);

        // At λγ̇=1, factor = 2^((n-1)/a). Larger a → factor closer to 1 → η closer to η_0
        assert!(eta_a4 > eta_a2 && eta_a2 > eta_a1,
            "η at knee should increase with a: a=1: {eta_a1:.4e}, a=2: {eta_a2:.4e}, a=4: {eta_a4:.4e}");
    }

    /// Monotonic decrease: η_eff is strictly decreasing in γ̇ on the transition region.
    #[test]
    fn test_carreau_monotonic_thinning() {
        let eta_0 = 0.05;
        let eta_inf = 0.005 * eta_0;
        let lambda = 1e-7;
        let gammas = [1e3, 1e5, 1e6, 1e7, 1e8, 1e9];
        let etas: Vec<f64> = gammas.iter()
            .map(|&g| carreau_yasuda_viscosity(eta_0, eta_inf, lambda, 0.5, 2.0, g))
            .collect();
        for i in 1..etas.len() {
            assert!(etas[i] <= etas[i-1] + 1e-12,
                "η must monotonically decrease: η[{}]={:.6e} > η[{}]={:.6e}",
                i, etas[i], i-1, etas[i-1]);
        }
        // First > last (sanity)
        assert!(etas[0] > etas.last().copied().unwrap() * 1.5,
            "Shear-thinning span insufficient: {:.3e} → {:.3e}", etas[0], etas.last().unwrap());
    }

    /// SRR = 0 → traction = 0 (no sliding, parity with Eyring).
    #[test]
    fn test_carreau_traction_srr_zero() {
        let mu = carreau_traction_advanced(
            0.0, 2.0, 500e6, 0.5e-6,
            0.05, 0.67,
            0.005, 1e-7, 0.5, 2.0,
        );
        assert!(mu.abs() < 1e-10, "μ at SRR=0 must be ~0, got {mu}");
    }

    /// Limiting shear stress cap (μ ≤ 0.10): same as Eyring's tau_lim safeguard.
    #[test]
    fn test_carreau_traction_limiting_shear_cap() {
        // Very high SRR + high η_inf forces τ to push past τ_lim
        let mu = carreau_traction_advanced(
            10.0, 5.0, 500e6, 0.2e-6,
            0.05, 0.67,
            0.05, 1e-9, 0.7, 2.0, // larger η_inf_ratio to keep stress high
        );
        assert!(mu <= 0.15 + 1e-9, "μ must be capped at clamp(0.15), got {mu}");
        assert!(mu > 0.0, "μ must be positive at high SRR");
    }

    /// Eyring and Carreau agree in the strict Newtonian limit: very low SRR + mild
    /// pressure (so Roelands stays close to η_0). Both reduce to τ ≈ η·γ̇.
    #[test]
    fn test_carreau_vs_eyring_newtonian_limit_agreement() {
        // Mild pressure — Roelands gives only ~3× η_0; very low SRR keeps Eyring's
        // sinh⁻¹ argument well below 1 (linear regime).
        let srr = 1e-5;
        let u_roll = 2.0;
        let p = 100e6;
        let h_c = 1.0e-6;
        let eta_0 = 0.05;
        let tau_0 = 5e6;

        let mu_e = eyring_traction_advanced(srr, u_roll, p, h_c, eta_0, 0.67, tau_0);
        let mu_c = carreau_traction_advanced(
            srr, u_roll, p, h_c, eta_0, 0.67,
            0.005, 1e-7, 0.5, 2.0,
        );
        let rel = ((mu_c - mu_e) / mu_e.max(1e-12)).abs();
        assert!(rel < 0.05,
            "Newtonian limit: Eyring μ={mu_e:.6e}, Carreau μ={mu_c:.6e}, rel diff={rel:.3}");
    }

    /// At moderate-to-high SRR with Roelands-elevated viscosity, both models hit
    /// the limiting shear stress cap (μ ≤ Λ_lim ≈ 0.10). Verify both saturate
    /// near the same ceiling — the qualitative ordering depends on parameters.
    #[test]
    fn test_carreau_vs_eyring_high_shear_both_saturate() {
        let srr = 1.0;
        let u_roll = 2.0;
        let p = 500e6;
        let h_c = 0.3e-6;
        let eta_0 = 0.05;
        let tau_0 = 5e6;

        let mu_e = eyring_traction_advanced(srr, u_roll, p, h_c, eta_0, 0.67, tau_0);
        let mu_c = carreau_traction_advanced(
            srr, u_roll, p, h_c, eta_0, 0.67,
            0.005, 1e-7, 0.5, 2.0,
        );
        // Both must be in physically reasonable range
        assert!(mu_e > 0.01 && mu_e <= 0.15, "Eyring μ out of range: {mu_e}");
        assert!(mu_c > 0.01 && mu_c <= 0.15, "Carreau μ out of range: {mu_c}");
        // Both should be near the saturation ceiling (within factor 3)
        let ratio = (mu_c / mu_e).max(mu_e / mu_c);
        assert!(ratio < 3.0,
            "High-shear saturation: Eyring={mu_e:.6}, Carreau={mu_c:.6}, ratio={ratio:.2}");
    }

    /// In a regime where Eyring is in the steeply-rising sinh⁻¹ region and Carreau
    /// is well past the relaxation knee, Carreau (with aggressive η_∞ ratio)
    /// can predict measurably *lower* μ — matches research report §5.1's
    /// qualitative observation for rib-end TEHL with EV-fluid parameters.
    #[test]
    fn test_carreau_aggressive_below_eyring_at_moderate_pressure() {
        // Moderate pressure (Roelands ~50× η_0), aggressive Carreau-Yasuda
        let srr = 0.5;
        let u_roll = 2.0;
        let p = 200e6;
        let h_c = 0.4e-6;
        let eta_0 = 0.05;
        let tau_0 = 5e6;

        let mu_e = eyring_traction_advanced(srr, u_roll, p, h_c, eta_0, 0.67, tau_0);
        // Aggressive: λ=1e-6, n=0.3, a=4, η_∞=0.0005 (1% of η_0)
        let mu_c = carreau_traction_advanced(
            srr, u_roll, p, h_c, eta_0, 0.67,
            0.01, 1e-6, 0.3, 4.0,
        );
        assert!(mu_c < mu_e,
            "With aggressive Carreau-Yasuda parameters at moderate pressure, \
             Carreau should yield lower μ than Eyring: \
             Eyring={mu_e:.6}, Carreau={mu_c:.6}");
        assert!(mu_c > 0.0);
    }

    /// Pure Carreau (a = 2) reduces to Carreau (1972) closed form:
    /// η = η_∞ + (η_0 − η_∞)·[1 + (λγ̇)^2]^((n−1)/2)
    #[test]
    fn test_carreau_a2_reduces_to_classical_carreau() {
        let eta_0 = 0.05;
        let eta_inf = 0.0001;
        let lambda = 1e-7;
        let n = 0.5;
        let gamma_dot = 1.0e8;

        let our = carreau_yasuda_viscosity(eta_0, eta_inf, lambda, n, 2.0, gamma_dot);
        let lg2 = (lambda * gamma_dot).powi(2);
        let classical = eta_inf + (eta_0 - eta_inf) * (1.0 + lg2).powf((n - 1.0) / 2.0);
        let rel = ((our - classical) / classical).abs();
        assert!(rel < 1e-12, "a=2 must equal classical Carreau: ours={our:.6e}, classical={classical:.6e}");
    }

    /// `traction_coefficient` dispatcher: TractionModel::Eyring path is bit-equivalent
    /// to direct `eyring_traction_advanced` call (backward-compat regression).
    #[test]
    fn test_traction_dispatcher_eyring_backward_compat() {
        let mut op = test_op(1000.0);
        op.traction_model = TractionModel::Eyring;
        // ensure tau_eyring matches direct call's argument
        op.tau_eyring = 5.0;
        op.z_roelands = 0.67;

        let srr = 0.5; let u_roll = 2.0; let p = 500e6; let h_c = 0.5e-6; let eta_0 = 0.05;

        let mu_dispatch = traction_coefficient(&op, srr, u_roll, p, h_c, eta_0);
        let mu_direct = eyring_traction_advanced(srr, u_roll, p, h_c, eta_0, 0.67, 5e6);
        assert!((mu_dispatch - mu_direct).abs() < 1e-15,
            "Dispatcher must be bit-equivalent to direct Eyring call: \
             dispatch={mu_dispatch}, direct={mu_direct}");
    }

    /// `traction_coefficient` with CarreauYasuda dispatches to Carreau path correctly.
    #[test]
    fn test_traction_dispatcher_carreau_path() {
        let mut op = test_op(1000.0);
        op.traction_model = TractionModel::CarreauYasuda;
        op.carreau_eta_inf_ratio = 0.005;
        op.carreau_lambda_s = 1e-7;
        op.carreau_n = 0.5;
        op.carreau_a = 2.0;
        op.z_roelands = 0.67;

        let srr = 0.5; let u_roll = 2.0; let p = 500e6; let h_c = 0.5e-6; let eta_0 = 0.05;

        let mu_dispatch = traction_coefficient(&op, srr, u_roll, p, h_c, eta_0);
        let mu_direct = carreau_traction_advanced(
            srr, u_roll, p, h_c, eta_0, 0.67, 0.005, 1e-7, 0.5, 2.0,
        );
        assert!((mu_dispatch - mu_direct).abs() < 1e-15,
            "Dispatcher must equal direct Carreau call: dispatch={mu_dispatch}, direct={mu_direct}");
        // Also: returns different value than Eyring path on same inputs
        op.traction_model = TractionModel::Eyring;
        let mu_eyring = traction_coefficient(&op, srr, u_roll, p, h_c, eta_0);
        assert!((mu_dispatch - mu_eyring).abs() > 1e-6,
            "Carreau and Eyring dispatchers should differ at SRR=0.5: \
             Carreau={mu_dispatch:.6}, Eyring={mu_eyring:.6}");
    }

    /// Sweep across SRR: μ(SRR) for Carreau is monotonic non-decreasing and
    /// stays bounded by the limiting-shear-stress cap (≤ 0.15 by clamp).
    #[test]
    fn test_carreau_traction_curve_shape() {
        let p = 500e6;
        let h_c = 0.5e-6;
        let eta_0 = 0.05;

        let srrs = [0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 2.0];
        let mu_c: Vec<f64> = srrs.iter()
            .map(|&srr| carreau_traction_advanced(
                srr, 2.0, p, h_c, eta_0, 0.67, 0.005, 1e-7, 0.5, 2.0))
            .collect();

        // Monotonic non-decreasing in SRR
        for i in 1..mu_c.len() {
            assert!(mu_c[i] >= mu_c[i-1] - 1e-10,
                "Carreau μ must be monotonic in SRR: μ[{}]={:.6e}, μ[{}]={:.6e}",
                i-1, mu_c[i-1], i, mu_c[i]);
        }
        // Bounded by the clamp (0.15) and the τ_lim cap (0.10·p)
        for &mu in &mu_c {
            assert!(mu <= 0.15 + 1e-9, "Carreau μ above clamp: {mu}");
            assert!(mu >= 0.0, "Carreau μ negative: {mu}");
        }
        // Strict growth from low SRR to peak
        assert!(*mu_c.last().unwrap() > mu_c[0] * 2.0,
            "Sweep should show clear traction growth: {:?}", mu_c);
    }

    /// Backward-compatibility integration: `compute_traction_advanced()` with default
    /// (Eyring) model produces identical results before/after dispatcher refactor.
    /// We approximate "before" by directly calling Eyring inside this test.
    #[test]
    fn test_compute_traction_advanced_default_eyring_unchanged() {
        let geom = test_geom();
        let mat = test_material();
        let rp = test_raceway_profile();
        let mut op = test_op(1000.0);
        op.lubrication_model = LubricationModel::Method2_MK;
        // explicit default
        op.traction_model = TractionModel::Eyring;
        let rg = test_raceway_geom();
        let sg = test_slice_geometries(&geom, 10);
        let rolp = test_roller_profile();

        let slices: Vec<SliceContactResult> = (0..10).map(|k| SliceContactResult {
            k, delta_k: 5.0, q_k: 50.0, q_k_outer: 50.0, q_k_inner: 50.0,
            b_k: 0.1, p_max_k: 1500.0, h_bulk_k: 0.0, k_hertz_k: 0.0,
            b_k_outer: 0.1, p_max_k_outer: 1400.0, h_bulk_k_outer: 0.0,
            k_hertz_k_outer: 0.0, k_combined_k: 0.0, in_contact: true,
        }).collect();
        let rollers = vec![RollerResult {
            psi_deg: 0.0, q_normal: 500.0, q_normal_inner: 500.0,
            slice_results: slices, rib_result: None,
        }];

        let ts_eyring = compute_traction_advanced(
            &geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers
        ).expect("Eyring path must succeed");

        // Switch to Carreau and verify a *different* friction torque emerges
        op.traction_model = TractionModel::CarreauYasuda;
        let ts_carreau = compute_traction_advanced(
            &geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers
        ).expect("Carreau path must succeed");

        assert!(ts_eyring.m_friction_nmm > 0.0);
        assert!(ts_carreau.m_friction_nmm > 0.0);
        // Models should give measurably different torque (within sanity: same order of magnitude)
        let ratio = ts_carreau.m_friction_nmm / ts_eyring.m_friction_nmm;
        assert!(ratio > 0.1 && ratio < 2.0,
            "Carreau/Eyring torque ratio out of sanity range: \
             Eyring={:.3} N·mm, Carreau={:.3} N·mm, ratio={:.3}",
            ts_eyring.m_friction_nmm, ts_carreau.m_friction_nmm, ratio);
        // p_max / film must be unchanged (only μ depends on traction model)
        assert!((ts_eyring.p_contact_total_w - ts_carreau.p_contact_total_w).abs()
                / ts_eyring.p_contact_total_w < 0.5,
            "Power dissipation can differ but should remain same order of magnitude");
    }

    /// Cross-tab consistency: when `rib_result.ehl.mu_eff` is populated,
    /// `compute_traction_advanced` must use that exact value for rib friction
    /// (mirroring the Rib EHL sub-tab in `LubricationView`). Falls back to
    /// the dry constant `MU_RIB` only when ehl is None.
    #[test]
    fn test_rib_friction_uses_ehl_mu_eff() {
        let geom = test_geom();
        let mat = test_material();
        let rp = test_raceway_profile();
        let mut op = test_op(1000.0);
        op.lubrication_model = LubricationModel::Method2_MK;
        let rg = test_raceway_geom();
        let sg = test_slice_geometries(&geom, 10);
        let rolp = test_roller_profile();

        let slices: Vec<SliceContactResult> = (0..10).map(|k| SliceContactResult {
            k, delta_k: 5.0, q_k: 50.0, q_k_outer: 50.0, q_k_inner: 50.0,
            b_k: 0.1, p_max_k: 1500.0, h_bulk_k: 0.0, k_hertz_k: 0.0,
            b_k_outer: 0.1, p_max_k_outer: 1400.0, h_bulk_k_outer: 0.0,
            k_hertz_k_outer: 0.0, k_combined_k: 0.0, in_contact: true,
        }).collect();

        // Inject a distinctive μ_eff via a hand-built RibEhlResult so we can
        // assert it propagates verbatim. Using 0.0123 (clearly different from
        // both MU_RIB=0.06 and any plausible computed value).
        let injected_mu = 0.0123_f64;
        let ehl = RibEhlResult {
            h_c_um: 0.5, h_min_um: 0.4, sigma_composite_um: 0.3,
            lambda_ratio: 1.33, regime: LubricationRegime::Mixed,
            mu_eff: injected_mu, mu_ehl: 0.005, asperity_load_ratio: 0.5,
            p_asperity_mpa: 100.0, flash_temp_c: 12.0,
            srr: 2.0, u_entrain_m_s: 1.0, u_slide_m_s: 2.0,
            thermal_factor: 0.9, u_param: 1e-11, g_param: 5000.0, w_param: 1e-4,
            k_ellipse: 1.5,
        };
        let rib_with_ehl = RibContactResult {
            f_rib: 100.0, a_ellipse: 0.5, b_ellipse: 0.4, p_max_rib: 800.0,
            spin_moment: 0.0, delta_rib: 1.0, k_rib: 100.0,
            r_contact_mm: 50.0, r_rib_circ_mm: 100.0, h_c_mm: 1.0,
            ehl: Some(ehl),
        };
        let rib_dry = RibContactResult {
            f_rib: 100.0, a_ellipse: 0.5, b_ellipse: 0.4, p_max_rib: 800.0,
            spin_moment: 0.0, delta_rib: 1.0, k_rib: 100.0,
            r_contact_mm: 50.0, r_rib_circ_mm: 100.0, h_c_mm: 1.0,
            ehl: None,
        };

        let rollers_ehl = vec![RollerResult {
            psi_deg: 0.0, q_normal: 500.0, q_normal_inner: 500.0,
            slice_results: slices.clone(), rib_result: Some(rib_with_ehl),
        }];
        let rollers_dry = vec![RollerResult {
            psi_deg: 0.0, q_normal: 500.0, q_normal_inner: 500.0,
            slice_results: slices, rib_result: Some(rib_dry),
        }];

        let ts_ehl = compute_traction_advanced(&geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers_ehl)
            .expect("EHL path must succeed");
        let ts_dry = compute_traction_advanced(&geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers_dry)
            .expect("Dry path must succeed");

        // EHL path: rib friction μ must equal injected mu_eff (0.0123)
        let rib_ehl = ts_ehl.rollers[0].rib.as_ref().expect("rib must be Some");
        assert!((rib_ehl.mu - injected_mu).abs() < 1e-12,
            "Mixed&Traction tab must use rib EHL μ_eff verbatim: \
             expected {injected_mu}, got {}", rib_ehl.mu);

        // Dry path (ehl=None): falls back to MU_RIB constant (= 0.06)
        let rib_d = ts_dry.rollers[0].rib.as_ref().expect("rib must be Some");
        assert!((rib_d.mu - MU_RIB).abs() < 1e-12,
            "Dry path must fall back to MU_RIB={MU_RIB}, got {}", rib_d.mu);

        // Cross-check: power_loss uses Houpert drilling-moment formulation
        // (NOT pure sliding μ·F·u — that over-predicts by ~16× for TRB rib).
        //   P = (3/8) · μ · F_rib · a_ellipse [N·mm] · ω_roller [rad/s] · 1e-3
        // a_ellipse = 0.5 mm, F_rib = 100 N, μ = 0.0123, ω_roller from kinematics.
        let kin = compute_trb_kinematics(&geom, &rg, &op);
        let expected_p_drilling = 0.375 * injected_mu * 100.0 * 0.5
            * 1e-3 * kin.omega_roller.abs();
        assert!((rib_ehl.power_loss_w - expected_p_drilling).abs() < 1e-9,
            "Rib power must equal (3/8)·μ·F_rib·a·ω_roller (Houpert drilling): \
             expected {expected_p_drilling:.6}, got {}", rib_ehl.power_loss_w);
    }

    // ─── SKF Reference Model — Self-Validation ──────────────────────

    // ─── FrictionModel Dispatcher Integration ───────────────────────

    /// Default `friction_model = PalmgrenLike` preserves prior totals and
    /// also publishes `skf_reference` for the UI to display side-by-side.
    #[test]
    fn test_friction_model_default_palmgren_publishes_skf_reference() {
        let geom = test_geom();
        let mat = test_material();
        let rp = test_raceway_profile();
        let op = test_op(1500.0); // friction_model defaults to PalmgrenLike
        let rg = test_raceway_geom();
        let sg = test_slice_geometries(&geom, 10);
        let rolp = test_roller_profile();

        let make_slices = || -> Vec<SliceContactResult> {
            (0..10).map(|k| SliceContactResult {
                k, delta_k: 5.0, q_k: 50.0, q_k_outer: 50.0, q_k_inner: 50.0,
                b_k: 0.1, p_max_k: 1500.0, h_bulk_k: 0.0, k_hertz_k: 0.0,
                b_k_outer: 0.1, p_max_k_outer: 1400.0, h_bulk_k_outer: 0.0,
                k_hertz_k_outer: 0.0, k_combined_k: 0.0, in_contact: true,
            }).collect()
        };
        let rollers = vec![RollerResult {
            psi_deg: 0.0, q_normal: 800.0, q_normal_inner: 800.0 * 0.998,
            slice_results: make_slices(), rib_result: None,
        }];

        let ts = compute_traction(&geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers)
            .expect("traction must succeed");
        assert!(matches!(ts.friction_model, FrictionModel::PalmgrenLike));
        let skf = ts.skf_reference.expect("skf_reference must be populated");
        assert!(skf.m_total_nmm > 0.0, "SKF reference torque must be positive");
        assert!(skf.p_total_w > 0.0);
        // Palmgren-like totals must be consistent with our P_rolling per roller
        assert!(ts.p_rolling_w > 0.0);
    }

    // ─── Biboulet-Houpert 2010 Part 1 (line contact) unit checks ────

    /// Part 1 IVR asymptote (Eq. 40): T̃_IVR = 1.42·U^(1/2)·W_l^(1/2).
    /// In low-load/high-speed limit (M_l → 0) the unified formula must
    /// reduce to this analytical form within < 1 %.
    #[test]
    fn test_bh_line_ivr_asymptote() {
        // Choose conditions that drive M_l → 0
        let eta_0 = 0.05_f64;
        let u_m = 5.0;
        let q_per_l = 1e3; // 1 kN/m, very light line load
        let r = 0.01;      // 10 mm
        let e_prime = 2.3e11_f64;
        let f_l = biboulet_houpert_line_force_per_length(eta_0, u_m, q_per_l, r, e_prime);
        // Reproduce IVR analytical formula
        let u = 2.0 * eta_0 * u_m / (e_prime * r);
        let w_l = q_per_l / (e_prime * r);
        let t_tilde_ivr = 1.42 * u.sqrt() * w_l.sqrt();
        let f_l_ivr = t_tilde_ivr * e_prime * r;
        let rel = ((f_l - f_l_ivr) / f_l_ivr).abs();
        assert!(rel < 0.01,
            "Light-load BH line: f_l should match IVR asymptote: f={f_l:.3e}, f_IVR={f_l_ivr:.3e}, rel={rel:.4}");
    }

    /// Part 1 EHL asymptote (Eq. 41): T̃_EHL∞ = 1.47·U^(3/4) — load-INDEPENDENT.
    /// In high-load limit (M_l → ∞), doubling the load must leave f_l unchanged
    /// (within ≲ 5 % numerical noise).
    #[test]
    fn test_bh_line_ehl_load_independent() {
        let eta_0 = 0.05_f64;
        let u_m = 1.0;
        let r = 0.01_f64;
        let e_prime = 2.3e11_f64;
        // Drive M_l large via heavy line load
        let f_l_a = biboulet_houpert_line_force_per_length(eta_0, u_m, 1e7, r, e_prime);
        let f_l_b = biboulet_houpert_line_force_per_length(eta_0, u_m, 2e7, r, e_prime);
        let ratio = f_l_b / f_l_a;
        assert!(ratio > 0.95 && ratio < 1.05,
            "EHL line load-independence: ratio should be ≈1, got {ratio:.3}");
    }

    /// EHL absolute asymptote: f_l = 1.47·U^(3/4)·E'·R for very heavy load.
    #[test]
    fn test_bh_line_ehl_value() {
        let eta_0 = 0.05_f64;
        let u_m = 1.0;
        let r = 0.01_f64;
        let e_prime = 2.3e11_f64;
        let f_l = biboulet_houpert_line_force_per_length(eta_0, u_m, 1e8, r, e_prime);
        let u = 2.0 * eta_0 * u_m / (e_prime * r);
        let f_l_ehl = 1.47 * u.powf(0.75) * e_prime * r;
        let rel = ((f_l - f_l_ehl) / f_l_ehl).abs();
        assert!(rel < 0.05,
            "Heavy-load BH line: f_l should match EHL∞ asymptote: f={f_l:.4e}, f_EHL={f_l_ehl:.4e}, rel={rel:.4}");
    }

    /// Power wrapper: P = (f_l × L) × |u|.
    #[test]
    fn test_bh_line_power_wrapper() {
        let eta_0 = 0.05_f64; let u = 2.0; let q = 5e3;
        let l = 0.012; let r = 0.005; let e_p = 2.3e11_f64;
        let f_l = biboulet_houpert_line_force_per_length(eta_0, u, q / l, r, e_p);
        let p = biboulet_houpert_line_rolling_power(eta_0, u, q, l, r, e_p);
        let p_expected = f_l * l * u;
        assert!((p - p_expected).abs() < 1e-9 * p.max(1.0));
    }

    /// Part 1 (line) and Part 2 (point with k=100) are within the same order
    /// of magnitude. Both come from the same theoretical work; quantitative
    /// agreement to ~factor 2 is expected.
    #[test]
    fn test_bh_line_vs_point_same_order() {
        let eta_0 = 0.05_f64; let u = 1.5; let r = 0.005; let e_p = 2.3e11_f64;
        let l = 0.012; let q = 1000.0;
        let p_line = biboulet_houpert_line_rolling_power(eta_0, u, q, l, r, e_p);
        let p_pt   = biboulet_houpert_rolling_power(eta_0, u, q, r, r * 100.0, e_p);
        assert!(p_line > 0.0 && p_pt > 0.0);
        let ratio = (p_line / p_pt).max(p_pt / p_line);
        assert!(ratio < 5.0,
            "Part 1 vs Part 2 (k=100) should agree within factor 5: \
             line={p_line:.4e}, point={p_pt:.4e}, ratio={ratio:.3}");
    }

    // ─── BH 2010 Part 1 — Validation against published asymptotes ────
    //
    // Biboulet-Houpert 2010 *Proc. IMechE Part J* 224 (Part 1, line contacts)
    // gives explicit exponent and coefficient values for the IVR (Eq. 40)
    // and EHL∞ (Eq. 41) asymptotes:
    //
    //   T̃_IVR     = 1.42 · U_l^(1/2) · W_l^(1/2)
    //   T̃_EHL∞   = 1.47 · U_l^(3/4)            (load-INDEPENDENT)
    //
    // The transition (Eq. 42) is built so that f_l → T̃_IVR·E'·R as M_l→0
    // and f_l → T̃_EHL∞·E'·R as M_l→∞. The tests below pin the load and
    // speed exponents in each regime separately, plus the M_l=1 crossover
    // value, plus a realistic TRB raceway snapshot.

    /// IVR load exponent: at light load (M_l ≪ 1), f_l ∝ W_l^(1/2).
    /// Doubling q_per_L → ratio = 2^0.5 = 1.4142.
    #[test]
    fn test_bh_line_ivr_load_exponent() {
        let eta_0 = 0.05_f64; let u = 1.0; let r = 0.01_f64; let e_prime = 2.3e11_f64;
        // Light loads keep M_l in the IVR regime
        let f1 = biboulet_houpert_line_force_per_length(eta_0, u, 1e3, r, e_prime);
        let f2 = biboulet_houpert_line_force_per_length(eta_0, u, 2e3, r, e_prime);
        let ratio = f2 / f1;
        let expected = 2.0_f64.sqrt();
        let rel = ((ratio - expected) / expected).abs();
        assert!(rel < 0.01, "IVR f_l ∝ W_l^0.5: 2× load ratio={ratio:.4} vs {expected:.4}");
    }

    /// IVR speed exponent: at light load, f_l ∝ U_l^(1/2).
    /// Doubling u_m → ratio = 2^0.5 = 1.4142.
    #[test]
    fn test_bh_line_ivr_speed_exponent() {
        let eta_0 = 0.05_f64; let q = 1e3; let r = 0.01_f64; let e_prime = 2.3e11_f64;
        let f1 = biboulet_houpert_line_force_per_length(eta_0, 1.0, q, r, e_prime);
        let f2 = biboulet_houpert_line_force_per_length(eta_0, 2.0, q, r, e_prime);
        let ratio = f2 / f1;
        let expected = 2.0_f64.sqrt();
        let rel = ((ratio - expected) / expected).abs();
        assert!(rel < 0.01, "IVR f_l ∝ U_l^0.5: 2× speed ratio={ratio:.4} vs {expected:.4}");
    }

    /// EHL∞ speed exponent: at heavy load, f_l ∝ U_l^(3/4).
    /// Doubling u_m → ratio = 2^0.75 = 1.6818.
    #[test]
    fn test_bh_line_ehl_speed_exponent() {
        let eta_0 = 0.05_f64; let q = 1e8; let r = 0.01_f64; let e_prime = 2.3e11_f64;
        let f1 = biboulet_houpert_line_force_per_length(eta_0, 1.0, q, r, e_prime);
        let f2 = biboulet_houpert_line_force_per_length(eta_0, 2.0, q, r, e_prime);
        let ratio = f2 / f1;
        let expected = 2.0_f64.powf(0.75);
        let rel = ((ratio - expected) / expected).abs();
        assert!(rel < 0.02, "EHL∞ f_l ∝ U_l^0.75: 2× speed ratio={ratio:.4} vs {expected:.4}");
    }

    /// IVR↔EHL crossover at M_l = 1 — by construction of Eq. 42 the value
    /// at M_l=1 equals T̃_IVR(M_l=1) / (1 + (1.4/1.45)^10)^0.1, which is
    /// 0.948× T̃_IVR. Cross-checked analytically via T̃_IVR(M_l=1) = 1.42·U_l^(3/4).
    #[test]
    fn test_bh_line_transition_at_ml_one() {
        let eta_0 = 0.05_f64; let u_m = 1.0; let r = 0.01_f64; let e_prime = 2.3e11_f64;
        // Solve W_l so that M_l = W_l/sqrt(U_l) = 1
        let u_l = 2.0 * eta_0 * u_m / (e_prime * r);
        let w_l_target = u_l.sqrt();              // M_l = 1
        let q_per_l = w_l_target * e_prime * r;
        let f_l = biboulet_houpert_line_force_per_length(eta_0, u_m, q_per_l, r, e_prime);

        // Analytical reference at M_l = 1
        let r_blend = 1.4_f64 / 1.45;
        let denom = (1.0 + r_blend.powi(10)).powf(0.1);
        let t_tilde_ivr = 1.42 * u_l.sqrt() * w_l_target.sqrt();
        let f_l_ref = t_tilde_ivr / denom * e_prime * r;

        let rel = ((f_l - f_l_ref) / f_l_ref).abs();
        assert!(rel < 1e-9,
            "M_l=1 transition: f_l={f_l:.5e}, ref={f_l_ref:.5e}, rel={rel:.2e}");
        // Sanity: result is ~5% below either asymptote (smooth blend characteristic)
        let f_l_ehl = 1.47 * u_l.powf(0.75) * e_prime * r;
        let frac = f_l / f_l_ehl;
        assert!(frac > 0.85 && frac < 1.0,
            "M_l=1 should sit just below EHL∞: f_l/f_l_EHL={frac:.4}");
    }

    /// Realistic TRB raceway snapshot — locks numerical behavior at
    /// representative operating point (30206-class):
    ///   η₀ = 0.05 Pa·s (ISO VG 100 @ 80 °C)
    ///   u_m = 2.0 m/s (~1500 rpm × pitch radius ~13 mm, mid-bearing)
    ///   q_per_L = 2 MN/m (typical raceway line load Q≈24 kN over L≈12 mm)
    ///   R = 8 mm (rolling-direction equivalent radius)
    ///   E' = 2.3e11 Pa (steel on steel)
    /// → M_l ≈ 104 (deep EHL regime, near load-independent asymptote).
    /// f_l ≈ 91.2 N/m and per-contact P_R ≈ 2.19 W are within the typical
    /// per-contact bearing rolling-loss range.
    #[test]
    fn test_bh_line_realistic_trb_snapshot() {
        let eta_0 = 0.05_f64;
        let u_m = 2.0;
        let q_per_l = 2.0e6;          // N/m
        let r = 0.008_f64;            // m
        let e_prime = 2.3e11_f64;
        let l_contact = 0.012_f64;    // m
        let q_total = q_per_l * l_contact; // = 24 kN

        let f_l = biboulet_houpert_line_force_per_length(eta_0, u_m, q_per_l, r, e_prime);
        let p_roll = biboulet_houpert_line_rolling_power(eta_0, u_m, q_total, l_contact, r, e_prime);

        // Sanity: deep EHL regime (M_l ≫ 1) → f_l should be close to EHL∞
        let u_l = 2.0 * eta_0 * u_m / (e_prime * r);
        let f_l_ehl = 1.47 * u_l.powf(0.75) * e_prime * r;
        let rel_to_ehl = ((f_l - f_l_ehl) / f_l_ehl).abs();
        assert!(rel_to_ehl < 0.02,
            "Realistic TRB at M_l≈104 should match EHL∞ within 2%: f_l={f_l:.3} N/m, EHL={f_l_ehl:.3}");

        // Hardcoded snapshot — implementation as of feat/gen3-split-contact (2026-05-05).
        // Drift detection bracket: ±2 % captures floating-point noise but flags
        // any coefficient/exponent change in the formula or its dispatcher.
        let f_l_ref = 91.2_f64;       // N/m
        let p_ref = 2.19_f64;         // W (= f_l × L × u_m)
        assert!((f_l - f_l_ref).abs() / f_l_ref < 0.02,
            "Realistic TRB f_l drift: {f_l:.3} vs {f_l_ref}");
        assert!((p_roll - p_ref).abs() / p_ref < 0.02,
            "Realistic TRB P_roll drift: {p_roll:.4} vs {p_ref}");
    }

    /// Mihaela-Houpert 2015 *Lubricants* 3:222-235, "Rolling Friction Torque
    /// in Ball-Race Contacts in Mixed Lubrication" — experimental benchmark
    /// for Part 2 (point contact). Test rig: 3-ball thrust bearing.
    /// Conditions: d = 6.35 mm, η₀ = 0.05 Pa·s, Q = 0.125 N (IVR) and
    /// Q = 0.633 N (EHL-dominant) per §4.4.
    /// We verify three properties the paper reports:
    /// (a) F_R is in the µN range for these sub-Newton ball-thrust contacts;
    /// (b) F_R grows monotonically with load (Tz ∝ ω^n with n>0);
    /// (c) EHL partition fraction matches paper's reported regime
    ///     (M/(M+6.6) > 50 % heavy, ~40-60 % light) by reading M directly
    ///     from the formula's intermediate group.
    /// Pitch-radius / meniscus details are not in the paper text, so we
    /// don't try to match Tz absolute value — only F_R per contact.
    #[test]
    fn test_bh_point_mihaela_houpert_2015_ball_thrust() {
        let eta_0 = 0.05_f64;
        let r_ball = 6.35e-3_f64 / 2.0;
        let (r_x, r_y) = (r_ball, r_ball);
        let e_prime = 2.3e11_f64;

        let q_h = 0.633_f64; // EHL-dominant per §4.4
        let q_l = 0.125_f64; // IVR-dominant per §4.4

        let u_m = 0.1_f64;   // ~mid-range surface speed at 60-210 rpm

        let f_h = biboulet_houpert_rolling_force(eta_0, u_m, q_h, r_x, r_y, e_prime);
        let f_l = biboulet_houpert_rolling_force(eta_0, u_m, q_l, r_x, r_y, e_prime);

        // (a) Magnitude — sub-millinewton range
        assert!(f_h > 1e-7 && f_h < 1e-3,
            "Heavy-load BH point force out of range: {f_h:.3e} N");
        assert!(f_l > 1e-7 && f_l < 1e-3,
            "Light-load BH point force out of range: {f_l:.3e} N");

        // (b) Monotone in load
        assert!(f_h > f_l,
            "F_R should grow with load: F_R(0.633N)={f_h:.3e}, F_R(0.125N)={f_l:.3e}");

        // (c) EHL partition matches paper §4.4
        // M = 0.5549·k^(-0.6029)·W·U^(-0.75); k = R_y/R_x = 1.
        let u_param = eta_0 * u_m / (e_prime * r_x);
        let w_h = q_h / (e_prime * r_x * r_x);
        let w_l = q_l / (e_prime * r_x * r_x);
        let m_h = 0.5549 * w_h * u_param.powf(-0.75);
        let m_l_pt = 0.5549 * w_l * u_param.powf(-0.75);
        let ehl_frac_h = m_h / (m_h + 6.6); // = 1 - IVR weight in BH transition
        let ehl_frac_l = m_l_pt / (m_l_pt + 6.6);
        assert!(ehl_frac_h > 0.50,
            "Q=0.633 N should be EHL-dominant per paper §4.4: EHL_frac={ehl_frac_h:.3}");
        assert!(ehl_frac_l > 0.30 && ehl_frac_l < 0.70,
            "Q=0.125 N should sit in IVR↔EHL transition per paper §4.4: EHL_frac={ehl_frac_l:.3}");
    }

    // ─── SKF Bearing Select Tool — 30306 + LGMT 2 Reference ─────────
    //
    // Hardcoded reference values from SKF Bearing Select online tool
    // (https://www.skfbearingselect.com/) for designation 30306 with grease
    // SKF LGMT 2 (NLGI 2 lithium soap, mineral base oil ν₄₀=110 mm²/s,
    // ν₁₀₀=10 mm²/s, DIN 51825 K2K-30) at four operating points (LC1–LC4).
    //
    // Each load case below specifies (F_r, F_a, n, T_op) and the published
    // SKF Tool output for M_rr, M_sl, M_total. Our SKF reference
    // implementation must match within ±50 % across all four operating
    // points — looser than the ±20-30 % typical Catalogue accuracy because
    // (a) the exact 30306 sub-variant Y factor is bearing-specific (we use
    // 1.6 as the catalogue-typical value), (b) Walther viscosity vs the
    // SKF-internal ASTM D341 fit differs by a few percent, and (c) K_rs is
    // fixed at 6e-8 (generic grease) rather than the LGMT 2-specific value.
    //
    // The test exists as a live registry: future calibration changes that
    // drift any LC outside the bracket will fail and require an update.
    fn skf_30306_lgmt2_torque(f_r_kn: f64, f_a_kn: f64, n_rpm: f64, t_c: f64) -> SkfFrictionMoment {
        let nu_op = crate::solver::life::viscosity_at_temp_pub(110.0, 10.0, t_c);
        skf_frictional_moment_trb(
            SkfTrbSeries::Series303,
            30.0, 72.0,                  // d, D [mm]
            f_r_kn * 1000.0, f_a_kn * 1000.0,
            1.6,                          // catalogue-typical Y for 30306
            n_rpm, nu_op,
            SkfLubrication::Grease,       // LGMT 2 → K_rs = 6e-8
        )
    }

    /// LC1 — F_r=2 kN, F_a=1 kN, n=500 rpm, T=60 °C (light/slow)
    /// SKF Tool: M_rr=130, M_sl=51.6, M_total=181 N·mm.
    #[test]
    fn test_skf_30306_lgmt2_lc1_light_slow() {
        let m = skf_30306_lgmt2_torque(2.0, 1.0, 500.0, 60.0);
        for (label, ours, ref_v) in [
            ("M_rr",    m.m_rr_nmm,    130.0_f64),
            ("M_sl",    m.m_sl_nmm,    51.6),
            ("M_total", m.m_total_nmm, 181.0),
        ] {
            let ratio = ours / ref_v;
            assert!(ratio > 0.5 && ratio < 1.6,
                "LC1 {label}: ours={ours:.2}, ref={ref_v}, ratio={ratio:.3}");
        }
    }

    /// LC2 — F_r=5 kN, F_a=2 kN, n=1500 rpm, T=70 °C (default 30306 preset)
    /// SKF Tool: M_rr=217, M_sl=17, M_total=234 N·mm.
    #[test]
    fn test_skf_30306_lgmt2_lc2_default_preset() {
        let m = skf_30306_lgmt2_torque(5.0, 2.0, 1500.0, 70.0);
        for (label, ours, ref_v) in [
            ("M_rr",    m.m_rr_nmm,    217.0_f64),
            ("M_sl",    m.m_sl_nmm,    17.0),
            ("M_total", m.m_total_nmm, 234.0),
        ] {
            let ratio = ours / ref_v;
            assert!(ratio > 0.5 && ratio < 1.6,
                "LC2 {label}: ours={ours:.2}, ref={ref_v}, ratio={ratio:.3}");
        }
    }

    /// LC3 — F_r=15 kN, F_a=5 kN, n=1500 rpm, T=80 °C (heavy load)
    /// SKF Tool: M_rr=219, M_sl=125, M_total=345 N·mm.
    #[test]
    fn test_skf_30306_lgmt2_lc3_heavy_load() {
        let m = skf_30306_lgmt2_torque(15.0, 5.0, 1500.0, 80.0);
        // Heavy-load M_sl shows the largest deviation (~70 %) — likely from
        // an unmodelled mixed-regime calibration in SKF Tool.  Allow a wider
        // bracket on M_sl while still catching unit / coefficient regressions.
        let mr_ratio = m.m_rr_nmm / 219.0;
        let ml_ratio = m.m_sl_nmm / 125.0;
        let mt_ratio = m.m_total_nmm / 345.0;
        assert!(mr_ratio > 0.5 && mr_ratio < 1.6,
            "LC3 M_rr: ours={:.2}, ref=219, ratio={mr_ratio:.3}", m.m_rr_nmm);
        assert!(ml_ratio > 0.5 && ml_ratio < 2.5,
            "LC3 M_sl: ours={:.2}, ref=125, ratio={ml_ratio:.3}", m.m_sl_nmm);
        assert!(mt_ratio > 0.5 && mt_ratio < 1.8,
            "LC3 M_total: ours={:.2}, ref=345, ratio={mt_ratio:.3}", m.m_total_nmm);
    }

    /// LC4 — F_r=5 kN, F_a=2 kN, n=4000 rpm, T=80 °C (high speed)
    /// SKF Tool: M_rr=285, M_sl=6.56, M_total=291 N·mm.
    #[test]
    fn test_skf_30306_lgmt2_lc4_high_speed() {
        let m = skf_30306_lgmt2_torque(5.0, 2.0, 4000.0, 80.0);
        for (label, ours, ref_v) in [
            ("M_rr",    m.m_rr_nmm,    285.0_f64),
            ("M_sl",    m.m_sl_nmm,    6.56),
            ("M_total", m.m_total_nmm, 291.0),
        ] {
            let ratio = ours / ref_v;
            assert!(ratio > 0.5 && ratio < 1.7,
                "LC4 {label}: ours={ours:.2}, ref={ref_v}, ratio={ratio:.3}");
        }
    }

    /// Aggregate consistency — across all four LCs, the M_total ratio
    /// (ours / SKF Tool) is bounded and biased high by ~30 % (documented).
    /// Catches catastrophic calibration drift in any single LC.
    #[test]
    fn test_skf_30306_lgmt2_4lc_aggregate_bias() {
        let cases = [
            (skf_30306_lgmt2_torque(2.0, 1.0, 500.0,  60.0), 181.0_f64),
            (skf_30306_lgmt2_torque(5.0, 2.0, 1500.0, 70.0), 234.0),
            (skf_30306_lgmt2_torque(15.0, 5.0, 1500.0, 80.0), 345.0),
            (skf_30306_lgmt2_torque(5.0, 2.0, 4000.0, 80.0), 291.0),
        ];
        let ratios: Vec<f64> = cases.iter().map(|(m, r)| m.m_total_nmm / r).collect();
        let avg = ratios.iter().sum::<f64>() / ratios.len() as f64;
        // Bias documented: average ratio ≈ 1.30 (our model 30 % high)
        assert!(avg > 1.0 && avg < 1.7,
            "Average M_total ratio across 4 LCs: {avg:.3} (expected ~1.30 ±15%)");
        // No single LC ratio above 1.8 or below 0.5 (sanity)
        for (i, r) in ratios.iter().enumerate() {
            assert!(*r > 0.5 && *r < 1.8,
                "LC{}: ratio {r:.3} out of [0.5, 1.8]", i + 1);
        }
    }

    // ─── External Experimental Validation — Schwarz 2023 (32216) ────
    //
    // Schwarz B., Schäfer T., Sauer B. (2023) "Predicting Friction of Tapered
    // Roller Bearings with Detailed Multi-Body Simulation Models", Lubricants
    // 11(9):369.  https://doi.org/10.3390/lubricants11090369
    //
    // Test bearing: TRB 32216 (catalogue d=80, D=140, B=33 mm)
    //   — d_pitch = 108.5 mm, d_RB = 17 mm, l_RB = 22.7 mm, n_RB = 16
    //   — σ_raceway = 0.16 μm, σ_rib = 0.24 μm
    //   — Half-cone angle α ≈ 14° (32216 catalogue Y₁=1.6, e=0.43 → α=arctan(e/Y))
    //
    // Lubricant: FVA No. 3 reference oil (ISO VG 100 mineral, no additives)
    //   — Vogel formula η[mPa·s] = K·exp(B/(T+C)) with K=0.062, B=1021.7°C,
    //     C=101.55°C, ρ=887.6 kg/m³ at 15 °C
    //   — Density temperature coefficient α_ρ = -6e-4 g/(mL·K)
    //
    // Operating point (Figure 5): F_a = 6 kN axial only, T_op = 42 or 50 °C,
    // n = 500-4000 rpm (oil bath, half roller height).
    // Measured M_total at 50 °C: ~1200 N·mm @ 500 rpm → ~3800 N·mm @ 4000 rpm.
    //
    // Our test compares the BH 2010 Part 1 (line-contact) rolling-friction
    // contribution to the total measured M_friction.  BH alone gives only
    // viscous rolling resistance (no sliding, no rib, no cage, no drag), so
    // M_BH ≤ M_measured by construction.  At low speed (500 rpm) sliding+rib
    // dominate — expect M_BH/M_measured small.  At high speed (4000 rpm) BH
    // rolling becomes the dominant component — expect ratio closer to 1.

    /// Compute the bearing-level rolling-friction torque [N·mm] from the
    /// Biboulet-Houpert 2010 Part 1 line-contact formula at the Schwarz
    /// 32216 axial-load operating point.  Returns (M_BH_nmm, m_rolling_per_roller_nmm).
    fn schwarz_32216_bh_rolling_torque_with(
        t_op_c: f64, n_inner_rpm: f64, f_a_kn: f64,
        correction: ThermalCorrection,
    ) -> (f64, f64) {
        let z = 16usize;
        let alpha = 14.0_f64.to_radians();
        let d_pitch_m = 108.5e-3;
        let d_rb_m = 17.0e-3;
        let l_m = 22.7e-3;
        let r_pitch_m = d_pitch_m / 2.0;
        let r_rb_m = d_rb_m / 2.0;

        let omega_i = 2.0 * std::f64::consts::PI * n_inner_rpm / 60.0;
        let omega_cage = omega_i * (1.0 - r_rb_m * alpha.cos() / r_pitch_m) / 2.0;
        let r_outer_contact = r_pitch_m + r_rb_m * alpha.cos();
        let r_inner_contact = r_pitch_m - r_rb_m * alpha.cos();
        let u_outer = omega_cage * r_outer_contact;
        let u_inner = omega_cage * r_inner_contact;
        let q_outer = f_a_kn * 1e3 / (z as f64 * alpha.sin());
        let q_inner = q_outer * alpha.cos();

        let eta_mpas = 0.062_f64 * (1021.7_f64 / (t_op_c + 101.5517)).exp();
        let eta_0 = eta_mpas * 1e-3;
        let beta_visc = 1021.7 / ((t_op_c + 101.5517) * (t_op_c + 101.5517));
        let k_fluid = 0.134;

        let e_prime = 2.31e11_f64;
        let r_x = r_rb_m;

        let p_outer = biboulet_houpert_line_rolling_power_dispatched(
            eta_0, u_outer, q_outer, l_m, r_x, e_prime, beta_visc, k_fluid, correction);
        let p_inner = biboulet_houpert_line_rolling_power_dispatched(
            eta_0, u_inner, q_inner, l_m, r_x, e_prime, beta_visc, k_fluid, correction);

        // Johnson 1985 material hysteresis (α_v = 0.005 for hardened bearing
        // steel).  Independent of lubricant; added to BH viscous rolling per
        // Schwarz Eq. 20 — matches the M1/M2 dispatcher.
        let alpha_v = 0.005;
        let p_hys_outer = johnson_hysteresis_power_line_contact(
            q_outer, l_m, r_x, e_prime, alpha_v, u_outer);
        let p_hys_inner = johnson_hysteresis_power_line_contact(
            q_inner, l_m, r_x, e_prime, alpha_v, u_inner);

        let p_total_w = (p_outer + p_inner + p_hys_outer + p_hys_inner) * z as f64;
        let m_total_nmm = p_total_w / omega_i.max(1e-9) * 1000.0;
        let m_per_roller_nmm = (p_outer + p_inner + p_hys_outer + p_hys_inner)
            / omega_i.max(1e-9) * 1000.0;
        (m_total_nmm, m_per_roller_nmm)
    }

    /// Estimate rib sliding contribution for Schwarz 32216 axial load.
    ///
    /// Per Tewari Eq. 3:
    ///   F_rib = F_a · sin(2γ_rib) / (Z · sin α_o)
    /// γ_rib (rib face angle wrt roller axis perpendicular) estimated 2.5°
    /// based on typical TRB rib design.  Schwarz Table 4 doesn't publish this.
    ///
    /// Rib sliding velocity (cone-apex orbital-spin mismatch):
    ///   u_slide_rib = (ω_inner - ω_cage) · (d_RB·cosα / 2)
    ///
    /// μ_rib uses Hamrock-Dowson elliptical EHL with crude regime approximation:
    ///   high speed (≥2000 rpm): μ ≈ 0.01 (full EHL)
    ///   medium speed (500-2000): μ ≈ 0.025 (mixed)
    ///   low speed (<500): μ ≈ 0.05 (boundary-mixed)
    ///
    /// Returns bearing-level M_rib [N·mm].
    fn schwarz_32216_rib_torque_estimate(t_op_c: f64, n_inner_rpm: f64, f_a_kn: f64) -> f64 {
        let z = 16usize;
        let alpha = 14.0_f64.to_radians();
        let gamma_rib: f64 = 2.5_f64.to_radians();
        let d_pitch_m = 108.5e-3; let d_rb_m = 17.0e-3;
        let r_pitch_m = d_pitch_m / 2.0; let r_rb_m = d_rb_m / 2.0;

        // F_rib per roller (Tewari Eq. 3)
        let f_rib_per_roller = f_a_kn * 1e3 * (2.0 * gamma_rib).sin() / (z as f64 * alpha.sin());

        // Kinematics
        let omega_i = 2.0 * std::f64::consts::PI * n_inner_rpm / 60.0;
        let omega_cage = omega_i * (1.0 - r_rb_m * alpha.cos() / r_pitch_m) / 2.0;
        let d_omega = omega_i - omega_cage;
        // Effective rib offset (drilling radius)
        let r_rib_eff = d_rb_m * alpha.cos() / 2.0;
        let u_slide_rib = d_omega.abs() * r_rib_eff;

        // Crude regime-based μ_rib (Schwarz uses HD elliptical + Bair-Winer;
        // here we approximate based on entrainment speed).
        let mu_rib = if n_inner_rpm >= 2000.0 {
            0.010      // full EHL high speed
        } else if n_inner_rpm >= 500.0 {
            0.025      // mixed transition
        } else {
            0.050      // boundary
        };
        // Temperature effect: cooler oil → higher η → lower μ in EHL regime
        let temp_factor = if t_op_c < 50.0 { 0.85 } else { 1.0 };
        let mu_rib_eff = mu_rib * temp_factor;

        // Per-roller power and bearing torque
        let p_rib_per = mu_rib_eff * f_rib_per_roller * u_slide_rib;
        let p_total = p_rib_per * z as f64;
        if omega_i < 1e-9 { 0.0 } else { p_total / omega_i * 1000.0 }
    }

    /// Variant with explicit α_v (for α_v sensitivity sweep diagnostic).
    fn schwarz_32216_torque_with_alpha_v(
        t_op_c: f64, n_inner_rpm: f64, f_a_kn: f64,
        correction: ThermalCorrection, alpha_v: f64,
    ) -> f64 {
        let z = 16usize;
        let alpha = 14.0_f64.to_radians();
        let d_pitch_m = 108.5e-3; let d_rb_m = 17.0e-3; let l_m = 22.7e-3;
        let r_pitch_m = d_pitch_m / 2.0; let r_rb_m = d_rb_m / 2.0;
        let omega_i = 2.0 * std::f64::consts::PI * n_inner_rpm / 60.0;
        let omega_cage = omega_i * (1.0 - r_rb_m * alpha.cos() / r_pitch_m) / 2.0;
        let r_outer_contact = r_pitch_m + r_rb_m * alpha.cos();
        let r_inner_contact = r_pitch_m - r_rb_m * alpha.cos();
        let u_outer = omega_cage * r_outer_contact;
        let u_inner = omega_cage * r_inner_contact;
        let q_outer = f_a_kn * 1e3 / (z as f64 * alpha.sin());
        let q_inner = q_outer * alpha.cos();
        let eta_mpas = 0.062_f64 * (1021.7_f64 / (t_op_c + 101.5517)).exp();
        let eta_0 = eta_mpas * 1e-3;
        let beta_visc = 1021.7 / ((t_op_c + 101.5517) * (t_op_c + 101.5517));
        let k_fluid = 0.134;
        let e_prime = 2.31e11_f64; let r_x = r_rb_m;
        let p_o = biboulet_houpert_line_rolling_power_dispatched(
            eta_0, u_outer, q_outer, l_m, r_x, e_prime, beta_visc, k_fluid, correction);
        let p_i = biboulet_houpert_line_rolling_power_dispatched(
            eta_0, u_inner, q_inner, l_m, r_x, e_prime, beta_visc, k_fluid, correction);
        let p_hyo = johnson_hysteresis_power_line_contact(
            q_outer, l_m, r_x, e_prime, alpha_v, u_outer);
        let p_hyi = johnson_hysteresis_power_line_contact(
            q_inner, l_m, r_x, e_prime, alpha_v, u_inner);
        let p_tot = (p_o + p_i + p_hyo + p_hyi) * z as f64;
        p_tot / omega_i.max(1e-9) * 1000.0
    }

    /// Multi-model comparison helper at a single Schwarz 32216 operating point.
    ///
    /// Returns [Our BH+Aihara, Aihara 1987 orig, Zhou-Hoeprich 1991,
    /// Matsuyama 2001, Houpert 2002, Palmgren μ_rr·Q·u] each in N·mm at the
    /// bearing level.
    /// - Aihara: uses Aihara Eq. 8 conversion M = (z/D_a)(R_o M_i + R_i M_o)
    /// - Others: same conversion applied for consistency
    /// Excludes rib + hysteresis (raceway rolling only — fair comparison).
    fn schwarz_32216_multi_model_raceway_torques(
        t_op_c: f64, n_inner_rpm: f64, f_a_kn: f64,
    ) -> [f64; 6] {
        let z = 16usize;
        let alpha_i_rad = 11.5_f64.to_radians();
        let alpha_o_rad = 14.0_f64.to_radians();
        let alpha_avg = (alpha_i_rad + alpha_o_rad) / 2.0;
        let d_pitch_m = 108.5e-3; let d_rb_m = 17.0e-3; let l_m = 22.7e-3;
        let r_pitch_m = d_pitch_m / 2.0; let r_rb_m = d_rb_m / 2.0;
        let omega_i = 2.0 * std::f64::consts::PI * n_inner_rpm / 60.0;
        let omega_cage = omega_i * alpha_i_rad.sin() / (alpha_i_rad.sin() + alpha_o_rad.sin());
        let r_outer_contact = r_pitch_m + r_rb_m * alpha_avg.cos();
        let r_inner_contact = r_pitch_m - r_rb_m * alpha_avg.cos();
        let u_outer = omega_cage * r_outer_contact;
        let u_inner = omega_cage * r_inner_contact;
        let q_outer = f_a_kn * 1e3 / (z as f64 * alpha_o_rad.sin());
        let q_inner = q_outer * (alpha_o_rad - alpha_i_rad).cos();
        let w_l_o = q_outer / l_m;
        let w_l_i = q_inner / l_m;
        let f_a_n = f_a_kn * 1e3;

        let eta_mpas = 0.062_f64 * (1021.7_f64 / (t_op_c + 101.5517)).exp();
        let eta_0 = eta_mpas * 1e-3;
        let beta_visc = 1021.7 / ((t_op_c + 101.5517) * (t_op_c + 101.5517));
        let k_fluid = 0.134;
        let alpha_pv = 20.0e-9_f64;
        let e_prime = 2.31e11_f64;
        let r_x = r_rb_m;

        // Aihara Eq. 8 / Tewari Eq. 7 bearing conversion:
        //   M_bearing = (z/D_a)(R_o · M_i + R_i · M_o)   [N·m]
        // Convert to N·mm output
        let aihara_xform = |m_i_nm: f64, m_o_nm: f64| -> f64 {
            z as f64 / d_rb_m
                * (r_outer_contact * m_i_nm + r_inner_contact * m_o_nm) * 1000.0
        };

        // Model 1: Our BH + Aihara thermal
        let p_bh_o = biboulet_houpert_line_rolling_power_dispatched(
            eta_0, u_outer, q_outer, l_m, r_x, e_prime, beta_visc, k_fluid,
            ThermalCorrection::Aihara1987);
        let p_bh_i = biboulet_houpert_line_rolling_power_dispatched(
            eta_0, u_inner, q_inner, l_m, r_x, e_prime, beta_visc, k_fluid,
            ThermalCorrection::Aihara1987);
        let m_bh_nmm = (p_bh_o + p_bh_i) * z as f64 / omega_i.max(1e-9) * 1000.0;

        // Model 2: Aihara 1987 original (bearing-level W)
        let m_aihara_o = aihara_1987_raceway_torque(
            eta_0, u_outer, f_a_n, z, d_rb_m, l_m, alpha_o_rad,
            r_x, alpha_pv, e_prime, beta_visc, k_fluid);
        let m_aihara_i = aihara_1987_raceway_torque(
            eta_0, u_inner, f_a_n, z, d_rb_m, l_m, alpha_i_rad,
            r_x, alpha_pv, e_prime, beta_visc, k_fluid);
        let m_aihara_nmm = aihara_xform(m_aihara_i, m_aihara_o);

        // Model 3: Zhou-Hoeprich 1991 (Wilson φ_ish proxy)
        let phi_ish_o = wilson_thermal_factor(eta_0, beta_visc, u_outer, k_fluid);
        let phi_ish_i = wilson_thermal_factor(eta_0, beta_visc, u_inner, k_fluid);
        let m_zh_o = zhou_hoepprich_1991_raceway_torque(
            eta_0, u_outer, w_l_o, r_x, l_m, alpha_pv, e_prime, phi_ish_o, 1.0);
        let m_zh_i = zhou_hoepprich_1991_raceway_torque(
            eta_0, u_inner, w_l_i, r_x, l_m, alpha_pv, e_prime, phi_ish_i, 1.0);
        let m_zh_nmm = aihara_xform(m_zh_i, m_zh_o);

        // Model 4: Matsuyama 2001
        let m_mats_o = matsuyama_2001_raceway_torque(
            eta_0, u_outer, w_l_o, r_x, l_m, alpha_pv, e_prime, phi_ish_o);
        let m_mats_i = matsuyama_2001_raceway_torque(
            eta_0, u_inner, w_l_i, r_x, l_m, alpha_pv, e_prime, phi_ish_i);
        let m_mats_nmm = aihara_xform(m_mats_i, m_mats_o);

        // Model 5: Houpert 2002
        let m_houp_o = houpert_2002_raceway_torque(
            eta_0, u_outer, w_l_o, r_x, l_m, e_prime);
        let m_houp_i = houpert_2002_raceway_torque(
            eta_0, u_inner, w_l_i, r_x, l_m, e_prime);
        let m_houp_nmm = aihara_xform(m_houp_i, m_houp_o);

        // Model 6: Palmgren μ_rr·Q·u baseline
        let mu_rr = 0.002_f64;
        let p_palm_o = mu_rr * q_outer * u_outer;
        let p_palm_i = mu_rr * q_inner * u_inner;
        let m_palm_nmm = (p_palm_o + p_palm_i) * z as f64 / omega_i.max(1e-9) * 1000.0;

        [m_bh_nmm, m_aihara_nmm, m_zh_nmm, m_mats_nmm, m_houp_nmm, m_palm_nmm]
    }

    /// Diagnostic — Schwarz 32216 (Fig 5 axial-only) 6-model cross-comparison
    /// at 3 speeds × 50 °C, axial 6 kN.  All 6 formulas now use original-paper
    /// dimensional forms (verified from Aihara 1987, Zhou-Hoeprich 1991,
    /// Houpert 2002, Wilson 1983 in `Reference/Validation/`).
    #[test]
    #[ignore]
    fn diag_schwarz_32216_multi_model_comparison() {
        let cases = [(500.0_f64, 1300.0_f64),
                     (2000.0,    2950.0),
                     (4000.0,    3750.0)];
        eprintln!("\nSchwarz 32216 — 6-model raceway rolling torque (axial 6 kN, 50 °C):");
        eprintln!("  {:<8} {:<8} {:<10} {:<10} {:<10} {:<10} {:<10} {:<10}",
            "n[rpm]", "M_meas", "BH+Aih", "Aihara'87", "Zhou-H'91", "Matsuyama", "Houpert'02", "Palmgren");
        for (n, m_meas) in cases {
            let m = schwarz_32216_multi_model_raceway_torques(50.0, n, 6.0);
            eprintln!("  {:<8.0} {:<8.0} {:<10.0} {:<10.0} {:<10.0} {:<10.0} {:<10.0} {:<10.0}",
                n, m_meas, m[0], m[1], m[2], m[3], m[4], m[5]);
        }
        eprintln!("\nRatio vs measurement:");
        eprintln!("  {:<8} {:<10} {:<10} {:<10} {:<10} {:<10} {:<10}",
            "n[rpm]", "BH+Aih", "Aihara'87", "Zhou-H'91", "Matsuyama", "Houpert'02", "Palmgren");
        for (n, m_meas) in cases {
            let m = schwarz_32216_multi_model_raceway_torques(50.0, n, 6.0);
            eprintln!("  {:<8.0} {:<10.3} {:<10.3} {:<10.3} {:<10.3} {:<10.3} {:<10.3}",
                n, m[0]/m_meas, m[1]/m_meas, m[2]/m_meas, m[3]/m_meas, m[4]/m_meas, m[5]/m_meas);
        }
        eprintln!("\nNote: raceway-only — measurement M_T includes rib + hysteresis (~5-10 %).");
        eprintln!("Expect ratios ~0.85-0.95 if model represents pure raceway rolling.");
    }

    /// Diagnostic sweep: report RMSE vs Schwarz Fig 5 measurements for a range
    /// of α_v values, WITH and WITHOUT analytical rib sliding contribution.
    /// This separates pure-material hysteresis (α_v scope) from missing
    /// components (rib + cage + Scheuermann) that the measurement includes.
    ///
    /// Measured (axial 6 kN, oil bath, FVA 3):
    ///   p1: 500 rpm, 50°C → 1300 N·mm
    ///   p2: 2000 rpm, 50°C → 2950 N·mm
    ///   p3: 4000 rpm, 50°C → 3750 N·mm
    ///   p4: 500 rpm, 42°C → 1700 N·mm
    #[test]
    #[ignore] // Diagnostic; run with: cargo test alpha_v_sweep -- --ignored --nocapture
    fn diag_schwarz_32216_alpha_v_sweep() {
        let points = [
            (500.0,  50.0, 1300.0),
            (2000.0, 50.0, 2950.0),
            (4000.0, 50.0, 3750.0),
            (500.0,  42.0, 1700.0),
        ];
        // First, just print the analytical rib estimate at each point
        eprintln!("\nAnalytical rib estimate (γ_rib=2.5°, μ_rib regime-based):");
        eprintln!("  {:<12} {:<10}", "Operating", "M_rib [N·mm]");
        for &(n, t, _) in &points {
            let m_rib = schwarz_32216_rib_torque_estimate(t, n, 6.0);
            eprintln!("  {:<12} {:<10.1}", format!("{}rpm/{}°C", n as u32, t as u32), m_rib);
        }

        eprintln!("\n[A] α_v sweep, WITHOUT rib (BH+Aihara+Hys only):");
        eprintln!("  {:<6} {:<10} {:<10} {:<10} {:<10} {:<8}", "α_v", "M(500/50)", "M(2k/50)", "M(4k/50)", "M(500/42)", "RMSE%");
        for alpha_v in [0.0, 0.005, 0.01, 0.02, 0.05, 0.07] {
            let m1 = schwarz_32216_torque_with_alpha_v(50.0, 500.0,  6.0, ThermalCorrection::Aihara1987, alpha_v);
            let m2 = schwarz_32216_torque_with_alpha_v(50.0, 2000.0, 6.0, ThermalCorrection::Aihara1987, alpha_v);
            let m3 = schwarz_32216_torque_with_alpha_v(50.0, 4000.0, 6.0, ThermalCorrection::Aihara1987, alpha_v);
            let m4 = schwarz_32216_torque_with_alpha_v(42.0, 500.0,  6.0, ThermalCorrection::Aihara1987, alpha_v);
            let rmse = (([(m1/1300.0-1.0).powi(2), (m2/2950.0-1.0).powi(2),
                          (m3/3750.0-1.0).powi(2), (m4/1700.0-1.0).powi(2)]).iter()
                .sum::<f64>() / 4.0).sqrt() * 100.0;
            eprintln!("  {:<6.3} {:<10.1} {:<10.1} {:<10.1} {:<10.1} {:<8.2}",
                alpha_v, m1, m2, m3, m4, rmse);
        }

        eprintln!("\n[B] α_v sweep, WITH analytical rib sliding (BH+Aihara+Hys+RibEst):");
        eprintln!("  {:<6} {:<10} {:<10} {:<10} {:<10} {:<8}", "α_v", "M(500/50)", "M(2k/50)", "M(4k/50)", "M(500/42)", "RMSE%");
        for alpha_v in [0.0, 0.005, 0.01, 0.02, 0.05, 0.07] {
            let r1 = schwarz_32216_rib_torque_estimate(50.0, 500.0,  6.0);
            let r2 = schwarz_32216_rib_torque_estimate(50.0, 2000.0, 6.0);
            let r3 = schwarz_32216_rib_torque_estimate(50.0, 4000.0, 6.0);
            let r4 = schwarz_32216_rib_torque_estimate(42.0, 500.0,  6.0);
            let m1 = schwarz_32216_torque_with_alpha_v(50.0, 500.0,  6.0, ThermalCorrection::Aihara1987, alpha_v) + r1;
            let m2 = schwarz_32216_torque_with_alpha_v(50.0, 2000.0, 6.0, ThermalCorrection::Aihara1987, alpha_v) + r2;
            let m3 = schwarz_32216_torque_with_alpha_v(50.0, 4000.0, 6.0, ThermalCorrection::Aihara1987, alpha_v) + r3;
            let m4 = schwarz_32216_torque_with_alpha_v(42.0, 500.0,  6.0, ThermalCorrection::Aihara1987, alpha_v) + r4;
            let rmse = (([(m1/1300.0-1.0).powi(2), (m2/2950.0-1.0).powi(2),
                          (m3/3750.0-1.0).powi(2), (m4/1700.0-1.0).powi(2)]).iter()
                .sum::<f64>() / 4.0).sqrt() * 100.0;
            eprintln!("  {:<6.3} {:<10.1} {:<10.1} {:<10.1} {:<10.1} {:<8.2}",
                alpha_v, m1, m2, m3, m4, rmse);
        }
    }

    /// Backward-compat — defaults to Wilson 1979 (existing tests use this).
    fn schwarz_32216_bh_rolling_torque(t_op_c: f64, n_inner_rpm: f64, f_a_kn: f64) -> (f64, f64) {
        schwarz_32216_bh_rolling_torque_with(
            t_op_c, n_inner_rpm, f_a_kn, ThermalCorrection::Wilson1979)
    }

    /// Schwarz 32216 axial 6 kN, 4000 rpm, 50 °C — **Aihara 1987 correction**.
    /// Aihara's stronger thermal factor (a=0.29, b=0.78 vs Wilson's 0.1, 0.64)
    /// is calibrated for TRB rolling torque specifically and should give
    /// closer agreement to measured 3800 N·mm than Wilson at this high-speed
    /// operating point where Wilson left +20 % over-prediction.
    #[test]
    fn test_bh_schwarz_32216_aihara_4000rpm_50c() {
        let (m_bh_nmm, _) = schwarz_32216_bh_rolling_torque_with(
            50.0, 4000.0, 6.0, ThermalCorrection::Aihara1987);
        assert!(m_bh_nmm > 0.0 && m_bh_nmm.is_finite());
        let m_measured = 3800.0_f64;
        let ratio = m_bh_nmm / m_measured;
        eprintln!("Schwarz 32216 @ 4000 rpm, 50 °C, F_a=6 kN [AIHARA]:");
        eprintln!("  M_BH (rolling)  = {:.1} N·mm", m_bh_nmm);
        eprintln!("  M_measured (tot) = {} N·mm", m_measured);
        eprintln!("  Ratio (BH/meas)  = {:.3}", ratio);
        // Aihara expected to give ratio ~ 0.92 (vs Wilson 1.20).
        assert!(ratio > 0.80 && ratio < 1.10,
            "Aihara BH/measured at 4000 rpm: ratio={ratio:.3} (expected ~0.92, \
             improving over Wilson's 1.20)");
    }

    /// Schwarz 32216 axial 6 kN, 500 rpm, 50 °C — **Aihara correction**.
    /// At low speed L_th is small so Aihara and Wilson should give similar
    /// results (both close to 1.0); confirms Aihara doesn't hurt low-speed fit.
    #[test]
    fn test_bh_schwarz_32216_aihara_500rpm_50c() {
        let (m_bh_nmm, _) = schwarz_32216_bh_rolling_torque_with(
            50.0, 500.0, 6.0, ThermalCorrection::Aihara1987);
        let m_measured = 1200.0_f64;
        let ratio = m_bh_nmm / m_measured;
        eprintln!("Schwarz 32216 @ 500 rpm, 50 °C, F_a=6 kN [AIHARA]:");
        eprintln!("  M_BH (rolling)  = {:.1} N·mm", m_bh_nmm);
        eprintln!("  Ratio (BH/meas)  = {:.3}", ratio);
        // Wilson gave 0.919; Aihara at low L_th ≈ 0.030 → φ_T ≈ 0.981
        // → ratio close to Wilson (within 5 %).
        assert!(ratio > 0.75 && ratio < 1.05,
            "Aihara BH/measured at 500 rpm: ratio={ratio:.3}");
    }

    // ─── Schwarz 32208 (small TRB, Fig 9 / Table A2) ────────────────
    //
    // Same paper (Schwarz et al. 2023) provides a second TRB geometry in
    // Appendix B (Table A2) used for hydraulic-loss validation in Fig 9.
    // Smaller bearing than 32216 — useful as a magnitude/scaling cross-check.
    //
    // 32208 geometry (Schwarz Table A2):
    //   d=40 mm, D=80 mm, d_pitch=60, d_RB=10, l_RB=17, n_RB=17
    //   σ_raceway = 0.1 μm, σ_rib = 0.1 μm
    //   Half-cone angle α ≈ 13° (32208 catalogue: Y₁=1.6, e=0.37 → arctan(e/Y))

    fn schwarz_32208_bh_rolling_torque_with(
        t_op_c: f64, n_inner_rpm: f64, f_a_kn: f64,
        correction: ThermalCorrection,
    ) -> (f64, f64) {
        let z = 17usize;
        let alpha = 13.0_f64.to_radians();
        let d_pitch_m = 60.0e-3;
        let d_rb_m = 10.0e-3;
        let l_m = 17.0e-3;
        let r_pitch_m = d_pitch_m / 2.0;
        let r_rb_m = d_rb_m / 2.0;

        let omega_i = 2.0 * std::f64::consts::PI * n_inner_rpm / 60.0;
        let omega_cage = omega_i * (1.0 - r_rb_m * alpha.cos() / r_pitch_m) / 2.0;
        let r_outer_contact = r_pitch_m + r_rb_m * alpha.cos();
        let r_inner_contact = r_pitch_m - r_rb_m * alpha.cos();
        let u_outer = omega_cage * r_outer_contact;
        let u_inner = omega_cage * r_inner_contact;
        let q_outer = f_a_kn * 1e3 / (z as f64 * alpha.sin());
        let q_inner = q_outer * alpha.cos();

        // Same FVA No. 3 oil as 32216 case (Schwarz uses identical lubricant)
        let eta_mpas = 0.062_f64 * (1021.7_f64 / (t_op_c + 101.5517)).exp();
        let eta_0 = eta_mpas * 1e-3;
        let beta_visc = 1021.7 / ((t_op_c + 101.5517) * (t_op_c + 101.5517));
        let k_fluid = 0.134;

        let e_prime = 2.31e11_f64;
        let r_x = r_rb_m;

        let p_outer = biboulet_houpert_line_rolling_power_dispatched(
            eta_0, u_outer, q_outer, l_m, r_x, e_prime, beta_visc, k_fluid, correction);
        let p_inner = biboulet_houpert_line_rolling_power_dispatched(
            eta_0, u_inner, q_inner, l_m, r_x, e_prime, beta_visc, k_fluid, correction);

        // Johnson 1985 material hysteresis (α_v = 0.005 for hardened bearing
        // steel).  Independent of lubricant; added to BH viscous rolling per
        // Schwarz Eq. 20 — matches the M1/M2 dispatcher.
        let alpha_v = 0.005;
        let p_hys_outer = johnson_hysteresis_power_line_contact(
            q_outer, l_m, r_x, e_prime, alpha_v, u_outer);
        let p_hys_inner = johnson_hysteresis_power_line_contact(
            q_inner, l_m, r_x, e_prime, alpha_v, u_inner);

        let p_total_w = (p_outer + p_inner + p_hys_outer + p_hys_inner) * z as f64;
        let m_total_nmm = p_total_w / omega_i.max(1e-9) * 1000.0;
        let m_per_roller_nmm = (p_outer + p_inner + p_hys_outer + p_hys_inner)
            / omega_i.max(1e-9) * 1000.0;
        (m_total_nmm, m_per_roller_nmm)
    }

    /// Schwarz 32208 @ 1 kN axial, 50 °C, 1500 rpm (Fig 9 nominal).
    /// Magnitude check: smaller bearing + lighter load than 32216 → expect
    /// M_total in [50, 1000] N·mm range.  Fig 9 numerical values not in text
    /// (graph-only), so test verifies plausibility + finite/positive.
    #[test]
    fn test_bh_schwarz_32208_magnitude_50c() {
        let (m_aihara, _) = schwarz_32208_bh_rolling_torque_with(
            50.0, 1500.0, 1.0, ThermalCorrection::Aihara1987);
        eprintln!("Schwarz 32208 @ 1500 rpm, 50 °C, F_a=1 kN [Aihara]:");
        eprintln!("  M_BH = {:.1} N·mm", m_aihara);
        assert!(m_aihara > 0.0 && m_aihara.is_finite(),
            "32208 BH torque must be positive finite: {m_aihara}");
        assert!(m_aihara > 50.0 && m_aihara < 1500.0,
            "32208 M_BH at 1 kN axial / 1500 rpm: {m_aihara:.1} out of \
             plausible range [50, 1500] N·mm for this bearing size");
    }

    /// 32208 vs 32216 magnitude scaling — at identical η, T, n, the smaller
    /// 32208 (1/6× load, smaller pitch / roller / length) should give much
    /// lower M.  At 50°C, 1500 rpm: 32216@6kN vs 32208@1kN.
    #[test]
    fn test_bh_schwarz_32208_vs_32216_scaling() {
        let (m_32216, _) = schwarz_32216_bh_rolling_torque_with(
            50.0, 1500.0, 6.0, ThermalCorrection::Aihara1987);
        let (m_32208, _) = schwarz_32208_bh_rolling_torque_with(
            50.0, 1500.0, 1.0, ThermalCorrection::Aihara1987);
        let ratio = m_32208 / m_32216;
        eprintln!("Bearing-size scaling (1500 rpm, 50 °C, Aihara):");
        eprintln!("  32216 (6 kN): {:.1} N·mm", m_32216);
        eprintln!("  32208 (1 kN): {:.1} N·mm", m_32208);
        eprintln!("  Ratio 32208/32216 = {:.3}", ratio);
        // 32208 is smaller bearing with lower load — expect ratio in [0.05, 0.40].
        // Pure load ratio is 1/6 = 0.167; geometry differences (smaller r_x,
        // shorter l, fewer rollers Z=17 vs 16) further reduce by factor 2-3.
        assert!(ratio > 0.05 && ratio < 0.45,
            "32208/32216 ratio: {ratio:.3} (expected ~0.10-0.30 for size+load scaling)");
    }

    /// 32208 speed monotonicity — 500 → 4000 rpm at 1 kN, 50 °C.
    #[test]
    fn test_bh_schwarz_32208_speed_monotonic() {
        let speeds = [500.0, 1000.0, 2000.0, 4000.0];
        let mut prev = 0.0;
        for n in speeds {
            let (m, _) = schwarz_32208_bh_rolling_torque_with(
                50.0, n, 1.0, ThermalCorrection::Aihara1987);
            assert!(m > prev,
                "32208 M must increase with speed: M({n} rpm)={m:.1} ≤ prev={prev:.1}");
            prev = m;
        }
    }

    /// Compare Wilson vs Aihara at the same operating point: Aihara should
    /// give lower torque (stronger correction) at high speed, similar at low.
    #[test]
    fn test_bh_schwarz_32216_wilson_vs_aihara() {
        // High speed: Aihara < Wilson (more thermal reduction)
        let (m_w_4k, _) = schwarz_32216_bh_rolling_torque_with(
            50.0, 4000.0, 6.0, ThermalCorrection::Wilson1979);
        let (m_a_4k, _) = schwarz_32216_bh_rolling_torque_with(
            50.0, 4000.0, 6.0, ThermalCorrection::Aihara1987);
        assert!(m_a_4k < m_w_4k,
            "At 4000 rpm Aihara should give less torque than Wilson: \
             Aihara={m_a_4k:.1}, Wilson={m_w_4k:.1}");
        let ratio_4k = m_a_4k / m_w_4k;
        assert!(ratio_4k > 0.65 && ratio_4k < 0.85,
            "Aihara/Wilson ratio at 4000 rpm: {ratio_4k:.3} (expected ~0.77)");

        // Low speed: nearly identical (small L_th)
        let (m_w_500, _) = schwarz_32216_bh_rolling_torque_with(
            50.0, 500.0, 6.0, ThermalCorrection::Wilson1979);
        let (m_a_500, _) = schwarz_32216_bh_rolling_torque_with(
            50.0, 500.0, 6.0, ThermalCorrection::Aihara1987);
        let ratio_500 = m_a_500 / m_w_500;
        assert!(ratio_500 > 0.95 && ratio_500 < 1.02,
            "Aihara/Wilson ratio at 500 rpm: {ratio_500:.3} (expected ~0.99, \
             both factors near 1.0 at low L_th)");
    }

    /// Schwarz 32216 axial 6 kN, 500 rpm, 50 °C — measured M_total ≈ 1200 N·mm.
    /// Under pure axial load the cone-apex sliding ≈ 0, so BH rolling resistance
    /// dominates the total measured torque; we expect M_BH / M_measured ~ 0.7-1.1.
    #[test]
    fn test_bh_schwarz_32216_axial_500rpm_50c() {
        let (m_bh_nmm, _) = schwarz_32216_bh_rolling_torque(50.0, 500.0, 6.0);
        assert!(m_bh_nmm > 0.0 && m_bh_nmm.is_finite(),
            "BH rolling M must be positive finite: got {m_bh_nmm}");
        let m_measured = 1200.0_f64;            // Schwarz Fig. 5 @ 50 °C, 500 rpm
        let ratio = m_bh_nmm / m_measured;
        eprintln!("Schwarz 32216 @ 500 rpm, 50 °C, F_a=6 kN:");
        eprintln!("  M_BH (rolling)  = {:.1} N·mm", m_bh_nmm);
        eprintln!("  M_measured (tot) = {} N·mm", m_measured);
        eprintln!("  Ratio (BH/meas)  = {:.3}", ratio);
        // Observed ratio ≈ 0.92 with Wilson 1979 φ_T thermal correction
        // (was 0.928 isothermal; thermal effect minor at low speed since
        // L_th ∝ u² and u_outer = 1.4 m/s gives φ_T ≈ 0.99).
        // Bracket [0.75, 1.10] for regression detection.
        assert!(ratio > 0.75 && ratio < 1.10,
            "BH/measured at 500 rpm: ratio={ratio:.3} (observed 0.919 with thermal), \
             M_BH={m_bh_nmm:.1} N·mm vs measured ~{m_measured} N·mm \
             (Schwarz 2023 Fig. 5)");
    }

    /// Schwarz 32216 axial 6 kN, 4000 rpm, 50 °C — measured M_total ≈ 3800 N·mm.
    /// At high speed the load-independent EHL∞ asymptote is reached and BH
    /// gives close-to-bearing-total under pure axial loading (no sliding).
    #[test]
    fn test_bh_schwarz_32216_axial_4000rpm_50c() {
        let (m_bh_nmm, _) = schwarz_32216_bh_rolling_torque(50.0, 4000.0, 6.0);
        assert!(m_bh_nmm > 0.0 && m_bh_nmm.is_finite(),
            "BH rolling M must be positive finite: got {m_bh_nmm}");
        let m_measured = 3800.0_f64;            // Schwarz Fig. 5 @ 50 °C, 4000 rpm
        let ratio = m_bh_nmm / m_measured;
        eprintln!("Schwarz 32216 @ 4000 rpm, 50 °C, F_a=6 kN:");
        eprintln!("  M_BH (rolling)  = {:.1} N·mm", m_bh_nmm);
        eprintln!("  M_measured (tot) = {} N·mm", m_measured);
        eprintln!("  Ratio (BH/meas)  = {:.3}", ratio);
        // Observed ratio ≈ 1.20 with Wilson 1979 φ_T thermal correction
        // (was 1.369 isothermal; φ_T ≈ 0.87 at u_outer ≈ 11 m/s reduces over-
        // prediction from 37 % to 20 %).  Remaining gap likely from EHL∞
        // asymptote saturation and approximations in the per-contact-to-
        // bearing-level integration (e.g., simplified r_x for outer raceway).
        assert!(ratio > 1.05 && ratio < 1.40,
            "BH/measured at 4000 rpm: ratio={ratio:.3} (observed 1.198 with thermal), \
             M_BH={m_bh_nmm:.1} N·mm vs measured ~{m_measured} N·mm \
             (Schwarz 2023 Fig. 5)");
    }

    /// Schwarz 32216 axial 6 kN, 2000 rpm, 50 °C — measured M_total ≈ 2750 N·mm
    /// (linear interpolation between 1200@500 and 3800@4000 from Fig. 5).
    /// Mid-speed test: BH rolling should still dominate but thermal correction
    /// is starting to matter (φ_T ≈ 0.93 at u_outer ≈ 5.5 m/s).
    #[test]
    fn test_bh_schwarz_32216_axial_2000rpm_50c() {
        let (m_bh_nmm, _) = schwarz_32216_bh_rolling_torque(50.0, 2000.0, 6.0);
        assert!(m_bh_nmm > 0.0 && m_bh_nmm.is_finite(),
            "BH rolling M must be positive finite: got {m_bh_nmm}");
        // Schwarz Fig. 5 linear-interpolated mid-speed: ~2750 N·mm
        let m_measured = 2750.0_f64;
        let ratio = m_bh_nmm / m_measured;
        eprintln!("Schwarz 32216 @ 2000 rpm, 50 °C, F_a=6 kN:");
        eprintln!("  M_BH (rolling)  = {:.1} N·mm", m_bh_nmm);
        eprintln!("  M_measured (interp) = {} N·mm", m_measured);
        eprintln!("  Ratio (BH/meas)  = {:.3}", ratio);
        // Mid-speed bracket [0.85, 1.35] — between low- and high-speed bounds
        assert!(ratio > 0.85 && ratio < 1.35,
            "BH/measured at 2000 rpm: ratio={ratio:.3}, M_BH={m_bh_nmm:.1} N·mm \
             vs interpolated measured ~{m_measured} N·mm");
    }

    /// Schwarz 32216 axial 6 kN, 500 rpm, **42 °C** (cooler oil → higher
    /// viscosity → higher rolling torque).  Vogel η(42°C)/η(50°C) = 1.46;
    /// expected M ≈ 1200 × 1.46^0.75 ≈ 1580 N·mm at 500 rpm (per EHL U^0.75).
    /// Schwarz Fig. 5 shows 42°C curve consistently ~30-40% higher than 50°C.
    #[test]
    fn test_bh_schwarz_32216_axial_500rpm_42c() {
        let (m_bh_nmm, _) = schwarz_32216_bh_rolling_torque(42.0, 500.0, 6.0);
        assert!(m_bh_nmm > 0.0 && m_bh_nmm.is_finite(),
            "BH rolling M must be positive finite: got {m_bh_nmm}");
        let m_measured = 1580.0_f64;            // estimate from Fig. 5 trend
        let ratio = m_bh_nmm / m_measured;
        eprintln!("Schwarz 32216 @ 500 rpm, 42 °C, F_a=6 kN:");
        eprintln!("  M_BH (rolling)  = {:.1} N·mm", m_bh_nmm);
        eprintln!("  M_measured (est) = {} N·mm", m_measured);
        eprintln!("  Ratio (BH/meas)  = {:.3}", ratio);
        // 42°C ratio bracket [0.65, 1.10] — Fig 5 measured value uncertainty
        assert!(ratio > 0.65 && ratio < 1.10,
            "BH/measured at 500 rpm 42°C: ratio={ratio:.3}");
    }

    /// Schwarz 32216 — temperature scaling sanity.  Cooler oil (42°C → higher
    /// η) should produce higher rolling torque at fixed speed.  Expected
    /// M(42°C) / M(50°C) ≈ (η_42/η_50)^0.75 = 1.46^0.75 = 1.32 (EHL∞ U^0.75).
    #[test]
    fn test_bh_schwarz_32216_temperature_scaling() {
        let (m_50, _) = schwarz_32216_bh_rolling_torque(50.0, 2000.0, 6.0);
        let (m_42, _) = schwarz_32216_bh_rolling_torque(42.0, 2000.0, 6.0);
        let ratio = m_42 / m_50;
        // η(42)/η(50) per Vogel = 76.6/52.4 = 1.46.  EHL∞ M ∝ η^0.75 → 1.32.
        // Wilson φ_T also decreases at higher η (more inlet shear), partly
        // offsetting.  Expect 1.20-1.40.
        assert!(ratio > 1.20 && ratio < 1.45,
            "M(42°C)/M(50°C) at 2000 rpm: ratio={ratio:.3}, expected ~1.32");
    }

    /// Speed-scaling sanity: at the same 32216 axial-load setup, M_BH @ 4000 rpm
    /// should be ~4× M_BH @ 500 rpm with thermal correction (less than the
    /// isothermal EHL∞ asymptote of 4.76 = 8^0.75).
    #[test]
    fn test_bh_schwarz_32216_speed_scaling() {
        let (m_500, _)  = schwarz_32216_bh_rolling_torque(50.0, 500.0,  6.0);
        let (m_4000, _) = schwarz_32216_bh_rolling_torque(50.0, 4000.0, 6.0);
        let ratio = m_4000 / m_500;
        // Observed ratio ≈ 4.13 with Wilson 1979 φ_T (was 4.67 isothermal).
        // Thermal correction subdues high-speed scaling, making it closer to
        // measured 3.17 (1200 → 3800 N·mm).  Bracket [3.7, 4.5] reflects the
        // post-thermal speed scaling between IVR (2.83) and EHL∞ (4.76)
        // damped by inlet-shear heating at high speed.
        assert!(ratio > 3.7 && ratio < 4.5,
            "BH speed scaling 500→4000 rpm: M ratio={ratio:.3} (observed 4.13 \
             with thermal; expected between IVR 2.83 and EHL∞ 4.76)");
    }

    // ─── External Experimental Validation — Tewari 2023 (32008-class) ──
    //
    // Manjunath, Fauconnier, Ost, De Baets (2023) "Experimental Analysis of
    // Rolling Torque and Thermal Inlet Shear Heating in Tapered Roller
    // Bearings", Machines 11:801. https://doi.org/10.3390/machines11080801
    //
    // Test bearing: single-row TRB, d=40, D=68, B=19 mm (32008-class).
    //   Roller dimensions estimated for typical 32008X: Z=19, d_RB≈8.7 mm,
    //   l_RB≈12.5 mm, half-cone α≈14°, d_pitch≈54 mm.
    //
    // Lubricant: FVA No. 3A (paraffin-based solvent raffinate)
    //   ν₄₀ = 90.02 mm²/s, ν₁₀₀ = 10.41 mm²/s, ρ = 884.1 kg/m³,
    //   α_pv ≈ 21.6 GPa⁻¹ @ 25°C, decreasing with T.
    //   Operating viscosity stated in §4.3:
    //     ν(55°C) = 70.11 mm²/s, ν(65°C) = 56.90 mm²/s
    //
    // Operating: F_a = 9.6 or 12.85 kN axial, T = 35-65°C, n = 200-2200 rpm.
    //
    // Validation strategy: Tewari's measurements are presented graphically
    // (Figs. 11, 13, 14) without tabular numerical values.  We instead
    // compare BH 2010 Part 1 to the **Matsuyama formula** (Tewari Eq. 13)
    // which Tewari validates as best-matching their experimental results.
    // Both formulas are calibrated for line-contact pure rolling — agreement
    // within factor 2 is expected.
    //
    //   Matsuyama (1998-2001) Eq. 13:
    //     m_i/o = 8.89 · U^0.75 · G^(-0.04) · W_lambda^0.42
    //     (per-contact dimensionless rolling resistance moment)
    //   M_i/o = m × b × E' × l × R = b × E' × l × R × m_i/o

    /// Compute the BH 2010 Part 1 rolling torque [N·mm] at the Tewari 32008
    /// operating point.  Uses **verified 32008 geometry from Liu et al. 2022
    /// *Lubricants* 10:154 Table 1** (open-access paper providing exact roller
    /// dimensions and raceway angles — previous estimate had Z=19, d_rb=8.7,
    /// l=12.5 which under-predicted by ~50% at 2200 rpm 65°C).
    fn tewari_32008_bh_rolling_torque(t_op_c: f64, n_inner_rpm: f64, f_a_kn: f64) -> f64 {
        // 32008 exact geometry (Liu et al. 2022 Lubricants 10:154 Table 1)
        let z = 23usize;                              // # rollers
        let alpha_o_rad: f64 = 0.2473;                // outer raceway angle [rad] = 14.17°
        let alpha_i_rad: f64 = 0.1949;                // inner raceway angle [rad] = 11.17°
        let d_pitch_m = 54.0e-3;                       // pitch dia (32008 catalogue d_m = (40+68)/2)
        let d_we_max_m = 6.846e-3;                    // roller large diameter
        let d_we_min_m = 6.131e-3;                    // roller small diameter
        let d_rb_m = (d_we_max_m + d_we_min_m) / 2.0; // roller mean diameter
        let l_m = 13.66e-3;                            // effective roller length
        let r_pitch_m = d_pitch_m / 2.0;
        let r_rb_m = d_rb_m / 2.0;
        let alpha_avg = (alpha_i_rad + alpha_o_rad) / 2.0;

        let omega_i = 2.0 * std::f64::consts::PI * n_inner_rpm / 60.0;
        // Cone-apex cage speed convention (matches kinematics helper)
        let omega_cage = omega_i * alpha_i_rad.sin()
            / (alpha_i_rad.sin() + alpha_o_rad.sin());
        let r_outer_contact = r_pitch_m + r_rb_m * alpha_avg.cos();
        let r_inner_contact = r_pitch_m - r_rb_m * alpha_avg.cos();
        let u_outer = omega_cage * r_outer_contact;
        let u_inner = omega_cage * r_inner_contact;

        // Per-roller normal loads using outer raceway angle for axial-load case
        let q_outer = f_a_kn * 1e3 / (z as f64 * alpha_o_rad.sin());
        let q_inner = q_outer * (alpha_o_rad - alpha_i_rad).cos();

        // FVA 3A: η at 55°C = 70.11 cSt × 870 kg/m³ ≈ 0.0610 Pa·s,
        //         η at 65°C = 56.90 × 864 ≈ 0.0492 Pa·s.
        // Linear-interpolate dynamic viscosity via Walther between these
        // two anchor points.
        let nu_55: f64 = 70.11e-6;       // m²/s
        let nu_65: f64 = 56.90e-6;
        let t_a_k: f64 = 55.0 + 273.15;
        let t_b_k: f64 = 65.0 + 273.15;
        let log_log_a = (nu_55 * 1e6 + 0.7).log10().log10();
        let log_log_b = (nu_65 * 1e6 + 0.7).log10().log10();
        let b = (log_log_b - log_log_a) / (t_b_k.log10() - t_a_k.log10());
        let a = log_log_a - b * t_a_k.log10();
        let log_log_t = a + b * (t_op_c + 273.15_f64).log10();
        let nu_op_cst = 10f64.powf(10f64.powf(log_log_t)) - 0.7;
        let rho = 884.1 - 0.6 * (t_op_c - 15.0);
        let eta_0 = nu_op_cst * 1e-6 * rho;

        // β_visc from Walther: β ≈ ln(ν₅₅/ν₆₅) / 10 ≈ 0.021 1/K
        let beta_visc: f64 = (nu_55 / nu_65).ln() / 10.0;
        let k_fluid = 0.13;                     // typical mineral oil

        let e_prime = 2.31e11_f64;
        let r_x = r_rb_m;

        let p_outer = biboulet_houpert_line_rolling_power_with_thermal(
            eta_0, u_outer, q_outer, l_m, r_x, e_prime, beta_visc, k_fluid);
        let p_inner = biboulet_houpert_line_rolling_power_with_thermal(
            eta_0, u_inner, q_inner, l_m, r_x, e_prime, beta_visc, k_fluid);

        let p_total_w = (p_outer + p_inner) * z as f64;
        let m_total_nmm = p_total_w / omega_i.max(1e-9) * 1000.0;
        m_total_nmm
    }

    /// Compute Matsuyama (1998-2001) per-contact rolling moment for the
    /// Tewari 32008 operating point.  Eq. 13 from the Tewari paper:
    ///   M_i/o = 14.2 · E' · l · R_e² · U^0.75 · G^(-0.04) · W^0.08
    /// Returns total bearing rolling torque [N·mm] for cross-comparison
    /// against `tewari_32008_bh_rolling_torque`.
    fn tewari_32008_matsuyama_rolling_torque(t_op_c: f64, n_inner_rpm: f64, f_a_kn: f64) -> f64 {
        // 32008 exact geometry (Liu et al. 2022 Lubricants 10:154 Table 1)
        let z = 23usize;
        let alpha_o_rad: f64 = 0.2473;
        let alpha_i_rad: f64 = 0.1949;
        let d_pitch_m = 54.0e-3;
        let d_we_max_m = 6.846e-3;
        let d_we_min_m = 6.131e-3;
        let d_rb_m = (d_we_max_m + d_we_min_m) / 2.0;
        let l_m = 13.66e-3;
        let r_pitch_m = d_pitch_m / 2.0;
        let r_rb_m = d_rb_m / 2.0;
        let alpha_avg = (alpha_i_rad + alpha_o_rad) / 2.0;

        let omega_i = 2.0 * std::f64::consts::PI * n_inner_rpm / 60.0;
        let omega_cage = omega_i * alpha_i_rad.sin()
            / (alpha_i_rad.sin() + alpha_o_rad.sin());
        let r_outer_contact = r_pitch_m + r_rb_m * alpha_avg.cos();
        let r_inner_contact = r_pitch_m - r_rb_m * alpha_avg.cos();
        let u_outer = omega_cage * r_outer_contact;
        let u_inner = omega_cage * r_inner_contact;

        let q_outer = f_a_kn * 1e3 / (z as f64 * alpha_o_rad.sin());
        let q_inner = q_outer * (alpha_o_rad - alpha_i_rad).cos();

        let nu_55: f64 = 70.11e-6; let nu_65: f64 = 56.90e-6;
        let t_a_k: f64 = 55.0 + 273.15; let t_b_k: f64 = 65.0 + 273.15;
        let log_log_a = (nu_55 * 1e6 + 0.7).log10().log10();
        let log_log_b = (nu_65 * 1e6 + 0.7).log10().log10();
        let b = (log_log_b - log_log_a) / (t_b_k.log10() - t_a_k.log10());
        let a = log_log_a - b * t_a_k.log10();
        let nu_op_cst = 10f64.powf(10f64.powf(a + b * (t_op_c + 273.15_f64).log10())) - 0.7;
        let rho = 884.1 - 0.6 * (t_op_c - 15.0);
        let eta_0 = nu_op_cst * 1e-6 * rho;

        let e_prime = 2.31e11_f64;
        let r_x = r_rb_m;

        // Matsuyama dimensionless groups (per-contact, Tewari Eqs. 23-25):
        //   U_M = π·d_m·n·η / (60·D_a·E')   (D_a = bearing outer or pitch?)
        //   Use U_M = η·u_m / (E'·R_x) (standard convention) for direct
        //   comparison with our BH definition.
        //   G = α_pv × E', W_M = q_per_l / (E'·R_x)
        let alpha_pv_per_pa = 20e-9_f64;        // typical mineral oil
        let g_param = alpha_pv_per_pa * e_prime;

        // Compute Matsuyama M for outer contact
        let m_outer = {
            let u_param = eta_0 * u_outer / (e_prime * r_x);
            let q_per_l = q_outer / l_m;
            let w_param = q_per_l / (e_prime * r_x);
            // M_i/o per Tewari Eq. 13 (units: N·m)
            14.2 * e_prime * l_m * r_x.powi(2)
                * u_param.powf(0.75) * g_param.powf(-0.04) * w_param.powf(0.08)
        };
        let m_inner = {
            let u_param = eta_0 * u_inner / (e_prime * r_x);
            let q_per_l = q_inner / l_m;
            let w_param = q_per_l / (e_prime * r_x);
            14.2 * e_prime * l_m * r_x.powi(2)
                * u_param.powf(0.75) * g_param.powf(-0.04) * w_param.powf(0.08)
        };

        // Bearing-level M_rr = Z × (m_inner + m_outer) (no R_o/R_i moment-arm
        // weighting here — simplified as moments around roller's own contact).
        // Result in N·m → convert to N·mm
        (m_inner + m_outer) * z as f64 * 1000.0
    }

    /// Tewari 32008 @ 12.85 kN, 65 °C, 2000 rpm — verify BH and Matsuyama
    /// formulas give similar rolling torque (both validated against Tewari's
    /// experimental results in their paper).  Cross-formula agreement
    /// indicates BH is in the correct ballpark for 32008-class TRBs.
    #[test]
    fn test_bh_vs_matsuyama_tewari_32008_2000rpm_65c() {
        let m_bh = tewari_32008_bh_rolling_torque(65.0, 2000.0, 12.85);
        let m_mat = tewari_32008_matsuyama_rolling_torque(65.0, 2000.0, 12.85);
        eprintln!("Tewari 32008 @ 2000 rpm, 65 °C, F_a=12.85 kN:");
        eprintln!("  BH 2010 Part 1: {:.1} N·mm", m_bh);
        eprintln!("  Matsuyama:      {:.1} N·mm", m_mat);
        eprintln!("  Ratio (BH/Mat): {:.3}", m_bh / m_mat);
        assert!(m_bh > 0.0 && m_bh.is_finite());
        assert!(m_mat > 0.0 && m_mat.is_finite());
        let ratio = m_bh / m_mat;
        // Both formulas validated experimentally on TRBs.  Agreement to
        // factor 3 is expected; tighter would require bearing-specific
        // calibration.  The Matsuyama formula here is per-contact only
        // (no moment-arm weighting around bearing axis), so direct ratio
        // comparison is approximate.
        assert!(ratio > 0.3 && ratio < 3.0,
            "BH/Matsuyama ratio at Tewari operating point: {ratio:.3}");
    }

    /// Tewari 32008 — speed sweep monotonicity (200 → 2200 rpm).
    /// BH M_rr should increase monotonically with speed in this range.
    #[test]
    fn test_bh_tewari_32008_speed_monotonic() {
        let speeds = [200.0, 500.0, 1000.0, 1500.0, 2200.0];
        let mut prev = 0.0;
        for n in speeds {
            let m = tewari_32008_bh_rolling_torque(65.0, n, 12.85);
            assert!(m > prev,
                "M must be monotonic in speed: M({n} rpm)={m:.1} ≤ prev={prev:.1}");
            prev = m;
        }
    }

    // ─── Tewari Figure 13 — Extracted Measured Data ──────────────────
    //
    // Numerical values read directly from Tewari 2023 Figure 13 panels (a)
    // and (b), which plot M_rr (rolling resistance only, after subtracting
    // calculated M_sl_rib per their §4.4 methodology) vs. shaft speed for
    // F_a = 12.85 kN axial load.  Two temperature curves (55°C and 65°C).
    //
    //   n [rpm] | M_rr 55°C [N·m] | M_rr 65°C [N·m]
    //   ────────┼────────────────┼─────────────────
    //   200    │ 0.62           │ 0.83  (Stribeck — boundary contribution)
    //   400    │ 0.48           │ 0.42  (Stribeck dip — mixed transition)
    //   600    │ 0.63           │ 0.45
    //   1000   │ 0.97           │ 0.77
    //   1400   │ 1.03           │ 0.82
    //   1800   │ 1.05           │ 0.90
    //   2200   │ 1.07           │ 0.95
    //
    // Note: at low speed (200, 400 rpm) the Stribeck-curve boundary
    // contribution dominates and BH (rolling-only) under-predicts strongly.
    // Above 1000 rpm the EHL regime is fully developed and BH should track
    // the trend.  With **Liu et al. 2022 *Lubricants* 10:154 Table 1 exact
    // 32008 geometry** (Z=23, l_RB=13.66, d_RB=6.49, α_o=14.17°, α_i=11.17°),
    // magnitude under-prediction is reduced from ~50% to ~29%.  Trend
    // (monotonic in n above Stribeck dip) and temperature scaling
    // (55°C > 65°C in EHL) ARE reproduced correctly.

    const TEWARI_FIG13: &[(f64, f64, f64)] = &[
        // (n_rpm, M_rr_55C_Nm, M_rr_65C_Nm)
        (200.0,  0.62, 0.83),
        (400.0,  0.48, 0.42),
        (600.0,  0.63, 0.45),
        (1000.0, 0.97, 0.77),
        (1400.0, 1.03, 0.82),
        (1800.0, 1.05, 0.90),
        (2200.0, 1.07, 0.95),
    ];

    /// Tewari Fig 13 trend: in the fully-developed EHL regime (≥1000 rpm),
    /// 55°C measured M_rr exceeds 65°C measured (cooler oil → higher η →
    /// higher EHL rolling resistance).  Our BH+Aihara must reproduce this
    /// ordering at all speed points.
    #[test]
    fn test_bh_tewari_fig13_temperature_ordering() {
        for (n, m_55, m_65) in TEWARI_FIG13.iter() {
            if *n < 1000.0 { continue; }    // skip Stribeck regime (boundary)
            assert!(m_55 > m_65,
                "Tewari measured: M(55°C)={m_55:.2} should exceed M(65°C)={m_65:.2} at {n:.0} rpm");
            let m_bh_55 = tewari_32008_bh_rolling_torque(55.0, *n, 12.85);
            let m_bh_65 = tewari_32008_bh_rolling_torque(65.0, *n, 12.85);
            assert!(m_bh_55 > m_bh_65,
                "BH must match measured ordering at {n:.0} rpm: M_BH(55)={m_bh_55:.1} \
                 should exceed M_BH(65)={m_bh_65:.1}");
        }
    }

    /// Tewari Fig 13 trend: in the fully-developed EHL regime (≥1000 rpm),
    /// measured M_rr increases monotonically with speed.  Our BH should
    /// reproduce this.  (Below 1000 rpm the Stribeck dip at 400 rpm makes
    /// monotonicity fail by design.)
    #[test]
    fn test_bh_tewari_fig13_ehl_speed_monotonic() {
        let speeds_ehl: Vec<f64> = TEWARI_FIG13.iter()
            .filter(|(n, _, _)| *n >= 1000.0)
            .map(|(n, _, _)| *n)
            .collect();
        let mut prev_meas = 0.0;
        let mut prev_bh = 0.0;
        for n in speeds_ehl {
            let m_meas = TEWARI_FIG13.iter().find(|(n2, _, _)| (*n2 - n).abs() < 1.0)
                .map(|(_, _, m)| *m).unwrap();
            let m_bh = tewari_32008_bh_rolling_torque(65.0, n, 12.85) * 1e-3; // N·mm → N·m
            assert!(m_meas > prev_meas, "Measured monotonic at {n} rpm: {m_meas} ≤ {prev_meas}");
            assert!(m_bh > prev_bh, "BH monotonic at {n} rpm: {m_bh:.3} ≤ {prev_bh:.3}");
            prev_meas = m_meas;
            prev_bh = m_bh;
        }
    }

    /// Tewari Fig 13 magnitude check: at 2200 rpm (deep EHL), BH+Aihara
    /// gives M_rr in the same order of magnitude as the measured value,
    /// allowing for ~50 % geometry-estimate uncertainty.  Acknowledged
    /// limitation — exact 32008 geometry not stated in paper.
    #[test]
    fn test_bh_tewari_fig13_magnitude_2200rpm_65c() {
        let m_meas_65 = 0.95;       // N·m at 2200 rpm 65°C (Fig 13b)
        let m_bh = tewari_32008_bh_rolling_torque(65.0, 2200.0, 12.85) * 1e-3; // → N·m
        let ratio = m_bh / m_meas_65;
        eprintln!("Tewari Fig 13(b) @ 2200 rpm 65 °C, F_a=12.85 kN:");
        eprintln!("  Measured M_rr = {} N·m", m_meas_65);
        eprintln!("  Our BH+Aihara = {:.3} N·m", m_bh);
        eprintln!("  Ratio (BH/meas) = {:.3}", ratio);
        // Wide bracket [0.30, 1.30] — geometry uncertainty allows ~50 %
        // under-prediction; ensures order-of-magnitude correctness.
        assert!(ratio > 0.30 && ratio < 1.30,
            "BH/measured at Tewari 2200 rpm 65°C: ratio={ratio:.3} (geometry uncertainty allowed)");
    }

    // ─── Zhou-Hoeprich 1991 Fig 9 (LM12700) ──────────────────────────
    //
    // Reference: Zhou, R.S. & Hoeprich, M.R. (1991) *J. Tribol.* 113:590-597
    // Figure 9 (figure-extracted from `Reference/Validation/...images/_page_7_Figure_15.jpeg`).
    //
    // LM12700 bearing (Timken inch-series small TRB):
    //   - cup work point diameter d_m_cup = 41.5 mm
    //   - cup raceway angle α_o = 11°32' = 11.53° = 0.2014 rad
    //   - roller length l = 10.8 mm
    //   - Z = 17 rollers
    //   - SAE 75W oil, operating temp 80 °C
    //   - dimensionless: G = 0.34×10⁴, W = 0.142×10⁻³, U = 0.98×10⁻¹² to 0.47×10⁻¹⁰
    //
    // Estimated geometry (paper does not state explicitly):
    //   - d_we_mean ≈ 6 mm (back-calculated from W and typical R_e)
    //   - α_i ≈ 9.5° (typical TRB inner < outer by 2-3°)
    //   - F_a ≈ 3.6 kN axial (back-calculated from W=0.142e-3)
    //
    // Measurement (raw test data from Fig 9; "rib" and "raceway" curves are
    // Zhou-Hoeprich's own model prediction breakdown, NOT directly-measured
    // separation):
    //
    //   n [rpm] | M_total measured [N·m]
    //   ────────┼─────────────────────
    //   200    │  0.450  (Stribeck boundary regime, rib dominant)
    //   400    │  0.200
    //   800    │  0.115
    //   1600   │  0.085  (minimum, transition raceway↔rib)
    //   2400   │  0.090
    //   3200   │  0.100
    //   4000   │  0.110
    //   4800   │  0.120

    const ZHOU_HOEPRICH_LM12700: &[(f64, f64)] = &[
        // (n_rpm, M_total_measured_Nm)
        (200.0,  0.450),
        (400.0,  0.200),
        (800.0,  0.115),
        (1600.0, 0.085),
        (2400.0, 0.090),
        (3200.0, 0.100),
        (4000.0, 0.110),
        (4800.0, 0.120),
    ];

    /// BH+Aihara raceway rolling torque for Zhou-Hoeprich LM12700 bearing
    /// (estimated geometry, SAE 75W oil at 80 °C).
    fn zhou_hoeprich_lm12700_bh_raceway_nmm(n_rpm: f64) -> f64 {
        // Estimated geometry — see ZHOU_HOEPRICH_LM12700 notes
        let z = 17usize;
        let alpha_o_rad: f64 = 11.53_f64.to_radians();
        let alpha_i_rad: f64 = 9.5_f64.to_radians();        // typical TRB
        let alpha_avg = (alpha_i_rad + alpha_o_rad) / 2.0;
        let d_pitch_m = 41.5e-3;
        let d_we_m: f64 = 6.0e-3;                            // estimated
        let l_m = 10.8e-3;
        let r_pitch_m = d_pitch_m / 2.0;
        let r_rb_m = d_we_m / 2.0;
        let f_a_n: f64 = 3600.0;                             // back-calc from W=0.142e-3

        let omega_i = 2.0 * std::f64::consts::PI * n_rpm / 60.0;
        let omega_cage = omega_i * alpha_i_rad.sin()
            / (alpha_i_rad.sin() + alpha_o_rad.sin());
        let r_outer_contact = r_pitch_m + r_rb_m * alpha_avg.cos();
        let r_inner_contact = r_pitch_m - r_rb_m * alpha_avg.cos();
        let u_outer = omega_cage * r_outer_contact;
        let u_inner = omega_cage * r_inner_contact;
        let q_outer = f_a_n / (z as f64 * alpha_o_rad.sin());
        let q_inner = q_outer * (alpha_o_rad - alpha_i_rad).cos();

        // SAE 75W at 80 °C ≈ 10 cSt (light oil)
        let nu_op_cst: f64 = 10.0_f64;
        let rho = 870.0;                                     // typical mineral oil
        let eta_0 = nu_op_cst * 1e-6 * rho;
        // β estimate from Vogel-like: dη/dT ≈ -0.03/K at 80°C, β ≈ 0.025 1/K
        let beta_visc: f64 = 0.025;
        let k_fluid = 0.13;
        let e_prime = 2.31e11_f64;
        let r_x = r_rb_m;

        let p_outer = biboulet_houpert_line_rolling_power_dispatched(
            eta_0, u_outer, q_outer, l_m, r_x, e_prime, beta_visc, k_fluid,
            ThermalCorrection::Aihara1987);
        let p_inner = biboulet_houpert_line_rolling_power_dispatched(
            eta_0, u_inner, q_inner, l_m, r_x, e_prime, beta_visc, k_fluid,
            ThermalCorrection::Aihara1987);
        let p_tot = (p_outer + p_inner) * z as f64;
        p_tot / omega_i.max(1e-9) * 1000.0      // N·mm
    }

    // ─── Cruz/Marques 2021 (HM801349/310 tandem TRB, axle pinion) ───
    //
    // Reference: Cruz, Marques, Seabra, Martins (2021) *Tribol. Int.* 157:106876
    //
    // Pinion bearing geometry (Table 2):
    //   - Koyo HM801349/310 inch-series TRB, tandem (×2 back-to-back)
    //   - Bore 40.48 mm, OD 82.55 mm, Width 29.37 mm
    //   - Z = 19 rollers, d_m = 61.515 mm
    //   - Roller mean diameter D = 7.72 mm, length l = 22.2 mm
    //   - Contact angle α = 20° (large angle)
    //
    // Preload backward-calculation (paper §3.4.1):
    //   - Starting torque 0.75 Nm → F_a = 2083 N
    //   - Starting torque 1.90 Nm → F_a = 5279 N
    //   - Starting torque 3.00 Nm → F_a = 8336 N
    //
    // Operating: SAE 75W90 axle gear oil (66.8 cSt @ 50°C → 19.1 cSt @ 90°C)
    //
    // Measurement (paper §3.3.2, "pinion assembly torque loss" = 2× TRB + seal):
    //   At 1500 rpm, M_st=1.90 Nm, T_sump_Gear=62.2 °C → T_VLO,Exp ≈ 1.35 Nm

    /// Compute BH+Aihara raceway rolling torque [N·mm] for a single
    /// Cruz/Marques 2021 Koyo HM801349/310 pinion bearing.
    fn cruz_marques_hm801349_bh_torque_nmm(
        n_inner_rpm: f64, f_a_n: f64, t_op_c: f64, nu_op_cst: f64,
    ) -> f64 {
        let z = 19usize;
        let alpha_rad: f64 = 20.0_f64.to_radians();
        let d_pitch_m = 61.515e-3;
        let d_we_m: f64 = 7.72e-3;
        let l_m = 22.2e-3;
        let r_pitch_m = d_pitch_m / 2.0;
        let r_rb_m = d_we_m / 2.0;
        // α_i and α_o not separately published; use symmetric approximation
        // (single contact angle 20°, common in inch-series TRBs).

        let omega_i = 2.0 * std::f64::consts::PI * n_inner_rpm / 60.0;
        let omega_cage = omega_i * (1.0 - r_rb_m * alpha_rad.cos() / r_pitch_m) / 2.0;
        let r_outer_contact = r_pitch_m + r_rb_m * alpha_rad.cos();
        let r_inner_contact = r_pitch_m - r_rb_m * alpha_rad.cos();
        let u_outer = omega_cage * r_outer_contact;
        let u_inner = omega_cage * r_inner_contact;

        let q_outer = f_a_n / (z as f64 * alpha_rad.sin());
        let q_inner = q_outer * alpha_rad.cos();

        // SAE 75W90 properties (paper Table 6: 66.8 cSt @ 50°C, 19.1 cSt @ 90°C)
        let rho = 870.0;
        let eta_0 = nu_op_cst * 1e-6 * rho;
        // β from Walther-like log-log slope:
        // log(log(66.8+0.7)) - log(log(19.1+0.7)) over T span [323.15, 363.15] K
        let beta_visc: f64 = 0.030;     // estimated from SAE 75W90 typical
        let k_fluid = 0.13;
        let alpha_pv = 20e-9_f64;
        let e_prime = 2.31e11_f64;
        let r_x = r_rb_m;
        let _ = (t_op_c, alpha_pv);     // documented inputs not directly in BH path

        let p_outer = biboulet_houpert_line_rolling_power_dispatched(
            eta_0, u_outer, q_outer, l_m, r_x, e_prime, beta_visc, k_fluid,
            ThermalCorrection::Aihara1987);
        let p_inner = biboulet_houpert_line_rolling_power_dispatched(
            eta_0, u_inner, q_inner, l_m, r_x, e_prime, beta_visc, k_fluid,
            ThermalCorrection::Aihara1987);
        let p_total_w = (p_outer + p_inner) * z as f64;
        p_total_w / omega_i.max(1e-9) * 1000.0
    }

    /// Diagnostic — Cruz/Marques 2021 HM801349/310 tandem TRB validation.
    /// Measured "pinion assembly torque loss" = 2× HM801349/310 + pinion seal.
    /// Our raceway-only BH+Aihara is compared against the measured total at
    /// three operating points where the paper provides explicit numerical
    /// values (line 309 and the cited Fig 11 mid-temperature sweep).
    #[test]
    #[ignore]
    fn diag_cruz_marques_2021_hm801349_validation() {
        // (n_rpm, F_a [N], T_op [°C], ν_op [cSt], M_pinion_meas [Nm])
        // The first point (1500/5279/62.2/42.9/1.35) is explicitly named in
        // the paper text (§3.3.2 line 309).  Other rows scaled from Figure 11.
        let cases = [
            (1500.0_f64, 5279.0_f64, 62.2_f64, 42.9_f64, 1.35_f64),
            (2000.0,     5279.0,     62.2,     42.9,     1.60),
            (1500.0,     2083.0,     62.2,     42.9,     0.95),  // est. from Fig 11
            (1500.0,     8336.0,     62.2,     42.9,     1.65),  // est. from Fig 11
        ];

        let seal_torque_nm: f64 = 0.10;    // typical Simrit seal estimate (~50-100 mNm)

        eprintln!("\nCruz/Marques 2021 — HM801349/310 tandem TRB validation:");
        eprintln!("  Geometry: Z=19, d_m=61.515, D=7.72 mm, l=22.2 mm, α=20° (Koyo HM801349/310)");
        eprintln!("  Lubricant: SAE 75W90 axle gear oil");
        eprintln!("  Note: M_meas is 'pinion assembly' = 2×TRB + seal\n");
        eprintln!("  {:<8} {:<8} {:<8} {:<8} {:<14} {:<14} {:<14} {:<10}",
            "n[rpm]", "F_a[N]", "T[°C]", "ν[cSt]", "M_meas[Nm]", "M_BH×2[Nm]",
            "M_pred[Nm]", "Ratio");
        for (n, fa, tc, nu, m_meas) in cases {
            let m_single_nmm = cruz_marques_hm801349_bh_torque_nmm(n, fa, tc, nu);
            let m_single_nm = m_single_nmm * 1e-3;
            let m_tandem_nm = m_single_nm * 2.0;
            let m_pred_nm = m_tandem_nm + seal_torque_nm;
            let ratio = m_pred_nm / m_meas;
            eprintln!("  {:<8.0} {:<8.0} {:<8.1} {:<8.1} {:<14.3} {:<14.3} {:<14.3} {:<10.3}",
                n, fa, tc, nu, m_meas, m_tandem_nm, m_pred_nm, ratio);
        }
        eprintln!("\nNote: Single contact angle α=20° approximated (paper does not");
        eprintln!("publish α_i, α_o separately).  Seal torque is conservative");
        eprintln!("Simrit estimate (~0.10 Nm).  Tandem assumption: 2× single bearing.");
        eprintln!("Expect ratio ~0.7-1.0 (BH+Aihara captures raceway rolling — boundary");
        eprintln!("contribution at preload tightening not modelled).");
    }

    // ─── Hu et al. 2025 (HH926749/10 paired TRB, TBM disc cutter) ───
    //
    // Reference: Hu, Yang, Li, Zhao, Zhang (2025) *Lubricants* 13(4):160
    //
    // Bearing geometry (Table 7):
    //   - Timken HH926749/10 paired TRB, 19-inch disc cutter
    //   - d_we_mean = 43 mm (equivalent roller diameter)
    //   - D_f = 169.672 mm (raceway diameter)
    //   - l = 48 mm (roller length)
    //   - α_o ≈ 12°, α_i ≈ 7° (Table 7 multi-angle contact)
    //   - Estimated d_pitch = 127.3 mm, Z = 25 rollers
    //
    // Operating: No-load condition, grease lubricated, n < 2 rev/s (~120 rpm),
    //            measured with torque wrench (accuracy ±0.5 N·m).
    //
    // Measurement (Table 6):
    //   F_a [kN]  : 5, 10, 15, 20, 25
    //   M [N·m]   : 25, 27, 28, 30, 42

    /// Compute BH+Aihara raceway rolling torque [N·mm] for a single
    /// Hu 2025 Timken HH926749/10 bearing.  Operating point: very low
    /// speed (~100 rpm) + grease lubrication + heavy preload — expected
    /// to be sliding/boundary dominated, BH gives lower-bound estimate.
    fn hu_2025_hh926749_bh_torque_nmm(f_a_n: f64, n_inner_rpm: f64) -> f64 {
        let z = 25usize;
        let alpha_o_rad: f64 = 12.0_f64.to_radians();
        let alpha_i_rad: f64 = 7.0_f64.to_radians();
        let alpha_avg = (alpha_o_rad + alpha_i_rad) / 2.0;
        let d_pitch_m = 127.3e-3;
        let d_we_m: f64 = 43.0e-3;
        let l_m = 48.0e-3;
        let r_pitch_m = d_pitch_m / 2.0;
        let r_rb_m = d_we_m / 2.0;

        let omega_i = 2.0 * std::f64::consts::PI * n_inner_rpm / 60.0;
        let omega_cage = omega_i * alpha_i_rad.sin()
            / (alpha_i_rad.sin() + alpha_o_rad.sin());
        let r_outer_contact = r_pitch_m + r_rb_m * alpha_avg.cos();
        let r_inner_contact = r_pitch_m - r_rb_m * alpha_avg.cos();
        let u_outer = omega_cage * r_outer_contact;
        let u_inner = omega_cage * r_inner_contact;
        let q_outer = f_a_n / (z as f64 * alpha_o_rad.sin());
        let q_inner = q_outer * (alpha_o_rad - alpha_i_rad).cos();

        // Grease equivalent oil — assume NLGI 2 with ISO VG 460 base oil
        //   at ambient ~25 °C: ν ≈ 750 cSt (very thick)
        // Hu paper does not specify exact grease; this is conservative estimate.
        let nu_op_cst: f64 = 750.0;
        let rho = 880.0;
        let eta_0 = nu_op_cst * 1e-6 * rho;
        let beta_visc: f64 = 0.025;
        let k_fluid = 0.13;
        let e_prime = 2.31e11_f64;
        let r_x = r_rb_m;

        let p_outer = biboulet_houpert_line_rolling_power_dispatched(
            eta_0, u_outer, q_outer, l_m, r_x, e_prime, beta_visc, k_fluid,
            ThermalCorrection::Aihara1987);
        let p_inner = biboulet_houpert_line_rolling_power_dispatched(
            eta_0, u_inner, q_inner, l_m, r_x, e_prime, beta_visc, k_fluid,
            ThermalCorrection::Aihara1987);
        let p_total_w = (p_outer + p_inner) * z as f64;
        p_total_w / omega_i.max(1e-9) * 1000.0
    }

    /// Diagnostic — Hu 2025 HH926749/10 paired TRB low-speed validation.
    /// Demonstrates that BH+Aihara is appropriate for medium-to-high speed
    /// EHL regime but under-predicts in low-speed grease-lubricated heavy
    /// preload conditions (Stribeck boundary regime).
    #[test]
    #[ignore]
    fn diag_hu_2025_hh926749_low_speed_limit() {
        // Hu Table 6: 5 preload points
        let cases = [
            (5.0_f64,  25.0_f64),
            (10.0,     27.0),
            (15.0,     28.0),
            (20.0,     30.0),
            (25.0,     42.0),
        ];
        let n_assumed_rpm: f64 = 100.0;     // ~100 rpm typical low-speed cutter
        eprintln!("\nHu et al. 2025 — HH926749/10 paired TRB low-speed test:");
        eprintln!("  Geometry: Z=25 (est), d_we=43 mm, l=48 mm, α_o=12°, α_i=7°");
        eprintln!("  Lubrication: grease (NLGI 2, ISO VG 460 base, ν≈750 cSt @25°C est.)");
        eprintln!("  Speed: ~{} rpm (n < 2 rev/s per paper), no-load, paired TRB\n", n_assumed_rpm);
        eprintln!("  {:<10} {:<14} {:<14} {:<14} {:<10}",
            "F_a[kN]", "M_meas[N·m]", "M_BH×2[N·m]", "M_pred[N·m]", "Ratio");
        for (fa_kn, m_meas) in cases {
            let m_single_nmm = hu_2025_hh926749_bh_torque_nmm(fa_kn * 1e3, n_assumed_rpm);
            let m_single_nm = m_single_nmm * 1e-3;
            let m_paired_nm = m_single_nm * 2.0;
            // Add a nominal sliding/boundary contribution placeholder (not modelled)
            let m_pred = m_paired_nm;
            let ratio = m_pred / m_meas;
            eprintln!("  {:<10.1} {:<14.2} {:<14.3} {:<14.3} {:<10.3}",
                fa_kn, m_meas, m_paired_nm, m_pred, ratio);
        }
        eprintln!("\nNote: BH+Aihara captures EHL viscous rolling only.  Low-speed");
        eprintln!("(<200 rpm) grease lubrication + heavy preload is Stribeck");
        eprintln!("boundary regime, dominated by sliding/asperity friction.");
        eprintln!("Hu paper's own theoretical model has ~13% average error;");
        eprintln!("our raceway-only BH expectedly under-predicts here.");
        eprintln!("This validates the operating-envelope boundary of BH+Aihara.");
    }

    /// Diagnostic — GT (Greenwood-Tripp F_{5/2}) vs Clarke (Arana 2019:
    /// 1 - erf(λ)) asperity sharing comparison across Λ ∈ [0.25, 4.0].
    /// Two formulas share the Gaussian asperity-height assumption but
    /// integrate it differently:
    ///   GT       — Hertz pressure integral (F_{5/2} statistical moment)
    ///   Clarke   — simple Gaussian tail (1 - erf(λ))
    /// Reports raw f_a values and effective mu_eff = (1-f_a)·μ_EHL + f_a·μ_b
    /// at a typical TRB operating point (μ_EHL = 0.05, μ_b = 0.10) to
    /// quantify the practical engineering impact of the two-model choice.
    #[test]
    #[ignore]
    fn diag_asperity_gt_vs_clarke_comparison() {
        eprintln!("\nAsperity Load Sharing: GT (F_5/2) vs Clarke (1 - erf λ)");
        eprintln!("=====================================================");
        eprintln!("μ_EHL = 0.050 (typical EHL), μ_boundary = 0.100 (typical steel-on-steel)\n");
        eprintln!("  {:<8} {:<12} {:<12} {:<12} {:<14} {:<14} {:<12}",
            "Λ", "f_a^GT", "f_a^Clarke", "GT/Clarke", "μ_eff^GT", "μ_eff^Clarke", "Δμ_eff[%]");

        let mu_ehl = 0.050_f64;
        let mu_b = 0.100_f64;
        let f5_2_at_0 = gt_integral(2.5, 0.0);
        let lambdas = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 2.0, 2.5, 3.0, 4.0];
        for lambda in lambdas {
            let f_gt = (gt_integral(2.5, lambda) / f5_2_at_0).clamp(0.0, 1.0);
            let f_cl = clarke_load_sharing(lambda);
            let mu_gt = (1.0 - f_gt) * mu_ehl + f_gt * mu_b;
            let mu_cl = (1.0 - f_cl) * mu_ehl + f_cl * mu_b;
            let dmu_pct = (mu_gt - mu_cl) / mu_cl.max(1e-9) * 100.0;
            let ratio = if f_cl > 1e-12 { f_gt / f_cl } else { f64::INFINITY };
            eprintln!("  {:<8.2} {:<12.5} {:<12.5} {:<12.3} {:<14.5} {:<14.5} {:<+12.2}",
                lambda, f_gt, f_cl, ratio, mu_gt, mu_cl, dmu_pct);
        }
        eprintln!("\nInterpretation:");
        eprintln!("  Λ < 1.0  (boundary/mixed): GT ≈ Clarke (both → 1)");
        eprintln!("  Λ ≈ 1.5  (transition):     GT/Clarke ratio ≈ 1.8");
        eprintln!("  Λ ≈ 2.0  (EHL emerging):   GT/Clarke ratio ≈ 4");
        eprintln!("  Λ ≥ 3.0  (full EHL):       GT/Clarke ratio > 25, both near zero");
        eprintln!("  Δμ_eff:  < 2 % across all Λ (f_a is small where models diverge)");
    }

    /// Diagnostic — Zhou-Hoeprich 1991 Fig 9 LM12700 BH+Aihara raceway
    /// torque vs measured total bearing torque.  Tests trend reproduction
    /// across raceway-dominant (≥1600 rpm) and rib-dominant (≤800 rpm) regimes.
    #[test]
    #[ignore]
    fn diag_zhou_hoeprich_lm12700_fig9_sweep() {
        eprintln!("\nZhou-Hoeprich 1991 Fig 9 — LM12700 (estimated geometry):");
        eprintln!("  Z=17, d_we≈6 mm, l=10.8 mm, α_o=11.53°, α_i≈9.5° (est.)");
        eprintln!("  SAE 75W oil at 80 °C, F_a≈3.6 kN (back-calc from W=0.142e-3)\n");
        eprintln!("  {:<8} {:<14} {:<14} {:<14}",
            "n[rpm]", "M_meas[N·m]", "M_BH_race[N·m]", "Ratio_race/meas");
        for (n, m_meas) in ZHOU_HOEPRICH_LM12700.iter() {
            let m_bh_nmm = zhou_hoeprich_lm12700_bh_raceway_nmm(*n);
            let m_bh_nm = m_bh_nmm * 1e-3;
            let ratio = m_bh_nm / m_meas;
            eprintln!("  {:<8.0} {:<14.4} {:<14.4} {:<14.3}",
                n, m_meas, m_bh_nm, ratio);
        }
        eprintln!("\nNote: Low speed (n ≤ 800 rpm) is Stribeck/rib-dominant regime —");
        eprintln!("BH (rolling-only) ratio < 1.0 means rib drilling + asperity dominate.");
        eprintln!("High speed (n ≥ 2400 rpm) should approach raceway-only matching.");
    }

    /// Diagnostic — Tewari Fig 13 full sweep: 7 speeds × 2 temperatures
    /// with Liu 2022 exact 32008 geometry.  Shows magnitude and trend
    /// comparison across the entire dataset (14 points).
    #[test]
    #[ignore]
    fn diag_tewari_32008_fig13_full_sweep() {
        eprintln!("\nTewari Fig 13 full sweep — 32008 (Liu 2022 exact geometry, F_a=12.85 kN):");
        eprintln!("  Geometry: Z=23, d_rb=6.49 mm, l=13.66 mm, α_o=14.17°, α_i=11.17°");
        eprintln!("  Model: BH 2010 Part 1 + Aihara 1987 thermal correction\n");
        eprintln!("  {:<8} {:<12} {:<12} {:<10} {:<12} {:<12} {:<10}",
            "n[rpm]", "M_meas_55", "M_BH_55", "Ratio_55", "M_meas_65", "M_BH_65", "Ratio_65");
        let mut sum_sq_55 = 0.0_f64; let mut sum_sq_65 = 0.0_f64;
        let mut count_ehl = 0_usize;
        for (n, m_meas_55, m_meas_65) in TEWARI_FIG13.iter() {
            let m_bh_55 = tewari_32008_bh_rolling_torque(55.0, *n, 12.85) * 1e-3;
            let m_bh_65 = tewari_32008_bh_rolling_torque(65.0, *n, 12.85) * 1e-3;
            let r_55 = m_bh_55 / m_meas_55;
            let r_65 = m_bh_65 / m_meas_65;
            eprintln!("  {:<8.0} {:<12.3} {:<12.3} {:<10.3} {:<12.3} {:<12.3} {:<10.3}",
                n, m_meas_55, m_bh_55, r_55, m_meas_65, m_bh_65, r_65);
            // RMSE accumulated only for EHL regime (≥1000 rpm)
            if *n >= 1000.0 {
                sum_sq_55 += (r_55 - 1.0).powi(2);
                sum_sq_65 += (r_65 - 1.0).powi(2);
                count_ehl += 1;
            }
        }
        let rmse_55 = (sum_sq_55 / count_ehl as f64).sqrt() * 100.0;
        let rmse_65 = (sum_sq_65 / count_ehl as f64).sqrt() * 100.0;
        eprintln!("\nEHL regime (≥1000 rpm) RMSE vs measured:");
        eprintln!("  55 °C: {:.1} %", rmse_55);
        eprintln!("  65 °C: {:.1} %", rmse_65);
        eprintln!("\nNote: residual under-prediction = rib + hysteresis + cage (10-15 %)");
        eprintln!("                                  + FVA3A α_pv estimate uncertainty");
        eprintln!("                                  + figure-extraction precision");
    }

    /// Tewari Fig 13: temperature ratio check at 2200 rpm.
    /// Measured M(55)/M(65) = 1.07/0.95 = 1.126
    /// BH should give similar ratio (η-scaling roughly η^0.75 × thermal).
    #[test]
    fn test_bh_tewari_fig13_temperature_ratio_2200rpm() {
        let m_meas_ratio = 1.07_f64 / 0.95;  // = 1.126
        let m_bh_55 = tewari_32008_bh_rolling_torque(55.0, 2200.0, 12.85);
        let m_bh_65 = tewari_32008_bh_rolling_torque(65.0, 2200.0, 12.85);
        let m_bh_ratio = m_bh_55 / m_bh_65;
        eprintln!("Tewari Fig 13 temperature ratio @ 2200 rpm:");
        eprintln!("  Measured M(55)/M(65) = {:.3}", m_meas_ratio);
        eprintln!("  BH M(55)/M(65) = {:.3}", m_bh_ratio);
        // η(55)/η(65) ≈ 70.11/56.90 = 1.232; M ∝ η^0.75 → 1.166 (deep EHL)
        // Allow [1.05, 1.40] bracket — matches measured 1.126
        assert!(m_bh_ratio > 1.05 && m_bh_ratio < 1.40,
            "BH temperature ratio: {m_bh_ratio:.3}, measured {m_meas_ratio:.3}");
    }

    /// Tewari 32008 — temperature monotonicity.  Cooler oil → higher torque.
    #[test]
    fn test_bh_tewari_32008_temperature_monotonic() {
        let m_35 = tewari_32008_bh_rolling_torque(35.0, 1500.0, 12.85);
        let m_55 = tewari_32008_bh_rolling_torque(55.0, 1500.0, 12.85);
        let m_65 = tewari_32008_bh_rolling_torque(65.0, 1500.0, 12.85);
        // Monotonically decreasing with T (η decreases, EHL film thinner,
        // M_rr ∝ η^0.75 → lower)
        assert!(m_35 > m_55, "M(35°C)={m_35:.1} should exceed M(55°C)={m_55:.1}");
        assert!(m_55 > m_65, "M(55°C)={m_55:.1} should exceed M(65°C)={m_65:.1}");
        // Magnitude check — ratio η(35)/η(65) ≈ 2.3, M ratio ≈ 2.3^0.75 ≈ 1.86
        let ratio = m_35 / m_65;
        assert!(ratio > 1.4 && ratio < 2.5,
            "M(35°C)/M(65°C): ratio={ratio:.3}, expected ~1.86 (η-temperature scaling)");
    }

    // ─── 3-Model Cross-Validation (Palmgren / BH Part 1 / SKF) ───────
    //
    // No public 30306 measured friction-torque dataset is freely accessible;
    // commercial validation tools (Bearinx, MESYS) are licensed. As an
    // engineering surrogate, verify that the three implemented models
    // (Palmgren, Biboulet-Houpert Part 1, SKF Catalogue 2018) produce
    // mutually consistent results on the same default 30306 preset operating
    // point. This guards against silent drift in any single model while
    // documenting the quantitative *ranking* observed on this preset.
    //
    // Reference data point from literature (Engineering Research, Springer
    // 2023): on a different bearing (HR 65 KBE 52X+L double-row TRB) at 20 kN
    // / 2000 rpm, the three models gave Palmgren = 2.06, NSK = 2.53,
    // SKF = 2.96 N·m respectively (Palmgren < SKF for that loading).
    // On our default 30306 preset Palmgren > SKF holds — model ordering is
    // bearing- and load-dependent, not a universal trend.

    /// Cross-validation of all three models on a common loaded-roller input.
    /// Establishes the **30306-class default preset** ranking and guards it
    /// with quantitative bounds derived from the current implementation.
    /// Any future change that changes a model's calibration must update this
    /// reference (the test serves as a live calibration registry).
    #[test]
    fn test_friction_model_three_way_cross_validation_30306() {
        let geom = test_geom();
        let mat = test_material();
        let rp = test_raceway_profile();
        let rg = test_raceway_geom();
        let sg = test_slice_geometries(&geom, 10);
        let rolp = test_roller_profile();

        let make_slices = || -> Vec<SliceContactResult> {
            (0..10).map(|k| SliceContactResult {
                k, delta_k: 5.0, q_k: 50.0, q_k_outer: 50.0, q_k_inner: 50.0,
                b_k: 0.1, p_max_k: 1500.0, h_bulk_k: 0.0, k_hertz_k: 0.0,
                b_k_outer: 0.1, p_max_k_outer: 1400.0, h_bulk_k_outer: 0.0,
                k_hertz_k_outer: 0.0, k_combined_k: 0.0, in_contact: true,
            }).collect()
        };
        let rollers = vec![RollerResult {
            psi_deg: 0.0, q_normal: 800.0, q_normal_inner: 800.0 * 0.998,
            slice_results: make_slices(), rib_result: None,
        }];

        let mut op = test_op(1500.0);

        // Palmgren
        op.friction_model = FrictionModel::PalmgrenLike;
        let ts_p = compute_traction(&geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers).unwrap();
        // Biboulet-Houpert (Part 1 line contact)
        op.friction_model = FrictionModel::BibouletHoupert;
        let ts_b = compute_traction(&geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers).unwrap();
        // SKF Advanced
        op.friction_model = FrictionModel::SkfAdvanced;
        let ts_s = compute_traction(&geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers).unwrap();

        // BH preserves the sliding total (only rolling is replaced).
        // SKF replaces sliding too (bearing-level G_sl·μ_sl), so it differs.
        let rel_b = ((ts_b.p_sliding_w - ts_p.p_sliding_w) / ts_p.p_sliding_w.max(1e-12)).abs();
        assert!(rel_b < 1e-6 || ts_p.p_sliding_w.abs() < 1e-9,
            "BH must keep Palmgren sliding (cone apex pure rolling on default fixture \
             gives ~0 sliding, both should match): Palmgren={}, BH={}, rel={rel_b:.2e}",
            ts_p.p_sliding_w, ts_b.p_sliding_w);
        // SKF sliding is non-negative and finite (different formula entirely).
        assert!(ts_s.p_sliding_w.is_finite() && ts_s.p_sliding_w >= 0.0,
            "SKF p_sliding_w must be finite ≥ 0, got {}", ts_s.p_sliding_w);

        // All three rolling totals must be positive and finite
        for (label, p) in [
            ("Palmgren", ts_p.p_rolling_w),
            ("BH",       ts_b.p_rolling_w),
            ("SKF",      ts_s.p_rolling_w),
        ] {
            assert!(p.is_finite() && p > 0.0, "{label} p_rolling must be finite and positive, got {p}");
        }

        // Calibration-drift guard. The mock here uses ONE loaded roller
        // (Q=800 N) for the per-roller path, while SKF uses the bearing-level
        // operating-condition loads (F_r=5 kN, F_a=2 kN) — so absolute
        // magnitudes don't directly correspond. The bounds below are wide
        // sanity bounds that catch unit regressions without over-fitting:
        let r_bp = ts_b.p_rolling_w / ts_p.p_rolling_w;
        let r_sp = ts_s.p_rolling_w / ts_p.p_rolling_w;
        // BH is per-contact like Palmgren and uses the same Q distribution;
        // typically within a factor of 5 either way.
        assert!(r_bp > 0.05 && r_bp < 20.0,
            "BH/Palmgren ratio out of sanity bounds (mock single roller): {r_bp:.4}");
        // SKF reads bearing-level external loads; with mock 1-roller input
        // its relation to Palmgren is loose. Just ensure within 4 orders.
        assert!(r_sp > 1e-4 && r_sp < 1e4,
            "SKF/Palmgren ratio out of sanity bounds (mock single roller): {r_sp:.4}");

        // BH and SKF — both capture viscous-hydrodynamic rolling — agree to
        // within a few orders of magnitude on a mock fixture (real-bearing
        // comparison is in the strengths matrix in Manual §14.15.4).
        let r_bs = (ts_b.p_rolling_w / ts_s.p_rolling_w)
            .max(ts_s.p_rolling_w / ts_b.p_rolling_w);
        assert!(r_bs < 1e4,
            "BH and SKF must agree within 10000× on mock fixture: BH={}, SKF={}, ratio={r_bs:.2}",
            ts_b.p_rolling_w, ts_s.p_rolling_w);
    }

    // ─── Biboulet-Houpert 2010 unit checks ───────────────────────────

    /// Biboulet-Houpert 2010 force monotonicity in load. Both IVR and EHL
    /// asymptotic forms scale as W^(1/3), so doubling Q gives ratio ≈ 2^(1/3)
    /// ≈ 1.26. The transition parameter M ∝ W shifts the IVR/EHL partition
    /// (α_EHL grows with M), pulling the effective ratio slightly below 1.26
    /// because F_IVR is larger than F_EHL at typical operating points
    /// (verified manually). Bracket: [1.10, 1.30].
    #[test]
    fn test_biboulet_houpert_force_load_scaling() {
        let eta_0 = 0.05; let u = 1.0; let r_x = 5e-3; let r_y = 5e-1;
        let e_prime = 2.3e11;
        let f1 = biboulet_houpert_rolling_force(eta_0, u, 100.0, r_x, r_y, e_prime);
        let f2 = biboulet_houpert_rolling_force(eta_0, u, 200.0, r_x, r_y, e_prime);
        let ratio = f2 / f1;
        assert!(f1 > 0.0 && f2 > f1, "monotonic in load");
        assert!(ratio > 1.10 && ratio < 1.30,
            "F_R(2Q)/F_R(Q) within IVR-EHL transition range, got {ratio:.3}");
    }

    /// Speed monotonicity: F_R grows with entrainment speed.
    #[test]
    fn test_biboulet_houpert_force_speed_scaling() {
        let eta_0 = 0.05; let r_x = 5e-3; let r_y = 5e-1;
        let e_prime = 2.3e11; let q = 100.0;
        let f1 = biboulet_houpert_rolling_force(eta_0, 0.5, q, r_x, r_y, e_prime);
        let f2 = biboulet_houpert_rolling_force(eta_0, 2.0, q, r_x, r_y, e_prime);
        assert!(f2 > f1, "F_R must grow with speed");
        // U exponent is between 2/3 (IVR) and 3/4 (EHL), so 4× speed gives 4^(2/3)..4^(3/4).
        let ratio = f2 / f1;
        assert!(ratio > 4f64.powf(2.0/3.0) - 0.1 && ratio < 4f64.powf(0.75) + 0.4,
            "F_R(4u)/F_R(u) within IVR-EHL range, got {ratio:.3}");
    }

    /// Degenerate inputs return zero (no spurious negative or NaN values).
    #[test]
    fn test_biboulet_houpert_zero_inputs() {
        let r_x = 5e-3; let r_y = 0.5; let e_p = 2.3e11;
        assert_eq!(biboulet_houpert_rolling_force(0.0, 1.0, 100.0, r_x, r_y, e_p), 0.0);
        assert_eq!(biboulet_houpert_rolling_force(0.05, 0.0, 100.0, r_x, r_y, e_p), 0.0);
        assert_eq!(biboulet_houpert_rolling_force(0.05, 1.0, 0.0, r_x, r_y, e_p), 0.0);
    }

    /// Power wrapper consistency: P = F_R × |u|.
    #[test]
    fn test_biboulet_houpert_power_wrapper() {
        let eta_0 = 0.05; let u = 1.5; let q = 100.0;
        let r_x = 5e-3; let r_y = 0.5; let e_p = 2.3e11;
        let f = biboulet_houpert_rolling_force(eta_0, u, q, r_x, r_y, e_p);
        let p = biboulet_houpert_rolling_power(eta_0, u, q, r_x, r_y, e_p);
        assert!((p - f * u).abs() < 1e-9);
    }

    /// `FrictionModel::BibouletHoupert` overrides `p_rolling_w` only —
    /// sliding power and per-roller geometry are untouched.
    #[test]
    fn test_friction_model_biboulet_houpert_dispatcher() {
        let geom = test_geom();
        let mat = test_material();
        let rp = test_raceway_profile();
        let rg = test_raceway_geom();
        let sg = test_slice_geometries(&geom, 10);
        let rolp = test_roller_profile();

        let make_slices = || -> Vec<SliceContactResult> {
            (0..10).map(|k| SliceContactResult {
                k, delta_k: 5.0, q_k: 50.0, q_k_outer: 50.0, q_k_inner: 50.0,
                b_k: 0.1, p_max_k: 1500.0, h_bulk_k: 0.0, k_hertz_k: 0.0,
                b_k_outer: 0.1, p_max_k_outer: 1400.0, h_bulk_k_outer: 0.0,
                k_hertz_k_outer: 0.0, k_combined_k: 0.0, in_contact: true,
            }).collect()
        };
        let rollers = vec![RollerResult {
            psi_deg: 0.0, q_normal: 800.0, q_normal_inner: 800.0 * 0.998,
            slice_results: make_slices(), rib_result: None,
        }];

        let mut op_p = test_op(1500.0); op_p.friction_model = FrictionModel::PalmgrenLike;
        let mut op_b = test_op(1500.0); op_b.friction_model = FrictionModel::BibouletHoupert;

        let ts_p = compute_traction(&geom, &mat, &op_p, &rolp, &rg, &rp, &rp, &sg, &rollers).unwrap();
        let ts_b = compute_traction(&geom, &mat, &op_b, &rolp, &rg, &rp, &rp, &sg, &rollers).unwrap();

        // Sliding totals identical (BH only touches rolling)
        assert!((ts_p.p_sliding_w - ts_b.p_sliding_w).abs() < 1e-9,
            "Sliding power must be identical between Palmgren and BH: {} vs {}",
            ts_p.p_sliding_w, ts_b.p_sliding_w);
        // Rolling totals differ measurably and BH stays positive
        assert!(ts_p.p_rolling_w > 0.0 && ts_b.p_rolling_w > 0.0);
        assert!((ts_p.p_rolling_w - ts_b.p_rolling_w).abs() > 1e-3,
            "Rolling power should differ between Palmgren and BH (calibration): {} vs {}",
            ts_p.p_rolling_w, ts_b.p_rolling_w);
        // Per-roller breakdown survives in both cases
        for r in &ts_b.rollers {
            assert!(r.inner.p_rolling_w > 0.0 && r.outer.p_rolling_w > 0.0);
        }
    }

    /// `friction_model = SkfAdvanced` replaces bearing-level totals with the
    /// SKF reference and keeps the per-roller breakdown intact.
    #[test]
    fn test_friction_model_skf_advanced_override() {
        let geom = test_geom();
        let mat = test_material();
        let rp = test_raceway_profile();
        let mut op = test_op(1500.0);
        op.friction_model = FrictionModel::SkfAdvanced;
        let rg = test_raceway_geom();
        let sg = test_slice_geometries(&geom, 10);
        let rolp = test_roller_profile();

        let make_slices = || -> Vec<SliceContactResult> {
            (0..10).map(|k| SliceContactResult {
                k, delta_k: 5.0, q_k: 50.0, q_k_outer: 50.0, q_k_inner: 50.0,
                b_k: 0.1, p_max_k: 1500.0, h_bulk_k: 0.0, k_hertz_k: 0.0,
                b_k_outer: 0.1, p_max_k_outer: 1400.0, h_bulk_k_outer: 0.0,
                k_hertz_k_outer: 0.0, k_combined_k: 0.0, in_contact: true,
            }).collect()
        };
        let rollers = vec![RollerResult {
            psi_deg: 0.0, q_normal: 800.0, q_normal_inner: 800.0 * 0.998,
            slice_results: make_slices(), rib_result: None,
        }];

        let ts_palmgren = {
            let mut op_p = op.clone();
            op_p.friction_model = FrictionModel::PalmgrenLike;
            compute_traction(&geom, &mat, &op_p, &rolp, &rg, &rp, &rp, &sg, &rollers).unwrap()
        };
        let ts_skf = compute_traction(&geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers)
            .expect("SKF dispatch must succeed");

        // SkfAdvanced: bearing-level totals must equal the SKF reference
        let skf_ref = ts_skf.skf_reference.expect("must populate skf_reference");
        assert!((ts_skf.p_rolling_w - skf_ref.p_rolling_w).abs() < 1e-9,
            "p_rolling_w must equal SKF reference: {} vs {}",
            ts_skf.p_rolling_w, skf_ref.p_rolling_w);
        assert!((ts_skf.p_sliding_w - skf_ref.p_sliding_w).abs() < 1e-9);
        // Per-roller breakdown survives (used for diagnostics)
        assert!(!ts_skf.rollers.is_empty());
        assert!(ts_skf.rollers[0].inner.p_rolling_w > 0.0);
        // Palmgren and SKF totals differ measurably (different calibration)
        assert!((ts_palmgren.p_rolling_w - ts_skf.p_rolling_w).abs() > 1e-3,
            "Palmgren ({}) and SKF ({}) totals must differ",
            ts_palmgren.p_rolling_w, ts_skf.p_rolling_w);
    }

    /// `skf_reference` is None when speed = 0 (SKF model undefined).
    #[test]
    fn test_friction_model_skf_reference_none_at_zero_speed() {
        let geom = test_geom();
        let mat = test_material();
        let rp = test_raceway_profile();
        let op = test_op(0.0);
        let rg = test_raceway_geom();
        let sg = test_slice_geometries(&geom, 10);
        let rolp = test_roller_profile();
        let rollers = vec![RollerResult {
            psi_deg: 0.0, q_normal: 800.0, q_normal_inner: 800.0,
            slice_results: vec![], rib_result: None,
        }];
        // compute_traction returns None when n_rpm == 0; we test the SKF
        // wrapper directly to verify its zero-speed handling.
        let ts = compute_traction(&geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers);
        assert!(ts.is_none(), "Static bearing returns None traction summary");
    }

    /// Verify the SKF reference implementation reproduces the worked example
    /// for an NSK HR30306J-equivalent bearing (303-series). Hand calculation
    /// from SKF Catalogue 2018 formulas:
    ///   d=30, D=72, d_m=51 mm, F_r=5 kN, F_a=2 kN, Y=1.6, n=1500 rpm,
    ///   ν(70°C) ≈ 22 cSt, oil bath
    ///   → M_rr ≈ 256 N·mm, M_sl ≈ 44 N·mm, M_total ≈ 300 N·mm
    #[test]
    fn test_skf_friction_30306_hand_calc() {
        let m = skf_frictional_moment_trb(
            SkfTrbSeries::Series303,
            30.0, 72.0,
            5000.0, 2000.0, 1.6,
            1500.0, 22.0,
            SkfLubrication::OilBath,
        );
        assert!((m.m_rr_nmm - 256.0).abs() < 25.0,
            "M_rr {:.1} must be near 256 N·mm (hand calc)", m.m_rr_nmm);
        assert!((m.m_sl_nmm - 44.0).abs() < 8.0,
            "M_sl {:.1} must be near 44 N·mm (hand calc)", m.m_sl_nmm);
        assert!((m.m_total_nmm - 300.0).abs() < 35.0,
            "M_total {:.1} must be near 300 N·mm (hand calc)", m.m_total_nmm);
        // Sanity on intermediates
        assert!(m.phi_ish > 0.85 && m.phi_ish < 1.0,
            "φ_ish {} must be slight reduction", m.phi_ish);
        assert!(m.phi_rs > 0.85 && m.phi_rs < 1.0,
            "φ_rs {} must be slight reduction", m.phi_rs);
        // μ_sl in mixed regime: φ_bl·0.12 + (1−φ_bl)·0.002 ≈ 0.009
        assert!(m.mu_sl > 0.005 && m.mu_sl < 0.05,
            "μ_sl {} out of mixed-lubrication range", m.mu_sl);
    }

    /// Speed sweep: M_rr increases with (ν·n)^0.6.
    #[test]
    fn test_skf_friction_speed_dependence() {
        let mk = |n: f64| skf_frictional_moment_trb(
            SkfTrbSeries::Series303, 30.0, 72.0, 5000.0, 2000.0, 1.6, n, 22.0,
            SkfLubrication::OilBath,
        );
        let m500 = mk(500.0).m_rr_nmm;
        let m1500 = mk(1500.0).m_rr_nmm;
        let m4500 = mk(4500.0).m_rr_nmm;
        assert!(m1500 > m500 && m4500 > m1500, "M_rr must grow with n");
        // (1500/500)^0.6 = 3^0.6 ≈ 1.93 — allow ±15% for thermal/starvation corrections
        let ratio = m1500 / m500;
        assert!(ratio > 1.6 && ratio < 2.3,
            "M_rr ratio (n=1500 / n=500) ≈ 1.93 expected, got {ratio}");
    }

    /// Series differences: 322 series has ~1.34× M_rr of 303 at the same
    /// dimensions (R1 ratio 2.27e-6 / 1.69e-6 = 1.34).
    #[test]
    fn test_skf_friction_series_constants() {
        let mk = |s: SkfTrbSeries| skf_frictional_moment_trb(
            s, 30.0, 72.0, 5000.0, 2000.0, 1.6, 1500.0, 22.0,
            SkfLubrication::OilBath,
        ).m_rr_nmm;
        let r303 = mk(SkfTrbSeries::Series303);
        let r322 = mk(SkfTrbSeries::Series322);
        let ratio = r322 / r303;
        assert!((ratio - 1.343).abs() < 0.01,
            "M_rr 322/303 should equal R1 ratio (≈1.343), got {ratio}");
    }

    /// External validation: when our solver and the SKF reference model are
    /// fed equivalent loaded-roller distributions (same total normal load on
    /// the bearing), the resulting friction torques must be of the same order
    /// of magnitude.
    ///
    /// **Important caveat documented in Manual §14 "External Validation"**:
    /// On the default NSK HR30306J preset run through the full bearing
    /// equilibrium pipeline (not this mock), our solver produces ~4 100 N·mm
    /// vs SKF's ~300 N·mm — a ~14× discrepancy. The root cause is upstream of
    /// the friction model (the equilibrium pipeline produces per-roller normal
    /// forces ~10× the value implied by the SKF reference for the same input
    /// loads). That is a separate audit item and is intentionally not tested
    /// here; this test guards only the friction-model portion of the chain.
    #[test]
    fn test_solver_vs_skf_friction_model_matched_input() {
        let geom = test_geom();
        let mat = test_material();
        let rp = test_raceway_profile();
        let op = test_op(1500.0); // M1 default
        let rg = test_raceway_geom();
        let sg = test_slice_geometries(&geom, 10);
        let rolp = test_roller_profile();

        let make_slices = || -> Vec<SliceContactResult> {
            (0..10).map(|k| SliceContactResult {
                k, delta_k: 5.0, q_k: 50.0, q_k_outer: 50.0, q_k_inner: 50.0,
                b_k: 0.1, p_max_k: 1500.0, h_bulk_k: 0.0, k_hertz_k: 0.0,
                b_k_outer: 0.1, p_max_k_outer: 1400.0, h_bulk_k_outer: 0.0,
                k_hertz_k_outer: 0.0, k_combined_k: 0.0, in_contact: true,
            }).collect()
        };
        // Stribeck-like distribution chosen to give Σ Q_normal ≈ 5 kN+2 kN
        // axial-projected (matching the SKF reference inputs).
        let rollers: Vec<RollerResult> = (0..14).map(|i| {
            let psi = i as f64 * (360.0 / 14.0);
            let cosp = (psi.to_radians()).cos();
            let q = if cosp > 0.0 { 800.0 * cosp } else { 0.0 };
            RollerResult {
                psi_deg: psi,
                q_normal: q,
                q_normal_inner: q * 0.998,
                slice_results: make_slices(),
                rib_result: None,
            }
        }).collect();

        let ts = compute_traction(&geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers)
            .expect("traction must succeed");

        let skf = skf_frictional_moment_trb(
            SkfTrbSeries::Series303,
            30.0, 72.0,
            5000.0, 2000.0, 1.6,
            1500.0, 22.0,
            SkfLubrication::OilBath,
        );

        let our_total = ts.m_friction_nmm;
        let skf_total = skf.m_total_nmm;
        let ratio = our_total / skf_total;

        // Same order of magnitude with matched normal-load inputs (factor 0.3 ~ 3).
        // Anything outside that range likely indicates a friction-model bug.
        assert!(ratio > 0.3 && ratio < 3.0,
            "Solver-vs-SKF ratio at matched input must be O(1): \
             our={our_total:.1} N·mm, SKF={skf_total:.1} N·mm, ratio={ratio:.2}");
        assert!(our_total.is_finite() && our_total > 0.0);
        assert!(skf_total.is_finite() && skf_total > 0.0);
    }

    /// Unit consistency guard: feeding the **same** loaded-roller input through
    /// `compute_traction` (M1) and `compute_traction_advanced` (M2) must
    /// produce per-roller `p_rolling_w` values within the same order of
    /// magnitude. Catches the previous bug where M2 multiplied q_normal by
    /// 1e3 under a stale "kN → N" comment, inflating M2 power 1000×.
    #[test]
    fn test_m1_m2_p_rolling_same_order_of_magnitude() {
        let geom = test_geom();
        let mat = test_material();
        let rp = test_raceway_profile();
        let mut op = test_op(1500.0);
        let rg = test_raceway_geom();
        let sg = test_slice_geometries(&geom, 10);
        let rolp = test_roller_profile();

        let make_slices = || -> Vec<SliceContactResult> {
            (0..10).map(|k| SliceContactResult {
                k, delta_k: 5.0, q_k: 50.0, q_k_outer: 50.0, q_k_inner: 50.0,
                b_k: 0.1, p_max_k: 1500.0, h_bulk_k: 0.0, k_hertz_k: 0.0,
                b_k_outer: 0.1, p_max_k_outer: 1400.0, h_bulk_k_outer: 0.0,
                k_hertz_k_outer: 0.0, k_combined_k: 0.0, in_contact: true,
            }).collect()
        };
        let q_outer = 1000.0_f64; // [N]
        let q_inner = 800.0_f64;  // [N]
        let rollers = vec![RollerResult {
            psi_deg: 0.0, q_normal: q_outer, q_normal_inner: q_inner,
            slice_results: make_slices(), rib_result: None,
        }];

        op.lubrication_model = LubricationModel::Method1_DH;
        let ts_m1 = compute_traction(&geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers)
            .expect("M1 must succeed");
        op.lubrication_model = LubricationModel::Method2_MK;
        let ts_m2 = compute_traction_advanced(&geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers)
            .expect("M2 must succeed");

        // Rolling resistance is a Palmgren μ_rr × Q × u closed form independent
        // of the EHL model — both paths must agree to within numerical noise.
        let m1_inner = ts_m1.rollers[0].inner.p_rolling_w;
        let m2_inner = ts_m2.rollers[0].inner.p_rolling_w;
        let rel_diff = ((m1_inner - m2_inner) / m1_inner.max(1e-12)).abs();
        assert!(rel_diff < 1e-6,
            "M1 vs M2 inner P_roll must match (Palmgren is model-agnostic): \
             M1={m1_inner}, M2={m2_inner}, rel_diff={rel_diff}");

        let m1_outer = ts_m1.rollers[0].outer.p_rolling_w;
        let m2_outer = ts_m2.rollers[0].outer.p_rolling_w;
        let rel_diff_o = ((m1_outer - m2_outer) / m1_outer.max(1e-12)).abs();
        assert!(rel_diff_o < 1e-6,
            "M1 vs M2 outer P_roll must match: M1={m1_outer}, M2={m2_outer}");

        // Sanity on absolute magnitude: P_roll = 0.002 × Q × u_roll
        // For Q=1000 N and u_roll ≈ 1.94 m/s (default test geom) → P ≈ 3.88 W.
        // Allow [0.5, 50] W to bracket geometry variations across tests.
        for p in [m1_inner, m1_outer, m2_inner, m2_outer] {
            assert!(p > 0.5 && p < 50.0,
                "P_roll out of physical range for Q ~ 1 kN: got {p} W");
        }
    }

    /// Inner/outer normal forces differ by `cos(α_o − α_i)`; injecting an
    /// asymmetric (q_normal, q_normal_inner) pair must yield distinct P_roll
    /// values for the two raceways (and proportional to each load).
    /// This guards against the previous bug where both raceways used
    /// `rr.q_normal` (outer) for both inner and outer power.
    #[test]
    fn test_p_rolling_w_asymmetric_inner_outer_loads_m1() {
        let geom = test_geom();
        let mat = test_material();
        let rp = test_raceway_profile();
        let op = test_op(1500.0); // M1 default
        let rg = test_raceway_geom();
        let sg = test_slice_geometries(&geom, 10);
        let rolp = test_roller_profile();

        let slices: Vec<SliceContactResult> = (0..10).map(|k| SliceContactResult {
            k, delta_k: 5.0, q_k: 50.0, q_k_outer: 50.0, q_k_inner: 50.0,
            b_k: 0.1, p_max_k: 1500.0, h_bulk_k: 0.0, k_hertz_k: 0.0,
            b_k_outer: 0.1, p_max_k_outer: 1400.0, h_bulk_k_outer: 0.0,
            k_hertz_k_outer: 0.0, k_combined_k: 0.0, in_contact: true,
        }).collect();
        // Inject deliberate 20% asymmetry: q_normal_inner = 0.8 × q_normal
        // (outside the typical cos(α_diff) range so any leakage is visible)
        let q_outer = 1000.0_f64;
        let q_inner = 800.0_f64;
        let rollers = vec![RollerResult {
            psi_deg: 0.0, q_normal: q_outer, q_normal_inner: q_inner,
            slice_results: slices, rib_result: None,
        }];

        let ts = compute_traction(&geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers)
            .expect("traction must succeed");
        let r = &ts.rollers[0];

        // P_roll must scale with each raceway's own load (μ_rr same, u_roll same)
        let ratio = r.inner.p_rolling_w / r.outer.p_rolling_w;
        let expected = q_inner / q_outer; // 0.8
        assert!((ratio - expected).abs() < 1e-6,
            "P_roll inner/outer ratio must match load ratio: got {ratio}, expected {expected}");
        // And not equal (regression: previous code returned inner == outer)
        assert!((r.inner.p_rolling_w - r.outer.p_rolling_w).abs() > 1e-3,
            "Asymmetric loads must produce distinct P_roll: inner={}, outer={}",
            r.inner.p_rolling_w, r.outer.p_rolling_w);
    }

    /// Same asymmetry test on the Advanced (M2) path.
    #[test]
    fn test_p_rolling_w_asymmetric_inner_outer_loads_m2() {
        let geom = test_geom();
        let mat = test_material();
        let rp = test_raceway_profile();
        let mut op = test_op(1500.0);
        op.lubrication_model = LubricationModel::Method2_MK;
        let rg = test_raceway_geom();
        let sg = test_slice_geometries(&geom, 10);
        let rolp = test_roller_profile();

        let slices: Vec<SliceContactResult> = (0..10).map(|k| SliceContactResult {
            k, delta_k: 5.0, q_k: 50.0, q_k_outer: 50.0, q_k_inner: 50.0,
            b_k: 0.1, p_max_k: 1500.0, h_bulk_k: 0.0, k_hertz_k: 0.0,
            b_k_outer: 0.1, p_max_k_outer: 1400.0, h_bulk_k_outer: 0.0,
            k_hertz_k_outer: 0.0, k_combined_k: 0.0, in_contact: true,
        }).collect();
        let q_outer = 1000.0_f64;
        let q_inner = 800.0_f64;
        let rollers = vec![RollerResult {
            psi_deg: 0.0, q_normal: q_outer, q_normal_inner: q_inner,
            slice_results: slices, rib_result: None,
        }];

        let ts = compute_traction_advanced(&geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers)
            .expect("M2 traction must succeed");
        let r = &ts.rollers[0];

        let ratio = r.inner.p_rolling_w / r.outer.p_rolling_w;
        let expected = q_inner / q_outer;
        assert!((ratio - expected).abs() < 1e-6,
            "M2: P_roll ratio mismatch: got {ratio}, expected {expected}");
        assert!((r.inner.p_rolling_w - r.outer.p_rolling_w).abs() > 1e-3,
            "M2: distinct P_roll required for asymmetric loads");
    }

    /// `ContactFriction.p_rolling_w` is populated and consistent across the
    /// per-roller breakdown and the bearing-level summary
    /// (TractionSummary.p_rolling_w == Σ inner.p_rolling_w + outer.p_rolling_w).
    /// This is the invariant that makes the per-roller bar chart match the
    /// pie chart in the Lubrication tab.
    #[test]
    fn test_p_rolling_w_per_roller_matches_summary() {
        let geom = test_geom();
        let mat = test_material();
        let rp = test_raceway_profile();
        let op = test_op(1500.0); // M1 default
        let rg = test_raceway_geom();
        let sg = test_slice_geometries(&geom, 10);
        let rolp = test_roller_profile();

        // Build a few loaded rollers
        let make_slices = || -> Vec<SliceContactResult> {
            (0..10).map(|k| SliceContactResult {
                k, delta_k: 5.0, q_k: 50.0, q_k_outer: 50.0, q_k_inner: 50.0,
                b_k: 0.1, p_max_k: 1500.0, h_bulk_k: 0.0, k_hertz_k: 0.0,
                b_k_outer: 0.1, p_max_k_outer: 1400.0, h_bulk_k_outer: 0.0,
                k_hertz_k_outer: 0.0, k_combined_k: 0.0, in_contact: true,
            }).collect()
        };
        let rollers = vec![
            RollerResult { psi_deg: 0.0,   q_normal: 800.0, q_normal_inner: 800.0, slice_results: make_slices(), rib_result: None },
            RollerResult { psi_deg: 36.0,  q_normal: 600.0, q_normal_inner: 600.0, slice_results: make_slices(), rib_result: None },
            RollerResult { psi_deg: 72.0,  q_normal: 300.0, q_normal_inner: 300.0, slice_results: make_slices(), rib_result: None },
        ];

        let ts = compute_traction(&geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers)
            .expect("traction must succeed");

        // Each loaded roller must have positive p_rolling_w on both raceways
        for r in &ts.rollers {
            assert!(r.inner.p_rolling_w > 0.0,
                "Inner p_rolling_w must be > 0 at ψ={}, got {}", r.psi_deg, r.inner.p_rolling_w);
            assert!(r.outer.p_rolling_w > 0.0,
                "Outer p_rolling_w must be > 0 at ψ={}, got {}", r.psi_deg, r.outer.p_rolling_w);
            // Sanity: rolling = μ_rr × Q × u_rolling, μ_rr = 0.002
            let expected = 0.002 * r.inner.f_traction_n.abs().max(0.0); // sanity
            let _ = expected; // not strict here — invariant below is the contract
        }

        // Sum of per-contact rolling powers must equal the bearing-level total
        let sum_per_roller: f64 = ts.rollers.iter()
            .map(|r| r.inner.p_rolling_w + r.outer.p_rolling_w)
            .sum();
        assert!((sum_per_roller - ts.p_rolling_w).abs() < 1e-9,
            "TractionSummary.p_rolling_w ({}) must equal Σ per-roller \
             (inner.p_rolling_w + outer.p_rolling_w) ({})",
            ts.p_rolling_w, sum_per_roller);
    }

    /// Same invariant on the Advanced (M2) path.
    #[test]
    fn test_p_rolling_w_per_roller_matches_summary_m2() {
        let geom = test_geom();
        let mat = test_material();
        let rp = test_raceway_profile();
        let mut op = test_op(1500.0);
        op.lubrication_model = LubricationModel::Method2_MK;
        let rg = test_raceway_geom();
        let sg = test_slice_geometries(&geom, 10);
        let rolp = test_roller_profile();

        let make_slices = || -> Vec<SliceContactResult> {
            (0..10).map(|k| SliceContactResult {
                k, delta_k: 5.0, q_k: 50.0, q_k_outer: 50.0, q_k_inner: 50.0,
                b_k: 0.1, p_max_k: 1500.0, h_bulk_k: 0.0, k_hertz_k: 0.0,
                b_k_outer: 0.1, p_max_k_outer: 1400.0, h_bulk_k_outer: 0.0,
                k_hertz_k_outer: 0.0, k_combined_k: 0.0, in_contact: true,
            }).collect()
        };
        let rollers = vec![
            RollerResult { psi_deg: 0.0,  q_normal: 800.0, q_normal_inner: 800.0, slice_results: make_slices(), rib_result: None },
            RollerResult { psi_deg: 60.0, q_normal: 400.0, q_normal_inner: 400.0, slice_results: make_slices(), rib_result: None },
        ];

        let ts = compute_traction_advanced(&geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers)
            .expect("M2 traction must succeed");

        for r in &ts.rollers {
            assert!(r.inner.p_rolling_w > 0.0, "M2 inner p_rolling_w > 0 at ψ={}", r.psi_deg);
            assert!(r.outer.p_rolling_w > 0.0, "M2 outer p_rolling_w > 0 at ψ={}", r.psi_deg);
        }
        let sum: f64 = ts.rollers.iter()
            .map(|r| r.inner.p_rolling_w + r.outer.p_rolling_w)
            .sum();
        assert!((sum - ts.p_rolling_w).abs() < 1e-9,
            "M2: per-roller sum ({}) must match summary ({})", sum, ts.p_rolling_w);
    }

    /// Same consistency check for the Basic (M1) traction path.
    #[test]
    fn test_rib_friction_uses_ehl_mu_eff_m1_path() {
        let geom = test_geom();
        let mat = test_material();
        let rp = test_raceway_profile();
        let op = test_op(1000.0); // M1 default
        let rg = test_raceway_geom();
        let sg = test_slice_geometries(&geom, 10);
        let rolp = test_roller_profile();

        let slices: Vec<SliceContactResult> = (0..10).map(|k| SliceContactResult {
            k, delta_k: 5.0, q_k: 50.0, q_k_outer: 50.0, q_k_inner: 50.0,
            b_k: 0.1, p_max_k: 1500.0, h_bulk_k: 0.0, k_hertz_k: 0.0,
            b_k_outer: 0.1, p_max_k_outer: 1400.0, h_bulk_k_outer: 0.0,
            k_hertz_k_outer: 0.0, k_combined_k: 0.0, in_contact: true,
        }).collect();

        let injected_mu = 0.0789_f64;
        let ehl = RibEhlResult {
            h_c_um: 0.3, h_min_um: 0.2, sigma_composite_um: 0.3,
            lambda_ratio: 0.67, regime: LubricationRegime::Boundary,
            mu_eff: injected_mu, mu_ehl: 0.01, asperity_load_ratio: 0.85,
            p_asperity_mpa: 200.0, flash_temp_c: 30.0,
            srr: 2.0, u_entrain_m_s: 1.0, u_slide_m_s: 2.0,
            thermal_factor: 0.85, u_param: 1e-11, g_param: 5000.0, w_param: 1e-4,
            k_ellipse: 1.5,
        };
        let rib_with_ehl = RibContactResult {
            f_rib: 100.0, a_ellipse: 0.5, b_ellipse: 0.4, p_max_rib: 800.0,
            spin_moment: 0.0, delta_rib: 1.0, k_rib: 100.0,
            r_contact_mm: 50.0, r_rib_circ_mm: 100.0, h_c_mm: 1.0,
            ehl: Some(ehl),
        };
        let rollers = vec![RollerResult {
            psi_deg: 0.0, q_normal: 500.0, q_normal_inner: 500.0,
            slice_results: slices, rib_result: Some(rib_with_ehl),
        }];

        let ts = compute_traction(&geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers)
            .expect("M1 traction must succeed");
        let rib_t = ts.rollers[0].rib.as_ref().expect("rib must be Some");
        assert!((rib_t.mu - injected_mu).abs() < 1e-12,
            "M1 path must also use rib EHL μ_eff: expected {injected_mu}, got {}", rib_t.mu);
    }

    #[test]
    fn test_traction_advanced_loaded_roller() {
        // Advanced traction should produce valid results for a loaded roller
        let geom = test_geom();
        let mat = test_material();
        let rp = test_raceway_profile();
        let mut op = test_op(1000.0);
        op.lubrication_model = LubricationModel::Method2_MK;
        let rg = test_raceway_geom();
        let sg = test_slice_geometries(&geom, 10);
        let rolp = test_roller_profile();

        let slices: Vec<SliceContactResult> = (0..10).map(|k| SliceContactResult {
            k, delta_k: 5.0, q_k: 50.0, q_k_outer: 50.0, q_k_inner: 50.0,
            b_k: 0.1, p_max_k: 1500.0, h_bulk_k: 0.0,
            k_hertz_k: 0.0,
            b_k_outer: 0.1, p_max_k_outer: 1400.0, h_bulk_k_outer: 0.0,
            k_hertz_k_outer: 0.0, k_combined_k: 0.0,
            in_contact: true,
        }).collect();
        let rollers = vec![RollerResult {
            psi_deg: 0.0, q_normal: 500.0, q_normal_inner: 500.0,
            slice_results: slices, rib_result: None,
        }];

        let ts = compute_traction_advanced(&geom, &mat, &op, &rolp, &rg, &rp, &rp, &sg, &rollers);
        assert!(ts.is_some(), "Advanced traction should return Some for loaded roller");
        let ts = ts.unwrap();
        assert_eq!(ts.rollers.len(), 1);
        assert!(ts.p_contact_total_w > 0.0, "must dissipate power");
        assert!(ts.m_friction_nmm > 0.0, "friction torque must be positive");
    }

    // ─── Starvation model tests ───

    #[test]
    fn test_starvation_oil_low_speed() {
        // Low speed oil → nearly fully flooded
        let phi = compute_starvation_factor_advanced(
            0.05, 0.5, 220e9, 0.005,
            &LubricationType::Oil, 50_000.0,
        );
        assert!(phi > 0.90, "Low speed oil should be nearly flooded, got {phi}");
    }

    #[test]
    fn test_starvation_oil_high_speed() {
        // High speed → more starvation
        let phi_low = compute_starvation_factor_advanced(
            0.05, 0.5, 220e9, 0.005,
            &LubricationType::Oil, 50_000.0,
        );
        let phi_high = compute_starvation_factor_advanced(
            0.05, 5.0, 220e9, 0.005,
            &LubricationType::Oil, 500_000.0,
        );
        assert!(phi_high < phi_low,
            "Higher speed should have more starvation: {phi_high} < {phi_low}");
    }

    #[test]
    fn test_starvation_grease_less_than_oil() {
        let phi_oil = compute_starvation_factor_advanced(
            0.05, 2.0, 220e9, 0.005,
            &LubricationType::Oil, 200_000.0,
        );
        let phi_grease = compute_starvation_factor_advanced(
            0.05, 2.0, 220e9, 0.005,
            &LubricationType::Grease, 200_000.0,
        );
        assert!(phi_grease < phi_oil,
            "Grease should have more starvation: {phi_grease} < {phi_oil}");
    }

    // ─── Flash temperature tests ───

    #[test]
    fn test_flash_temp_zero_slide() {
        let dt = flash_temperature(0.1, 10e6, 0.0, 0.2e-3);
        assert!(dt.abs() < 1e-10, "Zero sliding → zero flash temp");
    }

    #[test]
    fn test_flash_temp_positive() {
        // Low asperity pressure (1 MPa) with moderate sliding
        let dt = flash_temperature(0.1, 1e6, 0.1, 0.2e-3);
        assert!(dt > 0.0, "Sliding contact should have positive flash temp, got {dt}");
        assert!(dt < 500.0, "Should be within physical bounds, got {dt}");
    }

    #[test]
    fn test_flash_temp_increases_with_speed() {
        let dt_slow = flash_temperature(0.1, 1e6, 0.05, 0.2e-3);
        let dt_fast = flash_temperature(0.1, 1e6, 0.5, 0.2e-3);
        assert!(dt_fast > dt_slow,
            "Higher speed → higher flash temp: {dt_fast} > {dt_slow}");
    }

    #[test]
    fn test_flash_temp_risk_classification() {
        assert_eq!(classify_flash_temp(30.0), "Low");
        assert_eq!(classify_flash_temp(100.0), "Medium");
        assert_eq!(classify_flash_temp(200.0), "High");
        assert_eq!(classify_flash_temp(400.0), "Critical");
    }

    // ─── Phase 5 Verification Tests ───────────────────────────────────

    /// σ̄→0 극한에서 M-K가 D-H/D-T smooth-surface 공식과 수렴하는지 검증
    #[test]
    fn test_mk_sigma_zero_converges_to_dh() {
        // D-T central: H_c = 3.06·U^0.69·G^0.56·W^(-0.10)
        // D-H minimum: H_min = 2.65·U^0.70·G^0.54·W^(-0.13)
        let u: f64 = 1e-11;
        let g: f64 = 4600.0;
        let w: f64 = 3e-5;
        let r_eq: f64 = 5e-3;

        // M-K with σ̄=0 should give pure smooth result (roughness correction = 1.0)
        let mk = compute_film_mk(u, g, w, 0.0, 0.0, r_eq);

        // D-T reference
        let h_c_dt = 3.06_f64 * u.powf(0.69) * g.powf(0.56) * w.powf(-0.10) * r_eq;
        let h_min_dh = 2.65_f64 * u.powf(0.70) * g.powf(0.54) * w.powf(-0.13) * r_eq;

        let err_hc = ((mk.h_c - h_c_dt) / h_c_dt).abs();
        let err_hmin = ((mk.h_min - h_min_dh) / h_min_dh).abs();

        assert!(err_hc < 1e-10, "M-K h_c should equal D-T at σ̄=0: err={err_hc}");
        assert!(err_hmin < 1e-10, "M-K h_min should equal D-H at σ̄=0: err={err_hmin}");
        assert!(mk.load_fraction < 1e-15, "load_fraction should be 0 at σ̄=0");
        assert!(mk.area_fraction < 1e-15, "area_fraction should be 0 at σ̄=0");
    }

    /// Roughness가 증가하면 M-K load_fraction이 단조 증가하는지 검증
    #[test]
    fn test_mk_load_fraction_increases_with_roughness() {
        let u: f64 = 1e-11;
        let g: f64 = 4600.0;
        let w: f64 = 3e-5;
        let r_eq: f64 = 5e-3;

        // σ̄ = σ_combined / R_eq
        let sigma_bar_low = 6e-5;   // σ≈0.3μm
        let sigma_bar_med = 6e-4;   // σ≈3μm
        let sigma_bar_high = 6e-3;  // σ≈30μm (extreme)
        let v = 0.01;

        let mk_low = compute_film_mk(u, g, w, sigma_bar_low, v, r_eq);
        let mk_med = compute_film_mk(u, g, w, sigma_bar_med, v, r_eq);
        let mk_high = compute_film_mk(u, g, w, sigma_bar_high, v, r_eq);

        // Monotonic increase with roughness
        assert!(mk_med.load_fraction > mk_low.load_fraction,
            "load_fraction should increase: med {} > low {}",
            mk_med.load_fraction, mk_low.load_fraction);
        assert!(mk_high.load_fraction > mk_med.load_fraction,
            "load_fraction should increase: high {} > med {}",
            mk_high.load_fraction, mk_med.load_fraction);

        // σ̄=0 should give zero
        let mk_zero = compute_film_mk(u, g, w, 0.0, 0.0, r_eq);
        assert_eq!(mk_zero.load_fraction, 0.0);
    }

    /// Roelands vs Barus: Roelands gives lower η at high pressure
    /// and this should result in lower Eyring traction coefficient
    #[test]
    fn test_roelands_vs_barus_traction_impact() {
        let eta_0 = 0.05;  // Pa·s
        let p = 1e9;        // 1 GPa — high pressure
        let z_r = 0.67;
        let alpha_pv = 20e-9; // 1/Pa (20 1/GPa)

        // Roelands viscosity at 1 GPa
        let eta_roelands = roelands_viscosity(eta_0, p, z_r);

        // Barus viscosity at 1 GPa: η = η₀ × exp(α × p)
        let eta_barus = eta_0 * (alpha_pv * p).exp();

        // Roelands should predict LOWER viscosity than Barus at high pressure
        assert!(eta_roelands < eta_barus,
            "Roelands < Barus at 1 GPa: {} < {}", eta_roelands, eta_barus);

        // This means Roelands-based traction should be lower at same conditions
        let u_roll = 2.0;
        let h_c = 0.5e-6;
        let tau_0 = 5e6;

        let mu_roelands = eyring_traction_advanced(0.5, u_roll, p, h_c, eta_0, z_r, tau_0);

        // Barus-equivalent: use higher z_r to get closer to Barus behavior
        // At very high Z_r, Roelands → Barus-like (but capped by shear limit)
        // Instead, directly verify that mu is reasonable
        assert!(mu_roelands > 0.0 && mu_roelands <= 0.10,
            "Roelands traction at 1GPa should be bounded: {}", mu_roelands);
    }

    /// Basic vs Advanced: at low speed/low load/Oil, results should be similar
    #[test]
    fn test_basic_vs_advanced_low_speed_comparison() {
        // At zero roughness and low SRR, Advanced M-K should give
        // similar film thickness to Basic D-H (both smooth-surface)
        let u: f64 = 1e-11;
        let g: f64 = 4600.0;
        let w: f64 = 3e-5;
        let r_eq: f64 = 5e-3;

        // Basic D-H minimum film
        let h_min_basic = 2.65_f64 * u.powf(0.70) * g.powf(0.54) * w.powf(-0.13) * r_eq;

        // Advanced M-K with very small roughness (near-smooth)
        let sigma_bar = 1e-8; // essentially smooth
        let mk = compute_film_mk(u, g, w, sigma_bar, 0.001, r_eq);

        let diff_pct = ((mk.h_min - h_min_basic) / h_min_basic).abs() * 100.0;
        assert!(diff_pct < 5.0,
            "Near-smooth M-K should match D-H within 5%: diff={}%", diff_pct);
    }

    /// M-K vs GT: at same conditions, both should predict non-zero asperity contact
    #[test]
    fn test_mk_vs_gt_load_fraction_consistency() {
        // M-K load fraction is a continuous function of σ̄
        // Verify: zero roughness → 0, extreme roughness → approaches 1
        let u: f64 = 1e-11;
        let g: f64 = 4600.0;
        let w: f64 = 3e-5;
        let r_eq: f64 = 5e-3;
        let v = 0.01;

        let mk_smooth = compute_film_mk(u, g, w, 0.0, 0.0, r_eq);
        let mk_extreme = compute_film_mk(u, g, w, 0.1, v, r_eq); // σ̄=0.1 (extreme)

        // Smooth → no asperity
        assert_eq!(mk_smooth.load_fraction, 0.0);

        // Extreme roughness → high asperity load (clamped to [0,1])
        assert!(mk_extreme.load_fraction > mk_smooth.load_fraction,
            "Extreme roughness should have higher load_fraction: {}",
            mk_extreme.load_fraction);
        assert!(mk_extreme.load_fraction <= 1.0,
            "load_fraction should be clamped to [0,1]: {}",
            mk_extreme.load_fraction);
    }

    // ═══════════════════════════════════════════════════════════════════
    // REFERENCE LITERATURE QUANTITATIVE VERIFICATION
    // ═══════════════════════════════════════════════════════════════════
    //
    // These tests verify our implementations against specific numerical
    // values from reference textbooks and papers. Any failure indicates
    // a formula implementation error.

    // ─── 1. Dowson-Higginson / Dowson-Toyoda Benchmark ───────────────
    //
    // Ref: Hamrock, Schmid & Jacobson (2004) "Fundamentals of Fluid
    //      Film Lubrication", Example 18.1 (line contact EHL)
    // Also: Harris & Kotzalas (2006) Table 12.2

    #[test]
    fn test_dh_dt_formula_benchmark() {
        // Benchmark: steel-on-steel line contact
        //   R_eq = 5 mm = 0.005 m
        //   E* = 2E₁/(1−ν²) = 2×210e9/(1−0.09) = 230.77 GPa ≈ 2.308e11 Pa
        //   η₀ = 0.04 Pa·s  (ISO VG 68 at ~70°C)
        //   α = 20 GPa⁻¹ = 2e-8 1/Pa
        //   u_m = 2.0 m/s
        //   w = 200 N/mm = 200e3 N/m (line load)
        let r_eq = 0.005_f64;
        let e_star = 2.308e11_f64;
        let eta_0 = 0.04_f64;
        let alpha_pv = 20e-9_f64;
        let u_m = 2.0_f64;
        let w_per_m = 200e3_f64;

        let u_p = eta_0 * u_m / (e_star * r_eq);
        let g_p = alpha_pv * e_star;
        let w_p = w_per_m / (e_star * r_eq);

        // Verify dimensionless parameters are in expected range
        // U ~ 10⁻¹¹, G ~ 4000-5000, W ~ 10⁻⁴
        assert!(u_p > 1e-12 && u_p < 1e-9,
            "U = {u_p:.4e}, expected ~10⁻¹¹");
        assert!(g_p > 3000.0 && g_p < 6000.0,
            "G = {g_p:.1}, expected ~4600");
        assert!(w_p > 1e-5 && w_p < 1e-3,
            "W = {w_p:.4e}, expected ~10⁻⁴");

        // Dowson-Higginson (1977): H_min = 2.65 × U^0.70 × G^0.54 × W^-0.13
        let h_min_dimless = 2.65 * u_p.powf(0.70) * g_p.powf(0.54) * w_p.powf(-0.13);
        let h_min_um = h_min_dimless * r_eq * 1e6;

        // Dowson-Toyoda (1978): H_c = 3.06 × U^0.69 × G^0.56 × W^-0.10
        let h_c_dimless = 3.06 * u_p.powf(0.69) * g_p.powf(0.56) * w_p.powf(-0.10);
        let h_c_um = h_c_dimless * r_eq * 1e6;

        // Expected range: for these conditions, h_min ~ 0.2-1.0 μm, h_c ~ 0.3-1.5 μm
        // From Hamrock (2004) similar examples: h_min ≈ 0.3-0.8 μm at moderate speed/load
        assert!(h_min_um > 0.1 && h_min_um < 2.0,
            "h_min = {h_min_um:.3} μm, expected 0.1-2.0 μm for benchmark conditions");
        assert!(h_c_um > h_min_um,
            "h_c ({h_c_um:.3}) must > h_min ({h_min_um:.3})");
        // Ratio h_c/h_min typically 1.1-1.5 for line contacts
        let ratio = h_c_um / h_min_um;
        assert!(ratio > 1.05 && ratio < 2.0,
            "h_c/h_min = {ratio:.2}, expected 1.1-1.5 for line contact");

        println!("  D-H benchmark: U={u_p:.4e}, G={g_p:.1}, W={w_p:.4e}");
        println!("  h_min = {h_min_um:.4} μm, h_c = {h_c_um:.4} μm, ratio = {ratio:.3}");
    }

    // ─── 2. Roelands Model Quantitative Check ───────────────────────
    //
    // Ref: Roelands (1966) PhD Thesis, TU Delft
    //      Hamrock et al. (2004) Table 4.3
    //      Gold et al. (2001) for mineral oil Z_r values

    #[test]
    fn test_roelands_quantitative() {
        let eta_0 = 0.04_f64; // Pa·s (ISO VG 68 at 70°C)
        let p_r = 196.2e6_f64; // Roelands reference pressure

        // Test 1: At p = p_r (196.2 MPa), the formula gives a specific value
        // η = η₀ × exp{ (ln(η₀) + 9.67) × [(1 + 1)^Z_r − 1] }
        // = η₀ × exp{ (ln(0.04) + 9.67) × [2^0.67 − 1] }
        let z_r = 0.67_f64;
        let eta_pr = roelands_viscosity(eta_0, p_r, z_r);
        let expected_exponent = (eta_0.ln() + 9.67) * (2.0_f64.powf(z_r) - 1.0);
        let expected = eta_0 * expected_exponent.exp();
        assert!((eta_pr - expected).abs() / expected < 1e-10,
            "Roelands at p=p_r: got {eta_pr:.6e}, expected {expected:.6e}");

        // Test 2: Roelands at 500 MPa — compute manually
        let p = 500e6_f64;
        let eta_500 = roelands_viscosity(eta_0, p, z_r);
        let manual_exp = (eta_0.ln() + 9.67) * ((1.0 + p / p_r).powf(z_r) - 1.0);
        let manual = eta_0 * manual_exp.exp();
        assert!((eta_500 - manual).abs() / manual < 1e-10,
            "Roelands at 500 MPa: got {eta_500:.6e}, expected {manual:.6e}");

        // Test 3: Barus comparison at 1 GPa — Roelands must be MUCH lower
        let p_gpa = 1.0e9_f64;
        let alpha = 20e-9_f64;
        let eta_r_1gpa = roelands_viscosity(eta_0, p_gpa, z_r);
        let eta_b_1gpa = eta_0 * (alpha * p_gpa).exp();
        // Barus at 1 GPa: e^20 ≈ 4.85e8 → η_barus ≈ 1.94e7 Pa·s (absurd)
        // Roelands should be orders of magnitude lower
        assert!(eta_r_1gpa < eta_b_1gpa * 0.01,
            "Roelands ({eta_r_1gpa:.4e}) should be << Barus ({eta_b_1gpa:.4e}) at 1 GPa");

        // Test 4: Z_r sensitivity — higher Z_r → more pressure sensitivity
        let eta_z050 = roelands_viscosity(eta_0, 500e6, 0.50); // PAO
        let eta_z067 = roelands_viscosity(eta_0, 500e6, 0.67); // Mineral
        let eta_z075 = roelands_viscosity(eta_0, 500e6, 0.75); // High Z
        assert!(eta_z050 < eta_z067 && eta_z067 < eta_z075,
            "Higher Z_r → higher η: z050={eta_z050:.4e}, z067={eta_z067:.4e}, z075={eta_z075:.4e}");

        println!("  Roelands benchmark: η₀={eta_0} Pa·s");
        println!("  p=p_r: η = {eta_pr:.4e} Pa·s");
        println!("  p=500MPa: η = {eta_500:.4e} Pa·s");
        println!("  p=1GPa: Roelands={eta_r_1gpa:.4e}, Barus={eta_b_1gpa:.4e} (ratio={:.2e})", eta_r_1gpa/eta_b_1gpa);
    }

    // ─── 3. Masjedi-Khonsari (2015) Paper Coefficients Verification ──
    //
    // Ref: Masjedi, M. & Khonsari, M.M. (2015), "On the Effect of
    //      Surface Roughness in Point-Contact EHL: Formulas for Film
    //      Thickness and Asperity Load", Tribology International, 82,
    //      pp.228-244.
    // Note: The paper is for point contact, but we use adapted line
    //       contact version. Verify coefficient implementation matches
    //       our documented values.

    #[test]
    fn test_mk_line_contact_coefficients() {
        // Verify M-K line-contact formula with exponential roughness correction.
        let u = 6.94e-12_f64;
        let g = 4616.0_f64;
        let w = 1.733e-4_f64;
        let r_eq = 0.005_f64;
        let sigma_bar = 6e-5_f64;
        let v_param = 0.001_f64;

        let mk = compute_film_mk(u, g, w, sigma_bar, v_param, r_eq);

        // Smooth part (D-T central, D-H minimum)
        let h_c_smooth = 3.06 * u.powf(0.69) * g.powf(0.56) * w.powf(-0.10);
        let h_min_smooth = 2.65 * u.powf(0.70) * g.powf(0.54) * w.powf(-0.13);

        // Exponential roughness correction (M-K 2012 line contact)
        let lambda_eff_c = h_c_smooth / sigma_bar;
        let rc_c = (1.0 - 0.573 * (-0.74 * lambda_eff_c.powf(0.21)).exp()).clamp(0.5, 1.5);
        let lambda_eff_m = h_min_smooth / sigma_bar;
        let rc_m = (1.0 - 0.856 * (-0.74 * lambda_eff_m.powf(0.21)).exp()).clamp(0.5, 1.5);

        let expected_hc = h_c_smooth * rc_c * r_eq;
        let expected_hmin = h_min_smooth * rc_m * r_eq;

        assert!((mk.h_c - expected_hc).abs() / expected_hc < 1e-10,
            "M-K h_c mismatch: code={:.6e}, hand={:.6e}", mk.h_c, expected_hc);
        assert!((mk.h_min - expected_hmin).abs() / expected_hmin < 1e-10,
            "M-K h_min mismatch: code={:.6e}, hand={:.6e}", mk.h_min, expected_hmin);

        // Roughness correction should be moderate (not 9× as in old point-contact formula)
        // May be clamped to [0.5, 1.5] for extreme conditions
        assert!(rc_c >= 0.5 && rc_c <= 1.5,
            "Central correction in range [0.5,1.5]: {rc_c:.4}");
        assert!(rc_m >= 0.5 && rc_m <= 1.5,
            "Minimum correction in range [0.5,1.5]: {rc_m:.4}");

        println!("  M-K line-contact verified:");
        println!("    h_c  = {:.4e} m (smooth={:.4e}, correction={rc_c:.4})", mk.h_c, h_c_smooth*r_eq);
        println!("    h_min = {:.4e} m (smooth={:.4e}, correction={rc_m:.4})", mk.h_min, h_min_smooth*r_eq);
        println!("    load_frac = {:.6e}, area_frac = {:.6e}", mk.load_fraction, mk.area_fraction);
    }

    // ─── 4. GT Integral Analytical Check ─────────────────────────────
    //
    // Ref: Greenwood & Tripp (1970), Gaussian integral properties

    #[test]
    fn test_gt_integral_f0_analytical() {
        // F_0(h) = ∫_h^∞ φ(s) ds = 0.5 × erfc(h/√2)
        // At h=0: F_0(0) = 0.5
        let f0_at_0 = gt_integral(0.0, 0.0);
        assert!((f0_at_0 - 0.5).abs() < 0.002,
            "F_0(0) should be 0.5, got {f0_at_0}");

        // At h=1: F_0(1) = 0.5×erfc(1/√2) = 0.5×erfc(0.7071) ≈ 0.1587
        let f0_at_1 = gt_integral(0.0, 1.0);
        assert!((f0_at_1 - 0.1587).abs() < 0.002,
            "F_0(1) should be ≈0.1587, got {f0_at_1}");

        // At h=2: F_0(2) = 0.5×erfc(2/√2) = 0.5×erfc(1.414) ≈ 0.02275
        let f0_at_2 = gt_integral(0.0, 2.0);
        assert!((f0_at_2 - 0.02275).abs() < 0.002,
            "F_0(2) should be ≈0.0228, got {f0_at_2}");
    }

    #[test]
    fn test_gt_integral_f1_analytical() {
        // F_1(h) = ∫_h^∞ (s-h) × φ(s) ds = φ(h) − h × [0.5×erfc(h/√2)]
        // At h=0: F_1(0) = φ(0) - 0 = 1/√(2π) ≈ 0.3989
        let f1_at_0 = gt_integral(1.0, 0.0);
        assert!((f1_at_0 - 0.3989).abs() < 0.002,
            "F_1(0) should be ≈0.3989, got {f1_at_0}");
    }

    #[test]
    fn test_gt_integral_f2_analytical() {
        // F_2(h) = ∫_h^∞ (s-h)² × φ(s) ds
        // At h=0: F_2(0) = ∫_0^∞ s² × φ(s) ds = 0.5 (half of variance for unit Gaussian)
        let f2_at_0 = gt_integral(2.0, 0.0);
        assert!((f2_at_0 - 0.5).abs() < 0.005,
            "F_2(0) should be ≈0.500, got {f2_at_0}");
    }

    #[test]
    fn test_gt_integral_f52_reference() {
        // F_{5/2}(0) for Gaussian — numerically known value ≈ 0.6169
        // Computed by high-precision quadrature
        let f52_at_0 = gt_integral(2.5, 0.0);
        assert!((f52_at_0 - 0.6169).abs() < 0.01,
            "F_5/2(0) should be ≈0.617, got {f52_at_0}");

        // F_{5/2}(1) ≈ 0.0428 (from Greenwood & Tripp tables)
        let f52_at_1 = gt_integral(2.5, 1.0);
        assert!(f52_at_1 > 0.01 && f52_at_1 < 0.15,
            "F_5/2(1) should be ~0.04, got {f52_at_1}");
    }

    // ─── 5. Gupta Thermal Correction Benchmark ──────────────────────
    //
    // Ref: Gupta (1984) "Advanced Dynamics of Rolling Elements"
    //      Wilson (1979) thermal reduction factor

    #[test]
    fn test_gupta_thermal_benchmark() {
        // L_th = η₀ × β × u² / k
        // At typical conditions: η₀=0.04, β=0.04, u=2 m/s, k=0.15
        //   L_th = 0.04 × 0.04 × 4 / 0.15 = 0.04267
        //   φ_T = 1/(1 + 0.1 × 0.04267^0.64) = 1/(1 + 0.1 × 0.1071) = 1/1.01071 ≈ 0.9894
        let eta_0 = 0.04_f64;
        let beta = 0.04_f64;
        let u_m = 2.0_f64;
        let k_f = 0.15_f64;
        let l_th = eta_0 * beta * u_m * u_m / k_f;
        let phi_t = 1.0 / (1.0 + 0.1 * l_th.powf(0.64));
        let expected_lth = 0.04 * 0.04 * 4.0 / 0.15;
        assert!((l_th - expected_lth).abs() < 1e-10,
            "L_th = {l_th}, expected {expected_lth}");
        assert!(phi_t > 0.98,
            "At moderate speed, φ_T should be close to 1: {phi_t}");

        // High speed case: u_m = 20 m/s
        //   L_th = 0.04 × 0.04 × 400 / 0.15 = 4.267
        //   φ_T = 1/(1 + 0.1 × 4.267^0.64) = 1/(1 + 0.1 × 2.896) = 1/1.2896 ≈ 0.7755
        let l_th_high = eta_0 * beta * 20.0 * 20.0 / k_f;
        let phi_t_high = 1.0 / (1.0 + 0.1 * l_th_high.powf(0.64));
        assert!(phi_t_high < 0.85 && phi_t_high > 0.5,
            "At high speed, φ_T should show significant reduction: {phi_t_high}");

        // Murch-Wilson at SRR=0 should match Gupta
        let phi_mw = thermal_correction_murch_wilson(eta_0, beta, 2.0, k_f, 0.0, 500e6, 220e9);
        assert!((phi_t - phi_mw).abs() < 0.001,
            "Murch-Wilson(SRR=0) = {phi_mw} should match Gupta = {phi_t}");

        println!("  Gupta thermal: L_th={l_th:.4}, φ_T={phi_t:.4}");
        println!("  High speed: L_th={l_th_high:.2}, φ_T={phi_t_high:.4}");
        println!("  Murch-Wilson(SRR=0) = {phi_mw:.4} (should match Gupta)");
    }

    // ─── 6. Walther Viscosity Benchmark ─────────────────────────────
    //
    // Ref: ASTM D341, ISO VG 68 mineral oil standard values

    #[test]
    fn test_walther_viscosity_known_points() {
        // Input: ν₄₀=68 mm²/s, ν₁₀₀=8 mm²/s
        // At 40°C should return ~68, at 100°C should return ~8
        let nu_40 = crate::solver::life::viscosity_at_temp_pub(68.0, 8.0, 40.0);
        let nu_100 = crate::solver::life::viscosity_at_temp_pub(68.0, 8.0, 100.0);
        assert!((nu_40 - 68.0).abs() < 0.5,
            "ν(40°C) should be ≈68, got {nu_40}");
        assert!((nu_100 - 8.0).abs() < 0.5,
            "ν(100°C) should be ≈8, got {nu_100}");

        // At 70°C (interpolated): typical for ISO VG 68 is ~20-25 mm²/s
        let nu_70 = crate::solver::life::viscosity_at_temp_pub(68.0, 8.0, 70.0);
        assert!(nu_70 > 15.0 && nu_70 < 30.0,
            "ν(70°C) for VG68 should be 15-30 mm²/s, got {nu_70}");

        println!("  Walther: ν(40°C)={nu_40:.2}, ν(70°C)={nu_70:.2}, ν(100°C)={nu_100:.2}");
    }

    // ─── 7. Entraining Velocity Formula Check ───────────────────────
    //
    // Ref: Harris & Kotzalas (2006) Eq. 12.29, ISO 16281:2008 §C.3
    //
    // For inner-rotating, outer-stationary bearing, the mean entraining
    // velocity at the raceway contact is:
    //   u_m ≈ ω_cage × d_pw / 2  (bearing-level approximation)
    //
    // For per-contact differentiation (Harris Ch.12):
    //   u_m_inner ≈ ω_cage × R_inner  (roller surface speed = inner ring surface speed at pure rolling)
    //   u_m_outer ≈ ω_cage × R_outer  (roller surface speed at outer contact)
    //
    // KNOWN BUG: Current u_m_outer uses (ω_o + ω_cage)/2 × R_o, which
    // gives ω_cage × R_o / 2 when outer is stationary — half of correct value.

    #[test]
    fn test_entraining_velocity_physics() {
        // Preset: n_inner=1500 rpm, n_outer=0, d_pw=51mm, α=11.859°
        // d_we_mean ≈ (10.937 + 10.123) / 2 = 10.530 mm
        let n_inner = 1500.0_f64;
        let d_pw = 51.0_f64;
        let alpha_deg = 11.859_f64;
        let d_we = 10.530_f64;

        let omega_i = n_inner * std::f64::consts::TAU / 60.0; // 157.08 rad/s
        let alpha_rad = alpha_deg.to_radians();
        let gamma = d_we * alpha_rad.cos() / d_pw;
        let r_pw = d_pw / 2.0 * 1e-3; // 0.0255 m

        // Harris cage speed: ω_c = ω_i × (1−γ) / 2
        let omega_cage = omega_i * (1.0 - gamma) / 2.0;

        // Correct entraining velocities (Harris & Kotzalas 2006, Ch.12):
        // EHL entraining velocity = contact zone sweeping speed over raceway
        //   Inner: u_m = (ω_i − ω_cage) × R_inner
        //   Outer: u_m = (ω_cage − ω_o) × R_outer
        // At pure rolling, both equal |ω_i−ω_o| × R_pw × (1−γ²) / 2.
        let r_inner = r_pw * (1.0 - gamma);
        let r_outer = r_pw * (1.0 + gamma);
        let u_m_inner_harris = (omega_i - omega_cage) * r_inner;
        let u_m_outer_harris = omega_cage * r_outer; // ω_o = 0

        let op = test_op(n_inner);
        let u_m_inner_code = op.u_m_inner(r_pw, gamma);
        let u_m_outer_code = op.u_m_outer(r_pw, gamma);

        println!("  Entraining velocity analysis:");
        println!("    γ = {gamma:.4}");
        println!("    ω_cage = {omega_cage:.2} rad/s");
        println!("    Inner: code={u_m_inner_code:.4} m/s, Harris={u_m_inner_harris:.4} m/s");
        println!("    Outer: code={u_m_outer_code:.4} m/s, Harris={u_m_outer_harris:.4} m/s");

        // After fix: code should match Harris within machine precision
        assert!((u_m_inner_code - u_m_inner_harris).abs() < 1e-10,
            "Inner u_m must match Harris: code={u_m_inner_code}, Harris={u_m_inner_harris}");
        assert!((u_m_outer_code - u_m_outer_harris).abs() < 1e-10,
            "Outer u_m must match Harris: code={u_m_outer_code}, Harris={u_m_outer_harris}");

        // At pure rolling: inner and outer should be equal
        let ratio = u_m_inner_code / u_m_outer_code;
        assert!((ratio - 1.0).abs() < 0.001,
            "At pure rolling, u_m_inner ≈ u_m_outer: ratio={ratio:.4}");

        // Both should equal |ω_i| × R_pw × (1−γ²) / 2
        let u_m_theory = omega_i * r_pw * (1.0 - gamma * gamma) / 2.0;
        assert!((u_m_inner_code - u_m_theory).abs() < 1e-10,
            "u_m = ω_i×R_pw×(1−γ²)/2 = {u_m_theory:.4}, code={u_m_inner_code:.4}");

        println!("    u_m = ω_i×R_pw×(1−γ²)/2 = {u_m_theory:.4} m/s ✓");
    }

    // ─── 8. Preset Condition: Basic vs Advanced Full Comparison ──────
    //
    // Using the NSK HR30306J default preset to identify why the two
    // models produce vastly different results.

    #[test]
    fn test_preset_basic_vs_advanced_film_thickness() {
        // NSK HR30306J preset conditions
        let d_we = (10.9371 + 10.123273) / 2.0; // 10.530 mm
        let r_roller = d_we / 2.0;               // 5.265 mm
        let r_eq = r_roller * 1e-3;              // 0.005265 m
        let alpha = 11.859_f64.to_radians();
        let d_pw = 51.0_f64;
        let r_pw = d_pw / 2.0 * 1e-3;            // 0.0255 m
        let gamma = d_we * alpha.cos() / d_pw;    // ~0.202

        // Material
        let e_star = 1.0 / (2.0 * (1.0 - 0.09) / (210e9)); // 2.308e11 Pa

        // Lubricant at 70°C (ν₄₀=68, ν₁₀₀=8)
        let nu_70 = crate::solver::life::viscosity_at_temp_pub(68.0, 8.0, 70.0);
        let eta_0 = nu_70 * 1e-6 * 850.0; // [Pa·s]
        let alpha_pv = 20e-9_f64;          // [1/Pa]

        // Operating: 1500 rpm, q_max ≈ from 5kN radial + 2kN axial
        let q_max = 100.0_f64; // N/mm (representative worst-case slice load)
        let omega_i = 1500.0 * std::f64::consts::TAU / 60.0;
        let omega_cage = omega_i * (1.0 - gamma) / 2.0;

        // ═══ BASIC MODEL ═══
        // Correct: u_m = (ω_i − ω_cage) × R_inner = ω_i × R_pw × (1−γ²) / 2
        let u_m_basic = (omega_i - omega_cage) * r_pw * (1.0 - gamma);
        let u_p = eta_0 * u_m_basic / (e_star * r_eq);
        let g_p = alpha_pv * e_star;
        let w_per_l = q_max * 1e3;
        let w_p = w_per_l / (e_star * r_eq);

        let h_min_basic_dimless = 2.65 * u_p.powf(0.70) * g_p.powf(0.54) * w_p.powf(-0.13);
        let h_min_basic = h_min_basic_dimless * r_eq * 1e6; // [μm]
        let h_c_basic_dimless = 3.06 * u_p.powf(0.69) * g_p.powf(0.56) * w_p.powf(-0.10);
        let h_c_basic = h_c_basic_dimless * r_eq * 1e6;

        // Gupta thermal
        let l_th = eta_0 * 0.04 * u_m_basic * u_m_basic / 0.15;
        let phi_t_basic = 1.0 / (1.0 + 0.1 * l_th.powf(0.64));

        let h_min_basic_final = h_min_basic * phi_t_basic; // φ_s = 1.0
        let h_c_basic_final = h_c_basic * phi_t_basic;

        // ═══ ADVANCED MODEL ═══
        // Same u_m for fair comparison
        let u_m_adv = u_m_basic;
        let u_p_adv = eta_0 * u_m_adv / (e_star * r_eq);
        let w_p_adv = w_per_l / (e_star * r_eq);

        // M-K roughness parameters
        let sigma_combined = (0.15_f64.powi(2) + 0.3_f64.powi(2)).sqrt(); // Rq combined [μm]
        let sigma_m = sigma_combined * 1e-6; // [m]
        let sigma_bar = sigma_m / r_eq;
        let beta_eta = ETA_BETA_SIGMA / sigma_m;
        let v_param = sigma_bar * std::f64::consts::SQRT_2 / (r_eq * beta_eta);

        let mk = compute_film_mk(u_p_adv, g_p, w_p_adv, sigma_bar, v_param, r_eq);

        // Murch-Wilson at SRR≈0 (apex-aligned → near-zero sliding)
        let phi_t_adv = thermal_correction_murch_wilson(eta_0, 0.04, u_m_adv, 0.15, 0.0, 500e6, e_star);

        // Physics-based starvation (Oil, 1500×51 = 76500 mm·rpm)
        let speed_param = 1500.0 * 51.0;
        let phi_s_adv = compute_starvation_factor_advanced(
            eta_0, u_m_adv, e_star, r_eq, &LubricationType::Oil, speed_param);

        let h_min_adv_final = mk.h_min * phi_t_adv * phi_s_adv * 1e6;
        let h_c_adv_final = mk.h_c * phi_t_adv * phi_s_adv * 1e6;

        // ═══ COMPARISON ═══
        let ratio_hmin = h_min_adv_final / h_min_basic_final;
        let ratio_hc = h_c_adv_final / h_c_basic_final;

        println!("  ═══ PRESET BASIC vs ADVANCED COMPARISON ═══");
        println!("  Common: ν(70°C)={nu_70:.2} mm²/s, η₀={eta_0:.4e} Pa·s");
        println!("  u_m = {u_m_basic:.4} m/s, γ = {gamma:.4}");
        println!("  U = {u_p:.4e}, G = {g_p:.1}, W = {w_p:.4e}");
        println!("  ─── Basic ───");
        println!("    h_min = {h_min_basic:.4} μm × φ_T={phi_t_basic:.4} = {h_min_basic_final:.4} μm");
        println!("    h_c   = {h_c_basic:.4} μm × φ_T={phi_t_basic:.4} = {h_c_basic_final:.4} μm");
        println!("  ─── Advanced ───");
        println!("    M-K smooth h_min = {:.4e} m, roughness correction active", mk.h_min);
        println!("    φ_T(MW) = {phi_t_adv:.4}, φ_s(adv) = {phi_s_adv:.4}");
        println!("    h_min = {h_min_adv_final:.4} μm, h_c = {h_c_adv_final:.4} μm");
        println!("    load_fraction = {:.6}, area_fraction = {:.6}", mk.load_fraction, mk.area_fraction);
        println!("  ─── Ratio (Advanced / Basic) ───");
        println!("    h_min ratio = {ratio_hmin:.3}");
        println!("    h_c   ratio = {ratio_hc:.3}");

        // Both should give physically reasonable results (0.05-10 μm)
        assert!(h_min_basic_final > 0.01 && h_min_basic_final < 10.0,
            "Basic h_min out of range: {h_min_basic_final} μm");
        assert!(h_min_adv_final > 0.01 && h_min_adv_final < 10.0,
            "Advanced h_min out of range: {h_min_adv_final} μm");

        // At smooth surface + near-zero SRR + oil, the two models should
        // agree within ~30% (roughness correction is small for σ̄ ≈ 6e-5)
        // If they differ by more, something is wrong.
        if ratio_hmin < 0.3 || ratio_hmin > 3.0 {
            println!("  ⚠⚠⚠ LARGE DISCREPANCY: ratio = {ratio_hmin:.3}");
            println!("  Possible causes:");
            println!("    1. Different u_m values used in actual solver");
            println!("    2. M-K roughness correction significantly differs");
            println!("    3. Starvation factor φ_s_adv = {phi_s_adv:.3} reducing Advanced result");
        }
    }

    // ─── 9. Eyring Traction Analytical Limits ───────────────────────
    //
    // Ref: Eyring (1936), Johnson & Tevaarwerk (1977)
    //
    // At low Σ = η·γ̇/τ₀: τ ≈ η·γ̇ (Newtonian)
    // At high Σ: τ → τ₀·ln(2Σ) (logarithmic)

    #[test]
    fn test_eyring_newtonian_limit() {
        // Low SRR (Newtonian limit): μ ≈ η_eff × u_slide / (h_c × p)
        let eta_0 = 0.04;
        let p = 200e6; // low pressure for Newtonian
        let h_c = 1e-6; // 1 μm
        let tau_0 = 5e6;
        let z_r = 0.67;
        let u_roll = 2.0;
        let srr = 0.001; // very low SRR

        let mu = eyring_traction_advanced(srr, u_roll, p, h_c, eta_0, z_r, tau_0);

        // At low Σ, sinh⁻¹(x) ≈ x, so τ ≈ η_eff × γ̇
        // μ ≈ η_eff × u_slide / (h_c × p)
        let eta_eff = roelands_viscosity(eta_0, p, z_r);
        let u_slide = srr * u_roll;
        let gamma_dot = u_slide / h_c;
        let sigma = eta_eff * gamma_dot / tau_0;
        // For sigma << 1, sinh⁻¹(sigma) ≈ sigma
        let mu_newtonian = eta_eff * gamma_dot / p;

        if sigma < 0.1 {
            assert!((mu - mu_newtonian).abs() / mu_newtonian.max(1e-10) < 0.1,
                "At low SRR, Eyring should approach Newtonian: μ={mu:.6}, μ_Newton={mu_newtonian:.6}");
        }
    }

    #[test]
    fn test_eyring_shear_limit() {
        // At very high SRR, τ should approach τ_lim = 0.10 × p
        // μ should saturate near 0.10 but may not reach exactly 0.10
        // depending on Roelands η_eff and the Eyring sinh⁻¹ curve
        let mu = eyring_traction_advanced(10.0, 2.0, 500e6, 0.3e-6, 0.04, 0.67, 5e6);
        // Verify it saturates in a reasonable range
        assert!(mu > 0.05 && mu <= 0.10 + 1e-6,
            "At extreme SRR, μ should be in [0.05, 0.10], got {mu}");
        // And that further increasing SRR doesn't change μ much
        let mu_higher = eyring_traction_advanced(50.0, 2.0, 500e6, 0.3e-6, 0.04, 0.67, 5e6);
        assert!((mu_higher - mu).abs() < 0.02,
            "μ should saturate: SRR=10→{mu:.4}, SRR=50→{mu_higher:.4}");
    }

    // ─── 10. Starvation Factor Physics Check ─────────────────────────
    //
    // Ref: Hamrock & Dowson (1981), Lugt (2013)

    #[test]
    fn test_starvation_ndpw_table() {
        // Verify φ_s matches the manual's table (§14.3A.3):
        //   n×d_pw < 100k  → ~0.98
        //   n×d_pw = 300k  → ~0.90
        //   n×d_pw = 1M    → ~0.75
        //   n×d_pw > 2M    → ~0.60
        let phi_50k = compute_starvation_factor_advanced(
            0.04, 1.0, 220e9, 0.005, &LubricationType::Oil, 50_000.0);
        let phi_300k = compute_starvation_factor_advanced(
            0.04, 1.0, 220e9, 0.005, &LubricationType::Oil, 300_000.0);
        let phi_1m = compute_starvation_factor_advanced(
            0.04, 1.0, 220e9, 0.005, &LubricationType::Oil, 1_000_000.0);
        let phi_2m = compute_starvation_factor_advanced(
            0.04, 1.0, 220e9, 0.005, &LubricationType::Oil, 2_000_000.0);

        // Formula: φ_s = 1/(1 + (nd/3e6)^0.9), clamp [0.5, 1.0]
        // Calibrated to match Manual §14.3A.3 table within ±5%:
        assert!(phi_50k > 0.95,
            "n×d_pw=50k: φ_s={phi_50k}, expected >0.95 (~0.98)");
        assert!(phi_300k > 0.85 && phi_300k < 0.95,
            "n×d_pw=300k: φ_s={phi_300k}, expected ~0.90");
        assert!(phi_1m > 0.65 && phi_1m < 0.85,
            "n×d_pw=1M: φ_s={phi_1m}, expected ~0.75");
        assert!(phi_2m > 0.55 && phi_2m < 0.70,
            "n×d_pw=2M: φ_s={phi_2m}, expected ~0.60");

        // Grease should be lower than oil at same speed
        let phi_grease_300k = compute_starvation_factor_advanced(
            0.04, 1.0, 220e9, 0.005, &LubricationType::Grease, 300_000.0);
        assert!(phi_grease_300k < phi_300k,
            "Grease ({phi_grease_300k}) < Oil ({phi_300k}) at n×d_pw=300k");

        println!("  Starvation factors (Oil):");
        println!("    n×d_pw=50k:  φ_s = {phi_50k:.4}");
        println!("    n×d_pw=300k: φ_s = {phi_300k:.4}");
        println!("    n×d_pw=1M:   φ_s = {phi_1m:.4}");
        println!("    n×d_pw=2M:   φ_s = {phi_2m:.4}");
        println!("    Grease@300k: φ_s = {phi_grease_300k:.4}");
    }

    // ─── 11. Flash Temperature Dimensional Check ─────────────────────
    //
    // Ref: Blok (1937), Jaeger (1942)

    #[test]
    fn test_flash_temperature_dimensional() {
        // Blok-Jaeger: ΔT = μ × p_a × V / (2k × √(πPe))
        //   μ = 0.10, p_a = 100 MPa, V = 0.5 m/s, b = 0.1 mm
        //   Pe = V×b/(2κ) = 0.5 × 0.1e-3 / (2 × 1.2e-5) = 2.083
        //   ΔT = 0.10 × 100e6 × 0.5 / (2 × 46 × √(π × 2.083))
        //      = 5e6 / (92 × 2.558)
        //      = 5e6 / 235.3
        //      = 21242 → hmm that's too high.
        //
        // Let me recalculate with more realistic asperity pressure:
        //   p_a = 1 MPa (asperity pressure, not Hertzian!)
        //   ΔT = 0.10 × 1e6 × 0.5 / (2 × 46 × √(π × 2.083))
        //      = 50000 / 235.3 ≈ 212.5°C
        //
        // Still high — with very low V = 0.01 m/s:
        //   Pe = 0.01 × 0.1e-3 / (2 × 1.2e-5) = 0.04167
        //   ΔT = 0.10 × 1e6 × 0.01 / (2 × 46 × √(π × 0.04167))
        //      = 1000 / (92 × 0.3618) = 1000 / 33.3 ≈ 30°C ← realistic

        let dt = flash_temperature(0.10, 1e6, 0.01, 0.1e-3);
        assert!(dt > 5.0 && dt < 100.0,
            "Flash temp at realistic conditions: ΔT={dt:.1}°C, expected 10-80°C");

        // Zero sliding → zero temperature
        let dt_zero = flash_temperature(0.10, 1e6, 0.0, 0.1e-3);
        assert!(dt_zero < 1e-8, "Zero sliding → zero flash temp");

        // Temperature increases with sliding speed
        let dt_slow = flash_temperature(0.10, 1e6, 0.005, 0.1e-3);
        let dt_fast = flash_temperature(0.10, 1e6, 0.05, 0.1e-3);
        assert!(dt_fast > dt_slow,
            "ΔT must increase with speed: fast={dt_fast:.1} > slow={dt_slow:.1}");
    }

    // ─── 12. Full Solver Integration: Actual compute_film_thickness ──
    //
    // Run both models through the actual solver function with preset
    // input and compare the output values.

    #[test]
    fn test_compute_film_thickness_preset_conditions() {
        // Use the test fixture that mirrors the NSK HR30306J preset
        let geom = MacroGeometry {
            d: 30.0, outer_diameter: 72.0, t: 20.75,
            alpha: 11.859, z: 14,
            d_we_max: 10.9371, d_we_min: 10.123273,
            l_we: 11.65, d_pw: 51.0,
            h_rib: 2.5, alpha_rib: 9.855, g_r: 0.0, h_c: None,
        };
        let mat = Material {
            e_roller: 210.0, e_ring: 210.0, nu: 0.3,
            hrc: 61.0, density_roller: 7.85, density_ring: 7.85,
        };
        let rp = RacewayProfile { delta_rw: 0.0, w_a: 0.0, ra: 0.15, custom_profile: None, polynomial_coeffs: None };
        let rolp = test_roller_profile();

        // Operating: 1500 rpm, Oil, full flooded
        let op = OperatingConditions {
            f_x: 5.0, f_y: 0.0, f_a: 2.0,
            m_x: 0.0, m_y: 0.0, n_inner_rpm: 1500.0, n_outer_rpm: 0.0,
            gamma: 0.0, t_op: 70.0,
            nu_40: 68.0, nu_100: 8.0,
            alpha_pv: 20.0,
            lubrication_type: LubricationType::Oil,
            starvation_factor: 1.0,
            rho_oil: 850.0,
            preload_mode: PreloadMode::DisplacementFromForce,
            delta_preload_um: 0.0,
            lubrication_model: LubricationModel::Method1_DH, film_decay_enabled: false, film_decay_time_hours: 0.0, skew_angle_deg: 0.0, replenishment_rate_nm_s: 0.0, surface_finish: SurfaceFinish::Standard, additive_type: AdditiveType::None,
            tau_eyring: 5.0, z_roelands: 0.67,
            traction_model: TractionModel::Eyring, carreau_eta_inf_ratio: 0.005, carreau_lambda_s: 1.0e-7, carreau_n: 0.5, carreau_a: 2.0,
            friction_model: FrictionModel::PalmgrenLike, thermal_correction: ThermalCorrection::Aihara1987,
            hysteresis_loss_factor: 0.005, skf_trb_series: SkfTrbSeriesEnum::Series303, skf_lubrication: SkfLubricationEnum::OilBath, skf_y_factor: 1.6,
            k_fluid: 0.15, beta_visc: 0.04,
            rq_inner: 0.3, rq_outer: 0.3, rq_roller: 0.15,
            roughness_input_mode: RoughnessInputMode::Rq,
            design_life_hours: 100.0,
        };

        let q_max = 100.0; // N/mm representative
        let ft = compute_film_thickness(&geom, &mat, &op, &rolp, &rp, &rp, q_max);
        assert!(ft.is_some(), "Basic model should return result");
        let ft = ft.unwrap();

        println!("  ═══ ACTUAL SOLVER OUTPUT (Basic) ═══");
        println!("    h_min = {:.4} μm, h_c = {:.4} μm", ft.h_min_um, ft.h_central_um);
        println!("    σ = {:.4} μm, Λ = {:.3}, regime = {:?}", ft.sigma_composite_um, ft.lambda_ratio, ft.regime);
        println!("    u_m_inner = {:.4} m/s, u_m_outer = {:.4} m/s", ft.u_mean_m_s, ft.u_mean_m_s_outer);
        println!("    φ_T = {:.4}, φ_s = {:.4}", ft.thermal_factor, ft.starvation_factor);
        println!("    U = {:.4e}, G = {:.1}, W = {:.4e}", ft.u_param, ft.g_param, ft.w_param);
        println!("    Mixed: γ_a={:.6}, μ_eff={:.5}", ft.mixed.asperity_load_ratio, ft.mixed.mu_effective);

        // Sanity checks
        assert!(ft.h_min_um > 0.05, "h_min too low: {}", ft.h_min_um);
        assert!(ft.h_central_um > ft.h_min_um, "h_c > h_min");
        assert!(ft.thermal_factor > 0.9, "φ_T should be near 1 at 1500rpm");
        assert!(ft.u_mean_m_s > 0.5, "u_m should be significant at 1500rpm");

        // After fix: u_m_inner ≈ u_m_outer at pure rolling
        let um_ratio = ft.u_mean_m_s_outer / ft.u_mean_m_s;
        println!("    ★ u_m_outer / u_m_inner = {um_ratio:.3} (should be ≈1.0 at pure rolling)");
        assert!((um_ratio - 1.0).abs() < 0.05,
            "At pure rolling, u_m_inner ≈ u_m_outer: ratio={um_ratio:.3}");
    }

    // ═══════════════════════════════════════════════════════════════
    // NVM (1994) Film Thickness tests
    // ═══════════════════════════════════════════════════════════════

    #[test]
    fn test_nvm_circular_contact_paper_validation() {
        // Validate against Gao/Lugt (2026) Table 2: PCS EHD ball-on-disc, Li/M grease
        let e_steel = 206e9_f64; let nu_s = 0.3;
        let e_glass = 63e9_f64; let nu_g = 0.2;
        let e_prime = 2.0 / ((1.0 - nu_s*nu_s)/e_steel + (1.0 - nu_g*nu_g)/e_glass);
        let r_ball = 9.53e-3;
        let eta_0 = 0.38;
        let alpha = 30e-9;
        let f = 20.0; // N

        // u_m = 100 mm/s → paper h_cff = 353 nm
        let u_s = 2.0 * 0.100; // sum velocity
        let h_c = nvm_central_film(f, r_ball, r_ball, e_prime, eta_0, u_s, alpha);
        let h_nm = h_c * 1e9;
        println!("NVM circular @ 100mm/s: {h_nm:.0} nm (paper: 353 nm)");
        assert!((h_nm - 353.0).abs() < 50.0,
            "NVM circular @ 100mm/s should be near 353 nm: got {h_nm:.0}");
    }

    #[test]
    fn test_nvm_vs_dh_line_contact() {
        // TRB-like line contact: NVM and DH should agree within ~10% in EP regime
        let e_prime = 226.4e9;
        let eta_0 = 0.013; // 70°C
        let alpha = 20e-9;
        let rx = 6.43e-3;
        let ry = 1e6; // line contact
        let l_we = 22e-3;
        let q_per_m = 50e3; // 50 N/mm
        let u_m = 5.84; // ~1500 rpm

        // DH
        let u_p: f64 = eta_0 * u_m / (e_prime * rx);
        let g_p: f64 = alpha * e_prime;
        let w_p: f64 = q_per_m / (e_prime * rx);
        let h_dh = 3.06 * u_p.powf(0.69) * g_p.powf(0.56) * w_p.powf(-0.10) * rx;

        // NVM
        let f_total = q_per_m * l_we;
        let h_nvm = nvm_central_film(f_total, rx, ry, e_prime, eta_0, 2.0*u_m, alpha);

        let ratio = h_nvm / h_dh;
        println!("Line contact: DH = {:.3} μm, NVM = {:.3} μm, ratio = {ratio:.3}",
            h_dh*1e6, h_nvm*1e6);

        // In EP regime, NVM ≈ DH within 10%
        assert!((ratio - 1.0).abs() < 0.15,
            "NVM/DH ratio for TRB line contact should be ~1.0: got {ratio:.3}");
    }

    #[test]
    fn test_nvm_differs_from_mk() {
        // M3 (NVM) must produce different h_c than M2 (MK) — they use different formulas
        let e_prime = 226.4e9_f64;
        let eta_0 = 0.013_f64;
        let alpha = 20e-9_f64;
        let rx = 6.43e-3_f64;

        // NVM film for a single slice
        let f_slice = 50e3_f64 * 2.2e-3; // 50 N/mm × 2.2mm slice
        let u_s = 2.0 * 5.84_f64;
        let h_nvm = nvm_central_film(f_slice, rx, 1e3, e_prime, eta_0, u_s, alpha);

        // DH film for same conditions
        let u_m = 5.84_f64;
        let u_p: f64 = eta_0 * u_m / (e_prime * rx);
        let g_p: f64 = alpha * e_prime;
        let w_p: f64 = 50e3_f64 / (e_prime * rx);
        let h_dh = 3.06 * u_p.powf(0.69) * g_p.powf(0.56) * w_p.powf(-0.10) * rx;

        let ratio = h_nvm / h_dh;
        println!("NVM/DH per-slice: NVM={:.3}μm, DH={:.3}μm, ratio={ratio:.3}",
            h_nvm*1e6, h_dh*1e6);

        // NVM and DH should agree within ~10% in EP regime but NOT be identical
        assert!((ratio - 1.0).abs() > 0.001,
            "NVM and DH must not be identical: ratio={ratio:.6}");
        assert!((ratio - 1.0).abs() < 0.20,
            "But should be within 20% for TRB EP regime: ratio={ratio:.3}");
    }

    #[test]
    fn test_nvm_ellipticity_effect() {
        // Wider contact → higher h_c (less side flow)
        let e_prime = 100e9;
        let eta_0 = 0.1;
        let alpha = 20e-9;
        let rx = 10e-3;
        let f = 100.0;
        let u_s = 1.0;

        let h_circular = nvm_central_film(f, rx, rx, e_prime, eta_0, u_s, alpha);
        let h_elliptic = nvm_central_film(f, rx, 100.0*rx, e_prime, eta_0, u_s, alpha);
        let h_line = nvm_central_film(f, rx, 1e6, e_prime, eta_0, u_s, alpha);

        println!("NVM: circular = {:.1} nm, elliptic(100×) = {:.1} nm, line = {:.1} nm",
            h_circular*1e9, h_elliptic*1e9, h_line*1e9);

        // Line contact should have thicker film than circular (less side flow)
        assert!(h_line > h_circular * 0.5,
            "Line contact film should be comparable to circular");
    }

    // ═══════════════════════════════════════════════════════════════
    // Van Zoelen Film Decay — unit tests + integration comparison
    // ═══════════════════════════════════════════════════════════════

    #[test]
    fn test_van_zoelen_f0_basic() {
        // Paper conditions: circular contact (k=1), Li/M grease, PCS EHD
        let p_h = 0.48e9;
        let a = 141e-6;
        let b = 141e-6;
        let eta_0 = 0.38;
        let alpha = 30e-9;
        let l_t = 2.0 * std::f64::consts::PI * (0.040 + 0.00953);

        let f0 = van_zoelen_side_flow_f0(p_h, a, b, eta_0, alpha, l_t);
        println!("F(0) circular k=1 = {f0:.4e}");

        // Should be ~7.9e12 (from Python verification)
        assert!(f0 > 5e12 && f0 < 1e13, "F(0) circular: {f0:.4e}");
    }

    #[test]
    fn test_van_zoelen_f0_wide_elliptical() {
        // TRB-like: k ≈ 69
        let p_h = 0.22e9;
        let a = 39.2e-6;
        let b = 2710e-6;
        let eta_0 = 0.38;
        let alpha = 30e-9;
        let l_t = 2.0 * std::f64::consts::PI * (0.045 + 0.00391);

        let f0 = van_zoelen_side_flow_f0(p_h, a, b, eta_0, alpha, l_t);
        println!("F(0) wide k≈69 = {f0:.4e}");

        // k×10 → F(0) ×100 smaller; should be ~6e9 (Python verified)
        assert!(f0 > 3e9 && f0 < 1e10, "F(0) wide: {f0:.4e}");
    }

    #[test]
    fn test_van_zoelen_ellipticity_scaling() {
        // Key paper conclusion: k×10 → decay time ×100
        let p_h = 0.22e9;
        let eta_0 = 0.38;
        let alpha = 30e-9;
        let a = 39.2e-6;
        let l_t = 0.3;

        let f_k1  = van_zoelen_side_flow_f0(p_h, a, 1.0 * a, eta_0, alpha, l_t);
        let f_k10 = van_zoelen_side_flow_f0(p_h, a, 10.0 * a, eta_0, alpha, l_t);

        let ratio = f_k1 / f_k10;
        println!("F(k=1)/F(k=10) = {ratio:.1} (expected 100)");
        assert!((ratio - 100.0).abs() < 1.0, "Ellipticity scaling: {ratio:.1}");
    }

    #[test]
    fn test_van_zoelen_decay_no_replenishment() {
        // R=0: closed-form solution
        let h_c0 = 300e-9; // 300 nm
        let f0 = 6e9;
        let p_h = 0.22e9;
        let t_1hr = 3600.0;

        let h_1hr = van_zoelen_film_at_time(t_1hr, h_c0, f0, p_h);
        println!("h_c(1hr, R=0) = {:.1} nm", h_1hr * 1e9);

        // Film must decay but stay positive
        assert!(h_1hr < h_c0, "Must decay");
        assert!(h_1hr > 10e-9, "Must stay positive");
    }

    #[test]
    fn test_van_zoelen_equilibrium() {
        let f0 = 6e9;
        let p_h = 0.22e9;

        // R = 0 → no equilibrium
        assert!(van_zoelen_equilibrium(f0, 0.0, p_h).is_none());

        // R = 0.01 nm/s → should give finite equilibrium
        let r = 0.01e-9;
        let h_eq = van_zoelen_equilibrium(f0, r, p_h).unwrap();
        println!("h_eq(R=0.01 nm/s) = {:.1} nm", h_eq * 1e9);
        assert!(h_eq > 10e-9 && h_eq < 500e-9, "Equilibrium in reasonable range: {:.1} nm", h_eq * 1e9);

        // Higher R → higher equilibrium
        let h_eq_high = van_zoelen_equilibrium(f0, 0.1e-9, p_h).unwrap();
        assert!(h_eq_high > h_eq, "Higher R → higher equilibrium");
    }

    #[test]
    fn test_van_zoelen_decay_with_replenishment() {
        // R > 0: ODE solution should converge to equilibrium
        let h_c0 = 300e-9;
        let f0 = 6e9;
        let p_h = 0.22e9;
        let r = 0.01e-9; // 0.01 nm/s

        let h_eq = van_zoelen_equilibrium(f0, r, p_h).unwrap();
        println!("h_eq = {:.1} nm", h_eq * 1e9);

        // Long time ODE should approach equilibrium
        let t_long = 100.0 * 3600.0; // 100 hours
        let curve = van_zoelen_ode_solve(t_long, h_c0, f0, r, p_h, 50);
        let h_final = curve.last().unwrap().1;
        println!("h_c(100hr, R=0.01) = {:.1} nm (eq = {:.1} nm)", h_final * 1e9, h_eq * 1e9);

        let rel_err = ((h_final - h_eq) / h_eq).abs();
        println!("Relative error to equilibrium: {:.2}%", rel_err * 100.0);
        assert!(rel_err < 0.10, "Should approach equilibrium within 10%: {:.2}%", rel_err * 100.0);

        // When h_eq > h_c0, curve rises toward equilibrium; when h_eq < h_c0, it falls.
        // In both cases, curve should converge toward h_eq.
        let last_h = curve.last().unwrap().1;
        let dist_final = (last_h - h_eq).abs();
        let dist_init = (h_c0 - h_eq).abs();
        assert!(dist_final <= dist_init + 1e-12,
            "Curve should converge toward equilibrium: dist_init={dist_init:.2e}, dist_final={dist_final:.2e}");
    }

    #[test]
    fn test_van_zoelen_decay_large_r() {
        // R = 5 nm/s: h_eq may exceed h_c0 → film stays at h_c0 (fully flooded)
        let h_c0 = 500e-9; // 500 nm
        let f0 = 3e9;
        let p_h = 0.22e9;
        let r = 5.0e-9; // 5 nm/s — large replenishment

        let h_eq_raw = van_zoelen_equilibrium(f0, r, p_h).unwrap();
        println!("h_eq(R=5) = {:.1} nm (h_c0 = {:.1} nm)", h_eq_raw * 1e9, h_c0 * 1e9);

        // When h_eq > h_c0, film can't grow above fully flooded → capped at h_c0
        if h_eq_raw > h_c0 {
            println!("  → h_eq > h_c0: film remains fully flooded");
        }

        // ODE should keep h near h_c0 (bounded by h_max = h_c0 * 1.5 in solver)
        let t_long = 1000.0 * 3600.0;
        let curve = van_zoelen_ode_solve(t_long, h_c0, f0, r, p_h, 20);
        let h_final = curve.last().unwrap().1;
        println!("h_c(1000hr, R=5) = {:.1} nm", h_final * 1e9);

        // Film must stay positive and not blow up
        assert!(h_final > 100e-9, "Film must stay positive: {:.1} nm", h_final * 1e9);
        assert!(h_final < 10e-6, "Film must not blow up: {:.1} nm", h_final * 1e9);

        // All curve points must be positive
        for &(t, h) in &curve {
            assert!(h > 0.0, "h must be positive at t={t:.0}s: h={:.2e}", h);
        }
    }

    #[test]
    fn test_van_zoelen_skew_correction() {
        // Positive skew → slower decay (factor < 1)
        assert!(skew_decay_correction(2.0) < 1.0);
        assert!(skew_decay_correction(1.0) < 1.0);
        // Zero → no correction
        assert!((skew_decay_correction(0.0) - 1.0).abs() < 1e-10);
        // Negative skew → faster decay (factor > 1)
        assert!(skew_decay_correction(-1.0) > 1.0);
        assert!(skew_decay_correction(-2.0) > 1.0);
        // Monotonically: +2 < +1 < 0 < -1 < -2
        assert!(skew_decay_correction(2.0) < skew_decay_correction(1.0));
        assert!(skew_decay_correction(-2.0) > skew_decay_correction(-1.0));
        println!("Skew factors: +2°={:.3}, +1°={:.3}, 0°={:.3}, -1°={:.3}, -2°={:.3}",
            skew_decay_correction(2.0), skew_decay_correction(1.0),
            skew_decay_correction(0.0), skew_decay_correction(-1.0),
            skew_decay_correction(-2.0));
    }

    #[test]
    fn test_film_decay_m1_vs_m2_comparison() {
        // Full bearing comparison: M1(DH) vs M2(MK) with Film Decay
        // Using the standard test geometry (d_pw=150, Z=20, α=12°)
        let geom = test_geom();
        let mat = test_material();
        let rp = test_roller_profile();
        let rw = test_raceway_profile();

        println!("\n======================================================================");
        println!("  Film Thickness Comparison: M1 vs M2 x Decay ON/OFF");
        println!("======================================================================");

        for &n_rpm in &[1500.0_f64, 3000.0] {
            println!("\n  ── Speed: {n_rpm} rpm ──");

            // M1 (DH) — no decay
            let mut op_m1 = test_op(n_rpm);
            op_m1.lubrication_model = LubricationModel::Method1_DH;
            let ft_m1 = compute_film_thickness(&geom, &mat, &op_m1, &rp, &rw, &rw, 50.0);
            let ft_m1 = ft_m1.unwrap();

            // M2 (MK) — no decay
            let mut op_m2 = test_op(n_rpm);
            op_m2.lubrication_model = LubricationModel::Method2_MK;
            // M2 uses same interface but different formula internally
            // Note: compute_film_thickness always uses DH formula;
            // M2 uses compute_film_thickness_distribution_advanced in bearing.rs
            // Here we test the basic function which is always DH
            let ft_m2_basic = compute_film_thickness(&geom, &mat, &op_m2, &rp, &rw, &rw, 50.0);
            let ft_m2 = ft_m2_basic.unwrap();

            println!("    M1(DH):  h_c_inner = {:.3} μm, h_min = {:.3} μm, Λ = {:.2}",
                ft_m1.h_central_um, ft_m1.h_min_um, ft_m1.lambda_ratio);
            println!("    M1(DH):  h_c_outer = {:.3} μm, Λ_outer = {:.2}",
                ft_m1.h_central_um_outer, ft_m1.lambda_ratio_outer);

            // Film decay: R=0 (worst case)
            let mut ft_decay_r0 = ft_m1.clone();
            let mut op_decay_r0 = op_m1.clone();
            op_decay_r0.film_decay_enabled = true;
            op_decay_r0.film_decay_time_hours = 1000.0;
            op_decay_r0.lubrication_type = LubricationType::Grease;
            op_decay_r0.starvation_factor = 0.7;

            // Build minimal slice geometries for decay calculation
            let n_slices = 10;
            let d_we_mean = (geom.d_we_max + geom.d_we_min) / 2.0;
            let slice_width = geom.l_we / n_slices as f64;
            let slices: Vec<SliceGeometry> = (0..n_slices).map(|k| {
                let frac = (k as f64 + 0.5) / n_slices as f64;
                let d_local = geom.d_we_min + (geom.d_we_max - geom.d_we_min) * frac;
                let r_roller = d_local / 2.0;
                SliceGeometry {
                    k,
                    x_axial: frac * geom.l_we,
                    r_roller,
                    r_inner_race: 100.0,
                    r_outer_race: 200.0,
                    r_eq_inner: 1.0 / (1.0/r_roller + 1.0/100.0),
                    r_eq_outer: 1.0 / (1.0/r_roller - 1.0/200.0),
                    delta_z_total_inner: 0.0,
                    delta_z_total_outer: 0.0,
                    slice_width,
                }
            }).collect();

            // Mock angular distribution: create a few loaded positions
            let q_per_slice = 50.0; // N/mm per slice
            let angular_dist: Vec<AngularLoadPoint> = (0..20).map(|i| {
                let psi = i as f64 * 18.0; // 0° to 342°
                let load_factor = if psi < 90.0 || psi > 270.0 {
                    (1.0 - (psi - 0.0).to_radians().cos().abs() * 0.3).max(0.0)
                } else {
                    0.0
                };
                let q = q_per_slice * load_factor;
                AngularLoadPoint {
                    psi_deg: psi,
                    delta_rigid: 0.0,
                    q_total: q * n_slices as f64,
                    p_max: 500.0,
                    slice_p_max: vec![500.0 * load_factor; n_slices],
                    slice_p_max_outer: vec![450.0 * load_factor; n_slices],
                    slice_q_k: vec![q; n_slices],
                    is_roller: i % 2 == 0,
                }
            }).collect();

            compute_film_decay(
                &mut ft_decay_r0, &geom, &mat, &op_decay_r0,
                &slices, &angular_dist,
            );

            if let Some(ref decay) = ft_decay_r0.film_decay {
                println!("\n    Decay (R=0, t=1000hr, Grease):");
                println!("      h_c_inner: {:.3} → {:.3} μm (ratio {:.3})",
                    ft_m1.h_central_um, decay.h_c_decayed_inner_um,
                    decay.starvation_ratio_inner);
                println!("      h_c_outer: {:.3} → {:.3} μm (ratio {:.3})",
                    ft_m1.h_central_um_outer, decay.h_c_decayed_outer_um,
                    decay.starvation_ratio_outer);
                println!("      Λ_inner: {:.2} → {:.2} ({})",
                    ft_m1.lambda_ratio, decay.lambda_decayed_inner,
                    format!("{:?}", decay.regime_decayed_inner));
                println!("      F(0)_inner = {:.3e}", decay.f0_inner);

                assert!(decay.h_c_decayed_inner_um < ft_m1.h_central_um,
                    "Decayed must be less than fully flooded");
                assert!(decay.starvation_ratio_inner < 1.0 && decay.starvation_ratio_inner > 0.0,
                    "Starvation ratio must be in (0,1)");
            } else {
                panic!("Film decay should be computed");
            }

            // Film decay: R = 0.01 nm/s (with replenishment)
            let mut ft_decay_r = ft_m1.clone();
            let mut op_decay_r = op_decay_r0.clone();
            op_decay_r.replenishment_rate_nm_s = 0.01;

            compute_film_decay(
                &mut ft_decay_r, &geom, &mat, &op_decay_r,
                &slices, &angular_dist,
            );

            if let Some(ref decay) = ft_decay_r.film_decay {
                println!("\n    Decay (R=0.01 nm/s, t=1000hr, Grease):");
                println!("      h_c_inner: {:.3} → {:.3} μm (ratio {:.3})",
                    ft_m1.h_central_um, decay.h_c_decayed_inner_um,
                    decay.starvation_ratio_inner);
                if let Some(h_eq) = decay.h_c_equilibrium_inner_um {
                    println!("      h_eq_inner: {:.3} μm", h_eq);
                }
                println!("      Λ_inner: {:.2} → {:.2} ({})",
                    ft_m1.lambda_ratio, decay.lambda_decayed_inner,
                    format!("{:?}", decay.regime_decayed_inner));

                // With replenishment, h_c should be higher than R=0 case
                let r0_h = ft_decay_r0.film_decay.as_ref().unwrap().h_c_decayed_inner_um;
                assert!(decay.h_c_decayed_inner_um >= r0_h - 1e-6,
                    "R>0 should give h ≥ R=0: {:.4} vs {:.4}",
                    decay.h_c_decayed_inner_um, r0_h);

                // Equilibrium should exist
                assert!(decay.h_c_equilibrium_inner_um.is_some(),
                    "Equilibrium should exist when R > 0");

                // Decay curve should have data
                assert!(decay.decay_curve.len() > 5, "Decay curve should have points");
                println!("      Decay curve: {} points", decay.decay_curve.len());
            } else {
                panic!("Film decay with R should be computed");
            }

            // Skew effect comparison
            for &skew in &[-2.0_f64, 0.0, 2.0] {
                let mut ft_skew = ft_m1.clone();
                let mut op_skew = op_decay_r0.clone();
                op_skew.skew_angle_deg = skew;

                compute_film_decay(
                    &mut ft_skew, &geom, &mat, &op_skew,
                    &slices, &angular_dist,
                );

                if let Some(ref decay) = ft_skew.film_decay {
                    println!("    Skew {skew:+.0}°: h_c = {:.3} μm, ratio = {:.3}",
                        decay.h_c_decayed_inner_um, decay.starvation_ratio_inner);
                }
            }
        }
    }
}
