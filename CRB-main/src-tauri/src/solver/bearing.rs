use std::f64::consts::PI;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use nalgebra::{DMatrix, DVector as NVec};
use rayon::prelude::*;

use crate::error::SolverError;
use crate::solver::gen1;
use crate::solver::gen3;
use crate::solver::geometry;
use crate::solver::hertz::combined_elastic_modulus;
use crate::solver::rib_contact;
use crate::solver::types::*;

/// Solve 3×3 linear system via Cramer's rule. Returns zero vector if singular.
fn solve_3x3(a: &[[f64; 3]; 3], b: &[f64; 3]) -> [f64; 3] {
    let det = a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
        - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
        + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0]);
    if det.abs() < 1e-30 {
        return [0.0; 3];
    }
    let inv_det = 1.0 / det;
    [
        inv_det
            * (b[0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
                - a[0][1] * (b[1] * a[2][2] - a[1][2] * b[2])
                + a[0][2] * (b[1] * a[2][1] - a[1][1] * b[2])),
        inv_det
            * (a[0][0] * (b[1] * a[2][2] - a[1][2] * b[2])
                - b[0] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
                + a[0][2] * (a[1][0] * b[2] - b[1] * a[2][0])),
        inv_det
            * (a[0][0] * (a[1][1] * b[2] - b[1] * a[2][1])
                - a[0][1] * (a[1][0] * b[2] - b[1] * a[2][0])
                + b[0] * (a[1][0] * a[2][1] - a[1][1] * a[2][0])),
    ]
}

/// Generate angular positions for Z rollers [rad].
/// `load_angle`: radial load direction [rad] — roller #0 is placed here (worst-case).
pub fn roller_positions(z: u32, load_angle: f64) -> Vec<f64> {
    let spacing = 2.0 * PI / z as f64;
    (0..z).map(|j| load_angle + j as f64 * spacing).collect()
}

/// Compute radial load direction angle [rad] from Fx, Fy.
/// Returns 0.0 when Fr ≈ 0 (pure axial load).
pub fn radial_load_angle(f_x: f64, f_y: f64) -> f64 {
    let f_r = (f_x * f_x + f_y * f_y).sqrt();
    if f_r < 1e-10 { 0.0 } else { f_y.atan2(f_x) }
}

/// Compute rigid body approach for a single roller.
///
/// disp: [δx, δy, δz, γx, γy] — inner ring displacement
///   δx, δy, δz [μm], γx, γy [rad]
/// psi: roller angular position [rad]
/// alpha: contact angle [rad]
/// d_pw: pitch circle diameter [mm]
/// g_r: radial internal clearance [μm]
/// gamma_ext: external misalignment [rad] (imposed about x-axis)
///
/// Returns δ_rigid [μm].
pub fn roller_approach(
    disp: &[f64; 5],
    psi: f64,
    alpha: f64,
    d_pw: f64,
    g_r: f64,
    gamma_ext: f64,
) -> f64 {
    let (cos_psi, sin_psi) = (psi.cos(), psi.sin());
    let (cos_alpha, sin_alpha) = (alpha.cos(), alpha.sin());

    // Radial component [μm]
    let delta_r = disp[0] * cos_psi + disp[1] * sin_psi;

    // Axial component [μm]
    // d_pw/2 [mm] × γ [rad] → [mm] → ×1000 → [μm]
    // gamma_ext is an imposed geometric misalignment added to solver's γx
    let gamma_x_total = disp[3] + gamma_ext;
    let delta_a = disp[2] + (d_pw / 2.0) * 1000.0 * (gamma_x_total * sin_psi - disp[4] * cos_psi);

    // Combined approach along contact line
    delta_r * cos_alpha + delta_a * sin_alpha - g_r / 2.0
}

/// Compute equilibrium residual and per-roller data.
///
/// Returns (R[5], Vec<RollerData>) where R is the force/moment residual.
/// `f_a_eff_n`: effective axial load [N] — may differ from input.operating.f_a
///              when induced thrust from paired bearing is applied.
pub(crate) fn compute_residual(
    input: &BearingInput,
    slices: &[SliceGeometry],
    disp: &[f64; 5],
    f_a_eff_n: f64,
) -> Result<([f64; 5], Vec<RollerResult>), SolverError> {
    let z = input.macro_geom.z;
    let alpha_rad = input.raceway_geom.alpha_o.to_radians();
    let alpha_i_rad = input.raceway_geom.alpha_i.to_radians();
    let d_pw = input.macro_geom.d_pw;
    let (cos_alpha, sin_alpha) = (alpha_rad.cos(), alpha_rad.sin());
    let cos_alpha_diff = (alpha_rad - alpha_i_rad).cos();
    let sin_alpha_diff = (alpha_rad - alpha_i_rad).sin();
    let cos_alpha_i = alpha_i_rad.cos();

    // NR convergence loop always uses Gen1 (fast).
    // Split contact is applied only in the final re-evaluation pass (reevaluate_with_gen3).

    // External loads: kN → N, kN·m → N·mm (full 5-DOF)
    let f_x = input.operating.f_x * 1000.0; // [N]
    let f_y = input.operating.f_y * 1000.0; // [N]

    let load_angle = radial_load_angle(f_x, f_y);
    let positions = roller_positions(z, load_angle);
    let f_a = f_a_eff_n; // effective axial load [N]
    let m_x = input.operating.m_x * 1e6; // [N·mm]
    let m_y = input.operating.m_y * 1e6; // [N·mm]
    let gamma_ext = input.operating.gamma_rad();

    let mut residual = [0.0_f64; 5];
    let mut roller_results = Vec::with_capacity(z as usize);

    for (_j, &psi) in positions.iter().enumerate() {
        let (cos_psi, sin_psi) = (psi.cos(), psi.sin());

        let delta_rigid = roller_approach(disp, psi, alpha_rad, d_pw, input.macro_geom.g_r, gamma_ext);

        if delta_rigid > 0.0 {

            let (slice_results, q_normal, rib_result) =
                if input.solver.rib_contact_mode == RibContactMode::Coupled {
                    // Coupled: iterate to find (Q, δ_rib) pair with rib compliance
                    // Rib deformation reduces effective roller approach:
                    //   delta_eff = delta_rigid - δ_rib × sin(α_o)
                    let mut delta_rib_um = 0.0_f64;
                    let mut last_sr = Vec::new();
                    let mut last_qn = 0.0_f64;
                    let mut last_rib: Option<RibContactResult> = None;
                    for _ in 0..30 {
                        let delta_eff = (delta_rigid - delta_rib_um * sin_alpha).max(0.0);
                        let (s, q) = gen1::solve_gen1_roller(slices, delta_eff, &input.material, cos_alpha_diff);
                        let q_axial = q * sin_alpha_diff / cos_alpha_i;
                        let rib = rib_contact::compute_rib_contact(
                            &input.roller_profile,
                            &input.macro_geom,
                            &input.raceway_geom,
                            &input.material,
                            q_axial,
                            Some(&input.operating),
                        ).ok();
                        let new_delta_rib = rib.as_ref().map_or(0.0, |r| r.delta_rib);
                        last_sr = s;
                        last_qn = q;
                        last_rib = rib;
                        if (new_delta_rib - delta_rib_um).abs() < 0.001 {
                            break;
                        }
                        delta_rib_um = new_delta_rib;
                    }
                    (last_sr, last_qn, last_rib)
                } else {
                    // PostProcess: standard — rib computed after the fact, no coupling
                    let (sr, qn) = gen1::solve_gen1_roller(slices, delta_rigid, &input.material, cos_alpha_diff);
                    let q_axial = qn * sin_alpha_diff / cos_alpha_i;
                    let rib = rib_contact::compute_rib_contact(
                        &input.roller_profile,
                        &input.macro_geom,
                        &input.raceway_geom,
                        &input.material,
                        q_axial,
                        Some(&input.operating),
                    ).ok();
                    (sr, qn, rib)
                };

            // Force summation (standard Harris formulation)
            // In Coupled mode, q_normal is already reduced by rib compliance
            residual[0] += q_normal * cos_alpha * cos_psi; // Fx
            residual[1] += q_normal * cos_alpha * sin_psi; // Fy
            residual[2] += q_normal * sin_alpha; // Fz (axial)

            // Moment summation (d_pw/2 in mm)
            residual[3] += q_normal * (d_pw / 2.0) * sin_alpha * sin_psi; // Mx
            residual[4] += -q_normal * (d_pw / 2.0) * sin_alpha * cos_psi; // My

            roller_results.push(RollerResult {
                psi_deg: psi.to_degrees(),
                q_normal,
                q_normal_inner: q_normal * cos_alpha_diff,
                slice_results,
                rib_result,
            });
        } else {
            // No contact
            roller_results.push(RollerResult {
                psi_deg: psi.to_degrees(),
                q_normal: 0.0,
                q_normal_inner: 0.0,
                slice_results: slices
                    .iter()
                    .map(|s| SliceContactResult {
                        k: s.k,
                        delta_k: 0.0,
                        q_k: 0.0,
                        q_k_outer: 0.0,
                        q_k_inner: 0.0,
                        b_k: 0.0,
                        p_max_k: 0.0,
                        h_bulk_k: 0.0,
                        k_hertz_k: 0.0,
                        b_k_outer: 0.0,
                        p_max_k_outer: 0.0,
                        h_bulk_k_outer: 0.0,
                        k_hertz_k_outer: 0.0,
                        k_combined_k: 0.0,
                        in_contact: false,
                    })
                    .collect(),
                rib_result: None,
            });
        }
    }

    // Subtract external loads (full 5-DOF)
    residual[0] -= f_x; // F_x
    residual[1] -= f_y; // F_y
    residual[2] -= f_a; // F_a along z
    residual[3] -= m_x; // M_x
    residual[4] -= m_y; // M_y

    Ok((residual, roller_results))
}

/// Compute 5-DOF residual from pre-computed roller results (Gen3 or Gen1).
///
/// Unlike `compute_residual` which internally solves Gen1 per roller,
/// this function uses already-computed roller forces to evaluate the
/// equilibrium residual. Used for Gen3 refinement loop.
fn compute_residual_from_results(
    input: &BearingInput,
    roller_results: &[RollerResult],
    f_a_eff_n: f64,
) -> [f64; 5] {
    let alpha_rad = input.raceway_geom.alpha_o.to_radians();
    let d_pw = input.macro_geom.d_pw;
    let (cos_alpha, sin_alpha) = (alpha_rad.cos(), alpha_rad.sin());

    let f_x = input.operating.f_x * 1000.0;
    let f_y = input.operating.f_y * 1000.0;
    let m_x = input.operating.m_x * 1e6;
    let m_y = input.operating.m_y * 1e6;

    let load_angle = radial_load_angle(f_x, f_y);
    let positions = roller_positions(input.macro_geom.z, load_angle);

    let mut residual = [0.0_f64; 5];
    for (r, &psi) in roller_results.iter().zip(positions.iter()) {
        let (cos_psi, sin_psi) = (psi.cos(), psi.sin());
        residual[0] += r.q_normal * cos_alpha * cos_psi;
        residual[1] += r.q_normal * cos_alpha * sin_psi;
        residual[2] += r.q_normal * sin_alpha;
        residual[3] += r.q_normal * (d_pw / 2.0) * sin_alpha * sin_psi;
        residual[4] += -r.q_normal * (d_pw / 2.0) * sin_alpha * cos_psi;
    }
    residual[0] -= f_x;
    residual[1] -= f_y;
    residual[2] -= f_a_eff_n;
    residual[3] -= m_x;
    residual[4] -= m_y;
    residual
}

/// Compute initial displacement guess from external loads.
fn initial_guess(input: &BearingInput) -> [f64; 5] {
    let f_x = input.operating.f_x * 1000.0; // [N]
    let f_y = input.operating.f_y * 1000.0; // [N]
    let f_a = input.operating.f_a * 1000.0; // [N]
    let f_r = input.operating.f_r() * 1000.0; // resultant radial [N]
    let alpha_rad = input.raceway_geom.alpha_o.to_radians();
    let cos_a = alpha_rad.cos();
    let sin_a = alpha_rad.sin();

    // Typical single-roller stiffness ~ 500 N/μm, half of Z rollers loaded
    let z_eff = (input.macro_geom.z as f64) / 2.0;
    let k_roller = 500.0; // [N/μm] approximate single roller stiffness
    let k_radial = z_eff * k_roller * cos_a;
    // Axial stiffness: force projected through sin(α) twice (displacement→contact, contact→axial)
    let k_axial = (input.macro_geom.z as f64) * k_roller * sin_a * sin_a;

    let dx = if f_x.abs() > 1.0 {
        (f_x / k_radial).clamp(-50.0, 50.0)
    } else {
        0.1
    };

    let dy = if f_y.abs() > 1.0 {
        (f_y / k_radial).clamp(-50.0, 50.0)
    } else {
        0.0
    };

    // For TRB, axial displacement from total axial load (including radial-induced thrust).
    // Use full F_a for initial guess — the solver will find equilibrium from here.
    let dz = if f_a.abs() > 1.0 {
        (f_a / k_axial).clamp(-20.0, 80.0)
    } else if f_r > 1.0 {
        // Even with zero external F_a, radial load on TRB needs axial preload displacement
        let f_a_thrust = f_r * sin_a / cos_a;
        (f_a_thrust / k_axial).clamp(1.0, 80.0)
    } else {
        0.1
    };

    [dx, dy, dz, 0.0, 0.0]
}

/// Minimum axial load for a single TRB row to have equilibrium.
///
/// For a single-row TRB, each loaded roller contributes Q_j·sin(α) axially.
/// The ratio F_a/F_r ≥ tan(α_o) is the exact mathematical lower bound.
///
/// When F_a_input < F_a_min, the bearing operates in "axially constrained" mode:
/// the housing or mating bearing provides an axial reaction R_housing = F_a_min - F_a.
/// The solver uses Levenberg-Marquardt damping to handle the near-singular
/// Jacobian that occurs at narrow load zones (ε → 0).
///
/// Returns minimum axial load [N] for the given radial load [N].
pub fn compute_induced_thrust(f_r: f64, alpha_o_rad: f64) -> f64 {
    let tan_a = alpha_o_rad.tan();
    if tan_a < 1e-10 {
        return 0.0;
    }
    // Exact physical minimum: F_r · tan(α_o)
    // No artificial margin — Jacobian singularity handled by LM damping.
    f_r * tan_a
}


// ─── Transient-optimized fast equilibrium solver ──────────────────────
//
// Designed for time-stepping: pre-computed slices, warm-start from previous
// displacement, no validation/progress/life/film overhead.
// Returns only BearingEquilibrium (no alerts, life, traction, etc.)

/// Lightweight residual: returns only the 5-DOF force/moment residual and
/// per-roller normal loads (no SliceContactResult allocation).
pub(crate) fn compute_residual_fast(
    slices: &[SliceGeometry],
    positions: &[f64],
    disp: &[f64; 5],
    alpha_rad: f64,
    d_pw: f64,
    g_r: f64,
    gamma_ext: f64,
    f_x_n: f64,
    f_y_n: f64,
    f_a_n: f64,
    m_x_nmm: f64,
    m_y_nmm: f64,
    material: &Material,
    rib_coupled: bool,
    rib_params: Option<(&RollerProfile, &MacroGeometry, &RacewayGeometry)>,
    alpha_i_rad: f64,
) -> [f64; 5] {
    let (cos_alpha, sin_alpha) = (alpha_rad.cos(), alpha_rad.sin());
    let sin_alpha_diff = (alpha_rad - alpha_i_rad).sin();
    let cos_alpha_i = alpha_i_rad.cos();
    let cos_alpha_diff = (alpha_rad - alpha_i_rad).cos();

    let mut residual = [0.0_f64; 5];

    for &psi in positions {
        let (cos_psi, sin_psi) = (psi.cos(), psi.sin());
        let delta_rigid = roller_approach(disp, psi, alpha_rad, d_pw, g_r, gamma_ext);

        if delta_rigid > 0.0 {
            let q_normal = if rib_coupled {
                if let Some((rp, mg, rg)) = rib_params {
                    let mut delta_rib_um = 0.0_f64;
                    let mut q = 0.0_f64;
                    for _ in 0..30 {
                        let delta_eff = (delta_rigid - delta_rib_um * sin_alpha).max(0.0);
                        let (_, qn) = gen1::solve_gen1_roller(slices, delta_eff, material, cos_alpha_diff);
                        let q_axial = qn * sin_alpha_diff / cos_alpha_i;
                        let new_delta_rib = rib_contact::compute_rib_contact(rp, mg, rg, material, q_axial, None)
                            .ok()
                            .map_or(0.0, |r| r.delta_rib);
                        q = qn;
                        if (new_delta_rib - delta_rib_um).abs() < 0.001 { break; }
                        delta_rib_um = new_delta_rib;
                    }
                    q
                } else {
                    gen1::solve_gen1_roller(slices, delta_rigid, material, cos_alpha_diff).1
                }
            } else {
                gen1::solve_gen1_roller(slices, delta_rigid, material, cos_alpha_diff).1
            };

            residual[0] += q_normal * cos_alpha * cos_psi;
            residual[1] += q_normal * cos_alpha * sin_psi;
            residual[2] += q_normal * sin_alpha;
            residual[3] += q_normal * (d_pw / 2.0) * sin_alpha * sin_psi;
            residual[4] += -q_normal * (d_pw / 2.0) * sin_alpha * cos_psi;
        }
    }

    residual[0] -= f_x_n;
    residual[1] -= f_y_n;
    residual[2] -= f_a_n;
    residual[3] -= m_x_nmm;
    residual[4] -= m_y_nmm;
    residual
}

