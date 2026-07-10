//! Hertz 선접촉(line contact) — 표준식, SI.
//!
//! 참조: `CRB-main/src-tauri/src/solver/hertz.rs` (알고리즘·계수 참조만).
//! CRB 는 mm·MPa 단위였으나 본 crate 는 **SI(m, Pa, N/m)** 로 재구현한다.
//!
//! ## 지배식 (표준형)
//! 선접촉 반폭:  `b = sqrt( 4 * w' * R / (pi * E_red) )`  [m]
//! 최대압력:     `p0 = 2 * w' / (pi * b)`                  [Pa]
//!
//! 여기서 `w'` = 단위길이당 하중 [N/m], `R` = 등가반경 [m], `E_red` = 환산탄성계수 [Pa].
//!
//! ## E_red 규약 (SSOT, 전략 B)
//! `1/E_red = (1-nu1^2)/E1 + (1-nu2^2)/E2`.
//! 논문/일부 문헌의 `E'` 는 `E' = 2*E_red`. 표준 Hertz 선접촉식은 위처럼 `E_red` 를
//! 직접 쓴다(= `b = sqrt(4 w' R / (pi E_red))`). 만약 `E'` 형태 식
//! `b = sqrt(8 w' R / (pi E'))` 를 만나면 `E'=2 E_red` 치환 시 동일해진다.

/// 두 접촉체의 환산탄성계수 E_red [Pa].
///
/// `1/E_red = (1-nu1^2)/E1 + (1-nu2^2)/E2`.
/// (E1,E2 [Pa], nu1,nu2 [-]). 동종 강-강(210e9, 0.3) → 115.38e9 Pa.
pub fn e_red_from_pair(e1: f64, nu1: f64, e2: f64, nu2: f64) -> f64 {
    let inv = (1.0 - nu1 * nu1) / e1 + (1.0 - nu2 * nu2) / e2;
    1.0 / inv
}

/// Hertz 선접촉: 단위길이당 하중 w'[N/m], 환산탄성계수 e_red[Pa], 등가반경 r[m]
/// → (p0[Pa], b[m]).
///
/// - `p0`: 최대 접촉압력 [Pa]
/// - `b`:  접촉 반폭 [m]
///
/// 비물리 입력(≤0)은 (0,0) 반환.
pub fn hertz_line(w_per_len: f64, e_red: f64, r: f64) -> (f64, f64) {
    use std::f64::consts::PI;
    if w_per_len <= 0.0 || e_red <= 0.0 || r <= 0.0 {
        return (0.0, 0.0);
    }
    // b = sqrt(4 w' R / (pi E_red))   [m]
    let b = (4.0 * w_per_len * r / (PI * e_red)).sqrt();
    // p0 = 2 w' / (pi b)              [Pa]
    let p0 = 2.0 * w_per_len / (PI * b);
    (p0, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn e_red_steel_steel() {
        // E1=E2=210e9, nu=0.3 → E_red ≈ 115.4e9 Pa
        let e_red = e_red_from_pair(210e9, 0.3, 210e9, 0.3);
        assert_relative_eq!(e_red, 115.384_6e9, max_relative = 1e-4);
    }

    #[test]
    fn hertz_line_regression() {
        // 손계산 기대값 (task 명세):
        // E_red=115.4e9, w'=1e5 N/m, R=0.01 m → b≈1.05e-4 m, p0≈6.06e8 Pa
        let e_red = e_red_from_pair(210e9, 0.3, 210e9, 0.3);
        let (p0, b) = hertz_line(1e5, e_red, 0.01);
        assert_relative_eq!(b, 1.0505e-4, max_relative = 3e-3);
        assert_relative_eq!(p0, 6.060e8, max_relative = 3e-3);
    }

    #[test]
    fn hertz_line_nonphysical() {
        assert_eq!(hertz_line(-1.0, 115e9, 0.01), (0.0, 0.0));
        assert_eq!(hertz_line(1e5, 115e9, 0.0), (0.0, 0.0));
    }
}
