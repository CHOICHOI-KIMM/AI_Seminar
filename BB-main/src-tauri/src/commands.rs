// BB Contact Analysis — Tauri Commands
//
// BB Phase 1-S2 (2026-08-20): `types.rs` 를 ACBB 로 재작성하면서
// geometry·hertz·bearing 모듈이 일시 비활성화됨 (Plan Phase 1 작업 5).
// 그에 따라 아래 command 도 함께 해제한다:
//   compute_slice_geometry / compute_hertz_single_slice   (롤러 슬라이스)
//   solve_bearing / solve_bearing_dual                    (CRB 3-DOF 평형)
//
// ─── BB 후속 단계 신규 command (예정) ───────────────────────────────
// compute_geometry     : P1-S3 (A·α₀·R_i·Σρ·F(ρ))
// solve_ball_contact   : P2    (점접촉 타원 Hertz)
// solve_bearing_5dof   : P3    (5-DOF 평형 + 위상 스윕)
// compute_life         : P4    (ISO 16281 §5.2)
// compute_film         : P5    (Hamrock-Dowson 타원접촉)

use tauri::{AppHandle, Emitter};

use crate::solver::types::{ProgressReporter, SolverProgress};

/// Tauri event 기반 진행률 리포터.
/// P1-S3 이후 신규 command 에서 사용한다.
#[allow(dead_code)]
pub struct TauriReporter {
    pub app: AppHandle,
}

impl ProgressReporter for TauriReporter {
    fn report(&self, progress: SolverProgress) {
        let _ = self.app.emit("solver-progress", &progress);
    }
}