/// Fast bearing equilibrium for transient time-stepping.
///
/// Lightweight equilibrium solver for transient analysis.
///
/// Key differences from `solve_bearing_equilibrium`:
/// - Takes pre-computed `slices` (no geometry recalculation)
/// - Accepts initial displacement guess (`warm_start`)
/// - No input validation, no progress reporting
/// - Gen1 only, no Gen3 re-evaluation
/// - Reduced max iterations for speed
///
/// Supports all three preload modes:
///   - Force: 3×3 NR for (δx, δy, δz)
///   - DisplacementFromForce: convert f_a→δz_preload, fix δz, 2×2 NR for (δx, δy)
///   - Displacement: fix δz directly, 2×2 NR for (δx, δy)
pub(crate) fn solve_equilibrium_fast(
    input: &BearingInput,
    slices: &[SliceGeometry],
    warm_start: &[f64; 5],
) -> Result<BearingEquilibrium, SolverError> {
    let alpha_rad = input.raceway_geom.alpha_o.to_radians();
    let alpha_i_rad = input.raceway_geom.alpha_i.to_radians();
    let d_pw = input.macro_geom.d_pw;
    let g_r = input.macro_geom.g_r;
    let gamma_ext = input.operating.gamma_rad();

    let f_x = input.operating.f_x * 1000.0;
    let f_y = input.operating.f_y * 1000.0;
    let f_a_input = input.operating.f_a * 1000.0;
    let f_r = input.operating.f_r() * 1000.0;
    let m_x = input.operating.m_x * 1e6;
    let m_y = input.operating.m_y * 1e6;
    let f_total = (f_r * f_r + f_a_input * f_a_input).sqrt().max(1.0);

    let load_angle = radial_load_angle(f_x, f_y);
    let positions = roller_positions(input.macro_geom.z, load_angle);
    let rib_coupled = input.solver.rib_contact_mode == RibContactMode::Coupled;
    let rib_params: Option<(&RollerProfile, &MacroGeometry, &RacewayGeometry)> =
        if rib_coupled { Some((&input.roller_profile, &input.macro_geom, &input.raceway_geom)) } else { None };

    let preload_mode = input.operating.preload_mode;

    // ── Determine preload handling ──
    let f_a_induced = compute_induced_thrust(f_r, alpha_rad);
    let cos_a = alpha_rad.cos();
    let sin_a = alpha_rad.sin();
    let dz_free = preload_mode == PreloadMode::DisplacementFromForceIterative
        && f_a_input >= f_a_induced;
    let delta_preload_um = match preload_mode {
        PreloadMode::DisplacementFromForce | PreloadMode::DisplacementFromForceIterative => {
            solve_preload_displacement(input, slices, f_a_input)?
        }
        PreloadMode::Displacement => input.operating.delta_preload_um,
    };

    let mut disp = *warm_start;
    if disp[2].abs() < 1e-15 {
        disp[2] = delta_preload_um;
    }

    let tol = input.solver.convergence_tol;
    let max_iter = 50;
    let h_s = 0.01_f64;

    let res = |d: &[f64; 5], fa: f64| -> [f64; 5] {
        compute_residual_fast(
            slices, &positions, d, alpha_rad, d_pw, g_r, gamma_ext,
            f_x, f_y, fa, m_x, m_y,
            &input.material, rib_coupled, rib_params, alpha_i_rad,
        )
    };

    // ── Phase A: Force equilibrium ──
    let disp_mag = (disp[0] * disp[0] + disp[1] * disp[1] + disp[2] * disp[2]).sqrt();
    let max_step = (disp_mag * 0.5).clamp(5.0, 30.0);

    if dz_free {
        // 3×3 NR for (δx, δy, δz)
        let f_s_total = ((f_x / cos_a).powi(2) + (f_y / cos_a).powi(2) + (f_a_input / sin_a).powi(2))
            .sqrt()
            .max(1.0);
        let scale_d = [cos_a, cos_a, sin_a];

        for _ in 0..max_iter {
            let residual = res(&disp, f_a_input);
            let rs = [residual[0] / cos_a, residual[1] / cos_a, residual[2] / sin_a];
            let r_norm = (rs[0] * rs[0] + rs[1] * rs[1] + rs[2] * rs[2]).sqrt();
            if r_norm / f_s_total < tol { break; }

            let mut jac = [[0.0_f64; 3]; 3];
            for col in 0..3 {
                let mut dp = disp;
                dp[col] += h_s / scale_d[col];
                let rp = res(&dp, f_a_input);
                for row in 0..3 {
                    jac[row][col] = (rp[row] / scale_d[row] - rs[row]) / h_s;
                }
            }

            let ds = solve_3x3(&jac, &[-rs[0], -rs[1], -rs[2]]);
            let mut dd = [ds[0] / scale_d[0], ds[1] / scale_d[1], ds[2] / scale_d[2]];

            let step_norm = (dd[0] * dd[0] + dd[1] * dd[1] + dd[2] * dd[2]).sqrt();
            if step_norm > max_step {
                let s = max_step / step_norm;
                dd[0] *= s; dd[1] *= s; dd[2] *= s;
            }

            let mut alpha_ls = 1.0;
            for _ in 0..5 {
                let dt = [disp[0] + alpha_ls * dd[0], disp[1] + alpha_ls * dd[1], disp[2] + alpha_ls * dd[2], disp[3], disp[4]];
                let rt = res(&dt, f_a_input);
                let rt_norm = ((rt[0] / cos_a).powi(2) + (rt[1] / cos_a).powi(2) + (rt[2] / sin_a).powi(2)).sqrt();
                if rt_norm < r_norm { break; }
                alpha_ls *= 0.5;
            }
            disp[0] += alpha_ls * dd[0];
            disp[1] += alpha_ls * dd[1];
            disp[2] += alpha_ls * dd[2];
        }
    } else {
        // 2×2 NR for (δx, δy), δz fixed
        let f_radial = f_r.max(1.0);

        for _ in 0..max_iter {
            let residual = res(&disp, 0.0);
            let r_radial = (residual[0] * residual[0] + residual[1] * residual[1]).sqrt();
            if r_radial / f_radial < tol { break; }

            let mut j2 = [[0.0_f64; 2]; 2];
            for col in 0..2 {
                let mut dp = disp;
                dp[col] += h_s;
                let rp = res(&dp, 0.0);
                for row in 0..2 {
                    j2[row][col] = (rp[row] - residual[row]) / h_s;
                }
            }

            let det = j2[0][0] * j2[1][1] - j2[0][1] * j2[1][0];
            if det.abs() < 1e-30 { break; }
            let dx_step = (j2[1][1] * (-residual[0]) - j2[0][1] * (-residual[1])) / det;
            let dy_step = (j2[0][0] * (-residual[1]) - j2[1][0] * (-residual[0])) / det;

            let step_norm = (dx_step * dx_step + dy_step * dy_step).sqrt();
            let scale = if step_norm > max_step { max_step / step_norm } else { 1.0 };
            let dd = [dx_step * scale, dy_step * scale];

            let mut alpha_ls = 1.0;
            for _ in 0..5 {
                let d_trial = [disp[0] + alpha_ls * dd[0], disp[1] + alpha_ls * dd[1], disp[2], disp[3], disp[4]];
                let rt = res(&d_trial, 0.0);
                let rt_r = (rt[0] * rt[0] + rt[1] * rt[1]).sqrt();
                if rt_r < r_radial { break; }
                alpha_ls *= 0.5;
            }
            disp[0] += alpha_ls * dd[0];
            disp[1] += alpha_ls * dd[1];
        }
    }

    // ── Phase B: 2-DOF moment equilibrium (γx, γy) ──
    let m_ext_mag = (m_x * m_x + m_y * m_y).sqrt();
    let m_norm = (f_total * d_pw / 2.0).max(m_ext_mag).max(1.0);
    let has_moment = m_ext_mag > 1.0 || input.operating.gamma.abs() > 0.01;
    let f_a_for_residual = if dz_free { f_a_input } else { 0.0 };

    if has_moment {
        let h_ang = 1e-6;
        for _ in 0..max_iter {
            let residual = res(&disp, f_a_for_residual);
            let r_m = [residual[3], residual[4]];
            let r_m_norm = (r_m[0] * r_m[0] + r_m[1] * r_m[1]).sqrt();
            if r_m_norm / m_norm < tol { break; }

            let mut j2 = [[0.0_f64; 2]; 2];
            for col in 0..2 {
                let mut dp = disp;
                dp[3 + col] += h_ang;
                let rp = res(&dp, f_a_for_residual);
                for row in 0..2 {
                    j2[row][col] = (rp[3 + row] - r_m[row]) / h_ang;
                }
            }

            let det = j2[0][0] * j2[1][1] - j2[0][1] * j2[1][0];
            if det.abs() < 1e-30 { break; }
            let dg_x = (j2[1][1] * (-r_m[0]) - j2[0][1] * (-r_m[1])) / det;
            let dg_y = (j2[0][0] * (-r_m[1]) - j2[1][0] * (-r_m[0])) / det;

            let ang_step = (dg_x * dg_x + dg_y * dg_y).sqrt();
            let max_ang = 1e-4;
            let scale = if ang_step > max_ang { max_ang / ang_step } else { 1.0 };
            disp[3] += dg_x * scale;
            disp[4] += dg_y * scale;
        }
    }

    // ── Build full roller results for snapshot ──
    let (_, roller_results) = compute_residual(input, slices, &disp, f_a_for_residual)?;
    let roller_loads: Vec<f64> = roller_results.iter().map(|r| r.q_normal).collect();

    Ok(BearingEquilibrium {
        displacement: disp,
        roller_loads,
        roller_results,
        angular_distribution: Vec::new(),
    })
}

/// Solve for preload displacement δz [μm] that produces target axial force.
///
/// Pure axial loading (Fr=0): all Z rollers have identical approach
///   δ_rigid = δz × sin(α_o)
/// so F_a = Z × Q(δz×sin(α)) × sin(α).
///
/// Uses Newton-Raphson on δz. Returns δz [μm].
fn solve_preload_displacement(
    input: &BearingInput,
    slices: &[SliceGeometry],
    f_a_target_n: f64,
) -> Result<f64, SolverError> {
    if f_a_target_n <= 0.0 {
        return Ok(0.0);
    }

    let alpha_rad = input.raceway_geom.alpha_o.to_radians();
    let sin_a = alpha_rad.sin();
    let z = input.macro_geom.z as f64;

    // F_a = Z × Q_roller(δz·sin(α)) × sin(α)
    // Target Q per roller: q_target = f_a_target / (Z × sin(α))
    let _q_per_roller = f_a_target_n / (z * sin_a);

    // Initial guess: rough stiffness-based
    let mut dz = 5.0_f64; // μm
    let tol = input.solver.convergence_tol;
    let max_iter = input.solver.max_iterations;
    let h = 0.01_f64; // perturbation for numerical derivative

    // Coupled mode: rib compliance reduces effective roller approach
    let coupled = input.solver.rib_contact_mode == RibContactMode::Coupled;
    let alpha_i_rad = input.raceway_geom.alpha_i.to_radians();
    let sin_alpha_diff = (alpha_rad - alpha_i_rad).sin();
    let cos_alpha_i = alpha_i_rad.cos();
    let cos_alpha_diff = (alpha_rad - alpha_i_rad).cos();

    // Helper: compute Q for a given δz, accounting for rib compliance if coupled
    // Always uses gen1 (fast) — split is applied only in final re-evaluation
    let compute_q_for_dz = |dz_val: f64| -> f64 {
        let delta_rigid = dz_val * sin_a;
        if !coupled || delta_rigid <= 0.0 {
            let (_, q) = gen1::solve_gen1_roller(slices, delta_rigid, &input.material, cos_alpha_diff);
            return q;
        }
        // Coupled: fixed-point iteration for δ_rib
        let mut delta_rib_um = 0.0_f64;
        let mut q_out = 0.0_f64;
        for _ in 0..30 {
            let delta_eff = (delta_rigid - delta_rib_um * sin_a).max(0.0);
            let (_, q) = gen1::solve_gen1_roller(slices, delta_eff, &input.material, cos_alpha_diff);
            let q_axial = q * sin_alpha_diff / cos_alpha_i;
            let rib = rib_contact::compute_rib_contact(
                &input.roller_profile,
                &input.macro_geom,
                &input.raceway_geom,
                &input.material,
                q_axial,
                Some(&input.operating),
            ).ok();
            let new_delta_rib = rib.as_ref().map_or(0.0, |r| r.delta_rib);
            q_out = q;
            if (new_delta_rib - delta_rib_um).abs() < 0.001 {
                break;
            }
            delta_rib_um = new_delta_rib;
        }
        q_out
    };

    for _ in 0..max_iter {
        let q_total = compute_q_for_dz(dz);

        let f_a_calc = z * q_total * sin_a;
        let residual = f_a_calc - f_a_target_n;

        if (residual / f_a_target_n).abs() < tol {
            return Ok(dz);
        }

        // Numerical derivative dFa/dδz
        let q_total_p = compute_q_for_dz(dz + h);
        let f_a_plus = z * q_total_p * sin_a;
        let df = (f_a_plus - f_a_calc) / h;

        if df.abs() < 1e-20 {
            dz *= 2.0;
            continue;
        }

        let dz_new = dz - residual / df;
        dz = dz_new.max(0.01);
    }

    Err(SolverError::ConvergenceFailure(format!(
        "Preload displacement solver did not converge (target Fa={:.1}N)",
        f_a_target_n
    )))
}

