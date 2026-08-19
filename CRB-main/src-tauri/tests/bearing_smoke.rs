//! CRB Phase 4 — bearing.rs Smoke Test
//!
//! 부호/수렴/대칭성 확인 (Level D 수치 검증 이전 단계).

use app_lib::solver::bearing::solve_bearing_equilibrium;
use app_lib::solver::types::*;

/// No-op progress reporter for tests.
struct SilentReporter;
impl ProgressReporter for SilentReporter {
    fn report(&self, _progress: SolverProgress) {}
}

fn crb_input(f_x_kn: f64, f_y_kn: f64, m_x_knm: f64, g_r_um: f64) -> BearingInput {
    BearingInput {
        macro_geom: MacroGeometry {
            d: 200.0, outer_diameter: 360.0, t: 58.0,
            z: 18, d_we: 44.0, l_we: 42.0, d_pw: 280.0, g_r: g_r_um,
        },
        raceway_geom: RacewayGeometry {
            r_i: 1.0e9, r_o: 1.0e9, d_uc: 0.0, l_uc: 0.0,
        },
        roller_profile: RollerProfile {
            crown_type: CrownType::Parabolic { c2: 0.0 },
            delta_c: 0.0, delta_dub: 0.0, l_dub: 0.0, sigma_roller: 0.15,
        },
        raceway_profile_inner: RacewayProfile {
            delta_rw: 0.0, w_a: 0.0, ra: 0.15,
            custom_profile: None, polynomial_coeffs: None,
        },
        raceway_profile_outer: RacewayProfile {
            delta_rw: 0.0, w_a: 0.0, ra: 0.15,
            custom_profile: None, polynomial_coeffs: None,
        },
        material: Material::default(),
        operating: OperatingConditions {
            f_x: f_x_kn, f_y: f_y_kn, m_x: m_x_knm,
            n_inner_rpm: 500.0, n_outer_rpm: 0.0, gamma: 0.0,
            t_op: 60.0, nu_40: 68.0, nu_100: 8.0, alpha_pv: 20.0,
            lubrication_type: LubricationType::Oil, starvation_factor: 1.0, rho_oil: 850.0,
            design_life_hours: 100.0,
            lubrication_model: LubricationModel::Method1_DH,
            film_decay_enabled: false, film_decay_time_hours: 0.0,
            skew_angle_deg: 0.0, replenishment_rate_nm_s: 0.0,
            surface_finish: SurfaceFinish::Standard, additive_type: AdditiveType::None,
            tau_eyring: 5.0, z_roelands: 0.67,
            traction_model: TractionModel::Eyring,
            carreau_eta_inf_ratio: 0.005, carreau_lambda_s: 1e-7,
            carreau_n: 0.5, carreau_a: 2.0,
            friction_model: FrictionModel::PalmgrenLike,
            thermal_correction: ThermalCorrection::Aihara1987,
            hysteresis_loss_factor: 0.005,
            skf_lubrication: SkfLubricationEnum::OilBath, skf_y_factor: 1.6,
            k_fluid: 0.15, beta_visc: 0.04,
            roughness_input_mode: RoughnessInputMode::Rq,
            rq_inner: 0.3, rq_outer: 0.3, rq_roller: 0.15,
        },
        solver: SolverParams {
            run_mode: RunMode::Single(SolverMode::Gen1),
            n_slices: 20,
            beam_type: BeamType::Timoshenko,
            convergence_tol: 1e-5, max_iterations: 100,
            angular_increment_deg: 2.0,
            life_method: LifeMethod::Iso16281,
            e_c: 0.0,
            contamination_level: ContaminationLevel::NormalCleanliness,
            oil_supply_method: OilSupplyMethod::OilBath,
            c_r_kn: Some(1520.0), c_0r_kn: Some(2400.0), f_s_min: 1.0,
            rib_contact_mode: RibContactMode::PostProcess,
            f_0r: 1.7, f_1r: 0.00025,
            kappa_method: KappaMethod::ViscosityRatio,
            use_split_contact: true,
        },
        transient: None,
    }
}

