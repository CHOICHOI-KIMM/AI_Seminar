use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::solver::bearing;
use crate::solver::gen1;
use crate::solver::gen3;
use crate::solver::geometry::compute_slices;
use crate::solver::hertz::{combined_elastic_modulus, compute_slice_contact};
use crate::solver::rib_contact;
use crate::solver::transient;
use crate::solver::transient_io;
use crate::solver::types::*;

/// Tauri event-based progress reporter.
struct TauriReporter {
    app: AppHandle,
}

impl ProgressReporter for TauriReporter {
    fn report(&self, progress: SolverProgress) {
        let _ = self.app.emit("solver-progress", &progress);
    }
}

#[tauri::command]
pub fn compute_slice_geometry(input: BearingInput) -> Result<Vec<SliceGeometry>, String> {
    input.validate().map_err(|e| e.to_string())?;

    compute_slices(
        &input.macro_geom,
        &input.raceway_geom,
        &input.roller_profile,
        &input.raceway_profile_inner,
        &input.raceway_profile_outer,
        input.solver.n_slices,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn compute_hertz_single_slice(
    delta_k: f64,
    r_eq: f64,
    e_roller: f64,
    e_ring: f64,
    nu: f64,
    slice_width: f64,
    h1: f64,
    h2: f64,
) -> Result<SliceContactResult, String> {
    if r_eq <= 0.0 {
        return Err("Equivalent radius must be positive".into());
    }

    let e_star_gpa = combined_elastic_modulus(e_roller, nu, e_ring, nu);
    let e_star_mpa = e_star_gpa * 1000.0;
    let e_avg_mpa = ((e_roller + e_ring) / 2.0) * 1000.0;

    Ok(compute_slice_contact(
        0, delta_k, r_eq, r_eq, e_star_mpa, e_avg_mpa, nu, slice_width, h1, h2, 0.0,
    ))
}

// ─── Gen1 Solver Commands ──────────────────────────────────────────

#[derive(Serialize)]
pub struct Gen1Result {
    pub slice_results: Vec<SliceContactResult>,
    pub q_total: f64,
    pub delta_rigid: f64,
}

/// Solve single roller with Gen1 (independent slices) given delta_rigid directly.
#[tauri::command]
pub fn solve_roller_gen1(input: BearingInput, delta_rigid: f64) -> Result<Gen1Result, String> {
    input.validate().map_err(|e| e.to_string())?;

    let slices = compute_slices(
        &input.macro_geom,
        &input.raceway_geom,
        &input.roller_profile,
        &input.raceway_profile_inner,
        &input.raceway_profile_outer,
        input.solver.n_slices,
    )
    .map_err(|e| e.to_string())?;

    let cos_alpha_diff = (input.raceway_geom.alpha_o.to_radians() - input.raceway_geom.alpha_i.to_radians()).cos();
    let (slice_results, q_total) = gen1::solve_gen1_roller(&slices, delta_rigid, &input.material, cos_alpha_diff);

    Ok(Gen1Result {
        slice_results,
        q_total,
        delta_rigid,
    })
}

/// Solve single roller with Gen1 for a target normal load Q [N].
#[tauri::command]
pub fn solve_roller_gen1_for_load(
    input: BearingInput,
    q_target: f64,
) -> Result<Gen1Result, String> {
    input.validate().map_err(|e| e.to_string())?;

    let slices = compute_slices(
        &input.macro_geom,
        &input.raceway_geom,
        &input.roller_profile,
        &input.raceway_profile_inner,
        &input.raceway_profile_outer,
        input.solver.n_slices,
    )
    .map_err(|e| e.to_string())?;

    let cos_alpha_diff = (input.raceway_geom.alpha_o.to_radians() - input.raceway_geom.alpha_i.to_radians()).cos();
    let (slice_results, q_total, delta_rigid) =
        gen1::solve_gen1_for_load(&slices, q_target, &input.material, &input.solver, cos_alpha_diff)
            .map_err(|e| e.to_string())?;

    Ok(Gen1Result {
        slice_results,
        q_total,
        delta_rigid,
    })
}

// ─── Gen3 Solver Commands ──────────────────────────────────────────

#[derive(Serialize)]
pub struct Gen3Result {
    pub slice_results: Vec<SliceContactResult>,
    pub q_total: f64,
    pub delta_rigid: f64,
    pub beam_deflection: Vec<f64>,
    pub max_deflection: f64,
}

/// Solve single roller with Gen3 (beam-coupled) given delta_rigid directly.
#[tauri::command]
pub fn solve_roller_gen3(input: BearingInput, delta_rigid: f64) -> Result<Gen3Result, String> {
    input.validate().map_err(|e| e.to_string())?;

    let slices = compute_slices(
        &input.macro_geom,
        &input.raceway_geom,
        &input.roller_profile,
        &input.raceway_profile_inner,
        &input.raceway_profile_outer,
        input.solver.n_slices,
    )
    .map_err(|e| e.to_string())?;

    let cos_alpha_diff = (input.raceway_geom.alpha_o.to_radians() - input.raceway_geom.alpha_i.to_radians()).cos();
    let (slice_results, q_total) =
        gen3::solve_gen3_roller(&slices, delta_rigid, &input.material, &input.solver, cos_alpha_diff)
            .map_err(|e| e.to_string())?;

    let beam_deflection: Vec<f64> = slice_results
        .iter()
        .zip(slices.iter())
        .map(|(r, s)| delta_rigid - r.delta_k - s.delta_z_total_outer
            - s.delta_z_total_inner * cos_alpha_diff)
        .collect();
    let max_deflection = beam_deflection.iter().map(|w| w.abs()).fold(0.0, f64::max);

    Ok(Gen3Result {
        slice_results,
        q_total,
        delta_rigid,
        beam_deflection,
        max_deflection,
    })
}

/// Solve single roller with Gen3 for a target normal load Q [N].
#[tauri::command]
pub fn solve_roller_gen3_for_load(
    input: BearingInput,
    q_target: f64,
) -> Result<Gen3Result, String> {
    input.validate().map_err(|e| e.to_string())?;

    let slices = compute_slices(
        &input.macro_geom,
        &input.raceway_geom,
        &input.roller_profile,
        &input.raceway_profile_inner,
        &input.raceway_profile_outer,
        input.solver.n_slices,
    )
    .map_err(|e| e.to_string())?;

    let cos_alpha_diff = (input.raceway_geom.alpha_o.to_radians() - input.raceway_geom.alpha_i.to_radians()).cos();
    let (slice_results, q_total, delta_rigid) =
        gen3::solve_gen3_for_load(&slices, q_target, &input.material, &input.solver, cos_alpha_diff)
            .map_err(|e| e.to_string())?;

    let beam_deflection: Vec<f64> = slice_results
        .iter()
        .zip(slices.iter())
        .map(|(r, s)| delta_rigid - r.delta_k - s.delta_z_total_outer
            - s.delta_z_total_inner * cos_alpha_diff)
        .collect();
    let max_deflection = beam_deflection.iter().map(|w| w.abs()).fold(0.0, f64::max);

    Ok(Gen3Result {
        slice_results,
        q_total,
        delta_rigid,
        beam_deflection,
        max_deflection,
    })
}

// ─── Bearing Equilibrium Commands ────────────────────────────────────

/// Solve full bearing equilibrium (5-DOF) for given operating conditions.
#[tauri::command]
pub async fn solve_bearing(app: AppHandle, input: BearingInput) -> Result<BearingResult, String> {
    let reporter = TauriReporter { app };
    tauri::async_runtime::spawn_blocking(move || {
        bearing::solve_bearing_equilibrium(&input, &reporter).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("Task join error: {e}"))?
}

/// Solve bearing in dual mode: Gen1 + Gen3 comparison.
#[tauri::command]
pub async fn solve_bearing_dual(app: AppHandle, input: BearingInput) -> Result<DualModeComparison, String> {
    let reporter = TauriReporter { app };
    tauri::async_runtime::spawn_blocking(move || {
        bearing::solve_bearing_dual(&input, &reporter).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("Task join error: {e}"))?
}

/// Compute rib contact for a given axial force.
#[tauri::command]
pub fn compute_rib(input: BearingInput, q_axial: f64) -> Result<RibContactResult, String> {
    rib_contact::compute_rib_contact(
        &input.roller_profile,
        &input.macro_geom,
        &input.raceway_geom,
        &input.material,
        q_axial,
        Some(&input.operating),
    )
    .map_err(|e| e.to_string())
}

/// Solve transient roller dynamics.
#[tauri::command]
pub async fn solve_transient(app: AppHandle, input: BearingInput) -> Result<TransientResult, String> {
    let reporter = TauriReporter { app };
    tauri::async_runtime::spawn_blocking(move || {
        transient::solve_transient(&input, &reporter).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("Task join error: {e}"))?
}

/// Parse CSV load time series and return as LoadTimePoint vector.
#[tauri::command]
pub fn parse_load_csv(csv_text: String) -> Result<Vec<LoadTimePoint>, String> {
    transient_io::parse_load_csv(csv_text.as_bytes()).map_err(|e| e.to_string())
}

/// Run HMEHL analysis for the max-loaded roller contact.
/// HMEHL EHL analysis — computes EHL stress distribution on top of
/// Load Distribution results.
///
/// Load Distribution provides: q_k [N/mm], p_hertz [MPa], R_eq [mm]
/// HMEHL adds: EHL pressure spike, film thickness, temperature, friction.
///
/// The Hertz pressure is PASSED IN, not recalculated — guarantees match
/// with Load Distribution.
#[tauri::command]
pub fn run_hmehl(
    input: BearingInput,
    q_k_nmm: f64,       // per-slice line load [N/mm] from Load Distribution
    p_hertz_mpa: f64,    // Hertz p_max [MPa] from Load Distribution
    r_eq_mm: f64,        // equivalent radius [mm] from SliceGeometry
    is_inner: bool,
) -> Result<crate::solver::hmehl::HMEHLResult, String> {
    use crate::solver::hmehl::{ContactParams, HMEHLSolver};

    input.validate().map_err(|e| e.to_string())?;

    let geom = &input.macro_geom;
    let mat = &input.material;
    let op = &input.operating;

    // Material — SI units for HMEHL solver
    let nu = mat.nu;
    let e_prime = 2.0 / ((1.0 - nu * nu) / (mat.e_roller * 1e9) + (1.0 - nu * nu) / (mat.e_ring * 1e9));

    // Lubricant at operating temperature
    let nu_op = crate::solver::life::viscosity_at_temp_pub(op.nu_40, op.nu_100, op.t_op);
    let eta_0 = nu_op * 1e-6 * op.rho_oil;
    let alpha = op.alpha_pv * 1e-9;

    // Convert Load Distribution values to SI
    let r_eq = r_eq_mm * 1e-3;         // mm → m
    let l_contact = geom.l_we * 1e-3;  // mm → m
    let q_si = q_k_nmm * 1e3;          // N/mm → N/m
    let f_n = q_si * l_contact;         // N

    // Kinematics
    let d_we_mean = (geom.d_we_max + geom.d_we_min) / 2.0;
    let alpha_rad = geom.alpha.to_radians();
    let gamma = d_we_mean * alpha_rad.cos() / geom.d_pw;
    let r_pw = geom.d_pw / 2.0 * 1e-3;
    let omega_inner = op.n_inner_rpm * std::f64::consts::TAU / 60.0;
    let u_roller = omega_inner * r_pw * (1.0 - gamma * gamma) / 2.0;
    let srr_typical = 0.04;
    let u1 = u_roller * (1.0 + srr_typical / 2.0);
    let u2 = u_roller * (1.0 - srr_typical / 2.0);

    // Roughness
    let rq = if is_inner {
        ((op.rq_roller_eff().powi(2) + op.rq_inner_eff().powi(2)).sqrt()) * 1e-6
    } else {
        ((op.rq_roller_eff().powi(2) + op.rq_outer_eff().powi(2)).sqrt()) * 1e-6
    };

    let params = ContactParams {
        f_n,
        u1, u2,
        r_eq,
        l_contact,
        e_prime,
        eta_0,
        alpha,
        rho_0: op.rho_oil,
        rq,
        r_cl: 20e-6,
        hardness_pa: (mat.hrc * 10.0) * 9.81e6,
        t_inlet: op.t_op,
        k_solid: 46.0,
        rho_cp_solid: 3.6e6,
        k_lub: 0.14,
        rho_cp_lub: 1.7e6,
        visc_temp_index: 1.1,
    };

    let moes_m_log = {
        let up = params.eta_0 * params.u_m() / (params.e_prime * params.r_eq);
        let wp = params.q() / (params.e_prime * params.r_eq);
        wp / up.powf(0.75)
    };
    eprintln!("[HMEHL] {} q_k={:.1}N/mm, R_eq={:.3}mm, p_hertz(LD)={:.0}MPa, p_h(calc)={:.0}MPa, \
        F_n={:.1}N, u_m={:.3}m/s, η₀={:.4}Pa·s, α={:.1}GPa⁻¹, M={:.0}",
        if is_inner {"inner"} else {"outer"}, q_k_nmm, r_eq_mm,
        p_hertz_mpa, params.hertz_pressure()/1e6,
        params.f_n, params.u_m(), params.eta_0, params.alpha*1e9, moes_m_log);

    let solver = HMEHLSolver::new(256);
    let mut result = solver.solve(&params);

    // Override hertz_pressure_ref with the EXACT Load Distribution Hertz pressure
    // (eliminates any numerical difference from unit conversion)
    let p_h_ld = p_hertz_mpa * 1e6; // MPa → Pa
    let nx = result.hertz_pressure_ref.len();
    let domain = solver.domain_mult;
    for i in 0..nx {
        let x = -domain + i as f64 * (2.0 * domain / (nx - 1) as f64);
        result.hertz_pressure_ref[i] = if x.abs() < 1.0 {
            (1.0 - x * x).sqrt() * p_h_ld
        } else { 0.0 };
    }

    Ok(result)
}
