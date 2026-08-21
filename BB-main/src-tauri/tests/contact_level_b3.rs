// Level B-3 검증 — Harris & Mindel (1973) Fig. 15 실기 ACBB 점접촉 대조
//
// 출처: T.A. Harris, M.H. Mindel, "Rolling element bearing dynamics",
//       Wear 23(3), 311–337 (1973), Fig. 15 (SKF AE79Y003 프로그램 샘플 출력, p.330–331).
// 실행: cargo test --test contact_level_b3
//
// ── 왜 이 대조가 필요한가 ────────────────────────────────────────────
// Level B(Harris Table 6.1)는 **무차원 표**다. F(ρ) 를 직접 넣고 a*·b*·δ* 를 받는다.
// 여기서는 **실제 각접촉 볼베어링의 치수**에서 시작해
//   D_w, D_pw, f, α  →  γ  →  Σρ, F(ρ)  →  χ  →  a*, b*, δ*  →  a, b, δ, p_max
// 사슬 전체를 한 번에 대조한다. 즉 **기하 → 점접촉 사슬의 외부 검증**이며,
// Table 6.1 대조가 덮지 못하는 Σρ·F(ρ) 산출부를 포함한다.
//
// ── 이 예제의 한계 (반드시 인지) ─────────────────────────────────────
// 원 예제는 **24 000 rpm 고속**이라 볼당 원심력이 303,45 lb 로 접촉하중과 같은 규모다.
// 그 때문에 내륜 접촉각(29,762°)과 외륜 접촉각(14,475°)이 갈라진다. 우리 정적 모델은
// 이 상태를 **평형으로 재현할 수 없다.** 따라서 여기서는 평형을 풀지 않고,
// **인쇄된 (Q, α) 를 입력으로 주고 접촉량만** 대조한다. 5-DOF 검증은 Level D-3 이 맡는다.
//
// ── OCR 주의 (Theory §0 의 3경로 규칙) ───────────────────────────────
// MinerU 가 만든 통합 MD 의 이미지 판독에서 **내륜 최대접촉응력을 1,9922e5 psi 로
// 잘못 읽었다.** pypdfium2 로 p.331 을 scale 16 으로 재렌더한 결과 실제 값은
// **1,9522e5 psi** 이며, 이는 인쇄된 a·b 로 계산한 3Q/(2πab) = 1,9521e5 와 일치한다.
// 아래 상수는 **재렌더 판독값**이다.

use app_lib::solver::geometry::compute_geometry_derived;
use app_lib::solver::hertz;
use app_lib::solver::types::*;

// ── 단위 환산 (원 자료가 in·lb·psi 계) ──────────────────────────────
// 솔버 내부는 mm·N·MPa 이므로 **테스트 경계에서** 환산한다 (D-10 은 솔버 내부 규칙).
const MM_PER_IN: f64 = 25.4; // 정의값
const N_PER_LBF: f64 = 4.448_221_615_260_5; // NIST SP 811
const MPA_PER_PSI: f64 = N_PER_LBF / (MM_PER_IN * MM_PER_IN);

// ── Fig. 15 입력 (BEARING DIMENSIONS / OPERATIONAL CONDITIONS) ──────
const Z: u32 = 21;
const D_W_IN: f64 = 0.8125;
const D_PW_IN: f64 = 6.2008;
const F_OUTER: f64 = 0.5200; // CURVATURE OUTER RACEWAY
const F_INNER: f64 = 0.5200; // CURVATURE INNER RACEWAY
const E_PSI: f64 = 2.96e7; // MODULUS OF ELASTICITY (INNER/OUTER RING)
const NU: f64 = 0.25; // POISSON'S RATIO

// ── Fig. 15 출력 (OUTER / INNER RACEWAY DATA, AZIMUTH 0°) ───────────
struct Printed {
    label: &'static str,
    alpha_deg: f64,
    q_lb: f64,
    a_in: f64,       // CONTACT SEMI-MAJOR AXIS
    b_in: f64,       // CONTACT SEMI-MINOR AXIS
    delta_in: f64,   // CONTACT DEFORMATION
    p_max_psi: f64,  // MAXIMUM CONTACT STRESS
    /// 변형 자릿수를 스캔에서 확정할 수 있는가. `false` 면 **판정에서 제외**한다.
    delta_readable: bool,
}

