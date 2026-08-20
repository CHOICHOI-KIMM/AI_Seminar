// BB Contact Analysis — 공용 수치·물성 유틸
//
// BB Phase 1-S3 (2026-08-20) 신설.
// 접촉 형상(선/점)과 베어링 종류에 **무관한** 순수 함수만 둔다.
// CRB 의 `hertz.rs` / `geometry.rs` 에서 살린 3개를 여기로 모았다.
//
// ── 단위 (D-10) ─────────────────────────────────────────────────────
// 솔버 내부 단위는 mm · N · rad · MPa 다.
// **단위 환산 상수는 이 파일의 명시적 변환 함수 안에만 존재할 수 있다.**
// 다른 solver 모듈에는 환산 상수가 등장해서는 안 된다.

/// mm³ → cm³. 밀도가 관례상 [g/cm³] 로 주어지기 때문에 필요한 유일한 환산.
const MM3_PER_CM3: f64 = 1000.0;

/// 등가 탄성계수 E* [MPa].
///
/// `1/E* = (1−ν₁²)/E₁ + (1−ν₂²)/E₂`
///
/// CRB 원본(`hertz.rs::combined_elastic_modulus`)은 GPa 를 반환하고
/// 소비처 15곳이 각자 `* 1000.0` 을 곱했다. 여기서는 입력·출력 모두 MPa 다.
///
/// Harris 규약의 `E′ = 2E*` 와 혼동하지 말 것 (Theory §6 기호 대응).
pub fn combined_elastic_modulus_mpa(e1_mpa: f64, nu1: f64, e2_mpa: f64, nu2: f64) -> f64 {
    let inv = (1.0 - nu1 * nu1) / e1_mpa + (1.0 - nu2 * nu2) / e2_mpa;
    1.0 / inv
}

/// Harris 규약 등가 탄성계수 `E′ = 2 E*` [MPa].
///
/// Hamrock-Dowson 무차원수(U, G, W)와 베어링 카탈로그가 이 규약을 쓴다.
/// P5 에서 필요하다.
pub fn harris_e_prime_mpa(e1_mpa: f64, nu1: f64, e2_mpa: f64, nu2: f64) -> f64 {
    2.0 * combined_elastic_modulus_mpa(e1_mpa, nu1, e2_mpa, nu2)
}

/// 두 곡률반경의 직렬 합성 `1/R = 1/R₁ + 1/R₂` [mm].
///
/// 어느 쪽이든 `1e6 mm` 를 넘으면 평면으로 보고 무한대 취급한다
/// (CRB 원본 `geometry.rs::combine_curvature` 의 규약을 유지).
/// 오목면은 음의 반경으로 넣는다.
pub fn combine_curvature_mm(r1_mm: f64, r2_mm: f64) -> f64 {
    let inv1 = if r1_mm.abs() > 1.0e6 { 0.0 } else { 1.0 / r1_mm };
    let inv2 = if r2_mm.abs() > 1.0e6 { 0.0 } else { 1.0 / r2_mm };
    let inv = inv1 + inv2;
    if inv.abs() < 1.0e-12 {
        f64::INFINITY
    } else {
        1.0 / inv
    }
}

/// 구(볼)의 질량 [g].
///
/// `m = (π/6) D_w³ · ρ`. `D_w` 는 [mm], `ρ` 는 [g/cm³] 이므로
/// mm³ → cm³ 환산이 필요하다 — 이 파일이 그 환산을 담는 유일한 장소다.
pub fn sphere_mass_g(d_w_mm: f64, density_g_cm3: f64) -> f64 {
    let volume_mm3 = std::f64::consts::PI / 6.0 * d_w_mm.powi(3);
    volume_mm3 * density_g_cm3 / MM3_PER_CM3
}

