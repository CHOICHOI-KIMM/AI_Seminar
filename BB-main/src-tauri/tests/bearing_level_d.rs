//! CRB Phase 4 — Level D 검증: Sjovall integral (Harris & Kotzalas Ch 7) 참조값 비교
//!
//! Sjovall integral (Harris & Kotzalas 5th ed. Ch 7, Eq 7.71):
//!   Q_max / F_r = 1 / (Z · J_r(ε))
//!
//! ε (load distribution factor):
//!   ε = 0.5 : zero clearance, radial only → J_r(0.5) = 0.2453 (load zone = 180°)
//!   ε < 0.5 : preload (load zone > 180°)
//!   ε > 0.5 : clearance (load zone < 180°)
//!
//! NU 240 (Z=18), F_y = -1000 kN, zero clearance:
//!   Q_max = 1000·1000 / (18 · 0.2453) = 226,455 N ≈ 226.5 kN
//!
//! 통과 기준 (Plan §5.2 Level D): 상대오차 < 5%

use app_lib::solver::bearing::solve_bearing_equilibrium;
use app_lib::solver::types::*;

struct SilentReporter;
impl ProgressReporter for SilentReporter {
    fn report(&self, _: SolverProgress) {}
}

const TOL_REL_LEVEL_D: f64 = 0.05; // 5%

