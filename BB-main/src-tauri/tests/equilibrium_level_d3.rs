// Level D-3 검증 — Harris & Mindel (1973) 식 (81)~(90) 독립 전사 대조
//
// 출처: T.A. Harris, M.H. Mindel, "Rolling element bearing dynamics",
//       Wear 23(3), 311–337 (1973), §3.2–3.3 (p.325–326).
// 실행: cargo test --test equilibrium_level_d3
//
// ── 이것이 왜 Level C-1 보다 강한가 ──────────────────────────────────
// Level C-1 은 **우리 Theory §4.4 의 확정형**으로 잔차를 재조립했다. 같은 문서에서
// 나온 두 구현을 비교한 셈이라 전사 오류는 잡아도 **정식화 자체의 오류는 못 잡는다.**
//
// 여기서는 1973 년 원전의 식을 **원 표기 그대로** 테스트 안에 다시 구현한다.
// 원전은 방위 규약이 우리와 90° 돌아가 있고(F_2 ↔ sin φ, F_3 ↔ cos φ),
// 간섭량을 (A_1j, A_2j) 두 성분으로 쌓는 Jones 식 부기를 쓴다. 즉 **다른 좌표·다른 부기**로
// 같은 물리를 기술한 독립 경로다. 두 경로가 같은 답을 내야 한다.
//
// ── 좌표 사상 (회전 φ' = φ − π/2) ───────────────────────────────────
// 원전의 Y 축은 φ' = 90°, 우리 Y 축은 φ = 0° 이다. 반경평면을 −90° 회전하면
//
//     φ'_j = φ_j − π/2
//     Δ₁ = δ_x   Δ₂ = −δ_y   Δ₃ = δ_z   Δ₄ = γ_y   Δ₅ = γ_z
//     F₁ = F_x   F₂ = −F_y   F₃ = F_z   F₄ = M_y   F₅ = M_z
//
// 부호가 뒤집히는 짝(Δ₂, F₂)은 **함께** 뒤집히므로 가상일 F·Δ 는 보존된다.
// 이 사상 자체가 검증 대상이며, 임의로 맞춘 것이 아니라 회전변환에서 유도된다.

use bb_core::solver::bb::bearing::solve_bearing;
use bb_core::solver::bb::geometry::compute_geometry_derived;
use bb_core::solver::bb::hertz;
use bb_core::solver::bb::types::*;
use bb_core::solver::common::types::*;

const D_W: f64 = 11.5;
const D_PW: f64 = 70.0;
const ALPHA_DEG: f64 = 40.0;
const Z: u32 = 16;

fn geometry() -> BallBearingGeometry {
    let (r_i_mm, r_e_mm) = BallBearingGeometry::reference_groove_radii(D_W);
    BallBearingGeometry {
        bore_mm: 50.0,
        outer_diameter_mm: 90.0,
        width_mm: 20.0,
        z: Z,
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
        kind: BallBearingKind::Acbb,
        geometry: geometry(),
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
        solver: BbSolverParams {
            convergence_tol: 1e-13,
            ..BbSolverParams::default()
        },
    }
}

const CASES: [(f64, f64, f64, f64, f64); 6] = [
    (5_000.0, 0.0, 0.0, 0.0, 0.0),
    (5_000.0, 3_000.0, 0.0, 0.0, 0.0),
    (4_000.0, 2_500.0, 1_200.0, 6_000.0, 9_000.0),
    (2_000.0, 0.0, 6_000.0, -3.0e5, 0.0),
    (8_000.0, -4_000.0, 2_000.0, 1.5e5, -2.2e5),
    (500.0, 7_000.0, -3_500.0, 4.0e5, 3.0e5),
];

// ═══════════════════════════════════════════════════════════════════
//  원전 식의 독립 구현 — 우리 solver 모듈을 쓰지 않는다
// ═══════════════════════════════════════════════════════════════════

/// 원전 (83)(85) 의 상수. 입력 기하 원시값에서 **직접** 만든다.
struct Original {
    /// `B·D` — (85) `B = f₁ + f₂ − 1`
    bd: f64,
    /// `r̄` — (83) `r̄ = ½e + (f₂ − ½) D cos α°`
    r_bar: f64,
    /// `α°` — 자유 접촉각 [rad]
    alpha0: f64,
}

