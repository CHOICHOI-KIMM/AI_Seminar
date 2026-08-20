// Level D-1 검증 — Harris Table 7.4 대조 (Plan Phase 3-1)
//
// ★ **이 프로젝트에 남은 유일한 외부 문헌 검증이다.**
//   ISO 16281:2025 에는 수치 예제가 없다(전문 검색 0건). 다른 하나는 Level B(Table 6.1).
//
// 실행: cargo test --test equilibrium_level_d1 -- --nocapture
// 골든값: Harris & Kotzalas 5th ed. *Essential Concepts* Table 7.4 (원서 p.170, 육안 확인).
//
// ── 프로토콜 (P3-1 결정) ────────────────────────────────────────────
// ① **표는 `F_r tanα/F_a` 로 들어간다** (Harris Fig. 7.14 가 그 축으로 그려져 있다).
//    ε 로 들어가면 ε > 2,5 구간이 성기어 선형보간 오차가 커진다 — 실제로 초안에서
//    같은 데이터가 −7,8 % 로 나왔고, 진입 변수를 바꾸자 +2,1 % 가 되었다.
// ② **Z = 200 으로 적분 극한에 접근해 판정**하고, 실제 `Z = 16` 의
//    **이산화 오차는 별도로 기록**한다. 모델 오차와 이산화 오차를 섞지 않기 위해서다.
//    Z = 200 은 물리적 베어링이 아니라 Harris 의 적분(Z → ∞)에 접근하기 위한 수치 장치다
//    (볼 간섭은 모델이 검사하지 않는다).
// ③ Harris 는 **모든 하중 볼의 접촉각이 같다**고 가정한다(원서 p.170 명시).
//    우리 해는 볼마다 α_j 가 다르므로, **최대하중 볼의 α** 를 대표값으로 쓴다.
//
// ── ⚠ 구속 조건: ISO_3DOF 가 아니다 ────────────────────────────────
// Harris §7.5(조합하중)는 **미스얼라인먼트를 포함하지 않는 2-DOF**(δ_a, δ_r) 정식화다.
// `DofMask::ISO_3DOF` 는 `γ_z` 가 자유이므로 조건이 다르다.
// 따라서 여기서는 `δ_x`·`δ_y` 만 자유로 두고 나머지 셋을 0 으로 구속한다.
// (모멘트 반력은 하우징이 받는다고 보는 것이며, Harris 도 암묵적으로 그렇게 둔다.)

use app_lib::solver::bearing::solve_bearing;
use app_lib::solver::types::*;

/// Harris Table 7.4 — 점접촉 단열. `(ε, F_r tanα/F_a, J_r, J_a)`
/// `ε = 0` 행의 `J = 1/Z` 는 이산(볼 1개) 값이라 수치 대조에서 제외한다.
const TABLE_7_4: [(f64, f64, f64, f64); 13] = [
    (0.2, 0.9318, 0.1590, 0.1707),
    (0.3, 0.8964, 0.1892, 0.2110),
    (0.4, 0.8601, 0.2117, 0.2462),
    (0.5, 0.8225, 0.2288, 0.2782),
    (0.6, 0.7835, 0.2416, 0.3084),
    (0.7, 0.7427, 0.2505, 0.3374),
    (0.8, 0.6995, 0.2559, 0.3658),
    (0.9, 0.6529, 0.2576, 0.3945),
    (1.0, 0.6000, 0.2546, 0.4244),
    (1.25, 0.4338, 0.2289, 0.5044),
    (1.67, 0.3088, 0.1871, 0.6060),
    (2.5, 0.1850, 0.1339, 0.7240),
    (5.0, 0.0831, 0.0711, 0.8558),
];