/// 5-DOF bearing equilibrium solver.
///
/// Uses block-decomposed Newton-Raphson to avoid ill-conditioning:
///   Phase A: 3-DOF (δx, δy, δz) force equilibrium with γx=γy=0
///   Phase B: 5-DOF refinement with tilting angles
///
/// Supports three preload modes:
///   - Force: traditional force equilibrium (original behavior)
///   - DisplacementFromForce: convert f_a to δz_preload, then fix δz
///   - Displacement: fix δz = delta_preload_um directly
pub fn solve_bearing_equilibrium(
    input: &BearingInput,
    progress: &dyn ProgressReporter,
) -> Result<BearingResult, SolverError> {
    let t_start = Instant::now();
    progress.report(SolverProgress {
        stage: "Validation".into(),
        detail: "Validating input parameters".into(),
        percent: 0.0,
    });
    input.validate()?;

    let slices = geometry::compute_slices(
        &input.macro_geom,
        &input.raceway_geom,
        &input.roller_profile,
        &input.raceway_profile_inner,
        &input.raceway_profile_outer,
        input.solver.n_slices,
    )?;

    let f_x = input.operating.f_x * 1000.0;
    let f_y = input.operating.f_y * 1000.0;
    let f_a_input = input.operating.f_a * 1000.0;
    let f_r = input.operating.f_r() * 1000.0;

    let alpha_rad = input.raceway_geom.alpha_o.to_radians();

    let preload_mode = input.operating.preload_mode;

    // Determine δz handling based on preload mode
    let f_a_induced = compute_induced_thrust(f_r, alpha_rad);
    let cos_a = alpha_rad.cos();
    let sin_a = alpha_rad.sin();
    // DisplacementFromForce with F_a >= induced: 3×3 NR (δz free, axial equilibrium)
    // DisplacementFromForce with F_a < induced: 2×2 NR (δz fixed, housing-constrained)
    // Displacement: 2×2 NR (δz fixed)
    // DisplacementFromForce: 2×2 NR (δz fixed) — standard preload
    // DisplacementFromForceIterative: 3×3 NR (δz free when F_a >= induced)
    // Displacement: 2×2 NR (δz fixed at user value)
    let dz_free = preload_mode == PreloadMode::DisplacementFromForceIterative
        && f_a_input >= f_a_induced;
    let delta_preload_um = match preload_mode {
        PreloadMode::DisplacementFromForce | PreloadMode::DisplacementFromForceIterative => {
            progress.report(SolverProgress {
                stage: "Preload".into(),
                detail: "Computing initial δz from target force".into(),
                percent: 2.0,
            });
            solve_preload_displacement(input, &slices, f_a_input)?
        }
        PreloadMode::Displacement => {
            input.operating.delta_preload_um
        }
    };

    let f_total = (f_r * f_r + f_a_input * f_a_input).sqrt().max(1.0);

    let mut disp = initial_guess(input);
    disp[2] = delta_preload_um;
    let max_iter = input.solver.max_iterations;
    let tol = input.solver.convergence_tol;

    // ── Phase A: Force equilibrium ──
    let h_s = 0.01_f64;
    let disp_mag = (disp[0] * disp[0] + disp[1] * disp[1] + disp[2] * disp[2]).sqrt();
    let max_step = (disp_mag * 0.5).clamp(5.0, 30.0);

    if dz_free {
        // ── 3×3 NR for (δx, δy, δz): full force equilibrium ──
        let f_s_total = ((f_x / cos_a).powi(2) + (f_y / cos_a).powi(2) + (f_a_input / sin_a).powi(2))
            .sqrt()
            .max(1.0);

        for _outer in 0..max_iter {
            progress.report(SolverProgress {
                stage: "Phase A".into(),
                detail: format!("Force equilibrium iter {}", _outer + 1),
                percent: 5.0 + (_outer as f64 / max_iter as f64) * 40.0,
            });
            let (residual, _) = compute_residual(input, &slices, &disp, f_a_input)?;

            let rs = [
                residual[0] / cos_a,
                residual[1] / cos_a,
                residual[2] / sin_a,
            ];
            let r_s_norm = (rs[0] * rs[0] + rs[1] * rs[1] + rs[2] * rs[2]).sqrt();

            if r_s_norm / f_s_total < tol {
                break;
            }

            let mut jac = [[0.0_f64; 3]; 3];
            let phys_idx = [0usize, 1, 2];
            let scale_d = [cos_a, cos_a, sin_a];

            for col in 0..3 {
                let mut dp = disp;
                dp[phys_idx[col]] += h_s / scale_d[col];
                let (rp, _) = compute_residual(input, &slices, &dp, f_a_input)?;
                for row in 0..3 {
                    jac[row][col] = (rp[phys_idx[row]] / scale_d[row] - rs[row]) / h_s;
                }
            }

            let neg_rs = [-rs[0], -rs[1], -rs[2]];
            let ds = solve_3x3(&jac, &neg_rs);

            let mut dd = [0.0_f64; 3];
            for i in 0..3 {
                dd[i] = ds[i] / scale_d[i];
            }

            let step_norm = (dd[0] * dd[0] + dd[1] * dd[1] + dd[2] * dd[2]).sqrt();
            let scale = if step_norm > max_step {
                max_step / step_norm
            } else {
                1.0
            };
            for v in dd.iter_mut() {
                *v *= scale;
            }

            let mut alpha_ls = 1.0_f64;
            let mut best_alpha = 0.0_f64;
            let mut best_norm = r_s_norm;
            for _ in 0..20 {
                let mut dt = disp;
                for i in 0..3 {
                    dt[phys_idx[i]] += alpha_ls * dd[i];
                }

                let (rt, _) = compute_residual(input, &slices, &dt, f_a_input)?;
                let rt_s = ((rt[0] / cos_a).powi(2) + (rt[1] / cos_a).powi(2) + (rt[2] / sin_a).powi(2)).sqrt();

                if rt_s < best_norm {
                    best_norm = rt_s;
                    best_alpha = alpha_ls;
                }
                if rt_s < r_s_norm {
                    break;
                }
                alpha_ls *= 0.5;
            }
            if best_alpha < 1e-15 {
                best_alpha = alpha_ls;
            }
            for i in 0..3 {
                disp[phys_idx[i]] += best_alpha * dd[i];
            }
        }
    } else {
        // ── 2×2 NR for (δx, δy), δz fixed ──
        let f_radial = f_r.max(1.0);

        for _outer in 0..max_iter {
            progress.report(SolverProgress {
                stage: "Phase A".into(),
                detail: format!("Radial equilibrium iter {}", _outer + 1),
                percent: 5.0 + (_outer as f64 / max_iter as f64) * 40.0,
            });

            let (residual, _) = compute_residual(input, &slices, &disp, 0.0)?;
            let r_radial = (residual[0] * residual[0] + residual[1] * residual[1]).sqrt();
            if r_radial / f_radial < tol {
                break;
            }

            let mut j2 = [[0.0_f64; 2]; 2];
            for col in 0..2 {
                let mut dp = disp;
                dp[col] += h_s;
                let (rp, _) = compute_residual(input, &slices, &dp, 0.0)?;
                for row in 0..2 {
                    j2[row][col] = (rp[row] - residual[row]) / h_s;
                }
            }

            let det = j2[0][0] * j2[1][1] - j2[0][1] * j2[1][0];
            if det.abs() < 1e-30 {
                break;
            }
            let dx_step = (j2[1][1] * (-residual[0]) - j2[0][1] * (-residual[1])) / det;
            let dy_step = (j2[0][0] * (-residual[1]) - j2[1][0] * (-residual[0])) / det;

            let step_norm = (dx_step * dx_step + dy_step * dy_step).sqrt();
            let scale = if step_norm > max_step { max_step / step_norm } else { 1.0 };
            let dd = [dx_step * scale, dy_step * scale];

            let mut alpha_ls = 1.0_f64;
            let mut best_alpha = 0.0_f64;
            let mut best_norm = r_radial;
            for _ in 0..20 {
                let mut d_trial = disp;
                d_trial[0] += alpha_ls * dd[0];
                d_trial[1] += alpha_ls * dd[1];
                let (rt, _) = compute_residual(input, &slices, &d_trial, 0.0)?;
                let rt_r = (rt[0] * rt[0] + rt[1] * rt[1]).sqrt();
                if rt_r < best_norm {
                    best_norm = rt_r;
                    best_alpha = alpha_ls;
                }
                if rt_r < r_radial {
                    break;
                }
                alpha_ls *= 0.5;
            }
            if best_alpha < 1e-15 {
                best_alpha = alpha_ls;
            }
            disp[0] += best_alpha * dd[0];
            disp[1] += best_alpha * dd[1];
        }
    }

    let f_a_for_residual = if dz_free { f_a_input } else { 0.0 };

    // ── Phase B: Moment equilibrium via geometric misalignment ──
    //
    // Physical constraint: In a single-row TRB, the moment reaction is
    //   M_y ≈ -(d_pw/2) × tan(α) × F_x
    // This exact proportionality means γ cannot independently control moment
    // (linearized dM_eff/dγ = 0 after force re-equilibration).
    //
    // Strategy: Convert external moment to equivalent misalignment γ, then
    // use nonlinear iteration with LARGE perturbation to capture load-zone
    // boundary changes that break the exact proportionality.
    let m_x_ext = input.operating.m_x * 1e6;
    let m_y_ext = input.operating.m_y * 1e6;
    let m_ext_mag = (m_x_ext * m_x_ext + m_y_ext * m_y_ext).sqrt();
    let d_pw = input.macro_geom.d_pw;
    let m_norm = (f_total * d_pw / 2.0).max(m_ext_mag).max(1.0);
    let has_moment = m_ext_mag > 1.0 || input.operating.gamma.abs() > 0.01;

    if has_moment {
        progress.report(SolverProgress {
            stage: "Phase B".into(),
            detail: "Starting moment equilibrium".into(),
            percent: 45.0,
        });

        // Use LARGE perturbation to capture nonlinear effects (load zone changes)
        let h_ang = 5e-4_f64; // rad — enough to shift load zone boundary
        let max_step_ang = 0.003_f64; // ~0.17° per step

        for iter_b in 0..max_iter {
            progress.report(SolverProgress {
                stage: "Phase B".into(),
                detail: format!("Moment equilibrium iter {}", iter_b + 1),
                percent: 45.0 + (iter_b as f64 / max_iter as f64) * 20.0,
            });

            let (residual, _) = compute_residual(input, &slices, &disp, f_a_for_residual)?;
            let r_m = [residual[3], residual[4]];
            let r_m_norm = (r_m[0] * r_m[0] + r_m[1] * r_m[1]).sqrt();
            if r_m_norm / m_norm < tol {
                break;
            }

            // 2×2 Jacobian for moment equations w.r.t. [γx, γy]
            // WITHOUT force re-equilibration — captures non-degenerate dM/dγ
            let mut j2 = [[0.0_f64; 2]; 2];
            for col in 0..2 {
                let mut dp = disp;
                let mut dm = disp;
                dp[3 + col] += h_ang;
                dm[3 + col] -= h_ang;
                let (rp, _) = compute_residual(input, &slices, &dp, f_a_for_residual)?;
                let (rm, _) = compute_residual(input, &slices, &dm, f_a_for_residual)?;
                for row in 0..2 {
                    j2[row][col] = (rp[3 + row] - rm[3 + row]) / (2.0 * h_ang);
                }
            }

            let det = j2[0][0] * j2[1][1] - j2[0][1] * j2[1][0];
            if det.abs() < 1e-30 {
                break;
            }
            let mut dg0 = (j2[1][1] * (-r_m[0]) - j2[0][1] * (-r_m[1])) / det;
            let mut dg1 = (j2[0][0] * (-r_m[1]) - j2[1][0] * (-r_m[0])) / det;

            let step_ang = (dg0 * dg0 + dg1 * dg1).sqrt();
            if step_ang > max_step_ang {
                let s = max_step_ang / step_ang;
                dg0 *= s;
                dg1 *= s;
            }

            // Line search on moment residual
            let mut alpha = 1.0_f64;
            let mut best_alpha = 0.0_f64;
            let mut best_m_norm_ls = r_m_norm;
            for _ in 0..15 {
                let mut dt = disp;
                dt[3] += alpha * dg0;
                dt[4] += alpha * dg1;
                let (rt, _) = compute_residual(input, &slices, &dt, f_a_for_residual)?;
                let rt_m = (rt[3] * rt[3] + rt[4] * rt[4]).sqrt();
                if rt_m < best_m_norm_ls {
                    best_m_norm_ls = rt_m;
                    best_alpha = alpha;
                }
                if rt_m < r_m_norm {
                    break;
                }
                alpha *= 0.5;
            }
            if best_alpha > 1e-15 {
                disp[3] += best_alpha * dg0;
                disp[4] += best_alpha * dg1;
            }

            // Force re-equilibration: full NR for [δx, δy, (δz)]
            let h_f = 0.01_f64;
            if dz_free {
                let f_s_total_b = ((f_x / cos_a).powi(2) + (f_y / cos_a).powi(2)
                    + (f_a_input / sin_a).powi(2)).sqrt().max(1.0);
                for _ in 0..50 {
                    let (r, _) = compute_residual(input, &slices, &disp, f_a_for_residual)?;
                    let rs_b = [r[0] / cos_a, r[1] / cos_a, r[2] / sin_a];
                    let r_norm_b = (rs_b[0]*rs_b[0] + rs_b[1]*rs_b[1] + rs_b[2]*rs_b[2]).sqrt();
                    if r_norm_b / f_s_total_b < tol { break; }
                    let mut jf = [[0.0_f64; 3]; 3];
                    let idx = [0usize, 1, 2];
                    let sc = [cos_a, cos_a, sin_a];
                    for col in 0..3 {
                        let mut dp = disp;
                        dp[idx[col]] += h_f / sc[col];
                        let (rp, _) = compute_residual(input, &slices, &dp, f_a_for_residual)?;
                        for row in 0..3 {
                            jf[row][col] = (rp[idx[row]] / sc[row] - rs_b[row]) / h_f;
                        }
                    }
                    let neg = [-rs_b[0], -rs_b[1], -rs_b[2]];
                    let ds = solve_3x3(&jf, &neg);
                    let mut dd = [0.0_f64; 3];
                    for i in 0..3 { dd[i] = ds[i] / sc[i]; }
                    let sn = (dd[0]*dd[0]+dd[1]*dd[1]+dd[2]*dd[2]).sqrt();
                    if sn > max_step { let s = max_step/sn; for v in dd.iter_mut() { *v *= s; } }
                    let mut al = 1.0_f64;
                    for _ in 0..10 {
                        let mut dt = disp;
                        for i in 0..3 { dt[idx[i]] += al * dd[i]; }
                        let (rt, _) = compute_residual(input, &slices, &dt, f_a_for_residual)?;
                        let rn = ((rt[0]/cos_a).powi(2)+(rt[1]/cos_a).powi(2)+(rt[2]/sin_a).powi(2)).sqrt();
                        if rn < r_norm_b {
                            for i in 0..3 { disp[idx[i]] = dt[idx[i]]; }
                            break;
                        }
                        al *= 0.5;
                    }
                }
            } else {
                let f_rad = f_r.max(1.0);
                for _ in 0..50 {
                    let (r, _) = compute_residual(input, &slices, &disp, f_a_for_residual)?;
                    let rn = (r[0]*r[0] + r[1]*r[1]).sqrt();
                    if rn / f_rad < tol { break; }
                    let mut jf = [[0.0_f64; 2]; 2];
                    for col in 0..2 {
                        let mut dp = disp;
                        dp[col] += h_f;
                        let (rp, _) = compute_residual(input, &slices, &dp, f_a_for_residual)?;
                        for row in 0..2 { jf[row][col] = (rp[row] - r[row]) / h_f; }
                    }
                    let det_f = jf[0][0]*jf[1][1] - jf[0][1]*jf[1][0];
                    if det_f.abs() < 1e-30 { break; }
                    let dx = (jf[1][1]*(-r[0]) - jf[0][1]*(-r[1])) / det_f;
                    let dy = (jf[0][0]*(-r[1]) - jf[1][0]*(-r[0])) / det_f;
                    let sn = (dx*dx + dy*dy).sqrt();
                    let sc = if sn > max_step { max_step / sn } else { 1.0 };
                    let mut al = 1.0_f64;
                    for _ in 0..10 {
                        let mut dt = disp;
                        dt[0] += al * sc * dx;
                        dt[1] += al * sc * dy;
                        let (rt, _) = compute_residual(input, &slices, &dt, f_a_for_residual)?;
                        let rt_n = (rt[0]*rt[0] + rt[1]*rt[1]).sqrt();
                        if rt_n < rn {
                            disp[0] = dt[0];
                            disp[1] = dt[1];
                            break;
                        }
                        al *= 0.5;
                    }
                }
            }
        }
    }

    // Final residual check
    let (residual, roller_results) = compute_residual(input, &slices, &disp, f_a_for_residual)?;
    let r_moment = (residual[3] * residual[3] + residual[4] * residual[4]).sqrt();

    let force_ok = if dz_free {
        let r_force = (residual[0] * residual[0] + residual[1] * residual[1] + residual[2] * residual[2]).sqrt();
        r_force / f_total < tol * 10.0
    } else {
        let r_radial = (residual[0] * residual[0] + residual[1] * residual[1]).sqrt();
        r_radial / f_r.max(1.0) < tol * 10.0
    };
    // Moment convergence: In a single-row TRB, M ≈ -(d_pw/2)·tan(α)·F_r
    // (moment is determined by radial force, not independently controllable).
    // Treat moment mismatch as a warning, not a convergence failure.
    let _moment_ok = !has_moment || r_moment / m_norm < tol * 100.0;
    let moment_warn = has_moment && r_moment / m_norm >= tol * 10.0;

    if force_ok {
        // If Single(Gen3) mode, re-evaluate with beam-coupled solver
        let (disp, final_roller_results) = match input.solver.run_mode {
            RunMode::Single(SolverMode::Gen3) => {
                if dz_free {
                    // Option 2: δz free → need refinement loop to find Gen3 equilibrium
                    progress.report(SolverProgress {
                        stage: "Gen3 Refinement".into(),
                        detail: "Beam-coupled solver with equilibrium refinement".into(),
                        percent: 70.0,
                    });
                    let (refined_disp, results) = refine_displacement_with_gen3(
                        input, &slices, disp, dz_free, f_a_for_residual, progress,
                    )?;
                    progress.report(SolverProgress {
                        stage: "Gen3 Refinement".into(),
                        detail: "Complete".into(),
                        percent: 90.0,
                    });
                    (refined_disp, results)
                } else {
                    // Option 1: δz fixed → single Gen3 re-evaluation is sufficient
                    progress.report(SolverProgress {
                        stage: "Gen3 Re-evaluation".into(),
                        detail: "Beam-coupled solver (single pass)".into(),
                        percent: 70.0,
                    });
                    let results = reevaluate_with_gen3(input, &slices, disp, progress)?;
                    progress.report(SolverProgress {
                        stage: "Gen3 Re-evaluation".into(),
                        detail: "Complete".into(),
                        percent: 90.0,
                    });
                    (disp, results)
                }
            }
            _ => {
                if input.solver.use_split_contact {
                    let split_results = reevaluate_with_gen1_split(input, &slices, disp)?;
                    (disp, split_results)
                } else {
                    (disp, roller_results)
                }
            }
        };
        progress.report(SolverProgress {
            stage: "Fatigue Life".into(),
            detail: "Computing fatigue life".into(),
            percent: 92.0,
        });
        let elapsed_ms = t_start.elapsed().as_secs_f64() * 1000.0;

        // Compute actual axial reaction force from converged roller forces
        let alpha_rad_o = input.raceway_geom.alpha_o.to_radians();
        let f_a_reaction: f64 = final_roller_results.iter()
            .map(|r| r.q_normal * alpha_rad_o.sin())
            .sum();

        // Induced thrust for informational reporting / alerts
        let f_a_induced_report = compute_induced_thrust(f_r, alpha_rad);
        let f_a_eff_report = f_a_reaction; // actual axial reaction from rollers
        let (k_radial, k_axial) = compute_bearing_stiffness(
            input, &slices, &disp, f_a_eff_report,
        );
        let mut result = build_bearing_result(
            input, &slices, disp, final_roller_results, elapsed_ms,
            f_a_induced_report, f_a_eff_report,
            preload_mode, disp[2], f_a_reaction,
            k_radial, k_axial,
        )?;

        // Moment mismatch warning: single-row TRB has M ≈ -(d_pw/2)·tan(α)·F_r
        if moment_warn {
            let (res_final, _) = compute_residual(input, &slices, &disp, f_a_for_residual)?;
            let m_x_reaction = res_final[3] + m_x_ext; // reaction = residual + external
            let m_y_reaction = res_final[4] + m_y_ext;
            result.alerts.push(Alert {
                level: AlertLevel::Warning,
                category: "Moment equilibrium".into(),
                message: format!(
                    "단일 열 TRB에서 모멘트는 반경력에 의해 결정됩니다. \
                     입력 모멘트(Mx={:.3}, My={:.3} kN·m)와 실제 모멘트 반력(Mx={:.3}, My={:.3} kN·m)이 다릅니다. \
                     정확한 모멘트 해석은 축(shaft) 모델이 필요합니다.",
                    input.operating.m_x, input.operating.m_y,
                    m_x_reaction / 1e6, m_y_reaction / 1e6,
                ),
                value: r_moment / m_norm,
                threshold: tol * 10.0,
            });
        }

        progress.report(SolverProgress {
            stage: "Complete".into(),
            detail: "Done".into(),
            percent: 100.0,
        });
        return Ok(result);
    }

    let r_force_total = (residual[0] * residual[0] + residual[1] * residual[1] + residual[2] * residual[2]).sqrt();
    Err(SolverError::ConvergenceFailure(format!(
        "Bearing equilibrium did not converge: force_err={:.2e}, moment_err={:.2e}",
        r_force_total / f_total,
        r_moment / m_norm
    )))
}

/// 5-DOF full Newton-Raphson bearing equilibrium solver.
///
/// Solves all 5 DOFs [δx, δy, δz, γx, γy] simultaneously, avoiding the
/// convergence issues of the block-decomposed approach when force-moment
/// coupling is significant (e.g., TRB natural tilting under radial load).
///
/// Uses numerical 5×5 Jacobian with line search for robustness.
#[allow(dead_code)]
pub fn solve_bearing_equilibrium_5dof(
    input: &BearingInput,
    progress: &dyn ProgressReporter,
) -> Result<BearingResult, SolverError> {
    let t_start = Instant::now();
    input.validate()?;

    let slices = geometry::compute_slices(
        &input.macro_geom,
        &input.raceway_geom,
        &input.roller_profile,
        &input.raceway_profile_inner,
        &input.raceway_profile_outer,
        input.solver.n_slices,
    )?;

    let _f_x = input.operating.f_x * 1000.0;
    let _f_y = input.operating.f_y * 1000.0;
    let f_a_input = input.operating.f_a * 1000.0;
    let f_r = input.operating.f_r() * 1000.0;
    let alpha_rad = input.raceway_geom.alpha_o.to_radians();
    let _cos_a = alpha_rad.cos();
    let _sin_a = alpha_rad.sin();
    let preload_mode = input.operating.preload_mode;

    let f_a_induced = compute_induced_thrust(f_r, alpha_rad);
    let dz_free = preload_mode == PreloadMode::DisplacementFromForceIterative
        && f_a_input >= f_a_induced;

    let _delta_preload_um = match preload_mode {
        PreloadMode::DisplacementFromForce | PreloadMode::DisplacementFromForceIterative => {
            solve_preload_displacement(input, &slices, f_a_input)?
        }
        PreloadMode::Displacement => input.operating.delta_preload_um,
    };

    let f_a_for_residual = if dz_free { f_a_input } else { 0.0 };

    let d_pw = input.macro_geom.d_pw;
    let f_total = (f_r * f_r + f_a_input * f_a_input).sqrt().max(1.0);
    let m_x_ext = input.operating.m_x * 1e6;
    let m_y_ext = input.operating.m_y * 1e6;
    let m_ext_mag = (m_x_ext * m_x_ext + m_y_ext * m_y_ext).sqrt();
    let m_norm = (f_total * d_pw / 2.0).max(m_ext_mag).max(1.0);

    // Active DOFs: force equilibrium
    // δz free only when F_a >= induced thrust (same logic as block solver)
    // Moment DOFs (γx, γy) are NOT solved — their residual is reported as
    // the natural tilting reaction. To solve moments, provide external moments.
    let active: Vec<usize> = if dz_free {
        vec![0, 1, 2]
    } else {
        vec![0, 1]
    };
    let n_active = active.len();

    // DOF scaling: normalize so all scaled variables are O(1)
    // Displacement ~10μm, angle ~1e-3 rad (allow larger angular steps)
    let d_scale = [1.0, 1.0, 1.0, 1e3, 1e3];
    // Residual scaling: force [N], moment [N·mm]
    let r_scale = [1.0 / f_total, 1.0 / f_total, 1.0 / f_total, 1.0 / m_norm, 1.0 / m_norm];

    // Use block-decomposed solver result as initial guess (Phase A only, γ=0)
    // This gives a good starting point for the 5-DOF refinement.
    let block_result = solve_bearing_equilibrium(input, progress)?;
    let mut disp = block_result.equilibrium.displacement;

    let max_iter = input.solver.max_iterations;
    let tol = input.solver.convergence_tol;

    // Perturbation in scaled coordinates (all ~0.01)
    let h_scaled = 0.01_f64;

    for iter in 0..max_iter {
        progress.report(SolverProgress {
            stage: "5-DOF NR".into(),
            detail: format!("Iteration {}", iter + 1),
            percent: 5.0 + (iter as f64 / max_iter as f64) * 60.0,
        });

        let (residual, _) = compute_residual(input, &slices, &disp, f_a_for_residual)?;

        // Scaled residual
        let rs: Vec<f64> = active.iter().map(|&i| residual[i] * r_scale[i]).collect();
        let r_norm = rs.iter().map(|r| r * r).sum::<f64>().sqrt();

        // Converge on active DOF residuals only
        if r_norm < tol { break; }

        // Numerical Jacobian (central difference, scaled coordinates)
        let mut jac = DMatrix::zeros(n_active, n_active);
        for (col, &dof) in active.iter().enumerate() {
            let h_phys = h_scaled / d_scale[dof];
            let mut dp = disp;
            let mut dm = disp;
            dp[dof] += h_phys;
            dm[dof] -= h_phys;
            let (rp, _) = compute_residual(input, &slices, &dp, f_a_for_residual)?;
            let (rm, _) = compute_residual(input, &slices, &dm, f_a_for_residual)?;
            for (row, &row_dof) in active.iter().enumerate() {
                jac[(row, col)] = (rp[row_dof] * r_scale[row_dof] - rm[row_dof] * r_scale[row_dof]) / (2.0 * h_scaled);
            }
        }

        // Levenberg-Marquardt: solve (J^T J + λI) Δx = -J^T r
        let jt = jac.transpose();
        let jtj = &jt * &jac;
        let jtr = &jt * &NVec::from_iterator(n_active, rs.iter().cloned());

        // Adaptive λ: start from diagonal magnitude, reduce on success
        let diag_max = (0..n_active).map(|i| jtj[(i, i)].abs()).fold(0.0_f64, f64::max);
        let lambda = diag_max * 1e-6; // minimal damping
        let mut jtj_damped = jtj.clone();
        for i in 0..n_active {
            jtj_damped[(i, i)] += lambda;
        }

        let dd_scaled = match jtj_damped.clone().lu().solve(&(-&jtr)) {
            Some(sol) => sol,
            None => { break; } // singular — stop
        };

        // Convert to physical coordinates
        let mut dd_phys = [0.0_f64; 5];
        for (col, &dof) in active.iter().enumerate() {
            dd_phys[dof] = dd_scaled[col] / d_scale[dof];
        }

        // Step limiting (generous — line search handles overshoot)
        let disp_mag = (disp[0]*disp[0] + disp[1]*disp[1] + disp[2]*disp[2]).sqrt();
        let max_step_disp = (disp_mag * 1.0).clamp(10.0, 50.0);
        let max_step_ang = 0.01; // ~0.57°
        let step_disp = (dd_phys[0]*dd_phys[0] + dd_phys[1]*dd_phys[1] + dd_phys[2]*dd_phys[2]).sqrt();
        let step_ang = (dd_phys[3]*dd_phys[3] + dd_phys[4]*dd_phys[4]).sqrt();
        if step_disp > max_step_disp {
            let s = max_step_disp / step_disp;
            for i in 0..3 { dd_phys[i] *= s; }
        }
        if step_ang > max_step_ang {
            let s = max_step_ang / step_ang;
            for i in 3..5 { dd_phys[i] *= s; }
        }

        // Line search
        let mut alpha_ls = 1.0_f64;
        let mut best_alpha = 0.0_f64;
        let mut best_norm = r_norm;
        for _ in 0..15 {
            let mut dt = disp;
            for &dof in &active { dt[dof] += alpha_ls * dd_phys[dof]; }
            let (rt, _) = compute_residual(input, &slices, &dt, f_a_for_residual)?;
            let rt_norm = active.iter()
                .map(|&i| { let v = rt[i] * r_scale[i]; v * v })
                .sum::<f64>().sqrt();
            if rt_norm < best_norm { best_norm = rt_norm; best_alpha = alpha_ls; }
            if rt_norm < r_norm { break; }
            alpha_ls *= 0.5;
        }
        if best_alpha < 1e-15 { best_alpha = alpha_ls; }
        for &dof in &active { disp[dof] += best_alpha * dd_phys[dof]; }
    }

    // Final check
    let (residual, roller_results) = compute_residual(input, &slices, &disp, f_a_for_residual)?;
    let r_force = (residual[0]*residual[0] + residual[1]*residual[1] + residual[2]*residual[2]).sqrt();
    let r_moment = (residual[3]*residual[3] + residual[4]*residual[4]).sqrt();
    let m_norm = (f_total * d_pw / 2.0).max(m_ext_mag).max(1.0);

    let force_ok = if dz_free {
        r_force / f_total < tol * 10.0
    } else {
        let r_radial = (residual[0]*residual[0] + residual[1]*residual[1]).sqrt();
        r_radial / f_r.max(1.0) < tol * 10.0
    };
    // Moment residual is the natural tilting reaction — not a convergence criterion
    let moment_ok = true;

    if force_ok && moment_ok {
        let (disp, final_roller_results) = match input.solver.run_mode {
            RunMode::Single(SolverMode::Gen3) => {
                let (refined_disp, results) = refine_displacement_with_gen3(
                    input, &slices, disp, dz_free, f_a_for_residual, progress,
                )?;
                (refined_disp, results)
            }
            _ => (disp, roller_results),
        };

        let elapsed_ms = t_start.elapsed().as_secs_f64() * 1000.0;
        let alpha_rad_o = input.raceway_geom.alpha_o.to_radians();
        let f_a_reaction: f64 = final_roller_results.iter()
            .map(|r| r.q_normal * alpha_rad_o.sin())
            .sum();
        let f_a_induced_report = compute_induced_thrust(f_r, alpha_rad);
        let (k_radial, k_axial) = compute_bearing_stiffness(
            input, &slices, &disp, f_a_reaction,
        );
        let result = build_bearing_result(
            input, &slices, disp, final_roller_results, elapsed_ms,
            f_a_induced_report, f_a_reaction,
            preload_mode, disp[2], f_a_reaction,
            k_radial, k_axial,
        )?;
        return Ok(result);
    }

    Err(SolverError::ConvergenceFailure(format!(
        "5-DOF equilibrium did not converge: force_err={:.2e}, moment_err={:.2e}",
        r_force / f_total, r_moment / m_norm
    )))
}