const OUTER: Printed = Printed {
    label: "외륜",
    alpha_deg: 14.475,
    q_lb: 5.7107e2,
    a_in: 9.6824e-2,
    b_in: 1.2801e-2,
    // ⚠ 소수 둘째~셋째 자리를 확정할 수 없다. scale 34 + 오토콘트라스트까지 올려도
    //   원 스캔 DPI 가 한계라 픽셀만 커질 뿐 글리프가 분해되지 않는다.
    //   "6,8276" / "6,2276" / "6,1276" 이 모두 가능하다 → **판정에서 제외**.
    delta_in: 6.8276e-4,
    p_max_psi: 2.1999e5,
    delta_readable: false,
};

const INNER: Printed = Printed {
    label: "내륜",
    alpha_deg: 29.762,
    q_lb: 2.8776e2,
    a_in: 7.8504e-2,
    b_in: 8.9653e-3,
    delta_in: 4.0335e-4,
    p_max_psi: 1.9522e5, // ← 재렌더 판독 (MD 의 1,9922e5 는 OCR 오류)
    delta_readable: true,
};

/// 원 자료의 **운전 접촉각**에서 Σρ·F(ρ) 를 얻기 위한 기하.
///
/// `compute_geometry_derived` 는 `α₀` 에서 곡률을 계산하므로,
/// 공칭각·클리어런스를 **대조하려는 접촉각**으로 맞춰 넣는다.
/// (원 예제는 마운트 클리어런스 0 이고 원심력 때문에 각이 갈라진 상태이므로,
///  내륜·외륜을 각각 한 번씩 푼다.)
fn geometry_at(alpha_deg: f64) -> BallBearingGeometry {
    let alpha = alpha_deg.to_radians();
    BallBearingGeometry {
        // BEARING OD / BORE / WIDTH (Fig. 15 입력 표) — 곡률 계산에는 쓰이지 않으나
        // `validate()` 를 통과시키기 위해 원 자료값을 그대로 환산해 넣는다.
        bore_mm: 4.93 * MM_PER_IN,
        outer_diameter_mm: 7.48 * MM_PER_IN,
        width_mm: 1.28 * MM_PER_IN,
        z: Z,
        d_w_mm: D_W_IN * MM_PER_IN,
        d_pw_mm: D_PW_IN * MM_PER_IN,
        r_i_mm: F_INNER * D_W_IN * MM_PER_IN,
        r_e_mm: F_OUTER * D_W_IN * MM_PER_IN,
        alpha_nom_rad: alpha,
        clearance: ClearanceSpec::InitialAngleRad(alpha),
    }
}

fn material() -> Material {
    Material {
        e_ball_mpa: E_PSI * MPA_PER_PSI,
        e_ring_mpa: E_PSI * MPA_PER_PSI,
        nu: NU,
        ..Material::default()
    }
}

/// 한 레이스웨이의 (a, b, δ, p_max) 를 계산한다. 반환 단위는 mm·MPa.
fn compute(p: &Printed) -> (f64, f64, f64, f64) {
    let geom = geometry_at(p.alpha_deg);
    let derived = compute_geometry_derived(&geom).unwrap();
    let mat = material();
    let contact = hertz::compute_contact_derived(&derived, &mat).unwrap();
    let q_n = p.q_lb * N_PER_LBF;

    let inner = p.label == "내륜";
    let (sum_rho, chi, k_e, e_e, a_star, b_star) = if inner {
        (
            derived.sum_rho_i_per_mm,
            contact.chi_inner,
            contact.k_ellip_inner,
            contact.e_ellip_inner,
            contact.a_star_inner,
            contact.b_star_inner,
        )
    } else {
        (
            derived.sum_rho_e_per_mm,
            contact.chi_outer,
            contact.k_ellip_outer,
            contact.e_ellip_outer,
            contact.a_star_outer,
            contact.b_star_outer,
        )
    };

    let (a, b, p_max) = hertz::contact_ellipse(contact.e_star_mpa, sum_rho, a_star, b_star, q_n);
    // 변형은 ISO (36) 경로를 쓴다 (Harris (6.42) 와 대수적으로 동일 — Theory §3.4)
    let delta = hertz::single_contact_deflection_iso(&mat, sum_rho, chi, k_e, e_e, q_n);
    (a, b, delta, p_max)
}

fn pct(got: f64, want: f64) -> f64 {
    (got / want - 1.0) * 100.0
}

