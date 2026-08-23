// Level C 검증 — 5-DOF 평형 해석해 (Plan Phase 3-1)
//
// 판정 대상: BB_Development_Theory.md §4 (§4.4 확정 형태).
// 실행: cargo test --test equilibrium_level_c
//
// ── 검증 철학 ───────────────────────────────────────────────────────
// 솔버가 낸 답을 **솔버로 다시 확인하면 동어반복**이다. 여기서는
//  ① 평형 잔차를 **결과(φ_j, α_j, Q_j)만으로 독립 재조립**해 외력과 맞는지 본다 (C-1)
//  ② 순수 축하중은 **테스트 안에서 1-D 스칼라 방정식을 따로 풀어** 대조한다 (C-2)
//  ③ 나머지는 대칭성·불변성·단조성 같은 **구조적 성질**로 판정한다
//
// Level D-1(Harris Table 7.4) 이 유일한 외부 문헌 검증이며, 여기는 자체 정합성 단계다.

use bb_core::solver::bb::bearing::solve_bearing;
use bb_core::solver::bb::types::*;
use bb_core::solver::common::types::*;

const D_W: f64 = 11.5;
const D_PW: f64 = 70.0;
const ALPHA_DEG: f64 = 40.0;
const Z: u32 = 16;

fn geometry(z: u32) -> BallBearingGeometry {
    let (r_i_mm, r_e_mm) = BallBearingGeometry::reference_groove_radii(D_W);
    BallBearingGeometry {
        bore_mm: 50.0,
        outer_diameter_mm: 90.0,
        width_mm: 20.0,
        z,
        d_w_mm: D_W,
        d_pw_mm: D_PW,
        r_i_mm,
        r_e_mm,
        alpha_nom_rad: ALPHA_DEG.to_radians(),
        clearance: BbClearanceSpec::InitialAngleRad(ALPHA_DEG.to_radians()),
    }
}

fn make(fx: f64, fy: f64, fz: f64, my: f64, mz: f64) -> BbInput {
    BbInput {
        geometry: geometry(Z),
        material: Material::default(),
        operating: BbOperatingConditions {
            f_x_n: fx,
            f_y_n: fy,
            f_z_n: fz,
            m_y_nmm: my,
            m_z_nmm: mz,
            n_inner_rpm: 1000.0,
            n_outer_rpm: 0.0,
            temperature_c: 70.0,
        },
        solver: BbSolverParams::default(),
    }
}

// ═══════════════════════════════════════════════════════════════════
//  C-1. 평형 잔차 독립 재조립  ★ 가장 강한 자체 검증
// ═══════════════════════════════════════════════════════════════════

/// 결과의 `(φ_j, α_j, Q_j)` 만으로 5개 평형식을 다시 세워 외력과 대조한다.
/// 솔버 내부 상태를 전혀 쓰지 않으므로 동어반복이 아니다.
fn check_equilibrium(inp: &BbInput, tol: f64) {
    let r = solve_bearing(inp).unwrap();
    assert!(
        r.equilibrium.converged,
        "미수렴 (잔차 {:.3e})",
        r.equilibrium.residual_norm
    );
    let r_i = r.geometry.r_i_center_mm;

    let (mut fx, mut fy, mut fz, mut my, mut mz) = (0.0, 0.0, 0.0, 0.0, 0.0);
    for b in &r.equilibrium.ball_results {
        let (s, c) = b.alpha_rad.sin_cos();
        let (sp, cp) = b.phi_rad.sin_cos();
        fx += b.q_n * s;
        fy += b.q_n * c * cp;
        fz += b.q_n * c * sp;
        // Theory §4.4 확정 형태 — 팔은 R_i (D-9b)
        my += r_i * b.q_n * s * sp;
        mz -= r_i * b.q_n * s * cp;
    }

    let op = &inp.operating;
    let scale = (op.f_x_n.abs() + op.f_y_n.abs() + op.f_z_n.abs()).max(1.0);
    let m_scale = (op.m_y_nmm.abs() + op.m_z_nmm.abs()).max(scale * r_i);

    assert!((fx - op.f_x_n).abs() / scale < tol, "F_x: {fx} vs {}", op.f_x_n);
    assert!((fy - op.f_y_n).abs() / scale < tol, "F_y: {fy} vs {}", op.f_y_n);
    assert!((fz - op.f_z_n).abs() / scale < tol, "F_z: {fz} vs {}", op.f_z_n);
    assert!((my - op.m_y_nmm).abs() / m_scale < tol, "M_y: {my} vs {}", op.m_y_nmm);
    assert!((mz - op.m_z_nmm).abs() / m_scale < tol, "M_z: {mz} vs {}", op.m_z_nmm);
}

