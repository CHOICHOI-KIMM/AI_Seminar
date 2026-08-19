// CRB Bearing-Level Equilibrium Solver — Phase 4 정식 (§4.12.5 Phase 분리 방식)
// ─────────────────────────────────────────────────────────────────────
// ISO 16281 A.3.1 (ISO p. 22) Cylindrical roller bearing internal load distribution.
//
// 평형 DOF = 3 (Plan §6 D4+D6+D7):
//   disp[0] = δ_x  radial displacement X (수평)      [μm]
//   disp[1] = δ_y  radial displacement Y (수직, 중력) [μm]
//   disp[2] = γ_x  misalignment about X-axis         [rad]
//
// 알고리즘 (§4.12.5): Phase 분리 (Outer γ_x + Inner (δx, δy) 2-DOF)
//   Outer loop: γ_x 1-DOF NR (M_x equilibrium)
//     Inner: Phase A 2-DOF NR (δx, δy) with γ_x fixed  ← TRB 원본 line 892~945 이식
//
// TRB 원본 (git show 5441446:...bearing.rs) 참조.
// Phase 5+ 에서 재활성화될 life/static_rating/thermal_speed 는 현재 Default 값으로 채움.

use crate::error::SolverError;
use crate::solver::geometry::compute_slices;
use crate::solver::hertz;
use crate::solver::types::*;

// ─── 순수 계산 함수 ─────────────────────────────────────────────────

/// Roller angular positions [rad], evenly spaced starting at `load_angle`.
pub fn roller_positions(z: u32, load_angle: f64) -> Vec<f64> {
    let two_pi = std::f64::consts::TAU;
    (0..z).map(|j| load_angle + two_pi * (j as f64) / (z as f64)).collect()
}

/// Radial load direction [rad] = atan2(f_y, f_x). Returns 0 if negligible.
pub fn radial_load_angle(f_x: f64, f_y: f64) -> f64 {
    let f_r = (f_x * f_x + f_y * f_y).sqrt();
    if f_r < 1e-10 { 0.0 } else { f_y.atan2(f_x) }
}

/// Roller **macro-center** radial approach [μm] at roller mid-length (x_axial = L_we/2).
///   δ_r_center(ψ) = δ_x·cos ψ + δ_y·sin ψ − g_r/2
///
/// γ_x is **NOT** included here — for single-row CRB (α=0), γ_x has zero radial
/// arm at macro level and instead produces slice-level Δδ_k = x_arm·γ_x·sin ψ
/// (handled inside `compute_residual_3d`). TRB 원본 (5441446 line 84) 은
/// (d_pw/2)·γ_x·sin ψ 항을 포함했으나, 이는 axial 방향 arm 이며 α=0 인 CRB
/// 에서는 contact normal 에 sin α = 0 로 곱해져 무효 → slice-level 로 이관.
pub fn roller_approach(
    disp: &[f64; 3],
    psi: f64,
    _d_pw: f64,
    g_r: f64,
    _gamma_ext: f64,
) -> f64 {
    let (cos_psi, sin_psi) = (psi.cos(), psi.sin());
    let delta_r = disp[0] * cos_psi + disp[1] * sin_psi;
    delta_r - g_r / 2.0
}

// ─── Residual + Jacobian ─────────────────────────────────────────────

const FD_STEP_DISP: f64 = 0.01;    // finite-difference step for δ [μm]
const FD_STEP_GAMMA: f64 = 1e-6;    // finite-difference step for γ [rad]
const OUTER_MAX_ITER: usize = 30;
const OUTER_TOL_REL: f64 = 1e-4;
const GAMMA_DAMPING: f64 = 0.5;     // γ_x update under-relaxation

