// BB Contact Analysis — 점접촉 타원 Hertz
//
// BB Phase 2 (2026-08-20): CRB 선접촉 Hertz + Weber bulk 를 백지에서 교체.
//
// ── 근거 ────────────────────────────────────────────────────────────
// BB_Development_Theory.md §3, §6.
//   (E.1) χ 결정식,  (E.2)(E.3) 완전타원적분
//   (36)(37) 단일 접촉 변형,  (38) 총 변형,  (39) Q = c_P δ^1.5,  (40) c_P
//   Harris (6.38)(6.40) 접촉타원 반경,  (6.44)~(6.46) a*·b*·δ*,  (6.25) p_max
//
// ── ISO 와 Harris 의 관계 (P2 선조사에서 확인) ──────────────────────
// ISO (E.1) 과 Harris (6.30) 은 **같은 방정식**이다:
//     F(ρ) = [(χ²+1)E − 2K] / [(χ²−1)E]
// Harris 는 이를 **정방향**(χ 가정 → F(ρ) 계산)으로 써서 Table 6.1 을 만들었고,
// ISO 는 **역방향**(F(ρ) 주어짐 → χ 근찾기)으로 쓴다.
// 본 구현은 ISO 방향이 본선이고, Brewe-Hamrock 근사는 초기 구간·검산용이다 (P2 결정).
//
// ── 단위 (D-10) ─────────────────────────────────────────────────────
// mm · N · rad · MPa. 이 파일에 단위 환산 상수는 없다.

use crate::error::SolverError;
use crate::solver::bb::types::*;
use crate::solver::common::types::Material;
use crate::solver::common::util;

// ─── [P1 이관] rib_contact.rs 에서 이관 (2026-08-20) ───────────────────
// ACBB 점접촉에서 χ = a/b 비선형 방정식(ISO 16281 식 E.1)의 **초기 추정값** 및
// 검산용. 최종 χ 는 P2 에서 (E.1) 을 직접 풀어 확정한다.
// 계수 1.0339/0.6360, 1.5277/0.6023, 1.0003/0.5968 는 Harris & Kotzalas 5th ed.
// 식 (6.33)~(6.35) (Brewe-Hamrock 회귀) 와 일치함을 원서 대조로 확인.
// 주의: 반환되는 F_e, E_e 는 완전타원적분의 **회귀 근사**이지 정확값이 아니다.

/// Hamrock-Brewe approximation for elliptical Hertz coefficients.
/// r_x, r_y: equivalent radii [mm] (r_y / r_x >= 1)
/// Returns (k_e, F_e, E_e):
///   k_e: ellipticity ratio a/b
///   F_e: complete elliptic integral of the first kind (approx)
///   E_e: complete elliptic integral of the second kind (approx)
pub fn hertz_elliptical_coefficients(r_x: f64, r_y: f64) -> (f64, f64, f64) {
    // Ensure ratio >= 1
    let (rx, ry) = if r_y >= r_x { (r_x, r_y) } else { (r_y, r_x) };
    let ratio = ry / rx;

    let k_e = 1.0339 * ratio.powf(0.6360);
    let f_e = 1.5277 + 0.6023 * ratio.ln();
    let e_e = 1.0003 + 0.5968 / ratio;

    (k_e, f_e, e_e)
}

// ═══════════════════════════════════════════════════════════════════
//  χ ↔ F(ρ)
// ═══════════════════════════════════════════════════════════════════

/// 접촉타원 형상비 `χ` 로부터 상대 곡률차 `F(ρ)` 를 계산한다 — **정방향**.
///
/// ISO (E.1) 을 정리하면
/// ```text
///   F(ρ) = 1 − 2(K − E) / [(χ² − 1) E]
///        = 1 − 2 · [(K − E)/m] · (1 − m) / E ,   m = 1 − 1/χ²
/// ```
/// 두 번째 형태를 쓰는 이유: `χ → 1` 에서 `K − E → 0` 이고 `χ² − 1 → 0` 이라
/// 그대로 계산하면 0/0 이 된다. `(K − E)/m` 는 `m → 0` 에서 `π/4` 로 수렴하는
/// 유한값이며, 분자의 자릿수 소실은 `util::elliptic_k_minus_e` 의
/// **급수전개 분기**가 막는다 (P2 결정).
///
/// `F(ρ)` 는 `χ = 1` 에서 0, `χ → ∞` 에서 1 로 **단조 증가**한다.
pub fn f_rho_from_chi(chi: f64) -> f64 {
    if chi <= 1.0 {
        return 0.0;
    }
    let m = 1.0 - 1.0 / (chi * chi);
    let (_, e) = util::elliptic_k_e_agm(m);
    let k_minus_e_over_m = util::elliptic_k_minus_e(m) / m;
    1.0 - 2.0 * k_minus_e_over_m * (1.0 - m) / e
}

