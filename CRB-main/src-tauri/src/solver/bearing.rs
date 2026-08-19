// CRB Bearing-Level Equilibrium Solver — Phase 4 정식 구현
// ─────────────────────────────────────────────────────────────────────
// ISO 16281 A.3.1 (ISO p. 22) Cylindrical roller bearing internal load distribution.
//
// 평형 DOF = 3 (Plan §6 D4+D6+D7):
//   disp[0] = δ_x  radial displacement X (수평)      [μm]
//   disp[1] = δ_y  radial displacement Y (수직, 중력)  [μm]
//   disp[2] = γ_x  misalignment about X-axis           [rad]
//   (제거: δ_z (D4: F_a=0), γ_y (D6: single-plane))
//
// 좌표계 (D5): X=수평 radial, Y=수직(중력), Z=shaft axis
// 접촉력 방향 (D1): 순수 radial (α=0, rib 없음)
// Single row (D3)
//
// Newton-Raphson 반복:
//   [J]·Δ{disp} = -{R},   {R} = Σⱼ Q_j·[cos ψⱼ, sin ψⱼ, (d_pw/2)·sin ψⱼ] − [F_x, F_y, M_x]
//
// Phase 5+ 에서 재활성화될 life/static_rating/thermal_speed 는 현재 Default 값으로 채움.

use crate::error::SolverError;
use crate::solver::gen1;
use crate::solver::geometry::compute_slices;
use crate::solver::types::*;

// ─── 순수 계산 함수 (Phase 1 stub 에서 유지) ────────────────────────

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

/// Roller approach [μm] — CRB 3-DOF.
///   δ_rigid(ψ) = δ_x·cos ψ + δ_y·sin ψ
///                + (d_pw/2)·(γ_x + γ_ext)·sin ψ · 1000
///                − g_r / 2
pub fn roller_approach(
    disp: &[f64; 3],
    psi: f64,
    d_pw: f64,
    g_r: f64,
    gamma_ext: f64,
) -> f64 {
    let (cos_psi, sin_psi) = (psi.cos(), psi.sin());
    let delta_r = disp[0] * cos_psi + disp[1] * sin_psi;
    let gamma_x_total = disp[2] + gamma_ext;
    let axial_arm = (d_pw / 2.0) * 1000.0 * gamma_x_total * sin_psi;
    delta_r + axial_arm - g_r / 2.0
}

// ─── Level 1: Bearing Equilibrium (3-DOF Newton-Raphson) ────────────

const NR_MAX_ITER: usize = 100;
const NR_TOL_REL: f64 = 1e-5;    // relative residual tolerance
const NR_FD_STEP_DISP: f64 = 0.01;   // finite-difference step for δ [μm]
const NR_FD_STEP_GAMMA: f64 = 1e-6;  // finite-difference step for γ [rad]

/// Compute residual R[3] and per-roller Q_j given displacement.
/// Uses Gen1 (independent slice) as base — fast for NR loop.
fn compute_residual_gen1(
    input: &BearingInput,
    slices: &[SliceGeometry],
    disp: &[f64; 3],
) -> Result<([f64; 3], Vec<RollerResult>), SolverError> {
    let mg = &input.macro_geom;
    let z = mg.z;
    let d_pw = mg.d_pw;
    let cos_alpha_diff = 1.0; // CRB: α=0

    let f_x = input.operating.f_x * 1000.0; // kN → N
    let f_y = input.operating.f_y * 1000.0;
    let m_x = input.operating.m_x * 1e6;    // kN·m → N·mm
    let gamma_ext = input.operating.gamma_rad();

    let load_angle = radial_load_angle(f_x, f_y);
    let positions = roller_positions(z, load_angle);

    let mut residual = [0.0_f64; 3];
    let mut roller_results = Vec::with_capacity(z as usize);

    for &psi in positions.iter() {
        let (cos_psi, sin_psi) = (psi.cos(), psi.sin());
        let delta_rigid = roller_approach(disp, psi, d_pw, mg.g_r, gamma_ext);

        let (slice_results, q_normal) = if delta_rigid > 0.0 {
            gen1::solve_gen1_roller(slices, delta_rigid, &input.material, cos_alpha_diff)
        } else {
            // No contact — empty slice results
            (Vec::new(), 0.0)
        };

        // Residual accumulation (CRB: radial + tilting M_x only)
        residual[0] += q_normal * cos_psi;                        // F_x direction
        residual[1] += q_normal * sin_psi;                        // F_y direction
        residual[2] += q_normal * (d_pw / 2.0) * sin_psi;         // M_x direction

        roller_results.push(RollerResult {
            psi_deg: psi.to_degrees(),
            q_normal,
            q_normal_inner: q_normal,   // CRB: α=0 → inner = outer normal load
            slice_results,
            rib_result: None,           // D1: no rib
        });
    }

    residual[0] -= f_x;
    residual[1] -= f_y;
    residual[2] -= m_x;

    Ok((residual, roller_results))
}

