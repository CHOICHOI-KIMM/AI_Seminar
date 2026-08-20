// BB Contact Analysis — Tauri Commands
// BB Phase 1 (2026-08-20): 롤러 전용 command 영구 삭제.
//   - solve_roller_gen1 / _for_load / solve_roller_gen3 / _for_load  (슬라이스·빔 — 볼에 개념 없음)
//   - compute_rib / solve_transient / parse_load_csv / run_hmehl     (이전 단계에서 이미 제거)
// 남은 command 는 아직 CRB(선접촉) 코드 위에서 동작한다. ACBB 화는 P2(hertz)·P3(bearing) 에서.

use tauri::{AppHandle, Emitter};

use crate::solver::bearing;
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

// ─── BB 후속 단계 신규 command (예정) ───────────────────────────────
// solve_ball_contact   : P2 (점접촉 타원 Hertz)
// solve_bearing_5dof   : P3 (5-DOF 평형 + 위상 스윕)
// compute_life         : P4 (ISO 16281 §5.2)
// compute_film         : P5 (Hamrock-Dowson 타원접촉)
