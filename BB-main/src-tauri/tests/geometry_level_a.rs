//! CRB Phase 2 — Level A 검증: compute_slices 구현 검증 (3 tests)
//!
//! ⚠️ 설계 결정 (2026-08-19, 사용자 재검토):
//! 이전 8 tests 중 Test 1/2/3 (Hertz b, p_max, E*) 및 Test 4/7 (R_eq 파생값) 은
//! solver 함수와 analytical 함수가 **동일 공식** 을 사용하여 오차 0 = 동어반복 (tautology).
//! 진짜 검증이 아니라 self-consistency 확인이었음.
//!
//! 여기 남은 3 tests 는 순수한 slicer 구현 검증:
//!   - Test 5: slice 폭 균등 분할 (Σ = L_we)
//!   - Test 6: x_axial 대칭성
//!   - Test 8: end-to-end pipeline 연결 검증 (finite/positive 만, 값 정확도 아님)
//!
//! 실제 정확도 검증 (Level B): Phase 3+ 에서 Reference 도서 (Harris, Palmgren) 예제 값
//! 또는 FEA (ANSYS/ABAQUS) 결과와 비교 예정.

use app_lib::solver::geometry::compute_slices;
use app_lib::solver::hertz::{combined_elastic_modulus, hertz_half_width, hertz_max_pressure};
use app_lib::solver::types::{
    CrownType, MacroGeometry, RacewayGeometry, RollerProfile, RacewayProfile,
};

const TOL_REL: f64 = 1e-3; // Level A: 상대오차 < 0.1%

fn crb_geometry_nu240() -> (MacroGeometry, RacewayGeometry, RollerProfile, RacewayProfile) {
    let macro_geom = MacroGeometry {
        d: 200.0,
        outer_diameter: 360.0,
        t: 58.0,
        z: 18,
        d_we: 44.0,
        l_we: 42.0,
        d_pw: 280.0,
        g_r: 30.0,
    };
    let raceway_geom = RacewayGeometry {
        r_i: 1.0e9,
        r_o: 1.0e9,
        d_uc: 0.0,
        l_uc: 0.0,
    };
    let roller_profile = RollerProfile {
        crown_type: CrownType::Parabolic { c2: 0.0 },
        delta_c: 0.0,
        delta_dub: 0.0,
        l_dub: 0.0,
        sigma_roller: 0.15,
    };
    let raceway_profile = RacewayProfile {
        delta_rw: 0.0,
        w_a: 0.0,
        ra: 0.15,
        custom_profile: None,
        polynomial_coeffs: None,
    };
    (macro_geom, raceway_geom, roller_profile, raceway_profile)
}

/// Test 5: compute_slices — slice 폭 균등 분할 + 총합 = L_we
#[test]
fn level_a_compute_slices_uniform_slice_width() {
    let (mg, rg, rp, rwp) = crb_geometry_nu240();
    let n = 30;
    let slices = compute_slices(&mg, &rg, &rp, &rwp, &rwp, n).unwrap();

    let expected_width = mg.l_we / n as f64;
    for s in &slices {
        assert!(
            (s.slice_width - expected_width).abs() < 1e-10,
            "slice {}: width={} ≠ {}",
            s.k, s.slice_width, expected_width
        );
    }
    let total: f64 = slices.iter().map(|s| s.slice_width).sum();
    assert!(
        (total - mg.l_we).abs() < 1e-9,
        "sum of widths = {} ≠ L_we={}", total, mg.l_we
    );
}

/// Test 6: x_axial 대칭성 (slice 중심 좌표 대칭성)
#[test]
fn level_a_compute_slices_x_axial_symmetric() {
    let (mg, rg, rp, rwp) = crb_geometry_nu240();
    let n = 30;
    let slices = compute_slices(&mg, &rg, &rp, &rwp, &rwp, n).unwrap();
    let expected_width = mg.l_we / n as f64;

    assert!((slices[0].x_axial - 0.5 * expected_width).abs() < 1e-10);
    assert!(
        (slices[n - 1].x_axial - (mg.l_we - 0.5 * expected_width)).abs() < 1e-10
    );

    for k in 0..(n / 2) {
        let sum = slices[k].x_axial + slices[n - 1 - k].x_axial;
        assert!(
            (sum - mg.l_we).abs() < 1e-9,
            "asymmetric at k={}: {} + {} ≠ {}",
            k, slices[k].x_axial, slices[n - 1 - k].x_axial, mg.l_we
        );
    }
}

/// Test 8: End-to-end pipeline 연결 검증 (finite/positive only, 값 정확도 X)
///
/// compute_slices → hertz 함수 호출이 panic 없이 유효 값 반환하는지 확인.
/// ⚠️ 수치 자체는 self-consistency (Rust 와 Python 이 동일 공식) — Level B 별도.
#[test]
fn level_a_end_to_end_pipeline_valid() {
    let (mg, rg, rp, rwp) = crb_geometry_nu240();
    let slices = compute_slices(&mg, &rg, &rp, &rwp, &rwp, 30).unwrap();

    let e_star_gpa = combined_elastic_modulus(210.0, 0.3, 210.0, 0.3);
    let e_star_mpa = e_star_gpa * 1000.0;
    let q_per_mm = 500.0;

    let _ = TOL_REL; // TOL_REL 은 상단 상수로 유지 (문서 목적)

    for s in &slices {
        let b_inner = hertz_half_width(q_per_mm, s.r_eq_inner, e_star_mpa);
        let p_inner = hertz_max_pressure(q_per_mm, b_inner);
        assert!(b_inner.is_finite() && b_inner > 0.0, "slice {} inner b invalid: {}", s.k, b_inner);
        assert!(p_inner.is_finite() && p_inner > 0.0, "slice {} inner p_max invalid: {}", s.k, p_inner);

        let b_outer = hertz_half_width(q_per_mm, s.r_eq_outer, e_star_mpa);
        let p_outer = hertz_max_pressure(q_per_mm, b_outer);
        assert!(b_outer.is_finite() && b_outer > 0.0, "slice {} outer b invalid: {}", s.k, b_outer);
        assert!(p_outer.is_finite() && p_outer > 0.0, "slice {} outer p_max invalid: {}", s.k, p_outer);
    }
}