/// `F(ρ)` 로부터 `χ` 를 구한다 — ISO (E.1) 역방향.
///
/// 구간보장 이분법에 **Illinois 가속**(수정 regula falsi)을 얹었다.
/// 도함수가 필요 없고 브래킷을 절대 잃지 않으며 초선형 수렴한다.
/// 초기 상한은 Brewe-Hamrock 근사에서 잡는다.
///
/// `χ` 는 하중과 무관하므로 해석당 1회만 호출된다 — 속도는 무관하다.
pub fn solve_chi(f_rho: f64) -> Result<f64, SolverError> {
    if !(0.0..1.0).contains(&f_rho) {
        return Err(SolverError::InvalidGeometry(format!(
            "상대 곡률차 F(ρ) = {f_rho} 가 [0, 1) 밖입니다 — 기하 입력을 확인하십시오"
        )));
    }
    // F(ρ) = 0 ⟺ χ = 1 (원형 접촉). Table 6.1 첫 행.
    if f_rho < 1.0e-14 {
        return Ok(1.0);
    }

    let g = |chi: f64| f_rho_from_chi(chi) - f_rho;

    // 브래킷: [1+ε, hi]. hi 는 g(hi) > 0 이 될 때까지 확장.
    let lo0 = 1.0 + 1.0e-12;
    let mut hi = 2.0_f64;
    let mut ghi = g(hi);
    for _ in 0..80 {
        if ghi > 0.0 {
            break;
        }
        hi *= 2.0;
        ghi = g(hi);
    }
    if ghi <= 0.0 {
        return Err(SolverError::ConvergenceFailure(format!(
            "F(ρ) = {f_rho} 에 대한 χ 브래킷 확보 실패 (χ = {hi} 까지 확장)"
        )));
    }

    let mut a = lo0;
    let mut ga = g(a);
    let mut b = hi;
    let mut gb = ghi;

    for _ in 0..200 {
        // Illinois: regula falsi + 정체 측 가중치 절반
        let x = b - gb * (b - a) / (gb - ga);
        let gx = g(x);
        if gx.abs() < 1.0e-15 || (b - a).abs() < 1.0e-14 * x.max(1.0) {
            return Ok(x);
        }
        if gx.signum() == gb.signum() {
            b = x;
            gb = gx;
            ga *= 0.5;
        } else {
            a = x;
            ga = gx;
            gb *= 0.5;
        }
    }
    Err(SolverError::ConvergenceFailure(format!(
        "F(ρ) = {f_rho} 에 대한 χ 수렴 실패"
    )))
}

// ═══════════════════════════════════════════════════════════════════
//  무차원 접촉 계수 (Harris 6.44~6.46)
// ═══════════════════════════════════════════════════════════════════

/// `(a*, b*, δ*)` — Harris 식 (6.44)(6.45)(6.46).
///
/// ```text
///   a* = (2 χ² E / π)^(1/3)
///   b* = (2 E / (π χ))^(1/3)
///   δ* = (2K/π) · (π / (2 χ² E))^(1/3)
/// ```
/// `χ = 1` 에서 `E = K = π/2` 이므로 셋 다 정확히 1 이 된다 (Table 6.1 첫 행).
pub fn dimensionless_contact_coefficients(chi: f64, k_ellip: f64, e_ellip: f64) -> (f64, f64, f64) {
    let pi = std::f64::consts::PI;
    let a_star = (2.0 * chi * chi * e_ellip / pi).cbrt();
    let b_star = (2.0 * e_ellip / (pi * chi)).cbrt();
    let delta_star = (2.0 * k_ellip / pi) * (pi / (2.0 * chi * chi * e_ellip)).cbrt();
    (a_star, b_star, delta_star)
}