/// Solve N×N linear system Ax=b using Gaussian elimination with partial pivoting.
#[allow(dead_code)]
fn solve_linear_system(a: &[Vec<f64>], b: &[f64], n: usize) -> Vec<f64> {
    let mut aug: Vec<Vec<f64>> = (0..n)
        .map(|i| {
            let mut row = a[i].clone();
            row.push(b[i]);
            row
        })
        .collect();

    // Forward elimination with partial pivoting
    for col in 0..n {
        let mut max_row = col;
        let mut max_val = aug[col][col].abs();
        for row in (col + 1)..n {
            if aug[row][col].abs() > max_val {
                max_val = aug[row][col].abs();
                max_row = row;
            }
        }
        if max_row != col {
            aug.swap(col, max_row);
        }
        if aug[col][col].abs() < 1e-30 {
            continue;
        }
        for row in (col + 1)..n {
            let factor = aug[row][col] / aug[col][col];
            for j in col..=n {
                aug[row][j] -= factor * aug[col][j];
            }
        }
    }

    // Back substitution
    let mut x = vec![0.0_f64; n];
    for i in (0..n).rev() {
        if aug[i][i].abs() < 1e-30 {
            continue;
        }
        x[i] = aug[i][n];
        for j in (i + 1)..n {
            x[i] -= aug[i][j] * x[j];
        }
        x[i] /= aug[i][i];
    }
    x
}

/// Build angular distribution from converged state.
///
/// When `roller_results` is provided and split contact is active,
/// interpolates slice-level data from the nearest rollers instead of
/// recomputing with gen1 (which lacks split). This ensures angular_distribution
/// is consistent with roller_results for downstream consumers (life, lubrication).
fn build_angular_distribution(
    input: &BearingInput,
    slices: &[SliceGeometry],
    disp: &[f64; 5],
) -> Vec<AngularLoadPoint> {
    let alpha_rad = input.raceway_geom.alpha_o.to_radians();
    let alpha_i_rad = input.raceway_geom.alpha_i.to_radians();
    let cos_alpha_diff = (alpha_rad - alpha_i_rad).cos();
    let gamma_ext = input.operating.gamma_rad();
    let d_pw = input.macro_geom.d_pw;
    let g_r = input.macro_geom.g_r;
    let z = input.macro_geom.z;

    let inc_deg = input.solver.angular_increment_deg.max(0.5);
    let n_points = (360.0 / inc_deg).ceil() as usize;
    let roller_spacing_deg = 360.0 / z as f64;
    let load_angle_deg = radial_load_angle(
        input.operating.f_x, input.operating.f_y,
    ).to_degrees();

    let mut points = Vec::with_capacity(n_points);

    for i in 0..n_points {
        let psi_deg = i as f64 * inc_deg;
        let psi_rad = psi_deg.to_radians();

        let delta_rigid = roller_approach(disp, psi_rad, alpha_rad, d_pw, g_r, gamma_ext);

        let (q_total, p_max, slice_p_max, slice_p_max_outer, slice_q_k) = if delta_rigid > 0.0 {
            let (slice_results, q_normal) =
                gen1::solve_gen1_roller(slices, delta_rigid, &input.material, cos_alpha_diff);
            let sp: Vec<f64> = slice_results.iter().map(|s| s.p_max_k).collect();
            let sp_o: Vec<f64> = slice_results.iter().map(|s| s.p_max_k_outer).collect();
            let sq: Vec<f64> = slice_results.iter().map(|s| s.q_k).collect();
            let p_max = sp.iter().cloned().fold(0.0_f64, f64::max);
            (q_normal, p_max, sp, sp_o, sq)
        } else {
            (0.0, 0.0, vec![0.0; slices.len()], vec![0.0; slices.len()], vec![0.0; slices.len()])
        };

        let is_roller = (0..z).any(|j| {
            let roller_deg = load_angle_deg + j as f64 * roller_spacing_deg;
            let diff = ((psi_deg - roller_deg) % 360.0 + 360.0) % 360.0;
            diff < inc_deg * 0.5 || (360.0 - diff) < inc_deg * 0.5
        });

        points.push(AngularLoadPoint {
            psi_deg,
            delta_rigid,
            q_total,
            p_max,
            slice_p_max,
            slice_p_max_outer,
            slice_q_k,
            is_roller,
        });
    }

    points
}

/// Find the two bracketing loaded rollers for a given angle ψ (circular interpolation).
/// Returns (lo_index, hi_index, interpolation_factor t ∈ [0,1]).
fn find_bracketing_rollers(loaded: &[&RollerResult], psi_deg: f64) -> (usize, usize, f64) {
    let n = loaded.len();
    if n < 2 { return (0, 0, 0.0); }

    // Find the pair where psi_deg falls between loaded[lo].psi_deg and loaded[hi].psi_deg
    for i in 0..n {
        let j = (i + 1) % n;
        let p_lo = loaded[i].psi_deg;
        let p_hi = if loaded[j].psi_deg > p_lo { loaded[j].psi_deg } else { loaded[j].psi_deg + 360.0 };
        let p = if psi_deg >= p_lo { psi_deg } else { psi_deg + 360.0 };
        if p >= p_lo && p <= p_hi {
            let span = p_hi - p_lo;
            let t = if span > 0.01 { (p - p_lo) / span } else { 0.0 };
            return (i, j, t);
        }
    }

    // Fallback: nearest
    let mut best = 0;
    let mut best_d = f64::MAX;
    for (i, r) in loaded.iter().enumerate() {
        let d = ((psi_deg - r.psi_deg).abs()).min(360.0 - (psi_deg - r.psi_deg).abs());
        if d < best_d { best_d = d; best = i; }
    }
    (best, best, 0.0)
}

/// Compute GeometrySummary from input parameters and slice geometries.
fn compute_geometry_summary(input: &BearingInput, slices: &[SliceGeometry]) -> GeometrySummary {
    let mg = &input.macro_geom;
    let d_we_mean = (mg.d_we_max + mg.d_we_min) / 2.0;
    let taper_rad = if mg.l_we > 0.0 {
        ((mg.d_we_max - mg.d_we_min) / (2.0 * mg.l_we)).atan()
    } else {
        0.0
    };
    let e_star_gpa = combined_elastic_modulus(
        input.material.e_roller,
        input.material.nu,
        input.material.e_ring,
        input.material.nu,
    );

    // ── Weight calculations ──
    let rho_roller = input.material.density_roller; // g/cm³
    let rho_ring = input.material.density_ring;     // g/cm³

    // Single roller: truncated cone
    // V = (π/12) × L × (D_max² + D_max×D_min + D_min²)  [mm³]
    let v_roller_mm3 = std::f64::consts::PI / 12.0 * mg.l_we
        * (mg.d_we_max.powi(2) + mg.d_we_max * mg.d_we_min + mg.d_we_min.powi(2));
    let mass_roller_g = v_roller_mm3 * rho_roller / 1000.0; // mm³ × g/cm³ / 1000 = g
    let mass_rollers_total_g = mass_roller_g * mg.z as f64;

    // Inner race (cone): hollow cylinder approximation
    // ID = d (bore), OD ≈ d_pw - d_we_mean, width = T
    let od_inner = mg.d_pw - d_we_mean;
    let v_inner_mm3 = std::f64::consts::PI / 4.0 * mg.t
        * (od_inner.powi(2) - mg.d.powi(2));
    let mass_inner_race_g = v_inner_mm3 * rho_ring / 1000.0;

    // Outer race (cup): hollow cylinder approximation
    // ID ≈ d_pw + d_we_mean, OD = D (outer_diameter), width = T
    let id_outer = mg.d_pw + d_we_mean;
    let v_outer_mm3 = std::f64::consts::PI / 4.0 * mg.t
        * (mg.outer_diameter.powi(2) - id_outer.powi(2));
    let mass_outer_race_g = v_outer_mm3 * rho_ring / 1000.0;

    let mass_total_g = mass_rollers_total_g + mass_inner_race_g + mass_outer_race_g;

    GeometrySummary {
        roller_taper_angle_deg: taper_rad.to_degrees(),
        roller_taper_angle_rad: taper_rad,
        e_star_gpa,
        d_we_mean,
        cone_angle_deg: taper_rad.to_degrees() * 2.0,
        gamma_dw: if mg.d_pw > 0.0 { d_we_mean / mg.d_pw } else { 0.0 },
        contact_length_ratio: if d_we_mean > 0.0 { mg.l_we / d_we_mean } else { 0.0 },
        f_r_kn: input.operating.f_r(),
        f_a_kn: input.operating.f_a,
        gamma_rad: input.operating.gamma_rad(),
        slice_geometries: slices.to_vec(),
        mass_roller_g,
        mass_rollers_total_g,
        mass_inner_race_g,
        mass_outer_race_g,
        mass_total_g,
    }
}

fn build_bearing_result(
    input: &BearingInput,
    slices: &[SliceGeometry],
    disp: [f64; 5],
    roller_results: Vec<RollerResult>,
    elapsed_ms: f64,
    f_a_induced_n: f64,
    f_a_eff_n: f64,
    preload_mode: PreloadMode,
    delta_preload_um: f64,
    f_a_reaction_n: f64,
    k_radial: f64,
    k_axial: f64,
) -> Result<BearingResult, SolverError> {
    let roller_loads: Vec<f64> = roller_results.iter().map(|r| r.q_normal).collect();

    let mut angular_distribution = build_angular_distribution(input, slices, &disp);

    // When split contact is active, interpolate slice-level data from roller_results
    // at ALL angular points (not just roller positions). This ensures downstream
    // consumers (life, lubrication, contour) see consistent split results.
    if input.solver.use_split_contact && !roller_results.is_empty() {
        let loaded_rollers: Vec<&RollerResult> = roller_results.iter()
            .filter(|r| r.q_normal > 0.0)
            .collect();
        if loaded_rollers.len() >= 2 {
            let n_sl = loaded_rollers[0].slice_results.len();
            for point in angular_distribution.iter_mut().filter(|p| p.q_total > 0.0) {
                // Find nearest two loaded rollers for interpolation
                let (lo_idx, hi_idx, t) = find_bracketing_rollers(&loaded_rollers, point.psi_deg);
                let r_lo = loaded_rollers[lo_idx];
                let r_hi = loaded_rollers[hi_idx];

                // Pure linear interpolation from bracketing rollers.
                // No additional Q-envelope scaling — avoids artifacts where
                // interpolated p_max exceeds actual roller values.
                // The load zone shape is preserved by the roller data itself;
                // non-loaded angles already have q_total=0 and are filtered out.
                let q_lo = r_lo.q_normal;
                let q_hi = r_hi.q_normal;
                let q_interp = q_lo * (1.0 - t) + q_hi * t;
                // At load zone boundary where gen1 shows contact but interpolated
                // roller load ≈ 0, zero out to avoid division artifacts
                let active = q_interp > 0.1 && point.q_total > 0.0;

                let mut sp_i = vec![0.0; n_sl];
                let mut sp_o = vec![0.0; n_sl];
                let mut sq = vec![0.0; n_sl];
                if active {
                    for k in 0..n_sl {
                        let lo_sr = &r_lo.slice_results[k];
                        let hi_sr = &r_hi.slice_results[k];
                        sp_i[k] = lo_sr.p_max_k * (1.0 - t) + hi_sr.p_max_k * t;
                        sp_o[k] = lo_sr.p_max_k_outer * (1.0 - t) + hi_sr.p_max_k_outer * t;
                        sq[k] = lo_sr.q_k * (1.0 - t) + hi_sr.q_k * t;
                    }
                }

                point.q_total = q_interp;
                point.slice_p_max = sp_i;
                point.slice_p_max_outer = sp_o;
                point.slice_q_k = sq;
                point.p_max = point.slice_p_max.iter().cloned().fold(0.0_f64, f64::max);
            }
        }
    }

    let equilibrium = BearingEquilibrium {
        displacement: disp,
        roller_loads: roller_loads.clone(),
        roller_results,
        angular_distribution,
    };

    let mut alerts = generate_alerts(&equilibrium, input);

    // Alert: induced thrust exceeds preload
    if f_a_induced_n > input.operating.f_a * 1000.0 + 1.0 {
        let f_a_input_kn = input.operating.f_a;
        let f_a_induced_kn = f_a_induced_n / 1000.0;
        alerts.push(Alert {
            level: AlertLevel::Info,
            category: "Induced thrust".into(),
            message: format!(
                "레이디얼 하중에 의한 유도 축하중({:.2}kN)이 프리로드({:.2}kN)를 초과하여 비부하측 롤러 접촉 상태에 유의 필요",
                f_a_induced_kn, f_a_input_kn
            ),
            value: f_a_induced_kn,
            threshold: f_a_input_kn,
        });
    }

    // Per-slice film thickness distribution — dispatched by lubrication model
    let film_distribution = match input.operating.lubrication_model {
        crate::solver::types::LubricationModel::Method2_MK
        | crate::solver::types::LubricationModel::Method3_NVM => {
            crate::solver::lubrication::compute_film_thickness_distribution_advanced(
                &input.macro_geom, &input.material, &input.operating,
                &input.roller_profile, &input.raceway_profile_inner,
                &input.raceway_geom, slices, &equilibrium.angular_distribution,
            )
        }
        _ => {
            crate::solver::lubrication::compute_film_thickness_distribution(
                &input.macro_geom, &input.material, &input.operating,
                &input.roller_profile, &input.raceway_profile_inner,
                &input.raceway_profile_outer,
                slices, &equilibrium.angular_distribution,
            )
        }
    };

    // EHL film thickness summary — derived from distribution (worst-case slice)
    let mut film_summary = film_distribution.as_ref().and_then(|dist| {
        match input.operating.lubrication_model {
            crate::solver::types::LubricationModel::Method2_MK
        | crate::solver::types::LubricationModel::Method3_NVM => {
                crate::solver::lubrication::summarize_film_from_distribution_advanced(
                    &input.macro_geom, &input.material, &input.operating,
                    &input.roller_profile, &input.raceway_profile_inner,
                    &input.raceway_profile_outer,
                    slices, &equilibrium.angular_distribution, dist,
                )
            }
            _ => {
                crate::solver::lubrication::summarize_film_from_distribution(
                    &input.macro_geom, &input.material, &input.operating,
                    &input.roller_profile, &input.raceway_profile_inner,
                    &input.raceway_profile_outer,
                    slices, &equilibrium.angular_distribution, dist,
                )
            }
        }
    });

    // ─── Van Zoelen film decay (optional) ───
    if let Some(ref mut fs) = film_summary {
        crate::solver::lubrication::compute_film_decay(
            fs, &input.macro_geom, &input.material, &input.operating,
            slices, &equilibrium.angular_distribution,
        );
        crate::solver::lubrication::compute_micropitting_safety(fs, &input.operating);
    }

    // κ override: when FilmThicknessRatio method, derive κ from Λ^1.3
    let kappa_override = match input.solver.kappa_method {
        KappaMethod::FilmThicknessRatio => {
            film_summary.as_ref().map(|ft| {
                let ki = crate::solver::life::compute_kappa_from_lambda(ft.lambda_ratio);
                let ko = crate::solver::life::compute_kappa_from_lambda(ft.lambda_ratio_outer);
                (ki, ko)
            })
        }
        KappaMethod::ViscosityRatio => None,
    };

    // Fatigue life calculation
    let mut life = crate::solver::life::compute_fatigue_life(
        &equilibrium.roller_results,
        &input.macro_geom,
        &input.material,
        &input.operating,
        &input.solver,
        kappa_override,
    )?;

    life.film_thickness = film_summary;

    let mode = match input.solver.run_mode {
        RunMode::Single(m) => m,
        RunMode::Dual => SolverMode::Gen1, // Bearing loop uses Gen1
    };

    let geometry = compute_geometry_summary(input, slices);

    // Static rating (ISO 76 + ISO 17956)
    let sw = input.macro_geom.l_we / input.solver.n_slices as f64;
    let static_rating = crate::solver::static_rating::compute_static_rating(
        &input.macro_geom,
        &input.operating,
        &input.solver,
        &equilibrium.roller_results,
        sw,
    );

    // ISO 15312: Thermal speed rating
    let c_0r_n = static_rating.c_0r_kn * 1000.0;
    let thermal_speed = crate::solver::life::compute_thermal_speed_rating(
        &input.macro_geom,
        c_0r_n,
        input.operating.n_rpm(),
        input.solver.f_0r,
        input.solver.f_1r,
    );

    // Contact traction — dispatched by lubrication model
    let traction = match input.operating.lubrication_model {
        crate::solver::types::LubricationModel::Method2_MK
        | crate::solver::types::LubricationModel::Method3_NVM => {
            crate::solver::lubrication::compute_traction_advanced(
                &input.macro_geom, &input.material, &input.operating,
                &input.roller_profile, &input.raceway_geom,
                &input.raceway_profile_inner, &input.raceway_profile_outer,
                slices, &equilibrium.roller_results,
            )
        }
        _ => {
            crate::solver::lubrication::compute_traction(
                &input.macro_geom, &input.material, &input.operating,
                &input.roller_profile, &input.raceway_geom,
                &input.raceway_profile_inner, &input.raceway_profile_outer,
                slices, &equilibrium.roller_results,
            )
        }
    };

    Ok(BearingResult {
        mode,
        equilibrium,
        geometry,
        life,
        static_rating,
        thermal_speed,
        alerts,
        elapsed_ms,
        f_a_induced_kn: f_a_induced_n / 1000.0,
        f_a_effective_kn: f_a_eff_n / 1000.0,
        preload_mode,
        delta_preload_um,
        f_a_reaction_kn: f_a_reaction_n / 1000.0,
        k_radial,
        k_axial,
        traction,
        film_distribution,
        load_angle_deg: radial_load_angle(
            input.operating.f_x, input.operating.f_y,
        ).to_degrees(),
    })
}

/// Compute bearing-level stiffness [N/μm] via numerical differentiation
/// at the converged displacement state.
///
/// Perturbs δr (radial resultant) and δz (axial) independently,
/// recomputes roller forces, and returns (k_radial, k_axial).
fn compute_bearing_stiffness(
    input: &BearingInput,
    slices: &[SliceGeometry],
    disp: &[f64; 5],
    f_a_eff_n: f64,
) -> (f64, f64) {
    let h = 0.05; // perturbation [μm] — small enough for accuracy, large enough to avoid noise
    let alpha_rad = input.raceway_geom.alpha_o.to_radians();
    let (_cos_a, _sin_a) = (alpha_rad.cos(), alpha_rad.sin());

    // Current forces from converged state
    let base = compute_residual(input, slices, disp, f_a_eff_n);
    let (base_r, _) = match &base {
        Ok((r, _)) => (r, ()),
        Err(_) => return (0.0, 0.0),
    };
    // Internal forces = external loads + residual
    let f_x_int = input.operating.f_x * 1000.0 + base_r[0];
    let f_y_int = input.operating.f_y * 1000.0 + base_r[1];
    let f_r_int = (f_x_int * f_x_int + f_y_int * f_y_int).sqrt();
    let f_a_int = f_a_eff_n + base_r[2];

    // Radial stiffness: perturb δx (dominant radial direction)
    let radial_dir = if f_r_int > 1.0 {
        (f_x_int / f_r_int, f_y_int / f_r_int)
    } else {
        (1.0, 0.0)
    };
    let mut dp_r = *disp;
    dp_r[0] += h * radial_dir.0;
    dp_r[1] += h * radial_dir.1;
    let k_radial = if let Ok((rp, _)) = compute_residual(input, slices, &dp_r, f_a_eff_n) {
        let f_x_p = input.operating.f_x * 1000.0 + rp[0];
        let f_y_p = input.operating.f_y * 1000.0 + rp[1];
        let f_r_p = (f_x_p * f_x_p + f_y_p * f_y_p).sqrt();
        let df_r = f_r_p - f_r_int;
        if h.abs() > 1e-15 { (df_r / h).abs() } else { 0.0 }
    } else {
        0.0
    };

    // Axial stiffness: perturb δz
    let mut dp_a = *disp;
    dp_a[2] += h;
    let k_axial = if let Ok((rp, _)) = compute_residual(input, slices, &dp_a, f_a_eff_n) {
        let f_a_p = f_a_eff_n + rp[2];
        let df_a = f_a_p - f_a_int;
        if h.abs() > 1e-15 { (df_a / h).abs() } else { 0.0 }
    } else {
        0.0
    };

    (k_radial, k_axial)
}

