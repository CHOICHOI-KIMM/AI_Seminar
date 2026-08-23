// BB Contact Analysis — Tauri Commands
//
// BB Phase 1-S2 (2026-08-20): `types.rs` 를 ACBB 로 재작성하면서
// geometry·hertz·bearing 모듈이 일시 비활성화됨 (Plan Phase 1 작업 5).
// 그에 따라 아래 command 도 함께 해제한다:
//   compute_slice_geometry / compute_hertz_single_slice   (롤러 슬라이스)
//   solve_bearing / solve_bearing_dual                    (CRB 3-DOF 평형)
//
// ─── BB 후속 단계 신규 command (예정) ───────────────────────────────
// compute_life         : P4    (ISO 16281 §5.2)
// compute_film         : P5    (Hamrock-Dowson 타원접촉)

use tauri::{AppHandle, Emitter};

use crate::solver::bb::bearing;
use crate::solver::bb::geometry;
use crate::solver::bb::hertz;
use crate::solver::bb::types::*;
use crate::solver::common::types::*;

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
    pub derived: BbGeometryDerived,
    pub summary: BbGeometrySummary,
    pub alerts: Vec<Alert>,
}

/// ACBB 기하 전처리 (Theory §2, 식 A.1/A.3/A.4/E.4~E.7).
#[tauri::command]
pub fn compute_geometry(input: BbInput) -> Result<GeometryResponse, String> {
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

// ─── P2: 점접촉 타원 Hertz ────────────────────────────────────────────

/// 점접촉 전처리 + 주어진 볼 하중에서의 접촉타원·응력.
#[derive(serde::Serialize)]
pub struct ContactResponse {
    pub derived: BbContactDerived,
    /// 요청 하중 [N] (없으면 0)
    pub q_n: f64,
    /// 총 탄성변형 δ [mm] — 식 (38)
    pub delta_mm: f64,
    pub a_inner_mm: f64,
    pub b_inner_mm: f64,
    pub p_max_inner_mpa: f64,
    pub a_outer_mm: f64,
    pub b_outer_mm: f64,
    pub p_max_outer_mpa: f64,
    pub alerts: Vec<Alert>,
}

/// ACBB 점접촉 해석 (Theory §3, §6).
///
/// `q_n` 은 볼 1개에 걸리는 법선하중 [N]. 0 이면 전처리(χ·c_P)만 수행한다.
#[tauri::command]
pub fn compute_contact(input: BbInput, q_n: f64) -> Result<ContactResponse, String> {
    input.validate().map_err(|e| e.to_string())?;
    let geo = geometry::compute_geometry_derived(&input.geometry).map_err(|e| e.to_string())?;
    let derived =
        hertz::compute_contact_derived(&geo, &input.material).map_err(|e| e.to_string())?;

    let delta_mm = hertz::delta_from_q(derived.c_p_n_per_mm15, q_n);
    let (a_i, b_i, p_i) = hertz::contact_ellipse(
        derived.e_star_mpa,
        geo.sum_rho_i_per_mm,
        derived.a_star_inner,
        derived.b_star_inner,
        q_n,
    );
    let (a_e, b_e, p_e) = hertz::contact_ellipse(
        derived.e_star_mpa,
        geo.sum_rho_e_per_mm,
        derived.a_star_outer,
        derived.b_star_outer,
        q_n,
    );

    let mut alerts = Vec::new();
    let p_worst = p_i.max(p_e);
    if p_worst > hertz::SIGMA_HU_MPA {
        alerts.push(Alert {
            level: if p_worst > 4_000.0 {
                AlertLevel::Critical
            } else {
                AlertLevel::Warning
            },
            code: "CONTACT_STRESS_OVER_FATIGUE_LIMIT".into(),
            message: format!(
                "최대 접촉응력 {p_worst:.0} MPa 가 ISO 281 Annex B.3.1 권장 피로한계                  {:.0} MPa 를 초과합니다",
                hertz::SIGMA_HU_MPA
            ),
        });
    }

    Ok(ContactResponse {
        derived,
        q_n,
        delta_mm,
        a_inner_mm: a_i,
        b_inner_mm: b_i,
        p_max_inner_mpa: p_i,
        a_outer_mm: a_e,
        b_outer_mm: b_e,
        p_max_outer_mpa: p_e,
        alerts,
    })
}

// ─── P3-1: 5-DOF 평형 ─────────────────────────────────────────────────

/// ACBB 5-DOF 정적 평형 (Theory §4).
///
/// `BbSolverParams::dof_mask` 로 자유도를 구속할 수 있다 (`ISO_3DOF` 등).
/// `phase_sweep.enabled` 이면 케이지 위상 스윕 결과가 함께 반환된다.
#[tauri::command]
pub async fn solve_bearing(app: AppHandle, input: BbInput) -> Result<BbResult, String> {
    let _reporter = TauriReporter { app };
    tauri::async_runtime::spawn_blocking(move || {
        bearing::solve_bearing(&input).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("Task join error: {e}"))?
}