// ═══════════════════════════════════════════════════════════════════
//  전처리 — 하중 무관
// ═══════════════════════════════════════════════════════════════════

/// 점접촉 전처리 (Theory §8 2~3단계). 해석당 1회.
pub fn compute_contact_derived(
    derived: &BbGeometryDerived,
    material: &Material,
) -> Result<BbContactDerived, SolverError> {
    material.validate()?;

    let chi_inner = solve_chi(derived.f_rho_i)?;
    let chi_outer = solve_chi(derived.f_rho_e)?;

    let (k_i, e_i) = util::elliptic_k_e_agm(1.0 - 1.0 / (chi_inner * chi_inner));
    let (k_e, e_e) = util::elliptic_k_e_agm(1.0 - 1.0 / (chi_outer * chi_outer));

    let (a_star_i, b_star_i, d_star_i) = dimensionless_contact_coefficients(chi_inner, k_i, e_i);
    let (a_star_e, b_star_e, d_star_e) = dimensionless_contact_coefficients(chi_outer, k_e, e_e);

    let e_star_mpa = util::combined_elastic_modulus_mpa(
        material.e_ball_mpa,
        material.nu,
        material.e_ring_mpa,
        material.nu,
    );

    let c_p_n_per_mm15 = spring_constant_c_p(derived, material, chi_inner, k_i, e_i, chi_outer, k_e, e_e);

    Ok(BbContactDerived {
        chi_inner,
        chi_outer,
        k_ellip_inner: k_i,
        e_ellip_inner: e_i,
        k_ellip_outer: k_e,
        e_ellip_outer: e_e,
        a_star_inner: a_star_i,
        b_star_inner: b_star_i,
        delta_star_inner: d_star_i,
        a_star_outer: a_star_e,
        b_star_outer: b_star_e,
        delta_star_outer: d_star_e,
        e_star_mpa,
        c_p_n_per_mm15,
    })
}

/// ISO (38) 의 대괄호 항 — 내·외륜 기여의 합. 단위 [mm^(1/3)·… ] 무차원 아님.
fn deflection_bracket(
    derived: &BbGeometryDerived,
    chi_i: f64,
    k_i: f64,
    e_i: f64,
    chi_e: f64,
    k_e: f64,
    e_e: f64,
) -> f64 {
    k_i * (derived.sum_rho_i_per_mm / (chi_i * chi_i * e_i)).cbrt()
        + k_e * (derived.sum_rho_e_per_mm / (chi_e * chi_e * e_e)).cbrt()
}

/// 점접촉 스프링상수 `c_P` [N/mm^(3/2)] — ISO 식 (40).
///
/// ```text
///   c_P = 1,48 · E_st/(1 − ν²) · [ K(χ_i)·∛(Σρ_i/(χ_i² E_i))
///                                + K(χ_e)·∛(Σρ_e/(χ_e² E_e)) ]^(−3/2)
/// ```
/// 하이브리드(Si₃N₄) 식 (41) 은 범위 밖이다 (Plan D-5).
#[allow(clippy::too_many_arguments)]
pub fn spring_constant_c_p(
    derived: &BbGeometryDerived,
    material: &Material,
    chi_i: f64,
    k_i: f64,
    e_i: f64,
    chi_e: f64,
    k_e: f64,
    e_e: f64,
) -> f64 {
    let bracket = deflection_bracket(derived, chi_i, k_i, e_i, chi_e, k_e, e_e);
    1.48 * material.e_ring_mpa / (1.0 - material.nu * material.nu) * bracket.powf(-1.5)
}

// ═══════════════════════════════════════════════════════════════════
//  하중 ↔ 변형
// ═══════════════════════════════════════════════════════════════════

/// `Q = c_P δ^(3/2)` — ISO 식 (39). `δ` 는 양 접촉의 **총** 변형 [mm].
pub fn q_from_delta(c_p: f64, delta_mm: f64) -> f64 {
    if delta_mm <= 0.0 {
        0.0
    } else {
        c_p * delta_mm.powf(1.5)
    }
}

/// 식 (39) 의 역: `δ = (Q / c_P)^(2/3)` [mm].
pub fn delta_from_q(c_p: f64, q_n: f64) -> f64 {
    if q_n <= 0.0 {
        0.0
    } else {
        (q_n / c_p).powf(2.0 / 3.0)
    }
}

