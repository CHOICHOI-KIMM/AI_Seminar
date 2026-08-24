// Level E 검증 — 수명 · 정정격 (Plan §4 Phase 5-1, Theory §9.x)
//
// 판정 대상: BB_Development_Theory.md §7.1~§7.13.
// 실행: cargo test --test life_level_e -- --nocapture
//
// ── 검증 철학 ───────────────────────────────────────────────────────
// **ISO 에는 수치예제가 없다** (Theory §9.2 전문검색 0건).
// 사용자 결정 (2026-08-24): **내부 항등만. 외부 대조 없음.**
//
// 그래서 이 파일의 중심은 **ISO 76 Table 1 을 계산으로 재현**하는 것이다 (E-1).
// ISO/TR 10657:2021 §4.2.1 이 Table 1 의 산출 사슬을 명시하므로
// (TR-19)(TR-20) 에 **우리 솔버의** χ·E(χ) 를 넣어 41행을 되짚을 수 있다.
// 성공하면 표 전사 오류·보간 오류·χ 근찾기가 **동시에** 검증된다.
//
// ── 허용오차의 근거 (자의적으로 정하지 않는다) ──────────────────────
// ISO 76 Table 1 은 `f_0` 를 **소수 첫째 자리까지** 준다. 따라서 계산값이
// 표값과 다를 수 있는 최대폭은 규격 자신의 반올림 반폭 **0,05** 이다.
// 이보다 큰 편차만이 「불일치」다. 이 파일은 41행의 **실측 편차를 전부 출력**한다.

use bb_core::solver::bb::geometry::compute_geometry_derived;
use bb_core::solver::bb::hertz::{self, contact_ellipse, solve_chi};
use bb_core::solver::bb::life::{self, *};
use bb_core::solver::bb::static_rating::{self, *};
use bb_core::solver::bb::types::*;
use bb_core::solver::common::types::*;
use bb_core::solver::common::util;

// ═══════════════════════════════════════════════════════════════════
//  공통 — ISO/TR 10657:2021 §4.2.1 의 f_0 유도 사슬
// ═══════════════════════════════════════════════════════════════════

/// (TR-18) 앞 계수.
const TR_F0_COEFF: f64 = 2.072;
/// (TR-18) 의 응력 정규화 기준 [MPa].
const TR_SIGMA_REF_MPA: f64 = 4000.0;
/// ISO 76 Table 1 이 전제하는 최대 접촉응력 [MPa] (TR §4.2.1).
const TR_SIGMA_MAX_MPA: f64 = 4200.0;
/// 참조 기하 — 내륜 홈 반경비 `f_i = 0,52` → `D_w/(2 r_i) = 1/1,04`.
const INNER_GROOVE_TERM: f64 = 1.0 / 1.04;
/// 참조 기하 — 외륜 홈 반경비 `f_e = 0,53` → `D_w/(2 r_e) = 1/1,06`.
const OUTER_GROOVE_TERM: f64 = 1.0 / 1.06;
/// (TR-16) 의 하중분포 계수 — `C_0r = 0,2 Z Q_max cos α`.
const TR_LOAD_DISTRIBUTION_FACTOR: f64 = 0.2;

/// (TR-19) 의 분모 `2 + γ/(1−γ) − 1/1,04`.
fn tr19_denominator(gamma: f64) -> f64 {
    2.0 + gamma / (1.0 - gamma) - INNER_GROOVE_TERM
}

/// (TR-20) 의 분모 `2 − γ/(1+γ) − 1/1,06`.
fn tr20_denominator(gamma: f64) -> f64 {
    2.0 - gamma / (1.0 + gamma) - OUTER_GROOVE_TERM
}

/// 참조 기하(0,52 / 0,53)에서의 상대 곡률차 — Theory §2.5 (E.6)(E.7).
///
/// `compute_geometry_derived` 와 **같은 값**이어야 한다 (E-2 가 그것을 검사한다).
/// 여기서는 γ = 0 (`D_pw → ∞`) 까지 다뤄야 하므로 닫힌 형태를 직접 쓴다.
fn f_rho_reference(gamma: f64) -> (f64, f64) {
    let f_i = (gamma / (1.0 - gamma) + INNER_GROOVE_TERM) / tr19_denominator(gamma);
    let f_e = (-gamma / (1.0 + gamma) + OUTER_GROOVE_TERM) / tr20_denominator(gamma);
    (f_i, f_e)
}

/// 우리 솔버의 `solve_chi` · `elliptic_k_e_agm` 로 얻은 `(χ, E(χ))`.
fn chi_and_e(f_rho: f64) -> (f64, f64) {
    let chi = solve_chi(f_rho).expect("참조 기하의 F(ρ) 는 [0,1) 안이다");
    let (_, e_ellip) = util::elliptic_k_e_agm(1.0 - 1.0 / (chi * chi));
    (chi, e_ellip)
}

/// 한 궤도륜의 `f_0` — (TR-18).
fn f0_from_tr(chi: f64, e_ellip: f64, denominator: f64) -> f64 {
    TR_F0_COEFF
        * (TR_SIGMA_MAX_MPA / TR_SIGMA_REF_MPA).powi(3)
        * chi
        * (e_ellip / denominator).powi(2)
}

/// γ 하나에 대한 (내륜 f_0, 외륜 f_0, 지배륜의 χ, 지배륜의 E(χ), 지배륜이 내륜인가).
fn f0_pair(gamma: f64) -> (f64, f64, f64, f64, bool) {
    let (f_rho_i, f_rho_e) = f_rho_reference(gamma);
    let (chi_i, e_i) = chi_and_e(f_rho_i);
    let (chi_e, e_e) = chi_and_e(f_rho_e);
    let f0_i = f0_from_tr(chi_i, e_i, tr19_denominator(gamma));
    let f0_e = f0_from_tr(chi_e, e_e, tr20_denominator(gamma));
    if f0_i <= f0_e {
        (f0_i, f0_e, chi_i, e_i, true)
    } else {
        (f0_i, f0_e, chi_e, e_e, false)
    }
}

/// ISO/TR 10657:2021 Annex A Table A.1 — `f_i = 0,52` · `f_e = 0,53` 의 `(γ, κ, E(κ))`.
///
/// **출처**: `Reference/ISO_TR_10657_2021.md` (Annex A, normative).
/// ISO 76 Table 1 은 이 κ·E(κ) 를 (TR-19)(TR-20) 에 넣어 만든 값이다.
/// 여기서는 우리 `solve_chi` 가 이 표를 재현하는지 본다 — **κ 의 출처 차이**를 실측하기 위함이다.
const TR_TABLE_A1: [(f64, f64, f64); 41] = [
    (0.00, 6.418611, 1.033562), (0.01, 6.379259, 1.033904), (0.02, 6.340511, 1.034245),
    (0.03, 6.302379, 1.034589), (0.04, 6.264813, 1.034931), (0.05, 6.227835, 1.035273),
    (0.06, 6.191412, 1.035616), (0.07, 6.155499, 1.035959), (0.08, 6.120131, 1.036300),
    (0.09, 6.085280, 1.036643), (0.10, 8.675157, 1.020324), (0.11, 8.734202, 1.020093),
    (0.12, 8.794353, 1.019862), (0.13, 8.855571, 1.019633), (0.14, 8.917917, 1.019403),
    (0.15, 8.981393, 1.019172), (0.16, 9.046071, 1.018942), (0.17, 9.111951, 1.018712),
    (0.18, 9.179112, 1.018481), (0.19, 9.247558, 1.018251), (0.20, 9.317368, 1.018022),
    (0.21, 9.388544, 1.017792), (0.22, 9.461136, 1.017563), (0.23, 9.535239, 1.017333),
    (0.24, 9.610903, 1.017103), (0.25, 9.688080, 1.016873), (0.26, 9.766937, 1.016644),
    (0.27, 9.847477, 1.016415), (0.28, 9.929755, 1.016184), (0.29, 10.013839, 1.015956),
    (0.30, 10.099802, 1.015726), (0.31, 10.187774, 1.015496), (0.32, 10.277700, 1.015267),
    (0.33, 10.369800, 1.015039), (0.34, 10.464021, 1.014810), (0.35, 10.560521, 1.014581),
    (0.36, 10.659381, 1.014352), (0.37, 10.760716, 1.014124), (0.38, 10.864622, 1.013894),
    (0.39, 10.971136, 1.013665), (0.40, 11.080535, 1.013437),
];