fn original_constants(g: &BallBearingGeometry, alpha0: f64) -> Original {
    let d = g.d_w_mm;
    let f1 = g.r_e_mm / d; // 외륜 곡률비
    let f2 = g.r_i_mm / d; // 내륜 곡률비 (원전은 첨자 2 = 내륜)
    let b = f1 + f2 - 1.0;
    Original {
        bd: b * d,
        r_bar: 0.5 * g.d_pw_mm + (f2 - 0.5) * d * alpha0.cos(),
        alpha0,
    }
}

/// 원전 (81)(82) → 볼 j 의 간섭·접촉각.
///
/// `delta` 는 (A_1j, A_2j) 벡터 길이에서 `B·D` 를 뺀 값이다.
/// 원전 (1)(84) 의 정의와 같으며, 접촉이 끊기면 0 이다.
fn ball_original(o: &Original, phi_prime: f64, dl: [f64; 5]) -> (f64, f64) {
    let (sp, cp) = phi_prime.sin_cos();
    // (81)  A_1j = B D sin α° + Δ₁ + r̄ (Δ₄ cos φ_j + Δ₅ sin φ_j)
    let a1 = o.bd * o.alpha0.sin() + dl[0] + o.r_bar * (dl[3] * cp + dl[4] * sp);
    // (82)  A_2j = B D cos α° + Δ₂ sin φ_j + Δ₃ cos φ_j
    let a2 = o.bd * o.alpha0.cos() + dl[1] * sp + dl[2] * cp;
    let l = a1.hypot(a2);
    ((l - o.bd).max(0.0), a1.atan2(a2))
}

/// 원전 (86)~(90) 을 `F̄_w = 0` (정적) 으로 환원해 합산한다.
fn original_equilibrium(o: &Original, z: u32, c_p: f64, dl: [f64; 5]) -> [f64; 5] {
    let mut f = [0.0_f64; 5];
    for j in 0..z {
        // 원전 φ' 와 우리 φ 는 90° 차이 — 볼 배치 자체는 같다
        let phi = std::f64::consts::TAU * (j as f64) / (z as f64);
        let phi_prime = phi - std::f64::consts::FRAC_PI_2;
        let (delta, alpha) = ball_original(o, phi_prime, dl);
        if delta <= 0.0 {
            continue;
        }
        let q = c_p * delta.powf(1.5);
        let (sa, ca) = alpha.sin_cos();
        let (sp, cp) = phi_prime.sin_cos();
        f[0] += q * sa; // (86) F₁
        f[1] += q * ca * sp; // (87) F₂
        f[2] += q * ca * cp; // (88) F₃
        f[3] += o.r_bar * q * sa * cp; // (89) F₄
        f[4] += o.r_bar * q * sa * sp; // (90) F₅
    }
    f
}

// ═══════════════════════════════════════════════════════════════════
//  D-3a. 원전 상수 항등 — (83)(85) 와 우리 기하량
// ═══════════════════════════════════════════════════════════════════

/// `B·D = A` (곡률중심 거리) 와 `r̄ = R_i` (모멘트 팔) 가 같은 값인지 본다.
/// D-9b(팔은 `R_i`) 결정을 **원전이 직접 뒷받침**하는지 확인하는 자리다.
#[test]
fn d3a_original_constants_match_geometry() {
    println!("\n── Level D-3a : 원전 (83)(85) ↔ 우리 기하량 ──");
    let g = geometry();
    let d = compute_geometry_derived(&g).unwrap();
    let o = original_constants(&g, d.alpha_0_rad);

    let e_bd = (o.bd / d.a_mm - 1.0).abs();
    let e_rb = (o.r_bar / d.r_i_center_mm - 1.0).abs();
    println!("  (85) B·D = {:.10} / A   = {:.10}   rel {e_bd:.2e}", o.bd, d.a_mm);
    println!("  (83) r̄   = {:.10} / R_i = {:.10}   rel {e_rb:.2e}", o.r_bar, d.r_i_center_mm);
    assert!(e_bd < 1e-14, "B·D ≠ A ({e_bd:.3e})");
    assert!(e_rb < 1e-14, "r̄ ≠ R_i ({e_rb:.3e}) — D-9b 결정의 근거가 무너진다");
    println!("  → 원전 (83) 이 D-9b(모멘트 팔 = R_i)를 직접 뒷받침한다");
}

