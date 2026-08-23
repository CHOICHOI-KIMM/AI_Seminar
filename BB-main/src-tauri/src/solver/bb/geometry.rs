// BB Contact Analysis — ACBB 기하 전처리
//
// BB Phase 1-S3 (2026-08-20): CRB(슬라이스·프로파일) 기하를 백지에서 재작성.
//
// ── 근거 ────────────────────────────────────────────────────────────
// BB_Development_Theory.md §2. 식 번호는 ISO 16281:2025 기준.
//   (A.1) α₀ = arccos(1 − G_r op /(2A))
//   (A.3) A  = r_i + r_e − D_w
//   (A.4) R_i = D_pw/2 + (r_i − D_w/2) cos α₀
//   (E.4) Σρ_i, (E.5) Σρ_e, (E.6) F_i(ρ), (E.7) F_e(ρ)
//
// ── 성격 ────────────────────────────────────────────────────────────
// 여기 계산되는 값은 **전부 하중과 무관**하다 (Theory §8 1단계).
// 해석 시작 시 1회 계산해 캐시하면 이후 반복 비용은 O(Z) 로 끝난다.
//
// ── 단위 (D-10) ─────────────────────────────────────────────────────
// mm · N · rad. 이 파일에 단위 환산 상수는 없다.
// 볼 질량의 mm³→cm³ 환산만 `util::sphere_mass_g` 에 격리되어 있다.

use crate::error::SolverError;
use crate::solver::bb::types::*;
use crate::solver::common::types::{Alert, AlertLevel, Material};
use crate::solver::common::util;

/// 입력의 `ClearanceSpec` 을 (등가 직경 클리어런스 [mm], 초기 접촉각 [rad]) 로 환산.
///
/// 식 (A.1) 과 그 역: `G_r op = 2A (1 − cos α₀)`
fn resolve_clearance(
    clearance: ClearanceSpec,
    a_mm: f64,
) -> Result<(f64, f64), SolverError> {
    match clearance {
        ClearanceSpec::DiametralMm(g) => {
            // (A.1) 은 G_r op ≥ 0 에서만 정의된다. G < 0 이면 arccos 인수가 1 을
            // 넘어 정의역을 벗어난다 — ISO 16281 은 음의 클리어런스(예압)를
            // 클리어런스로 표현하지 않는다. ACBB 의 예압은 축방향 하중으로 준다.
            if g < 0.0 {
                return Err(SolverError::InvalidInput(format!(
                    "음의 운전 클리어런스({g} mm)는 식 (A.1) 의 정의역 밖입니다.                      ACBB 예압은 ClearanceSpec::AxialPreloadN 으로 지정하십시오 (P3 지원)"
                )));
            }
            let arg = 1.0 - g / (2.0 * a_mm);
            if !(-1.0..=1.0).contains(&arg) {
                return Err(SolverError::InvalidGeometry(format!(
                    "운전 클리어런스 G_r op = {g} mm 가 A = {a_mm} mm 에 비해 과대합니다 \
                     (식 A.1 의 arccos 인수 {arg} 가 [−1, 1] 밖)"
                )));
            }
            Ok((g, arg.acos()))
        }
        ClearanceSpec::InitialAngleRad(alpha_0) => {
            // (A.1) 역산
            let g = 2.0 * a_mm * (1.0 - alpha_0.cos());
            Ok((g, alpha_0))
        }
        ClearanceSpec::AxialPreloadN(_) => Err(SolverError::InvalidInput(
            "축방향 예압(F_a0) → α₀ 역산은 P3 평형 솔버에서 지원합니다. \
             P1 단계에서는 DiametralMm 또는 InitialAngleRad 로 지정하십시오"
                .into(),
        )),
    }
}