/// ISO 76 Table 1 이 값을 주는 자릿수(소수 첫째 자리)의 **반올림 반폭**.
///
/// 허용오차를 자의적으로 정하지 않기 위한 유일한 객관적 기준이다.
const ISO76_TABLE1_ROUNDING_HALF_WIDTH: f64 = 0.05;

// ═══════════════════════════════════════════════════════════════════
//  E-1. ISO 76 Table 1 의 41행을 계산으로 재현한다  ★ 이 Level 의 중심
// ═══════════════════════════════════════════════════════════════════

#[test]
fn e1_iso76_table1_is_reproduced_from_the_tr10657_chain() {
    println!(
        "\n── E-1 : ISO 76 Table 1 재현 (TR-19/TR-20 + 우리 χ·E(χ), σ_max = {TR_SIGMA_MAX_MPA} MPa)"
    );
    println!("   γ     f_0(내)  f_0(외)  min→f_0   ISO 76   Δ절대     Δ상대     지배륜");

    let mut worst_abs = 0.0_f64;
    let mut worst_rel = 0.0_f64;
    let mut worst_gamma = f64::NAN;

    for (gamma, expected) in F0_TABLE {
        let (f0_i, f0_e, _, _, inner_governs) = f0_pair(gamma);
        let computed = f0_i.min(f0_e);
        let d_abs = computed - expected;
        let d_rel = d_abs / expected;
        if d_abs.abs() > worst_abs {
            worst_abs = d_abs.abs();
            worst_gamma = gamma;
        }
        worst_rel = worst_rel.max(d_rel.abs());
        println!(
            "  {gamma:.2}  {f0_i:8.4} {f0_e:8.4} {computed:8.4}  {expected:7.2}  \
             {d_abs:+8.4}  {:+7.3} %   {}",
            d_rel * 100.0,
            if inner_governs { "내륜" } else { "외륜" }
        );

        // 표 자신의 반올림 반폭보다 큰 편차만이 불일치다.
        assert!(
            d_abs.abs() < ISO76_TABLE1_ROUNDING_HALF_WIDTH,
            "γ = {gamma:.2} : 계산 f_0 = {computed:.4} 가 ISO 76 Table 1 값 {expected} 과 \
             {d_abs:+.4} 차이 — 표의 반올림 반폭 {ISO76_TABLE1_ROUNDING_HALF_WIDTH} 를 넘습니다"
        );
    }
    println!(
        "   최대 편차: |Δ| = {worst_abs:.4} (γ = {worst_gamma:.2}), 상대 {:.3} % \
         — 표의 반올림 반폭 {ISO76_TABLE1_ROUNDING_HALF_WIDTH} 이내\n",
        worst_rel * 100.0
    );
}

#[test]
fn e1b_our_chi_reproduces_tr10657_table_a1() {
    // κ·E(κ) 의 **출처가 다르다** — TR 은 자체 Table A.1, 우리는 solve_chi + AGM.
    // 그 차이가 f_0 재현오차에 얼마나 기여하는지 실측한다.
    println!("\n── E-1b : 지배륜의 χ·E(χ) ↔ ISO/TR 10657 Table A.1");
    let mut worst_chi = 0.0_f64;
    let mut worst_e = 0.0_f64;

    for (gamma, kappa_ref, e_ref) in TR_TABLE_A1 {
        let (_, _, chi, e_ellip, inner) = f0_pair(gamma);
        let d_chi = (chi - kappa_ref) / kappa_ref;
        let d_e = (e_ellip - e_ref) / e_ref;
        worst_chi = worst_chi.max(d_chi.abs());
        worst_e = worst_e.max(d_e.abs());
        println!(
            "  {gamma:.2}  χ = {chi:11.6} (TR {kappa_ref:11.6}, Δ {:+.5} %)  \
             E = {e_ellip:.6} (TR {e_ref:.6}, Δ {:+.5} %)  {}",
            d_chi * 100.0,
            d_e * 100.0,
            if inner { "내륜" } else { "외륜" }
        );
        assert!(d_chi.abs() < 1.0e-4, "γ={gamma:.2} χ 편차 {d_chi:.3e}");
        assert!(d_e.abs() < 1.0e-4, "γ={gamma:.2} E(χ) 편차 {d_e:.3e}");
    }
    // f_0 ∝ χ·E(χ)² 이므로 출처 차이의 f_0 기여는 대략 |Δχ| + 2|ΔE| 이다.
    println!(
        "   최대 Δχ = {worst_chi:.3e}, 최대 ΔE = {worst_e:.3e} \
         → f_0 기여 ≈ {:.3e} (= {:.5} %)\n",
        worst_chi + 2.0 * worst_e,
        (worst_chi + 2.0 * worst_e) * 100.0
    );
}

#[test]
fn e1c_f0_peak_is_the_governing_ring_crossover() {
    // Theory §7.11 의 「γ = 0,09 에서 16,5 최대」는 전사 오류가 아니라
    // **지배 궤도륜이 외륜 → 내륜으로 바뀌는 지점**임을 계산으로 확인한다.
    for gamma in [0.0, 0.05, 0.09] {
        let (f0_i, f0_e, _, _, inner) = f0_pair(gamma);
        assert!(!inner, "γ={gamma}: 외륜이 지배해야 한다 (내 {f0_i}, 외 {f0_e})");
    }
    for gamma in [0.10, 0.20, 0.40] {
        let (f0_i, f0_e, _, _, inner) = f0_pair(gamma);
        assert!(inner, "γ={gamma}: 내륜이 지배해야 한다 (내 {f0_i}, 외 {f0_e})");
    }
    // 표의 최대도 같은 자리다.
    let peak = f_0(F0_PEAK_GAMMA).unwrap();
    assert!((peak - 16.5).abs() < 1e-12);
    for (gamma, value) in F0_TABLE {
        assert!(value <= peak + 1e-12, "γ={gamma} 의 값이 최대를 넘는다");
    }
}

// ═══════════════════════════════════════════════════════════════════
//  E-2. (TR-19)(TR-20) 의 분모 ↔ 우리 Σρ · D_w/2  — 대수적 항등
// ═══════════════════════════════════════════════════════════════════

/// γ 를 지정해 참조 기하(0,52 / 0,53)의 베어링을 만든다. `α = 0` 이므로 `γ = D_w/D_pw`.
fn geometry_for_gamma(gamma: f64) -> BallBearingGeometry {
    let d_w_mm = 12.7;
    let (r_i_mm, r_e_mm) = BallBearingGeometry::reference_groove_radii(d_w_mm);
    let d_pw_mm = d_w_mm / gamma;
    BallBearingGeometry {
        bore_mm: d_pw_mm - 2.0 * d_w_mm,
        outer_diameter_mm: d_pw_mm + 2.0 * d_w_mm,
        width_mm: 2.0 * d_w_mm,
        z: 16,
        d_w_mm,
        d_pw_mm,
        r_i_mm,
        r_e_mm,
        alpha_nom_rad: 0.0,
        clearance: BbClearanceSpec::InitialAngleRad(0.0),
    }
}