#[test]
fn c1_equilibrium_residual_pure_axial() {
    check_equilibrium(&make(5_000.0, 0.0, 0.0, 0.0, 0.0), 1e-8);
}

#[test]
fn c1b_equilibrium_residual_combined() {
    check_equilibrium(&make(5_000.0, 3_000.0, 0.0, 0.0, 0.0), 1e-8);
}

#[test]
fn c1c_equilibrium_residual_full_5dof() {
    // 5-DOF 전부 활성 — 2축 반경 + 2축 모멘트
    check_equilibrium(&make(4_000.0, 2_500.0, 1_200.0, 6_000.0, 9_000.0), 1e-8);
}

// ═══════════════════════════════════════════════════════════════════
//  C-2. 순수 축하중 — 독립 1-D 해석해 대조
// ═══════════════════════════════════════════════════════════════════

/// 순수 축하중에서는 모든 볼이 동일하므로 미지수가 `δ_x` 하나다.
/// 테스트 안에서 이분법으로 직접 풀어 솔버와 대조한다 (독립 경로).
fn axial_scalar_solution(a_mm: f64, alpha_0: f64, c_p: f64, z: u32, f_x: f64) -> f64 {
    let (sa0, ca0) = alpha_0.sin_cos();
    let f = |dx: f64| {
        let x = a_mm * sa0 + dx;
        let rr = a_mm * ca0;
        let l = (x * x + rr * rr).sqrt();
        let d = l - a_mm;
        if d <= 0.0 {
            return -f_x;
        }
        (z as f64) * c_p * d.powf(1.5) * (x / l) - f_x
    };
    let (mut lo, mut hi) = (0.0_f64, 1.0_f64);
    while f(hi) < 0.0 {
        hi *= 2.0;
        assert!(hi < 1e6, "브래킷 확보 실패");
    }
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if f(mid) > 0.0 {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    0.5 * (lo + hi)
}

#[test]
fn c2_pure_axial_matches_scalar_solution() {
    use bb_core::solver::bb::geometry::compute_geometry_derived;
    use bb_core::solver::bb::hertz::compute_contact_derived;

    for f_x in [500.0, 2_000.0, 5_000.0, 20_000.0] {
        let mut inp = make(f_x, 0.0, 0.0, 0.0, 0.0);
        // 기본 tol(1e-8)에서는 δ_x 상대오차가 ~3e-9 로 남는다.
        // 스칼라해(이분법 200회)는 사실상 정확하므로, 솔버를 조여 비교 강도를 올린다.
        inp.solver.convergence_tol = 1e-13;
        let d = compute_geometry_derived(&inp.geometry).unwrap();
        let c = compute_contact_derived(&d, &inp.material).unwrap();

        let want = axial_scalar_solution(d.a_mm, d.alpha_0_rad, c.c_p_n_per_mm15, Z, f_x);
        let got = solve_bearing(&inp).unwrap().equilibrium.displacement[0];
        assert!(
            (got - want).abs() / want < 1e-9,
            "F_x={f_x}: 솔버 δ_x={got:.9e} vs 스칼라해 {want:.9e}"
        );
    }
}

#[test]
fn c2b_pure_axial_is_perfectly_symmetric() {
    let r = solve_bearing(&make(5_000.0, 0.0, 0.0, 0.0, 0.0)).unwrap();
    assert_eq!(r.equilibrium.loaded_count, Z);
    let b0 = &r.equilibrium.ball_results[0];
    for b in &r.equilibrium.ball_results {
        assert!((b.q_n - b0.q_n).abs() / b0.q_n < 1e-12, "Q 비대칭");
        assert!((b.delta_mm - b0.delta_mm).abs() / b0.delta_mm < 1e-12, "δ 비대칭");
        assert!((b.alpha_rad - b0.alpha_rad).abs() < 1e-14, "α 비대칭");
    }
    // 하중이 걸리면 운전 접촉각이 초기 접촉각보다 커진다
    assert!(
        b0.alpha_rad > ALPHA_DEG.to_radians(),
        "α_j = {} 가 α₀ 이하",
        b0.alpha_rad.to_degrees()
    );
}

// ═══════════════════════════════════════════════════════════════════
//  C-3. 예압 (D-2, 두 모델)
// ═══════════════════════════════════════════════════════════════════

fn preload_input(model: BbPreloadModel, fx: f64) -> BbInput {
    let mut inp = make(fx, 0.0, 0.0, 0.0, 0.0);
    inp.geometry.clearance = BbClearanceSpec::AxialPreloadN(2_000.0);
    inp.solver.preload_model = model;
    inp
}

#[test]
fn c3_preload_loads_all_balls_uniformly() {
    for model in [BbPreloadModel::Spring, BbPreloadModel::Rigid] {
        let r = solve_bearing(&preload_input(model, 0.0)).unwrap();
        assert!(r.equilibrium.converged, "{model:?} 미수렴");
        assert_eq!(r.equilibrium.loaded_count, Z, "{model:?}: 전 볼 접촉이어야 함");
        let q0 = r.equilibrium.ball_results[0].q_n;
        assert!(q0 > 0.0);
        for b in &r.equilibrium.ball_results {
            assert!((b.q_n - q0).abs() / q0 < 1e-12, "{model:?}: 예압 하중 불균등");
        }
    }
}

#[test]
fn c3b_preload_models_agree_without_external_load() {
    // 강체 예압의 δ_x0 는 스프링 예압의 무하중 해로 역산되므로 둘이 같아야 한다
    let a = solve_bearing(&preload_input(BbPreloadModel::Spring, 0.0)).unwrap();
    let b = solve_bearing(&preload_input(BbPreloadModel::Rigid, 0.0)).unwrap();
    let (da, db) = (a.equilibrium.displacement[0], b.equilibrium.displacement[0]);
    assert!((da - db).abs() / da.abs() < 1e-9, "δ_x0: 스프링 {da} vs 강체 {db}");
    let (qa, qb) = (a.equilibrium.q_max_n, b.equilibrium.q_max_n);
    assert!((qa - qb).abs() / qa < 1e-9, "Q_max: {qa} vs {qb}");
}

#[test]
fn c3c_preload_models_diverge_under_axial_load() {
    let dx0 = solve_bearing(&preload_input(BbPreloadModel::Spring, 0.0))
        .unwrap()
        .equilibrium
        .displacement[0];
    let a = solve_bearing(&preload_input(BbPreloadModel::Spring, 3_000.0)).unwrap();
    let b = solve_bearing(&preload_input(BbPreloadModel::Rigid, 3_000.0)).unwrap();

    assert!(
        a.equilibrium.displacement[0] > dx0,
        "스프링: 축하중이 걸리면 δ_x 가 늘어야 함"
    );
    assert!(
        (b.equilibrium.displacement[0] - dx0).abs() / dx0.abs() < 1e-9,
        "강체: δ_x 가 δ_x0 에 고정돼야 함"
    );
    assert!(
        a.equilibrium.q_max_n > b.equilibrium.q_max_n,
        "정력 예압이 더 큰 볼하중을 낸다"
    );
}

// ═══════════════════════════════════════════════════════════════════
//  C-4. 회전 불변성
// ═══════════════════════════════════════════════════════════════════

#[test]
fn c4_pitch_rotation_invariance() {
    // 반경하중을 볼 피치(2π/Z) 만큼 돌리면 하중분포는 인덱스만 한 칸 밀린 동일 집합이어야 한다.
    let f = 4_000.0;
    let pitch = std::f64::consts::TAU / Z as f64;

    let base = solve_bearing(&make(3_000.0, f, 0.0, 0.0, 0.0)).unwrap();
    let (s, c) = pitch.sin_cos();
    let rot = solve_bearing(&make(3_000.0, f * c, f * s, 0.0, 0.0)).unwrap();

    let mut q_base: Vec<f64> = base.equilibrium.ball_results.iter().map(|b| b.q_n).collect();
    let mut q_rot: Vec<f64> = rot.equilibrium.ball_results.iter().map(|b| b.q_n).collect();
    q_base.sort_by(|a, b| a.partial_cmp(b).unwrap());
    q_rot.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let qmax = q_base.last().copied().unwrap().max(1.0);
    for (a, b) in q_base.iter().zip(q_rot.iter()) {
        assert!((a - b).abs() / qmax < 1e-9, "피치 회전 불변성 위반: {a} vs {b}");
    }
}

#[test]
fn c4b_radial_magnitude_invariance() {
    // 같은 크기의 반경하중이면 방향이 달라도 Q_max 는 (위상 스윕 기준) 같아야 한다.
    let f = 4_000.0;
    let mut worst = Vec::new();
    for deg in [0.0_f64, 37.0, 90.0, 213.0] {
        let (s, c) = deg.to_radians().sin_cos();
        let mut inp = make(3_000.0, f * c, f * s, 0.0, 0.0);
        // 위상 스윕은 이산 표본이라 방향마다 표본점이 달라진다.
        // 표본을 늘리면 오차가 O(Δφ²) 로 줄어드는 것을 이용한다 (24 → 180 이면 ~1/56).
        inp.solver.phase_sweep = BbPhaseSweep {
            enabled: true,
            n_phase: 180,
        };
        let r = solve_bearing(&inp).unwrap();
        worst.push(r.phase_sweep.unwrap().worst_q_max_n);
    }
    let w0 = worst[0];
    for w in &worst {
        assert!((w - w0).abs() / w0 < 1e-5, "방향 불변성 위반: {w} vs {w0}");
    }
}

// ═══════════════════════════════════════════════════════════════════
//  C-5. 위상 스윕 (D-8)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn c5_phase_sweep_bounds_and_periodicity() {
    let mut inp = make(2_000.0, 5_000.0, 0.0, 0.0, 0.0);
    inp.solver.phase_sweep = BbPhaseSweep {
        enabled: true,
        n_phase: 36,
    };
    let r = solve_bearing(&inp).unwrap();
    let sw = r.phase_sweep.as_ref().unwrap();

    assert_eq!(sw.curve.len(), 36);
    // φ₀ = 0 은 기본해와 같아야 한다
    assert!(
        (sw.curve[0].1 - r.equilibrium.q_max_n).abs() / r.equilibrium.q_max_n < 1e-12,
        "φ₀=0 이 기본해와 불일치"
    );
    // 최악값은 곡선의 최댓값이고 기본해 이상이다
    let curve_max = sw.curve.iter().map(|p| p.1).fold(0.0_f64, f64::max);
    assert!((sw.worst_q_max_n - curve_max).abs() / curve_max < 1e-12);
    assert!(sw.worst_q_max_n >= r.equilibrium.q_max_n * (1.0 - 1e-12));
    // 스윕 구간은 [0, 2π/Z)
    let pitch = std::f64::consts::TAU / Z as f64;
    assert!(sw.curve.last().unwrap().0 < pitch);
    // 반경하중이 있으면 위상에 따라 실제로 변동한다
    let curve_min = sw.curve.iter().map(|p| p.1).fold(f64::INFINITY, f64::min);
    assert!(
        (curve_max - curve_min) / curve_max > 1e-4,
        "반경하중인데 위상 변동이 없음"
    );
}

