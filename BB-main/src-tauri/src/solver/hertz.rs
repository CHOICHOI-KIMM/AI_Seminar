use std::f64::consts::PI;

use crate::solver::types::SliceContactResult;

/// Combined elastic modulus E* for two bodies in contact.
/// E1, E2 in [GPa], nu1, nu2 dimensionless.
/// Returns E* in [GPa].
pub fn combined_elastic_modulus(e1: f64, nu1: f64, e2: f64, nu2: f64) -> f64 {
    let inv = (1.0 - nu1 * nu1) / e1 + (1.0 - nu2 * nu2) / e2;
    1.0 / inv
}

/// Hertz contact half-width for line contact.
/// q: load per unit length [N/mm]
/// r_eq: equivalent radius [mm]
/// e_star: combined elastic modulus [MPa] (note: convert GPa→MPa before calling)
/// Returns b [mm].
pub fn hertz_half_width(q: f64, r_eq: f64, e_star: f64) -> f64 {
    // b = sqrt(4 * q * r_eq / (pi * e_star))
    // Derivation: b² = 4qR/(πE*) for line contact (Hertz theory)
    if q <= 0.0 || r_eq <= 0.0 || e_star <= 0.0 {
        return 0.0;
    }
    (4.0 * q * r_eq / (PI * e_star)).sqrt()
}

/// Maximum Hertz contact pressure for line contact.
/// q: load per unit length [N/mm]
/// b: contact half-width [mm]
/// Returns p_max [MPa].
pub fn hertz_max_pressure(q: f64, b: f64) -> f64 {
    if b <= 0.0 {
        return 0.0;
    }
    2.0 * q / (PI * b)
}

/// Weber bulk (sub-surface) deformation.
/// q: load per unit length [N/mm]
/// e: Young's modulus [MPa]
/// nu: Poisson's ratio
/// b: contact half-width [mm]
/// h1: depth to first surface (roller half-height at slice) [mm]
/// h2: depth to second surface (raceway thickness at slice) [mm]
/// Returns h_bulk [μm].
pub fn weber_bulk_deformation(q: f64, e: f64, nu: f64, b: f64, h1: f64, h2: f64) -> f64 {
    if q <= 0.0 || b <= 0.0 || e <= 0.0 || h1 <= 0.0 || h2 <= 0.0 {
        return 0.0;
    }
    // Weber formula: δ_bulk = (4q(1-ν²))/(πlE) * [ln(2√(h1·h2)/b) - ν/(2(1-ν))]
    // Here q is per unit length, so the l factor is already accounted for.
    let geometric_mean = (h1 * h2).sqrt();
    let log_term = (2.0 * geometric_mean / b).ln();
    let poisson_term = nu / (2.0 * (1.0 - nu));

    let delta_mm = 4.0 * q * (1.0 - nu * nu) / (PI * e) * (log_term - poisson_term);
    delta_mm * 1000.0 // convert mm to μm
}