#[test]
fn e2_curvature_sum_equals_the_tr10657_denominators() {
    // Σρ_i = (2/D_w)(2 + γ/(1−γ) − D_w/(2r_i)),  r_i = 0,52 D_w ⇒ D_w/(2r_i) = 1/1,04
    // ⇒ Σρ_i · D_w/2 = (TR-19) 의 분모. 외륜도 같은 구조다.
    println!("\n── E-2 : Σρ · D_w/2 ↔ (TR-19)(TR-20) 분모");
    for gamma in [0.05, 0.09, 0.10, 0.19, 0.25, 0.40] {
        let geom = geometry_for_gamma(gamma);
        let d = compute_geometry_derived(&geom).unwrap();
        assert!((d.gamma - gamma).abs() < 1e-14, "γ 재현 실패");

        let lhs_i = d.sum_rho_i_per_mm * geom.d_w_mm / 2.0;
        let lhs_e = d.sum_rho_e_per_mm * geom.d_w_mm / 2.0;
        let rhs_i = tr19_denominator(gamma);
        let rhs_e = tr20_denominator(gamma);
        println!(
            "  γ={gamma:.2}  내 {lhs_i:.12} ↔ {rhs_i:.12}   외 {lhs_e:.12} ↔ {rhs_e:.12}"
        );
        assert!((lhs_i - rhs_i).abs() / rhs_i < 1e-14, "내륜 항등 실패 γ={gamma}");
        assert!((lhs_e - rhs_e).abs() / rhs_e < 1e-14, "외륜 항등 실패 γ={gamma}");

        // F(ρ) 도 같은 사슬에서 나온다 — 닫힌 형태 ↔ 솔버 전처리
        let (f_i, f_e) = f_rho_reference(gamma);
        assert!((d.f_rho_i - f_i).abs() < 1e-14);
        assert!((d.f_rho_e - f_e).abs() < 1e-14);
    }
    println!();
}

// ═══════════════════════════════════════════════════════════════════
//  E-3. Q_max = C_0r/(0,2 Z cos α) → p_max ≈ 4 200 MPa
// ═══════════════════════════════════════════════════════════════════

#[test]
fn e3_static_rating_load_produces_4200_mpa_on_the_governing_raceway() {
    println!("\n── E-3 : Q_max = C_0r/(0,2 Z cos α) 에서의 p_max");
    for gamma in [0.05, 0.09, 0.10, 0.20, 0.30, 0.40] {
        let geom = geometry_for_gamma(gamma);
        let material = Material::default();
        let derived = compute_geometry_derived(&geom).unwrap();
        let contact = hertz::compute_contact_derived(&derived, &material).unwrap();

        let f_0_value = f_0(gamma).unwrap();
        // 표값(소수 첫째 자리)이 아니라 **계산값**을 쓴다 — 반올림이 응력에 섞이지 않게 한다.
        let (f0_i, f0_e, _, _, inner_governs) = f0_pair(gamma);
        let f_0_exact = f0_i.min(f0_e);

        let c_0r = basic_static_radial_load_rating_n(
            f_0_exact,
            geom.z,
            geom.d_w_mm,
            geom.alpha_nom_rad,
        );
        let q_max = c_0r
            / (TR_LOAD_DISTRIBUTION_FACTOR * f64::from(geom.z) * geom.alpha_nom_rad.cos());

        let (_, _, p_inner) = contact_ellipse(
            contact.e_star_mpa,
            derived.sum_rho_i_per_mm,
            contact.a_star_inner,
            contact.b_star_inner,
            q_max,
        );
        let (_, _, p_outer) = contact_ellipse(
            contact.e_star_mpa,
            derived.sum_rho_e_per_mm,
            contact.a_star_outer,
            contact.b_star_outer,
            q_max,
        );
        let p_gov = if inner_governs { p_inner } else { p_outer };
        let rel = (p_gov - TR_SIGMA_MAX_MPA) / TR_SIGMA_MAX_MPA;
        println!(
            "  γ={gamma:.2}  f_0(표) {f_0_value:5.2} / f_0(계산) {f_0_exact:7.4}  \
             Q_max = {q_max:9.1} N  p_i = {p_inner:7.1}  p_e = {p_outer:7.1}  \
             지배 {p_gov:7.1} MPa (Δ {:+.4} %)",
            rel * 100.0
        );
        // (TR-18) 의 2,072 는 절사된 계수다 — 그만큼만 어긋난다.
        assert!(
            rel.abs() < 1.0e-3,
            "γ={gamma}: 지배 궤도륜의 p_max = {p_gov:.1} MPa 가 4 200 MPa 에서 {:.3} % 벗어남",
            rel * 100.0
        );
        // 지배륜 쪽이 더 높은 응력을 받아야 한다 (그래서 f_0 가 작다).
        assert!(
            p_gov >= p_inner.min(p_outer) - 1e-9,
            "지배륜 판정과 응력 크기가 어긋난다"
        );
    }
    println!();
}

// ═══════════════════════════════════════════════════════════════════
//  E-4. 비단조 표의 보간 — 값 기반 탐색이면 반드시 틀린다
// ═══════════════════════════════════════════════════════════════════

#[test]
fn e4_interpolation_does_not_assume_monotonicity() {
    // (a) 41 격자점 완전 재현
    for (gamma, expected) in F0_TABLE {
        assert!((f_0(gamma).unwrap() - expected).abs() < 1e-12, "γ={gamma}");
    }
    // (b) 표가 **실제로 비단조**임을 확인 (증가 구간과 감소 구간이 둘 다 존재)
    let mut has_up = false;
    let mut has_down = false;
    for w in F0_TABLE.windows(2) {
        if w[1].1 > w[0].1 {
            has_up = true;
        }
        if w[1].1 < w[0].1 {
            has_down = true;
        }
    }
    assert!(has_up && has_down, "ISO 76 Table 1 은 비단조여야 한다");

    // (c) 굴곡점을 **가로지르는** 구간의 보간이 이웃 격자점의 선형내삽과 같아야 한다.
    //     0,09(16,5) → 0,10(16,4) 은 감소 구간이다. 값 기준 이분탐색이면 여기서 깨진다.
    assert!((f_0(0.095).unwrap() - 16.45).abs() < 1e-12);
    // 그 직전 0,08(16,3) → 0,09(16,5) 는 증가 구간이다.
    assert!((f_0(0.085).unwrap() - 16.4).abs() < 1e-12);
    // 같은 f_0 값(16,4)이 γ = 0,085 와 γ = 0,10 두 곳에서 나온다 — **역함수가 없다**.
    assert!((f_0(0.10).unwrap() - 16.4).abs() < 1e-12);

    // (d) f_c (ISO 281 Table 2) 도 γ = 0,19 에서 최대 60,0 인 비단조 표다.
    for (gamma, expected) in F_C_TABLE {
        assert!((f_c(gamma).unwrap() - expected).abs() < 1e-12, "γ={gamma}");
    }
    assert!((f_c(0.185).unwrap() - 59.95).abs() < 1e-12);
    assert!((f_c(0.195).unwrap() - 59.95).abs() < 1e-12);
    assert!((f_c(F_C_PEAK_GAMMA).unwrap() - 60.0).abs() < 1e-12);

    // (e) 정의역 밖은 외삽하지 않고 오류다 (규격이 값을 주지 않는다).
    assert!(f_0(-1e-9).is_err());
    assert!(f_0(0.40 + 1e-9).is_err());
    assert!(f_c(0.01 - 1e-9).is_err());
    assert!(f_c(0.40 + 1e-9).is_err());
}

