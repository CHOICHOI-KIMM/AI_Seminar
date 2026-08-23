// Level A 검증 — ACBB 기하 (Plan §5, Phase 1)
//
// 판정 대상: BB_Development_Theory.md §2 의 식 (A.1)(A.3)(A.4)(E.4)~(E.7).
// 실행: cargo test --test geometry_level_a
//
// ── 검증 철학 ───────────────────────────────────────────────────────
// 구현식을 그대로 베껴 비교하면 동어반복(tautology)이 된다.
// 곡률합·곡률차는 **ISO 축약형이 아니라 개별 주곡률로부터 조립**하여
// 독립 경로로 대조한다 (Harris Ch.6 의 Σρ = Σ(ρ_1x + ρ_1y + ρ_2x + ρ_2y) 방식).

use bb_core::solver::bb::geometry::{
    collect_geometry_alerts, compute_geometry_derived, compute_geometry_summary,
};
use bb_core::solver::bb::types::*;
use bb_core::solver::common::types::*;

const D_W: f64 = 11.5;
const D_PW: f64 = 70.0;
const ALPHA_DEG: f64 = 40.0;

fn fixture() -> BallBearingGeometry {
    let (r_i_mm, r_e_mm) = BallBearingGeometry::reference_groove_radii(D_W);
    BallBearingGeometry {
        bore_mm: 50.0,
        outer_diameter_mm: 90.0,
        width_mm: 20.0,
        z: 16,
        d_w_mm: D_W,
        d_pw_mm: D_PW,
        r_i_mm,
        r_e_mm,
        alpha_nom_rad: ALPHA_DEG.to_radians(),
        clearance: BbClearanceSpec::InitialAngleRad(ALPHA_DEG.to_radians()),
    }
}

fn operating(rpm: f64) -> BbOperatingConditions {
    BbOperatingConditions {
        f_x_n: 0.0,
        f_y_n: 0.0,
        f_z_n: 0.0,
        m_y_nmm: 0.0,
        m_z_nmm: 0.0,
        n_inner_rpm: rpm,
        n_outer_rpm: 0.0,
        temperature_c: 70.0,
    }
}

fn rel(a: f64, b: f64) -> f64 {
    ((a - b) / b).abs()
}

// ═══════════════════════════════════════════════════════════════════
//  A-1. 식 (A.3) — 곡률중심 간 거리 A
// ═══════════════════════════════════════════════════════════════════

#[test]
fn a1_curvature_center_distance() {
    let g = fixture();
    let d = compute_geometry_derived(&g).unwrap();
    // Annex B.2 참조기하: A = (0,52 + 0,53 − 1) D_w = 0,05 D_w
    // 이 항등식은 구현식과 무관하게 성립한다.
    assert!(rel(d.a_mm, 0.05 * D_W) < 1e-12, "A = {}", d.a_mm);
}

// ═══════════════════════════════════════════════════════════════════
//  A-2. 식 (A.1) — 초기 접촉각 왕복 항등
// ═══════════════════════════════════════════════════════════════════

#[test]
fn a2_initial_angle_roundtrip() {
    // α₀ → G_r op → α₀ 가 항등이어야 한다.
    for deg in [0.0_f64, 5.0, 15.0, 25.0, 40.0, 60.0] {
        let mut g = fixture();
        g.alpha_nom_rad = deg.to_radians();
        g.clearance = BbClearanceSpec::InitialAngleRad(deg.to_radians());
        let d1 = compute_geometry_derived(&g).unwrap();

        g.clearance = BbClearanceSpec::DiametralMm(d1.g_r_op_mm);
        let d2 = compute_geometry_derived(&g).unwrap();

        assert!(
            (d2.alpha_0_rad - deg.to_radians()).abs() < 1e-12,
            "α₀ = {deg}° 왕복 실패: {} rad",
            d2.alpha_0_rad
        );
    }
}

