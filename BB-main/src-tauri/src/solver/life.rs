use crate::error::SolverError;
use crate::solver::types::*;

// ─── Public API ─────────────────────────────────────────────────────

/// Compute fatigue life using the method specified in solver params.
pub fn compute_fatigue_life(
    roller_results: &[RollerResult],
    geom: &MacroGeometry,
    material: &Material,
    operating: &OperatingConditions,
    solver: &SolverParams,
    kappa_override: Option<(f64, f64)>,
) -> Result<FatigueLifeResult, SolverError> {
    compute_iso16281(roller_results, geom, material, operating, solver, kappa_override)
}

// ─── Shared Utilities ───────────────────────────────────────────────

/// ISO 281 f_c factor for TRB (roller bearing, line contact).
/// Quadratic fit to ISO 281:2007 Table 6 (radial roller bearings, inner ring rotating).
/// Valid for γ ∈ [0.01, 0.35].
fn iso281_fc(d_we: f64, alpha_deg: f64, d_pw: f64) -> f64 {
    let gamma = d_we * alpha_deg.to_radians().cos() / d_pw;
    // Polynomial fit: f_c ≈ 93.76 − 60·γ + 15·γ²
    // Derived from ISO 281:2007 Table 6 data points:
    //   γ=0.05→90.8, γ=0.10→87.9, γ=0.15→85.1, γ=0.20→82.4, γ=0.25→79.7
    let fc = 93.76 - 60.0 * gamma + 15.0 * gamma * gamma;
    fc.clamp(60.0, 95.0)
}

/// Raw raceway capacity factors (Lundberg-Palmgren, line contact).
/// Returns (fi, fo) WITHOUT normalization — ISO 16281 uses these directly.
fn raw_raceway_factors(gamma: f64) -> (f64, f64) {
    let fi = (1.0 - gamma).powf(29.0 / 27.0) * (1.0 + gamma).powf(-1.0 / 4.0);
    let fo = (1.0 + gamma).powf(29.0 / 27.0) * (1.0 - gamma).powf(-1.0 / 4.0);
    (fi, fo)
}

/// Compute basic dynamic load rating C_r [kN] for single-row TRB.
/// ISO 281:2007, Eq. (7) for roller bearings.
fn compute_c_r(geom: &MacroGeometry) -> f64 {
    let alpha_rad = geom.alpha.to_radians();
    let d_we = (geom.d_we_max + geom.d_we_min) / 2.0; // mean roller diameter [mm]
    let fc = iso281_fc(d_we, geom.alpha, geom.d_pw);
    let z = geom.z as f64;
    let l_we = geom.l_we; // [mm]

    // C_r = b_m × f_c × (i × L_we × cos(α))^(7/9) × Z^(3/4) × D_we^(29/27)
    // b_m ≈ 1.1 (material/manufacturing factor for standard bearing steel)
    let b_m = 1.1;
    let c_r_n = b_m * fc
        * (l_we * alpha_rad.cos()).powf(7.0 / 9.0)
        * z.powf(3.0 / 4.0)
        * d_we.powf(29.0 / 27.0);

    c_r_n / 1000.0 // [N] → [kN]
}

/// Dynamic equivalent bearing load P [kN] for TRB.
/// ISO 281: P = X·F_r + Y·F_a
fn compute_p_equiv(f_r: f64, f_a: f64, alpha_deg: f64) -> f64 {
    let alpha_rad = alpha_deg.to_radians();
    let e = 1.5 * alpha_rad.tan(); // demarcation factor

    let (x, y) = if f_r.abs() < 1e-12 {
        (0.0, 1.0 / alpha_rad.tan()) // pure axial
    } else if f_a / f_r <= e {
        (1.0, 0.0) // radial dominant
    } else {
        (0.4, 0.4 / alpha_rad.tan()) // combined
    };

    (x * f_r + y * f_a).max(1e-6)
}

/// Public wrapper for lubrication module access.
pub fn viscosity_at_temp_pub(nu_40: f64, nu_100: f64, t_op: f64) -> f64 {
    viscosity_at_temp(nu_40, nu_100, t_op)
}

/// Viscosity at operating temperature using 2-point Walther interpolation.
/// ASTM D341 simplified: log(log(ν + 0.7)) is linear in log(T + 273.15).
fn viscosity_at_temp(nu_40: f64, nu_100: f64, t_op: f64) -> f64 {
    let log_log = |nu: f64| (nu + 0.7).ln().ln();
    let log_t = |t: f64| (t + 273.15).ln();

    let ll_40 = log_log(nu_40);
    let ll_100 = log_log(nu_100);
    let lt_40 = log_t(40.0);
    let lt_100 = log_t(100.0);

    // Linear interpolation in log-log-log space
    let slope = (ll_100 - ll_40) / (lt_100 - lt_40);
    let ll_t = ll_40 + slope * (log_t(t_op) - lt_40);

    // Inverse: ν = exp(exp(ll_t)) - 0.7
    (ll_t.exp().exp() - 0.7).max(0.5)
}

/// Required reference viscosity ν₁ [mm²/s] at operating temperature.
/// ISO 281:2007 Eq.(28) for n < 1000 rpm, Eq.(29) for n ≥ 1000 rpm.
fn reference_viscosity(d_pw: f64, n_rpm: f64) -> f64 {
    if n_rpm < 1e-3 {
        return 1e6; // static → infinite viscosity requirement
    }
    let dm = d_pw; // [mm]
    if n_rpm < 1000.0 {
        45000.0 / (n_rpm.powf(0.83) * dm.sqrt())  // Eq.(28)
    } else {
        4500.0 / (n_rpm.sqrt() * dm.sqrt())         // Eq.(29)
    }
}