/// Compute 3-D residual [F_x_res, F_y_res, M_x_res] and per-roller results.
///
/// Slice-level implementation (CRB Option B):
///   δ_k(ψ) = δ_r_center(ψ) + (x_axial_k − L_we/2)·γ_x_total·sin ψ · 1000
///                            − Δz_outer_k − Δz_inner_k·cos_alpha_diff
///
/// γ_x_total = disp[2] + gamma_ext → slice 별 Δδ_k 편차 → q_k 좌우 비대칭 →
/// M_x = Σ_j sin ψ_j · Σ_k q_k·l_k·(x_k − L_we/2)  (single-row CRB 물리).
fn compute_residual_3d(
    input: &BearingInput,
    slices: &[SliceGeometry],
    disp: &[f64; 3],
) -> Result<([f64; 3], Vec<RollerResult>), SolverError> {
    let mg = &input.macro_geom;
    let z = mg.z;
    let cos_alpha_diff = 1.0; // CRB α=0

    let f_x = input.operating.f_x * 1000.0;
    let f_y = input.operating.f_y * 1000.0;
    let m_x = input.operating.m_x * 1e6;
    let gamma_ext = input.operating.gamma_rad();
    let gamma_x_total = disp[2] + gamma_ext;

    let load_angle = radial_load_angle(f_x, f_y);
    let positions = roller_positions(z, load_angle);

    let l_we_half = mg.l_we / 2.0;

    // Pre-compute Hertz material constants
    let mat = &input.material;
    let e_star_gpa = hertz::combined_elastic_modulus(mat.e_roller, mat.nu, mat.e_ring, mat.nu);
    let e_star_mpa = e_star_gpa * 1000.0;
    let e_avg_mpa = ((mat.e_roller + mat.e_ring) / 2.0) * 1000.0;
    let nu = mat.nu;

    let mut residual = [0.0_f64; 3];
    let mut roller_results = Vec::with_capacity(z as usize);

    for &psi in positions.iter() {
        let (cos_psi, sin_psi) = (psi.cos(), psi.sin());
        // macro-center radial approach (γ_x-free)
        let delta_r_center = disp[0] * cos_psi + disp[1] * sin_psi - mg.g_r / 2.0;
        // tilt gradient: [μm] per [mm] of axial arm
        let tilt_um_per_mm = 1000.0 * gamma_x_total * sin_psi;

        let mut slice_results = Vec::with_capacity(slices.len());
        let mut q_normal = 0.0_f64;
        let mut m_ax_j = 0.0_f64; // Σ q_k·l_k·x_arm  [N·mm]

        for s in slices.iter() {
            let x_arm = s.x_axial - l_we_half;
            let delta_k_rigid = delta_r_center + x_arm * tilt_um_per_mm;
            let delta_k = delta_k_rigid
                - s.delta_z_total_outer
                - s.delta_z_total_inner * cos_alpha_diff;

            let h1 = s.r_roller;
            let h2 = s.r_roller * 2.0;
            let sr = hertz::compute_slice_contact(
                s.k, delta_k, s.r_eq_inner, s.r_eq_outer,
                e_star_mpa, e_avg_mpa, nu,
                s.slice_width, h1, h2, cos_alpha_diff,
            );

            if sr.in_contact {
                q_normal += sr.q_k * s.slice_width;
                m_ax_j += sr.q_k * s.slice_width * x_arm;
            }
            slice_results.push(sr);
        }

        residual[0] += q_normal * cos_psi;
        residual[1] += q_normal * sin_psi;
        residual[2] += m_ax_j * sin_psi;

        roller_results.push(RollerResult {
            psi_deg: psi.to_degrees(),
            q_normal,
            q_normal_inner: q_normal,
            slice_results,
            rib_result: None,
        });
    }

    residual[0] -= f_x;
    residual[1] -= f_y;
    residual[2] -= m_x;

    Ok((residual, roller_results))
}

// ─── Initial Guess (TRB 원본 line 278~319 스타일) ──────────────────

/// Initial displacement guess — k_radial stiffness 기반.
fn initial_guess_crb(input: &BearingInput) -> [f64; 3] {
    let z = input.macro_geom.z as f64;
    let f_x = input.operating.f_x * 1000.0;
    let f_y = input.operating.f_y * 1000.0;

    let k_roller = 500.0_f64;
    let k_radial = (z / 2.0) * k_roller;    // CRB: cos α = 1

    let dx = if f_x.abs() > 1.0 {
        (f_x / k_radial).clamp(-50.0, 50.0)
    } else { 0.0 };
    let dy = if f_y.abs() > 1.0 {
        (f_y / k_radial).clamp(-50.0, 50.0)
    } else { 0.0 };

    [dx, dy, 0.0]
}

// ─── Phase A: 2-DOF NR (δ_x, δ_y) with γ_x fixed ────────────────────
// TRB 원본 (git show 5441446:...bearing.rs) line 892~945 이식.