/// 단일 접촉의 탄성변형 [mm] — ISO 식 (36)/(37).
///
/// ```text
///   δ = ∛(4,5 ((1−ν²)/(πE))²) · K(χ) · ∛(Σρ/(χ² E(χ))) · Q^(2/3)
/// ```
pub fn single_contact_deflection_iso(
    material: &Material,
    sum_rho_per_mm: f64,
    chi: f64,
    k_ellip: f64,
    e_ellip: f64,
    q_n: f64,
) -> f64 {
    if q_n <= 0.0 {
        return 0.0;
    }
    let pi = std::f64::consts::PI;
    let compliance = (1.0 - material.nu * material.nu) / (pi * material.e_ring_mpa);
    let lead = (4.5 * compliance * compliance).cbrt();
    lead * k_ellip * (sum_rho_per_mm / (chi * chi * e_ellip)).cbrt() * q_n.powf(2.0 / 3.0)
}

/// 단일 접촉의 탄성변형 [mm] — **Harris 식 (6.42)** (교차검증용).
///
/// ```text
///   δ = δ* · [3Q / (2 Σρ E*)]^(2/3) · Σρ/2
/// ```
/// ISO (36) 과 대수적으로 동일한 식이다 (P2 선조사에서 전개 확인).
/// 따라서 이 대조는 **물리 모델 교차가 아니라 전사·구현 검증**이다.
pub fn single_contact_deflection_harris(
    e_star_mpa: f64,
    sum_rho_per_mm: f64,
    delta_star: f64,
    q_n: f64,
) -> f64 {
    if q_n <= 0.0 {
        return 0.0;
    }
    let bracket = 3.0 * q_n / (2.0 * sum_rho_per_mm * e_star_mpa);
    delta_star * bracket.powf(2.0 / 3.0) * sum_rho_per_mm / 2.0
}

// ═══════════════════════════════════════════════════════════════════
//  접촉타원과 응력
// ═══════════════════════════════════════════════════════════════════

/// 접촉타원 반경과 최대 접촉응력.
///
/// Harris (6.38)(6.40): `a = a*·[3Q/(2Σρ E*)]^(1/3)`, `b = b*·[…]^(1/3)`
/// Harris (6.25): `p_max = 3Q / (2π a b)`
///
/// 반환: `(a [mm], b [mm], p_max [MPa])`
pub fn contact_ellipse(
    e_star_mpa: f64,
    sum_rho_per_mm: f64,
    a_star: f64,
    b_star: f64,
    q_n: f64,
) -> (f64, f64, f64) {
    if q_n <= 0.0 {
        return (0.0, 0.0, 0.0);
    }
    let bracket = (3.0 * q_n / (2.0 * sum_rho_per_mm * e_star_mpa)).cbrt();
    let a = a_star * bracket;
    let b = b_star * bracket;
    let p_max = 3.0 * q_n / (2.0 * std::f64::consts::PI * a * b);
    (a, b, p_max)
}

/// ISO 281 Annex B.3.1 이 권장하는 피로한계 접촉응력 [MPa].
pub const SIGMA_HU_MPA: f64 = 1500.0;

#[cfg(test)]
mod tests {
    use super::*;

    fn geom_fixture() -> BallBearingGeometry {
        let d_w_mm = 11.5;
        let (r_i_mm, r_e_mm) = BallBearingGeometry::reference_groove_radii(d_w_mm);
        BallBearingGeometry {
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
        }
    }

    #[test]
    fn f_rho_is_zero_at_unit_chi() {
        assert!(f_rho_from_chi(1.0).abs() < 1e-15);
    }

    #[test]
    fn f_rho_is_monotonic_and_bounded() {
        let mut prev = -1.0;
        for chi in [1.0, 1.001, 1.1, 1.5, 2.0, 5.0, 10.0, 40.0, 200.0] {
            let f = f_rho_from_chi(chi);
            assert!(f > prev, "χ={chi} 에서 단조성 위반: {f}");
            assert!((0.0..1.0).contains(&f), "χ={chi} 에서 범위 위반: {f}");
            prev = f;
        }
    }