#[test]
fn a2b_clearance_to_angle_is_monotonic() {
    // G_r op 가 커지면 α₀ 도 커진다 (식 A.1 의 arccos 이 단조감소이므로).
    let mut prev = -1.0;
    for g_mm in [0.0_f64, 0.05, 0.10, 0.20, 0.30, 0.50] {
        let mut geom = fixture();
        geom.clearance = BbClearanceSpec::DiametralMm(g_mm);
        let d = compute_geometry_derived(&geom).unwrap();
        assert!(
            d.alpha_0_rad > prev,
            "G = {g_mm} mm 에서 α₀ 단조성 위반"
        );
        prev = d.alpha_0_rad;
    }
}

#[test]
fn a2c_zero_clearance_gives_zero_angle() {
    let mut g = fixture();
    g.clearance = BbClearanceSpec::DiametralMm(0.0);
    let d = compute_geometry_derived(&g).unwrap();
    assert!(d.alpha_0_rad.abs() < 1e-12);
}

// ═══════════════════════════════════════════════════════════════════
//  A-3. 식 (A.4) — 틸트 모멘트 팔 R_i  (D-9)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn a3_tilt_arm_exceeds_pitch_radius() {
    // R_i = D_pw/2 + (r_i − D_w/2) cos α₀ 이고 r_i > D_w/2 이므로
    // 항상 R_i > D_pw/2 여야 한다. CRB 가 쓰던 d_pw/2 는 과소평가다.
    let g = fixture();
    let d = compute_geometry_derived(&g).unwrap();
    assert!(
        d.r_i_center_mm > D_PW / 2.0,
        "R_i = {} 가 D_pw/2 = {} 이하",
        d.r_i_center_mm,
        D_PW / 2.0
    );
    // 참조기하 α₀ = 40° 에서의 과소평가량
    let underestimate = d.r_i_center_mm - D_PW / 2.0;
    assert!(
        underestimate > 0.1,
        "과소평가량 {underestimate} mm 가 예상보다 작음"
    );
}

#[test]
fn a3b_tilt_arm_decreases_with_contact_angle() {
    // cos α₀ 가 줄어들므로 α₀ 가 커지면 R_i 는 감소한다.
    let mut prev = f64::INFINITY;
    for deg in [0.0_f64, 10.0, 20.0, 30.0, 40.0, 50.0] {
        let mut g = fixture();
        g.clearance = BbClearanceSpec::InitialAngleRad(deg.to_radians());
        let d = compute_geometry_derived(&g).unwrap();
        assert!(d.r_i_center_mm < prev, "α₀ = {deg}° 에서 R_i 단조성 위반");
        prev = d.r_i_center_mm;
    }
}

// ═══════════════════════════════════════════════════════════════════
//  A-4. 식 (E.4)~(E.7) — 곡률합·곡률차 (독립 조립 경로 대조)
// ═══════════════════════════════════════════════════════════════════

/// 개별 주곡률로부터 Σρ 와 F(ρ) 를 조립한다 (ISO 축약형을 쓰지 않는다).
///
/// 볼:            ρ_1x = ρ_1y = 2 / D_w
/// 내륜 레이스웨이: ρ_2x = +2γ / (D_w (1−γ)),  ρ_2y = −1 / r_i
/// 외륜 레이스웨이: ρ_2x = −2γ / (D_w (1+γ)),  ρ_2y = −1 / r_e
///
/// Σρ  = ρ_1x + ρ_1y + ρ_2x + ρ_2y
/// F(ρ) = ((ρ_1x − ρ_1y) + (ρ_2x − ρ_2y)) / Σρ    (ρ_1x = ρ_1y 이므로 첫 항은 0)
fn assemble_from_principal_curvatures(
    d_w: f64,
    gamma: f64,
    r_race: f64,
    inner: bool,
) -> (f64, f64) {
    let rho_1x = 2.0 / d_w;
    let rho_1y = 2.0 / d_w;
    let rho_2x = if inner {
        2.0 * gamma / (d_w * (1.0 - gamma))
    } else {
        -2.0 * gamma / (d_w * (1.0 + gamma))
    };
    let rho_2y = -1.0 / r_race;
    let sum = rho_1x + rho_1y + rho_2x + rho_2y;
    let f_rho = ((rho_1x - rho_1y) + (rho_2x - rho_2y)) / sum;
    (sum, f_rho)
}