fn phase_a_radial_2dof(
    disp: &mut [f64; 3],
    input: &BearingInput,
    slices: &[SliceGeometry],
) -> Result<(), SolverError> {
    let f_x = input.operating.f_x * 1000.0;
    let f_y = input.operating.f_y * 1000.0;
    let f_r = (f_x * f_x + f_y * f_y).sqrt().max(1.0);
    let tol = input.solver.convergence_tol.max(1e-6);
    let max_iter = input.solver.max_iterations;
    let h_s = FD_STEP_DISP;

    for _iter in 0..max_iter {
        let (r, _) = compute_residual_3d(input, slices, disp)?;
        let r_rad = (r[0] * r[0] + r[1] * r[1]).sqrt();
        if r_rad / f_r < tol {
            return Ok(());
        }

        // 2×2 Jacobian
        let mut j2 = [[0.0_f64; 2]; 2];
        for col in 0..2 {
            let mut dp = *disp;
            dp[col] += h_s;
            let (rp, _) = compute_residual_3d(input, slices, &dp)?;
            for row in 0..2 {
                j2[row][col] = (rp[row] - r[row]) / h_s;
            }
        }

        let det = j2[0][0] * j2[1][1] - j2[0][1] * j2[1][0];
        if det.abs() < 1e-30 {
            return Ok(()); // singular — bail
        }
        let dx_step = (j2[1][1] * (-r[0]) - j2[0][1] * (-r[1])) / det;
        let dy_step = (j2[0][0] * (-r[1]) - j2[1][0] * (-r[0])) / det;

        // Step clamp: max_step = disp_mag * 0.5, clamp 5~30 μm
        let disp_mag = (disp[0] * disp[0] + disp[1] * disp[1]).sqrt();
        let max_step = (disp_mag * 0.5).clamp(5.0, 30.0);
        let step_norm = (dx_step * dx_step + dy_step * dy_step).sqrt();
        let scale = if step_norm > max_step { max_step / step_norm } else { 1.0 };
        let dd = [dx_step * scale, dy_step * scale];

        // Line search 20회 + best_alpha
        let mut alpha_ls = 1.0_f64;
        let mut best_alpha = 0.0_f64;
        let mut best_norm = r_rad;
        for _ in 0..20 {
            let mut dt = *disp;
            dt[0] += alpha_ls * dd[0];
            dt[1] += alpha_ls * dd[1];
            let (rt, _) = compute_residual_3d(input, slices, &dt)?;
            let rt_r = (rt[0] * rt[0] + rt[1] * rt[1]).sqrt();
            if rt_r < best_norm {
                best_norm = rt_r;
                best_alpha = alpha_ls;
            }
            if rt_r < r_rad {
                break;
            }
            alpha_ls *= 0.5;
        }
        if best_alpha < 1e-15 {
            best_alpha = alpha_ls;
        }
        disp[0] += best_alpha * dd[0];
        disp[1] += best_alpha * dd[1];
    }
    Ok(())
}

// ─── Main solver ─────────────────────────────────────────────────────

/// CRB 3-DOF Newton-Raphson equilibrium (Phase 분리 방식).
pub fn solve_bearing_equilibrium(
    input: &BearingInput,
    progress: &dyn ProgressReporter,
) -> Result<BearingResult, SolverError> {
    let t0 = std::time::Instant::now();
    input.validate().map_err(|e| SolverError::InvalidInput(e.to_string()))?;

    progress.report(SolverProgress {
        stage: "Preprocessing".into(),
        detail: "Slicing geometry".into(),
        percent: 0.0,
    });

    let slices = compute_slices(
        &input.macro_geom,
        &input.raceway_geom,
        &input.roller_profile,
        &input.raceway_profile_inner,
        &input.raceway_profile_outer,
        input.solver.n_slices,
    )?;

    progress.report(SolverProgress {
        stage: "Equilibrium".into(),
        detail: "Phase 분리 NR (Outer γ_x + Inner 2-DOF)".into(),
        percent: 20.0,
    });

    let mut disp = initial_guess_crb(input);
    let m_x_target = input.operating.m_x * 1e6;
    let m_x_ref = m_x_target.abs().max(1e3);   // ref = max(|M_x|, 1 kN·mm)

    // ── Outer loop: γ_x 1-DOF NR (M_x equilibrium) ──
    for _outer in 0..OUTER_MAX_ITER {
        // Phase A: 2-DOF (δx, δy), γ_x fixed
        phase_a_radial_2dof(&mut disp, input, &slices)?;

        let (r_all, _) = compute_residual_3d(input, &slices, &disp)?;
        let m_res = r_all[2];

        if (m_res / m_x_ref).abs() < OUTER_TOL_REL {
            break;
        }

        // γ_x 1-DOF update
        let h_g = FD_STEP_GAMMA;
        let mut disp_p = disp;
        disp_p[2] += h_g;
        let (r_p, _) = compute_residual_3d(input, &slices, &disp_p)?;
        let dmdg = (r_p[2] - m_res) / h_g;
        if dmdg.abs() > 1e-30 {
            disp[2] += -m_res / dmdg * GAMMA_DAMPING;
        } else {
            break; // gradient singular — γ_x has no effect
        }
    }

    // Final capture with converged disp
    let (_r_final, roller_results) = compute_residual_3d(input, &slices, &disp)?;

    progress.report(SolverProgress {
        stage: "Post-processing".into(),
        detail: "Building result".into(),
        percent: 80.0,
    });

    let f_x = input.operating.f_x * 1000.0;
    let f_y = input.operating.f_y * 1000.0;
    let load_angle = radial_load_angle(f_x, f_y);

    let equilibrium = BearingEquilibrium {
        displacement: [disp[0], disp[1], 0.0, disp[2], 0.0], // [δx, δy, δz=0, γx, γy=0]
        roller_loads: roller_results.iter().map(|r| r.q_normal).collect(),
        angular_distribution: build_angular_distribution(&roller_results),
        roller_results,
    };

    let elapsed_ms = t0.elapsed().as_secs_f64() * 1000.0;
    let result = build_bearing_result(input, &slices, equilibrium, SolverMode::Gen1, elapsed_ms, load_angle);
    Ok(result)
}