/// `F_r tanα/F_a` 로 표를 조회한다 (감소 순 열).
/// `col`: 0 = ε, 2 = J_r, 3 = J_a
fn lookup(ratio: f64, col: usize) -> f64 {
    let get = |r: &(f64, f64, f64, f64)| match col {
        0 => r.0,
        2 => r.2,
        _ => r.3,
    };
    if ratio >= TABLE_7_4[0].1 {
        return get(&TABLE_7_4[0]);
    }
    for w in TABLE_7_4.windows(2) {
        if ratio >= w[1].1 {
            let t = (ratio - w[0].1) / (w[1].1 - w[0].1);
            return get(&w[0]) + t * (get(&w[1]) - get(&w[0]));
        }
    }
    get(TABLE_7_4.last().unwrap())
}

/// Harris §7.5 조건에 맞춘 구속: 미스얼라인먼트 없음 (2-DOF).
const HARRIS_2DOF: DofMask = DofMask {
    x: Dof::Free,
    y: Dof::Free,
    z: Dof::Prescribed(0.0),
    gy: Dof::Prescribed(0.0),
    gz: Dof::Prescribed(0.0),
};

const ALPHA_DEG: f64 = 40.0;

struct Point {
    ratio: f64,
    eps: f64,
    j_r: f64,
    j_a: f64,
    alpha_deg: f64,
    loaded_fraction: f64,
}

fn solve_point(z: u32, f_a: f64, f_r: f64) -> Point {
    let d_w = 11.5;
    let (r_i, r_e) = BallBearingGeometry::reference_groove_radii(d_w);
    let inp = BearingInput {
        geometry: BallBearingGeometry {
            bore_mm: 50.0,
            outer_diameter_mm: 90.0,
            width_mm: 20.0,
            z,
            d_w_mm: d_w,
            d_pw_mm: 70.0,
            r_i_mm: r_i,
            r_e_mm: r_e,
            alpha_nom_rad: ALPHA_DEG.to_radians(),
            clearance: ClearanceSpec::InitialAngleRad(ALPHA_DEG.to_radians()),
        },
        material: Material::default(),
        operating: OperatingConditions {
            f_x_n: f_a,
            f_y_n: f_r,
            f_z_n: 0.0,
            m_y_nmm: 0.0,
            m_z_nmm: 0.0,
            n_inner_rpm: 0.0,
            n_outer_rpm: 0.0,
            temperature_c: 20.0,
        },
        solver: SolverParams {
            convergence_tol: 1e-12,
            max_iterations: 300,
            dof_mask: HARRIS_2DOF,
            ..SolverParams::default()
        },
    };
    let r = solve_bearing(&inp).expect("해석 실패");
    let eq = &r.equilibrium;
    assert!(eq.converged, "미수렴 (Z={z}, F_a={f_a}, F_r={f_r})");

    let loaded: Vec<&BallResult> = eq.ball_results.iter().filter(|b| b.loaded).collect();
    assert!(!loaded.is_empty());
    // Harris 의 '모든 볼 동일 α' 가정에 대응하는 대표값 — 최대하중 볼의 접촉각
    let alpha = loaded
        .iter()
        .max_by(|a, b| a.q_n.partial_cmp(&b.q_n).unwrap())
        .unwrap()
        .alpha_rad;

    let zf = z as f64;
    Point {
        // Harris (7.66)(7.68) 을 J 에 대해 푼 형태.
        // (7.70) 의 `sin α` 는 원서 오식이며 `cos α` 가 맞다 (Theory §9.1 주의 1, T-5 해소)
        ratio: f_r * alpha.tan() / f_a,
        eps: 0.5 * (1.0 + eq.displacement[0] * alpha.tan() / eq.displacement[1]),
        j_r: f_r / (zf * eq.q_max_n * alpha.cos()),
        j_a: f_a / (zf * eq.q_max_n * alpha.sin()),
        alpha_deg: alpha.to_degrees(),
        loaded_fraction: loaded.len() as f64 / zf,
    }
}

/// 표가 덮는 범위를 훑는 하중 조합. `F_r tanα₀/F_a` 가 목표비가 되도록 잡는다.
const RATIO_SWEEP: [f64; 9] = [0.10, 0.15, 0.20, 0.30, 0.40, 0.50, 0.60, 0.70, 0.85];