// ═══════════════════════════════════════════════════════════════════
//  E-5. P_0r = max{(76-2), (76-3)} 의 분기 경계
// ═══════════════════════════════════════════════════════════════════

#[test]
fn e5_p0r_branches_meet_at_the_crossover_axial_load() {
    // X_0 F_r + Y_0 F_a = F_r  ⇔  F_a* = F_r (1 − X_0) / Y_0
    for alpha_deg in [5.0_f64, 15.0, 25.0, 40.0, 45.0] {
        let (x_0, y_0) = x0_y0(alpha_deg.to_radians()).unwrap();
        let f_r = 1_000.0;
        let f_a_cross = f_r * (1.0 - x_0) / y_0;

        let at = static_equivalent_radial_load_n(x_0, y_0, f_r, f_a_cross);
        assert!(
            (at - f_r).abs() < 1e-9,
            "α={alpha_deg}° 경계에서 두 식이 만나지 않는다: {at} vs {f_r}"
        );

        // 경계 아래는 (76-3) 이, 위는 (76-2) 가 이긴다.
        let below = static_equivalent_radial_load_n(x_0, y_0, f_r, f_a_cross * 0.5);
        assert!((below - f_r).abs() < 1e-12, "α={alpha_deg}° 경계 아래");
        let above_fa = f_a_cross * 2.0;
        let above = static_equivalent_radial_load_n(x_0, y_0, f_r, above_fa);
        assert!(
            (above - (x_0 * f_r + y_0 * above_fa)).abs() < 1e-12,
            "α={alpha_deg}° 경계 위"
        );
        assert!(above > f_r);
    }
    // α = 0 (레이디얼 접촉) 행도 같은 구조를 갖는다.
    let (x_0, y_0) = x0_y0(0.0).unwrap();
    let f_a_cross = 1_000.0 * (1.0 - x_0) / y_0;
    assert!((static_equivalent_radial_load_n(x_0, y_0, 1_000.0, f_a_cross) - 1_000.0).abs() < 1e-9);

    // 순수 반경하중이면 언제나 (76-3).
    assert!((static_equivalent_radial_load_n(0.5, 0.26, 500.0, 0.0) - 500.0).abs() < 1e-12);
    // 순수 축하중이면 언제나 (76-2).
    assert!((static_equivalent_radial_load_n(0.5, 0.26, 0.0, 500.0) - 130.0).abs() < 1e-12);

    // S_0 = C_0r / P_0r (76-14)
    assert!((static_safety_factor(2_000.0, 500.0) - 4.0).abs() < 1e-12);
}

// ═══════════════════════════════════════════════════════════════════
//  E-6. Y_0 보간이 α = 0 과 섞이지 않는가
// ═══════════════════════════════════════════════════════════════════

#[test]
fn e6_y0_interpolates_only_inside_the_angular_contact_range() {
    // (a) α = 0 행은 그대로. X_0 = 0,6 은 **각접촉 어디에서도 나오면 안 된다**.
    let (x0_radial, y0_radial) = x0_y0(0.0).unwrap();
    assert!((x0_radial - X0_RADIAL_CONTACT).abs() < 1e-15);
    assert!((y0_radial - Y0_RADIAL_CONTACT).abs() < 1e-15);

    // (b) 각접촉 격자점 완전 재현 + X_0 상수
    for (deg, y_0_expected) in Y0_ANGULAR_TABLE_DEG {
        let (x_0, y_0) = x0_y0(deg.to_radians()).unwrap();
        assert!((x_0 - X0_ANGULAR_CONTACT).abs() < 1e-15, "α={deg}° 의 X_0");
        assert!((y_0 - y_0_expected).abs() < 1e-12, "α={deg}° 의 Y_0");
    }

    // (c) 각접촉 구간 전역에서 X_0 는 0,5 이고 Y_0 는 [0,22, 0,52] 안이다.
    //     α = 0 의 (0,6 · 0,5) 가 섞였다면 X_0 가 0,5 를 벗어나거나
    //     Y_0 가 5° 근처에서 0,5 쪽으로 끌려간다.
    let mut deg = Y0_ANGULAR_MIN_DEG;
    while deg <= Y0_ANGULAR_MAX_DEG + 1e-12 {
        let (x_0, y_0) = x0_y0(deg.to_radians()).unwrap();
        assert!((x_0 - 0.5).abs() < 1e-15, "α={deg}° 에서 X_0 = {x_0}");
        assert!((0.22..=0.52).contains(&y_0), "α={deg}° 에서 Y_0 = {y_0}");
        deg += 0.25;
    }
    // 5° 바로 위에서 Y_0 는 0,52 에서 **내려간다** (0,5 쪽으로 올라가지 않는다).
    assert!(x0_y0(5.5_f64.to_radians()).unwrap().1 < 0.52);

    // (d) 0 < α < 5° 는 규격이 값을 주지 않는다 — 보간도 외삽도 하지 않는다.
    for deg in [1e-6_f64, 1.0, 2.5, 4.999] {
        assert!(x0_y0(deg.to_radians()).is_err(), "α={deg}° 는 오류여야 한다");
    }
    // 45° 초과도 마찬가지.
    assert!(x0_y0(45.001_f64.to_radians()).is_err());

    // (e) 중간각 선형보간 — 표에 없는 각은 이웃 두 행의 내삽이어야 한다.
    assert!((x0_y0(22.5_f64.to_radians()).unwrap().1 - 0.40).abs() < 1e-12);
    assert!((x0_y0(37.5_f64.to_radians()).unwrap().1 - 0.275).abs() < 1e-12);
}

// ═══════════════════════════════════════════════════════════════════
//  E-7. 동정격 쪽 — f_c · X/Y/e 보간 경계
// ═══════════════════════════════════════════════════════════════════