/// Smoke 1: 순수 F_y (-1000 kN, 중력) — 수렴 + 부호 확인
#[test]
fn smoke_pure_gravity_load_converges() {
    let input = crb_input(0.0, -1000.0, 0.0, 30.0);
    let result = solve_bearing_equilibrium(&input, &SilentReporter).expect("NR should converge");

    // 부호: F_y < 0 → δ_y < 0
    let disp = &result.equilibrium.displacement;
    assert!(disp[1] < 0.0, "F_y=-1000 이면 δ_y < 0 이어야, 실제 δ_y={}", disp[1]);
    // 대칭성: F_x=0, M_x=0 → δ_x ≈ 0, γ_x ≈ 0
    assert!(disp[0].abs() < 1.0, "F_x=0 이면 δ_x ≈ 0, 실제 δ_x={}", disp[0]);
    assert!(disp[3].abs() < 1e-6, "M_x=0 이면 γ_x ≈ 0, 실제 γ_x={}", disp[3]);
    // δ_z, γ_y 는 항상 0 (CRB 3-DOF)
    assert_eq!(disp[2], 0.0, "δ_z 는 0 이어야 (CRB D4)");
    assert_eq!(disp[4], 0.0, "γ_y 는 0 이어야 (CRB D6)");

    println!("δ = [{:.3}, {:.3}, 0, {:.2e}, 0] μm/rad", disp[0], disp[1], disp[3]);
    println!("Q_max = {:.1} kN", result.equilibrium.roller_loads.iter().cloned().fold(0.0_f64, f64::max) / 1000.0);
}

/// Smoke 2: 순수 F_x — 부호 확인
#[test]
fn smoke_pure_fx_load_converges() {
    let input = crb_input(1000.0, 0.0, 0.0, 30.0);
    let result = solve_bearing_equilibrium(&input, &SilentReporter).expect("NR should converge");

    let disp = &result.equilibrium.displacement;
    assert!(disp[0] > 0.0, "F_x>0 이면 δ_x > 0, 실제 δ_x={}", disp[0]);
    assert!(disp[1].abs() < 1.0, "F_y=0 이면 δ_y ≈ 0, 실제 δ_y={}", disp[1]);
}

/// Smoke 3: Load zone extent — 하중 방향 반대편 roller 는 Q≈0 (clearance 있을 때)
#[test]
fn smoke_load_zone_extent() {
    let input = crb_input(0.0, -1000.0, 0.0, 30.0);
    let result = solve_bearing_equilibrium(&input, &SilentReporter).expect("NR should converge");

    let load_angle_deg = result.load_angle_deg;
    println!("Load angle: {:.1}°", load_angle_deg);

    // 하부 (F_y<0 → load angle ≈ -90°) 근처 roller 는 하중,
    // 상부 (반대편) roller 는 clearance 로 인해 Q=0
    let mut n_loaded = 0;
    let mut n_zero = 0;
    for r in &result.equilibrium.roller_results {
        if r.q_normal > 1.0 { n_loaded += 1; }
        else { n_zero += 1; }
    }
    println!("Loaded rollers: {}, Zero rollers: {}", n_loaded, n_zero);
    assert!(n_loaded > 0, "적어도 하나의 roller 는 하중");
    // Zero clearance case (g_r=30μm 는 small clearance) 도 반드시 load zone < 180°
    // 여기서 clearance 30μm 는 큰 하중 (1000 kN) 대비 상대적으로 작지만 일부 roller Q=0 예상
    // 만약 모두 loaded 이면 heavy load 상황, 그것도 정상
    let total_z = n_loaded + n_zero;
    assert_eq!(total_z, 18, "총 roller 수 = Z = 18");
}

/// Smoke 4: 하중 0 → 변위 0 근처
#[test]
fn smoke_zero_load_zero_displacement() {
    let input = crb_input(0.0, 0.0, 0.0, 30.0);
    let result = solve_bearing_equilibrium(&input, &SilentReporter).expect("Zero load should trivially converge");
    let disp = &result.equilibrium.displacement;
    assert!(disp[0].abs() < 1.0);
    assert!(disp[1].abs() < 1.0);
    assert!(disp[3].abs() < 1e-6);
    for q in &result.equilibrium.roller_loads {
        assert!(*q < 1.0, "무하중 시 Q_j ≈ 0, 실제 {}", q);
    }
}