/// 원전 (84) `α° = arccos(1 − p_d/(2BD))` 는 **클리어런스로 각이 정해지는 경우**의 식이다.
/// ISO 16281 (A.1) 과 같은 형태이므로, 직경 클리어런스를 준 입력에서 일치해야 한다.
#[test]
fn d3b_original_free_contact_angle_matches_iso_a1() {
    println!("\n── Level D-3b : 원전 (84) ↔ ISO 16281 (A.1) ──");
    let mut worst = 0.0_f64;
    for p_d in [0.005, 0.02, 0.05, 0.12, 0.30] {
        let mut g = geometry();
        g.clearance = BbClearanceSpec::DiametralMm(p_d);
        let d = compute_geometry_derived(&g).unwrap();
        let o = original_constants(&g, d.alpha_0_rad);
        let alpha_orig = (1.0 - p_d / (2.0 * o.bd)).acos();
        let e = (alpha_orig - d.alpha_0_rad).abs();
        worst = worst.max(e);
        println!(
            "  p_d = {p_d:5.3} mm  →  원전 {:.9} rad / ISO {:.9} rad   Δ {e:.2e}",
            alpha_orig, d.alpha_0_rad
        );
    }
    println!("  → 최대 편차 {worst:.2e} rad");
    assert!(worst < 1e-14, "원전 (84) 와 ISO (A.1) 불일치 {worst:.3e}");
}

// ═══════════════════════════════════════════════════════════════════
//  D-3c. 볼별 간섭·접촉각 대조
// ═══════════════════════════════════════════════════════════════════

/// 원전 (81)(82) 로 다시 만든 `(δ_j, α_j)` 가 우리 솔버 결과와 같은가.
/// 평형식에 들어가기 **전 단계**를 먼저 격리해 본다.
#[test]
fn d3c_ball_kinematics_match_original() {
    println!("\n── Level D-3c : 원전 (81)(82) ↔ 볼별 (δ_j, α_j) ──");
    println!("   케이스                     max Δδ [mm]   max Δα [rad]");
    let mut worst_d = 0.0_f64;
    let mut worst_a = 0.0_f64;

    for (i, &(fx, fy, fz, my, mz)) in CASES.iter().enumerate() {
        let inp = make(fx, fy, fz, my, mz);
        let r = solve_bearing(&inp).unwrap();
        assert!(r.equilibrium.converged, "케이스 {i} 미수렴");
        let d = compute_geometry_derived(&inp.geometry).unwrap();
        let o = original_constants(&inp.geometry, d.alpha_0_rad);

        let u = r.equilibrium.displacement;
        // Δ₂ = −δ_y (원전 부호 사상)
        let dl = [u.dx_mm, -u.dy_mm, u.dz_mm, u.ry_rad, u.rz_rad];

        let (mut ed, mut ea) = (0.0_f64, 0.0_f64);
        for (j, b) in r.equilibrium.ball_results.iter().enumerate() {
            let phi = std::f64::consts::TAU * (j as f64) / (Z as f64);
            let (delta, alpha) = ball_original(&o, phi - std::f64::consts::FRAC_PI_2, dl);
            ed = ed.max((delta - b.delta_mm).abs());
            if b.loaded {
                ea = ea.max((alpha - b.alpha_rad).abs());
            }
        }
        worst_d = worst_d.max(ed);
        worst_a = worst_a.max(ea);
        println!("   [{i}] {fx:6.0} {fy:6.0} {fz:6.0} {my:8.1e} {mz:8.1e}   {ed:.3e}   {ea:.3e}");
    }
    println!("  → 최대 Δδ {worst_d:.2e} mm · Δα {worst_a:.2e} rad");
    assert!(worst_d < 1e-12, "간섭량 불일치 {worst_d:.3e} mm");
    assert!(worst_a < 1e-12, "접촉각 불일치 {worst_a:.3e} rad");
}

// ═══════════════════════════════════════════════════════════════════
//  D-3d. 5-DOF 평형식 대조  ★ 본 검증
// ═══════════════════════════════════════════════════════════════════