#[test]
fn e7_dynamic_rating_tables_and_boundaries() {
    // (a) f_c 40 격자점 + 최대 60,0 @ γ = 0,19 (Theory §7.2)
    assert_eq!(F_C_TABLE.len(), 40);
    for (gamma, expected) in F_C_TABLE {
        assert!((f_c(gamma).unwrap() - expected).abs() < 1e-12);
        assert!(expected <= 60.0 + 1e-12);
    }

    // (b) X/Y/e Table 3 (단열) 격자점 + 정의역
    for (deg, x, y, e) in XYE_TABLE_DEG {
        let (gx, gy, ge) = x_y_e(deg.to_radians(), None).unwrap();
        assert!((gx - x).abs() < 1e-12 && (gy - y).abs() < 1e-12 && (ge - e).abs() < 1e-12);
    }
    // 중간각은 선형보간
    let (x, y, e) = x_y_e(22.5_f64.to_radians(), None).unwrap();
    assert!((x - 0.42).abs() < 1e-12);
    assert!((y - 0.935).abs() < 1e-12);
    assert!((e - 0.625).abs() < 1e-12);
    // P5-1b: α < 20° 는 이제 **상대 축하중을 주면** 결정된다. 없으면 여전히 오류다.
    for deg in [0.0_f64, 5.0, 10.0, 15.0, 19.999] {
        assert!(
            x_y_e(deg.to_radians(), None).is_err(),
            "α={deg}° 는 상대 축하중 없이는 결정할 수 없다"
        );
        assert!(x_y_e(deg.to_radians(), Some(1.0)).is_ok(), "α={deg}°");
    }
    assert!(x_y_e(45.001_f64.to_radians(), None).is_err());
    assert!(x_y_e(-0.001_f64.to_radians(), Some(1.0)).is_err());

    // (c) P 의 바깥 겹 분기 — F_a/F_r ≤ e 이면 P = F_r
    let alpha = 40.0_f64.to_radians();
    let (x, y, e) = x_y_e(alpha, None).unwrap();
    let f_r = 1_000.0;
    let z = 16;
    let d_w = 11.5;
    let below = dynamic_equivalent_radial_load_n(alpha, f_r, f_r * e * 0.99, z, d_w).unwrap();
    assert!((below - f_r).abs() < 1e-12);
    let f_a = f_r * e * 1.01;
    let above = dynamic_equivalent_radial_load_n(alpha, f_r, f_a, z, d_w).unwrap();
    assert!((above - (x * f_r + y * f_a)).abs() < 1e-12);
    // 경계에서 두 식의 차이는 e 정의상 연속에 가깝다 (X + Y e ≈ 1 은 규격이 보장하지 않는다)
    // → 여기서는 **분기 자체가 존재**하고 큰 쪽이 아니라 **조건**으로 갈린다는 사실만 고정한다.
    assert!(above > 0.0);

    // (d) Q_ci / Q_ce — 내륜 0,407 · 외륜 0,389, 중괄호 지수 ±10/3
    assert!((Q_C_INNER_COEFF - 0.407).abs() < 1e-15);
    assert!((Q_C_OUTER_COEFF - 0.389).abs() < 1e-15);
    let gamma = 0.19;
    let d_w = 12.7;
    let (r_i, r_e) = BallBearingGeometry::reference_groove_radii(d_w);
    let t = q_c_common_term(gamma, r_i, r_e, d_w);
    let c_r = 50_000.0;
    let z = 16;
    let q_ci = q_ci_n(c_r, z, alpha, t);
    let q_ce = q_ce_n(c_r, z, alpha, t);
    // 지수 부호가 반전되어 있는지 — 내륜식에서 T→1/T 로 바꾸면 외륜식의 괄호와 같아진다.
    let inner_bracket = (1.0 + t.powf(10.0 / 3.0)).powf(0.3);
    let outer_bracket = (1.0 + t.powf(-10.0 / 3.0)).powf(0.3);
    assert!(
        (q_ci - c_r / (Q_C_INNER_COEFF * f64::from(z) * alpha.cos()) * inner_bracket).abs()
            < 1e-6
    );
    assert!(
        (q_ce - c_r / (Q_C_OUTER_COEFF * f64::from(z) * alpha.cos()) * outer_bracket).abs()
            < 1e-6
    );

    // (e) 롤러 계수 오용 방지 — 볼 값인지 고정한다 (Theory §7.8 · §7.9 경고)
    assert!((C_U_BALL_COEFF - 0.2288).abs() < 1e-15, "롤러 (B.14) 의 0,2453 이 섞였다");
    assert!((C_U_SIMPLE_DIVISOR - 22.0).abs() < 1e-15, "롤러 (B.20) 의 8,2 가 섞였다");
    assert!((B_M_BALL - 1.3).abs() < 1e-15);
    assert!((SIGMA_HU_MPA - 1_500.0).abs() < 1e-12);
    // a_ISO 볼 식 (31)~(33): κ = 1, e_C C_u/P = 0 이면 대괄호 = 1 → a_ISO = 0,1
    assert!((a_iso_ball(1.0, 0.0).unwrap() - 0.1).abs() < 1e-12);

    // (f) a_ISO 구간 경계의 연속성 — (32)/(33) 은 κ = 1 에서 같은 계수 1,9987 를 쓴다
    let below = a_iso_ball(1.0 - 1e-9, 0.5).unwrap();
    let above = a_iso_ball(1.0, 0.5).unwrap();
    assert!((below / above - 1.0).abs() < 1e-6, "κ = 1 경계 {below} vs {above}");
    // (31)/(32) 는 κ = 0,4 에서 계수가 다르므로 **불연속이 규격 자체**다 — 크기만 본다
    let b04 = a_iso_ball(0.4 - 1e-9, 0.5).unwrap();
    let a04 = a_iso_ball(0.4, 0.5).unwrap();
    assert!(b04 > 0.0 && a04 > 0.0);
    // κ < 0,1 은 계산 불가
    assert!(a_iso_ball(0.099, 0.5).is_err());
    // κ > 4 는 4 로 클램프
    assert!((a_iso_ball(4.0, 0.5).unwrap() - a_iso_ball(40.0, 0.5).unwrap()).abs() < 1e-12);
    // 상한 50
    assert!(a_iso_ball(2.0, 100.0).unwrap() <= A_ISO_CAP + 1e-12);

    // (g) ν₁ 분기 (28)/(29)
    let b = nu_1_mm2_s(999.0, 70.0).unwrap();
    let a = nu_1_mm2_s(1_001.0, 70.0).unwrap();
    assert!((b / a - 1.0).abs() < 0.05, "ν₁ 분기 {b} vs {a}");
}

// ═══════════════════════════════════════════════════════════════════
//  E-8. 차원 · 스케일링
// ═══════════════════════════════════════════════════════════════════

fn life_input() -> BbInput {
    let d_w_mm = 11.5;
    let (r_i_mm, r_e_mm) = BallBearingGeometry::reference_groove_radii(d_w_mm);
    BbInput {
        kind: BallBearingKind::Acbb,
        geometry: BallBearingGeometry {
            bore_mm: 50.0,
            outer_diameter_mm: 90.0,
            width_mm: 20.0,
            z: 16,
            d_w_mm,
            d_pw_mm: 70.0,
            r_i_mm,
            r_e_mm,
            alpha_nom_rad: 40.0_f64.to_radians(),
            clearance: BbClearanceSpec::InitialAngleRad(40.0_f64.to_radians()),
        },
        material: Material::default(),
        operating: BbOperatingConditions {
            f_x_n: 4_000.0,
            f_y_n: 2_000.0,
            f_z_n: 0.0,
            m_y_nmm: 0.0,
            m_z_nmm: 0.0,
            n_inner_rpm: 1_500.0,
            n_outer_rpm: 0.0,
            temperature_c: 70.0,
        },
        solver: BbSolverParams::default(),
    }
}

