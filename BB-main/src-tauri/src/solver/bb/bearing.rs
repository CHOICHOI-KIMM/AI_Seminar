// BB Contact Analysis — 베어링 평형 솔버 (5-DOF)
//
// BB Phase 3-1 (2026-08-20): CRB 3-DOF 솔버를 백지에서 교체.
//
// ── 근거 ────────────────────────────────────────────────────────────
// BB_Development_Theory.md §4 (특히 §4.4 「본 SW 가 구현하는 확정 형태」, §4.5 문헌 계보).
// 확정형은 Harris & Mindel (1973) 식 (81)(82)(86)~(90) 의 정적 환원형과 동일하다.
//   - 모멘트 팔은 `R_i` 로 통일 (D-9b) — ISO (A.8) 의 `D_pw/2` 가 아니다
//   - 틸트 항은 선형 `R_i γ` (D-9c) — ISO (A.2) 의 `sin ψ` 가 아니다
//   - 이 조합이 **가상일 공액**이므로 야코비안이 대칭·양반정부호가 된다
//
// ── 좌표계 (D-7) ────────────────────────────────────────────────────
// X = 회전축, Y·Z = 반경방향. 미지수 (δ_x, δ_y, δ_z, γ_y, γ_z).
//
// ── 단위 (D-10) ─────────────────────────────────────────────────────
// mm · N · rad. 이 파일에 단위 환산 상수는 없다.
//
// ── 정식화 ──────────────────────────────────────────────────────────
// 볼 j (φ_j = 2π(j−1)/Z) 에 대해
//
//   X_j = A sin α₀ + δ_x − R_i (γ_z cos φ_j − γ_y sin φ_j)      축 성분
//   R_j = A cos α₀ + δ_y cos φ_j + δ_z sin φ_j                  반경 성분
//   L_j = √(X_j² + R_j²),   δ_j = max(0, L_j − A)
//   sin α_j = X_j/L_j,      cos α_j = R_j/L_j
//   Q_j = c_P δ_j^(3/2)
//
// 내부 일반화 힘 (포텐셜의 기울기):
//
//   g(u) = Σ_j Q_j · v_j ,   v_j = ∂δ_j/∂u = sin α_j · a_j + cos α_j · b_j
//
//   a_j = ∂X_j/∂u = [1, 0, 0, +R_i sin φ_j, −R_i cos φ_j]
//   b_j = ∂R_j/∂u = [0, cos φ_j, sin φ_j, 0, 0]
//
// 야코비안 (포텐셜의 헤시안) — a_j·b_j 가 u 에 무관하므로
//
//   J = Σ_j [ K_j · v_j v_jᵀ + (Q_j/L_j) · w_j w_jᵀ ] ,   w_j = cos α_j · a_j − sin α_j · b_j
//   K_j = dQ_j/dδ_j = 1.5 c_P √δ_j
//
// **볼당 rank-2 업데이트이며 대칭·양반정부호다.** 수치미분이 필요 없고,
// 이 구조 자체가 §4.5 의 에너지 공액 결론에 대한 코드 수준 확인이다.

use nalgebra::{Matrix5, Vector5};

use crate::error::SolverError;
use crate::solver::bb::geometry;
use crate::solver::bb::hertz;
use crate::solver::bb::types::*;
use crate::solver::common::types::{Alert, AlertLevel};
use crate::solver::common::util;

/// 미지수·잔차 대각 스케일링에 쓰는 길이 (= `R_i`).
///
/// 틸트 `γ` 를 `R_i γ` (길이 차원) 로, 모멘트 잔차를 `M/R_i` (힘 차원) 로 바꿔
/// 5×5 야코비안의 모든 성분을 같은 차원으로 만든다. 자의적 상수가 아니라
/// 기하에서 나오는 물리량이다 (Plan §3.4 스케일링 결정).
///
/// 스케일된 미지수: `ũ = [δ_x, δ_y, δ_z, R_i γ_y, R_i γ_z]` [mm]
/// 스케일된 잔차:   `r̃ = [F_x, F_y, F_z, M_y/R_i, M_z/R_i]` [N]
#[derive(Debug, Clone, Copy)]
struct BallKinematics {
    /// `a_j` — 스케일 공간에서 `[1, 0, 0, sin φ_j, −cos φ_j]` (R_i 가 빠짐)
    a: Vector5<f64>,
    /// `b_j` — `[0, cos φ_j, sin φ_j, 0, 0]`
    b: Vector5<f64>,
    phi: f64,
}

