//! CRB Phase 3 — Level C 검증: Gen1 ↔ Gen3 진짜 독립 비교
//!
//! [Plan §3.5] Level C 검증 설계
//!   - Gen1 (독립 slice, O(n) 비선형 스프링)
//!   - Gen3 (Timoshenko beam + Newton-Raphson, O(n²))
//!   → 서로 다른 알고리즘. Phase 2 동어반복 함정 회피.
//!
//! Flat profile + 강체 roller 가정에서 두 알고리즘 결과가 수렴해야 함
//! (이론적 근거: Gen3 의 beam 항 [K_beam]{w} 는 flat + 균일 하중 시 rigid body mode 만).
//!
//! 통과 기준 (§3.6):
//!   - Q_total 상대오차 < 1%
//!   - q_k 분포 L2 오차 < 2%
//!   - beam deflection w_k < 0.1 μm (Gen3, flat 조건)

use app_lib::solver::gen1::solve_gen1_roller;
use app_lib::solver::gen3::solve_gen3_roller;
use app_lib::solver::geometry::compute_slices;
use app_lib::solver::types::{
    BeamType, CrownType, MacroGeometry, Material, RacewayGeometry, RollerProfile,
    RacewayProfile, RunMode, SolverMode, SolverParams,
};

const N_SLICES: usize = 30;
const TOL_Q_TOTAL: f64 = 0.01;    // Q_total 상대오차 < 1%
const TOL_Q_L2: f64 = 0.02;       // q_k L2 오차 < 2%
const CRB_COS_ALPHA_DIFF: f64 = 1.0; // CRB α = 0

fn crb_geometry_nu240() -> (MacroGeometry, RacewayGeometry, RollerProfile, RacewayProfile, Material, SolverParams) {
    let macro_geom = MacroGeometry {
        d: 200.0, outer_diameter: 360.0, t: 58.0,
        z: 18, d_we: 44.0, l_we: 42.0, d_pw: 280.0, g_r: 30.0,
    };
    let raceway_geom = RacewayGeometry {
        r_i: 1.0e9, r_o: 1.0e9, d_uc: 0.0, l_uc: 0.0,
    };
    let flat_profile = RollerProfile {
        crown_type: CrownType::Parabolic { c2: 0.0 },
        delta_c: 0.0, delta_dub: 0.0, l_dub: 0.0, sigma_roller: 0.15,
    };
    let raceway_profile = RacewayProfile {
        delta_rw: 0.0, w_a: 0.0, ra: 0.15,
        custom_profile: None, polynomial_coeffs: None,
    };
    let material = Material::default();
    let solver = SolverParams {
        run_mode: RunMode::Single(SolverMode::Gen1),
        n_slices: N_SLICES,
        beam_type: BeamType::Timoshenko,
        convergence_tol: 1e-6,
        max_iterations: 200,
        angular_increment_deg: 2.0,
        life_method: app_lib::solver::types::LifeMethod::Iso16281,
        e_c: 0.0,
        contamination_level: app_lib::solver::types::ContaminationLevel::NormalCleanliness,
        oil_supply_method: app_lib::solver::types::OilSupplyMethod::OilBath,
        c_r_kn: Some(1520.0), c_0r_kn: Some(2400.0), f_s_min: 1.0,
        rib_contact_mode: app_lib::solver::types::RibContactMode::PostProcess,
        f_0r: 1.7, f_1r: 0.00025,
        kappa_method: app_lib::solver::types::KappaMethod::ViscosityRatio,
        use_split_contact: true,
    };
    (macro_geom, raceway_geom, flat_profile, raceway_profile, material, solver)
}

