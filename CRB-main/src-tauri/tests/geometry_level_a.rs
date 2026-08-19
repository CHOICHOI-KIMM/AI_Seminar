//! CRB Phase 2 — Level A 검증: 단일 slice Hertz 접촉 해석해 vs solver 결과
//!
//! Plan §Phase 2 §2.5 통과 기준: 상대 오차 < 0.1%
//!
//! ISO 16281 Clause 6.3.2 (p. 22) — Cylindrical roller line contact 해석식:
//!   b       = √(4·q·R / (π·E*))                    [contact half-width, mm]
//!   p_max   = 2·q / (π·b) = √(q·E*/(π·R))          [max Hertz pressure, MPa]
//!
//! 단위계:
//!   q   : line load [N/mm]  (= Q / L_slice)
//!   R   : equivalent radius [mm]
//!   E*  : combined elastic modulus [MPa]
//!   b   : contact half-width [mm]
//!   p   : pressure [MPa]

use app_lib::solver::geometry::compute_slices;
use app_lib::solver::hertz::{combined_elastic_modulus, hertz_half_width, hertz_max_pressure};
use app_lib::solver::types::{
    CrownType, MacroGeometry, RacewayGeometry, RollerProfile, RacewayProfile,
};
use std::f64::consts::PI;

const TOL_REL: f64 = 1e-3; // Level A: 상대오차 < 0.1%

// ─── Analytical (Palmgren / Hertz line contact) ─────────────────────────────

fn analytical_half_width(q_nmm: f64, r_eq_mm: f64, e_star_mpa: f64) -> f64 {
    (4.0 * q_nmm * r_eq_mm / (PI * e_star_mpa)).sqrt()
}

fn analytical_max_pressure(q_nmm: f64, r_eq_mm: f64, e_star_mpa: f64) -> f64 {
    (q_nmm * e_star_mpa / (PI * r_eq_mm)).sqrt()
}

// ─── Test 1: hertz.rs 단일 함수 vs 해석해 ────────────────────────────────────

#[test]
fn level_a_hertz_half_width_matches_analytical() {
    // NU 240 근사 조건
    let q_nmm = 500.0;       // 500 N/mm line load
    let r_eq_mm = 22.0;      // D_we/2 = 22 mm
    let e_star_mpa = 115_384.6; // E=210 GPa, ν=0.3, 두 body 같은 재질 → E* = E/(2(1-ν²)) = 115.38 GPa

    let b_solver = hertz_half_width(q_nmm, r_eq_mm, e_star_mpa);
    let b_analytical = analytical_half_width(q_nmm, r_eq_mm, e_star_mpa);

    let rel_err = ((b_solver - b_analytical) / b_analytical).abs();
    assert!(
        rel_err < TOL_REL,
        "half_width: solver={:.6}, analytical={:.6}, rel_err={:.2e}",
        b_solver, b_analytical, rel_err
    );
}

#[test]
fn level_a_hertz_max_pressure_matches_analytical() {
    let q_nmm = 500.0;
    let r_eq_mm = 22.0;
    let e_star_mpa = 115_384.6;

    let b_solver = hertz_half_width(q_nmm, r_eq_mm, e_star_mpa);
    let p_solver = hertz_max_pressure(q_nmm, b_solver);
    let p_analytical = analytical_max_pressure(q_nmm, r_eq_mm, e_star_mpa);

    let rel_err = ((p_solver - p_analytical) / p_analytical).abs();
    assert!(
        rel_err < TOL_REL,
        "p_max: solver={:.3}, analytical={:.3}, rel_err={:.2e}",
        p_solver, p_analytical, rel_err
    );
}

#[test]
fn level_a_combined_elastic_modulus() {
    let e = 210.0; // GPa
    let nu = 0.3;
    let e_star_solver = combined_elastic_modulus(e, nu, e, nu);
    let e_star_expected = e / (2.0 * (1.0 - nu * nu));

    let rel_err = ((e_star_solver - e_star_expected) / e_star_expected).abs();
    assert!(
        rel_err < TOL_REL,
        "E*: solver={:.6}, expected={:.6}, rel_err={:.2e}",
        e_star_solver, e_star_expected, rel_err
    );
}

// ─── Test 2: compute_slices — 균일 원통 roller 검증 ─────────────────────────

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
        r_i: 1.0e9, // 원통 raceway 근사
        r_o: 1.0e9,
        d_uc: 0.0,
        l_uc: 0.0,
    };
    let roller_profile = RollerProfile {
        crown_type: CrownType::Parabolic { c2: 0.0 }, // flat (Level A 조건)
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

#[test]
fn level_a_compute_slices_uniform_roller_radius() {
    let (mg, rg, rp, rwp) = crb_geometry_nu240();
    let slices = compute_slices(&mg, &rg, &rp, &rwp, &rwp, 30).unwrap();

    let r_expected = mg.d_we / 2.0; // 22.0 mm
    for s in &slices {
        assert!(
            (s.r_roller - r_expected).abs() < 1e-10,
            "slice {}: r_roller={} ≠ {}",
            s.k, s.r_roller, r_expected
        );
    }
}

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
    // 총합 = L_we
    let total: f64 = slices.iter().map(|s| s.slice_width).sum();
    assert!((total - mg.l_we).abs() < 1e-9, "sum of widths = {} ≠ L_we={}", total, mg.l_we);
}

