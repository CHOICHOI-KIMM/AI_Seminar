// BB Contact Analysis — 기본 정정격하중 (ISO 76:2006)
//
// BB Phase 5-1 S5-1-1 (2026-08-24): **백지 신규**.
// 같은 이름의 구 파일은 P4-S0-1 에서 영구 삭제된 롤러(TRB) 판이라 관련이 없다.
//
// ── 근거 ────────────────────────────────────────────────────────────
// BB_Development_Theory.md §7.11 ~ §7.13 (ISO 76:2006, 2026-08-24 전사).
//   (76-1)  C_0r = f_0 · i · Z · D_w² · cos α
//   (76-2)  P_0r = X_0 F_r + Y_0 F_a
//   (76-3)  P_0r = F_r
//   (76-14) S_0  = C_0r / P_0r
//
// ── 범위 (사용자 확정 2026-08-24) ───────────────────────────────────
// **단열 레이디얼 볼만.** thrust ball(ISO 76 §6)·복열 조합·자동조심은 전부 제외.
// 따라서 i = 1 고정이며 (76-15) S_0 = C_0a/P_0a 는 구현하지 않는다.
//
// ── 전제 (Theory §7.11~7.13 말미 표) ────────────────────────────────
//  · Table 1 은 E = 2,07×10⁵ MPa · ν = 0,3 을 전제한다 (`Material::default()`)
//  · Table 1 은 최대 볼하중을 5F_r/(Z cos α) 로 **가정**한 규격값이다.
//    우리 솔버가 실제 평형에서 얻는 Q_max 와는 **다른 경로**이므로 직접 비교 금지.
//  · σ_max = 4 200 MPa (영구변형 한계). §7.9 의 σ_Hu = 1 500 MPa(피로한계)와 다른 기준.
//  · α 는 **공칭 접촉각** `alpha_nom_rad` 다. α₀(클리어런스 유래)·α_j(하중 유래)가 아니다.
//
// ── 단위 (D-10) ─────────────────────────────────────────────────────
// mm · N · rad. 이 파일에 단위 환산 상수는 없다.

use crate::error::SolverError;
use crate::solver::bb::types::*;
use crate::solver::common::types::{Alert, AlertLevel};
use crate::solver::common::util;

// ═══════════════════════════════════════════════════════════════════
//  f_0 — ISO 76 Table 1 (레이디얼·각접촉 볼 열)
// ═══════════════════════════════════════════════════════════════════

/// ISO 76:2006 Table 1 — 레이디얼·각접촉 홈 볼베어링의 `f_0`.
///
/// 진입 변수는 `γ = D_w cos α / D_pw` (Clause 4) 이며 `BbGeometryDerived::gamma`
/// 와 **같은 양**이다. 중간값은 선형보간한다 (Table 1 NOTE).
///
/// ⚠️ **비단조 표다.** γ = 0,09 에서 16,5 로 최대가 되고 이후 감소한다.
///    이 굴곡은 전사 오류가 아니라 **지배 궤도륜이 외륜 → 내륜으로 바뀌는 지점**이다
///    (ISO/TR 10657 Table A.1 의 κ 열이 같은 γ 에서 6,08 → 8,68 로 도약한다).
///    값을 기준으로 탐색하는 보간은 여기서 반드시 틀린다.
pub const F0_TABLE: [(f64, f64); 41] = [
    (0.00, 14.7), (0.01, 14.9), (0.02, 15.1), (0.03, 15.3), (0.04, 15.5),
    (0.05, 15.7), (0.06, 15.9), (0.07, 16.1), (0.08, 16.3), (0.09, 16.5),
    (0.10, 16.4), (0.11, 16.1), (0.12, 15.9), (0.13, 15.6), (0.14, 15.4),
    (0.15, 15.2), (0.16, 14.9), (0.17, 14.7), (0.18, 14.4), (0.19, 14.2),
    (0.20, 14.0), (0.21, 13.7), (0.22, 13.5), (0.23, 13.2), (0.24, 13.0),
    (0.25, 12.8), (0.26, 12.5), (0.27, 12.3), (0.28, 12.1), (0.29, 11.8),
    (0.30, 11.6), (0.31, 11.4), (0.32, 11.2), (0.33, 10.9), (0.34, 10.7),
    (0.35, 10.5), (0.36, 10.3), (0.37, 10.0), (0.38, 9.8), (0.39, 9.6),
    (0.40, 9.4),
];

