// BB Contact Analysis — Tauri Commands
//
// BB Phase 1-S2 (2026-08-20): `types.rs` 를 ACBB 로 재작성하면서
// geometry·hertz·bearing 모듈이 일시 비활성화됨 (Plan Phase 1 작업 5).
// 그에 따라 아래 command 도 함께 해제한다:
//   compute_slice_geometry / compute_hertz_single_slice   (롤러 슬라이스)
//   solve_bearing / solve_bearing_dual                    (CRB 3-DOF 평형)
//
// ─── BB 후속 단계 신규 command (예정) ───────────────────────────────
// solve_ball_contact   : P2    (점접촉 타원 Hertz)
// solve_bearing_5dof   : P3    (5-DOF 평형 + 위상 스윕)
// compute_life         : P4    (ISO 16281 §5.2)
// compute_film         : P5    (Hamrock-Dowson 타원접촉)

use tauri::{AppHandle, Emitter};

use crate::solver::geometry;
use crate::solver::types::*;

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

// ─── P1-S3: 기하 전처리 ──────────────────────────────────────────────

/// 하중 무관 기하 전처리 결과 + 요약 + 경고.
///
/// 단위는 전부 솔버 내부 단위 (mm · N · rad, D-10).
/// UI 표시용 μm·kN·° 환산은 프론트엔드가 담당한다.
#[derive(serde::Serialize)]
pub struct GeometryResponse {
    pub derived: GeometryDerived,
    pub summary: GeometrySummary,
    pub alerts: Vec<Alert>,
}

/// ACBB 기하 전처리 (Theory §2, 식 A.1/A.3/A.4/E.4~E.7).
#[tauri::command]
pub fn compute_geometry(input: BearingInput) -> Result<GeometryResponse, String> {
    input.validate().map_err(|e| e.to_string())?;
    let derived = geometry::compute_geometry_derived(&input.geometry).map_err(|e| e.to_string())?;
    let summary = geometry::compute_geometry_summary(
        &input.geometry,
        &derived,
        &input.operating,
        &input.material,
    );
    let alerts = geometry::collect_geometry_alerts(&summary);
    Ok(GeometryResponse {
        derived,
        summary,
        alerts,
    })
}