#[test]
fn level_a_compute_slices_x_axial_symmetric() {
    // Slice 중심 좌표가 [0.5·Δ, L_we - 0.5·Δ] 범위, 균등 간격
    let (mg, rg, rp, rwp) = crb_geometry_nu240();
    let n = 30;
    let slices = compute_slices(&mg, &rg, &rp, &rwp, &rwp, n).unwrap();
    let expected_width = mg.l_we / n as f64;

    // 첫 slice = 0.5·Δ, 마지막 slice = (n-0.5)·Δ = L_we - 0.5·Δ
    assert!((slices[0].x_axial - 0.5 * expected_width).abs() < 1e-10);
    assert!(
        (slices[n - 1].x_axial - (mg.l_we - 0.5 * expected_width)).abs() < 1e-10
    );

    // 대칭성: x[k] + x[n-1-k] = L_we
    for k in 0..(n / 2) {
        let sum = slices[k].x_axial + slices[n - 1 - k].x_axial;
        assert!(
            (sum - mg.l_we).abs() < 1e-9,
            "asymmetric at k={}: {} + {} ≠ {}",
            k, slices[k].x_axial, slices[n - 1 - k].x_axial, mg.l_we
        );
    }
}

#[test]
fn level_a_compute_slices_r_eq_crb_orbital() {
    // CRB 궤도 곡률 검증:
    //   Flat raceway (r_race → ∞) 이면 R_eq = D_we/2·(1 ∓ γ),  γ = D_we/D_pw
    let (mg, rg, rp, rwp) = crb_geometry_nu240();
    let slices = compute_slices(&mg, &rg, &rp, &rwp, &rwp, 30).unwrap();

    let gamma = mg.d_we / mg.d_pw; // 44/280 = 0.1571
    let r_eq_inner_expected = (mg.d_we / 2.0) * (1.0 - gamma); // = 18.543 mm
    let r_eq_outer_expected = (mg.d_we / 2.0) * (1.0 + gamma); // = 25.457 mm

    for s in &slices {
        let rel_i = ((s.r_eq_inner - r_eq_inner_expected) / r_eq_inner_expected).abs();
        let rel_o = ((s.r_eq_outer - r_eq_outer_expected) / r_eq_outer_expected).abs();
        assert!(
            rel_i < TOL_REL,
            "slice {}: R_eq_inner={:.6}, expected={:.6}, rel_err={:.2e}",
            s.k, s.r_eq_inner, r_eq_inner_expected, rel_i
        );
        assert!(
            rel_o < TOL_REL,
            "slice {}: R_eq_outer={:.6}, expected={:.6}, rel_err={:.2e}",
            s.k, s.r_eq_outer, r_eq_outer_expected, rel_o
        );
    }
}

// ─── Test 3: End-to-end — compute_slices → hertz 해석해 비교 ────────────────

#[test]
fn level_a_end_to_end_slice_hertz_matches_analytical() {
    // 완전 조건: NU 240 flat profile, per-slice line load 부여 → Hertz 계산 → 해석해와 비교
    let (mg, rg, rp, rwp) = crb_geometry_nu240();
    let slices = compute_slices(&mg, &rg, &rp, &rwp, &rwp, 30).unwrap();

    let e_star_gpa = combined_elastic_modulus(210.0, 0.3, 210.0, 0.3);
    let e_star_mpa = e_star_gpa * 1000.0;
    let q_per_mm = 500.0; // uniform line load per slice

    for s in &slices {
        // Solver
        let b_s_inner = hertz_half_width(q_per_mm, s.r_eq_inner, e_star_mpa);
        let p_s_inner = hertz_max_pressure(q_per_mm, b_s_inner);
        // Analytical
        let b_a_inner = analytical_half_width(q_per_mm, s.r_eq_inner, e_star_mpa);
        let p_a_inner = analytical_max_pressure(q_per_mm, s.r_eq_inner, e_star_mpa);

        assert!(
            ((b_s_inner - b_a_inner) / b_a_inner).abs() < TOL_REL,
            "inner b: solver={}, analytical={}", b_s_inner, b_a_inner
        );
        assert!(
            ((p_s_inner - p_a_inner) / p_a_inner).abs() < TOL_REL,
            "inner p_max: solver={}, analytical={}", p_s_inner, p_a_inner
        );
    }
}