/// Numerical Jacobian via forward finite differences.
fn compute_jacobian_gen1(
    input: &BearingInput,
    slices: &[SliceGeometry],
    disp: &[f64; 3],
    r0: &[f64; 3],
) -> Result<[[f64; 3]; 3], SolverError> {
    let steps = [NR_FD_STEP_DISP, NR_FD_STEP_DISP, NR_FD_STEP_GAMMA];
    let mut jac = [[0.0_f64; 3]; 3];

    for k in 0..3 {
        let mut disp_p = *disp;
        disp_p[k] += steps[k];
        let (r_p, _) = compute_residual_gen1(input, slices, &disp_p)?;
        for i in 0..3 {
            jac[i][k] = (r_p[i] - r0[i]) / steps[k];
        }
    }
    Ok(jac)
}

/// Solve 3×3 linear system via Gaussian elimination (small system).
fn solve_3x3(jac: [[f64; 3]; 3], b: [f64; 3]) -> Option<[f64; 3]> {
    let m = jac;
    let det = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
    if det.abs() < 1e-30 { return None; }

    // Cramer's rule (small 3x3)
    let mut x = [0.0_f64; 3];
    for k in 0..3 {
        let mut mk = m;
        for row in 0..3 { mk[row][k] = b[row]; }
        let det_k = mk[0][0] * (mk[1][1] * mk[2][2] - mk[1][2] * mk[2][1])
            - mk[0][1] * (mk[1][0] * mk[2][2] - mk[1][2] * mk[2][0])
            + mk[0][2] * (mk[1][0] * mk[2][1] - mk[1][1] * mk[2][0]);
        x[k] = det_k / det;
    }
    Some(x)
}

/// Initial displacement guess — TRB 원본 스타일 (k_radial stiffness).
///
///   k_roller ≈ 500 N/μm (typical single-roller Hertz stiffness)
///   k_radial = (Z/2) · k_roller  (CRB α=0: cos α = 1)
///   δ_x = clamp(f_x / k_radial, -50, 50) μm
///   δ_y = clamp(f_y / k_radial, -50, 50) μm
fn initial_disp_guess(input: &BearingInput) -> [f64; 3] {
    let z = input.macro_geom.z as f64;
    let f_x = input.operating.f_x * 1000.0;   // kN → N
    let f_y = input.operating.f_y * 1000.0;

    let k_roller = 500.0_f64;                  // N/μm (typical Hertz)
    let k_radial = (z / 2.0) * k_roller;       // CRB: cos α = 1

    let dx = if f_x.abs() > 1.0 {
        (f_x / k_radial).clamp(-50.0, 50.0)
    } else { 0.0 };
    let dy = if f_y.abs() > 1.0 {
        (f_y / k_radial).clamp(-50.0, 50.0)
    } else { 0.0 };

    [dx, dy, 0.0]
}