/// `f_0` 가 최대(16,5)가 되는 γ — 비단조 굴곡점.
pub const F0_PEAK_GAMMA: f64 = 0.09;

/// ISO 76 Table 1 보간. 표 밖(γ ∉ [0, 0,40])이면 **오류**다.
///
/// 규격이 값을 주지 않은 곳에 외삽값을 지어내지 않는다.
pub fn f_0(gamma: f64) -> Result<f64, SolverError> {
    util::interpolate_linear_table(&F0_TABLE, gamma).ok_or_else(|| {
        SolverError::InvalidGeometry(format!(
            "γ = {gamma:.6} 가 ISO 76 Table 1 의 정의역 [0, 0,40] 밖입니다 — \
             규격이 값을 주지 않아 f_0 를 외삽하지 않습니다"
        ))
    })
}

// ═══════════════════════════════════════════════════════════════════
//  C_0r — ISO 76 식 (1)
// ═══════════════════════════════════════════════════════════════════

/// 열 수 i. 단열 고정 (범위: 단열 레이디얼 볼만).
pub const ROW_COUNT: f64 = 1.0;

/// 기본 정정격 반경하중 `C_0r` [N] — (76-1).
///
/// `alpha_nom_rad` 는 **공칭** 접촉각이다.
pub fn basic_static_radial_load_rating_n(
    f_0_value: f64,
    z: u32,
    d_w_mm: f64,
    alpha_nom_rad: f64,
) -> f64 {
    f_0_value * ROW_COUNT * f64::from(z) * d_w_mm * d_w_mm * alpha_nom_rad.cos()
}

// ═══════════════════════════════════════════════════════════════════
//  X_0 · Y_0 — ISO 76 Table 2 (단열)
// ═══════════════════════════════════════════════════════════════════

/// 레이디얼 접촉(α = 0) 행의 `X_0`. **각접촉 행과 보간하지 않는다** — 불연속이다.
pub const X0_RADIAL_CONTACT: f64 = 0.6;
/// 레이디얼 접촉(α = 0) 행의 `Y_0`.
pub const Y0_RADIAL_CONTACT: f64 = 0.5;
/// 각접촉 구간(5°~45°)의 `X_0` — **상수 0,5**.
pub const X0_ANGULAR_CONTACT: f64 = 0.5;

/// ISO 76 Table 2 의 각접촉 `Y_0` — (접촉각 [°], `Y_0`).
///
/// **`Y_0` 만** 접촉각으로 선형보간한다 (ISO 76 §5.2.1).
/// 표의 진입 변수가 도(°)이므로 격자도 도로 보관한다 — 물리량이 아니라 **표의 키**다.
pub const Y0_ANGULAR_TABLE_DEG: [(f64, f64); 9] = [
    (5.0, 0.52), (10.0, 0.50), (15.0, 0.46), (20.0, 0.42), (25.0, 0.38),
    (30.0, 0.33), (35.0, 0.29), (40.0, 0.26), (45.0, 0.22),
];

/// 각접촉 `Y_0` 표의 하한 [°].
pub const Y0_ANGULAR_MIN_DEG: f64 = 5.0;
/// 각접촉 `Y_0` 표의 상한 [°].
pub const Y0_ANGULAR_MAX_DEG: f64 = 45.0;