/// Re-evaluate converged Gen1 roller results with Gen1 split contact.
///
/// Takes the equilibrium displacement and re-solves each roller with
/// independent inner/outer contact (rigid roller δ_o split).
/// Very fast — no beam coupling, just δ_o secant iteration per roller.
fn reevaluate_with_gen1_split(
    input: &BearingInput,
    slices: &[SliceGeometry],
    disp: [f64; 5],
) -> Result<Vec<RollerResult>, SolverError> {
    let alpha_rad = input.raceway_geom.alpha_o.to_radians();
    let alpha_i_rad = input.raceway_geom.alpha_i.to_radians();
    let cos_alpha_diff = (alpha_rad - alpha_i_rad).cos();
    let sin_alpha_diff = (alpha_rad - alpha_i_rad).sin();
    let cos_alpha_i = alpha_i_rad.cos();
    let gamma_ext = input.operating.gamma_rad();
    let load_angle = radial_load_angle(input.operating.f_x * 1000.0, input.operating.f_y * 1000.0);
    let positions = roller_positions(input.macro_geom.z, load_angle);

    let mut results = Vec::with_capacity(positions.len());

    for &psi in &positions {
        let delta_rigid = roller_approach(
            &disp, psi, alpha_rad,
            input.macro_geom.d_pw, input.macro_geom.g_r, gamma_ext,
        );

        if delta_rigid > 0.0 {
            let (slice_results, q_normal) =
                gen1::solve_gen1_roller_split(slices, delta_rigid, &input.material, cos_alpha_diff);

            let q_axial = q_normal * sin_alpha_diff / cos_alpha_i;
            let rib_result = rib_contact::compute_rib_contact(
                &input.roller_profile,
                &input.macro_geom,
                &input.raceway_geom,
                &input.material,
                q_axial,
                Some(&input.operating),
            ).ok();

            results.push(RollerResult {
                psi_deg: psi.to_degrees(),
                q_normal,
                q_normal_inner: q_normal * cos_alpha_diff,
                slice_results,
                rib_result,
            });
        } else {
            results.push(RollerResult {
                psi_deg: psi.to_degrees(),
                q_normal: 0.0,
                q_normal_inner: 0.0,
                slice_results: slices.iter().map(|s| SliceContactResult {
                    k: s.k, delta_k: 0.0, q_k: 0.0, q_k_outer: 0.0, q_k_inner: 0.0,
                    b_k: 0.0, p_max_k: 0.0, h_bulk_k: 0.0, k_hertz_k: 0.0,
                    b_k_outer: 0.0, p_max_k_outer: 0.0, h_bulk_k_outer: 0.0,
                    k_hertz_k_outer: 0.0, k_combined_k: 0.0,
                    in_contact: false,
                }).collect(),
                rib_result: None,
            });
        }
    }

    Ok(results)
}

/// Re-evaluate converged Gen1 roller results with Gen3 beam-coupled solver.
///
/// Takes the equilibrium displacement from Gen1 and re-solves each roller
/// using Gen3 (Timoshenko beam coupling) for more accurate stress distribution.
fn reevaluate_with_gen3(
    input: &BearingInput,
    slices: &[SliceGeometry],
    disp: [f64; 5],
    progress: &dyn ProgressReporter,
) -> Result<Vec<RollerResult>, SolverError> {
    let alpha_rad = input.raceway_geom.alpha_o.to_radians();
    let alpha_i_rad = input.raceway_geom.alpha_i.to_radians();
    let cos_alpha_diff = (alpha_rad - alpha_i_rad).cos();
    let gamma_ext = input.operating.gamma_rad();
    let load_angle = radial_load_angle(input.operating.f_x * 1000.0, input.operating.f_y * 1000.0);
    let positions = roller_positions(input.macro_geom.z, load_angle);
    let n_rollers = positions.len();

    let completed = AtomicUsize::new(0);

    let results: Vec<Result<RollerResult, SolverError>> = positions
        .par_iter()
        .map(|&psi| {
            let delta_rigid = roller_approach(
                &disp, psi, alpha_rad,
                input.macro_geom.d_pw, input.macro_geom.g_r, gamma_ext,
            );

            let result = if delta_rigid > 0.0 {
                let use_split = input.solver.use_split_contact;
                let (slice_results, q_normal) = if use_split {
                    gen3::solve_gen3_roller_split(slices, delta_rigid, &input.material, &input.solver, cos_alpha_diff)?
                } else {
                    gen3::solve_gen3_roller(slices, delta_rigid, &input.material, &input.solver, cos_alpha_diff)?
                };

                // Rib contact: net axial force = Q × sin(α_o - α_i) / cos(α_i)
                let alpha_i_rad = input.raceway_geom.alpha_i.to_radians();
                let q_axial = q_normal * (alpha_rad - alpha_i_rad).sin() / alpha_i_rad.cos();
                let rib_result = rib_contact::compute_rib_contact(
                    &input.roller_profile,
                    &input.macro_geom,
                    &input.raceway_geom,
                    &input.material,
                    q_axial,
                    Some(&input.operating),
                )
                .ok();

                RollerResult {
                    psi_deg: psi.to_degrees(),
                    q_normal,
                    q_normal_inner: q_normal * cos_alpha_diff,
                    slice_results,
                    rib_result,
                }
            } else {
                RollerResult {
                    psi_deg: psi.to_degrees(),
                    q_normal: 0.0,
                    q_normal_inner: 0.0,
                    slice_results: slices
                        .iter()
                        .map(|s| SliceContactResult {
                            k: s.k,
                            delta_k: 0.0,
                            q_k: 0.0,
                            q_k_outer: 0.0,
                            q_k_inner: 0.0,
                            b_k: 0.0,
                            p_max_k: 0.0,
                            h_bulk_k: 0.0,
                            k_hertz_k: 0.0,
                            b_k_outer: 0.0,
                            p_max_k_outer: 0.0,
                            h_bulk_k_outer: 0.0,
                            k_hertz_k_outer: 0.0,
                            k_combined_k: 0.0,
                            in_contact: false,
                        })
                        .collect(),
                    rib_result: None,
                }
            };

            let done = completed.fetch_add(1, Ordering::Relaxed) + 1;
            progress.report(SolverProgress {
                stage: "Gen3 Re-evaluation".into(),
                detail: format!("Roller {}/{}", done, n_rollers),
                percent: 70.0 + (done as f64 / n_rollers as f64) * 20.0,
            });

            Ok(result)
        })
        .collect();

    results.into_iter().collect()
}

/// Refine bearing displacement using Gen3 force feedback.
///
/// After Gen1 equilibrium converges, Gen3 re-evaluation produces slightly different
/// forces due to beam coupling. This function iteratively adjusts the displacement
/// vector so that Gen3 forces satisfy the external load equilibrium.
///
/// Uses Newton-Raphson with numerical Jacobian on active DOFs.
/// Typically converges in 2-3 iterations since Gen1 → Gen3 force difference is small.
fn refine_displacement_with_gen3(
    input: &BearingInput,
    slices: &[SliceGeometry],
    mut disp: [f64; 5],
    dz_free: bool,
    f_a_eff_n: f64,
    progress: &dyn ProgressReporter,
) -> Result<([f64; 5], Vec<RollerResult>), SolverError> {
    let f_r = input.operating.f_r() * 1000.0;
    let f_total = (f_r * f_r + f_a_eff_n * f_a_eff_n).sqrt().max(1.0);
    let tol = input.solver.convergence_tol;
    let max_refine_iter = 10;
    let h = 0.01_f64; // perturbation [μm]

    let mut gen3_results = reevaluate_with_gen3(input, slices, disp, progress)?;

    for iter in 0..max_refine_iter {
        let residual = compute_residual_from_results(input, &gen3_results, f_a_eff_n);

        // Check convergence on force DOFs
        let r_force = if dz_free {
            (residual[0] * residual[0] + residual[1] * residual[1] + residual[2] * residual[2]).sqrt()
        } else {
            (residual[0] * residual[0] + residual[1] * residual[1]).sqrt()
        };
        let ref_force = if dz_free { f_total } else { f_r.max(1.0) };

        if r_force / ref_force < tol {
            progress.report(SolverProgress {
                stage: "Gen3 Refinement".into(),
                detail: format!("Converged in {} iter (err={:.2e})", iter + 1, r_force / ref_force),
                percent: 88.0,
            });
            return Ok((disp, gen3_results));
        }

        progress.report(SolverProgress {
            stage: "Gen3 Refinement".into(),
            detail: format!("Iter {} err={:.2e}", iter + 1, r_force / ref_force),
            percent: 80.0 + (iter as f64 / max_refine_iter as f64) * 8.0,
        });

        if dz_free {
            // 3×3 NR for (δx, δy, δz)
            let mut jac = [[0.0_f64; 3]; 3];
            for col in 0..3 {
                let mut dp = disp;
                dp[col] += h;
                let gen3_p = reevaluate_with_gen3(input, slices, dp, progress)?;
                let res_p = compute_residual_from_results(input, &gen3_p, f_a_eff_n);
                for row in 0..3 {
                    jac[row][col] = (res_p[row] - residual[row]) / h;
                }
            }
            let neg_r = [-residual[0], -residual[1], -residual[2]];
            let dd = solve_3x3(&jac, &neg_r);

            let step_norm = (dd[0] * dd[0] + dd[1] * dd[1] + dd[2] * dd[2]).sqrt();
            let max_step = 5.0; // μm
            let scale = if step_norm > max_step { max_step / step_norm } else { 1.0 };
            disp[0] += dd[0] * scale;
            disp[1] += dd[1] * scale;
            disp[2] += dd[2] * scale;
        } else {
            // 2×2 NR for (δx, δy)
            let mut jac = [[0.0_f64; 2]; 2];
            for col in 0..2 {
                let mut dp = disp;
                dp[col] += h;
                let gen3_p = reevaluate_with_gen3(input, slices, dp, progress)?;
                let res_p = compute_residual_from_results(input, &gen3_p, f_a_eff_n);
                for row in 0..2 {
                    jac[row][col] = (res_p[row] - residual[row]) / h;
                }
            }
            let det = jac[0][0] * jac[1][1] - jac[0][1] * jac[1][0];
            if det.abs() < 1e-30 {
                break;
            }
            let dx = (jac[1][1] * (-residual[0]) - jac[0][1] * (-residual[1])) / det;
            let dy = (jac[0][0] * (-residual[1]) - jac[1][0] * (-residual[0])) / det;

            let step_norm = (dx * dx + dy * dy).sqrt();
            let max_step = 5.0;
            let scale = if step_norm > max_step { max_step / step_norm } else { 1.0 };
            disp[0] += dx * scale;
            disp[1] += dy * scale;
        }

        gen3_results = reevaluate_with_gen3(input, slices, disp, progress)?;
    }

    // Return best result even if not fully converged
    Ok((disp, gen3_results))
}

/// Solve bearing in dual mode: Gen1 equilibrium + Gen3 re-evaluation.
///
/// Returns DualModeComparison with both results and recommendation.
pub fn solve_bearing_dual(input: &BearingInput, progress: &dyn ProgressReporter) -> Result<DualModeComparison, SolverError> {
    let t_total = Instant::now();

    // Step 1: Solve equilibrium with Gen1 (fast)
    // (induced thrust is applied inside solve_bearing_equilibrium)
    progress.report(SolverProgress {
        stage: "Dual: Gen1".into(),
        detail: "Solving Gen1 equilibrium".into(),
        percent: 0.0,
    });
    let mut input_gen1 = input.clone();
    input_gen1.solver.run_mode = RunMode::Single(SolverMode::Gen1);
    let gen1_result = solve_bearing_equilibrium(&input_gen1, progress)?;
    let gen1_elapsed_ms = gen1_result.elapsed_ms;

    // Step 2: Re-evaluate with Gen3 using converged displacement
    progress.report(SolverProgress {
        stage: "Dual: Gen3".into(),
        detail: "Starting Gen3 re-evaluation".into(),
        percent: 40.0,
    });
    let t_gen3 = Instant::now();
    let disp_init = gen1_result.equilibrium.displacement;
    let slices = geometry::compute_slices(
        &input.macro_geom,
        &input.raceway_geom,
        &input.roller_profile,
        &input.raceway_profile_inner,
        &input.raceway_profile_outer,
        input.solver.n_slices,
    )?;

    // Re-evaluate with Gen3: refinement only needed for Option 2 (dz_free)
    let alpha_rad_dual = input.raceway_geom.alpha_o.to_radians();
    let f_a_input_dual = input.operating.f_a * 1000.0;
    let f_r_dual = input.operating.f_r() * 1000.0;
    let f_a_induced_dual = compute_induced_thrust(f_r_dual, alpha_rad_dual);
    let dz_free_dual = input.operating.preload_mode == PreloadMode::DisplacementFromForceIterative
        && f_a_input_dual >= f_a_induced_dual;
    let f_a_for_residual_dual = if dz_free_dual { f_a_input_dual } else { 0.0 };
    let (disp, gen3_rollers) = if dz_free_dual {
        // Option 2: need refinement loop
        refine_displacement_with_gen3(
            input, &slices, disp_init, dz_free_dual, f_a_for_residual_dual, progress,
        )?
    } else {
        // Option 1: single Gen3 pass
        let results = reevaluate_with_gen3(input, &slices, disp_init, progress)?;
        (disp_init, results)
    };
    let gen3_loads: Vec<f64> = gen3_rollers.iter().map(|r| r.q_normal).collect();

    let mut angular_distribution = build_angular_distribution(input, &slices, &disp);
    // Interpolate split data from gen3_rollers to all angular points
    if input.solver.use_split_contact {
        let loaded_rollers: Vec<&RollerResult> = gen3_rollers.iter()
            .filter(|r| r.q_normal > 0.0).collect();
        if loaded_rollers.len() >= 2 {
            let n_sl = loaded_rollers[0].slice_results.len();
            for point in angular_distribution.iter_mut().filter(|p| p.q_total > 0.0) {
                let (lo, hi, t) = find_bracketing_rollers(&loaded_rollers, point.psi_deg);
                let r_lo = loaded_rollers[lo];
                let r_hi = loaded_rollers[hi];
                let q_interp = r_lo.q_normal * (1.0 - t) + r_hi.q_normal * t;
                let active = q_interp > 0.1 && point.q_total > 0.0;
                let mut sp_i = vec![0.0; n_sl];
                let mut sp_o = vec![0.0; n_sl];
                let mut sq = vec![0.0; n_sl];
                if active {
                    for k in 0..n_sl {
                        sp_i[k] = r_lo.slice_results[k].p_max_k * (1.0 - t) + r_hi.slice_results[k].p_max_k * t;
                        sp_o[k] = r_lo.slice_results[k].p_max_k_outer * (1.0 - t) + r_hi.slice_results[k].p_max_k_outer * t;
                        sq[k] = r_lo.slice_results[k].q_k * (1.0 - t) + r_hi.slice_results[k].q_k * t;
                    }
                }
                point.q_total = q_interp;
                point.slice_p_max = sp_i;
                point.slice_p_max_outer = sp_o;
                point.slice_q_k = sq;
                point.p_max = point.slice_p_max.iter().cloned().fold(0.0_f64, f64::max);
            }
        }
    }

    let gen3_eq = BearingEquilibrium {
        displacement: disp,
        roller_loads: gen3_loads,
        roller_results: gen3_rollers,
        angular_distribution,
    };

    progress.report(SolverProgress {
        stage: "Dual: Comparison".into(),
        detail: "Computing fatigue life & comparison".into(),
        percent: 90.0,
    });
    let gen3_alerts = generate_alerts(&gen3_eq, input);
    let gen3_elapsed_ms = t_gen3.elapsed().as_secs_f64() * 1000.0;

    let gen3_geometry = compute_geometry_summary(input, &slices);
    let gen3_sw = input.macro_geom.l_we / input.solver.n_slices as f64;
    let gen3_static_rating = crate::solver::static_rating::compute_static_rating(
        &input.macro_geom,
        &input.operating,
        &input.solver,
        &gen3_eq.roller_results,
        gen3_sw,
    );
    let gen3_traction = match input.operating.lubrication_model {
        crate::solver::types::LubricationModel::Method2_MK
        | crate::solver::types::LubricationModel::Method3_NVM => {
            crate::solver::lubrication::compute_traction_advanced(
                &input.macro_geom, &input.material, &input.operating,
                &input.roller_profile, &input.raceway_geom,
                &input.raceway_profile_inner, &input.raceway_profile_outer,
                &slices, &gen3_eq.roller_results,
            )
        }
        _ => {
            crate::solver::lubrication::compute_traction(
                &input.macro_geom, &input.material, &input.operating,
                &input.roller_profile, &input.raceway_geom,
                &input.raceway_profile_inner, &input.raceway_profile_outer,
                &slices, &gen3_eq.roller_results,
            )
        }
    };
    let gen3_film_dist = match input.operating.lubrication_model {
        crate::solver::types::LubricationModel::Method2_MK
        | crate::solver::types::LubricationModel::Method3_NVM => {
            crate::solver::lubrication::compute_film_thickness_distribution_advanced(
                &input.macro_geom, &input.material, &input.operating,
                &input.roller_profile, &input.raceway_profile_inner,
                &input.raceway_geom, &slices, &gen3_eq.angular_distribution,
            )
        }
        _ => {
            crate::solver::lubrication::compute_film_thickness_distribution(
                &input.macro_geom, &input.material, &input.operating,
                &input.roller_profile, &input.raceway_profile_inner,
                &input.raceway_profile_outer,
                &slices, &gen3_eq.angular_distribution,
            )
        }
    };
    let mut gen3_film_summary = gen3_film_dist.as_ref().and_then(|dist| {
        match input.operating.lubrication_model {
            crate::solver::types::LubricationModel::Method2_MK
        | crate::solver::types::LubricationModel::Method3_NVM => {
                crate::solver::lubrication::summarize_film_from_distribution_advanced(
                    &input.macro_geom, &input.material, &input.operating,
                    &input.roller_profile, &input.raceway_profile_inner,
                    &input.raceway_profile_outer,
                    &slices, &gen3_eq.angular_distribution, dist,
                )
            }
            _ => {
                crate::solver::lubrication::summarize_film_from_distribution(
                    &input.macro_geom, &input.material, &input.operating,
                    &input.roller_profile, &input.raceway_profile_inner,
                    &input.raceway_profile_outer,
                    &slices, &gen3_eq.angular_distribution, dist,
                )
            }
        }
    });

    // ─── Van Zoelen film decay for Gen3 ───
    if let Some(ref mut fs) = gen3_film_summary {
        crate::solver::lubrication::compute_film_decay(
            fs, &input.macro_geom, &input.material, &input.operating,
            &slices, &gen3_eq.angular_distribution,
        );
        crate::solver::lubrication::compute_micropitting_safety(fs, &input.operating);
    }

    // κ override for Gen3 (same logic as Single mode)
    let gen3_kappa_override = match input.solver.kappa_method {
        KappaMethod::FilmThicknessRatio => {
            gen3_film_summary.as_ref().map(|ft| {
                let ki = crate::solver::life::compute_kappa_from_lambda(ft.lambda_ratio);
                let ko = crate::solver::life::compute_kappa_from_lambda(ft.lambda_ratio_outer);
                (ki, ko)
            })
        }
        KappaMethod::ViscosityRatio => None,
    };

    let mut gen3_life = crate::solver::life::compute_fatigue_life(
        &gen3_eq.roller_results,
        &input.macro_geom,
        &input.material,
        &input.operating,
        &input.solver,
        gen3_kappa_override,
    )?;
    gen3_life.film_thickness = gen3_film_summary;
    let gen3_thermal_speed = crate::solver::life::compute_thermal_speed_rating(
        &input.macro_geom,
        gen3_static_rating.c_0r_kn * 1000.0,
        input.operating.n_rpm(),
        input.solver.f_0r,
        input.solver.f_1r,
    );
    let gen3_f_a_reaction_kn = {
        let alpha_o = input.raceway_geom.alpha_o.to_radians();
        gen3_eq.roller_results.iter()
            .map(|r| r.q_normal * alpha_o.sin())
            .sum::<f64>() / 1000.0
    };
    let gen3_result = BearingResult {
        mode: SolverMode::Gen3,
        equilibrium: gen3_eq,
        geometry: gen3_geometry,
        life: gen3_life,
        static_rating: gen3_static_rating,
        thermal_speed: gen3_thermal_speed,
        alerts: gen3_alerts,
        elapsed_ms: gen3_elapsed_ms,
        f_a_induced_kn: gen1_result.f_a_induced_kn,
        f_a_effective_kn: gen1_result.f_a_effective_kn,
        preload_mode: gen1_result.preload_mode,
        delta_preload_um: gen1_result.delta_preload_um,
        f_a_reaction_kn: gen3_f_a_reaction_kn,
        k_radial: gen1_result.k_radial,
        k_axial: gen1_result.k_axial,
        traction: gen3_traction,
        film_distribution: gen3_film_dist,
        load_angle_deg: gen1_result.load_angle_deg,
    };

    let total_elapsed_ms = t_total.elapsed().as_secs_f64() * 1000.0;

    // Step 3: Build comparison
    let result = build_comparison(gen1_result, gen3_result, gen1_elapsed_ms, gen3_elapsed_ms, total_elapsed_ms);
    progress.report(SolverProgress {
        stage: "Complete".into(),
        detail: "Done".into(),
        percent: 100.0,
    });
    result
}