/// CRB 3-DOF Newton-Raphson equilibrium solver.
///
/// ISO 16281 A.3.1 based (Cylindrical roller bearings).
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
        detail: "Newton-Raphson (Gen1 base)".into(),
        percent: 20.0,
    });

    // NR loop
    let mut disp = initial_disp_guess(input);
    let mut residual = [0.0_f64; 3];
    let mut roller_results: Vec<RollerResult> = Vec::new();
    let mut converged = false;

    let f_x = input.operating.f_x * 1000.0;
    let f_y = input.operating.f_y * 1000.0;
    let m_x = input.operating.m_x * 1e6;
    let f_ref = ((f_x * f_x + f_y * f_y).sqrt() + m_x.abs() / (input.macro_geom.d_pw / 2.0))
        .max(1.0);

    for iter in 0..NR_MAX_ITER {
        let (r, res) = compute_residual_gen1(input, &slices, &disp)?;
        residual = r;
        roller_results = res;

        let r_norm = (r[0] * r[0] + r[1] * r[1] + (r[2] / (input.macro_geom.d_pw / 2.0)).powi(2)).sqrt();
        let rel = r_norm / f_ref;

        if rel < NR_TOL_REL {
            converged = true;
            progress.report(SolverProgress {
                stage: "Equilibrium".into(),
                detail: format!("Converged in {} iterations, rel={:.2e}", iter, rel),
                percent: 60.0,
            });
            break;
        }

        let jac = compute_jacobian_gen1(input, &slices, &disp, &r)?;
        let neg_r = [-r[0], -r[1], -r[2]];
        let dx = solve_3x3(jac, neg_r).ok_or_else(||
            SolverError::ConvergenceFailure("Jacobian singular at NR step".into()))?;

        // TRB 원본 스타일: step 크기 clamp (5~30 μm) + Line search 20회 + best_alpha
        let disp_mag = (disp[0] * disp[0] + disp[1] * disp[1]).sqrt();
        let max_step = (disp_mag * 0.5).clamp(5.0, 30.0);
        let step_norm = (dx[0] * dx[0] + dx[1] * dx[1]).sqrt();
        let step_scale = if step_norm > max_step {
            max_step / step_norm
        } else { 1.0 };
        let dx_scaled = [dx[0] * step_scale, dx[1] * step_scale, dx[2] * step_scale];

        // Line search: 20회 반감, best_alpha 유지
        let mut alpha_ls = 1.0_f64;
        let mut best_alpha = 0.0_f64;
        let mut best_norm = r_norm;
        for _ in 0..20 {
            let mut disp_try = disp;
            for k in 0..3 { disp_try[k] += alpha_ls * dx_scaled[k]; }
            let (r_try, _) = compute_residual_gen1(input, &slices, &disp_try)?;
            let r_try_norm = (r_try[0].powi(2) + r_try[1].powi(2)
                + (r_try[2] / (input.macro_geom.d_pw / 2.0)).powi(2)).sqrt();
            if r_try_norm < best_norm {
                best_norm = r_try_norm;
                best_alpha = alpha_ls;
            }
            if r_try_norm < r_norm {
                break;
            }
            alpha_ls *= 0.5;
        }
        if best_alpha < 1e-15 {
            best_alpha = alpha_ls;   // last (smallest) alpha as fallback
        }
        for k in 0..3 { disp[k] += best_alpha * dx_scaled[k]; }
    }

    if !converged {
        return Err(SolverError::ConvergenceFailure(
            format!("NR failed after {} iterations, residual {:?}", NR_MAX_ITER, residual)
        ));
    }

    // Post-process: build BearingResult
    progress.report(SolverProgress {
        stage: "Post-processing".into(),
        detail: "Building result".into(),
        percent: 80.0,
    });

    let load_angle = radial_load_angle(f_x, f_y);
    let equilibrium = BearingEquilibrium {
        displacement: [disp[0], disp[1], 0.0, disp[2], 0.0],  // [δx, δy, δz=0, γx, γy=0]
        roller_loads: roller_results.iter().map(|r| r.q_normal).collect(),
        angular_distribution: build_angular_distribution(&roller_results, input.macro_geom.z),
        roller_results,
    };

    let elapsed_ms = t0.elapsed().as_secs_f64() * 1000.0;
    let result = build_bearing_result(input, &slices, equilibrium, SolverMode::Gen1, elapsed_ms, load_angle);
    Ok(result)
}

/// CRB Dual-mode: Gen1 + Gen3 comparison.
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

    // Gen3 for CRB: identical result in this Phase 4 minimal impl
    // (Phase 3 Level C 검증됨: flat + 균일 D_we → Gen3 = Gen1)
    // 후속 Phase: Gen3 를 실제로 별개 pass 로 돌리도록 확장
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

fn build_angular_distribution(roller_results: &[RollerResult], _z: u32) -> Vec<AngularLoadPoint> {
    roller_results.iter().map(|r| {
        let p_max = r.slice_results.iter().map(|s| s.p_max_k).fold(0.0_f64, f64::max);
        let slice_p_max: Vec<f64> = r.slice_results.iter().map(|s| s.p_max_k).collect();
        AngularLoadPoint {
            psi_deg: r.psi_deg,
            delta_rigid: 0.0,   // Phase 4 minimal: rigid approach 미저장
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

    // GeometrySummary (Phase 4 minimal — CRB 원통 대응)
    let d_we_mean = mg.d_we;
    let volume_roller_mm3 = std::f64::consts::PI * (d_we_mean / 2.0).powi(2) * mg.l_we;
    let mass_roller_g = volume_roller_mm3 * mat.density_roller * 1e-3;
    let mass_rollers_total_g = mass_roller_g * (mg.z as f64);

    let e_star = crate::solver::hertz::combined_elastic_modulus(
        mat.e_roller, mat.nu, mat.e_ring, mat.nu);
    let geometry = GeometrySummary {
        roller_taper_angle_deg: 0.0,      // CRB: β=0
        roller_taper_angle_rad: 0.0,
        e_star_gpa: e_star,
        d_we_mean,
        cone_angle_deg: 0.0,              // CRB: α=0
        gamma_dw: d_we_mean / mg.d_pw,
        contact_length_ratio: mg.l_we / d_we_mean,
        f_r_kn: (input.operating.f_x.powi(2) + input.operating.f_y.powi(2)).sqrt(),
        f_a_kn: 0.0,                      // D4
        gamma_rad: input.operating.gamma_rad(),
        slice_geometries: slices.to_vec(),
        mass_roller_g,
        mass_rollers_total_g,
        mass_inner_race_g: 0.0,           // Phase 5+ 정확 계산
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
