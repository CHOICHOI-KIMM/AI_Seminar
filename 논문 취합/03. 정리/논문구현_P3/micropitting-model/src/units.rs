//! 단위 규약(SSOT) 및 단위 변환 헬퍼.
//!
//! 본 crate 전체는 **SI 기본단위**(m, Pa, s, N)로 계산한다. 외부(논문/CRB)에서
//! mm·GPa·μm 등으로 주어진 값은 경계에서 아래 헬퍼로 SI 로 변환한 뒤 사용한다.
//!
//! | 물리량        | SSOT 단위 | 비고                          |
//! |---------------|-----------|-------------------------------|
//! | 길이/간극/유막 | m         | μm·mm 아님                    |
//! | 압력/응력      | Pa        | 인장 +                        |
//! | 시간          | s         |                               |
//! | 하중(선하중)   | N/m       | 단위길이당 (line load)        |
//! | 점도 eta0     | Pa·s      |                               |
//! | 압점도계수     | Pa^-1     | `alpha_visc`                  |

/// mm → m
#[inline]
pub fn mm_to_m(x: f64) -> f64 {
    x * 1e-3
}

/// m → mm
#[inline]
pub fn m_to_mm(x: f64) -> f64 {
    x * 1e3
}

/// μm → m
#[inline]
pub fn um_to_m(x: f64) -> f64 {
    x * 1e-6
}

/// m → μm
#[inline]
pub fn m_to_um(x: f64) -> f64 {
    x * 1e6
}

/// GPa → Pa
#[inline]
pub fn gpa_to_pa(x: f64) -> f64 {
    x * 1e9
}

/// Pa → GPa
#[inline]
pub fn pa_to_gpa(x: f64) -> f64 {
    x * 1e-9
}

/// N/mm (선하중) → N/m
#[inline]
pub fn n_per_mm_to_n_per_m(x: f64) -> f64 {
    x * 1e3
}

/// alpha_visc: 1/GPa → 1/Pa  (CRB는 alpha_pv 를 nGPa^-1 등으로 저장; 경계에서 정규화)
#[inline]
pub fn inv_gpa_to_inv_pa(x: f64) -> f64 {
    x * 1e-9
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn roundtrip_length() {
        assert_relative_eq!(m_to_mm(mm_to_m(12.5)), 12.5, max_relative = 1e-12);
        assert_relative_eq!(m_to_um(um_to_m(0.3)), 0.3, max_relative = 1e-12);
    }

    #[test]
    fn roundtrip_pressure() {
        assert_relative_eq!(pa_to_gpa(gpa_to_pa(210.0)), 210.0, max_relative = 1e-12);
    }
}