/// Viscosity ratio κ = ν_actual / ν_required (Method 1: ISO 281 Eq.27).
pub fn compute_kappa(operating: &OperatingConditions, d_pw: f64) -> f64 {
    let nu_actual = viscosity_at_temp(operating.nu_40, operating.nu_100, operating.t_op);
    let nu_ref = reference_viscosity(d_pw, operating.n_rpm());
    (nu_actual / nu_ref).max(0.01)
}

/// Viscosity ratio κ from film thickness ratio Λ (Method 2: ISO/TR 1281-2 Eq.57).
/// κ ≈ Λ^1.3, capped at κ=4 per ISO 281 §9.3.3.4.
pub fn compute_kappa_from_lambda(lambda: f64) -> f64 {
    lambda.max(0.0).powf(1.3).clamp(0.01, 4.0)
}

/// ISO 281:2007 Annex A: contamination factor e_C.
///
/// e_C = min(C₁·κ^0.68·D_pw^0.55, 1) × (1 − C₂·D_pw^(−1/3))
///
/// Constants C₁, C₂ from ISO 281:2007 Table A.1, selected by
/// oil supply method and contamination level (ISO 4406 cleanliness).
pub fn compute_e_c(kappa: f64, d_pw: f64, level: ContaminationLevel, supply: OilSupplyMethod) -> f64 {
    let (c1, c2) = match supply {
        OilSupplyMethod::OilBath => level.c1_c2_oil_bath(),
        OilSupplyMethod::CirculatingWithFilter => level.c1_c2_circulating(),
        OilSupplyMethod::Grease => level.c1_c2_grease(),
    };
    let term1 = (c1 * kappa.max(0.01).powf(0.68) * d_pw.powf(0.55)).min(1.0);
    let term2 = 1.0 - c2 * d_pw.powf(-1.0 / 3.0);
    (term1 * term2).clamp(0.0, 1.0)
}

/// ISO 281:2007 life modification factor a_ISO.
///
/// Piecewise closed-form from ISO 281:2007 Tables 6-8, as reproduced in
/// ISO 281:2007 Tables 6-8 (closed-form fit).  The formula is:
///
///   a_ISO = 0.1 × [1 − (1.5859 − C₁/κ^C₂) × (e_c·C_u/P)^0.4]^(−9.185)
///
/// with region-dependent constants C₁, C₂.
fn compute_a_iso(kappa: f64, e_c: f64, c_u_over_p: f64) -> f64 {
    if kappa < 0.01 {
        return 0.1; // below minimum usable range
    }

    let x = (e_c * c_u_over_p.min(0.5)).max(0.0); // combined contamination-load term

    if x < 1e-12 {
        return 0.1; // no fatigue limit contribution → minimum
    }

    // Region-dependent constants (C₁, C₂)
    let (c1, c2) = if kappa < 0.4 {
        (1.3993, 0.054381)
    } else if kappa < 1.0 {
        (1.2348, 0.19087)
    } else if kappa <= 4.0 {
        (1.2348, 0.071739)
    } else {
        // κ > 4: use κ=4 constants (upper plateau)
        (1.2348, 0.071739)
    };

    let kappa_eff = kappa.min(4.0); // cap κ contribution at 4.0
    let bracket = 1.5859 - c1 / kappa_eff.powf(c2);
    let inner = 1.0 - bracket * x.powf(0.4);

    let a_iso = if inner <= 0.0 {
        50.0 // bracket exceeds 1 → extremely favorable conditions
    } else {
        0.1 * inner.powf(-9.185)
    };

    a_iso.clamp(0.1, 50.0)
}

/// Compute inner/outer raceway dynamic capacity factors (raw, unnormalized).
/// ISO 16281: per-ring factors based on Lundberg-Palmgren line contact theory.
/// Returns (fi, fo) where:
///   Q_ci = (C_r / b_m) × fi / (Z × sin α)   [per-roller inner ring capacity]
///   Q_ce = (C_r / b_m) × fo / (Z × sin α)   [per-roller outer ring capacity]
#[allow(dead_code)]
fn raceway_capacity_factors(d_we: f64, alpha_deg: f64, d_pw: f64) -> (f64, f64) {
    let gamma = d_we * alpha_deg.to_radians().cos() / d_pw;
    raw_raceway_factors(gamma)
}

// ─── Lamina-Level Core (ISO 16281:2025) ──────────────────────────────
//
// ISO 16281 lamina-level exponents (Lundberg-Palmgren theory):
//   - Inner ring equiv load exponent: 4   (Formula 25, rotating ring)
//   - Outer ring equiv load exponent: 4.5 (Formula 27, stationary ring)
//   - Lamina life exponent: 4.5           (Formula 29)
//   - Lamina-to-bearing combination: -8/9 (Formula 29)
//
// Edge stress correction f[j,k] is not applied (f=1):
//   - Crowned profiles: slice-level Hertz already captures load distribution
//   - Uncrowned profiles: would require non-Hertzian contact model (C.1)