fn sweep(z: u32) -> Vec<(Point, f64, f64)> {
    let f_a = 4_000.0;
    let tan0 = ALPHA_DEG.to_radians().tan();
    RATIO_SWEEP
        .iter()
        .map(|&target| {
            let p = solve_point(z, f_a, target * f_a / tan0);
            let jr_ref = lookup(p.ratio, 2);
            let ja_ref = lookup(p.ratio, 3);
            (p, jr_ref, ja_ref)
        })
        .collect()
}

// ═══════════════════════════════════════════════════════════════════
//  D-1. Harris Table 7.4 대조 (Z = 200, 적분 극한)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn d1_harris_table_7_4_integral_limit() {
    println!("\n── Level D-1 : Harris Table 7.4 대조 (Z = 200, 적분 극한) ──");
    println!("  F_r·tanα/F_a    ε(우리/표)      J_r(우리/표)  오차%     J_a(우리/표)  오차%   α[°]  하중볼");

    let mut worst_jr = 0.0_f64;
    let mut worst_ja = 0.0_f64;
    for (p, jr_ref, ja_ref) in sweep(200) {
        let e_r = (p.j_r / jr_ref - 1.0) * 100.0;
        let e_a = (p.j_a / ja_ref - 1.0) * 100.0;
        worst_jr = worst_jr.max(e_r.abs());
        worst_ja = worst_ja.max(e_a.abs());
        println!(
            "     {:6.3}      {:6.3}/{:6.3}   {:7.4}/{:7.4}  {:+6.2}   {:7.4}/{:7.4}  {:+6.2}  {:5.1}  {:.0}%",
            p.ratio,
            p.eps,
            lookup(p.ratio, 0),
            p.j_r,
            jr_ref,
            e_r,
            p.j_a,
            ja_ref,
            e_a,
            p.alpha_deg,
            p.loaded_fraction * 100.0
        );
        assert!(
            e_r.abs() <= 5.0,
            "J_r 오차 {e_r:.2} % 가 판정 기준 5 % 초과 (ratio {:.3})",
            p.ratio
        );
        assert!(
            e_a.abs() <= 5.0,
            "J_a 오차 {e_a:.2} % 가 판정 기준 5 % 초과 (ratio {:.3})",
            p.ratio
        );
    }
    println!("  → 최대 오차 : J_r {worst_jr:.2} % · J_a {worst_ja:.2} %   (판정 ≤ 5 %)");
}

// ═══════════════════════════════════════════════════════════════════
//  D-1b. 이산화 오차 (실제 Z = 16) — 판정이 아니라 기록
// ═══════════════════════════════════════════════════════════════════

#[test]
fn d1b_discretization_error_at_real_z() {
    println!("\n── Level D-1b : 실제 Z = 16 의 이산화 오차 (기록용) ──");
    println!("  F_r·tanα/F_a    J_r 오차%   J_a 오차%   α[°]  하중볼");

    let mut worst = 0.0_f64;
    for (p, jr_ref, ja_ref) in sweep(16) {
        let e_r = (p.j_r / jr_ref - 1.0) * 100.0;
        let e_a = (p.j_a / ja_ref - 1.0) * 100.0;
        worst = worst.max(e_r.abs()).max(e_a.abs());
        println!(
            "     {:6.3}       {:+6.2}     {:+6.2}    {:5.1}  {:.0}%",
            p.ratio,
            e_r,
            e_a,
            p.alpha_deg,
            p.loaded_fraction * 100.0
        );
    }
    println!("  → Z = 16 최대 편차 {worst:.2} % (Harris 는 적분값이므로 이 차이는 이산화 효과다)");
    // 이산화 오차가 판정 기준 자체를 삼키지 않는지만 확인한다
    assert!(worst < 5.0, "Z = 16 이산화 오차 {worst:.2} % 가 지나치게 큼");
}