/// Compute complete contact result for a single slice (dual-raceway model).
///
/// delta_k [μm]: available approach along the outer raceway contact normal,
///   computed as: δ_rigid − Δz_outer − Δz_inner·cos(α_o−α_i)
///
/// cos_alpha_diff: cos(α_o − α_i), projection factor from inner raceway
///   normal to outer raceway normal.  When 0.0, falls back to outer-only
///   model (legacy behavior).
///
/// q_k is determined by solving the dual-raceway Hertz equation:
///   δ_hertz(q, R_outer) + δ_hertz(q, R_inner)·cos(α_o−α_i) = delta_k
///
/// This accounts for deformation at BOTH raceways and the geometric
/// projection between inner/outer contact normals.
pub fn compute_slice_contact(
    k: usize,
    delta_k: f64,
    r_eq_inner: f64,
    r_eq_outer: f64,
    e_star: f64,
    e: f64,
    nu: f64,
    _slice_width: f64,
    h1: f64,
    h2: f64,
    cos_alpha_diff: f64,
) -> SliceContactResult {
    if delta_k <= 0.0 {
        return SliceContactResult {
            k,
            delta_k,
            q_k: 0.0,
            q_k_outer: 0.0,
            q_k_inner: 0.0,
            b_k: 0.0,
            p_max_k: 0.0,
            h_bulk_k: 0.0,
            k_hertz_k: 0.0,
            b_k_outer: 0.0,
            p_max_k_outer: 0.0,
            h_bulk_k_outer: 0.0,
            k_hertz_k_outer: 0.0,
            k_combined_k: 0.0,
            in_contact: false,
        };
    }

    let delta_mm = delta_k / 1000.0; // μm to mm

    // Solve q from dual-raceway approach:
    //   δ_hertz_outer(q) + δ_hertz_inner(q)·cos(α_o−α_i) = delta
    // When cos_alpha_diff == 0.0, reduces to outer-only (legacy).
    let q_k = solve_q_from_dual_delta(delta_mm, r_eq_inner, r_eq_outer, e_star, cos_alpha_diff);

    // Outer raceway results
    let b_k_outer = hertz_half_width(q_k, r_eq_outer, e_star);
    let p_max_k_outer = hertz_max_pressure(q_k, b_k_outer);
    let h_bulk_k_outer = weber_bulk_deformation(q_k, e, nu, b_k_outer, h1, h2);
    let dq = 0.001 * q_k.max(1.0);
    let k_hertz_k_outer = tangent_stiffness(q_k, dq, r_eq_outer, e_star);

    // Inner raceway results: inner normal load = q_k · cos(α_o−α_i)
    // When cos_alpha_diff == 0 (legacy outer-only), use q_k as-is
    let q_k_inner = if cos_alpha_diff.abs() < 1e-12 { q_k } else { q_k * cos_alpha_diff };
    let b_k = hertz_half_width(q_k_inner, r_eq_inner, e_star);
    let p_max_k = hertz_max_pressure(q_k_inner, b_k);
    let h_bulk_k = weber_bulk_deformation(q_k_inner, e, nu, b_k, h1, h2);
    let dq_inner = 0.001 * q_k_inner.max(1.0);
    let k_hertz_k = tangent_stiffness(q_k_inner, dq_inner, r_eq_inner, e_star);

    // Combined slice stiffness: Hertz mutual-approach along outer normal.
    // δ_total = δ_hertz_outer(q) + δ_hertz_inner(q·cos)·cos
    //         = q/k_hertz_outer + q·cos²/k_hertz_inner
    // ⇒ 1/k_combined = 1/k_hertz_outer + cos²/k_hertz_inner
    // Weber bulk is NOT a separate spring — it is already embedded in
    // Hertz mutual approach (both formulas share the 2/E* = 4(1−ν²)/E
    // prefactor, differing only in closure: R vs √(h1·h2)).
    let cos_sq = if cos_alpha_diff.abs() < 1e-12 { 0.0 } else { cos_alpha_diff * cos_alpha_diff };
    let mut inv_k = 0.0;
    if k_hertz_k_outer > 0.0 { inv_k += 1.0 / k_hertz_k_outer; }
    if k_hertz_k > 0.0 && cos_sq > 0.0 { inv_k += cos_sq / k_hertz_k; }
    let k_combined_k = if inv_k > 0.0 { 1.0 / inv_k } else { 0.0 };

    SliceContactResult {
        k,
        delta_k,
        q_k,
        q_k_outer: q_k,
        q_k_inner,
        b_k,
        p_max_k,
        h_bulk_k,
        k_hertz_k,
        b_k_outer,
        p_max_k_outer,
        h_bulk_k_outer,
        k_hertz_k_outer,
        k_combined_k,
        in_contact: true,
    }
}

/// Compute tangent stiffness dq/dδ [N/mm per μm] for a single raceway.
fn tangent_stiffness(q: f64, dq: f64, r_eq: f64, e_star: f64) -> f64 {
    let delta_plus = hertz_approach(q + dq, r_eq, e_star);
    let delta_curr = hertz_approach(q, r_eq, e_star);
    if (delta_plus - delta_curr).abs() > 1e-15 {
        dq / ((delta_plus - delta_curr) * 1000.0)
    } else {
        0.0
    }
}