#[test]
fn c5b_pure_axial_has_no_phase_dependence() {
    let mut inp = make(5_000.0, 0.0, 0.0, 0.0, 0.0);
    inp.solver.phase_sweep = BbPhaseSweep {
        enabled: true,
        n_phase: 12,
    };
    let r = solve_bearing(&inp).unwrap();
    let sw = r.phase_sweep.as_ref().unwrap();
    let q0 = sw.curve[0].1;
    for (_, q) in &sw.curve {
        assert!((q - q0).abs() / q0 < 1e-12, "순수 축하중인데 위상 의존이 있음");
    }
}

// ═══════════════════════════════════════════════════════════════════
//  C-6. DOF 구속 (D-1)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn c6_iso_3dof_constrains_and_still_equilibrates() {
    // ISO_3DOF 는 δ_z·γ_y 를 0 으로 묶는다. 그 평면 안에서는 평형이 성립해야 한다.
    let mut inp = make(4_000.0, 3_000.0, 0.0, 0.0, 5_000.0);
    inp.solver.dof_mask = BbDofMask::ISO_3DOF;
    let r = solve_bearing(&inp).unwrap();
    assert!(r.equilibrium.converged);
    assert_eq!(r.equilibrium.displacement[2], 0.0, "δ_z 미구속");
    assert_eq!(r.equilibrium.displacement[3], 0.0, "γ_y 미구속");

    // 자유 자유도(F_x, F_y, M_z)의 평형만 재조립해 확인
    let r_i = r.geometry.r_i_center_mm;
    let (mut fx, mut fy, mut mz) = (0.0, 0.0, 0.0);
    for b in &r.equilibrium.ball_results {
        let (s, c) = b.alpha_rad.sin_cos();
        let cp = b.phi_rad.cos();
        fx += b.q_n * s;
        fy += b.q_n * c * cp;
        mz -= r_i * b.q_n * s * cp;
    }
    let scale = 7_000.0_f64;
    assert!((fx - 4_000.0).abs() / scale < 1e-8, "F_x = {fx}");
    assert!((fy - 3_000.0).abs() / scale < 1e-8, "F_y = {fy}");
    assert!((mz - 5_000.0).abs() / (scale * r_i) < 1e-8, "M_z = {mz}");
}

