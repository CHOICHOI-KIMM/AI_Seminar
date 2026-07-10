//! Dowson-Toyoda (1978) 중심유막두께 h_c — 보정 제외(열/기아 φ_t, φ_s 미포함).
//!
//! 참조: `CRB-main/src-tauri/src/solver/lubrication.rs::compute_film_thickness`
//! (무차원 U,G,W 정의 및 h_c 계수만 참조). CRB 는 φ_t·φ_s 를 곱했으나 본 함수는
//! **순수 Dowson-Toyoda smooth 값**만 반환한다.
//!
//! ## 무차원 군 (CRB/표준 정렬, E_red 사용)
//! - 속도    U = eta0 * u_mean / (E_red * R)      [-]
//! - 재료    G = alpha_visc * E_red               [-]
//! - 하중    W = w' / (E_red * R)                 [-]   (w' = 단위길이당 하중 N/m)
//!
//! ## 중심유막 (Dowson-Toyoda 1978)
//! `H_c = 3.06 * U^0.69 * G^0.56 * W^-0.10`,  `h_c = H_c * R`  [m]
//!
//! ## E_red 규약
//! 위 U,G,W 는 `E_red`(= `1/E_red=(1-nu1^2)/E1+(1-nu2^2)/E2`)를 직접 사용한다(CRB 와 동일).
//! 문헌이 `E'` 로 표기한 경우 `E' = 2*E_red`. Dowson-Toyoda 계수(3.06, 지수)는 `E'`
//! 기준으로 유도되었으므로, `E'` 규약으로 정확히 재현하려면 U,G,W 에 `E'=2*E_red` 를
//! 대입해야 한다. 본 구현은 **CRB 정렬(E_red 직접 사용)** 을 SSOT 로 채택한다(D0 표 참조).

/// Dowson-Toyoda 중심유막두께 h_c [m] (보정 φ_t, φ_s 제외).
///
/// 입력(SI): eta0[Pa·s], u_mean[m/s], alpha_visc[Pa^-1], e_red[Pa],
/// w_per_len[N/m], r_eq[m]. 비물리 입력(≤0)은 0 반환.
pub fn dowson_toyoda_hc(
    eta0: f64,
    u_mean: f64,
    alpha_visc: f64,
    e_red: f64,
    w_per_len: f64,
    r_eq: f64,
) -> f64 {
    if eta0 <= 0.0
        || u_mean <= 0.0
        || alpha_visc <= 0.0
        || e_red <= 0.0
        || w_per_len <= 0.0
        || r_eq <= 0.0
    {
        return 0.0;
    }
    let u_param = eta0 * u_mean / (e_red * r_eq);
    let g_param = alpha_visc * e_red;
    let w_param = w_per_len / (e_red * r_eq);

    let h_c_dimless =
        3.06 * u_param.powf(0.69) * g_param.powf(0.56) * w_param.powf(-0.10);
    h_c_dimless * r_eq
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn dowson_toyoda_regression() {
        // 회귀 손계산: eta0=0.01, u=1, alpha=2e-8, E_red=115.4e9, w'=1e5, R=0.01
        //  U = 0.01*1/(115.4e9*0.01)      = 8.6655e-12
        //  G = 2e-8*115.4e9               = 2308
        //  W = 1e5/(115.4e9*0.01)         = 8.6655e-5
        //  H_c = 3.06*U^0.69*G^0.56*W^-0.10 ≈ 1.388e-5
        //  h_c = H_c*R ≈ 1.388e-7 m (≈ 0.139 μm)
        let e_red = 115.384_6e9;
        let h_c = dowson_toyoda_hc(0.01, 1.0, 2e-8, e_red, 1e5, 0.01);
        assert_relative_eq!(h_c, 1.388e-7, max_relative = 2e-2);
    }

    #[test]
    fn dowson_toyoda_monotonic_speed() {
        // 속도 증가 → 유막 증가 (지수 0.69 > 0)
        let e_red = 115.384_6e9;
        let h1 = dowson_toyoda_hc(0.01, 1.0, 2e-8, e_red, 1e5, 0.01);
        let h2 = dowson_toyoda_hc(0.01, 2.0, 2e-8, e_red, 1e5, 0.01);
        assert!(h2 > h1);
    }

    #[test]
    fn dowson_toyoda_nonphysical() {
        assert_eq!(dowson_toyoda_hc(0.0, 1.0, 2e-8, 115e9, 1e5, 0.01), 0.0);
        assert_eq!(dowson_toyoda_hc(0.01, 1.0, 2e-8, 115e9, 1e5, 0.0), 0.0);
    }
}