fn ball_kinematics(z: u32, phase0: f64) -> Vec<BallKinematics> {
    let n = z as usize;
    (0..n)
        .map(|j| {
            let phi = phase0 + std::f64::consts::TAU * (j as f64) / (n as f64);
            let (s, c) = phi.sin_cos();
            BallKinematics {
                a: Vector5::new(1.0, 0.0, 0.0, s, -c),
                b: Vector5::new(0.0, c, s, 0.0, 0.0),
                phi,
            }
        })
        .collect()
}

/// 한 볼의 순간 상태.
struct BallState {
    delta: f64,
    q: f64,
    /// dQ/dδ
    k: f64,
    sin_a: f64,
    cos_a: f64,
    l: f64,
    loaded: bool,
}

fn ball_state(
    kin: &BallKinematics,
    u: &Vector5<f64>,
    a_dist: f64,
    alpha_0: f64,
    c_p: f64,
) -> BallState {
    let (sa0, ca0) = alpha_0.sin_cos();
    let x = a_dist * sa0 + kin.a.dot(u);
    let r = a_dist * ca0 + kin.b.dot(u);
    let l = (x * x + r * r).sqrt();
    let delta = l - a_dist;
    if delta <= 0.0 || l <= 0.0 {
        return BallState {
            delta: 0.0,
            q: 0.0,
            k: 0.0,
            sin_a: if l > 0.0 { x / l } else { alpha_0.sin() },
            cos_a: if l > 0.0 { r / l } else { alpha_0.cos() },
            l: l.max(a_dist),
            loaded: false,
        };
    }
    BallState {
        delta,
        q: c_p * delta.powf(1.5),
        k: 1.5 * c_p * delta.sqrt(),
        sin_a: x / l,
        cos_a: r / l,
        l,
        loaded: true,
    }
}

/// 잔차 `g(u) − f_ext` (스케일 공간) 와 야코비안을 동시에 계산한다.
fn residual_and_jacobian(
    kins: &[BallKinematics],
    u: &Vector5<f64>,
    f_ext: &Vector5<f64>,
    a_dist: f64,
    alpha_0: f64,
    c_p: f64,
) -> (Vector5<f64>, Matrix5<f64>, u32) {
    let mut g = Vector5::zeros();
    let mut jac = Matrix5::zeros();
    let mut loaded = 0u32;

    for kin in kins {
        let st = ball_state(kin, u, a_dist, alpha_0, c_p);
        if !st.loaded {
            continue; // active set: 비접촉 볼은 잔차·야코비안에 기여하지 않는다
        }
        loaded += 1;
        let v = kin.a * st.sin_a + kin.b * st.cos_a;
        let w = kin.a * st.cos_a - kin.b * st.sin_a;
        g += v * st.q;
        jac += v * v.transpose() * st.k + w * w.transpose() * (st.q / st.l);
    }
    (g - f_ext, jac, loaded)
}