#[test]
fn c6b_prescribed_axial_displacement_produces_reaction() {
    // δ_x 를 구속하면(강체 예압과 같은 기구) 그 방향은 반력이 된다.
    let dx0 = solve_bearing(&preload_input(BbPreloadModel::Spring, 0.0))
        .unwrap()
        .equilibrium
        .displacement[0];

    let mut inp = make(0.0, 0.0, 0.0, 0.0, 0.0);
    inp.solver.dof_mask = BbDofMask {
        x: BbDof::Prescribed(dx0),
        ..BbDofMask::FULL
    };
    let r = solve_bearing(&inp).unwrap();
    assert!((r.equilibrium.displacement[0] - dx0).abs() / dx0.abs() < 1e-12);

    // 반력 = Σ Q_j sin α_j 가 원래 예압 하중 2 kN 이어야 한다
    let fx: f64 = r
        .equilibrium
        .ball_results
        .iter()
        .map(|b| b.q_n * b.alpha_rad.sin())
        .sum();
    assert!((fx - 2_000.0).abs() / 2_000.0 < 1e-6, "반력 {fx} N ≠ 예압 2000 N");
}

// ═══════════════════════════════════════════════════════════════════
//  C-7. 물리 정합 · 단조성
// ═══════════════════════════════════════════════════════════════════