/// `(X_0, Y_0)` — ISO 76 Table 2 (단열).
///
/// - α = 0 → 레이디얼 접촉 행 (0,6 · 0,5) 을 **그대로** 쓴다.
/// - α ∈ [5°, 45°] → `X_0 = 0,5` 고정, `Y_0` 만 선형보간.
/// - 0 < α < 5° 및 α > 45° → 표 밖이라 **오류**.
///
/// ⚠️ α = 0 과 α = 5° 사이를 **보간하지 않는다.** `X_0` 가 0,6 → 0,5 로 불연속이고
///    ISO 는 「각접촉 중간각」만 보간하라고 한다 (Theory §7.12).
pub fn x0_y0(alpha_nom_rad: f64) -> Result<(f64, f64), SolverError> {
    if alpha_nom_rad == 0.0 {
        return Ok((X0_RADIAL_CONTACT, Y0_RADIAL_CONTACT));
    }
    let alpha_deg = alpha_nom_rad.to_degrees();
    let y_0 = util::interpolate_linear_table(&Y0_ANGULAR_TABLE_DEG, alpha_deg).ok_or_else(|| {
        SolverError::InvalidGeometry(format!(
            "공칭 접촉각 {alpha_deg:.3}° 가 ISO 76 Table 2 각접촉 구간 \
             [{Y0_ANGULAR_MIN_DEG}°, {Y0_ANGULAR_MAX_DEG}°] 밖입니다. \
             α = 0 은 레이디얼 접촉 행(X_0 = 0,6)으로 별도 처리되며, \
             그 사이 구간은 X_0 가 불연속이라 규격이 보간을 허용하지 않습니다"
        ))
    })?;
    Ok((X0_ANGULAR_CONTACT, y_0))
}

// ═══════════════════════════════════════════════════════════════════
//  P_0r · S_0
// ═══════════════════════════════════════════════════════════════════

/// 정적 등가 반경하중 `P_0r` [N] — (76-2)(76-3) 의 **큰 쪽**.
pub fn static_equivalent_radial_load_n(x_0: f64, y_0: f64, f_r_n: f64, f_a_n: f64) -> f64 {
    (x_0 * f_r_n + y_0 * f_a_n).max(f_r_n)
}

/// 정적 안전계수 `S_0` — (76-14). `P_0r = 0` 이면 +∞.
pub fn static_safety_factor(c_0r_n: f64, p_0r_n: f64) -> f64 {
    if p_0r_n <= 0.0 {
        f64::INFINITY
    } else {
        c_0r_n / p_0r_n
    }
}

/// ISO 76 Table 4 — 충격하중 시 권장 최소 `S_0` (각주 a: 하중 크기 미상이면 1,5).
pub const S0_GUIDE_SHOCK: f64 = 1.5;
/// ISO 76 Table 4 — 보통 운전 권장 최소 `S_0`.
pub const S0_GUIDE_NORMAL: f64 = 1.0;

// ═══════════════════════════════════════════════════════════════════
//  종합
// ═══════════════════════════════════════════════════════════════════