/// 초기 추정값 — `c_P` 기반 해석해 (자의적 상수 없음).
///
/// - 축: 순수 축하중 해석해 `F_x = Z c_P δ^(3/2) sin α₀` 에서 `δ`, 그리고 `δ_x ≈ δ/sin α₀`
/// - 반경: Sjövall 하중분포에서 `Q_max = F_r /(Z J_r cos α₀)`.
///   `J_r = 0.2288` 은 **Harris Table 7.4 의 ε = 0.5 (클리어런스 0) 값**이다 (Theory §9.1).
///   즉 하드코딩 상수가 아니라 문헌 표값이다.
/// - 틸트: 0
fn initial_guess(f_ext: &Vector5<f64>, z: u32, alpha_0: f64, c_p: f64) -> Vector5<f64> {
    /// Harris Table 6.1/7.4 계열 — 점접촉 Sjövall 적분 `J_r(ε = 0.5)`
    const J_R_AT_HALF: f64 = 0.2288;

    let zf = z as f64;
    let (sa0, ca0) = alpha_0.sin_cos();

    let axial = if f_ext[0].abs() > 0.0 && sa0.abs() > 1.0e-6 {
        let q = f_ext[0].abs() / (zf * sa0.abs());
        let delta = (q / c_p).powf(2.0 / 3.0);
        f_ext[0].signum() * delta / sa0.abs()
    } else {
        0.0
    };

    let f_r = (f_ext[1] * f_ext[1] + f_ext[2] * f_ext[2]).sqrt();
    let radial_mag = if f_r > 0.0 && ca0.abs() > 1.0e-6 {
        let q_max = f_r / (zf * J_R_AT_HALF * ca0.abs());
        let delta = (q_max / c_p).powf(2.0 / 3.0);
        delta / ca0.abs()
    } else {
        0.0
    };
    let (uy, uz) = if f_r > 0.0 {
        (radial_mag * f_ext[1] / f_r, radial_mag * f_ext[2] / f_r)
    } else {
        (0.0, 0.0)
    };

    // 틸트는 0 에서 시작한다 — 모멘트 하중의 해석적 초기값은 문헌에 없고,
    // 야코비안이 SPD 라 0 에서 출발해도 안정적으로 수렴한다.
    Vector5::new(axial, uy, uz, 0.0, 0.0)
}