#[test]
fn d1c_discretization_error_shrinks_with_z() {
    // Z 를 늘리면 적분값에 수렴해야 한다 — 이산화 효과임을 확인하는 근거
    let mut prev = f64::INFINITY;
    let mut worsts = Vec::new();
    for z in [16u32, 32, 64, 200] {
        let w = sweep(z)
            .iter()
            .map(|(p, jr, ja)| ((p.j_r / jr - 1.0).abs()).max((p.j_a / ja - 1.0).abs()))
            .fold(0.0_f64, f64::max)
            * 100.0;
        worsts.push((z, w));
        prev = prev.min(w);
    }
    println!("\n── Level D-1c : Z 증가에 따른 최대 편차 ──");
    for (z, w) in &worsts {
        println!("  Z = {z:3}  →  {w:.2} %");
    }
    let (_, w16) = worsts[0];
    let (_, w200) = worsts[worsts.len() - 1];
    assert!(w200 <= w16, "Z 를 늘렸는데 편차가 줄지 않음 ({w16:.2} → {w200:.2})");
}

// ═══════════════════════════════════════════════════════════════════
//  D-1d. ε 정의식 독립 확인
// ═══════════════════════════════════════════════════════════════════

#[test]
fn d1d_epsilon_formula_matches_table() {
    // ε = ½(1 + δ_a tanα/δ_r) 는 Harris (7.65) 의 하중구간 한계각에서 나온 관계다.
    // 우리 해의 변위로 계산한 ε 가 표의 ε 열과 맞는지 본다 — J 대조와 독립인 경로다.
    //
    // ⚠ 표의 ε 열은 2,5 와 5,0 사이가 한 칸이다. 그 구간을 선형보간하면 ε 의 비선형성
    //    때문에 편차가 10 % 대까지 벌어진다 — **표의 보간 오차이지 ε 공식의 오류가 아니다.**
    //    따라서 표가 조밀한 ε ≤ 2,5 (ratio ≥ 0,185) 구간에서만 판정하고,
    //    성긴 구간은 참고로 출력만 한다.
    const DENSE_RATIO_MIN: f64 = 0.185; // 표의 ε = 2,5 행
    println!("
── Level D-1d : ε 정의식 대조 ──");
    let mut worst_dense = 0.0_f64;
    for (p, _, _) in sweep(200) {
        let eps_ref = lookup(p.ratio, 0);
        let err = (p.eps / eps_ref - 1.0) * 100.0;
        let dense = p.ratio >= DENSE_RATIO_MIN;
        if dense {
            worst_dense = worst_dense.max(err.abs());
        }
        println!(
            "  ratio {:6.3}  ε 우리 {:6.3} / 표 {:6.3}  → {:+6.2} %  {}",
            p.ratio,
            p.eps,
            eps_ref,
            err,
            if dense { "" } else { "(표 성긴 구간 — 판정 제외)" }
        );
    }
    println!("  → 조밀 구간(ε ≤ 2,5) 최대 편차 {worst_dense:.2} %");
    assert!(worst_dense < 3.0, "ε 정의식 편차 {worst_dense:.2} % 가 과대");
}

// ═══════════════════════════════════════════════════════════════════
//  D-1e. 하중구간 물리 정합
// ═══════════════════════════════════════════════════════════════════

#[test]
fn d1e_loaded_zone_shrinks_as_radial_share_grows() {
    // 반경 비중이 커질수록 하중구간이 좁아진다 (ε 감소)
    let mut prev_eps = f64::INFINITY;
    let mut prev_frac = 1.01_f64;
    for (p, _, _) in sweep(200) {
        assert!(p.eps < prev_eps, "ratio {:.3}: ε 단조성 위반", p.ratio);
        assert!(
            p.loaded_fraction <= prev_frac + 1e-9,
            "ratio {:.3}: 하중구간 단조성 위반",
            p.ratio
        );
        prev_eps = p.eps;
        prev_frac = p.loaded_fraction;
    }
}