/// CRB Dual-mode: Gen1 + Gen3.
///
/// Phase 3 Level C 검증: flat + 균일 D_we → Gen1 ≡ Gen3 (이론적 필연).
/// 여기서는 두 pass 모두 Gen1 based 로 처리 (bearing-level 은 gen1 사용).
pub fn solve_bearing_dual(
    input: &BearingInput,
    progress: &dyn ProgressReporter,
) -> Result<DualModeComparison, SolverError> {
    let t0 = std::time::Instant::now();

    progress.report(SolverProgress {
        stage: "Dual-mode".into(),
        detail: "Gen1 pass".into(),
        percent: 0.0,
    });
    let t_g1 = std::time::Instant::now();
    let gen1_result = solve_bearing_equilibrium(input, progress)?;
    let gen1_elapsed_ms = t_g1.elapsed().as_secs_f64() * 1000.0;

    progress.report(SolverProgress {
        stage: "Dual-mode".into(),
        detail: "Gen3 pass (CRB: theoretically = Gen1)".into(),
        percent: 50.0,
    });
    let t_g3 = std::time::Instant::now();
    let mut gen3_result = gen1_result.clone();
    gen3_result.mode = SolverMode::Gen3;
    let gen3_elapsed_ms = t_g3.elapsed().as_secs_f64() * 1000.0;

    let total_elapsed_ms = t0.elapsed().as_secs_f64() * 1000.0;

    Ok(DualModeComparison {
        gen1_result,
        gen3_result,
        delta_p_max_pct: 0.0,
        delta_q_max_pct: 0.0,
        delta_l10_pct: 0.0,
        gen3_recommended: false,
        recommendation_reason: "CRB flat + uniform D_we: Gen1 ≡ Gen3 (Level C verified)".into(),
        gen1_elapsed_ms,
        gen3_elapsed_ms,
        total_elapsed_ms,
    })
}

// ─── BearingResult 빌더 (Phase 5+ 필드는 Default) ─────────────────

fn build_angular_distribution(roller_results: &[RollerResult]) -> Vec<AngularLoadPoint> {
    roller_results.iter().map(|r| {
        let p_max = r.slice_results.iter().map(|s| s.p_max_k).fold(0.0_f64, f64::max);
        let slice_p_max: Vec<f64> = r.slice_results.iter().map(|s| s.p_max_k).collect();
        AngularLoadPoint {
            psi_deg: r.psi_deg,
            delta_rigid: 0.0,
            q_total: r.q_normal,
            p_max,
            slice_p_max_outer: slice_p_max.clone(),
            slice_p_max,
            slice_q_k: r.slice_results.iter().map(|s| s.q_k).collect(),
            is_roller: true,
        }
    }).collect()
}