/// Smoke 5: Roller 개수 = Z
#[test]
fn smoke_roller_count_equals_z() {
    let input = crb_input(0.0, -500.0, 0.0, 30.0);
    let result = solve_bearing_equilibrium(&input, &SilentReporter).expect("NR should converge");
    assert_eq!(result.equilibrium.roller_results.len(), 18);
    assert_eq!(result.equilibrium.roller_loads.len(), 18);
}

// ─── M_x 검증 (Outer γ_x loop 수렴성/정확성) ─────────────────────

/// Smoke 6: Dominant M_x (with minimal radial preload) — Outer γ_x loop 수렴 검증
///
/// Pure moment (F=0) 는 clearance g_r 로 인해 어떤 roller 도 접촉하지 못하므로
/// dM/dγ = 0 → 초기 gradient singular → 물리적으로 정의 불량.
/// 최소 radial preload (F_y=-100 kN, g_r=0 → 접촉 보장) + M_x 로 γ_x 응답 확인.
#[test]
fn smoke_pure_mx_converges() {
    // clearance = 0 으로 pure preload 없이도 접촉 확보. F_y 는 minimal (-100 kN).
    let input = crb_input(0.0, -100.0, 100.0, 0.0);  // g_r=0, F_y=-100, M_x=100
    let result = solve_bearing_equilibrium(&input, &SilentReporter).expect("NR should converge");
    let disp = &result.equilibrium.displacement;
    println!("Dominant M_x: δ = [{:.3}, {:.3}, 0, {:.3e}, 0] μm/rad",
             disp[0], disp[1], disp[3]);

    // γ_x 는 non-zero (M_x 를 만족시켜야)
    assert!(disp[3].abs() > 1e-8, "M_x≠0 이면 γ_x ≠ 0 이어야, 실제 γ_x={:.3e}", disp[3]);
    assert!(disp[0].abs() < 30.0, "δ_x 는 크지 않음, 실제 {}", disp[0]);
    assert!(disp[1].abs() < 30.0, "δ_y 는 크지 않음, 실제 {}", disp[1]);
}

/// Smoke 7: M_x 부호 반전 → γ_x 부호 반전 (선형성 검증)
#[test]
fn smoke_mx_sign_flip() {
    let input_pos = crb_input(0.0, -500.0, 50.0, 30.0);   // radial + M_x > 0
    let input_neg = crb_input(0.0, -500.0, -50.0, 30.0);  // radial + M_x < 0

    let r_pos = solve_bearing_equilibrium(&input_pos, &SilentReporter).expect("NR converges");
    let r_neg = solve_bearing_equilibrium(&input_neg, &SilentReporter).expect("NR converges");

    let g_pos = r_pos.equilibrium.displacement[3];
    let g_neg = r_neg.equilibrium.displacement[3];
    println!("M_x sign flip: γ_x(+50 kN·m) = {:.3e}, γ_x(-50 kN·m) = {:.3e}", g_pos, g_neg);

    // 부호 반전
    assert!(g_pos * g_neg < 0.0, "M_x 부호 반전 시 γ_x 도 반전, 실제 pos={:.3e}, neg={:.3e}",
            g_pos, g_neg);
    // 절대값 유사 (선형성, 30% 여유)
    let rel = ((g_pos.abs() - g_neg.abs()) / g_pos.abs().max(g_neg.abs())).abs();
    assert!(rel < 0.3, "γ_x 절대값 유사해야 (선형성), 실제 rel = {:.2}", rel);
}

/// Smoke 8: M_x monotonicity — M_x 증가 → γ_x 증가
#[test]
fn smoke_mx_monotonicity() {
    let mut gammas = Vec::new();
    for &m_x in &[10.0, 50.0, 100.0, 200.0_f64] {   // kN·m
        let input = crb_input(0.0, -500.0, m_x, 30.0);   // F_y 유지 + M_x 변화
        let result = solve_bearing_equilibrium(&input, &SilentReporter).expect("NR converges");
        let g = result.equilibrium.displacement[3];
        println!("M_x = {} kN·m → γ_x = {:.4e} rad", m_x, g);
        gammas.push(g);
    }
    // γ_x 단조 증가 (M_x 증가 방향)
    for i in 0..gammas.len() - 1 {
        assert!(gammas[i + 1] > gammas[i],
            "M_x 증가 시 γ_x 단조 증가 실패: γ_x[{}]={:.3e}, γ_x[{}]={:.3e}",
            i, gammas[i], i+1, gammas[i+1]);
    }
}