#[test]
fn e8_dimensions_and_scaling_of_the_life_chain() {
    let input = life_input();
    let conditions = BbLifeConditions {
        nu_mm2_s: 30.0,
        ..BbLifeConditions::default()
    };
    let r = bb_core::commands::solve_life(&input, &conditions).unwrap();

    println!("\n── E-8 : 수명 사슬 차원");
    println!(
        "  f_c = {:.3}  C_r = {:.1} N  C_0r = {:.1} N  P_0r = {:.1} N  S_0 = {:.3}",
        r.f_c, r.c_r_n, r.static_rating.c_0r_n, r.static_rating.p_0r_n, r.static_rating.s_0
    );
    println!(
        "  L_10r = {:.4} ×10⁶ rev   P_ref r = {:.1} N   κ = {:?}   a_ISO = {:?}   L_nmr = {:?}",
        r.l_10r_mrev, r.p_ref_r_n, r.kappa, r.a_iso, r.l_nmr_mrev
    );

    // (a) C_0r [N] — (76-1) 로 되짚는다
    let f_0_value = f_0(r.static_rating.gamma).unwrap();
    let c_0r_check = f_0_value
        * f64::from(input.geometry.z)
        * input.geometry.d_w_mm.powi(2)
        * input.geometry.alpha_nom_rad.cos();
    assert!((r.static_rating.c_0r_computed_n - c_0r_check).abs() / c_0r_check < 1e-12);
    assert!(r.static_rating.c_0r_n > 0.0 && r.static_rating.c_0r_n.is_finite());

    // (b) S_0 = C_0r / P_0r — 무차원
    assert!(
        (r.static_rating.s_0 - r.static_rating.c_0r_n / r.static_rating.p_0r_n).abs() < 1e-9
    );
    // P_0r 은 F_r 보다 작을 수 없다 (76-3)
    assert!(r.static_rating.p_0r_n >= r.static_rating.f_r_n - 1e-9);
    // F_r · F_a 가 좌표계 규약대로 조립되었는가 (D-7)
    let f_r_expected =
        (input.operating.f_y_n.powi(2) + input.operating.f_z_n.powi(2)).sqrt();
    assert!((r.static_rating.f_r_n - f_r_expected).abs() < 1e-12);
    assert!((r.static_rating.f_a_n - input.operating.f_x_n.abs()).abs() < 1e-12);

    // (c) C_0r 은 **공칭** 접촉각을 쓴다 — α₀·α_j 가 아니다.
    //     같은 기하에서 클리어런스만 바꿔 α₀ 를 흔들어도 C_0r 이 변하면 안 된다.
    let mut moved = input.clone();
    moved.geometry.clearance = BbClearanceSpec::InitialAngleRad(20.0_f64.to_radians());
    let r2 = bb_core::commands::solve_life(&moved, &conditions).unwrap();
    assert!(
        (r2.static_rating.c_0r_computed_n - r.static_rating.c_0r_computed_n).abs()
            / r.static_rating.c_0r_computed_n
            < 1e-14,
        "C_0r 이 α₀ 에 따라 변한다 — 공칭 접촉각을 쓰지 않고 있다"
    );
    assert!(
        (r2.c_r_computed_n - r.c_r_computed_n).abs() / r.c_r_computed_n < 1e-14,
        "C_r 이 α₀ 에 따라 변한다"
    );

    // (d) L_10r [10⁶ rev] · P_ref r [N] — 식 (11)
    assert!(r.l_10r_mrev.is_finite() && r.l_10r_mrev > 0.0);
    assert!((r.p_ref_r_n - r.c_r_n / r.l_10r_mrev.cbrt()).abs() / r.p_ref_r_n < 1e-12);

    // (e) 볼하중을 s 배 하면 L_10r 은 s^(−3) — 식 (9) 의 지수 구조
    let loads: Vec<f64> = (0..16).map(|j| 100.0 * (1.0 + f64::from(j))).collect();
    let q_ei = equivalent_ball_load_n(&loads, ROTATING_RING_EXPONENT);
    let q_ee = equivalent_ball_load_n(&loads, STATIONARY_RING_EXPONENT);
    let base = l_10r_mrev(r.q_ci_n, q_ei, r.q_ce_n, q_ee);
    for s in [0.5_f64, 2.0, 4.0] {
        let scaled: Vec<f64> = loads.iter().map(|q| q * s).collect();
        let q_ei_s = equivalent_ball_load_n(&scaled, ROTATING_RING_EXPONENT);
        let q_ee_s = equivalent_ball_load_n(&scaled, STATIONARY_RING_EXPONENT);
        let life = l_10r_mrev(r.q_ci_n, q_ei_s, r.q_ce_n, q_ee_s);
        let expected = base * s.powi(-3);
        assert!(
            (life - expected).abs() / expected < 1e-12,
            "s = {s}: L_10r = {life}, 기대 {expected}"
        );
    }

    // (f) 하중 ↑ → 수명 ↓ (단조)
    let mut prev = f64::INFINITY;
    for scale in [1.0_f64, 1.5, 2.0, 3.0] {
        let mut heavier = input.clone();
        heavier.operating.f_x_n *= scale;
        heavier.operating.f_y_n *= scale;
        let rr = bb_core::commands::solve_life(&heavier, &conditions).unwrap();
        assert!(rr.l_10r_mrev < prev, "하중 {scale}× 에서 수명이 줄지 않았다");
        prev = rr.l_10r_mrev;
    }

    // (g) L_nmr = a_1 · a_ISO · L_10r — 식 (13)
    let a_iso = r.a_iso.expect("ν 를 주었으므로 a_ISO 가 나와야 한다");
    let l_nmr = r.l_nmr_mrev.expect("a_ISO 가 있으면 L_nmr 도 있어야 한다");
    assert!((l_nmr - r.a_1 * a_iso * r.l_10r_mrev).abs() / l_nmr < 1e-12);
    // κ = ν/ν₁ — 식 (27)
    let nu_1 = r.nu_1_mm2_s.unwrap();
    assert!((r.kappa.unwrap() - conditions.nu_mm2_s / nu_1).abs() < 1e-12);

    // (h) ν 미지정이면 κ·a_ISO·L_nmr 을 **지어내지 않는다**
    let no_lube = bb_core::commands::solve_life(&input, &BbLifeConditions::default()).unwrap();
    assert!(no_lube.kappa.is_none());
    assert!(no_lube.a_iso.is_none());
    assert!(no_lube.l_nmr_mrev.is_none());
    assert!(no_lube.alerts.iter().any(|a| a.code == "KAPPA_UNAVAILABLE"));
    // L_10r 은 윤활과 무관하다
    assert!((no_lube.l_10r_mrev - r.l_10r_mrev).abs() / r.l_10r_mrev < 1e-12);

    // (i) C_u 두 경로 — 같은 오더인가 (Plan P5-1 「C_u 2경로」)
    println!(
        "  C_u 정밀법 = {:.1} N, 간이법 = {:.1} N, 비 = {:.3}",
        r.c_u_precise_n,
        r.c_u_simplified_n,
        r.c_u_simplified_n / r.c_u_precise_n
    );
    assert!(r.c_u_precise_n > 0.0 && r.c_u_simplified_n > 0.0);
    let ratio = r.c_u_simplified_n / r.c_u_precise_n;
    assert!((0.1..=10.0).contains(&ratio), "C_u 두 경로가 오더가 다르다: {ratio}");
    println!();
}

#[test]
fn e8b_life_conditions_reject_out_of_range_inputs() {
    let input = life_input();
    for bad in [
        BbLifeConditions { e_c: -0.1, ..Default::default() },
        BbLifeConditions { e_c: 1.1, ..Default::default() },
        BbLifeConditions { a_1: 0.0, ..Default::default() },
        BbLifeConditions { nu_mm2_s: -1.0, ..Default::default() },
    ] {
        assert!(bb_core::commands::solve_life(&input, &bad).is_err());
    }
    // 기본값은 통과한다
    assert!(bb_core::commands::solve_life(&input, &BbLifeConditions::default()).is_ok());
}

#[test]
fn e8c_static_rating_alerts_fire_on_oversized_grooves() {
    let mut input = life_input();
    input.geometry.r_i_mm = 0.56 * input.geometry.d_w_mm;
    input.geometry.r_e_mm = 0.57 * input.geometry.d_w_mm;
    let r = bb_core::commands::solve_life(&input, &BbLifeConditions::default()).unwrap();
    assert!(
        r.static_rating
            .alerts
            .iter()
            .any(|a| a.code == "STATIC_RATING_GROOVE_OVER_REFERENCE"),
        "홈 반경 초과 경고가 없다"
    );
}

