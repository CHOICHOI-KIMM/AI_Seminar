// BB Contact Analysis — 베어링 계열 무관 공통 데이터 모델
//
// P4-S0-2 (2026-08-23): `solver/types.rs` 에서 **베어링 계열에 의존하지 않는
// 6개 타입만** 분리했다 (Plan §3.6.1.6 「진짜 공통 (6)」).
//   SolverProgress · ProgressReporter · NoopReporter · Material · Alert · AlertLevel
//
// ⚠ `SolverError` 는 `solver/` 밖(`src/error.rs`)이라 이동 대상이 아니다.
// ⚠ 볼 전용 타입은 `solver/bb/types.rs` 에 있다. 이 파일은 **`bb/` 를 참조하지 않는다.**
//
// 단위 규약은 D-10 (mm · N · rad · MPa) 을 그대로 따른다.

use serde::{Deserialize, Serialize};

use crate::error::SolverError;

// ═══════════════════════════════════════════════════════════════════
//  진행률 리포터
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverProgress {
    pub stage: String,
    pub detail: String,
    pub percent: f64,
}

pub trait ProgressReporter: Send + Sync {
    fn report(&self, progress: SolverProgress);
}

/// 진행률을 버리는 리포터. 테스트·비대화형 호출용.
pub struct NoopReporter;

impl ProgressReporter for NoopReporter {
    fn report(&self, _progress: SolverProgress) {}
}
// ═══════════════════════════════════════════════════════════════════
//  입력 — 재질
// ═══════════════════════════════════════════════════════════════════

/// 재질 물성. 탄성계수는 **MPa** 로 통일한다 (D-10).
///
/// CRB 는 `[GPa]` 로 보관하고 소비처마다 `* 1000.0` 을 곱했다 (15중 중복).
/// 그 암묵적 계약을 제거했다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    /// 볼 탄성계수 [MPa]. ISO 16281 Clause 4 NOTE 1: 강 = 207 000 MPa
    pub e_ball_mpa: f64,
    /// 레이스웨이 탄성계수 [MPa]
    pub e_ring_mpa: f64,
    /// 포아송비. ISO 16281 Clause 4 NOTE 6: 강 = 0,3
    pub nu: f64,
    /// 경도 [HRC] — 정적정격·마이크로피팅 판정용 (P4)
    pub hrc: f64,
    /// 볼 밀도 [g/cm³]
    pub density_ball_g_cm3: f64,
    /// 링 밀도 [g/cm³]
    pub density_ring_g_cm3: f64,
}

impl Default for Material {
    /// ISO 16281 Clause 4 NOTE 1 / NOTE 6 의 강(steel) 기준값.
    fn default() -> Self {
        Self {
            e_ball_mpa: 207_000.0,
            e_ring_mpa: 207_000.0,
            nu: 0.3,
            hrc: 60.0,
            density_ball_g_cm3: 7.85,
            density_ring_g_cm3: 7.85,
        }
    }
}

impl Material {
    pub fn validate(&self) -> Result<(), SolverError> {
        if self.e_ball_mpa <= 0.0 || self.e_ring_mpa <= 0.0 {
            return Err(SolverError::InvalidInput(
                "탄성계수는 양수여야 합니다 [MPa]".into(),
            ));
        }
        if !(0.0..0.5).contains(&self.nu) {
            return Err(SolverError::InvalidInput(
                "포아송비는 [0, 0.5) 범위여야 합니다".into(),
            ));
        }
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════
//  경고 (Alert)
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub level: AlertLevel,
    pub code: String,
    pub message: String,
}