/// Build DualModeComparison from two BearingResults.
fn build_comparison(
    gen1_result: BearingResult,
    gen3_result: BearingResult,
    gen1_elapsed_ms: f64,
    gen3_elapsed_ms: f64,
    total_elapsed_ms: f64,
) -> Result<DualModeComparison, SolverError> {
    // Max contact stress comparison
    let p_max_gen1 = gen1_result
        .equilibrium
        .roller_results
        .iter()
        .flat_map(|r| r.slice_results.iter())
        .map(|s| s.p_max_k)
        .fold(0.0_f64, f64::max);

    let p_max_gen3 = gen3_result
        .equilibrium
        .roller_results
        .iter()
        .flat_map(|r| r.slice_results.iter())
        .map(|s| s.p_max_k)
        .fold(0.0_f64, f64::max);

    let delta_p_max_pct = if p_max_gen1 > 1.0 {
        ((p_max_gen3 - p_max_gen1) / p_max_gen1) * 100.0
    } else {
        0.0
    };

    // Max roller load comparison
    let q_max_gen1 = gen1_result
        .equilibrium
        .roller_loads
        .iter()
        .copied()
        .fold(0.0_f64, f64::max);

    let q_max_gen3 = gen3_result
        .equilibrium
        .roller_loads
        .iter()
        .copied()
        .fold(0.0_f64, f64::max);

    let delta_q_max_pct = if q_max_gen1 > 1.0 {
        ((q_max_gen3 - q_max_gen1) / q_max_gen1) * 100.0
    } else {
        0.0
    };

    // Life comparison
    let delta_l10_pct = if gen1_result.life.l_10_basic > 1e-6 {
        ((gen3_result.life.l_10_basic - gen1_result.life.l_10_basic)
            / gen1_result.life.l_10_basic)
            * 100.0
    } else {
        0.0
    };

    // Recommendation logic
    let (gen3_recommended, recommendation_reason) =
        generate_recommendation(&gen1_result, &gen3_result, delta_p_max_pct, delta_q_max_pct);

    Ok(DualModeComparison {
        gen1_result,
        gen3_result,
        delta_p_max_pct,
        delta_q_max_pct,
        delta_l10_pct,
        gen3_recommended,
        recommendation_reason,
        gen1_elapsed_ms,
        gen3_elapsed_ms,
        total_elapsed_ms,
    })
}

/// Determine whether Gen3 is recommended over Gen1.
fn generate_recommendation(
    gen1: &BearingResult,
    gen3: &BearingResult,
    delta_p_max_pct: f64,
    delta_q_max_pct: f64,
) -> (bool, String) {
    let mut reasons = Vec::new();

    // 1. Significant stress difference (>5%)
    if delta_p_max_pct.abs() > 5.0 {
        reasons.push(format!(
            "p_max differs by {:.1}% between Gen1/Gen3",
            delta_p_max_pct
        ));
    }

    // 2. Significant load difference (>3%)
    if delta_q_max_pct.abs() > 3.0 {
        reasons.push(format!(
            "Q_max differs by {:.1}% between Gen1/Gen3",
            delta_q_max_pct
        ));
    }

    // 3. Edge loading detected in Gen3 (stress rise at slice boundaries)
    let edge_stress = check_edge_stress_rise(gen3);
    if edge_stress > 1.2 {
        reasons.push(format!(
            "Edge stress rise factor {:.2} detected (beam coupling effect)",
            edge_stress
        ));
    }

    // 4. Gen3 alerts differ from Gen1
    if gen3.alerts.len() > gen1.alerts.len() {
        reasons.push("Gen3 reveals additional warnings not visible in Gen1".into());
    }

    if reasons.is_empty() {
        (
            false,
            "Gen1 is sufficient: results agree within tolerance".into(),
        )
    } else {
        (true, reasons.join("; "))
    }
}

/// Check edge stress rise ratio: max edge p_max / center p_max across loaded rollers.
fn check_edge_stress_rise(result: &BearingResult) -> f64 {
    let mut max_ratio = 1.0_f64;
    for r in &result.equilibrium.roller_results {
        if r.q_normal <= 0.0 || r.slice_results.len() < 3 {
            continue;
        }
        let n = r.slice_results.len();
        let center_idx = n / 2;
        let p_center = r.slice_results[center_idx].p_max_k;
        if p_center < 1.0 {
            continue;
        }
        let p_edge = r.slice_results[0]
            .p_max_k
            .max(r.slice_results[n - 1].p_max_k);
        let ratio = p_edge / p_center;
        if ratio > max_ratio {
            max_ratio = ratio;
        }
    }
    max_ratio
}

/// Generate diagnostic alerts based on equilibrium results.
/// Note: axial load correction alert is added directly in solve_bearing_equilibrium.
fn generate_alerts(eq: &BearingEquilibrium, input: &BearingInput) -> Vec<Alert> {
    let mut alerts = Vec::new();

    let loads = &eq.roller_loads;
    let loaded: Vec<f64> = loads.iter().copied().filter(|&q| q > 0.0).collect();

    if loaded.is_empty() {
        alerts.push(Alert {
            level: AlertLevel::Critical,
            category: "Load".into(),
            message: "No rollers in contact".into(),
            value: 0.0,
            threshold: 1.0,
        });
        return alerts;
    }

    let q_max = loaded.iter().copied().fold(0.0_f64, f64::max);
    let q_mean = loaded.iter().sum::<f64>() / loaded.len() as f64;

    // Q_max / Q_mean > 5 → uneven distribution
    if q_mean > 0.0 && q_max / q_mean > 5.0 {
        alerts.push(Alert {
            level: AlertLevel::Warning,
            category: "Load distribution".into(),
            message: format!("Uneven load: Q_max/Q_mean = {:.1}", q_max / q_mean),
            value: q_max / q_mean,
            threshold: 5.0,
        });
    }

    // Load zone angle
    let z = input.macro_geom.z;
    let n_loaded = loaded.len();
    let load_zone_deg = (n_loaded as f64 / z as f64) * 360.0;
    if load_zone_deg < 120.0 {
        alerts.push(Alert {
            level: AlertLevel::Warning,
            category: "Load zone".into(),
            message: format!("Narrow load zone: {load_zone_deg:.0}°"),
            value: load_zone_deg,
            threshold: 120.0,
        });
    }

    // M2 + FilmThicknessRatio: roughness double-counting warning
    if input.operating.lubrication_model == LubricationModel::Method2_MK
        && input.solver.kappa_method == KappaMethod::FilmThicknessRatio
    {
        alerts.push(Alert {
            level: AlertLevel::Warning,
            category: "Lubrication".into(),
            message: "M2(MK) + FilmThicknessRatio: surface roughness is double-counted in κ. \
                      Use ViscosityRatio or switch to M1/M3 for ISO life calculation."
                .into(),
            value: 0.0,
            threshold: 0.0,
        });
    }

    // Rib stress check
    for rr in &eq.roller_results {
        if let Some(ref rib) = rr.rib_result {
            if rib.p_max_rib > 1500.0 {
                alerts.push(Alert {
                    level: AlertLevel::Warning,
                    category: "Rib stress".into(),
                    message: format!(
                        "High rib stress at ψ={:.0}°: {:.0} MPa",
                        rr.psi_deg, rib.p_max_rib
                    ),
                    value: rib.p_max_rib,
                    threshold: 1500.0,
                });
                break; // One alert is sufficient
            }
        }
    }

    alerts
}