/// 5-DOF 평형 해석 (한 위상에서).
///
/// `phase0` 은 케이지 위상 [rad] (D-8). `mask` 로 자유도를 구속한다 (D-1).
#[allow(clippy::too_many_arguments)]
fn solve_at_phase(
    geom: &BallBearingGeometry,
    derived: &BbGeometryDerived,
    contact: &BbContactDerived,
    operating: &BbOperatingConditions,
    params: &BbSolverParams,
    preload_n: f64,
    phase0: f64,
) -> Result<BbEquilibrium, SolverError> {
    let r_i = derived.r_i_center_mm;
    let kins = ball_kinematics(geom.z, phase0);

    // 외력 (스케일 공간): 모멘트는 R_i 로 나눠 힘 차원으로
    let f_ext = Vector5::new(
        operating.f_x_n + preload_n,
        operating.f_y_n,
        operating.f_z_n,
        operating.m_y_nmm / r_i,
        operating.m_z_nmm / r_i,
    );

    let dofs = params.dof_mask.as_array();
    let free: Vec<bool> = dofs.iter().map(|d| d.is_free()).collect();
    // 구속값을 스케일 공간으로: δ 는 그대로 [mm], γ 는 R_i 를 곱해 길이 차원으로
    let prescribed: Vec<f64> = dofs
        .iter()
        .enumerate()
        .map(|(i, d)| if i >= 3 { d.value() * r_i } else { d.value() })
        .collect();

    let mut u = initial_guess(&f_ext, geom.z, derived.alpha_0_rad, contact.c_p_n_per_mm15);
    // 구속 자유도는 지정값으로 고정
    for i in 0..5 {
        if !free[i] {
            u[i] = prescribed[i];
        }
    }

    // 잔차 정규화 기준 — 외력 크기, 하한은 최대 볼하중 규모
    let f_scale = f_ext.norm().max(1.0);

    let mut converged = false;
    let mut iterations = 0u32;
    let mut residual_norm = f64::INFINITY;
    let mut loaded = 0u32;

    for it in 0..params.max_iterations {
        iterations = it + 1;
        let (r_full, j_full, n_loaded) = residual_and_jacobian(
            &kins,
            &u,
            &f_ext,
            derived.a_mm,
            derived.alpha_0_rad,
            contact.c_p_n_per_mm15,
        );
        loaded = n_loaded;

        // 구속 자유도의 잔차는 판정에서 제외한다
        let mut r_masked = r_full;
        for (i, fr) in free.iter().enumerate() {
            if !fr {
                r_masked[i] = 0.0;
            }
        }
        residual_norm = r_masked.norm() / f_scale;
        if residual_norm < params.convergence_tol {
            converged = true;
            break;
        }
        if n_loaded == 0 {
            return Err(SolverError::ConvergenceFailure(
                "접촉하는 볼이 하나도 없습니다 — 하중·클리어런스 입력을 확인하십시오".into(),
            ));
        }

        // 구속 자유도를 소거한 축소계 구성
        let idx: Vec<usize> = (0..5).filter(|i| free[*i]).collect();
        let n = idx.len();
        let mut a = vec![0.0; n * n];
        let mut b = vec![0.0; n];
        for (ri, &i) in idx.iter().enumerate() {
            b[ri] = -r_full[i];
            for (ci, &c) in idx.iter().enumerate() {
                a[ri * n + ci] = j_full[(i, c)];
            }
        }
        let step_reduced = solve_dense(&mut a, &mut b, n).ok_or_else(|| {
            SolverError::ConvergenceFailure(format!(
                "야코비안이 특이합니다 (반복 {it}, 접촉 볼 {n_loaded}개)"
            ))
        })?;

        let mut step = Vector5::zeros();
        for (ri, &i) in idx.iter().enumerate() {
            step[i] = step_reduced[ri];
        }

        // backtracking line search — 단순 감소 조건
        let mut alpha = 1.0_f64;
        let mut accepted = false;
        for _ in 0..30 {
            let trial = u + step * alpha;
            let (rt, _, _) = residual_and_jacobian(
                &kins,
                &trial,
                &f_ext,
                derived.a_mm,
                derived.alpha_0_rad,
                contact.c_p_n_per_mm15,
            );
            let mut rt_m = rt;
            for (i, fr) in free.iter().enumerate() {
                if !fr {
                    rt_m[i] = 0.0;
                }
            }
            if rt_m.norm() / f_scale < residual_norm {
                u = trial;
                accepted = true;
                break;
            }
            alpha *= 0.5;
        }
        if !accepted {
            // 감소하는 스텝을 못 찾음 — 최소 스텝으로 전진하지 않고 종료
            break;
        }
    }

    // 결과 조립
    let mut ball_results = Vec::with_capacity(geom.z as usize);
    let mut q_max = 0.0_f64;
    for kin in &kins {
        let st = ball_state(kin, &u, derived.a_mm, derived.alpha_0_rad, contact.c_p_n_per_mm15);
        let (a_i, b_i, p_i) = hertz::contact_ellipse(
            contact.e_star_mpa,
            derived.sum_rho_i_per_mm,
            contact.a_star_inner,
            contact.b_star_inner,
            st.q,
        );
        let (a_e, b_e, p_e) = hertz::contact_ellipse(
            contact.e_star_mpa,
            derived.sum_rho_e_per_mm,
            contact.a_star_outer,
            contact.b_star_outer,
            st.q,
        );
        q_max = q_max.max(st.q);
        ball_results.push(BallResult {
            phi_rad: kin.phi.rem_euclid(std::f64::consts::TAU),
            delta_mm: st.delta,
            alpha_rad: st.sin_a.atan2(st.cos_a),
            q_n: st.q,
            loaded: st.loaded,
            a_inner_mm: a_i,
            b_inner_mm: b_i,
            p_max_inner_mpa: p_i,
            a_outer_mm: a_e,
            b_outer_mm: b_e,
            p_max_outer_mpa: p_e,
        });
    }

    // 스케일 공간 → 물리 단위 (γ = ũ / R_i)
    let displacement = Displacement {
        dx_mm: u[0],
        dy_mm: u[1],
        dz_mm: u[2],
        ry_rad: u[3] / r_i,
        rz_rad: u[4] / r_i,
    };

    Ok(BbEquilibrium {
        displacement,
        ball_results,
        q_max_n: q_max,
        loaded_count: loaded,
        converged,
        iterations,
        residual_norm,
    })
}