#[test]
fn c7_radial_load_concentrates_near_load_direction() {
    // +Y 반경하중 → φ = 0 근처 볼이 최대, 반대편(φ ≈ π)은 비접촉.
    //
    // ⚠ 이 성질은 **α₀ = 0 (DGBB, 클리어런스 0)** 에서 나타난다.
    //    α₀ = 40° 인 ACBB 는 축 방향 성분 때문에 작은 축하중에도 전 볼이 접촉한다
    //    (초안에서 이 전제를 틀리게 잡아 실패했다).
    let mut inp = make(0.0, 8_000.0, 0.0, 0.0, 0.0);
    inp.geometry.alpha_nom_rad = 0.0;
    inp.geometry.clearance = BbClearanceSpec::DiametralMm(0.0);
    let r = solve_bearing(&inp).unwrap();
    assert!(r.equilibrium.converged);
    let balls = &r.equilibrium.ball_results;
    let imax = balls
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.q_n.partial_cmp(&b.1.q_n).unwrap())
        .unwrap()
        .0;
    assert_eq!(imax, 0, "최대하중 볼이 φ=0 이 아님 (idx {imax})");
    // 클리어런스 0 · α₀ = 0 이면 하중측 절반만 접촉한다 (고전적 결과)
    assert!(
        r.equilibrium.loaded_count < Z && r.equilibrium.loaded_count >= Z / 2 - 1,
        "하중구간이 절반 근처여야 함 ({}개)",
        r.equilibrium.loaded_count
    );
    // 반대편 볼은 비접촉
    let opposite = &balls[(Z / 2) as usize];
    assert!(!opposite.loaded, "φ≈π 볼이 접촉 (Q={})", opposite.q_n);
}