/// 원전 (86)~(90) 을 `F̄_w = 0` 으로 환원해 합산한 5개 하중이 인가 외력과 맞는가.
///
/// **여기서 부호 사상이 시험된다.** F₄ = M_y, F₅ = M_z 가 성립하려면 우리 코드가
/// `M_y = +R_i Σ Q sinα sinφ`, `M_z = −R_i Σ Q sinα cosφ` 여야 한다.
/// 부호가 반대라면 이 시험은 **전 케이스에서 동시에** 실패한다.
#[test]
fn d3d_five_dof_equilibrium_matches_original() {
    println!("\n── Level D-3d : 원전 (86)~(90) ↔ 인가 외력 ──");
    println!("   케이스                     max rel.err");
    let mut worst = 0.0_f64;

    for (i, &(fx, fy, fz, my, mz)) in CASES.iter().enumerate() {
        let inp = make(fx, fy, fz, my, mz);
        let r = solve_bearing(&inp).unwrap();
        let d = compute_geometry_derived(&inp.geometry).unwrap();
        let c = hertz::compute_contact_derived(&d, &inp.material).unwrap();
        let o = original_constants(&inp.geometry, d.alpha_0_rad);

        let u = r.equilibrium.displacement;
        let dl = [u.dx_mm, -u.dy_mm, u.dz_mm, u.ry_rad, u.rz_rad];
        let f = original_equilibrium(&o, Z, c.c_p_n_per_mm15, dl);

        // 사상: F₁=F_x, F₂=−F_y, F₃=F_z, F₄=M_y, F₅=M_z
        let want = [fx, -fy, fz, my, mz];
        // 힘과 모멘트를 같은 차원에서 비교하기 위해 모멘트는 r̄ 로 나눈다
        let scale_f = (fx.abs() + fy.abs() + fz.abs()).max(1.0);
        let scale_m = (my.abs() + mz.abs()).max(scale_f * o.r_bar);

        let mut e = 0.0_f64;
        for k in 0..5 {
            let s = if k < 3 { scale_f } else { scale_m };
            e = e.max((f[k] - want[k]).abs() / s);
        }
        worst = worst.max(e);
        println!("   [{i}] {fx:6.0} {fy:6.0} {fz:6.0} {my:8.1e} {mz:8.1e}   {e:.3e}");
    }
    println!("  → 최대 상대오차 {worst:.2e}   (판정 < 1e-8)");
    assert!(
        worst < 1e-8,
        "원전 (86)~(90) 과 불일치 {worst:.3e} — 좌표 사상 또는 정식화가 다르다"
    );
}

// ═══════════════════════════════════════════════════════════════════
//  D-3e. 모멘트 부호 규약 — 문서와 코드 중 어느 쪽이 맞는가
// ═══════════════════════════════════════════════════════════════════

/// Theory §4.4 의 인쇄된 형태
/// ```text
///   M_y = −R_i Σ Q sin α sin φ      M_z = +R_i Σ Q sin α cos φ
/// ```
/// 는 **같은 블록의 운동학과 모순**이다. `X_j = … − R_i(γ_z cos φ − γ_y sin φ)` 이므로
/// 가상일 공액은 `∂δ_j/∂γ_y = sin α_j · (+R_i sin φ_j)` → `M_y = +R_i Σ Q sin α sin φ` 이다.
///
/// 이 시험은 두 부호 중 어느 쪽이 인가 모멘트를 재현하는지 **직접** 가른다.
/// 원전 (89)(90) 과의 대조(D-3d)와 독립인 확인이다.
#[test]
fn d3e_moment_sign_convention() {
    println!("\n── Level D-3e : 모멘트 부호 규약 판별 ──");
    let inp = make(4_000.0, 2_500.0, 1_200.0, 6_000.0, 9_000.0);
    let r = solve_bearing(&inp).unwrap();
    let r_i = r.geometry.r_i_center_mm;

    let (mut plus_y, mut plus_z) = (0.0, 0.0);
    for b in &r.equilibrium.ball_results {
        let (sa, _) = b.alpha_rad.sin_cos();
        let (sp, cp) = b.phi_rad.sin_cos();
        plus_y += r_i * b.q_n * sa * sp;
        plus_z += r_i * b.q_n * sa * cp;
    }
    let (my, mz) = (inp.operating.m_y_nmm, inp.operating.m_z_nmm);
    println!("   +R_i ΣQ sinα sinφ = {plus_y:12.4}   인가 M_y = {my:12.4}");
    println!("   +R_i ΣQ sinα cosφ = {plus_z:12.4}   인가 M_z = {mz:12.4}");

    let e_code = ((plus_y - my).abs() + (-plus_z - mz).abs()) / (my.abs() + mz.abs());
    let e_doc = ((-plus_y - my).abs() + (plus_z - mz).abs()) / (my.abs() + mz.abs());
    println!("   코드 규약 (M_y=+, M_z=−) 오차 {e_code:.3e}");
    println!("   문서 §4.4 인쇄형 (M_y=−, M_z=+) 오차 {e_doc:.3e}");
    assert!(
        e_code < 1e-8,
        "코드 규약이 인가 모멘트를 재현하지 못함 {e_code:.3e}"
    );
    assert!(
        e_doc > 1.0,
        "두 규약이 구분되지 않음 — 시험이 부호를 가르지 못한다"
    );
    println!("  → 코드가 맞고 **Theory §4.4 의 인쇄된 M_y·M_z 부호가 오기**다");
}