/// Compute contact half-width and max pressure for a known load q_k.
/// Used to get outer raceway stress from inner-raceway-determined q_k.
/// Returns (b, p_max, h_bulk, k_hertz).
#[allow(dead_code)]
pub fn contact_stress_from_q(
    q_k: f64,
    r_eq: f64,
    e_star: f64,
    e: f64,
    nu: f64,
    h1: f64,
    h2: f64,
) -> (f64, f64, f64, f64) {
    if q_k <= 0.0 {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let b = hertz_half_width(q_k, r_eq, e_star);
    let p_max = hertz_max_pressure(q_k, b);
    let h_bulk = weber_bulk_deformation(q_k, e, nu, b, h1, h2);

    // Tangent stiffness: dq/dδ (numerical)
    let dq = 0.001 * q_k.max(1.0);
    let delta_plus = hertz_approach(q_k + dq, r_eq, e_star);
    let delta_curr = hertz_approach(q_k, r_eq, e_star);
    let k_hertz = if (delta_plus - delta_curr).abs() > 1e-15 {
        dq / ((delta_plus - delta_curr) * 1000.0) // [N/mm per μm]
    } else {
        0.0
    };

    (b, p_max, h_bulk, k_hertz)
}

/// Hertz approach (elastic deformation) for line contact.
/// q: load per unit length [N/mm]
/// r_eq: equivalent radius [mm]
/// e_star: combined elastic modulus [MPa]
/// Returns approach in [mm].
pub fn hertz_approach(q: f64, r_eq: f64, e_star: f64) -> f64 {
    if q <= 0.0 {
        return 0.0;
    }
    let b = hertz_half_width(q, r_eq, e_star);
    if b <= 0.0 {
        return 0.0;
    }
    // δ = (2q)/(π*E*) * [ln(4R_eq/b) - 0.5]
    (2.0 * q / (PI * e_star)) * ((4.0 * r_eq / b).ln() - 0.5)
}

/// Solve for q given dual-raceway combined approach using Newton-Raphson.
///
/// Solves: δ_hertz(q, R_outer) + δ_hertz(q, R_inner)·cos_alpha_diff = delta
///
/// When cos_alpha_diff == 0.0, reduces to outer-only (legacy behavior).
///
/// delta: target combined approach [mm]
/// r_eq_inner, r_eq_outer: equivalent radii [mm]
/// e_star: combined elastic modulus [MPa]
/// cos_alpha_diff: cos(α_o − α_i)
/// Returns q [N/mm].
pub fn solve_q_from_dual_delta(
    delta: f64,
    r_eq_inner: f64,
    r_eq_outer: f64,
    e_star: f64,
    cos_alpha_diff: f64,
) -> f64 {
    if delta <= 0.0 {
        return 0.0;
    }

    // If cos_alpha_diff is zero, outer-only (legacy)
    if cos_alpha_diff.abs() < 1e-12 {
        return solve_q_from_delta(delta, r_eq_outer, e_star);
    }

    // q is the outer-normal line load; inner sees q_inner = q · cos(α_o−α_i)
    let dual_approach = |q: f64| -> f64 {
        let q_inner = q * cos_alpha_diff;
        hertz_approach(q, r_eq_outer, e_star)
            + hertz_approach(q_inner, r_eq_inner, e_star) * cos_alpha_diff
    };

    // Initial guess (same heuristic as single, slightly reduced for dual)
    let mut q = e_star * r_eq_outer.sqrt() * delta.powf(10.0 / 9.0) * 0.3;
    if q <= 0.0 {
        q = 1.0;
    }

    for _ in 0..50 {
        let f = dual_approach(q) - delta;
        if f.abs() < 1e-12 {
            break;
        }

        let dq = q * 1e-6 + 1e-10;
        let f_plus = dual_approach(q + dq) - delta;
        let df = (f_plus - f) / dq;

        if df.abs() < 1e-20 {
            break;
        }

        let q_new = q - f / df;
        if q_new <= 0.0 {
            q *= 0.5;
        } else {
            q = q_new;
        }
    }

    q.max(0.0)
}

/// Solve for q given target approach delta using Newton-Raphson (single raceway).
/// delta: target approach [mm]
/// r_eq: equivalent radius [mm]
/// e_star: combined elastic modulus [MPa]
/// Returns q [N/mm].
pub fn solve_q_from_delta(delta: f64, r_eq: f64, e_star: f64) -> f64 {
    if delta <= 0.0 {
        return 0.0;
    }

    let mut q = e_star * r_eq.sqrt() * delta.powf(10.0 / 9.0) * 0.5;
    if q <= 0.0 {
        q = 1.0;
    }

    for _ in 0..50 {
        let f = hertz_approach(q, r_eq, e_star) - delta;
        if f.abs() < 1e-12 {
            break;
        }

        let dq = q * 1e-6 + 1e-10;
        let f_plus = hertz_approach(q + dq, r_eq, e_star) - delta;
        let df = (f_plus - f) / dq;

        if df.abs() < 1e-20 {
            break;
        }

        let q_new = q - f / df;
        if q_new <= 0.0 {
            q *= 0.5;
        } else {
            q = q_new;
        }
    }

    q.max(0.0)
}

/// Compute contact results for a SINGLE raceway given approach delta [μm].
/// Returns (q, b, p_max, h_bulk, k_hertz, k_bulk).
/// Returns all zeros if delta <= 0 (no contact).
pub fn single_raceway_contact(
    delta_um: f64,
    r_eq: f64,
    e_star: f64,
    e: f64,
    nu: f64,
    h1: f64,
    h2: f64,
) -> (f64, f64, f64, f64, f64) {
    if delta_um <= 0.0 {
        return (0.0, 0.0, 0.0, 0.0, 0.0);
    }
    let delta_mm = delta_um / 1000.0;
    let q = solve_q_from_delta(delta_mm, r_eq, e_star);
    let b = hertz_half_width(q, r_eq, e_star);
    let p_max = hertz_max_pressure(q, b);
    let h_bulk = weber_bulk_deformation(q, e, nu, b, h1, h2);
    let dq = 0.001 * q.max(1.0);
    let k_hertz = tangent_stiffness(q, dq, r_eq, e_star);
    (q, b, p_max, h_bulk, k_hertz)
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combined_elastic_modulus_steel() {
        // Two steel bodies: E=210 GPa, ν=0.3
        let e_star = combined_elastic_modulus(210.0, 0.3, 210.0, 0.3);
        // E* = 1 / (2*(1-0.09)/210) = 210 / (2*0.91) = 115.38 GPa
        let expected = 210.0 / (2.0 * 0.91);
        assert!((e_star - expected).abs() / expected < 1e-6);
    }

    #[test]
    fn test_combined_elastic_modulus_dissimilar() {
        let e_star = combined_elastic_modulus(210.0, 0.3, 70.0, 0.33);
        assert!(e_star > 0.0);
        assert!(e_star < 210.0);
    }

    #[test]
    fn test_hertz_half_width_standard_case() {
        // Case 1: Standard steel, R_eq=5mm, q=500 N/mm
        let e_star_gpa = combined_elastic_modulus(210.0, 0.3, 210.0, 0.3);
        let e_star_mpa = e_star_gpa * 1000.0;
        let r_eq = 5.0;
        let q = 500.0;

        let b = hertz_half_width(q, r_eq, e_star_mpa);
        // b = sqrt(4 * 500 * 5 / (π * 115384.6)) ≈ sqrt(10000/362377) ≈ sqrt(0.0276) ≈ 0.166 mm
        assert!((b - 0.166).abs() < 0.01, "b = {b}, expected ~0.166");
    }

    #[test]
    fn test_hertz_max_pressure_standard_case() {
        // Case 1: q=500 N/mm, b≈0.166 mm
        let e_star_gpa = combined_elastic_modulus(210.0, 0.3, 210.0, 0.3);
        let e_star_mpa = e_star_gpa * 1000.0;
        let q = 500.0;
        let b = hertz_half_width(q, 5.0, e_star_mpa);
        let p_max = hertz_max_pressure(q, b);

        // p_max = 2*500 / (π*0.166) ≈ 1916 MPa
        assert!(
            (p_max - 1916.0).abs() < 50.0,
            "p_max = {p_max}, expected ~1916"
        );
    }

    #[test]
    fn test_no_contact() {
        // Case 2: δ ≤ 0 → no contact
        let result = compute_slice_contact(0, -1.0, 5.0, 5.0, 115384.0, 210000.0, 0.3, 1.0, 5.0, 10.0, 0.0);
        assert!(!result.in_contact);
        assert_eq!(result.q_k, 0.0);
        assert_eq!(result.b_k, 0.0);
        assert_eq!(result.p_max_k, 0.0);
    }

    #[test]
    fn test_high_load_case() {
        // Case 3: q=2000 N/mm, R_eq=4mm
        let e_star_gpa = combined_elastic_modulus(210.0, 0.3, 210.0, 0.3);
        let e_star_mpa = e_star_gpa * 1000.0;
        let q = 2000.0;
        let r_eq = 4.0;

        let b = hertz_half_width(q, r_eq, e_star_mpa);
        let p_max = hertz_max_pressure(q, b);

        // b = sqrt(4*2000*4/(π*115384.6)) ≈ sqrt(32000/362377) ≈ 0.297 mm
        // p_max = 2*2000/(π*0.297) ≈ 4286 MPa
        assert!(
            (p_max - 4286.0).abs() < 200.0,
            "p_max = {p_max}, expected ~4286"
        );
    }

    #[test]
    fn test_weber_bulk_deformation() {
        // Case 4: Weber bulk deformation
        let q = 500.0;   // N/mm
        let e = 210000.0; // MPa
        let nu = 0.3;
        let e_star_gpa = combined_elastic_modulus(210.0, 0.3, 210.0, 0.3);
        let e_star_mpa = e_star_gpa * 1000.0;
        let b = hertz_half_width(q, 5.0, e_star_mpa);
        let h1 = 5.0;
        let h2 = 10.0;

        let h_bulk = weber_bulk_deformation(q, e, nu, b, h1, h2);
        assert!(h_bulk > 0.0, "Weber deformation should be positive");
        // Rough check: order of magnitude should be ~1-10 μm for these conditions
        assert!(h_bulk < 50.0, "Weber deformation {h_bulk} μm seems too large");
        assert!(h_bulk > 0.1, "Weber deformation {h_bulk} μm seems too small");
    }

    #[test]
    fn test_weber_zero_load() {
        let h = weber_bulk_deformation(0.0, 210000.0, 0.3, 0.1, 5.0, 10.0);
        assert_eq!(h, 0.0);
    }

    #[test]
    fn test_hertz_approach_consistency() {
        // Verify: compute delta from q, then recover q from delta
        let e_star_gpa = combined_elastic_modulus(210.0, 0.3, 210.0, 0.3);
        let e_star_mpa = e_star_gpa * 1000.0;
        let r_eq = 5.0;
        let q_original = 500.0;

        let delta = hertz_approach(q_original, r_eq, e_star_mpa);
        assert!(delta > 0.0);

        let q_recovered = solve_q_from_delta(delta, r_eq, e_star_mpa);
        let rel_err = (q_recovered - q_original).abs() / q_original;
        assert!(
            rel_err < 0.001,
            "q_recovered={q_recovered}, q_original={q_original}, rel_err={rel_err}"
        );
    }

    #[test]
    fn test_compute_slice_contact_positive_delta() {
        let e_star_gpa = combined_elastic_modulus(210.0, 0.3, 210.0, 0.3);
        let e_star_mpa = e_star_gpa * 1000.0;

        let result = compute_slice_contact(
            0,       // k
            5.0,     // delta_k [μm]
            5.0,     // r_eq_inner [mm]
            5.0,     // r_eq_outer [mm]
            e_star_mpa,
            210000.0, // E [MPa]
            0.3,     // nu
            1.5,     // slice_width [mm]
            5.0,     // h1 [mm]
            10.0,    // h2 [mm]
            0.0,     // cos_alpha_diff (legacy)
        );

        assert!(result.in_contact);
        assert!(result.q_k > 0.0);
        assert!(result.b_k > 0.0);
        assert!(result.p_max_k > 0.0);
    }

    #[test]
    fn test_level_a_verification_case1() {
        // Level A Verification: standard steel contact
        // E=210GPa, ν=0.3, R_eq=5mm, q=500 N/mm
        let e_star_gpa = combined_elastic_modulus(210.0, 0.3, 210.0, 0.3);
        let e_star_mpa = e_star_gpa * 1000.0;

        let b = hertz_half_width(500.0, 5.0, e_star_mpa);
        let p_max = hertz_max_pressure(500.0, b);

        // Expected: b ≈ 0.166 mm, p_max ≈ 1916 MPa
        let b_err = (b - 0.166).abs() / 0.166;
        let p_err = (p_max - 1916.0).abs() / 1916.0;

        assert!(b_err < 0.05, "Level A b error: {:.4}%", b_err * 100.0);
        assert!(p_err < 0.05, "Level A p_max error: {:.4}%", p_err * 100.0);
    }

    #[test]
    fn test_single_raceway_contact_consistency() {
        let e_star_gpa = combined_elastic_modulus(210.0, 0.3, 210.0, 0.3);
        let e_star_mpa = e_star_gpa * 1000.0;
        let delta_um = 5.0;
        let r_eq = 5.0;

        let (q, b, p_max, _, _) = single_raceway_contact(
            delta_um, r_eq, e_star_mpa, 210000.0, 0.3, 5.0, 10.0,
        );

        // Verify round-trip: q → delta → q
        let delta_recovered = hertz_approach(q, r_eq, e_star_mpa) * 1000.0;
        assert!((delta_recovered - delta_um).abs() / delta_um < 0.01,
            "Round-trip: input={delta_um}, recovered={delta_recovered}");
        assert!(b > 0.0);
        assert!(p_max > 0.0);
    }

    #[test]
    fn test_single_raceway_no_contact() {
        let e_star = 115000.0;
        let (q, b, p_max, _, _) = single_raceway_contact(-1.0, 5.0, e_star, 210000.0, 0.3, 5.0, 10.0);
        assert_eq!(q, 0.0);
        assert_eq!(b, 0.0);
        assert_eq!(p_max, 0.0);
    }
}