#[test]
fn c7b_load_deflection_is_monotonic() {
    let mut prev = 0.0_f64;
    for f in [1_000.0, 2_000.0, 4_000.0, 8_000.0, 16_000.0] {
        let r = solve_bearing(&make(f, 0.0, 0.0, 0.0, 0.0)).unwrap();
        let dx = r.equilibrium.displacement[0];
        assert!(dx > prev, "F_x={f}: δ_x 단조성 위반");
        prev = dx;
    }
}

#[test]
fn c7c_contact_angle_rises_with_axial_share() {
    // 축하중 비중이 커지면 운전 접촉각이 커진다
    let mut prev = 0.0_f64;
    for fx in [500.0, 2_000.0, 5_000.0, 12_000.0] {
        let r = solve_bearing(&make(fx, 2_000.0, 0.0, 0.0, 0.0)).unwrap();
        let a = r.equilibrium.ball_results[0].alpha_rad;
        assert!(a > prev, "F_x={fx}: 접촉각 단조성 위반 ({} deg)", a.to_degrees());
        prev = a;
    }
}

// ═══════════════════════════════════════════════════════════════════
//  C-8. 수렴 실패를 삼키지 않는다
// ═══════════════════════════════════════════════════════════════════

#[test]
fn c8_convergence_is_reported() {
    let r = solve_bearing(&make(5_000.0, 3_000.0, 0.0, 0.0, 0.0)).unwrap();
    assert!(r.equilibrium.converged);
    assert!(r.equilibrium.iterations >= 1);
    assert!(r.equilibrium.residual_norm.is_finite());
    assert!(r.equilibrium.residual_norm < BbSolverParams::default().convergence_tol);
    assert!(
        !r.alerts.iter().any(|a| a.code == "NOT_CONVERGED"),
        "정상 수렴인데 NOT_CONVERGED Alert 발생"
    );
}

#[test]
fn c8b_zero_load_with_clearance_is_a_trivial_equilibrium() {
    // 하중 0 · 클리어런스 있음 → 접촉 볼 0, 변위 0 이 **정당한 평형해**다.
    // (초안에서 이를 오류로 보았으나, 잔차가 0 이므로 자명해가 맞다.)
    // 솔버의 '접촉 볼 0' 오류 경로는 반복 도중 해가 접촉을 잃는 경우를 막는 가드다.
    let mut inp = make(0.0, 0.0, 0.0, 0.0, 0.0);
    inp.geometry.clearance = BbClearanceSpec::DiametralMm(0.05);
    let r = solve_bearing(&inp).unwrap();
    assert!(r.equilibrium.converged);
    assert_eq!(r.equilibrium.loaded_count, 0);
    assert_eq!(r.equilibrium.q_max_n, 0.0);
    assert!(r.equilibrium.ball_results.iter().all(|b| !b.loaded));
    assert!(!r.alerts.iter().any(|a| a.code == "NOT_CONVERGED"));
}
