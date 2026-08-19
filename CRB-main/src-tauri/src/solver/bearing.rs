// CRB Bearing-Level Equilibrium Solver
// ─────────────────────────────────────────────────────────────────────
// Phase 1.3-B 하이브리드 stub 상태.
//
// Phase 4 (Bearing-Level Equilibrium) 에서 재작성 예정:
//   - ISO 16281 A.3.1 알고리즘 (CRB)
//   - 평형 DOF = 3: (δx, δy, γx)  (D4+D6+D7)
//   - 접촉력 방향 = pure radial
//   - Single row 만 (D3)
//
// 현재는 순수 계산 함수만 유지, 통합 함수는 error return stub.
// ─────────────────────────────────────────────────────────────────────

use crate::error::SolverError;
use crate::solver::types::*;

/// Compute roller angular positions [rad] around the bearing.
///
/// ψ_j = load_angle + 2π·(j - 0.5)/z  (j = 1..=z, roller #0 at load_angle)
///
/// CRB: single row, uniform spacing.
pub fn roller_positions(z: u32, load_angle: f64) -> Vec<f64> {
    let two_pi = std::f64::consts::TAU;
    (0..z)
        .map(|j| load_angle + two_pi * (j as f64) / (z as f64))
        .collect()
}

/// Radial load direction angle [rad] = atan2(f_y, f_x).
/// Returns 0.0 if load magnitude is negligible.
pub fn radial_load_angle(f_x: f64, f_y: f64) -> f64 {
    let f_r = (f_x * f_x + f_y * f_y).sqrt();
    if f_r < 1e-10 { 0.0 } else { f_y.atan2(f_x) }
}

/// Roller approach (rigid-body geometric interference) [μm].
///
/// CRB (D5+D6+D7): 3-DOF only.
/// disp = [δx, δy, γx] where:
///   δx, δy = radial displacement [μm]
///   γx     = misalignment about X-axis [rad]  (D6: single-plane, γy = 0)
///
/// δ_rigid(ψ) = δx·cos(ψ) + δy·sin(ψ) - g_r/2
///              + (d_pw/2) · γx · sin(ψ) · 1000    (axial-arm contribution)
///
/// NOTE: CRB 에서 α=0 이므로 원래 TRB 식의 sin(α) 성분은 자동 소멸.
///       axial arm 항은 misalignment 로 인한 slice-level tilt 를 나타내며,
///       Phase 4 에서 정확한 형태 재검토.
pub fn roller_approach(
    disp: &[f64; 3],
    psi: f64,
    d_pw: f64,
    g_r: f64,
    gamma_ext: f64,
) -> f64 {
    let (cos_psi, sin_psi) = (psi.cos(), psi.sin());
    let delta_r = disp[0] * cos_psi + disp[1] * sin_psi;
    let gamma_x_total = disp[2] + gamma_ext;
    let axial_arm = (d_pw / 2.0) * 1000.0 * gamma_x_total * sin_psi;
    delta_r + axial_arm - g_r / 2.0
}

// ─────────────────────────────────────────────────────────────────────
// Phase 1 stub — 이하 함수는 Phase 4 에서 재작성 예정.
// 현재 호출 시 SolverError 반환 (panic 아님).
// ─────────────────────────────────────────────────────────────────────

const STUB_MSG: &str = "CRB Phase 1 stub: bearing solver 는 Phase 4 (A.3.1 3-DOF) 에서 재작성 예정";

/// [STUB] Full bearing equilibrium.
pub fn solve_bearing_equilibrium(
    _input: &BearingInput,
    _progress: &dyn ProgressReporter,
) -> Result<BearingResult, SolverError> {
    Err(SolverError::InvalidInput(STUB_MSG.into()))
}

/// [STUB] Dual-mode Gen1 + Gen3 comparison.
pub fn solve_bearing_dual(
    _input: &BearingInput,
    _progress: &dyn ProgressReporter,
) -> Result<DualModeComparison, SolverError> {
    Err(SolverError::InvalidInput(STUB_MSG.into()))
}