/// 자연 3차 스플라인 보간 (Thomas 알고리즘).
///
/// `points` 는 x 오름차순이어야 한다. 정의역 밖은 끝값으로 클램프한다.
/// 점이 2개 이하면 선형 보간 / 상수로 축퇴한다.
///
/// CRB 원본(`geometry.rs::cubic_spline_interpolate`)을 그대로 옮겼다.
/// ACBB 에서는 측정 프로파일·물성 테이블 보간에 쓸 수 있다.
pub fn cubic_spline_interpolate(points: &[(f64, f64)], x: f64) -> f64 {
    let n = points.len();
    if n == 0 {
        return 0.0;
    }
    if n == 1 {
        return points[0].1;
    }
    if x <= points[0].0 {
        return points[0].1;
    }
    if x >= points[n - 1].0 {
        return points[n - 1].1;
    }
    if n == 2 {
        let (x0, y0) = points[0];
        let (x1, y1) = points[1];
        return y0 + (y1 - y0) * (x - x0) / (x1 - x0);
    }

    // 구간 폭
    let h: Vec<f64> = (0..n - 1).map(|i| points[i + 1].0 - points[i].0).collect();

    // 삼중대각계 구성 (자연 경계조건: 양 끝 2차 도함수 = 0)
    let mut a = vec![0.0; n];
    let mut b = vec![0.0; n];
    let mut c = vec![0.0; n];
    let mut d = vec![0.0; n];
    b[0] = 1.0;
    b[n - 1] = 1.0;
    for i in 1..n - 1 {
        a[i] = h[i - 1];
        b[i] = 2.0 * (h[i - 1] + h[i]);
        c[i] = h[i];
        d[i] = 6.0
            * ((points[i + 1].1 - points[i].1) / h[i] - (points[i].1 - points[i - 1].1) / h[i - 1]);
    }

    // Thomas 전진소거 / 후진대입
    let mut cp = vec![0.0; n];
    let mut dp = vec![0.0; n];
    cp[0] = c[0] / b[0];
    dp[0] = d[0] / b[0];
    for i in 1..n {
        let m = b[i] - a[i] * cp[i - 1];
        cp[i] = if i < n - 1 { c[i] / m } else { 0.0 };
        dp[i] = (d[i] - a[i] * dp[i - 1]) / m;
    }
    let mut m2 = vec![0.0; n];
    m2[n - 1] = dp[n - 1];
    for i in (0..n - 1).rev() {
        m2[i] = dp[i] - cp[i] * m2[i + 1];
    }

    // 해당 구간 탐색 후 3차식 평가
    let mut k = 0;
    for i in 0..n - 1 {
        if x >= points[i].0 && x <= points[i + 1].0 {
            k = i;
            break;
        }
    }
    let (x0, y0) = points[k];
    let (x1, y1) = points[k + 1];
    let hk = h[k];
    let t1 = m2[k] * (x1 - x).powi(3) / (6.0 * hk);
    let t2 = m2[k + 1] * (x - x0).powi(3) / (6.0 * hk);
    let t3 = (y0 / hk - m2[k] * hk / 6.0) * (x1 - x);
    let t4 = (y1 / hk - m2[k + 1] * hk / 6.0) * (x - x0);
    t1 + t2 + t3 + t4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e_star_steel_matches_closed_form() {
        // 강-강 동일 재질: E* = E / (2(1−ν²))
        let e = 207_000.0;
        let nu = 0.3;
        let expected = e / (2.0 * (1.0 - nu * nu));
        let got = combined_elastic_modulus_mpa(e, nu, e, nu);
        assert!((got - expected).abs() / expected < 1e-12);
    }

    #[test]
    fn e_star_dissimilar_lies_between() {
        let got = combined_elastic_modulus_mpa(207_000.0, 0.3, 300_000.0, 0.26);
        assert!(got > 0.0 && got < 300_000.0);
    }

    #[test]
    fn harris_e_prime_is_twice_e_star() {
        let e_star = combined_elastic_modulus_mpa(207_000.0, 0.3, 207_000.0, 0.3);
        let e_prime = harris_e_prime_mpa(207_000.0, 0.3, 207_000.0, 0.3);
        assert!((e_prime - 2.0 * e_star).abs() / e_prime < 1e-12);
    }

    #[test]
    fn curvature_series_combination() {
        // 1/R = 1/4 + 1/6 → R = 2.4
        assert!((combine_curvature_mm(4.0, 6.0) - 2.4).abs() < 1e-12);
    }

    #[test]
    fn curvature_treats_large_radius_as_plane() {
        // 평면(무한 반경) + R=5 → R_eq = 5
        assert!((combine_curvature_mm(1.0e9, 5.0) - 5.0).abs() < 1e-9);
    }

    #[test]
    fn sphere_mass_matches_analytic() {
        // D_w = 11.5 mm, ρ = 7.85 g/cm³
        let d: f64 = 11.5;
        let rho: f64 = 7.85;
        let expected = std::f64::consts::PI / 6.0 * d.powi(3) * rho / 1000.0;
        let got = sphere_mass_g(d, rho);
        assert!((got - expected).abs() / expected < 1e-12);
        // 크기 감각: 지름 11.5 mm 강구는 약 6.25 g
        assert!(got > 6.0 && got < 6.5);
    }

    #[test]
    fn spline_reproduces_linear_data() {
        let pts = [(0.0, 0.0), (1.0, 1.0), (2.0, 2.0), (3.0, 3.0)];
        assert!((cubic_spline_interpolate(&pts, 1.5) - 1.5).abs() < 1e-9);
    }

    #[test]
    fn spline_approximates_quadratic() {
        let pts = [(0.0, 0.0), (1.0, 1.0), (2.0, 4.0), (3.0, 9.0), (4.0, 16.0)];
        let got = cubic_spline_interpolate(&pts, 1.5);
        assert!((got - 2.25).abs() < 0.05, "got {got}");
    }

    #[test]
    fn spline_clamps_outside_domain() {
        let pts = [(0.0, 5.0), (1.0, 7.0), (2.0, 9.0)];
        assert!((cubic_spline_interpolate(&pts, -1.0) - 5.0).abs() < 1e-12);
        assert!((cubic_spline_interpolate(&pts, 99.0) - 9.0).abs() < 1e-12);
    }
}