// ═══════════════════════════════════════════════════════════════════
//  참조 — 모듈 경로 고정 (리팩터 시 조용히 깨지지 않게)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn e0_module_paths_are_stable() {
    let _ = life::f_c(0.19).unwrap();
    let _ = static_rating::f_0(0.09).unwrap();
    assert_eq!(F0_TABLE.len(), 41);
    assert_eq!(TR_TABLE_A1.len(), 41);
    assert_eq!(Y0_ANGULAR_TABLE_DEG.len(), 9);
    assert_eq!(XYE_TABLE_DEG.len(), 6);
}

// ═══════════════════════════════════════════════════════════════════
//  E-10. ISO 281 Table 3 **상단부** — α < 20° (Theory §7.3.1, Plan P5-1b)
// ═══════════════════════════════════════════════════════════════════
//
// 전사(Theory §7.3.1)를 코드가 그대로 재현하는지, 그리고 §7.3.1 이 명시한
// **함정 2건**을 구현이 실제로 지키는지 고정한다.

/// §7.3.1 E-9 — 제2열 격자의 원값 [psi].
const XYE_COL2_PSI: [f64; 9] = [25.0, 50.0, 100.0, 150.0, 200.0, 300.0, 500.0, 750.0, 1000.0];
/// psi → MPa 환산 (§7.3.1 E-9 가 명시한 값). **테스트 전용** — 솔버는 MPa 만 쓴다.
const PSI_TO_MPA: f64 = 0.006_894_8;

/// Theory §7.3.1 「제1열」 전사 — α = 5°/10°/15° 각 9점.
const XYE_COL1_TRANSCRIBED: [(f64, [f64; 9]); 3] = [
    (5.0, [0.173, 0.346, 0.692, 1.04, 1.38, 2.08, 3.46, 5.19, 6.92]),
    (10.0, [0.175, 0.35, 0.7, 1.05, 1.4, 2.1, 3.5, 5.25, 7.0]),
    (15.0, [0.178, 0.357, 0.714, 1.07, 1.43, 2.14, 3.57, 5.35, 7.14]),
];

/// ISO 자릿수 = **유효숫자 3자리** (0,172 · 1,03 · 6,89 · 7,00 이 모두 3자리다).
fn round_to_3_significant_digits(v: f64) -> f64 {
    if v == 0.0 {
        return 0.0;
    }
    let factor = 10f64.powf(2.0 - v.abs().log10().floor());
    (v * factor).round() / factor
}

#[test]
fn e10a_low_table_grid_points_are_reproduced_exactly() {
    assert_eq!(XYE_ROW_GRID.len(), 9);
    assert_eq!(XYE_LOW_SINGLE.len(), 5);
    for w in XYE_ROW_GRID.windows(2) {
        assert!(w[1] > w[0], "제2열 격자가 오름차순이 아니다");
    }
    for block in XYE_LOW_SINGLE {
        for (k, rel) in XYE_ROW_GRID.iter().enumerate() {
            let (x, y, e) = x_y_e_low(block.alpha_deg.to_radians(), *rel);
            assert!(
                (x - block.x).abs() < 1e-12,
                "α={}° r={rel} X {x} ≠ {}",
                block.alpha_deg,
                block.x
            );
            assert!(
                (y - block.y[k]).abs() < 1e-12,
                "α={}° r={rel} Y {y} ≠ {}",
                block.alpha_deg,
                block.y[k]
            );
            assert!(
                (e - block.e[k]).abs() < 1e-12,
                "α={}° r={rel} e {e} ≠ {}",
                block.alpha_deg,
                block.e[k]
            );
        }
    }
}

#[test]
fn e10b_trap1_five_degrees_single_row_equals_radial_contact() {
    // 🔴 함정 1 — ISO 는 α = 5° 단열 칸에 숫자 대신 「radial contact 값을 쓰라」는
    //    문장을 넣었다. 그래서 단열 X·Y·e 는 α ∈ [0°, 5°] 에서 **상수**다.
    for rel in [0.172, 0.3, 0.689, 1.0, 1.38, 2.5, 3.45, 6.0, 6.89] {
        let at0 = x_y_e_low(0.0, rel);
        let at5 = x_y_e_low(5.0_f64.to_radians(), rel);
        let at2_5 = x_y_e_low(2.5_f64.to_radians(), rel);
        assert!((at0.0 - at5.0).abs() < 1e-12 && (at0.0 - at2_5.0).abs() < 1e-12, "X @ r={rel}");
        assert!((at0.1 - at5.1).abs() < 1e-12 && (at0.1 - at2_5.1).abs() < 1e-12, "Y @ r={rel}");
        assert!((at0.2 - at5.2).abs() < 1e-12 && (at0.2 - at2_5.2).abs() < 1e-12, "e @ r={rel}");
        // 그리고 5°→10° 에서 **처음** 변한다
        let at10 = x_y_e_low(10.0_f64.to_radians(), rel);
        assert!((at0.0 - at10.0).abs() > 1e-6, "5°→10° 에서 X 가 변하지 않았다");
    }
    // α = 5° 의 e 표(0,23…0,52)는 **복열 전용**이다 — 단열에 새어들면 안 된다.
    let (_, _, e5) = x_y_e_low(5.0_f64.to_radians(), XYE_ROW_GRID[0]);
    assert!((e5 - XYE_LOW_E_RADIAL[0]).abs() < 1e-12, "단열 5° e 는 0,19 여야 한다");
    assert!((e5 - 0.23).abs() > 1e-6, "복열 5° e(0,23)가 단열에 새어들었다");
}

#[test]
fn e10c_trap2_twenty_degrees_is_invariant_to_relative_axial_load() {
    // 🔴 함정 2 — 20° 이상은 상대 축하중 의존이 사라진다. 20° 를 9점 전부 같은
    //    값으로 채웠으므로 두 경로(상단부·하단부)가 20° 에서 정확히 만나야 한다.
    let (x0, y0, e0) = (XYE_TABLE_DEG[0].1, XYE_TABLE_DEG[0].2, XYE_TABLE_DEG[0].3);
    for rel in [0.01, 0.172, 1.0, 2.07, 6.89, 100.0] {
        let via_high = x_y_e(20.0_f64.to_radians(), Some(rel)).unwrap();
        let via_low = x_y_e_low(20.0_f64.to_radians(), rel);
        assert!((via_high.0 - x0).abs() < 1e-12 && (via_low.0 - x0).abs() < 1e-12, "X @ r={rel}");
        assert!((via_high.1 - y0).abs() < 1e-12 && (via_low.1 - y0).abs() < 1e-12, "Y @ r={rel}");
        assert!((via_high.2 - e0).abs() < 1e-12 && (via_low.2 - e0).abs() < 1e-12, "e @ r={rel}");
    }
    // 15°→20° 접합 — 17,5° 는 두 끝의 중점이어야 한다
    let a = x_y_e(17.5_f64.to_radians(), Some(1.38)).unwrap();
    let lo = x_y_e(15.0_f64.to_radians(), Some(1.38)).unwrap();
    assert!((a.0 - 0.5 * (lo.0 + x0)).abs() < 1e-12, "17,5° 는 15°·20° 의 중점이어야 한다");
    assert!((a.1 - 0.5 * (lo.1 + y0)).abs() < 1e-12);
    assert!((a.2 - 0.5 * (lo.2 + e0)).abs() < 1e-12);
}