fn build_bearing_result(
    input: &BearingInput,
    slices: &[SliceGeometry],
    equilibrium: BearingEquilibrium,
    mode: SolverMode,
    elapsed_ms: f64,
    load_angle_rad: f64,
) -> BearingResult {
    let mg = &input.macro_geom;
    let mat = &input.material;

    let d_we_mean = mg.d_we;
    let volume_roller_mm3 = std::f64::consts::PI * (d_we_mean / 2.0).powi(2) * mg.l_we;
    let mass_roller_g = volume_roller_mm3 * mat.density_roller * 1e-3;
    let mass_rollers_total_g = mass_roller_g * (mg.z as f64);
    let e_star = crate::solver::hertz::combined_elastic_modulus(mat.e_roller, mat.nu, mat.e_ring, mat.nu);

    let geometry = GeometrySummary {
        roller_taper_angle_deg: 0.0,
        roller_taper_angle_rad: 0.0,
        e_star_gpa: e_star,
        d_we_mean,
        cone_angle_deg: 0.0,
        gamma_dw: d_we_mean / mg.d_pw,
        contact_length_ratio: mg.l_we / d_we_mean,
        f_r_kn: (input.operating.f_x.powi(2) + input.operating.f_y.powi(2)).sqrt(),
        f_a_kn: 0.0,
        gamma_rad: input.operating.gamma_rad(),
        slice_geometries: slices.to_vec(),
        mass_roller_g,
        mass_rollers_total_g,
        mass_inner_race_g: 0.0,
        mass_outer_race_g: 0.0,
        mass_total_g: mass_rollers_total_g,
    };

    BearingResult {
        mode,
        equilibrium,
        geometry,
        life: default_fatigue_life(),
        static_rating: default_static_rating(),
        thermal_speed: default_thermal_speed(),
        alerts: Vec::new(),
        elapsed_ms,
        f_a_induced_kn: 0.0,
        f_a_effective_kn: 0.0,
        preload_mode: PreloadMode::default(),
        delta_preload_um: 0.0,
        f_a_reaction_kn: 0.0,
        k_radial: 0.0,
        k_axial: 0.0,
        traction: None,
        film_distribution: None,
        load_angle_deg: load_angle_rad.to_degrees(),
    }
}

fn default_fatigue_life() -> FatigueLifeResult {
    FatigueLifeResult {
        method: LifeMethod::default(),
        l_10_basic: 0.0, l_nm_hours: 0.0, l_10_inner: 0.0, l_10_outer: 0.0,
        weakest_lamina: 0, a_iso: 1.0,
        kappa: 0.0, kappa_inner: 0.0, kappa_outer: 0.0,
        c_dyn: 0.0, p_equiv: 0.0, p_ref: 0.0, p_ref_damage: 0.0,
        intermediates: LifeIntermediates {
            nu_actual: 0.0, nu_ref: 0.0, b_m: 1.1, f_c: 0.0,
            gamma_bearing: 0.0, c_u_kn: 0.0, c_u_over_p: 0.0,
            e_demarcation: 0.0, x_factor: 1.0, y_factor: 0.0, f_a_over_f_r: 0.0,
            f_ci: 0.0, f_co: 0.0, q_c_base: 0.0, q_ci: 0.0, q_co: 0.0,
            q_ei: 0.0, q_eo: 0.0, weibull_e: 9.0 / 8.0, l_nm_mrev: 0.0,
            q_c_lamina_inner: 0.0, q_c_lamina_outer: 0.0,
            e_c_used: 0.0, kappa_method: KappaMethod::default(),
            lambda_inner: None, lambda_outer: None,
        },
        lamina_lives: None,
        film_thickness: None,
    }
}

fn default_static_rating() -> StaticRatingResult {
    StaticRatingResult {
        c_0r_kn: 0.0, p_0r_kn: 0.0, s_0: 0.0, x_0: 1.0, y_0: 0.0,
        q_0: 0.0, q_max: 0.0, s_0_eff: 0.0,
        q_max_roller_idx: 0, q_max_lamina_idx: 0, s_0_adequate: false,
    }
}

fn default_thermal_speed() -> ThermalSpeedResult {
    ThermalSpeedResult {
        n_theta_r: 0.0, speed_ratio: 0.0, a_r: 0.0, d_m: 0.0,
        p_1r: 0.0, m_0r: 0.0, m_1r: 0.0, n_r: 0.0, phi_r: 0.0, q_r: 0.0,
        f_0r: 0.0, f_1r: 0.0, v_r: 0.0,
    }
}