// ═══════════════════════════════════════════════════════════════════
//  B-3a. 인쇄값 자체의 정합 — 대조 전에 자료를 먼저 검산한다
// ═══════════════════════════════════════════════════════════════════

/// 스캔본 판독값을 그대로 믿지 않는다. `p_max = 3Q/(2πab)` 는 Hertz 의 정의이므로
/// **인쇄된 Q·a·b 만으로** 인쇄된 p_max 가 재현되어야 한다. 재현되지 않으면
/// 판독이 틀린 것이고, 그 값을 기준으로 우리 코드를 판정해서는 안 된다.
#[test]
fn b3a_printed_values_are_self_consistent() {
    println!("\n── Level B-3a : Fig. 15 인쇄값 자체 검산 (판독 신뢰성) ──");
    let mut worst = 0.0_f64;
    for p in [&OUTER, &INNER] {
        let recomputed = 3.0 * p.q_lb / (2.0 * std::f64::consts::PI * p.a_in * p.b_in);
        let e = pct(recomputed, p.p_max_psi);
        worst = worst.max(e.abs());
        println!(
            "  {}  3Q/(2πab) = {recomputed:.4e} psi / 인쇄 {:.4e} psi  → {e:+.3} %",
            p.label, p.p_max_psi
        );
    }
    println!("  → 최대 편차 {worst:.3} % (인쇄 유효숫자 5자리 → 반올림 한계 ~0,01 %)");
    assert!(
        worst < 0.05,
        "인쇄값이 자기정합하지 않음 ({worst:.3} %) — 스캔 판독을 다시 확인해야 한다"
    );
}

// ═══════════════════════════════════════════════════════════════════
//  B-3b. 기하 → 점접촉 사슬 대조  ★ 본 검증
// ═══════════════════════════════════════════════════════════════════

/// 판정 기준 **≤ 2 %**.
///
/// 완전 일치는 원리상 기대할 수 없다 — 1973 년 SKF 프로그램은 접촉각을 내륜·외륜
/// 각각으로 두고 원심력·자이로 항까지 포함한 상태에서 곡률을 잡았으며, 우리는
/// 인쇄된 접촉각을 그대로 받아 ISO 16281 (E.4)~(E.7) 로 곡률을 다시 만든다.
/// 두 경로가 같은 값에 수렴하는지를 보는 것이 이 시험의 목적이다.
#[test]
fn b3b_real_bearing_point_contact_chain() {
    println!("\n── Level B-3b : Harris & Mindel Fig. 15 실기 점접촉 대조 ──");
    println!("   레이스  항목        우리          인쇄          오차");
    let mut worst = 0.0_f64;
    let mut worst_name = String::new();

    for p in [&OUTER, &INNER] {
        let (a, b, delta, p_max) = compute(p);
        let rows: [(&str, f64, f64, &str); 4] = [
            ("반장축 a", a / MM_PER_IN, p.a_in, "in"),
            ("반단축 b", b / MM_PER_IN, p.b_in, "in"),
            ("변형 δ", delta / MM_PER_IN, p.delta_in, "in"),
            ("p_max", p_max / MPA_PER_PSI, p.p_max_psi, "psi"),
        ];
        for (name, got, want, unit) in rows {
            let e = pct(got, want);
            let judged = !(name == "변형 δ" && !p.delta_readable);
            if judged && e.abs() > worst {
                worst = e.abs();
                worst_name = format!("{} {name}", p.label);
            }
            println!(
                "   {}   {name:9}  {got:12.5e}  {want:12.5e}  {e:+7.2} %  [{unit}]{}",
                p.label,
                if judged { "" } else { "   ← 판독불가, 판정 제외" }
            );
        }
    }
    println!("  → 최대 오차 {worst:.2} % ({worst_name})   (판정 ≤ 2 %, 판독불가 1건 제외)");
    println!("     a·b 는 양 레이스 모두 −0,68 % 로 동일한 계통 편차다 (p_max 는 그 결과로 +1,37 %).");
    println!("     오차 패턴이 아니라 공통 배율이며, Fig. 15 의 탄성계수 표에 볼의 E 가 없어");
    println!("     (HOUSING·OUTER RING·INNER RING·SHAFT 만 인쇄) E* 를 원 자료로 확정할 수 없다.");
    assert!(worst < 2.0, "실기 점접촉 대조 오차 {worst:.2} % 가 과대 ({worst_name})");
}