/// ISO 76 정정격 일괄 계산 (Theory §7.11~7.13).
///
/// `c_0r_override_n` 이 `Some` 이면 (카탈로그 값 등) 그것을 `C_0r` 로 쓴다.
/// `f_0` 와 자체 산출값은 그래도 결과에 실어 보낸다 (검산용).
pub fn compute_static_rating(
    geom: &BallBearingGeometry,
    derived: &BbGeometryDerived,
    operating: &BbOperatingConditions,
    c_0r_override_n: Option<f64>,
) -> Result<BbStaticRatingResult, SolverError> {
    let f_0_value = f_0(derived.gamma)?;
    let (x_0, y_0) = x0_y0(geom.alpha_nom_rad)?;

    let c_0r_computed_n =
        basic_static_radial_load_rating_n(f_0_value, geom.z, geom.d_w_mm, geom.alpha_nom_rad);
    let c_0r_n = c_0r_override_n.unwrap_or(c_0r_computed_n);

    let f_r_n = operating.radial_magnitude();
    let f_a_n = operating.f_x_n.abs();
    let p_0r_n = static_equivalent_radial_load_n(x_0, y_0, f_r_n, f_a_n);
    let s_0 = static_safety_factor(c_0r_n, p_0r_n);

    let mut alerts = Vec::new();

    // ISO 76 §5.1.1 의 적용 조건 — 홈 반경이 기준을 넘으면 f_0 를 감소시켜야 한다.
    // 감소식은 ISO/TR 10657 에 있으나 P5-1 범위 밖이라 경고로 대체한다 (Theory §7.11).
    let f_i = geom.r_i_mm / geom.d_w_mm;
    let f_e = geom.r_e_mm / geom.d_w_mm;
    if f_i > 0.52 || f_e > 0.53 {
        alerts.push(Alert {
            level: AlertLevel::Warning,
            code: "STATIC_RATING_GROOVE_OVER_REFERENCE".into(),
            message: format!(
                "홈 반경비가 ISO 76 §5.1.1 적용 조건(f_i ≤ 0,52 / f_e ≤ 0,53)을 초과합니다 \
                 (f_i = {f_i:.4}, f_e = {f_e:.4}). 규격은 f_0 를 감소시키라고 하며 \
                 감소식은 ISO/TR 10657 에 있으나 본 단계에서는 미구현입니다 — C_0r 이 과대평가됩니다"
            ),
        });
    }

    if s_0 < S0_GUIDE_NORMAL {
        alerts.push(Alert {
            level: AlertLevel::Critical,
            code: "STATIC_SAFETY_FACTOR_LOW".into(),
            message: format!(
                "정적 안전계수 S_0 = {s_0:.2} 가 ISO 76 Table 4 의 보통 운전 지침값 \
                 {S0_GUIDE_NORMAL} 미만입니다 — 궤도륜에 영구변형이 생길 수 있습니다"
            ),
        });
    } else if s_0 < S0_GUIDE_SHOCK {
        alerts.push(Alert {
            level: AlertLevel::Warning,
            code: "STATIC_SAFETY_FACTOR_BELOW_SHOCK_GUIDE".into(),
            message: format!(
                "정적 안전계수 S_0 = {s_0:.2} 가 ISO 76 Table 4 의 충격하중 지침값 \
                 {S0_GUIDE_SHOCK} 미만입니다 (각주 a: 하중 크기를 모르면 최소 1,5)"
            ),
        });
    }

    Ok(BbStaticRatingResult {
        f_0: f_0_value,
        gamma: derived.gamma,
        c_0r_n,
        c_0r_computed_n,
        x_0,
        y_0,
        f_r_n,
        f_a_n,
        p_0r_n,
        s_0,
        alerts,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f0_table_grid_points_are_reproduced() {
        for (gamma, expected) in F0_TABLE {
            assert!((f_0(gamma).unwrap() - expected).abs() < 1e-12, "γ={gamma}");
        }
    }

    #[test]
    fn f0_peak_is_at_gamma_0_09() {
        let peak = f_0(F0_PEAK_GAMMA).unwrap();
        assert!((peak - 16.5).abs() < 1e-12);
        assert!(f_0(0.08).unwrap() < peak);
        assert!(f_0(0.10).unwrap() < peak);
    }

    #[test]
    fn f0_rejects_outside_table() {
        assert!(f_0(-1e-9).is_err());
        assert!(f_0(0.4 + 1e-9).is_err());
    }

    #[test]
    fn x0_y0_radial_contact_row_is_not_blended() {
        let (x, y) = x0_y0(0.0).unwrap();
        assert!((x - 0.6).abs() < 1e-15);
        assert!((y - 0.5).abs() < 1e-15);
        // 5° 바로 위는 각접촉 행
        let (x5, y5) = x0_y0(5.0_f64.to_radians()).unwrap();
        assert!((x5 - 0.5).abs() < 1e-15);
        assert!((y5 - 0.52).abs() < 1e-12);
        // 그 사이는 규격이 값을 주지 않는다
        assert!(x0_y0(2.5_f64.to_radians()).is_err());
        assert!(x0_y0(50.0_f64.to_radians()).is_err());
    }

    #[test]
    fn p0r_takes_the_larger_branch() {
        // 순수 반경하중이면 (76-3) 이 이긴다: 0,5·F_r < F_r
        assert!((static_equivalent_radial_load_n(0.5, 0.26, 1000.0, 0.0) - 1000.0).abs() < 1e-12);
        // 축하중이 크면 (76-2) 가 이긴다
        let p = static_equivalent_radial_load_n(0.5, 0.26, 1000.0, 10_000.0);
        assert!((p - (500.0 + 2600.0)).abs() < 1e-12);
    }

    #[test]
    fn safety_factor_is_infinite_without_load() {
        assert!(static_safety_factor(1000.0, 0.0).is_infinite());
    }
}