/// 하중 무관 기하 전처리 (Theory §2).
pub fn compute_geometry_derived(
    geom: &BallBearingGeometry,
) -> Result<GeometryDerived, SolverError> {
    geom.validate()?;

    let d_w = geom.d_w_mm;
    let d_pw = geom.d_pw_mm;
    let r_i = geom.r_i_mm;
    let r_e = geom.r_e_mm;

    // (A.3) 곡률중심 간 거리
    let a_mm = r_i + r_e - d_w;

    // (A.1) 초기 접촉각 + 등가 클리어런스
    let (g_r_op_mm, alpha_0_rad) = resolve_clearance(geom.clearance, a_mm)?;

    // (A.4) 내륜 홈 곡률중심 반경 — **틸트 모멘트 팔** (D-9)
    let r_i_center_mm = d_pw / 2.0 + (r_i - d_w / 2.0) * alpha_0_rad.cos();

    // Clause 4: γ = D_w cos α / D_pw  (공칭 접촉각 α 사용, T-2 참조)
    let gamma = d_w * geom.alpha_nom_rad.cos() / d_pw;
    if gamma >= 1.0 {
        return Err(SolverError::InvalidGeometry(
            "γ = D_w cos α / D_pw 가 1 이상입니다 (식 E.4/E.6 의 1−γ 가 0 이하)".into(),
        ));
    }

    // (E.4)(E.5) 곡률합 [1/mm]
    let ki = gamma / (1.0 - gamma);
    let ke = gamma / (1.0 + gamma);
    let ci = d_w / (2.0 * r_i);
    let ce = d_w / (2.0 * r_e);

    let den_i = 2.0 + ki - ci;
    let den_e = 2.0 - ke - ce;
    if den_i <= 0.0 || den_e <= 0.0 {
        return Err(SolverError::InvalidGeometry(
            "곡률합의 분모가 0 이하입니다 — 홈 반경이 볼 반경에 지나치게 가깝습니다".into(),
        ));
    }

    let sum_rho_i_per_mm = 2.0 / d_w * den_i;
    let sum_rho_e_per_mm = 2.0 / d_w * den_e;

    // (E.6)(E.7) 상대 곡률차. (E.7) 분자 첫 항은 음수임에 주의.
    let f_rho_i = (ki + ci) / den_i;
    let f_rho_e = (-ke + ce) / den_e;

    Ok(GeometryDerived {
        a_mm,
        alpha_0_rad,
        r_i_center_mm,
        gamma,
        sum_rho_i_per_mm,
        sum_rho_e_per_mm,
        f_rho_i,
        f_rho_e,
        g_r_op_mm,
    })
}

/// UI 표시·검산용 기하 요약.
///
/// `n·D_pw` 는 ISO 16281 A.4 의 고속 판정 지표다 (D-3).
/// 1×10⁶ mm/min 을 넘으면 정적 평형 가정 범위 밖이다.
pub fn compute_geometry_summary(
    geom: &BallBearingGeometry,
    derived: &GeometryDerived,
    operating: &OperatingConditions,
    material: &Material,
) -> GeometrySummary {
    GeometrySummary {
        a_mm: derived.a_mm,
        alpha_0_rad: derived.alpha_0_rad,
        r_i_center_mm: derived.r_i_center_mm,
        gamma: derived.gamma,
        sum_rho_i_per_mm: derived.sum_rho_i_per_mm,
        sum_rho_e_per_mm: derived.sum_rho_e_per_mm,
        f_rho_i: derived.f_rho_i,
        f_rho_e: derived.f_rho_e,
        g_r_op_mm: derived.g_r_op_mm,
        osculation_inner: geom.r_i_mm / geom.d_w_mm,
        osculation_outer: geom.r_e_mm / geom.d_w_mm,
        ball_mass_g: util::sphere_mass_g(geom.d_w_mm, material.density_ball_g_cm3),
        n_dpw_mm_per_min: operating.relative_speed_rpm().abs() * geom.d_pw_mm,
    }
}

/// ISO 16281 A.4 고속 판정 임계값 [mm/min] (D-3).
pub const N_DPW_STATIC_LIMIT: f64 = 1.0e6;