#[test]
fn e10d_outside_the_grid_is_clamped_not_extrapolated() {
    // ISO 는 외삽을 허용하지 않는다 → 양 끝 값으로 클램프한다.
    for deg in [0.0_f64, 5.0, 10.0, 15.0, 12.3] {
        let a = deg.to_radians();
        let lo_out = x_y_e_low(a, 0.01);
        let lo_edge = x_y_e_low(a, XYE_ROW_GRID[0]);
        assert!((lo_out.0 - lo_edge.0).abs() < 1e-12);
        assert!((lo_out.1 - lo_edge.1).abs() < 1e-12);
        assert!((lo_out.2 - lo_edge.2).abs() < 1e-12);

        let hi_out = x_y_e_low(a, 100.0);
        let hi_edge = x_y_e_low(a, XYE_ROW_GRID[8]);
        assert!((hi_out.0 - hi_edge.0).abs() < 1e-12);
        assert!((hi_out.1 - hi_edge.1).abs() < 1e-12);
        assert!((hi_out.2 - hi_edge.2).abs() < 1e-12);
    }
    // F_a = 0 (r = 0) 도 하단 클램프로 흡수된다
    assert!((x_y_e_low(0.0, 0.0).1 - XYE_LOW_Y_RADIAL[0]).abs() < 1e-12);
}

#[test]
fn e10e_high_angle_results_are_unchanged_by_p5_1b() {
    // 회귀 — α = 20~45° 는 상대 축하중과 무관하게 종전 값 그대로여야 한다.
    for (deg, x, y, e) in XYE_TABLE_DEG {
        for rel in [None, Some(0.01), Some(1.03), Some(6.89), Some(500.0)] {
            let (gx, gy, ge) = x_y_e(deg.to_radians(), rel).unwrap();
            assert!((gx - x).abs() < 1e-12, "α={deg}° X");
            assert!((gy - y).abs() < 1e-12, "α={deg}° Y");
            assert!((ge - e).abs() < 1e-12, "α={deg}° e");
        }
    }
    // 중간각도 종전과 같다 (E-7 이 고정한 22,5° 값)
    let (x, y, e) = x_y_e(22.5_f64.to_radians(), Some(2.07)).unwrap();
    assert!((x - 0.42).abs() < 1e-12 && (y - 0.935).abs() < 1e-12 && (e - 0.625).abs() < 1e-12);
}

#[test]
fn e10f_column1_reproduces_column2_over_cos_alpha() {
    // Theory §7.3.1 Level E-9 (TR-21):  제1열 = 제2열 / cos α
    // 제2열 격자 자체도 psi 원값에서 재현되는지 함께 본다 (전사 자체검증).
    let mut mismatches = Vec::new();
    for (k, psi) in XYE_COL2_PSI.iter().enumerate() {
        let exact_mpa = psi * PSI_TO_MPA;
        let rounded = round_to_3_significant_digits(exact_mpa);
        if (rounded - XYE_ROW_GRID[k]).abs() > 1e-9 {
            mismatches.push(format!("제2열 #{k}: {rounded} ≠ {}", XYE_ROW_GRID[k]));
        }
        for (alpha_deg, col1) in XYE_COL1_TRANSCRIBED {
            let computed = round_to_3_significant_digits(exact_mpa / alpha_deg.to_radians().cos());
            if (computed - col1[k]).abs() > 1e-9 {
                mismatches.push(format!("α={alpha_deg}° 제1열 #{k}: {computed} ≠ {}", col1[k]));
            }
        }
    }
    println!(
        "\n── E-10f : (TR-21) 제1열 = 제2열/cos α — 검사 36건, 불일치 {}건",
        mismatches.len()
    );
    assert!(mismatches.is_empty(), "TR-21 재현 실패:\n  {}", mismatches.join("\n  "));
}

#[test]
fn e10g_double_row_transcription_is_intact() {
    // 현 SW 는 단열만 계산하지만 복열 전사값도 죽은 코드가 되지 않게 여기서 검증한다.
    assert_eq!(XYE_LOW_DOUBLE.len(), 4);
    for b in XYE_LOW_DOUBLE {
        assert!((0.0..=15.0).contains(&b.alpha_deg));
        for w in b.y.windows(2) {
            assert!(w[1] <= w[0] + 1e-12, "α={}° 복열 Y 가 증가했다", b.alpha_deg);
        }
        for w in b.e.windows(2) {
            assert!(w[1] >= w[0] - 1e-12, "α={}° 복열 e 가 감소했다", b.alpha_deg);
        }
    }
    // α = 0° 복열은 단열과 **완전히 동일**하다 (Theory §7.3.1)
    assert!((XYE_LOW_DOUBLE[0].x - XYE_LOW_SINGLE[0].x).abs() < 1e-12);
    assert_eq!(XYE_LOW_DOUBLE[0].y, XYE_LOW_SINGLE[0].y);
    assert_eq!(XYE_LOW_DOUBLE[0].e, XYE_LOW_SINGLE[0].e);
    // α = 10°/15° 의 e 는 단열·복열이 같고, α = 5° 만 갈린다 (함정 1 의 뒷면)
    assert_eq!(XYE_LOW_DOUBLE[2].e, XYE_LOW_SINGLE[2].e);
    assert_eq!(XYE_LOW_DOUBLE[3].e, XYE_LOW_SINGLE[3].e);
    assert_ne!(XYE_LOW_DOUBLE[1].e, XYE_LOW_SINGLE[1].e);
    // > e 구간 X 스팟 (전사 확인)
    assert!((XYE_LOW_DOUBLE[1].x - 0.78).abs() < 1e-12);
    assert!((XYE_LOW_DOUBLE[2].x - 0.75).abs() < 1e-12);
    assert!((XYE_LOW_DOUBLE[3].x - 0.72).abs() < 1e-12);
    // 복열은 ≤ e 구간에서도 Y ≠ 0 이다 (α = 0° 제외) — 단열과 다른 점
    assert!(XYE_LOW_DOUBLE[1].y_below.iter().all(|v| *v > 1.0));
}

#[test]
fn e10h_relative_axial_load_is_the_second_column_in_mpa() {
    // 제2열 = F_a/(Z D_w²) [N/mm² = MPa]. i 는 곱하지 않는다 (ACBB, Theory §7.3.1 ⚠).
    let z = 16;
    let d_w = 11.5;
    let f_a = 2_000.0;
    let expected = f_a / (f64::from(z) * d_w * d_w);
    assert!((relative_axial_load_mpa(f_a, z, d_w) - expected).abs() < 1e-12);
    // 방어 — Z = 0 또는 D_w = 0 이면 0 (하단 클램프로 흡수)
    assert_eq!(relative_axial_load_mpa(f_a, 0, d_w), 0.0);
    assert_eq!(relative_axial_load_mpa(f_a, z, 0.0), 0.0);
}

#[test]
fn e10i_p_r_is_now_available_below_twenty_degrees() {
    // P5-1b 의 목적 — `p_r_n` 이 α < 20° 에서도 Some 이 된다 (Plan §5-2.3 표).
    let mut input = life_input();
    input.geometry.alpha_nom_rad = 15.0_f64.to_radians();
    input.geometry.clearance = BbClearanceSpec::InitialAngleRad(15.0_f64.to_radians());
    let r = bb_core::commands::solve_life(&input, &BbLifeConditions::default()).unwrap();
    let p = r.p_r_n.expect("α = 15° 에서도 P_r 이 나와야 한다 (P5-1b)");
    assert!(p.is_finite() && p > 0.0, "P_r = {p}");
    println!("\n── E-10i : α = 15° 에서 P_r = {p:.1} N");
    // α ≥ 20° 경로도 여전히 살아 있다
    let r40 = bb_core::commands::solve_life(&life_input(), &BbLifeConditions::default()).unwrap();
    assert!(r40.p_r_n.is_some());
}