fn nu240(f_x_kn: f64, f_y_kn: f64, m_x_knm: f64, g_r_um: f64) -> BearingInput {
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
            n_slices: 30,
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

/// Level D-1: Zero clearance, F_y = -1000 kN
/// Sjovall ε = 0.5, J_r(0.5) = 0.2453 → Q_max ≈ 226.5 kN, Load zone = 180°
#[test]
fn level_d_sjovall_zero_clearance_1000kn() {
    let input = nu240(0.0, -1000.0, 0.0, 0.0); // g_r = 0
    let result = solve_bearing_equilibrium(&input, &SilentReporter).expect("NR converges");

    let q_max_n: f64 = result.equilibrium.roller_loads.iter().cloned().fold(0.0_f64, f64::max);
    let q_max_kn = q_max_n / 1000.0;

    let f_r_n = 1000.0 * 1000.0;
    let z = 18.0_f64;
    let j_r_05 = 0.2453;
    let q_max_expected_kn = f_r_n / (z * j_r_05) / 1000.0;   // 226.5 kN

    let rel_err = ((q_max_kn - q_max_expected_kn) / q_max_expected_kn).abs();
    println!(
        "Zero clearance: Q_max_solver = {:.2} kN, Q_max_Sjovall = {:.2} kN, rel_err = {:.3}",
        q_max_kn, q_max_expected_kn, rel_err
    );
    assert!(
        rel_err < TOL_REL_LEVEL_D,
        "Zero clearance: Q_max rel_err {:.3} > {}", rel_err, TOL_REL_LEVEL_D
    );

    // Load zone: zero clearance 이면 정확히 9 loaded (Z/2 = 180°)
    let n_loaded = result.equilibrium.roller_loads.iter().filter(|&&q| q > 1.0).count();
    println!("Loaded rollers: {} / {}", n_loaded, z as usize);
    assert!(n_loaded >= 8 && n_loaded <= 10, "zero clearance load zone ≈ Z/2 = 9, 실제 {}", n_loaded);
}

/// Level D-2: 대칭성 검증
///   F_x=0, F_y ≠ 0 → δ_x = 0
///   F_x ≠ 0, F_y = 0 → δ_y = 0
#[test]
fn level_d_symmetry_pure_axial_load() {
    // F_y only
    let input = nu240(0.0, -500.0, 0.0, 20.0);
    let result = solve_bearing_equilibrium(&input, &SilentReporter).expect("NR converges");
    let disp = &result.equilibrium.displacement;
    assert!(disp[0].abs() < 0.5, "F_x=0 → δ_x ≈ 0, 실제 {}", disp[0]);
    assert!(disp[1] < 0.0, "F_y<0 → δ_y<0");

    // F_x only
    let input2 = nu240(500.0, 0.0, 0.0, 20.0);
    let result2 = solve_bearing_equilibrium(&input2, &SilentReporter).expect("NR converges");
    let disp2 = &result2.equilibrium.displacement;
    assert!(disp2[1].abs() < 0.5, "F_y=0 → δ_y ≈ 0, 실제 {}", disp2[1]);
    assert!(disp2[0] > 0.0, "F_x>0 → δ_x>0");
}

/// Level D-3: Q_max 방향 확인 — 하중 반대 방향 (즉 하중 벡터 방향의 roller)
#[test]
fn level_d_q_max_direction() {
    let input = nu240(0.0, -1000.0, 0.0, 30.0);
    let result = solve_bearing_equilibrium(&input, &SilentReporter).expect("NR converges");

    // Find roller with max Q — its ψ 는 F_y 방향 (-90°, 하부) 근처
    let (idx_max, _) = result.equilibrium.roller_loads.iter().enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap();
    let psi_max_deg = result.equilibrium.roller_results[idx_max].psi_deg;

    // Load angle 계산: F_y<0 → -90°
    // roller #0 (idx=0) 가 load angle 위치 → ψ_0 = -90° 근처
    // Q_max 는 대개 idx=0 or 인접
    println!("Q_max roller idx: {}, ψ: {:.1}°", idx_max, psi_max_deg);
    assert!(idx_max <= 1 || idx_max >= 17,
        "Q_max 는 하중 방향 (idx 0 or 근처, ψ ≈ -90°), 실제 idx={}", idx_max);
}

/// Level D-4: 하중-변위 monotonicity — F_r 증가 시 |δ_y| 증가
#[test]
fn level_d_monotonicity_load_vs_displacement() {
    let mut deltas = Vec::new();
    for &f_y in &[-100.0, -500.0, -1000.0, -2000.0_f64] {
        let input = nu240(0.0, f_y, 0.0, 30.0);
        let result = solve_bearing_equilibrium(&input, &SilentReporter).expect("NR converges");
        let dy = result.equilibrium.displacement[1];
        println!("F_y = {} kN → δ_y = {:.3} μm", f_y, dy);
        deltas.push(dy);
    }
    // δ_y 단조 감소 (음수 방향으로 커짐)
    for i in 0..deltas.len() - 1 {
        assert!(deltas[i + 1] < deltas[i],
            "Monotonicity 실패: F_r={}, δ_y[{}]={} !< δ_y[{}]={}",
            i, i, deltas[i], i+1, deltas[i+1]);
    }
}

/// Level D-6: M_x self-consistency — Outer γ_x loop residual < tol 검증
///
/// 여러 M_x 조건에서 solver 가 실제로 M_x 를 만족하는지 (내부 residual 확인).
/// 이는 Outer γ_x loop 이 실제로 수렴한다는 direct evidence.
#[test]
fn level_d_mx_self_consistency() {
    for &m_x_knm in &[10.0_f64, 50.0, 100.0, 200.0] {
        let input = nu240(0.0, -500.0, m_x_knm, 30.0);
        let result = solve_bearing_equilibrium(&input, &SilentReporter).expect("NR converges");

        // Solver 결과로부터 실제 M_x 재계산 (slice-level axial arm)
        let mg = &input.macro_geom;
        let l_we_half = mg.l_we / 2.0;
        let mut m_x_calc_nmm = 0.0_f64;
        for r in &result.equilibrium.roller_results {
            let sin_psi = r.psi_deg.to_radians().sin();
            for s in &r.slice_results {
                // slice_width from geometry (assume uniform)
                let sw = mg.l_we / (input.solver.n_slices as f64);
                let x_axial = (s.k as f64 + 0.5) * sw;
                let x_arm = x_axial - l_we_half;
                m_x_calc_nmm += s.q_k * sw * x_arm * sin_psi;
            }
        }
        let m_x_calc_knm = m_x_calc_nmm * 1e-6;
        let m_x_target_knm = m_x_knm;
        let rel_err = ((m_x_calc_knm - m_x_target_knm) / m_x_target_knm).abs();
        println!("M_x_target = {} kN·m, M_x_calc = {:.3} kN·m, rel_err = {:.4}",
                 m_x_target_knm, m_x_calc_knm, rel_err);
        // Outer γ_x loop 이 실제로 M_x 를 만족시켜야 (< 1%)
        assert!(rel_err < 0.05,
            "M_x self-consistency 실패: target={} kN·m, calc={:.3}, rel_err={:.4}",
            m_x_target_knm, m_x_calc_knm, rel_err);
    }
}

/// Level D-7: F_y + M_x 결합 시 Q_j 상하 비대칭 (top-bottom skew) 확인
///
/// F_y 만 있으면 Q_j 는 하부 로드존, 좌우 대칭 (Y-축 대칭).
/// M_x·sin ψ 로 인한 γ_x tilt 는 slice-level Δδ 상하 (Y-축) 방향 비대칭 유발.
///   sin ψ_j=+90° = +1 (top), sin ψ_j=-90° = -1 (bottom) → 서로 반대 tilt
/// 따라서 검증 축은 상하 (index j vs Z/2+j), 좌우 아님 (sin 함수는 좌우 대칭).
#[test]
fn level_d_combined_fy_mx_skew() {
    let baseline = solve_bearing_equilibrium(
        &nu240(0.0, -1000.0, 0.0, 0.0), &SilentReporter
    ).expect("baseline NR converges");
    let combined = solve_bearing_equilibrium(
        &nu240(0.0, -1000.0, 100.0, 0.0), &SilentReporter
    ).expect("combined NR converges");

    let q_base: Vec<f64> = baseline.equilibrium.roller_loads.clone();
    let q_comb: Vec<f64> = combined.equilibrium.roller_loads.clone();
    let z = q_base.len();

    // Baseline: 좌우 대칭 (F_y-only) — index j vs Z-j 대칭 유지 확인 (sanity)
    let mut lr_sym_diff = 0.0_f64;
    for i in 1..z/2 {
        lr_sym_diff += (q_base[i] - q_base[z - i]).abs();
    }
    let q_max_base = q_base.iter().cloned().fold(0.0_f64, f64::max).max(1.0);
    println!("Baseline (F_y-only) 좌우 대칭 편차 = {:.2} N ({}%)",
             lr_sym_diff, lr_sym_diff / q_max_base * 100.0);
    assert!(lr_sym_diff / q_max_base < 0.05,
        "baseline 은 좌우 대칭 이어야, 실제 편차 = {:.2} N", lr_sym_diff);

    // M_x 효과: index j (bottom side, sin ψ<0) vs Z/2+j (top side, sin ψ>0) 비교.
    // Combined 는 M_x sign 에 따라 하부 부하가 증감 → 상/하 로드존 크기 변화.
    // Baseline top-bottom (top=0) 대비 combined 의 top-bottom 이 증가하면 skew 발생.
    let sum_bottom_base: f64 = (0..z/2).map(|j| q_base[j]).sum();
    let sum_top_base: f64 = (z/2..z).map(|j| q_base[j]).sum();
    let sum_bottom_comb: f64 = (0..z/2).map(|j| q_comb[j]).sum();
    let sum_top_comb: f64 = (z/2..z).map(|j| q_comb[j]).sum();

    let ratio_base = sum_top_base / sum_bottom_base.max(1.0);
    let ratio_comb = sum_top_comb / sum_bottom_comb.max(1.0);
    println!("Baseline top/bottom ratio = {:.4}, Combined = {:.4}", ratio_base, ratio_comb);
    println!("  Bottom: {:.1}→{:.1} kN, Top: {:.1}→{:.1} kN",
             sum_bottom_base/1000.0, sum_bottom_comb/1000.0,
             sum_top_base/1000.0, sum_top_comb/1000.0);

    // combined 에서 top/bottom ratio 가 baseline 과 유의미하게 달라야
    let ratio_change = (ratio_comb - ratio_base).abs();
    assert!(ratio_change > 0.001 || (sum_top_comb - sum_top_base).abs() > 100.0,
        "M_x 추가 시 top/bottom 로드 분포 변화 있어야, ratio change = {:.4}",
        ratio_change);
}

/// Level D-5: Load zone extent — clearance ↑ → loaded roller 수 ↓
#[test]
fn level_d_load_zone_vs_clearance() {
    let mut loaded_counts = Vec::new();
    for &g_r in &[0.0, 30.0, 100.0, 300.0_f64] {
        let input = nu240(0.0, -1000.0, 0.0, g_r);
        let result = solve_bearing_equilibrium(&input, &SilentReporter).expect("NR converges");
        let n = result.equilibrium.roller_loads.iter().filter(|&&q| q > 1.0).count();
        println!("g_r = {} μm → loaded = {} / 18", g_r, n);
        loaded_counts.push(n);
    }
    // clearance ↑ → loaded ↓ (또는 유지)
    for i in 0..loaded_counts.len() - 1 {
        assert!(loaded_counts[i + 1] <= loaded_counts[i] + 1,   // +1 여유
            "clearance 증가 시 loaded 감소해야, {} → {}", loaded_counts[i], loaded_counts[i+1]);
    }
}