/// 기하·운전조건에서 나오는 경고를 수집한다.
pub fn collect_geometry_alerts(summary: &GeometrySummary) -> Vec<Alert> {
    let mut alerts = Vec::new();

    if summary.n_dpw_mm_per_min > N_DPW_STATIC_LIMIT {
        alerts.push(Alert {
            level: AlertLevel::Warning,
            code: "HIGH_SPEED".into(),
            message: format!(
                "n·D_pw = {:.3e} mm/min 이 ISO 16281 A.4 의 정적 가정 한계 1e6 을 초과합니다. \
                 원심력·자이로 모멘트가 하중분포를 바꿀 수 있으나 본 해석은 정적 평형만 다룹니다",
                summary.n_dpw_mm_per_min
            ),
        });
    }

    // Annex B.2 참조기하에서 크게 벗어난 홈 반경은 ISO 281 f_c 표의 전제를 깬다 (T-9)
    if summary.osculation_inner > 0.52 || summary.osculation_outer > 0.53 {
        alerts.push(Alert {
            level: AlertLevel::Warning,
            code: "GROOVE_RADIUS_OVER_REFERENCE".into(),
            message: format!(
                "홈 반경비가 ISO 281 §5.1.1 전제(f_i ≤ 0,52 / f_e ≤ 0,53)를 초과합니다 \
                 (f_i = {:.4}, f_e = {:.4}). f_c 감소 보정이 필요하나 ISO/TR 8646 미확보 상태입니다 (T-9)",
                summary.osculation_inner, summary.osculation_outer
            ),
        });
    }

    alerts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> BallBearingGeometry {
        let d_w_mm = 11.5;
        let (r_i_mm, r_e_mm) = BallBearingGeometry::reference_groove_radii(d_w_mm);
        BallBearingGeometry {
            bore_mm: 50.0,
            outer_diameter_mm: 90.0,
            width_mm: 20.0,
            z: 16,
            d_w_mm,
            d_pw_mm: 70.0,
            r_i_mm,
            r_e_mm,
            alpha_nom_rad: 40.0_f64.to_radians(),
            clearance: ClearanceSpec::InitialAngleRad(40.0_f64.to_radians()),
        }
    }

    #[test]
    fn axial_preload_is_rejected_with_explicit_message() {
        let mut g = fixture();
        g.clearance = ClearanceSpec::AxialPreloadN(500.0);
        let err = compute_geometry_derived(&g).unwrap_err();
        assert!(format!("{err}").contains("P3"));
    }

    #[test]
    fn oversized_clearance_is_rejected() {
        let mut g = fixture();
        // A = 0.05 D_w = 0.575 mm → G_r op > 4A 면 arccos 인수가 −1 미만
        g.clearance = ClearanceSpec::DiametralMm(10.0);
        assert!(compute_geometry_derived(&g).is_err());
    }

    #[test]
    fn gamma_uses_nominal_contact_angle() {
        let g = fixture();
        let d = compute_geometry_derived(&g).unwrap();
        let expected = g.d_w_mm * g.alpha_nom_rad.cos() / g.d_pw_mm;
        assert!((d.gamma - expected).abs() < 1e-15);
    }

    #[test]
    fn summary_osculation_and_mass() {
        let g = fixture();
        let d = compute_geometry_derived(&g).unwrap();
        let op = OperatingConditions {
            f_x_n: 0.0,
            f_y_n: 0.0,
            f_z_n: 0.0,
            m_y_nmm: 0.0,
            m_z_nmm: 0.0,
            n_inner_rpm: 1500.0,
            n_outer_rpm: 0.0,
            temperature_c: 70.0,
        };
        let s = compute_geometry_summary(&g, &d, &op, &Material::default());
        assert!((s.osculation_inner - 0.52).abs() < 1e-12);
        assert!((s.osculation_outer - 0.53).abs() < 1e-12);
        assert!((s.n_dpw_mm_per_min - 1500.0 * 70.0).abs() < 1e-9);
        assert!(s.ball_mass_g > 6.0 && s.ball_mass_g < 6.5);
    }

    #[test]
    fn high_speed_alert_fires_only_above_limit() {
        let g = fixture();
        let d = compute_geometry_derived(&g).unwrap();
        let mk = |rpm: f64| OperatingConditions {
            f_x_n: 0.0,
            f_y_n: 0.0,
            f_z_n: 0.0,
            m_y_nmm: 0.0,
            m_z_nmm: 0.0,
            n_inner_rpm: rpm,
            n_outer_rpm: 0.0,
            temperature_c: 70.0,
        };
        // n·D_pw = rpm × 70. 한계 1e6 → rpm = 14286
        let below = compute_geometry_summary(&g, &d, &mk(14_000.0), &Material::default());
        let above = compute_geometry_summary(&g, &d, &mk(15_000.0), &Material::default());
        assert!(!collect_geometry_alerts(&below)
            .iter()
            .any(|a| a.code == "HIGH_SPEED"));
        assert!(collect_geometry_alerts(&above)
            .iter()
            .any(|a| a.code == "HIGH_SPEED"));
    }

    #[test]
    fn negative_diametral_clearance_is_rejected() {
        // 식 (A.1) 은 G_r op ≥ 0 에서만 정의된다 (arccos 인수 ≤ 1).
        // ACBB 예압은 클리어런스가 아니라 축방향 하중으로 준다.
        let mut g = fixture();
        g.clearance = ClearanceSpec::DiametralMm(-0.02);
        let err = compute_geometry_derived(&g).unwrap_err();
        assert!(format!("{err}").contains("AxialPreloadN"));
    }
}