/// 부분피벗 가우스 소거 (n ≤ 5). 성공 시 해를 반환한다.
///
/// nalgebra 의 LU 를 쓰지 않는 이유: 구속 마스크 때문에 크기가 가변(1~5)이라
/// 고정크기 타입을 쓸 수 없고, 이 규모에서는 직접 소거가 더 단순하다.
fn solve_dense(a: &mut [f64], b: &mut [f64], n: usize) -> Option<Vec<f64>> {
    for k in 0..n {
        let mut piv = k;
        let mut best = a[k * n + k].abs();
        for r in (k + 1)..n {
            let v = a[r * n + k].abs();
            if v > best {
                best = v;
                piv = r;
            }
        }
        if best < 1.0e-300 {
            return None;
        }
        if piv != k {
            for c in 0..n {
                a.swap(k * n + c, piv * n + c);
            }
            b.swap(k, piv);
        }
        let d = a[k * n + k];
        for r in (k + 1)..n {
            let f = a[r * n + k] / d;
            if f == 0.0 {
                continue;
            }
            for c in k..n {
                a[r * n + c] -= f * a[k * n + c];
            }
            b[r] -= f * b[k];
        }
    }
    let mut x = vec![0.0; n];
    for k in (0..n).rev() {
        let mut s = b[k];
        for c in (k + 1)..n {
            s -= a[k * n + c] * x[c];
        }
        x[k] = s / a[k * n + k];
    }
    if x.iter().any(|v| !v.is_finite()) {
        return None;
    }
    Some(x)
}

/// 지정된 예압 하중 `F_a0` [N] (없으면 0).
fn preload_force(geom: &BallBearingGeometry) -> f64 {
    match geom.clearance {
        BbClearanceSpec::AxialPreloadN(f) => f,
        _ => 0.0,
    }
}

/// 강체(스페이서) 예압에서 구속할 축변위 `δ_x0` [mm] 를 `F_a0` 로부터 역산한다.
///
/// 무하중 상태에서 순수 축하중 `F_a0` 만 걸었을 때의 평형 축변위를 구한다.
/// 즉 **두 예압 모델은 무하중에서 정확히 같은 상태**를 준다 (Level C 판정 항목).
fn preload_displacement(
    geom: &BallBearingGeometry,
    derived: &BbGeometryDerived,
    contact: &BbContactDerived,
    params: &BbSolverParams,
    f_a0: f64,
) -> Result<f64, SolverError> {
    let axial_only = BbOperatingConditions {
        f_x_n: 0.0,
        f_y_n: 0.0,
        f_z_n: 0.0,
        m_y_nmm: 0.0,
        m_z_nmm: 0.0,
        n_inner_rpm: 0.0,
        n_outer_rpm: 0.0,
        temperature_c: 20.0,
    };
    let mut p = params.clone();
    // 축만 자유, 나머지는 0 구속 (순수 축하중 문제)
    p.dof_mask = BbDofMask {
        x: BbDof::Free,
        y: BbDof::Prescribed(0.0),
        z: BbDof::Prescribed(0.0),
        gy: BbDof::Prescribed(0.0),
        gz: BbDof::Prescribed(0.0),
    };
    p.phase_sweep = BbPhaseSweep {
        enabled: false,
        n_phase: 1,
    };
    let eq = solve_at_phase(geom, derived, contact, &axial_only, &p, f_a0, 0.0)?;
    if !eq.converged {
        return Err(SolverError::ConvergenceFailure(format!(
            "강체 예압의 δ_x0 역산이 수렴하지 않았습니다 (F_a0 = {f_a0} N)"
        )));
    }
    Ok(eq.displacement.dx_mm)
}