/// Core lamina-level life computation (ISO 16281:2025).
///
/// Formula (33): per-lamina a_ISOk for modified reference life.
///   L_nmr = a_1 × ( Σ_k { a_ISOk^(-9/8) × [ (q_ci/q_eik)^(-9/2) + (q_ce/q_eek)^(-9/2) ] } )^(-8/9)
///
/// Per-lamina reference load P_sk (Formula 31):
///   P_sk = 0.323 × Z × cos(α) × n_s × { [q_eik^(9/2) + (1.038 × q_ci/q_ce × q_eek)^(9/2)] / [1 + (1.038 × q_ci/q_ce)^(9/2)] }^(2/9)
///
/// a_ISOk = f(e_C × C_u / P_sk, κ)
///
/// Returns (l_10_basic, l_nm_modified, l_10_inner, l_10_outer, weakest_lamina, a_iso_bearing, p_ref_damage, lamina_lives).
fn compute_lamina_level_life(
    roller_results: &[RollerResult],
    geom: &MacroGeometry,
    q_c_inner: f64,
    q_c_outer: f64,
    kappa_inner: f64,
    kappa_outer: f64,
    e_c: f64,
    c_u_kn: f64,
) -> Result<(f64, f64, f64, f64, usize, f64, f64, Vec<LaminaLife>), SolverError> {
    let z = geom.z as f64;
    let alpha_rad = geom.alpha.to_radians();
    let n_slices = roller_results
        .iter()
        .find(|r| !r.slice_results.is_empty())
        .map(|r| r.slice_results.len())
        .unwrap_or(0);

    if n_slices == 0 {
        return Err(SolverError::InvalidInput(
            "No slice data available for lamina-level life calculation".into(),
        ));
    }

    let ns = n_slices as f64;
    let slice_width = geom.l_we / ns; // [mm] — lamina width

    // ISO 16281 exponents (Lundberg-Palmgren line contact)
    let p_inner = 4.0;   // inner ring (rotating) equiv load exponent
    let p_outer = 4.5;   // outer ring (stationary) equiv load exponent
    let life_exp = 4.5;  // lamina life exponent: L_k = (q_c/q_e)^4.5

    // Ratio for Formula (31)
    let q_ratio = if q_c_outer > 1e-12 { q_c_inner / q_c_outer } else { 1.0 };
    let ratio_term = 1.038 * q_ratio;
    let ratio_denom = 1.0 + ratio_term.powf(4.5); // [1 + (1.038 × q_ci/q_ce)^(9/2)]

    let mut lamina_lives = Vec::with_capacity(n_slices);
    let mut basic_damage_sum = 0.0_f64;
    let mut modified_damage_sum = 0.0_f64;

    for k in 0..n_slices {
        // Inner ring (rotating): Weibull equivalent load, exponent 4 (Eq. 5.6)
        // q_k is line load [N/mm]; multiply by slice_width to get per-lamina force [N]
        let sum_inner: f64 = roller_results
            .iter()
            .map(|r| {
                if k < r.slice_results.len() && r.slice_results[k].in_contact {
                    (r.slice_results[k].q_k.max(0.0) * slice_width).powf(p_inner)
                } else {
                    0.0
                }
            })
            .sum();
        let q_ei_k = (sum_inner / z).powf(1.0 / p_inner);

        // Outer ring (stationary): Weibull equivalent load, exponent 4.5 (Eq. 5.7)
        let sum_outer: f64 = roller_results
            .iter()
            .map(|r| {
                if k < r.slice_results.len() && r.slice_results[k].in_contact {
                    (r.slice_results[k].q_k.max(0.0) * slice_width).powf(p_outer)
                } else {
                    0.0
                }
            })
            .sum();
        let q_eo_k = (sum_outer / z).powf(1.0 / p_outer);

        // Lamina life: L_k = (q_c / q_e)^4.5
        let l10_k_inner = if q_ei_k > 1e-6 {
            (q_c_inner / q_ei_k).powf(life_exp)
        } else {
            1e12
        };
        let l10_k_outer = if q_eo_k > 1e-6 {
            (q_c_outer / q_eo_k).powf(life_exp)
        } else {
            1e12
        };

        // Basic damage (no a_ISO): Σ_k [ (q_ci/q_eik)^(-9/2) + (q_ce/q_eek)^(-9/2) ]
        let d_inner = if l10_k_inner < 1e12 { 1.0 / l10_k_inner } else { 0.0 };
        let d_outer = if l10_k_outer < 1e12 { 1.0 / l10_k_outer } else { 0.0 };
        basic_damage_sum += d_inner + d_outer;

        // ISO 16281:2025 Formula (31): per-lamina reference load P_sk [kN]
        let p_sk = if q_ei_k > 1e-6 || q_eo_k > 1e-6 {
            let inner_term = q_ei_k.powf(4.5);
            let outer_adj = (ratio_term * q_eo_k).powf(4.5);
            let numerator = ((inner_term + outer_adj) / ratio_denom).powf(2.0 / 9.0);
            0.323 * z * alpha_rad.cos() * ns * numerator / 1000.0 // N → kN
        } else {
            0.0
        };

        // Per-lamina a_ISOk = f(e_C × C_u / P_sk, κ) — separate for inner/outer
        let a_iso_k_inner = if p_sk > 1e-6 {
            compute_a_iso(kappa_inner, e_c, c_u_kn / p_sk)
        } else {
            1.0 // unloaded lamina
        };
        let a_iso_k_outer = if p_sk > 1e-6 {
            compute_a_iso(kappa_outer, e_c, c_u_kn / p_sk)
        } else {
            1.0 // unloaded lamina
        };

        // Modified damage: Formula (33): Σ_k { a_ISOk^(-9/8) × [ ... ] }
        // Apply separate a_ISO per ring
        let we = 9.0 / 8.0;
        modified_damage_sum += a_iso_k_inner.powf(-we) * d_inner + a_iso_k_outer.powf(-we) * d_outer;

        lamina_lives.push(LaminaLife {
            k,
            q_equiv_inner: q_ei_k,
            q_equiv_outer: q_eo_k,
            l_10_inner: l10_k_inner.min(1e12),
            l_10_outer: l10_k_outer.min(1e12),
            p_sk,
            a_iso_k_inner,
            a_iso_k_outer,
        });
    }

    // Basic reference rating life: L_10r = (Σ_k D_k)^(-8/9)
    let comb_exp = 8.0 / 9.0;
    let l_10_basic = if basic_damage_sum > 0.0 {
        basic_damage_sum.powf(-comb_exp)
    } else {
        1e12
    };

    // Modified reference rating life: Formula (33)
    let l_nm_modified = if modified_damage_sum > 0.0 {
        modified_damage_sum.powf(-comb_exp)
    } else {
        1e12
    };

    // Effective bearing-level a_ISO = L_nmr / L_10r
    let a_iso_bearing = if l_10_basic > 1e-12 && l_10_basic < 1e12 {
        l_nm_modified / l_10_basic
    } else {
        1.0
    };

    // Damage-weighted P_ref: weighted average of P_sk by damage contribution
    let p_ref_damage = if basic_damage_sum > 0.0 {
        let weighted_sum: f64 = lamina_lives.iter().map(|l| {
            let d = if l.l_10_inner < 1e12 { 1.0 / l.l_10_inner } else { 0.0 }
                  + if l.l_10_outer < 1e12 { 1.0 / l.l_10_outer } else { 0.0 };
            l.p_sk * d
        }).sum();
        weighted_sum / basic_damage_sum
    } else {
        0.0
    };

    // Extract per-ring lives for reporting (Weibull combination with e=9/8)
    let weibull_e = 9.0 / 8.0;
    let mut inner_damage = 0.0_f64;
    let mut outer_damage = 0.0_f64;
    for lam in &lamina_lives {
        if lam.l_10_inner < 1e12 {
            inner_damage += lam.l_10_inner.powf(-weibull_e);
        }
        if lam.l_10_outer < 1e12 {
            outer_damage += lam.l_10_outer.powf(-weibull_e);
        }
    }
    let l_10_inner = if inner_damage > 0.0 { inner_damage.powf(-1.0 / weibull_e) } else { 1e12 };
    let l_10_outer = if outer_damage > 0.0 { outer_damage.powf(-1.0 / weibull_e) } else { 1e12 };

    // Find weakest lamina
    let weakest = lamina_lives
        .iter()
        .min_by(|a, b| {
            let life_a = a.l_10_inner.min(a.l_10_outer);
            let life_b = b.l_10_inner.min(b.l_10_outer);
            life_a.partial_cmp(&life_b).unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|l| l.k)
        .unwrap_or(0);

    Ok((l_10_basic, l_nm_modified, l_10_inner, l_10_outer, weakest, a_iso_bearing, p_ref_damage, lamina_lives))
}

// ─── ISO 16281:2025 Lamina-Level Life ────────────────────────────────

fn compute_iso16281(
    roller_results: &[RollerResult],
    geom: &MacroGeometry,
    _material: &Material,
    operating: &OperatingConditions,
    solver: &SolverParams,
    kappa_override: Option<(f64, f64)>,
) -> Result<FatigueLifeResult, SolverError> {
    let c_r = solver.c_r_kn.unwrap_or_else(|| compute_c_r(geom));
    let (kappa_inner, kappa_outer) = match kappa_override {
        Some((ki, ko)) => (ki, ko),
        None => {
            let k = compute_kappa(operating, geom.d_pw);
            (k, k) // ViscosityRatio: same for both
        }
    };
    let kappa = kappa_inner; // representative value (for e_c, backward compat)
    let nu_actual = viscosity_at_temp(operating.nu_40, operating.nu_100, operating.t_op);
    let nu_ref = reference_viscosity(geom.d_pw, operating.n_rpm());

    let z = geom.z as f64;
    let d_we = (geom.d_we_max + geom.d_we_min) / 2.0;
    let alpha_rad = geom.alpha.to_radians();
    let b_m = 1.1;
    let gamma_bearing = d_we * alpha_rad.cos() / geom.d_pw;
    let (f_ci, f_co) = raw_raceway_factors(gamma_bearing);
    let fc = iso281_fc(d_we, geom.alpha, geom.d_pw);

    // ISO 16281: per-roller capacity from C_r, split by raw raceway factors.
    // Divide by b_m to remove manufacturing factor before applying ring split.
    // Q_ci = (C_r / b_m) × fi / (Z × sin α)
    let q_c_base = c_r * 1000.0 / (b_m * z * alpha_rad.sin()); // [N] theoretical base
    let q_ci = q_c_base * f_ci;
    let q_co = q_c_base * f_co;

    // Per-lamina capacity: q_c = Q_c × (1/n_s)^(7/9)  (Eq. 5.3, 5.4)
    let n_slices = roller_results
        .iter()
        .find(|r| !r.slice_results.is_empty())
        .map(|r| r.slice_results.len())
        .unwrap_or(solver.n_slices);
    let ns = n_slices as f64;
    let q_c_lamina_inner = q_ci * (1.0 / ns).powf(7.0 / 9.0);
    let q_c_lamina_outer = q_co * (1.0 / ns).powf(7.0 / 9.0);

    // ISO 281 equivalent load and C_u
    let f_r = operating.f_r();
    let p = compute_p_equiv(f_r, operating.f_a, geom.alpha);
    let c_0r = solver.c_0r_kn.unwrap_or_else(|| crate::solver::static_rating::compute_c_0r(geom));
    let c_u = c_0r / 8.2; // ISO 281:2007 B.3.3

    // Contamination factor: auto (ISO 281 Annex A) or user override
    let e_c = if solver.e_c <= 0.0 { compute_e_c(kappa, geom.d_pw, solver.contamination_level, solver.oil_supply_method) } else { solver.e_c };

    // ISO 16281:2025 Formula (33): per-lamina a_ISOk
    let (l_10_basic, l_nm_modified, l_10_inner, l_10_outer, weakest, a_iso, p_ref_damage, lamina_lives) =
        compute_lamina_level_life(
            roller_results, geom,
            q_c_lamina_inner, q_c_lamina_outer,
            kappa_inner, kappa_outer, e_c, c_u,
        )?;

    // ISO 16281 back-calculated P_ref: C_r / L_10r(ISO 16281)^(3/10)
    // "만약 하중이 균일했다면 이 L₁₀ᵣ를 주었을 등가하중"
    let p_ref = if l_10_basic > 1e-12 && l_10_basic < 1e12 {
        c_r / l_10_basic.powf(3.0 / 10.0)
    } else {
        0.0
    };

    let e_demarcation = 1.5 * alpha_rad.tan();
    let (x_factor, y_factor) = if f_r.abs() < 1e-12 {
        (0.0, 1.0 / alpha_rad.tan())
    } else if operating.f_a / f_r <= e_demarcation {
        (1.0, 0.0)
    } else {
        (0.4, 0.4 / alpha_rad.tan())
    };

    let l_nm_hours = if operating.n_rpm() > 1e-3 {
        l_nm_modified * 1e6 / (60.0 * operating.n_rpm())
    } else {
        f64::INFINITY
    };

    let intermediates = LifeIntermediates {
        nu_actual,
        nu_ref,
        b_m,
        f_c: fc,
        gamma_bearing,
        c_u_kn: c_u,
        c_u_over_p: c_u / p,
        e_demarcation,
        x_factor,
        y_factor,
        f_a_over_f_r: if f_r > 1e-12 { operating.f_a / f_r } else { f64::INFINITY },
        f_ci,
        f_co,
        q_c_base: q_c_base * b_m,
        q_ci,
        q_co,
        q_ei: 0.0,
        q_eo: 0.0,
        weibull_e: 9.0 / 8.0,
        l_nm_mrev: l_nm_modified,
        q_c_lamina_inner,
        q_c_lamina_outer,
        e_c_used: e_c,
        kappa_method: solver.kappa_method,
        lambda_inner: kappa_override.map(|(ki, _)| ki.powf(1.0 / 1.3)),
        lambda_outer: kappa_override.map(|(_, ko)| ko.powf(1.0 / 1.3)),
    };

    Ok(FatigueLifeResult {
        method: LifeMethod::Iso16281,
        l_10_basic,
        l_nm_hours,
        l_10_inner,
        l_10_outer,
        weakest_lamina: weakest,
        a_iso,
        kappa: kappa_inner, // representative (backward compat)
        kappa_inner,
        kappa_outer,
        c_dyn: c_r,
        p_equiv: p,
        p_ref,
        p_ref_damage,
        intermediates,
        lamina_lives: Some(lamina_lives),
        film_thickness: None,
    })
}

// ─── ISO 15312: Thermal Speed Rating ────────────────────────────────

/// Compute ISO 15312:2018 thermal speed rating for a TRB.
///
/// Solves the energy balance equation (Formula 11) iteratively:
///   (π·nθr / 30e3) × [1e-7·f₀ᵣ·(vᵣ·nθr)^(2/3)·dm³ + f₁ᵣ·P₁ᵣ·dm] = qᵣ·Aᵣ
///
/// # Arguments
/// * `geom`      — macro geometry (d, D, T)
/// * `c_0r_n`    — basic static load rating C₀ᵣ [N]
/// * `n_rpm`     — operating rotational speed [rpm] (used for speed_ratio only)
/// * `f_0r`      — ISO 15312 Table A.1 coefficient (load-independent friction)
/// * `f_1r`      — ISO 15312 Table A.1 coefficient (load-dependent friction)
pub fn compute_thermal_speed_rating(
    geom: &MacroGeometry,
    c_0r_n: f64,
    n_rpm: f64,
    f_0r: f64,
    f_1r: f64,
) -> ThermalSpeedResult {
    let d = geom.d;
    let cap_d = geom.outer_diameter;
    let t = geom.t;

    // Eq. (2): heat emitting reference surface area for TRB
    let a_r = std::f64::consts::PI * t * (d + cap_d);

    // Mean bearing diameter
    let d_m = 0.5 * (d + cap_d);

    // Reference kinematic viscosity at θᵣ=70°C: 12 mm²/s (ISO VG 32, radial bearings §5.2.3.1a)
    let v_r = 12.0_f64;

    // Reference load §5.2.2.1: P₁ᵣ = 0.05·C₀ᵣ [N]
    let p_1r = 0.05 * c_0r_n;

    // Reference heat flow density qᵣ (§5.3.2, radial bearing curve 1)
    let q_r = if a_r <= 50_000.0 {
        0.016
    } else {
        0.016 * (a_r / 50_000.0_f64).powf(-0.34)
    };

    // Reference heat flow Φᵣ = qᵣ·Aᵣ  [Eq. 10]
    let phi_r = q_r * a_r;

    // Solve Eq. (11) for nθr by bisection.
    // LHS(n) = (π·n/30e3)·[1e-7·f₀ᵣ·(vᵣ·n)^(2/3)·dm³ + f₁ᵣ·P₁ᵣ·dm]
    // Find n such that LHS(n) = phi_r
    let lhs = |n: f64| -> f64 {
        let m0r = 1e-7 * f_0r * (v_r * n).powf(2.0 / 3.0) * d_m.powi(3);
        let m1r = f_1r * p_1r * d_m;
        (std::f64::consts::PI * n / 30_000.0) * (m0r + m1r)
    };

    // Bisection between 1 and 1e7 rpm
    let mut lo = 1.0_f64;
    let mut hi = 1.0e7_f64;
    for _ in 0..64 {
        let mid = 0.5 * (lo + hi);
        if lhs(mid) < phi_r {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let n_theta_r = 0.5 * (lo + hi);

    // Evaluate moments and power at nθr
    let m_0r = 1e-7 * f_0r * (v_r * n_theta_r).powf(2.0 / 3.0) * d_m.powi(3);
    let m_1r = f_1r * p_1r * d_m;
    let n_r = (std::f64::consts::PI * n_theta_r / 30_000.0) * (m_0r + m_1r);

    let speed_ratio = if n_theta_r > 0.0 { n_rpm / n_theta_r } else { 0.0 };

    ThermalSpeedResult {
        n_theta_r,
        speed_ratio,
        a_r,
        d_m,
        p_1r,
        m_0r,
        m_1r,
        n_r,
        phi_r,
        q_r,
        f_0r,
        f_1r,
        v_r,
    }
}

// ─── Tests ─────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_roller_result(n_slices: usize, q_normal: f64, psi_deg: f64) -> RollerResult {
        let q_per_slice = q_normal / n_slices as f64;
        let q_k = q_per_slice; // N/mm (assuming 1mm slice width for simplicity)
        let slice_results: Vec<SliceContactResult> = (0..n_slices)
            .map(|k| SliceContactResult {
                k,
                delta_k: if q_normal > 0.0 { 5.0 } else { 0.0 },
                q_k: if q_normal > 0.0 { q_k } else { 0.0 },
                q_k_outer: if q_normal > 0.0 { q_k } else { 0.0 },
                q_k_inner: if q_normal > 0.0 { q_k } else { 0.0 },
                b_k: if q_normal > 0.0 { 0.1 } else { 0.0 },
                p_max_k: if q_normal > 0.0 { 1500.0 } else { 0.0 },
                h_bulk_k: 0.0,
                k_hertz_k: 0.0,
                b_k_outer: if q_normal > 0.0 { 0.1 } else { 0.0 },
                p_max_k_outer: if q_normal > 0.0 { 1500.0 } else { 0.0 },
                h_bulk_k_outer: 0.0,
                k_hertz_k_outer: 0.0,
                k_combined_k: 0.0,
                in_contact: q_normal > 0.0,
            })
            .collect();

        RollerResult {
            psi_deg,
            q_normal,
            q_normal_inner: q_normal,
            slice_results,
            rib_result: None,
        }
    }

    fn make_roller_result_edge_loaded(
        n_slices: usize,
        q_normal: f64,
        psi_deg: f64,
    ) -> RollerResult {
        // Edge-loaded: stress increases toward the ends
        let slice_results: Vec<SliceContactResult> = (0..n_slices)
            .map(|k| {
                let x = (k as f64 - (n_slices - 1) as f64 / 2.0).abs()
                    / ((n_slices - 1) as f64 / 2.0);
                let stress_factor = 1.0 + 0.5 * x * x; // higher at edges
                let q_k = if q_normal > 0.0 {
                    (q_normal / n_slices as f64) * stress_factor
                } else {
                    0.0
                };
                SliceContactResult {
                    k,
                    delta_k: if q_normal > 0.0 { 5.0 } else { 0.0 },
                    q_k,
                    q_k_outer: q_k,
                    q_k_inner: q_k,
                    b_k: if q_normal > 0.0 { 0.1 } else { 0.0 },
                    p_max_k: if q_normal > 0.0 {
                        1500.0 * stress_factor
                    } else {
                        0.0
                    },
                    h_bulk_k: 0.0,
                    k_hertz_k: 0.0,
                    b_k_outer: if q_normal > 0.0 { 0.1 } else { 0.0 },
                    p_max_k_outer: if q_normal > 0.0 {
                        1500.0 * stress_factor
                    } else {
                        0.0
                    },
                    h_bulk_k_outer: 0.0,
                    k_hertz_k_outer: 0.0,
                    k_combined_k: 0.0,
                    in_contact: q_normal > 0.0,
                }
            })
            .collect();

        RollerResult {
            psi_deg,
            q_normal,
            q_normal_inner: q_normal,
            slice_results,
            rib_result: None,
        }
    }

    fn test_geom() -> MacroGeometry {
        MacroGeometry {
            d: 120.0,
            outer_diameter: 180.0,
            t: 38.0,
            alpha: 12.0,
            z: 20,
            d_we_max: 14.5,
            d_we_min: 13.0,
            l_we: 22.0,
            d_pw: 150.0,
            h_rib: 3.0,
            alpha_rib: 84.0,
            g_r: 0.0,
            h_c: None,
        }
    }

    fn test_operating() -> OperatingConditions {
        OperatingConditions {
            f_x: 10.0,
            f_y: 0.0,
            f_a: 5.0,
            m_x: 0.0,
            m_y: 0.0,
            n_inner_rpm: 1000.0,
            n_outer_rpm: 0.0,
            gamma: 0.0,
            t_op: 70.0,
            nu_40: 68.0,
            nu_100: 8.0,
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

    fn test_solver() -> SolverParams {
        SolverParams::default()
    }

    fn make_uniform_rollers(z: u32, n_slices: usize, q_per_roller: f64) -> Vec<RollerResult> {
        (0..z)
            .map(|j| {
                let psi = j as f64 * 360.0 / z as f64;
                make_roller_result(n_slices, q_per_roller, psi)
            })
            .collect()
    }

    // ─── Kappa Tests ─────────────────────────────────────────────

    #[test]
    fn test_kappa_standard_conditions() {
        // nu_40=68, nu_100=8, t_op=70°C, n=1000rpm, d_pw=150mm
        let op = test_operating();
        let kappa = compute_kappa(&op, 150.0);
        // ν_actual at 70°C ≈ 17 mm²/s (interpolated between 8 and 68)
        // ν_ref for d_pw=150, n=1000: Eq.(29) → 4500/(√1000 × √150) = 4500/387.3 ≈ 11.6
        // κ ≈ 17/11.6 ≈ 1.5
        assert!(kappa > 0.5, "κ={kappa} should be > 0.5");
        assert!(kappa < 20.0, "κ={kappa} should be < 20");
    }

    #[test]
    fn test_kappa_low_speed() {
        let mut op = test_operating();
        op.n_inner_rpm = 50.0; // very low speed
        let kappa = compute_kappa(&op, 150.0);
        // Low speed → high ν_ref → low κ
        assert!(kappa > 0.0, "κ must be positive");
    }

    // ─── a_ISO Tests ─────────────────────────────────────────────

    #[test]
    fn test_a_iso_good_lubrication() {
        // κ=2.0, clean (e_c=1.0), high Cu/P=0.5 → a_ISO ≈ 3.08
        let a = compute_a_iso(2.0, 1.0, 0.5);
        assert!(a > 1.0, "a_iso={a} should be > 1.0 for κ=2, Cu/P=0.5");
        assert!(a < 10.0, "a_iso={a} should be < 10.0");
    }

    #[test]
    fn test_a_iso_moderate_cu_p() {
        // κ=2.0, clean, Cu/P=0.1 → a_ISO ≈ 0.52 (ISO 281: low Cu/P limits benefit)
        let a = compute_a_iso(2.0, 1.0, 0.1);
        assert!(a > 0.4 && a < 0.7, "a_iso={a} should be ≈0.52 for κ=2, Cu/P=0.1");
    }

    #[test]
    fn test_a_iso_poor_lubrication() {
        // κ=0.3, contaminated, low Cu/P
        let a = compute_a_iso(0.3, 0.5, 0.05);
        assert!(a < 0.5, "a_iso={a} should be < 0.5 for poor conditions");
        assert!(a >= 0.1, "a_iso={a} must be ≥ 0.1");
    }

    #[test]
    fn test_a_iso_clamped() {
        // Very high κ → capped at 50
        let a = compute_a_iso(10.0, 1.0, 0.5);
        assert!(a <= 50.0, "a_iso must not exceed 50");
        assert!(a > 1.0, "a_iso should be > 1 for favorable conditions");
        // Very low κ → minimum 0.1
        let a_low = compute_a_iso(0.01, 0.1, 0.01);
        assert!(a_low >= 0.1, "a_iso must be ≥ 0.1");
    }

    #[test]
    fn test_a_iso_kappa_monotonic() {
        // a_ISO should increase with κ (for fixed e_c, Cu/P)
        let a1 = compute_a_iso(0.5, 1.0, 0.3);
        let a2 = compute_a_iso(1.0, 1.0, 0.3);
        let a3 = compute_a_iso(2.0, 1.0, 0.3);
        let a4 = compute_a_iso(4.0, 1.0, 0.3);
        assert!(a1 < a2, "a_iso should increase with κ: {a1} < {a2}");
        assert!(a2 < a3, "a_iso should increase with κ: {a2} < {a3}");
        assert!(a3 < a4, "a_iso should increase with κ: {a3} < {a4}");
    }

    #[test]
    fn test_a_iso_matches_iso281_table() {
        // Cross-check against ISO 281:2007 Table 7 reference values
        // κ=1.0, e_c=1.0, Cu/P=0.5 → a_ISO ≈ 1.71
        let a = compute_a_iso(1.0, 1.0, 0.5);
        assert!((a - 1.71).abs() < 0.2, "a_iso={a} should be ≈1.71 for κ=1, Cu/P=0.5");
        // κ=4.0, e_c=1.0, Cu/P=0.5 → a_ISO ≈ 5.59
        let a4 = compute_a_iso(4.0, 1.0, 0.5);
        assert!((a4 - 5.59).abs() < 1.0, "a_iso={a4} should be ≈5.59 for κ=4, Cu/P=0.5");
    }

    // ─── ISO 16281 Life Tests ────────────────────────────────────

    #[test]
    fn test_iso16281_basic_life() {
        let geom = test_geom();
        let op = test_operating();
        let mat = Material::default();
        let solver = test_solver();
        let rollers = make_uniform_rollers(20, 30, 500.0); // uniform 500N per roller

        let result = compute_fatigue_life(&rollers, &geom, &mat, &op, &solver, None).unwrap();
        assert_eq!(result.method, LifeMethod::Iso16281);
        assert!(result.l_10_basic > 0.0, "L_10 must be positive");
        assert!(result.l_nm_hours > 0.0, "L_nm must be positive");
        assert!(result.kappa > 0.0, "κ must be positive");
        assert!(result.c_dyn > 0.0, "C must be positive");
        assert!(result.p_equiv > 0.0, "P must be positive");
        assert!(result.lamina_lives.is_some(), "ISO 16281 now provides lamina detail");
    }

    #[test]
    fn test_iso16281_higher_load_shorter_life() {
        let geom = test_geom();
        let mat = Material::default();
        let solver = test_solver();

        let op_low = OperatingConditions {
            f_x: 5.0,
            f_a: 3.0,
            ..test_operating()
        };
        let op_high = OperatingConditions {
            f_x: 20.0,
            f_a: 10.0,
            ..test_operating()
        };

        let rollers_low = make_uniform_rollers(20, 30, 300.0);
        let rollers_high = make_uniform_rollers(20, 30, 1000.0);

        let life_low =
            compute_fatigue_life(&rollers_low, &geom, &mat, &op_low, &solver, None).unwrap();
        let life_high =
            compute_fatigue_life(&rollers_high, &geom, &mat, &op_high, &solver, None).unwrap();

        assert!(
            life_low.l_10_basic > life_high.l_10_basic,
            "Higher load → shorter life: L_10_low={} vs L_10_high={}",
            life_low.l_10_basic,
            life_high.l_10_basic
        );
    }

    #[test]
    fn test_iso16281_inner_outer_split() {
        let geom = test_geom();
        let mat = Material::default();
        let solver = test_solver();
        let op = test_operating();
        let rollers = make_uniform_rollers(20, 30, 500.0);

        let result = compute_fatigue_life(&rollers, &geom, &mat, &op, &solver, None).unwrap();
        // Combined life should be less than either ring life
        assert!(
            result.l_10_basic <= result.l_10_inner,
            "Combined ≤ inner: {} vs {}",
            result.l_10_basic,
            result.l_10_inner
        );
        assert!(
            result.l_10_basic <= result.l_10_outer,
            "Combined ≤ outer: {} vs {}",
            result.l_10_basic,
            result.l_10_outer
        );
    }

    // ─── Edge Load Life Test ─────────────────────────────────────

    #[test]
    fn test_edge_loaded_shorter_life() {
        // Edge loading should result in shorter life than uniform loading
        let geom = test_geom();
        let mat = Material::default();
        let solver = test_solver();
        let op = test_operating();

        let uniform_rollers = make_uniform_rollers(20, 30, 500.0);
        let edge_rollers: Vec<RollerResult> = (0..20)
            .map(|j| {
                let psi = j as f64 * 18.0;
                make_roller_result_edge_loaded(30, 500.0, psi)
            })
            .collect();

        let life_uniform =
            compute_fatigue_life(&uniform_rollers, &geom, &mat, &op, &solver, None).unwrap();
        let life_edge =
            compute_fatigue_life(&edge_rollers, &geom, &mat, &op, &solver, None).unwrap();

        assert!(
            life_edge.l_10_basic < life_uniform.l_10_basic,
            "Edge loading should reduce life: uniform={}, edge={}",
            life_uniform.l_10_basic,
            life_edge.l_10_basic
        );
    }

    // ─── HR30306J ν₁ Regression ─────────────────────────────────

    #[test]
    fn test_nu1_hr30306j_matches_masta() {
        // D_pw=51.56mm, n=1500rpm → ν₁ = 4500/(√1500 × √51.56) = 16.18
        let nu1 = reference_viscosity(51.56, 1500.0);
        assert!((nu1 - 16.18).abs() < 0.02, "ν₁={nu1:.4}, expected 16.18 (MASTA)");
    }

    #[test]
    fn test_kappa_hr30306j() {
        // ν₄₀=220, ν₁₀₀=18.09, T_op=68°C, D_pw=51.56, n=1500
        let op = OperatingConditions {
            nu_40: 220.0, nu_100: 18.09, t_op: 68.0, n_inner_rpm: 1500.0,
            ..test_operating()
        };
        let kappa = compute_kappa(&op, 51.56);
        // MASTA κ = 3.4134, our ν(68°C) ≈ 54.26 → κ ≈ 54.26/16.18 ≈ 3.35
        assert!((kappa - 3.41).abs() < 0.15, "κ={kappa:.4}, expected ~3.35-3.41");
    }

    // ─── Viscosity Tests ─────────────────────────────────────────

    #[test]
    fn test_viscosity_interpolation() {
        // At 40°C should return ~68
        let nu_at_40 = viscosity_at_temp(68.0, 8.0, 40.0);
        assert!(
            (nu_at_40 - 68.0).abs() < 1.0,
            "ν(40°C)={nu_at_40} should be ≈68"
        );
        // At 100°C should return ~8
        let nu_at_100 = viscosity_at_temp(68.0, 8.0, 100.0);
        assert!(
            (nu_at_100 - 8.0).abs() < 0.5,
            "ν(100°C)={nu_at_100} should be ≈8"
        );
        // At 70°C should be between 8 and 68
        let nu_at_70 = viscosity_at_temp(68.0, 8.0, 70.0);
        assert!(nu_at_70 > 8.0 && nu_at_70 < 68.0, "ν(70°C)={nu_at_70}");
    }

    // ─── C_r Calculation Test ────────────────────────────────────

    #[test]
    fn test_c_r_reasonable() {
        let geom = test_geom();
        let c_r = compute_c_r(&geom);
        // For a medium-size TRB (d=120mm), C_r should be in range 50~500 kN
        assert!(
            c_r > 10.0 && c_r < 1000.0,
            "C_r={c_r} kN should be in reasonable range"
        );
    }

    #[test]
    fn test_thermal_speed_rating() {
        // NSK HR30306J: d=30, D=72, T=20.75, C0r=59.8kN, n=1500rpm
        use crate::solver::types::*;
        let geom = MacroGeometry {
            d: 30.0, outer_diameter: 72.0, t: 20.75, alpha: 11.859,
            z: 14, d_we_max: 10.9371, d_we_min: 10.123273, l_we: 11.65,
            d_pw: 51.0, h_rib: 2.5, alpha_rib: 9.855, g_r: 0.0, h_c: None,
        };
        let result = compute_thermal_speed_rating(&geom, 59800.0, 1500.0, 3.0, 0.0004);
        // nθr should be a reasonable speed for small TRB (~10000-50000 rpm)
        assert!(result.n_theta_r > 1000.0, "nθr={} should be > 1000", result.n_theta_r);
        assert!(result.speed_ratio > 0.0, "speed_ratio should be positive");
        assert!((result.n_r - result.phi_r).abs() < 1e-6, "energy balance: Nr ≈ Φr at nθr");
    }

    #[test]
    fn test_p_equiv_pure_radial() {
        // F_a/F_r < e → P = F_r
        let p = compute_p_equiv(10.0, 0.5, 12.0);
        // e = 1.5 × tan(12°) ≈ 0.319, F_a/F_r = 0.05 < 0.319
        assert!(
            (p - 10.0).abs() < 0.01,
            "Pure radial: P={p} should be ≈ F_r=10"
        );
    }

    #[test]
    fn test_p_equiv_combined() {
        // F_a/F_r > e → P = 0.4·F_r + Y·F_a
        let p = compute_p_equiv(10.0, 5.0, 12.0);
        // F_a/F_r = 0.5 > 0.319, Y = 0.4/tan(12°) ≈ 1.88
        // P = 0.4×10 + 1.88×5 = 4 + 9.4 = 13.4
        assert!(p > 10.0, "Combined load P={p} should be > F_r=10");
    }
}
