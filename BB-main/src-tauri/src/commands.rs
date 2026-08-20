// CRB Contact Analysis — Tauri Commands
// Phase 1.3-B 하이브리드 stub 상태.
// 살아있는 command: slice geometry, single-slice Hertz, Gen1/Gen3 roller solve, bearing equilibrium
// 제거된 command (Phase 2~7 에서 재활성화):
//   - compute_rib       (D1: rib contact 제외 → 영구 제거)
//   - solve_transient   (Phase 7)
//   - parse_load_csv    (Phase 7)
//   - run_hmehl         (Phase 7)

use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::solver::bearing;
use crate::solver::gen1;
use crate::solver::gen3;
use crate::solver::geometry::compute_slices;
use crate::solver::hertz::{combined_elastic_modulus, compute_slice_contact};
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
/// CRB: cos_alpha_diff = 1.0 (α_i = α_o = 0).
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

    // CRB: raceway taper angle = 0 → cos_alpha_diff = 1.0
    let cos_alpha_diff = 1.0;
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

    let cos_alpha_diff = 1.0; // CRB
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

    let cos_alpha_diff = 1.0; // CRB
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

    let cos_alpha_diff = 1.0; // CRB
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

/// Solve full bearing equilibrium (CRB: 3-DOF δx/δy/γx) for given operating conditions.
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

// ─── Phase 1 stub: 아래 command 는 CRB Phase 후속 단계에서 재활성화 ───
// compute_rib      : D1 (rib contact 제외) → 영구 제거
// solve_transient  : Phase 7 (transient dynamics)
// parse_load_csv   : Phase 7 (LoadTimePoint 재작성 후)
// run_hmehl        : Phase 7 (HMEHL)