#[test]
fn a4_curvature_sums_match_independent_assembly() {
    let g = fixture();
    let d = compute_geometry_derived(&g).unwrap();

    let (sum_i, f_i) = assemble_from_principal_curvatures(D_W, d.gamma, g.r_i_mm, true);
    let (sum_e, f_e) = assemble_from_principal_curvatures(D_W, d.gamma, g.r_e_mm, false);

    assert!(
        rel(d.sum_rho_i_per_mm, sum_i) < 1e-12,
        "Σρ_i: ISO {} vs 조립 {}",
        d.sum_rho_i_per_mm,
        sum_i
    );
    assert!(
        rel(d.sum_rho_e_per_mm, sum_e) < 1e-12,
        "Σρ_e: ISO {} vs 조립 {}",
        d.sum_rho_e_per_mm,
        sum_e
    );
    assert!(rel(d.f_rho_i, f_i) < 1e-12, "F_i: ISO {} vs 조립 {}", d.f_rho_i, f_i);
    assert!(rel(d.f_rho_e, f_e) < 1e-12, "F_e: ISO {} vs 조립 {}", d.f_rho_e, f_e);
}

#[test]
fn a4b_curvature_sums_are_positive_and_inner_dominates() {
    // 내륜 접촉은 볼록–볼록에 가까워 곡률합이 더 크다 (더 가혹한 접촉).
    let g = fixture();
    let d = compute_geometry_derived(&g).unwrap();
    assert!(d.sum_rho_i_per_mm > 0.0);
    assert!(d.sum_rho_e_per_mm > 0.0);
    assert!(
        d.sum_rho_i_per_mm > d.sum_rho_e_per_mm,
        "Σρ_i {} 가 Σρ_e {} 이하",
        d.sum_rho_i_per_mm,
        d.sum_rho_e_per_mm
    );
}

#[test]
fn a4c_relative_curvature_difference_in_unit_interval() {
    // F(ρ) 는 식 (E.1) 의 입력이며 [0, 1) 이어야 χ 가 유한하다.
    for deg in [0.0_f64, 15.0, 30.0, 40.0, 55.0] {
        let mut g = fixture();
        g.alpha_nom_rad = deg.to_radians();
        g.clearance = BbClearanceSpec::InitialAngleRad(deg.to_radians());
        let d = compute_geometry_derived(&g).unwrap();
        assert!(
            (0.0..1.0).contains(&d.f_rho_i),
            "α = {deg}° 에서 F_i = {}",
            d.f_rho_i
        );
        assert!(
            (0.0..1.0).contains(&d.f_rho_e),
            "α = {deg}° 에서 F_e = {}",
            d.f_rho_e
        );
    }
}

#[test]
fn a4d_curvature_sum_has_reciprocal_length_dimension() {
    // D_w 를 k 배 하면 Σρ 는 1/k 배가 되어야 한다 (차원 검사).
    let mut g1 = fixture();
    let mut g2 = fixture();
    let k = 2.0;
    g2.d_w_mm = D_W * k;
    let (ri2, re2) = BallBearingGeometry::reference_groove_radii(g2.d_w_mm);
    g2.r_i_mm = ri2;
    g2.r_e_mm = re2;
    g2.d_pw_mm = D_PW * k; // γ 를 보존하려면 D_pw 도 같은 비율로
    g1.d_pw_mm = D_PW;

    let d1 = compute_geometry_derived(&g1).unwrap();
    let d2 = compute_geometry_derived(&g2).unwrap();

    assert!(rel(d2.gamma, d1.gamma) < 1e-12, "γ 가 보존되지 않음");
    assert!(
        rel(d2.sum_rho_i_per_mm, d1.sum_rho_i_per_mm / k) < 1e-12,
        "Σρ_i 차원 불일치"
    );
    // F(ρ) 는 무차원이므로 불변
    assert!(rel(d2.f_rho_i, d1.f_rho_i) < 1e-12, "F_i 가 무차원이 아님");
}

// ═══════════════════════════════════════════════════════════════════
//  A-5. Annex B.2 참조 기하
// ═══════════════════════════════════════════════════════════════════