fn compute_q_l2_error(q_ref: &[f64], q_test: &[f64]) -> f64 {
    assert_eq!(q_ref.len(), q_test.len(), "q_k length mismatch");
    let ref_norm_sq: f64 = q_ref.iter().map(|q| q * q).sum();
    if ref_norm_sq < 1e-20 {
        // Both zero → treat as 0 error
        return 0.0;
    }
    let diff_norm_sq: f64 = q_ref.iter().zip(q_test.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum();
    (diff_norm_sq / ref_norm_sq).sqrt()
}

/// Level C-1: Q_total 비교 (Gen1 ↔ Gen3, 5 delta 조건)
#[test]
fn level_c_gen1_gen3_q_total_convergence() {
    let (mg, rg, rp, rwp, mat, solver) = crb_geometry_nu240();
    let slices = compute_slices(&mg, &rg, &rp, &rwp, &rwp, N_SLICES).unwrap();

    for &delta_rigid in &[5.0, 10.0, 20.0, 50.0, 100.0_f64] {
        let (_, q_total_gen1) = solve_gen1_roller(&slices, delta_rigid, &mat, CRB_COS_ALPHA_DIFF);
        let (_, q_total_gen3) = solve_gen3_roller(&slices, delta_rigid, &mat, &solver, CRB_COS_ALPHA_DIFF).unwrap();

        let rel_err = ((q_total_gen1 - q_total_gen3) / q_total_gen1).abs();
        println!(
            "δ={:.1} μm: Q_gen1={:.3} N, Q_gen3={:.3} N, rel_err={:.4}",
            delta_rigid, q_total_gen1, q_total_gen3, rel_err
        );
        assert!(
            rel_err < TOL_Q_TOTAL,
            "δ={} μm: Gen1↔Gen3 Q_total rel_err={:.4} > {:.4}",
            delta_rigid, rel_err, TOL_Q_TOTAL
        );
    }
}

/// Level C-2: q_k slice 분포 L2 오차 비교
#[test]
fn level_c_gen1_gen3_qk_distribution_l2() {
    let (mg, rg, rp, rwp, mat, solver) = crb_geometry_nu240();
    let slices = compute_slices(&mg, &rg, &rp, &rwp, &rwp, N_SLICES).unwrap();

    for &delta_rigid in &[10.0, 50.0_f64] {
        let (res_gen1, _) = solve_gen1_roller(&slices, delta_rigid, &mat, CRB_COS_ALPHA_DIFF);
        let (res_gen3, _) = solve_gen3_roller(&slices, delta_rigid, &mat, &solver, CRB_COS_ALPHA_DIFF).unwrap();

        // Use outer raceway q_k (available in both results)
        let q_gen1: Vec<f64> = res_gen1.iter().map(|r| r.q_k).collect();
        let q_gen3: Vec<f64> = res_gen3.iter().map(|r| r.q_k).collect();

        let l2_err = compute_q_l2_error(&q_gen1, &q_gen3);
        println!("δ={:.1} μm: q_k L2 rel_err = {:.4}", delta_rigid, l2_err);
        assert!(
            l2_err < TOL_Q_L2,
            "δ={} μm: q_k L2 err={:.4} > {:.4}",
            delta_rigid, l2_err, TOL_Q_L2
        );
    }
}

/// Level C-3: Gen3 beam deflection 은 flat profile 조건에서 매우 작아야 (rigid body 만)
#[test]
fn level_c_gen3_beam_deflection_small_flat_profile() {
    let (mg, rg, rp, rwp, mat, solver) = crb_geometry_nu240();
    let slices = compute_slices(&mg, &rg, &rp, &rwp, &rwp, N_SLICES).unwrap();
    let delta_rigid = 50.0;

    let (res_gen3, _) = solve_gen3_roller(&slices, delta_rigid, &mat, &solver, CRB_COS_ALPHA_DIFF).unwrap();

    // beam deflection = δ_rigid - δ_k - dz (Phase 1 commands.rs 계산과 동일)
    let cos_alpha_diff = CRB_COS_ALPHA_DIFF;
    let max_w: f64 = res_gen3.iter().zip(slices.iter())
        .map(|(r, s)| (delta_rigid - r.delta_k - s.delta_z_total_outer
            - s.delta_z_total_inner * cos_alpha_diff).abs())
        .fold(0.0_f64, f64::max);

    println!("δ={:.1} μm: max beam deflection |w| = {:.6} μm (flat profile)", delta_rigid, max_w);
    // Flat profile + rigid body projection removed → w 는 매우 작아야
    // (수치 반올림 + NR 잔차 수준). Level C 통과 기준 0.1 μm 는 여유 있음.
    assert!(
        max_w < 0.1,
        "flat profile 에서 max w = {:.4} μm > 0.1 μm — beam coupling 이상",
        max_w
    );
}