// ═══════════════════════════════════════════════════════════════════
//  B-3c. 원 예제의 평형 정합 — 왜 정적 모델로 못 푸는지의 근거
// ═══════════════════════════════════════════════════════════════════

/// 인쇄된 값들이 실제로 **원심력이 있는 평형**을 이루는지 확인한다.
/// 축방향은 원심력이 관여하지 않으므로 우리 축평형식이 그대로 성립해야 하고,
/// 반경방향의 불균형이 정확히 인쇄된 원심력이어야 한다.
///
/// 이것이 성립하면 「이 예제는 정적 모델의 범위 밖」이라는 판단이
/// 추측이 아니라 **자료로 뒷받침되는 사실**이 된다.
#[test]
fn b3c_example_is_outside_static_model_scope() {
    println!("\n── Level B-3c : 원 예제의 평형 (정적 모델 범위 밖임의 근거) ──");
    const F_A_APPLIED_LB: f64 = 3.0e3; // APPLIED LOAD X-AXIS
    const F_C_PRINTED_LB: f64 = 3.0345e2; // CENTRIFUGAL FORCE

    let (si, ci) = INNER.alpha_deg.to_radians().sin_cos();
    let (so, co) = OUTER.alpha_deg.to_radians().sin_cos();

    // ① 볼 1개의 축방향 평형 — 원심력은 반경방향이므로 축에는 관여하지 않는다
    let ax_inner = INNER.q_lb * si;
    let ax_outer = OUTER.q_lb * so;
    let e_ball = pct(ax_outer, ax_inner);

    // ② 베어링 전체 축평형 (우리 식 F_x = Σ Q sin α, 전 볼 동일하중)
    let f_a_sum = Z as f64 * ax_inner;
    let e_total = pct(f_a_sum, F_A_APPLIED_LB);

    // ③ 반경방향 불균형 = 원심력
    let f_c = OUTER.q_lb * co - INNER.q_lb * ci;
    let e_fc = pct(f_c, F_C_PRINTED_LB);

    println!("  ① 볼 축평형   Q_o sinα_o = {ax_outer:8.3} lb / Q_i sinα_i = {ax_inner:8.3} lb  → {e_ball:+.3} %");
    println!("  ② 전체 축평형 Z·Q_i sinα_i = {f_a_sum:8.2} lb / 인가 {F_A_APPLIED_LB:.0} lb        → {e_total:+.3} %");
    println!("  ③ 반경 불균형 Q_o cosα_o − Q_i cosα_i = {f_c:8.3} lb / 인쇄 원심력 {F_C_PRINTED_LB:.2} lb → {e_fc:+.3} %");

    // 볼 1개의 축평형은 **정확히** 맞을 수 없다 — 원 모델의 축방향식 (3.88) 에는
    // 자이로 모멘트 항 λ M_g/D 가 들어 있고 우리 정적 모델은 그것을 쓰지 않는다.
    // 따라서 판정은 「불균형이 인쇄된 자이로 항의 규모를 넘지 않을 것」으로 둔다.
    const M_G_Z_IN_LB: f64 = 2.9765e-1; // GYROSCOPIC MOMENT Z-AXIS
    let gyro_scale = M_G_Z_IN_LB / D_W_IN; // M_g / D  [lb]
    let imbalance = (ax_outer - ax_inner).abs();
    println!("     └ 불균형 {imbalance:.3} lb vs 자이로 항 규모 M_g/D = {gyro_scale:.3} lb");
    assert!(
        imbalance < gyro_scale,
        "볼 축평형 불균형 {imbalance:.3} lb 가 자이로 항 규모 {gyro_scale:.3} lb 를 넘음"
    );
    assert!(e_total.abs() < 0.1, "전체 축평형 불일치 {e_total:.3} %");
    assert!(e_fc.abs() < 0.5, "반경 불균형이 인쇄 원심력과 다름 {e_fc:.3} %");

    // 원심력이 접촉하중과 같은 규모임을 수치로 남긴다
    let ratio = F_C_PRINTED_LB / INNER.q_lb;
    println!("  → 원심력 / 내륜 접촉하중 = {ratio:.2}  ⇒ 정적 모델(원심력 0)로는 재현 불가");
    assert!(
        ratio > 0.5,
        "원심력이 작다면 정적 모델로 풀 수 있어야 한다 — 전제 재확인 필요"
    );
}