/// 5-DOF 평형 해석 (위상 스윕 포함).
pub fn solve_bearing(input: &BbInput) -> Result<BbResult, SolverError> {
    let t0 = std::time::Instant::now();
    input.validate()?;

    // 예압 지정이면 클리어런스 0 · α₀ = α_nom 으로 기하를 푼다
    let mut geom_for_derived = input.geometry.clone();
    let preload_n = preload_force(&input.geometry);
    if preload_n != 0.0 {
        geom_for_derived.clearance = BbClearanceSpec::InitialAngleRad(input.geometry.alpha_nom_rad);
    }

    let derived = geometry::compute_geometry_derived(&geom_for_derived)?;
    let contact = hertz::compute_contact_derived(&derived, &input.material)?;

    // 예압 모델 분기 (D-2)
    //   Spring — F_a0 를 외부 축하중에 더한다 (하중 제어)
    //   Rigid  — F_a0 로 역산한 δ_x0 를 구속한다 (변위 제어). 축 자유도가 사라지므로
    //            외부 축하중은 반력으로만 나타난다
    let mut params = input.solver.clone();
    let mut spring_force = 0.0;
    if preload_n != 0.0 {
        match params.preload_model {
            BbPreloadModel::Spring => spring_force = preload_n,
            BbPreloadModel::Rigid => {
                let dx0 =
                    preload_displacement(&geom_for_derived, &derived, &contact, &params, preload_n)?;
                params.dof_mask.x = BbDof::Prescribed(dx0);
            }
        }
    }
    let summary = geometry::compute_geometry_summary(
        &geom_for_derived,
        &derived,
        &input.operating,
        &input.material,
    );
    let mut alerts = geometry::collect_geometry_alerts(&summary);

    let base = solve_at_phase(
        &geom_for_derived,
        &derived,
        &contact,
        &input.operating,
        &params,
        spring_force,
        0.0,
    )?;

    // 케이지 위상 스윕 (D-8)
    let sweep = if params.phase_sweep.enabled {
        let n = params.phase_sweep.n_phase.max(1);
        let span = std::f64::consts::TAU / input.geometry.z as f64;
        let mut curve = Vec::with_capacity(n as usize);
        let mut worst_q = (f64::NEG_INFINITY, 0.0);
        let mut worst_p = (f64::NEG_INFINITY, 0.0);
        for i in 0..n {
            let phase0 = span * (i as f64) / (n as f64);
            let eq = solve_at_phase(
                &geom_for_derived,
                &derived,
                &contact,
                &input.operating,
                &params,
                spring_force,
                phase0,
            )?;
            let p = eq
                .ball_results
                .iter()
                .map(|b| b.p_max_inner_mpa.max(b.p_max_outer_mpa))
                .fold(0.0_f64, f64::max);
            if eq.q_max_n > worst_q.0 {
                worst_q = (eq.q_max_n, phase0);
            }
            if p > worst_p.0 {
                worst_p = (p, phase0);
            }
            curve.push((phase0, eq.q_max_n));
        }
        Some(BbPhaseSweepResult {
            worst_q_max_n: worst_q.0,
            worst_q_max_phase_rad: worst_q.1,
            worst_p_max_mpa: worst_p.0,
            worst_p_max_phase_rad: worst_p.1,
            curve,
        })
    } else {
        None
    };

    if !base.converged {
        alerts.push(Alert {
            level: AlertLevel::Critical,
            code: "NOT_CONVERGED".into(),
            message: format!(
                "평형 반복이 수렴하지 않았습니다 (반복 {}, 상대잔차 {:.3e}). \
                 결과를 신뢰할 수 없습니다",
                base.iterations, base.residual_norm
            ),
        });
    }

    let p_worst = base
        .ball_results
        .iter()
        .map(|b| b.p_max_inner_mpa.max(b.p_max_outer_mpa))
        .fold(0.0_f64, f64::max);
    if p_worst > hertz::SIGMA_HU_MPA {
        alerts.push(Alert {
            level: if p_worst > 4_000.0 {
                AlertLevel::Critical
            } else {
                AlertLevel::Warning
            },
            code: "CONTACT_STRESS_OVER_FATIGUE_LIMIT".into(),
            message: format!(
                "최대 접촉응력 {p_worst:.0} MPa 가 ISO 281 Annex B.3.1 권장 피로한계 1500 MPa 를 초과합니다"
            ),
        });
    }

    Ok(BbResult {
        geometry: summary,
        equilibrium: base,
        phase_sweep: sweep,
        alerts,
        elapsed_ms: util::duration_ms(t0.elapsed()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::common::types::Material;

    fn fixture(z: u32) -> BallBearingGeometry {
        let d_w_mm = 11.5;
        let (r_i_mm, r_e_mm) = BallBearingGeometry::reference_groove_radii(d_w_mm);
        BallBearingGeometry {
            bore_mm: 50.0,
            outer_diameter_mm: 90.0,
            width_mm: 20.0,
            z,
            d_w_mm,
            d_pw_mm: 70.0,
            r_i_mm,
            r_e_mm,
            alpha_nom_rad: 40.0_f64.to_radians(),
            clearance: BbClearanceSpec::InitialAngleRad(40.0_f64.to_radians()),
        }
    }

    fn input(z: u32, fx: f64, fy: f64) -> BbInput {
        BbInput {
            kind: BallBearingKind::Acbb,
            geometry: fixture(z),
            material: Material::default(),
            operating: BbOperatingConditions {
                f_x_n: fx,
                f_y_n: fy,
                f_z_n: 0.0,
                m_y_nmm: 0.0,
                m_z_nmm: 0.0,
                n_inner_rpm: 1000.0,
                n_outer_rpm: 0.0,
                temperature_c: 70.0,
            },
            solver: BbSolverParams::default(),
        }
    }

    #[test]
    fn pure_axial_converges_and_is_symmetric() {
        let r = solve_bearing(&input(16, 5_000.0, 0.0)).unwrap();
        assert!(r.equilibrium.converged, "미수렴: {:?}", r.equilibrium.residual_norm);
        assert_eq!(r.equilibrium.loaded_count, 16);
        // 순수 축하중이면 모든 볼이 같은 하중
        let q0 = r.equilibrium.ball_results[0].q_n;
        for b in &r.equilibrium.ball_results {
            assert!((b.q_n - q0).abs() / q0 < 1e-12, "볼 하중 비대칭: {} vs {q0}", b.q_n);
        }
    }

    #[test]
    fn jacobian_is_symmetric() {
        // 가상일 공액 구조의 코드 수준 확인 (Theory §4.5)
        let inp = input(16, 3_000.0, 2_000.0);
        let d = geometry::compute_geometry_derived(&inp.geometry).unwrap();
        let c = hertz::compute_contact_derived(&d, &inp.material).unwrap();
        let kins = ball_kinematics(inp.geometry.z, 0.0);
        let u = Vector5::new(0.01, 0.008, 0.003, 1e-4, 2e-4);
        let f = Vector5::zeros();
        let (_, j, _) = residual_and_jacobian(&kins, &u, &f, d.a_mm, d.alpha_0_rad, c.c_p_n_per_mm15);
        for r in 0..5 {
            for cc in 0..5 {
                let a = j[(r, cc)];
                let b = j[(cc, r)];
                let scale = a.abs().max(b.abs()).max(1.0);
                assert!((a - b).abs() / scale < 1e-12, "J[{r}][{cc}] 비대칭: {a} vs {b}");
            }
        }
    }

    #[test]
    fn combined_load_converges() {
        let r = solve_bearing(&input(16, 5_000.0, 3_000.0)).unwrap();
        assert!(r.equilibrium.converged);
        assert!(r.equilibrium.loaded_count > 0 && r.equilibrium.loaded_count <= 16);
        assert!(r.equilibrium.q_max_n > 0.0);
    }

    #[test]
    fn iso_3dof_mask_zeroes_constrained_dof() {
        let mut inp = input(16, 5_000.0, 3_000.0);
        inp.solver.dof_mask = BbDofMask::ISO_3DOF;
        let r = solve_bearing(&inp).unwrap();
        assert!(r.equilibrium.converged);
        assert_eq!(r.equilibrium.displacement.dz_mm, 0.0, "δ_z 가 구속되지 않음");
        assert_eq!(r.equilibrium.displacement.ry_rad, 0.0, "γ_y 가 구속되지 않음");
    }

    #[test]
    fn preload_models_agree_without_external_load() {
        // 무하중에서 스프링 예압과 강체 예압은 **완전히 같은 해**를 준다 (Level C 항목)
        let mut a = input(16, 0.0, 0.0);
        a.geometry.clearance = BbClearanceSpec::AxialPreloadN(2_000.0);
        a.solver.preload_model = BbPreloadModel::Spring;
        let mut b = a.clone();
        b.solver.preload_model = BbPreloadModel::Rigid;

        let ra = solve_bearing(&a).unwrap();
        let rb = solve_bearing(&b).unwrap();
        assert!(ra.equilibrium.converged && rb.equilibrium.converged);
        let dxa = ra.equilibrium.displacement.dx_mm;
        let dxb = rb.equilibrium.displacement.dx_mm;
        assert!((dxa - dxb).abs() / dxa.abs() < 1e-9, "δ_x0: 스프링 {dxa} vs 강체 {dxb}");
        assert!(
            (ra.equilibrium.q_max_n - rb.equilibrium.q_max_n).abs() / ra.equilibrium.q_max_n < 1e-9
        );
    }

    #[test]
    fn preload_models_diverge_under_axial_load() {
        // 축하중을 걸면 스프링은 δ_x 가 늘고, 강체는 δ_x 가 고정된다
        let mut a = input(16, 3_000.0, 0.0);
        a.geometry.clearance = BbClearanceSpec::AxialPreloadN(2_000.0);
        a.solver.preload_model = BbPreloadModel::Spring;
        let mut b = a.clone();
        b.solver.preload_model = BbPreloadModel::Rigid;

        let mut no_load = a.clone();
        no_load.operating.f_x_n = 0.0;
        let dx0 = solve_bearing(&no_load).unwrap().equilibrium.displacement.dx_mm;

        let ra = solve_bearing(&a).unwrap();
        let rb = solve_bearing(&b).unwrap();
        assert!(ra.equilibrium.displacement.dx_mm > dx0, "스프링: δ_x 가 늘어야 함");
        assert!(
            (rb.equilibrium.displacement.dx_mm - dx0).abs() / dx0.abs() < 1e-9,
            "강체: δ_x 가 δ_x0 에 고정돼야 함"
        );
        assert!(ra.equilibrium.q_max_n > rb.equilibrium.q_max_n, "스프링이 더 큰 볼하중");
    }

    #[test]
    fn preload_loads_all_balls_without_external_force() {
        let mut inp = input(16, 0.0, 0.0);
        inp.geometry.clearance = BbClearanceSpec::AxialPreloadN(2_000.0);
        let r = solve_bearing(&inp).unwrap();
        assert!(r.equilibrium.converged);
        assert_eq!(r.equilibrium.loaded_count, 16, "예압 상태에서 전 볼 접촉이어야 함");
        let q0 = r.equilibrium.ball_results[0].q_n;
        assert!(q0 > 0.0);
        for b in &r.equilibrium.ball_results {
            assert!((b.q_n - q0).abs() / q0 < 1e-12, "예압 하중 불균등");
        }
    }
}