#[test]
fn a5_reference_groove_radii() {
    let (r_i, r_e) = BallBearingGeometry::reference_groove_radii(D_W);
    assert!(rel(r_i, 0.52 * D_W) < 1e-12);
    assert!(rel(r_e, 0.53 * D_W) < 1e-12);

    let g = fixture();
    let d = compute_geometry_derived(&g).unwrap();
    let op = operating(1500.0);
    let s = compute_geometry_summary(&g, &d, &op, &Material::default());
    assert!(rel(s.osculation_inner, 0.52) < 1e-12);
    assert!(rel(s.osculation_outer, 0.53) < 1e-12);
}

// ═══════════════════════════════════════════════════════════════════
//  A-6. 입력 검증 · 정의역
// ═══════════════════════════════════════════════════════════════════

#[test]
fn a6_domain_guards() {
    // 예압은 P3 소관 — 조용히 0 으로 처리하지 않고 명시적으로 거부
    let mut g = fixture();
    g.clearance = BbClearanceSpec::AxialPreloadN(500.0);
    assert!(compute_geometry_derived(&g).is_err());

    // 음의 클리어런스는 식 (A.1) 정의역 밖
    let mut g = fixture();
    g.clearance = BbClearanceSpec::DiametralMm(-0.01);
    assert!(compute_geometry_derived(&g).is_err());

    // 과대 클리어런스도 정의역 밖
    let mut g = fixture();
    g.clearance = BbClearanceSpec::DiametralMm(100.0);
    assert!(compute_geometry_derived(&g).is_err());

    // 홈 반경이 볼 반경 이하면 접촉이 성립하지 않음
    let mut g = fixture();
    g.r_i_mm = D_W / 2.0;
    assert!(compute_geometry_derived(&g).is_err());
}

// ═══════════════════════════════════════════════════════════════════
//  A-7. 고속 판정 (D-3) · 홈반경 경고 (T-9)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn a7_high_speed_alert_boundary() {
    let g = fixture();
    let d = compute_geometry_derived(&g).unwrap();
    // n·D_pw = rpm × 70 → 한계 1e6 은 rpm ≈ 14 285.7
    let below = compute_geometry_summary(&g, &d, &operating(14_000.0), &Material::default());
    let above = compute_geometry_summary(&g, &d, &operating(15_000.0), &Material::default());
    assert!(!collect_geometry_alerts(&below)
        .iter()
        .any(|a| a.code == "HIGH_SPEED"));
    assert!(collect_geometry_alerts(&above)
        .iter()
        .any(|a| a.code == "HIGH_SPEED"));
}

#[test]
fn a7b_groove_radius_over_reference_alert() {
    // ISO 281 §5.1.1 은 f_i ≤ 0,52 / f_e ≤ 0,53 을 f_c 표의 전제로 둔다.
    let mut g = fixture();
    g.r_i_mm = 0.56 * D_W;
    let d = compute_geometry_derived(&g).unwrap();
    let s = compute_geometry_summary(&g, &d, &operating(1000.0), &Material::default());
    assert!(collect_geometry_alerts(&s)
        .iter()
        .any(|a| a.code == "GROOVE_RADIUS_OVER_REFERENCE"));
}

// ═══════════════════════════════════════════════════════════════════
//  A-8. D-10 단위 규약 기계 검증
// ═══════════════════════════════════════════════════════════════════