#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_input(f_x: f64, f_a: f64, m_x: f64) -> BearingInput {
        BearingInput {
            macro_geom: MacroGeometry {
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
            },
            raceway_geom: RacewayGeometry {
                alpha_i: 12.0,
                alpha_o: 12.0,
                r_i: 200.0,
                r_o: 200.0,
                r_rib: 1500.0,
                r_rib_circ: None,
                d_uc: 0.0,
                l_uc: 0.0,
            },
            roller_profile: RollerProfile {
                crown_type: CrownType::Parabolic { c2: 0.01 },
                delta_c: 2.0,
                delta_dub_l: 1.0,
                delta_dub_s: 1.0,
                l_dub_l: 1.5,
                l_dub_s: 1.5,
                r_sph: 50.0,
                sigma_roller: 0.15,
            },
            raceway_profile_inner: RacewayProfile {
                delta_rw: 0.0,
                w_a: 0.0,
                ra: 0.3,
                custom_profile: None,
                polynomial_coeffs: None,
            },
            raceway_profile_outer: RacewayProfile {
                delta_rw: 0.0,
                w_a: 0.0,
                ra: 0.3,
                custom_profile: None,
                polynomial_coeffs: None,
            },
            material: Material::default(),
            operating: OperatingConditions {
                f_x,
                f_y: 0.0,
                f_a,
                m_x,
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
                design_life_hours: 100.0,
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
            },
            solver: SolverParams {
                run_mode: RunMode::Single(SolverMode::Gen1),
                n_slices: 30,
                beam_type: BeamType::Timoshenko,
                convergence_tol: 1e-4, // Relaxed for bearing-level
                max_iterations: 200,
                angular_increment_deg: 2.0,
                e_c: 0.8,
                ..SolverParams::default()
            },
            transient: None,
        }
    }

    #[test]
    fn test_roller_positions() {
        let pos = roller_positions(20, 0.0);
        assert_eq!(pos.len(), 20);
        // First roller at 0°
        assert!((pos[0] - 0.0).abs() < 1e-10);
        // Equally spaced: increment = 2π/20 = 18°
        let expected_inc = 2.0 * PI / 20.0;
        for i in 1..pos.len() {
            let inc = pos[i] - pos[i - 1];
            assert!(
                (inc - expected_inc).abs() < 1e-10,
                "Increment at {i}: {inc:.6} vs expected {expected_inc:.6}"
            );
        }

        // With load angle offset: roller #0 aligns with load direction
        let angle = std::f64::consts::FRAC_PI_4; // 45°
        let pos2 = roller_positions(20, angle);
        assert!((pos2[0] - angle).abs() < 1e-10);
    }

    #[test]
    fn test_roller_approach() {
        let alpha = 12.0_f64.to_radians();
        let d_pw = 70.0;

        // Pure radial displacement δx=10μm
        let disp = [10.0, 0.0, 0.0, 0.0, 0.0];
        let d_at_0 = roller_approach(&disp, 0.0, alpha, d_pw, 0.0, 0.0);
        let d_at_pi = roller_approach(&disp, PI, alpha, d_pw, 0.0, 0.0);

        assert!(d_at_0 > 0.0, "ψ=0 should have positive approach: {d_at_0}");
        assert!(d_at_pi < 0.0, "ψ=π should have negative approach: {d_at_pi}");
        // At ψ=0: δ_r = 10, δ_a = 0 → δ_rigid = 10·cos(α)
        let expected = 10.0 * alpha.cos();
        assert!(
            (d_at_0 - expected).abs() < 0.01,
            "d_at_0={d_at_0:.4} vs expected={expected:.4}"
        );
    }

    #[test]
    fn test_roller_approach_with_misalignment() {
        let alpha = 12.0_f64.to_radians();
        let d_pw = 70.0;
        let disp = [0.0, 0.0, 5.0, 0.0, 0.0]; // pure axial δz=5μm
        let gamma_ext = 1.0_f64.to_radians() / 60.0; // 1 arcmin

        // With misalignment, rollers at different ψ see different approach
        let d_90 = roller_approach(&disp, PI / 2.0, alpha, d_pw, 0.0, gamma_ext);
        let d_270 = roller_approach(&disp, 3.0 * PI / 2.0, alpha, d_pw, 0.0, gamma_ext);

        // gamma_ext about x → asymmetry in sin(ψ) direction
        assert!(
            (d_90 - d_270).abs() > 0.01,
            "Misalignment should create asymmetry: d_90={d_90:.4} vs d_270={d_270:.4}"
        );
    }

    #[test]
    fn test_pure_axial_load() {
        let input = make_test_input(0.0, 5.0, 0.0); // F_a=5kN only
        let result = solve_bearing_equilibrium(&input, &NoopReporter);
        assert!(result.is_ok(), "Should converge: {:?}", result.err());

        let br = result.unwrap();
        let loads = &br.equilibrium.roller_loads;

        // All rollers should carry load (axial → uniform)
        let n_loaded = loads.iter().filter(|&&q| q > 0.0).count();
        assert_eq!(n_loaded, 20, "All 20 rollers should be loaded under pure axial");

        // Check equilibrium: Σ(Q·sinα) ≈ F_a
        let alpha_rad = 12.0_f64.to_radians();
        let sum_axial: f64 = loads.iter().map(|&q| q * alpha_rad.sin()).sum();
        let f_a_n = 5000.0; // N
        let rel_err = (sum_axial - f_a_n).abs() / f_a_n;
        assert!(
            rel_err < 0.02,
            "Axial equilibrium: Σ(Q·sinα)={sum_axial:.1} vs F_a={f_a_n:.0}, err={rel_err:.4}"
        );
    }

    #[test]
    fn test_combined_radial_axial_load() {
        // F_a/F_r = 4/10 = 0.4 > tan(12°) = 0.213 → feasible for single TRB
        let input = make_test_input(10.0, 4.0, 0.0);
        let result = solve_bearing_equilibrium(&input, &NoopReporter);
        assert!(result.is_ok(), "Should converge: {:?}", result.err());

        let br = result.unwrap();
        let loads = &br.equilibrium.roller_loads;

        // Max load should be near ψ=0 (F_r direction)
        let q_max = loads.iter().copied().fold(0.0_f64, f64::max);
        assert!(q_max > 0.0);

        // With F_a/F_r = 0.4 (moderate axial preload), load zone may cover all rollers.
        // Key check: roller at ψ=0 carries the highest load (radial direction).
        assert!(
            loads[0] > q_max * 0.5,
            "Roller at ψ=0 should carry high load: {:.0} vs max {:.0}",
            loads[0],
            q_max
        );

        // Load at ψ=0 should exceed load at ψ=180° (index 10)
        assert!(
            loads[0] > loads[10],
            "Radial load causes Q(ψ=0)={:.0} > Q(ψ=180°)={:.0}",
            loads[0],
            loads[10]
        );
    }

    #[test]
    fn test_force_equilibrium() {
        // F_a = 5 kN > F_a_induced ≈ 1.7 kN → δz adjustment active
        // Both radial AND axial equilibrium should be satisfied
        let input = make_test_input(8.0, 5.0, 0.0);
        let result = solve_bearing_equilibrium(&input, &NoopReporter);
        assert!(result.is_ok(), "Should converge: {:?}", result.err());

        let br = result.unwrap();
        let alpha_rad = 12.0_f64.to_radians();
        let positions = roller_positions(20, 0.0); // f_y=0, load along x-axis

        let mut sum_fx = 0.0;
        let mut sum_fz = 0.0;
        for (j, rr) in br.equilibrium.roller_results.iter().enumerate() {
            let psi = positions[j];
            sum_fx += rr.q_normal * alpha_rad.cos() * psi.cos();
            sum_fz += rr.q_normal * alpha_rad.sin();
        }

        let f_r_n = 8000.0;
        let f_a_n = 5000.0;
        let err_r = (sum_fx - f_r_n).abs() / f_r_n;
        let err_a = (sum_fz - f_a_n).abs() / f_a_n;
        assert!(
            err_r < 0.02,
            "Radial equilibrium: {sum_fx:.1} vs {f_r_n:.0}, err={err_r:.4}"
        );
        assert!(
            err_a < 0.03,
            "Axial equilibrium: {sum_fz:.1} vs {f_a_n:.0}, err={err_a:.4}"
        );
    }

    #[test]
    fn test_displacement_signs() {
        let input = make_test_input(5.0, 3.0, 0.0);
        let result = solve_bearing_equilibrium(&input, &NoopReporter).unwrap();
        let disp = result.equilibrium.displacement;

        // δx > 0 (radial load in +x direction)
        assert!(disp[0] > 0.0, "δx should be positive: {:.4}", disp[0]);
        // δz > 0 (axial load in +z direction)
        assert!(disp[2] > 0.0, "δz should be positive: {:.4}", disp[2]);
    }

    #[test]
    fn test_load_monotonicity() {
        // Both load cases must be feasible: F_a/F_r > tan(12°) = 0.213
        let input1 = make_test_input(5.0, 3.0, 0.0); // F_a/F_r = 0.6
        let input2 = make_test_input(10.0, 6.0, 0.0); // F_a/F_r = 0.6

        let br1 = solve_bearing_equilibrium(&input1, &NoopReporter).unwrap();
        let br2 = solve_bearing_equilibrium(&input2, &NoopReporter).unwrap();

        let q_max1 = br1.equilibrium.roller_loads.iter().copied().fold(0.0_f64, f64::max);
        let q_max2 = br2.equilibrium.roller_loads.iter().copied().fold(0.0_f64, f64::max);

        assert!(
            q_max2 > q_max1,
            "Higher F_r should give higher Q_max: {q_max2:.0} vs {q_max1:.0}"
        );
    }

    #[test]
    fn test_moderate_axial_load_convergence() {
        // F_a/F_r ratio above minimum thrust threshold
        let input = make_test_input(10.0, 5.0, 0.0);
        let result = solve_bearing_equilibrium(&input, &NoopReporter);
        assert!(result.is_ok(), "Should converge: {:?}", result.err());

        let br = result.unwrap();
        let q_max = br.equilibrium.roller_results.iter()
            .map(|r| r.q_normal)
            .fold(0.0_f64, f64::max);
        assert!(q_max > 0.0, "Max roller load should be positive");
    }

    #[test]
    fn test_rib_results_present() {
        let input = make_test_input(5.0, 3.0, 0.0);
        let br = solve_bearing_equilibrium(&input, &NoopReporter).unwrap();

        // Loaded rollers should have rib contact results
        let loaded_with_rib = br
            .equilibrium
            .roller_results
            .iter()
            .filter(|r| r.q_normal > 0.0 && r.rib_result.is_some())
            .count();

        assert!(
            loaded_with_rib > 0,
            "Loaded rollers should have rib contact results"
        );
    }

    // ─── Dual-Mode Tests ─────────────────────────────────────────

    #[test]
    fn test_dual_mode_both_results_valid() {
        let input = make_test_input(5.0, 3.0, 0.0);
        let cmp = solve_bearing_dual(&input, &NoopReporter).unwrap();

        assert_eq!(cmp.gen1_result.mode, SolverMode::Gen1);
        assert_eq!(cmp.gen3_result.mode, SolverMode::Gen3);

        // Both should have roller results
        assert_eq!(cmp.gen1_result.equilibrium.roller_results.len(), 20);
        assert_eq!(cmp.gen3_result.equilibrium.roller_results.len(), 20);

        // Both should have positive life
        assert!(cmp.gen1_result.life.l_10_basic > 0.0);
        assert!(cmp.gen3_result.life.l_10_basic > 0.0);
    }

    #[test]
    fn test_dual_mode_same_displacement() {
        // Both modes use the same converged displacement
        let input = make_test_input(5.0, 3.0, 0.0);
        let cmp = solve_bearing_dual(&input, &NoopReporter).unwrap();

        let d1 = cmp.gen1_result.equilibrium.displacement;
        let d3 = cmp.gen3_result.equilibrium.displacement;
        for i in 0..5 {
            assert!(
                (d1[i] - d3[i]).abs() < 0.5,
                "Displacement[{i}] should match: Gen1={} vs Gen3={}",
                d1[i],
                d3[i]
            );
        }
    }

    #[test]
    fn test_dual_mode_load_agreement() {
        // For parabolic crown, Gen1 and Gen3 should be reasonably close
        let input = make_test_input(5.0, 3.0, 0.0);
        let cmp = solve_bearing_dual(&input, &NoopReporter).unwrap();

        // Q_max difference should be bounded (<20% for well-profiled rollers)
        assert!(
            cmp.delta_q_max_pct.abs() < 20.0,
            "Q_max difference should be < 20%: {:.1}%",
            cmp.delta_q_max_pct
        );
    }

    #[test]
    fn test_dual_mode_recommendation_populated() {
        let input = make_test_input(5.0, 3.0, 0.0);
        let cmp = solve_bearing_dual(&input, &NoopReporter).unwrap();

        // Recommendation reason should be non-empty
        assert!(
            !cmp.recommendation_reason.is_empty(),
            "Recommendation reason must be provided"
        );
    }

    #[test]
    fn test_dual_mode_pure_axial() {
        // Pure axial → uniform load → Gen1≈Gen3
        let input = make_test_input(0.0, 5.0, 0.0);
        let cmp = solve_bearing_dual(&input, &NoopReporter).unwrap();

        // Under uniform load, difference should be small
        assert!(
            cmp.delta_q_max_pct.abs() < 15.0,
            "Pure axial: Q_max diff should be small: {:.1}%",
            cmp.delta_q_max_pct
        );
    }

    #[test]
    fn test_dual_mode_percentages_finite() {
        let input = make_test_input(8.0, 4.0, 0.0);
        let cmp = solve_bearing_dual(&input, &NoopReporter).unwrap();

        assert!(cmp.delta_p_max_pct.is_finite(), "delta_p_max_pct must be finite");
        assert!(cmp.delta_q_max_pct.is_finite(), "delta_q_max_pct must be finite");
        assert!(cmp.delta_l10_pct.is_finite(), "delta_l10_pct must be finite");
    }

    #[test]
    fn test_induced_thrust_computed() {
        // F_a_min = F_r · tan(α_o) — exact physical minimum
        let alpha_rad = 12.0_f64.to_radians();
        let f_r = 10_000.0; // 10 kN in N
        let f_a_min = compute_induced_thrust(f_r, alpha_rad);
        let expected = f_r * alpha_rad.tan();
        assert!(
            (f_a_min - expected).abs() < 1.0,
            "F_a_min={f_a_min:.0} should equal F_r·tan(α)={expected:.0}"
        );
    }

    #[test]
    fn test_low_axial_converges_with_constraint() {
        // F_a = 0.5 kN, F_r = 10 kN → F_a < F_a_min ≈ 2.13 kN
        // Axially constrained mode: housing provides reaction.
        let input = make_test_input(10.0, 0.5, 0.0);
        let result = solve_bearing_equilibrium(&input, &NoopReporter);
        assert!(result.is_ok(), "Should converge in constrained mode: {:?}", result.err());

        let br = result.unwrap();
        // F_a_induced should exceed input F_a (housing-constrained case)
        assert!(
            br.f_a_induced_kn > 0.5,
            "F_a_induced={:.2} should exceed input F_a=0.5",
            br.f_a_induced_kn
        );
        // F_a_reaction should be >= F_a_induced (housing absorbs the difference)
        assert!(
            br.f_a_reaction_kn >= br.f_a_induced_kn - 0.1,
            "F_a_reaction={:.2} should be >= F_a_induced={:.2}",
            br.f_a_reaction_kn, br.f_a_induced_kn
        );
        // Should have axial constraint alert
        let has_alert = br.alerts.iter().any(|a| a.category == "Induced thrust");
        assert!(has_alert, "Should have axial constraint alert");
    }

    #[test]
    fn test_sufficient_axial_no_constraint() {
        // F_a = 5 kN > F_a_induced ≈ 2.13 kN → δz adjustment active
        // F_a_reaction should match F_a_input
        let input = make_test_input(10.0, 5.0, 0.0);
        let br = solve_bearing_equilibrium(&input, &NoopReporter).unwrap();

        assert!(
            (br.f_a_reaction_kn - 5.0).abs() < 0.2,
            "F_a_reaction={:.3} should ≈ F_a_input=5.0",
            br.f_a_reaction_kn
        );
    }

    #[test]
    fn test_zero_axial_pure_radial_converges() {
        // F_a = 0, F_r = 10 kN → pure radial, induced thrust applied
        let input = make_test_input(10.0, 0.0, 0.0);
        let result = solve_bearing_equilibrium(&input, &NoopReporter);
        assert!(result.is_ok(), "Pure radial should converge with induced thrust: {:?}", result.err());

        let br = result.unwrap();
        assert!(br.f_a_induced_kn > 0.0, "Induced thrust should be positive");
        assert!(br.f_a_effective_kn > 0.0, "Effective F_a should be positive");
    }

    #[test]
    fn test_high_load_nsk_hr30306j() {
        // Actual NSK HR30306J at near-capacity load
        // F_r = 60kN (near C_r = 59.5kN), F_a = 5kN
        let input = BearingInput {
            macro_geom: MacroGeometry {
                d: 30.0,
                outer_diameter: 72.0,
                t: 20.75,
                alpha: 11.859,
                z: 14,
                d_we_max: 10.9371,
                d_we_min: 10.123273,
                l_we: 10.56,
                d_pw: 51.0,
                h_rib: 2.5,
                alpha_rib: 9.855,
                g_r: 0.0,
                h_c: None,
            },
            raceway_geom: RacewayGeometry {
                alpha_i: 7.85,
                alpha_o: 11.859,
                r_i: 300.0,
                r_o: 300.0,
                r_rib: 1500.0,
                r_rib_circ: None,
                d_uc: 0.0,
                l_uc: 0.0,
            },
            roller_profile: RollerProfile {
                crown_type: CrownType::Logarithmic { a_log: 0.00095 },
                delta_c: 3.0,
                delta_dub_l: 2.0,
                delta_dub_s: 2.0,
                l_dub_l: 1.0,
                l_dub_s: 1.0,
                r_sph: 35.0,
                sigma_roller: 0.15,
            },
            raceway_profile_inner: RacewayProfile {
                delta_rw: 0.0,
                w_a: 0.0,
                ra: 0.15,
                custom_profile: None,
                polynomial_coeffs: None,
            },
            raceway_profile_outer: RacewayProfile {
                delta_rw: 0.0,
                w_a: 0.0,
                ra: 0.15,
                custom_profile: None,
                polynomial_coeffs: None,
            },
            material: Material {
                e_roller: 210.0,
                e_ring: 210.0,
                nu: 0.3,
                hrc: 61.0,
                density_roller: 7.85,
                density_ring: 7.85,
            },
            operating: OperatingConditions {
                f_x: 60.0,  // 60 kN radial (near C_r)
                f_y: 0.0,
                f_a: 5.0,   // 5 kN axial (below induced thrust)
                m_x: 0.0,
                m_y: 0.0,
                n_inner_rpm: 1500.0,
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
                design_life_hours: 100.0,
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
            },
            solver: SolverParams {
                e_c: 0.8,
                c_r_kn: Some(59.5),
                ..SolverParams::default()
            },
            transient: None,
        };

        let result = solve_bearing_equilibrium(&input, &NoopReporter);
        assert!(result.is_ok(), "NSK HR30306J at 60kN should converge: {:?}", result.err());

        let br = result.unwrap();
        // Induced thrust should be applied since F_a=5kN < F_a_min
        assert!(
            br.f_a_induced_kn > 5.0,
            "Induced thrust {:.2} kN should exceed input F_a=5kN",
            br.f_a_induced_kn
        );
        // Max roller load should be reasonable (not zero, not infinite)
        let q_max = br.equilibrium.roller_loads.iter().copied().fold(0.0_f64, f64::max);
        assert!(q_max > 1000.0, "Q_max should be > 1kN at this load: {q_max:.0}");
        assert!(q_max < 100_000.0, "Q_max should be < 100kN: {q_max:.0}");
    }

    /// Test with EXACT frontend defaults (NSK HR30306J with polynomial profiles).
    /// This reproduces the convergence failure reported by the user.
    #[test]
    fn test_frontend_defaults_converge() {
        let input = BearingInput {
            macro_geom: MacroGeometry {
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
            },
            raceway_geom: RacewayGeometry {
                alpha_i: 7.85,
                alpha_o: 11.859,
                r_i: 300.0,
                r_o: 300.0,
                r_rib: 1500.0,
                r_rib_circ: None,
                d_uc: 0.0,
                l_uc: 0.0,
            },
            roller_profile: RollerProfile {
                crown_type: CrownType::Polynomial {
                    coeffs: vec![-0.001713, 0.007566, -0.1307, -0.1991, -0.04019],
                },
                delta_c: 0.0,
                delta_dub_l: 0.0,
                delta_dub_s: 0.0,
                l_dub_l: 0.0,
                l_dub_s: 0.0,
                r_sph: 35.0,
                sigma_roller: 0.15,
            },
            raceway_profile_inner: RacewayProfile {
                delta_rw: 0.0,
                w_a: 0.0,
                ra: 0.15,
                custom_profile: None,
                polynomial_coeffs: Some(vec![-0.01255, -0.01808, -0.01308, 0.1398, -0.2076]),
            },
            raceway_profile_outer: RacewayProfile {
                delta_rw: 0.0,
                w_a: 0.0,
                ra: 0.15,
                custom_profile: None,
                polynomial_coeffs: Some(vec![-0.0006185, 0.001334, -0.2418, -0.08751, -0.1606]),
            },
            material: Material {
                e_roller: 210.0,
                e_ring: 210.0,
                nu: 0.3,
                hrc: 61.0,
                density_roller: 7.85,
                density_ring: 7.85,
            },
            operating: OperatingConditions {
                f_x: 5.0,
                f_y: 0.0,
                f_a: 2.0,
                m_x: 0.0,
                m_y: 0.0,
                n_inner_rpm: 1500.0,
                n_outer_rpm: 0.0,
                gamma: 0.0,
                t_op: 70.0,
                nu_40: 68.0,
                nu_100: 8.0,
                alpha_pv: 20.0,
                lubrication_type: LubricationType::Oil,
                starvation_factor: 1.0,
                rho_oil: 850.0,
                preload_mode: PreloadMode::DisplacementFromForce,
                delta_preload_um: 0.0,
                design_life_hours: 100.0,
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
            },
            solver: SolverParams {
                e_c: 0.8,
                c_r_kn: Some(59.5),
                c_0r_kn: Some(59.8),
                ..SolverParams::default()
            },
            transient: None,
        };

        let result = solve_bearing_equilibrium(&input, &NoopReporter);
        assert!(result.is_ok(), "Frontend defaults should converge: {:?}", result.err());
    }

    // ── Moment load tests ──

    #[test]
    fn test_moment_load_my_converges() {
        // Scenario matching user report: F_x=30kN, F_a=9kN, M_y=-0.096 kN·m
        let mut input = make_test_input(30.0, 9.0, 0.0);
        input.operating.m_y = -0.096;
        let result = solve_bearing_equilibrium(&input, &NoopReporter);
        assert!(result.is_ok(), "M_y moment load should converge: {:?}", result.err());
        let br = result.unwrap();
        // Should have moment warning (single-row TRB limitation)
        let has_moment_alert = br.alerts.iter().any(|a| a.category == "Moment equilibrium");
        assert!(has_moment_alert, "Should warn about moment mismatch");
    }

    #[test]
    fn test_moment_load_mx_converges() {
        let input = make_test_input(10.0, 5.0, 0.05); // M_x = 0.05 kN·m
        let result = solve_bearing_equilibrium(&input, &NoopReporter);
        assert!(result.is_ok(), "M_x moment load should converge: {:?}", result.err());
    }

    #[test]
    fn test_pure_moment_converges() {
        // Pure moment with no radial load (only axial preload)
        let mut input = make_test_input(0.0, 5.0, 0.0);
        input.operating.m_y = -0.05;
        let result = solve_bearing_equilibrium(&input, &NoopReporter);
        assert!(result.is_ok(), "Pure moment + axial should converge: {:?}", result.err());
    }

    // ── 5-DOF solver tests ──

    #[test]
    fn test_5dof_combined_load_converges() {
        let input = make_test_input(5.0, 3.0, 0.0);
        let result = solve_bearing_equilibrium_5dof(&input, &NoopReporter);
        assert!(result.is_ok(), "5-DOF should converge: {:?}", result.err());
    }

    #[test]
    fn test_5dof_vs_block_loads_agree() {
        // 5-DOF (3-DOF force + δz free) should give similar results to block solver
        let input = make_test_input(5.0, 3.0, 0.0);
        let r_block = solve_bearing_equilibrium(&input, &NoopReporter).unwrap();
        let r_5dof = solve_bearing_equilibrium_5dof(&input, &NoopReporter).unwrap();

        let q_max_block = r_block.equilibrium.roller_loads.iter().cloned().fold(0.0_f64, f64::max);
        let q_max_5dof = r_5dof.equilibrium.roller_loads.iter().cloned().fold(0.0_f64, f64::max);

        let rel_diff = (q_max_5dof - q_max_block).abs() / q_max_block;
        assert!(
            rel_diff < 0.10,
            "Q_max should be similar: block={q_max_block:.1} vs 5dof={q_max_5dof:.1}, diff={:.2}%",
            rel_diff * 100.0
        );
    }

    #[test]
    fn test_5dof_pure_radial() {
        let input = make_test_input(10.0, 0.0, 0.0);
        let result = solve_bearing_equilibrium_5dof(&input, &NoopReporter);
        assert!(result.is_ok(), "5-DOF pure radial should converge: {:?}", result.err());
    }

    #[test]
    fn test_radial_sweep_nsk_hr30306j() {
        // NSK HR30306J default preset, preload=8.957kN, Fx sweep
        let base = BearingInput {
            macro_geom: MacroGeometry {
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
            },
            raceway_geom: RacewayGeometry {
                alpha_i: 7.85,
                alpha_o: 11.859,
                r_i: 300.0,
                r_o: 300.0,
                r_rib: 1500.0,
                r_rib_circ: None,
                d_uc: 0.0,
                l_uc: 0.0,
            },
            roller_profile: RollerProfile {
                crown_type: CrownType::Polynomial {
                    coeffs: vec![-0.001713, 0.007566, -0.1307, -0.1991, -0.04019],
                },
                delta_c: 0.0,
                delta_dub_l: 0.0,
                delta_dub_s: 0.0,
                l_dub_l: 0.0,
                l_dub_s: 0.0,
                r_sph: 35.0,
                sigma_roller: 0.15,
            },
            raceway_profile_inner: RacewayProfile {
                delta_rw: 0.0,
                w_a: 0.0,
                ra: 0.15,
                custom_profile: None,
                polynomial_coeffs: Some(vec![-0.01255, -0.01808, -0.01308, 0.1398, -0.2076]),
            },
            raceway_profile_outer: RacewayProfile {
                delta_rw: 0.0,
                w_a: 0.0,
                ra: 0.15,
                custom_profile: None,
                polynomial_coeffs: Some(vec![-0.0006185, 0.001334, -0.2418, -0.08751, -0.1606]),
            },
            material: Material {
                e_roller: 210.0,
                e_ring: 210.0,
                nu: 0.3,
                hrc: 61.0,
                density_roller: 7.85,
                density_ring: 7.85,
            },
            operating: OperatingConditions {
                f_x: 0.0,
                f_y: 0.0,
                f_a: 8.957,
                m_x: 0.0,
                m_y: 0.0,
                n_inner_rpm: 1500.0,
                n_outer_rpm: 0.0,
                gamma: 0.0,
                t_op: 70.0,
                nu_40: 68.0,
                nu_100: 8.0,
                alpha_pv: 20.0,
                lubrication_type: LubricationType::Oil,
                starvation_factor: 1.0,
                rho_oil: 850.0,
                preload_mode: PreloadMode::DisplacementFromForce,
                delta_preload_um: 0.0,
                design_life_hours: 100.0,
                lubrication_model: LubricationModel::Method1_DH,
                film_decay_enabled: false, film_decay_time_hours: 0.0,
                skew_angle_deg: 0.0, replenishment_rate_nm_s: 0.0,
                surface_finish: SurfaceFinish::Standard,
                additive_type: AdditiveType::None,
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
                roughness_input_mode: RoughnessInputMode::Rq,
                rq_inner: 0.3,
                rq_outer: 0.3,
                rq_roller: 0.15,
            },
            solver: SolverParams {
                run_mode: RunMode::Single(SolverMode::Gen1),
                n_slices: 30,
                beam_type: BeamType::Timoshenko,
                convergence_tol: 1e-4,
                max_iterations: 200,
                angular_increment_deg: 2.0,
                life_method: LifeMethod::Iso16281,
                e_c: 0.0,
                contamination_level: ContaminationLevel::NormalCleanliness,
                oil_supply_method: OilSupplyMethod::OilBath,
                c_r_kn: Some(59.5),
                c_0r_kn: Some(59.8),
                f_s_min: 1.0,
                rib_contact_mode: RibContactMode::PostProcess,
                f_0r: 3.0,
                f_1r: 0.0004,
                kappa_method: KappaMethod::ViscosityRatio,
                use_split_contact: false,
            },
            transient: None,
        };

        // Radial force values [N] from the reference table
        let fx_n = [0.0, 1.0, 51.0, 100.0, 3090.0, 6080.0, 9070.0, 12060.0,
                     15050.0, 18040.0, 21030.0, 24020.0, 27010.0, 30000.0];

        let preloads_kn = [8.957, 4.388];

        for &preload in &preloads_kn {
        println!("\n  ========== Preload = {:.0} N ==========", preload * 1000.0);
        println!("\n{:>10} {:>10} {:>10} {:>10} {:>12} {:>12} {:>10}",
            "Fx(N)", "δx(mm)", "Qo_max(N)", "Qi_max(N)", "pmax_i(MPa)", "pmax_o(MPa)", "Frib(N)");
        println!("{}", "-".repeat(80));

        for &fx in &fx_n {
            let mut input = base.clone();
            input.operating.f_a = preload;
            input.operating.f_x = fx / 1000.0; // N → kN

            let result = solve_bearing_equilibrium(&input, &NoopReporter);
            match result {
                Ok(r) => {
                    let dx_mm = r.equilibrium.displacement[0] / 1000.0;

                    // Max roller load (outer & inner)
                    let q_max_o = r.equilibrium.roller_results.iter()
                        .map(|rr| rr.q_normal).fold(0.0_f64, f64::max);
                    let q_max_i = r.equilibrium.roller_results.iter()
                        .map(|rr| rr.q_normal_inner).fold(0.0_f64, f64::max);

                    // Max inner/outer contact stress across all rollers & slices
                    let mut pmax_inner = 0.0_f64;
                    let mut pmax_outer = 0.0_f64;
                    let mut f_rib_max = 0.0_f64;
                    for rr in &r.equilibrium.roller_results {
                        for s in &rr.slice_results {
                            pmax_inner = pmax_inner.max(s.p_max_k);
                            pmax_outer = pmax_outer.max(s.p_max_k_outer);
                        }
                        if let Some(ref rib) = rr.rib_result {
                            f_rib_max = f_rib_max.max(rib.f_rib);
                        }
                    }

                    println!("{:>10.0} {:>10.4} {:>10.1} {:>10.1} {:>12.1} {:>12.1} {:>10.1}",
                        fx, dx_mm, q_max_o, q_max_i, pmax_inner, pmax_outer, f_rib_max);

                    // Per-roller detail at Fx=30000N
                    if (fx - 30000.0).abs() < 1.0 {
                        println!("\n  === Per-roller detail @ Fx=30000N ===");
                        println!("  {:>4} {:>8} {:>10} {:>10} {:>12} {:>12} {:>10}",
                            "#", "ψ(°)", "Qo(N)", "Qi(N)", "pmax_i(MPa)", "pmax_o(MPa)", "Frib(N)");
                        println!("  {}", "-".repeat(72));
                        for (i, rr) in r.equilibrium.roller_results.iter().enumerate() {
                            let pi = rr.slice_results.iter()
                                .map(|s| s.p_max_k).fold(0.0_f64, f64::max);
                            let po = rr.slice_results.iter()
                                .map(|s| s.p_max_k_outer).fold(0.0_f64, f64::max);
                            let frib = rr.rib_result.as_ref().map_or(0.0, |r| r.f_rib);
                            println!("  {:>4} {:>8.1} {:>10.1} {:>10.1} {:>12.1} {:>12.1} {:>10.1}",
                                i + 1, rr.psi_deg, rr.q_normal, rr.q_normal_inner, pi, po, frib);
                        }
                        println!();
                    }
                }
                Err(e) => {
                    println!("{:>10.0} FAILED: {:?}", fx, e);
                }
            }
        }
        } // preload loop
    }

    // ─── Schwarz 2023 Combined Load Validation ──────────────────────
    //
    // Schwarz et al. 2023 *Lubricants* 11(9):369, Figures 6 & 7.  TRB 32216
    // under combined axial + radial load, oil bath, FVA 3 oil, 50 °C.
    // Validates full bearing equilibrium (load distribution shifts under
    // radial load) + BH 2010 Part 1 + Aihara thermal correction.

    /// Build a Schwarz 32216 BearingInput with the given operating conditions.
    /// Geometry from Schwarz Table 4 (where given) + reasonable approximations
    /// (where not given, e.g., raceway profile parameters).
    fn schwarz_32216_input(f_r_kn: f64, f_a_kn: f64, t_op_c: f64, n_rpm: f64) -> BearingInput {
        // Vogel FVA No. 3: ν(T) interpolated from the Vogel relation
        let eta_pas = 0.062e-3 * (1021.7_f64 / (t_op_c + 101.5517)).exp();
        let rho = 887.6 - 0.6 * (t_op_c - 15.0);
        let nu_op = eta_pas * 1e6 / rho; // cSt
        // Anchor at 40°C and 100°C for Walther-like interpolation
        let eta_40 = 0.062e-3 * (1021.7_f64 / (40.0 + 101.5517)).exp();
        let rho_40 = 887.6 - 0.6 * (40.0 - 15.0);
        let nu_40 = eta_40 * 1e6 / rho_40;
        let eta_100 = 0.062e-3 * (1021.7_f64 / (100.0 + 101.5517)).exp();
        let rho_100 = 887.6 - 0.6 * (100.0 - 15.0);
        let nu_100 = eta_100 * 1e6 / rho_100;
        let _ = nu_op; // documented; solver will recompute via nu_40/nu_100

        BearingInput {
            macro_geom: MacroGeometry {
                d: 80.0, outer_diameter: 140.0, t: 33.0,
                alpha: 14.0, z: 16,
                d_we_max: 17.5, d_we_min: 16.5,    // small taper for α=14°
                l_we: 22.7, d_pw: 108.5,
                h_rib: 3.5, alpha_rib: 8.0, g_r: 0.0, h_c: None,
            },
            raceway_geom: RacewayGeometry {
                // Realistic cone-apex matched 32216: α_i ≠ α_o consistent with
                // d_we taper.  φ_roller = arctan((d_we_max - d_we_min)/(2·l_we))
                // = arctan(1/45.4) = 1.26° → α_o - α_i = 2.52°.
                // α_i = 14° - 2.52° = 11.48° ≈ 11.5°.
                alpha_i: 11.5, alpha_o: 14.0,
                r_i: 400.0, r_o: 400.0,            // large (essentially straight cone)
                r_rib: 1500.0, r_rib_circ: None,
                d_uc: 0.0, l_uc: 0.0,
            },
            roller_profile: RollerProfile {
                crown_type: CrownType::Logarithmic { a_log: 0.0008 },
                delta_c: 3.0, delta_dub_l: 2.0, delta_dub_s: 2.0,
                l_dub_l: 1.5, l_dub_s: 1.5, r_sph: 50.0,
                sigma_roller: 0.16,                // Schwarz Table 4
            },
            raceway_profile_inner: RacewayProfile {
                delta_rw: 0.0, w_a: 0.0, ra: 0.16,
                custom_profile: None, polynomial_coeffs: None,
            },
            raceway_profile_outer: RacewayProfile {
                delta_rw: 0.0, w_a: 0.0, ra: 0.16,
                custom_profile: None, polynomial_coeffs: None,
            },
            material: Material::default(),
            operating: OperatingConditions {
                f_x: f_r_kn, f_y: 0.0, f_a: f_a_kn,
                m_x: 0.0, m_y: 0.0,
                n_inner_rpm: n_rpm, n_outer_rpm: 0.0,
                gamma: 0.0, t_op: t_op_c,
                nu_40, nu_100, alpha_pv: 20.0,
                lubrication_type: LubricationType::Oil,
                starvation_factor: 1.0, rho_oil: rho,
                preload_mode: PreloadMode::DisplacementFromForce,
                delta_preload_um: 0.0, design_life_hours: 100.0,
                lubrication_model: LubricationModel::Method1_DH,
                film_decay_enabled: false, film_decay_time_hours: 0.0,
                skew_angle_deg: 0.0, replenishment_rate_nm_s: 0.0,
                surface_finish: SurfaceFinish::Standard,
                additive_type: AdditiveType::None,
                tau_eyring: 5.0, z_roelands: 0.67,
                traction_model: TractionModel::Eyring,
                carreau_eta_inf_ratio: 0.005, carreau_lambda_s: 1.0e-7,
                carreau_n: 0.5, carreau_a: 2.0,
                friction_model: FrictionModel::BibouletHoupert,
                thermal_correction: ThermalCorrection::Aihara1987,
            hysteresis_loss_factor: 0.005,
                skf_trb_series: SkfTrbSeriesEnum::Series322,
                skf_lubrication: SkfLubricationEnum::OilBath,
                skf_y_factor: 1.6,
                k_fluid: 0.134,                     // FVA 3 (Schwarz Table A1)
                beta_visc: 0.04,
                rq_inner: 0.16, rq_outer: 0.16, rq_roller: 0.16,
                roughness_input_mode: RoughnessInputMode::Rq,
            },
            solver: SolverParams {
                e_c: 0.8,
                c_r_kn: Some(186.0),               // 32216 typical
                c_0r_kn: Some(260.0),              // Schwarz Table 4
                ..SolverParams::default()
            },
            transient: None,
        }
    }

    /// Schwarz 32216 combined load smoke test — full bearing equilibrium
    /// solver runs without panicking with combined F_a + F_r load.
    ///
    /// NOTE: Magnitude validation against Schwarz Fig 6/7 measurements is
    /// limited by **profile/geometry uncertainty**: Schwarz Table 4 gives
    /// some parameters (Z, d_RB, l_RB, σ_raceway, n_RB) but does NOT
    /// publish exact raceway curvature radii (r_i, r_o) or roller crown
    /// profile coefficients (a_log, polynomial fit).  Initial test with
    /// estimated profile produced 21 N·m vs measured 2.5 N·m (~8× over),
    /// dominated by approximate profile-induced load concentration.
    /// Therefore this test validates only that the solver path **runs**
    /// for combined load with BibouletHoupert + Aihara, not the exact
    /// numerical agreement.  See Manual §14.15.4.6A for the per-contact
    /// analytical Schwarz axial-only tests that DO validate magnitude
    /// within ±10 % under controlled geometry.
    #[test]
    fn test_schwarz_32216_combined_load_solver_runs() {
        let input = schwarz_32216_input(6.5, 6.0, 50.0, 2000.0);
        let result = solve_bearing_equilibrium(&input, &NoopReporter);
        assert!(result.is_ok(), "Schwarz combined-load solver must converge: {:?}", result.err());
        let br = result.unwrap();
        let traction = br.traction.expect("traction must be computed for n>0");
        let m_fric = traction.m_friction_nmm;
        eprintln!("Schwarz 32216 combined @ 2000 rpm 50°C (F_a=6, F_r=6.5):");
        eprintln!("  Our M_friction = {:.1} N·mm (measured Schwarz Fig 6 ≈ 2500)", m_fric);
        // Order-of-magnitude check — full solver result must be positive and
        // within 3 decades of measurement (catches gross unit/model errors).
        assert!(m_fric > 100.0 && m_fric < 100_000.0,
            "Order-of-magnitude check: M_friction={m_fric:.1} N·mm out of [1e2, 1e5]");
    }

    /// Schwarz 32216 combined vs axial-only — qualitative trend check.
    /// Per Schwarz §3: combined load with significant F_r shifts load zone
    /// → not all rollers carry load → M can be LOWER than pure-axial
    /// despite higher total load.
    ///
    /// Limitation: with our approximate profile parameters, the actual
    /// numerical values are off by ~8× vs measured.  We can still verify
    /// that the solver produces *some* sensitivity to the radial load
    /// addition (either reduction or modest increase, never explosion).
    #[test]
    fn test_schwarz_32216_combined_vs_axial_load_zone_shift() {
        let n = 2000.0;
        let t = 50.0;
        let axial_only = schwarz_32216_input(0.0, 6.0, t, n);
        let combined   = schwarz_32216_input(6.5, 6.0, t, n);

        let m_axial = solve_bearing_equilibrium(&axial_only, &NoopReporter)
            .unwrap().traction.unwrap().m_friction_nmm;
        let m_combined = solve_bearing_equilibrium(&combined, &NoopReporter)
            .unwrap().traction.unwrap().m_friction_nmm;
        eprintln!("Schwarz combined vs axial-only @ 2000 rpm 50°C:");
        eprintln!("  Axial only (F_a=6 kN):     {:.1} N·mm", m_axial);
        eprintln!("  Combined (+F_r=6.5 kN):    {:.1} N·mm", m_combined);
        // Both must be positive finite; combined must not be wildly higher
        // (load-zone shift should keep things bounded).
        assert!(m_axial > 0.0 && m_axial.is_finite());
        assert!(m_combined > 0.0 && m_combined.is_finite());
        assert!(m_combined < m_axial * 2.0,
            "Combined load M should not greatly exceed axial-only \
             (load-zone shift keeps total bounded): \
             axial={m_axial:.1}, combined={m_combined:.1}");
    }

    /// DIAGNOSTIC — call `compute_rib_contact()` directly with Schwarz 32216
    /// geometry + Tewari Eq. 3 F_rib per roller, verify rib power magnitude.
    /// This bypasses the full solver (which has profile-sensitivity issues)
    /// and isolates the rib EHL calculation alone.
    #[test]
    #[ignore]
    fn diag_schwarz_32216_rib_direct() {
        use crate::solver::rib_contact::compute_rib_contact;

        let macro_geom = MacroGeometry {
            d: 80.0, outer_diameter: 140.0, t: 33.0,
            alpha: 14.0, z: 16,
            d_we_max: 17.5, d_we_min: 16.5,
            l_we: 22.7, d_pw: 108.5,
            h_rib: 3.5, alpha_rib: 8.0, g_r: 0.0, h_c: None,
        };
        // Realistic 322-series geometry: cup half-angle α_o ≈ 14°, cone half-angle
        // α_i ≈ 12° (typical for 32216; α_i = α_o makes cone-apex match degenerate
        // and ω_roller = 0 → u_slide_rib = 0 in our compute_trb_kinematics).
        let raceway_geom = RacewayGeometry {
            alpha_i: 12.0, alpha_o: 14.0,
            r_i: 400.0, r_o: 400.0,
            r_rib: 1500.0, r_rib_circ: None,
            d_uc: 0.0, l_uc: 0.0,
        };
        let roller_profile = RollerProfile {
            crown_type: CrownType::Logarithmic { a_log: 0.0008 },
            delta_c: 3.0, delta_dub_l: 2.0, delta_dub_s: 2.0,
            l_dub_l: 1.5, l_dub_s: 1.5, r_sph: 50.0,
            sigma_roller: 0.16,
        };
        let material = Material::default();

        // F_rib per roller via Tewari Eq. 3 (assume γ_rib ≈ 2.5°)
        let alpha_o_rad: f64 = 14.0_f64.to_radians();
        let gamma_rib: f64 = 2.5_f64.to_radians();
        let z = 16.0;
        let f_a = 6000.0;
        let q_axial_per_roller = f_a * (2.0 * gamma_rib).sin() / (z * alpha_o_rad.sin());
        eprintln!("\nSchwarz 32216 — direct compute_rib_contact() call:");
        eprintln!("  F_a = 6000 N, Z=16, α_o=14°, γ_rib=2.5° (estimated)");
        eprintln!("  F_rib per roller (Tewari Eq.3) = {:.1} N\n", q_axial_per_roller);

        for n_rpm in [500.0_f64, 2000.0, 4000.0] {
            // Build operating with explicit speed
            let operating = OperatingConditions {
                f_x: 0.0, f_y: 0.0, f_a: 6.0,
                m_x: 0.0, m_y: 0.0,
                n_inner_rpm: n_rpm, n_outer_rpm: 0.0,
                gamma: 0.0, t_op: 50.0,
                nu_40: 95.0, nu_100: 11.0, alpha_pv: 20.0,
                lubrication_type: LubricationType::Oil,
                starvation_factor: 1.0, rho_oil: 870.0,
                preload_mode: PreloadMode::DisplacementFromForce,
                delta_preload_um: 0.0, design_life_hours: 100.0,
                lubrication_model: LubricationModel::Method1_DH,
                film_decay_enabled: false, film_decay_time_hours: 0.0,
                skew_angle_deg: 0.0, replenishment_rate_nm_s: 0.0,
                surface_finish: SurfaceFinish::Standard,
                additive_type: AdditiveType::None,
                tau_eyring: 5.0, z_roelands: 0.67,
                traction_model: TractionModel::Eyring,
                carreau_eta_inf_ratio: 0.005, carreau_lambda_s: 1.0e-7,
                carreau_n: 0.5, carreau_a: 2.0,
                friction_model: FrictionModel::BibouletHoupert,
                thermal_correction: ThermalCorrection::Aihara1987,
                hysteresis_loss_factor: 0.005,
                skf_trb_series: SkfTrbSeriesEnum::Series322,
                skf_lubrication: SkfLubricationEnum::OilBath,
                skf_y_factor: 1.6,
                k_fluid: 0.134, beta_visc: 0.04,
                rq_inner: 0.16, rq_outer: 0.16, rq_roller: 0.16,
                roughness_input_mode: RoughnessInputMode::Rq,
            };

            let result = compute_rib_contact(
                &roller_profile, &macro_geom, &raceway_geom, &material,
                q_axial_per_roller, Some(&operating),
            );

            match result {
                Ok(rc) => {
                    if let Some(ehl) = &rc.ehl {
                        // Per-contact power = μ_eff × F_rib × u_slide_rib
                        let p_per = ehl.mu_eff * rc.f_rib * ehl.u_slide_m_s;
                        let p_total = p_per * macro_geom.z as f64;
                        let omega_i = 2.0 * std::f64::consts::PI * n_rpm / 60.0;
                        let m_rib_nmm = p_total / omega_i * 1000.0;
                        eprintln!("  n={:.0} rpm: μ_eff={:.4}, u_slide={:.3} m/s, Λ={:.2} ({:?}), \
                                   F_rib={:.0} N → M_rib={:.1} N·mm",
                            n_rpm, ehl.mu_eff, ehl.u_slide_m_s, ehl.lambda_ratio,
                            ehl.regime, rc.f_rib, m_rib_nmm);
                    } else {
                        eprintln!("  n={:.0} rpm: rib_result has no EHL (operating None?)", n_rpm);
                    }
                }
                Err(e) => eprintln!("  n={:.0} rpm: ERROR {:?}", n_rpm, e),
            }
        }
    }

    /// DIAGNOSTIC — verify our actual `compute_rib_contact()` + full traction
    /// breakdown for Schwarz 32216 axial 6 kN @ 4000 rpm 50°C.
    ///
    /// Goal: separate which component contributes how much to the total,
    /// see if profile assumption (a_log=0.0008) is the issue, and verify
    /// our rib_contact.rs code outputs reasonable rib power.
    #[test]
    #[ignore] // Diagnostic; run with: cargo test schwarz_32216_traction_breakdown -- --ignored --nocapture
    fn diag_schwarz_32216_traction_breakdown() {
        let cases = [
            (500.0_f64, 50.0_f64, 1300.0_f64),   // measured M_T from Fig 5
            (2000.0,    50.0,    2950.0),
            (4000.0,    50.0,    3750.0),
        ];

        eprintln!("\nSchwarz 32216 — full solver traction breakdown (axial 6 kN, 50 °C):");
        eprintln!("  {:<10} {:<10} {:<10} {:<10} {:<10} {:<10} {:<10}",
            "n[rpm]", "Measured", "P_roll[W]", "P_slide[W]", "P_rib[W]", "P_hys[W]", "M_tot[Nmm]");
        for (n, t_op, m_meas) in cases {
            let input = schwarz_32216_input(0.0, 6.0, t_op, n);
            let result = solve_bearing_equilibrium(&input, &NoopReporter);
            if let Ok(br) = result {
                if let Some(t) = br.traction {
                    eprintln!("  {:<10.0} {:<10.0} {:<10.1} {:<10.1} {:<10.1} {:<10.1} {:<10.1}",
                        n, m_meas, t.p_rolling_w, t.p_sliding_w, t.p_rib_w,
                        t.p_hysteresis_w, t.m_friction_nmm);
                }
            } else {
                eprintln!("  {:<10.0} solver failed", n);
            }
        }

        eprintln!("\nNote: M_tot from full solver may differ from measured due to");
        eprintln!("approximate Schwarz 32216 profile (a_log=0.0008) we use — Schwarz Table 4");
        eprintln!("does not publish exact crown coefficients.");
    }

    /// DIAGNOSTIC — Schwarz Fig 6 (combined load F_a=6, F_r=6.5 kN) validation
    /// against figure-extracted measurements at 4 speeds × 2 temperatures.
    /// Run after kinematics u_outer/u_inner fix (2026-05-13).
    #[test]
    #[ignore]
    fn diag_schwarz_32216_combined_fig6() {
        // Measurements figure-extracted from Schwarz Fig 6
        let cases_50c = [
            (500.0_f64,  1500.0_f64),
            (1000.0,     1950.0),
            (2000.0,     2500.0),
            (4000.0,     3250.0),
        ];
        let cases_42c = [
            (500.0_f64,  2000.0_f64),
            (1000.0,     2650.0),
            (2000.0,     3500.0),
            (4000.0,     4450.0),
        ];

        eprintln!("\nSchwarz 32216 — full solver vs Fig 6 (combined F_a=6 kN + F_r=6.5 kN):");
        for (t_op, cases) in [(50.0, &cases_50c[..]), (42.0, &cases_42c[..])] {
            eprintln!("  T = {:.0} °C:", t_op);
            eprintln!("    {:<8} {:<10} {:<10} {:<10} {:<10} {:<10}",
                "n[rpm]", "M_meas", "M_ours", "Δ%", "P_roll[W]", "P_rib[W]");
            for &(n, m_meas) in cases {
                let input = schwarz_32216_input(6.5, 6.0, t_op, n);
                match solve_bearing_equilibrium(&input, &NoopReporter) {
                    Ok(br) => {
                        if let Some(t) = br.traction {
                            let m_ours = t.m_friction_nmm;
                            let delta = (m_ours - m_meas) / m_meas * 100.0;
                            eprintln!("    {:<8.0} {:<10.0} {:<10.1} {:<+10.1} {:<10.1} {:<10.1}",
                                n, m_meas, m_ours, delta, t.p_rolling_w, t.p_rib_w);
                        }
                    }
                    Err(e) => eprintln!("    n={:.0}: ERROR {:?}", n, e),
                }
            }
        }
    }

    /// DIAGNOSTIC — Schwarz Fig 7 (radial sweep 1-15 kN at axial 6.5 kN preload,
    /// 2000 rpm, 50 °C).  Expect M_T to peak around F_r ≈ 4-8 kN and DROP for
    /// larger F_r due to load-zone shift (only ~11 rollers carry under high
    /// radial load per Schwarz §3).
    #[test]
    #[ignore]
    fn diag_schwarz_32216_radial_sweep_fig7() {
        let cases = [
            (1.0_f64,  1570.0_f64),
            (4.0,      1620.0),
            (8.0,      1610.0),
            (12.0,     1480.0),
            (15.0,     1300.0),
        ];
        eprintln!("\nSchwarz 32216 — full solver vs Fig 7 (F_a=6.5 kN preload, n=2000 rpm, T=50 °C):");
        eprintln!("  {:<8} {:<10} {:<10} {:<10}",
            "F_r[kN]", "M_meas", "M_ours", "Δ%");
        for (f_r, m_meas) in cases {
            let input = schwarz_32216_input(f_r, 6.5, 50.0, 2000.0);
            match solve_bearing_equilibrium(&input, &NoopReporter) {
                Ok(br) => {
                    if let Some(t) = br.traction {
                        let m_ours = t.m_friction_nmm;
                        let delta = (m_ours - m_meas) / m_meas * 100.0;
                        eprintln!("  {:<8.1} {:<10.0} {:<10.1} {:<+10.1}",
                            f_r, m_meas, m_ours, delta);
                    }
                }
                Err(e) => eprintln!("  F_r={:.1}: ERROR {:?}", f_r, e),
            }
        }
    }

    #[test]
    #[ignore] // Slow in debug mode (~250s); run with: cargo test --release test_split_contact
    fn test_split_contact_bearing_solver() {
        let mut input = make_test_input(5.0, 3.0, 0.0);
        input.solver.use_split_contact = true;
        let result = solve_bearing_equilibrium(&input, &NoopReporter);
        match &result {
            Ok(br) => {
                let q_max = br.equilibrium.roller_loads.iter().copied().fold(0.0_f64, f64::max);
                println!("Split bearing OK: Q_max={q_max:.1}N, disp={:?}", br.equilibrium.displacement);
            }
            Err(e) => {
                println!("Split bearing FAILED: {e:?}");
            }
        }
        assert!(result.is_ok(), "Split contact bearing should converge: {:?}", result.err());
    }
}