    #[test]
    fn chi_solver_roundtrip() {
        // χ → F(ρ) → χ 왕복 항등
        for chi in [1.0001, 1.05, 1.5, 2.0, 3.0, 6.0, 12.0, 30.0] {
            let f = f_rho_from_chi(chi);
            let back = solve_chi(f).unwrap();
            assert!(
                ((back - chi) / chi).abs() < 1e-9,
                "χ={chi} → F(ρ)={f} → χ={back}"
            );
        }
    }

    #[test]
    fn chi_solver_handles_zero_f_rho() {
        assert!((solve_chi(0.0).unwrap() - 1.0).abs() < 1e-15);
    }

    #[test]
    fn chi_solver_rejects_out_of_range() {
        assert!(solve_chi(-0.1).is_err());
        assert!(solve_chi(1.0).is_err());
    }

    #[test]
    fn dimensionless_coefficients_unity_at_circular_contact() {
        // χ = 1 → K = E = π/2 → a* = b* = δ* = 1 (Harris Table 6.1 첫 행)
        let half_pi = std::f64::consts::FRAC_PI_2;
        let (a, b, d) = dimensionless_contact_coefficients(1.0, half_pi, half_pi);
        assert!((a - 1.0).abs() < 1e-15, "a*={a}");
        assert!((b - 1.0).abs() < 1e-15, "b*={b}");
        assert!((d - 1.0).abs() < 1e-15, "δ*={d}");
    }

    #[test]
    fn contact_derived_is_self_consistent() {
        let g = geom_fixture();
        let d = crate::solver::bb::geometry::compute_geometry_derived(&g).unwrap();
        let m = Material::default();
        let c = compute_contact_derived(&d, &m).unwrap();

        // 내륜 곡률차가 더 크므로 타원이 더 길쭉하다
        assert!(c.chi_inner > c.chi_outer, "χ_i={} χ_e={}", c.chi_inner, c.chi_outer);
        assert!(c.chi_inner > 1.0 && c.chi_outer > 1.0);
        assert!(c.c_p_n_per_mm15 > 0.0);
        // 강-강 E* = E/(2(1−ν²)) = 207000/(2·0.91)
        let expected_e_star = 207_000.0 / (2.0 * (1.0 - 0.09));
        assert!(((c.e_star_mpa - expected_e_star) / expected_e_star).abs() < 1e-12);
    }

    #[test]
    fn load_deflection_roundtrip() {
        let g = geom_fixture();
        let d = crate::solver::bb::geometry::compute_geometry_derived(&g).unwrap();
        let c = compute_contact_derived(&d, &Material::default()).unwrap();
        for q in [1.0, 100.0, 1_000.0, 10_000.0] {
            let delta = delta_from_q(c.c_p_n_per_mm15, q);
            let back = q_from_delta(c.c_p_n_per_mm15, delta);
            assert!(((back - q) / q).abs() < 1e-12, "Q={q} → δ={delta} → Q={back}");
        }
    }

    #[test]
    fn no_contact_returns_zero() {
        assert_eq!(q_from_delta(1000.0, 0.0), 0.0);
        assert_eq!(delta_from_q(1000.0, -5.0), 0.0);
        let (a, b, p) = contact_ellipse(100_000.0, 0.2, 1.5, 0.7, 0.0);
        assert_eq!((a, b, p), (0.0, 0.0, 0.0));
    }

    #[test]
    fn ellipse_aspect_ratio_equals_chi() {
        // a/b = a*/b* = χ 여야 한다
        let g = geom_fixture();
        let d = crate::solver::bb::geometry::compute_geometry_derived(&g).unwrap();
        let c = compute_contact_derived(&d, &Material::default()).unwrap();
        let (a, b, _) = contact_ellipse(
            c.e_star_mpa,
            d.sum_rho_i_per_mm,
            c.a_star_inner,
            c.b_star_inner,
            5_000.0,
        );
        assert!(
            ((a / b - c.chi_inner) / c.chi_inner).abs() < 1e-12,
            "a/b={} χ={}",
            a / b,
            c.chi_inner
        );
    }

    // Brewe-Hamrock 근사 오차 검증은 Level B (B-7) 에서 정확한 타원적분과
    // 대조한다. 여기서 회귀식끼리 비교하면 순환이 된다.
}