#[test]
fn a8_no_unit_conversion_constants_outside_util() {
    // Plan D-10: 솔버 내부는 mm·N·rad. 환산 상수는 util.rs 의
    // 명시적 변환 함수 안에만 존재할 수 있다.
    //
    // 판정 대상은 **연산자에 인접한** 배율 상수다 (`* 1000.0`, `/ 1e-3` 등).
    // 단순한 수치 리터럴(회전속도 1000.0 rpm 같은 값)은 환산이 아니다.
    // `#[cfg(test)]` 이후는 픽스처라 검사에서 제외한다.
    //
    // 스캔 대상 (P4-S0-2 에서 경로 갱신 · `common/types.rs` 추가):
    //   bb/{types,geometry,hertz,bearing}.rs · common/types.rs · solver/mod.rs
    //
    // ⚠ `common/util.rs` 는 **의도적으로 제외**한다. D-10 이 「환산 상수는
    //    util.rs 의 명시적 변환 함수 안에만 존재할 수 있다」고 정한 유일한
    //    허용 지점이므로, 스캔에 넣으면 규약이 허용한 것을 규약 검사가 잡는
    //    모순이 된다. 이 제외는 A-8 신설 때부터의 설계이며 유지한다.
    let sources: [(&str, &str); 6] = [
        ("bb/types.rs", include_str!("../src/solver/bb/types.rs")),
        ("bb/geometry.rs", include_str!("../src/solver/bb/geometry.rs")),
        ("bb/hertz.rs", include_str!("../src/solver/bb/hertz.rs")),
        ("bb/bearing.rs", include_str!("../src/solver/bb/bearing.rs")),
        ("common/types.rs", include_str!("../src/solver/common/types.rs")),
        ("mod.rs", include_str!("../src/solver/mod.rs")),
    ];
    let magnitudes = ["1000.0", "1_000.0", "1e3", "1e-3", "0.001"];

    for (name, src) in sources {
        let body = src.split("#[cfg(test)]").next().unwrap_or(src);
        for (lineno, line) in body.lines().enumerate() {
            let code = line.split("//").next().unwrap_or("");
            for m in magnitudes {
                for op in ['*', '/'] {
                    let after = format!("{op} {m}");
                    let before = format!("{m} {op}");
                    assert!(
                        !code.contains(&after) && !code.contains(&before),
                        "{name}:{} 에 단위 환산 연산 `{op} {m}` 발견 — D-10 위반\n  {line}",
                        lineno + 1
                    );
                }
            }
        }
    }
}

#[test]
fn a8b_dimensional_fields_carry_unit_suffix() {
    // 유차원 필드는 이름에 단위를 달아 프론트·JSON 오독을 차단한다 (P1-S3 결정).
    // 대상은 `pub <name>: f64` 형태의 스칼라 필드에 한정한다.
    // P4-S0-2: `bb/types.rs` + `common/types.rs` 양쪽을 스캔한다.
    // (`common/util.rs` 는 a8 과 같은 이유로 제외 — 위 주석 참조.)
    let sources: [&str; 2] = [
        include_str!("../src/solver/bb/types.rs"),
        include_str!("../src/solver/common/types.rs"),
    ];
    let dimensionless = [
        "nu", "hrc", "gamma", "f_rho_i", "f_rho_e", "percent",
        "osculation_inner", "osculation_outer", "convergence_tol", "residual_norm",
        // BbContactDerived — χ·타원적분·무차원 계수는 전부 무차원
        "chi_inner", "chi_outer",
        "k_ellip_inner", "e_ellip_inner", "k_ellip_outer", "e_ellip_outer",
        "a_star_inner", "b_star_inner", "delta_star_inner",
        "a_star_outer", "b_star_outer", "delta_star_outer",
    ];
    let suffixes = [
        "_mm", "_n", "_nmm", "_rad", "_mpa", "_per_mm", "_g_cm3", "_rpm", "_c",
        "_ms", "_g", "_mm_per_min", "_n_per_mm15",
    ];

    let mut offenders = Vec::new();
    for line in sources.iter().flat_map(|s| s.lines()) {
        let t = line.trim();
        // `pub 이름: f64,` 형태만 검사 (bool/u32/Vec/enum 등은 무차원 또는 비스칼라)
        if !t.starts_with("pub ") || !t.contains(": f64") {
            continue;
        }
        let name = t
            .trim_start_matches("pub ")
            .split(':')
            .next()
            .unwrap_or("")
            .trim();
        if name.is_empty() || name.contains(' ') || dimensionless.contains(&name) {
            continue;
        }
        if !suffixes.iter().any(|s| name.ends_with(s)) {
            offenders.push(name.to_string());
        }
    }
    assert!(
        offenders.is_empty(),
        "단위 접미사 없는 유차원 f64 필드: {offenders:?}"
    );
}
