//! # M2 — 윤활(EHL) 모듈: 압력 리플 특수해 + Eyring 방향별 유효점도
//!
//! Phase 0 가 동결한 인터페이스([`crate::types`])·유틸([`crate::util`]) 위에
//! **M2 만** 구현한다(공유 파일 수정 금지).
//!
//! ## 과제 범위 (이번 Phase)
//! - **압력 리플 특수해(particular solution)** 와 **진폭감소비**(amplitude reduction ratio).
//! - **Eyring 방향별(directional) 유효점도**를 Poiseuille 항에 반영.
//! - 상보파(complementary function, 입구/출구에서 발생해 접촉을 통과하는 이동파)는
//!   **후속 Phase**. 여기서는 국소 정상해(steady particular solution)만 다룬다.
//!
//! ## 물리 모델 (Greenwood–Morales-Espejo 1994 정성, GW1994)
//! 고압 중앙부에서 정상 Reynolds 식을 매끈한 해 주위로 선형화하면, 파수 `k` 의 단일
//! 정현파 거칠기(간극 섭동 `s_a`)에 대한 압력 리플 `p_a` 는 다음 전달함수를 만족한다.
//!
//! ### SSOT 표준형 (E' = 2·E_red 1회 치환 반영)
//! ```text
//!   p_a / s_a = (κ · E_red / 2) · i·Q / (1 − i·Q − i·C·Q)     [Pa/m]
//! ```
//! - `κ`(kappa) = 거칠기 파수 |k| [1/m] (= `alpha_wave`).
//! - `E_red` = 환산탄성계수 [Pa] (`1/E_red=(1-ν²)/E₁+(1-ν²)/E₂`).
//!   논문 `E'` 는 `E'=2·E_red` 이므로 전달함수 계수 `E_red/2 = E'/4`,
//!   탄성결합 계수 `2/(E_red·κ) = 4/(E'·κ)` 로 1회 치환됨(주석 명기).
//! - `Q` = 무차원 진폭감소 파라미터
//!   `Q = 24·η_eff·(u₂−ū)·k_x / (h0³·|k|³·E_red)`  (아래 유도 참조; 식[5] 부속 Q정의).
//!   대류속도는 **평균구름 ū 가 아니라 특수해 이동속도차 (u₂−ū)=−Δu/2**(미끄럼 절반)이다.
//! - `C` = 압축성(밀도-압력) 보정 계수 [-]. 식[4]`ρ_a=(ρ/B)p_a`→`C = h·E'·κ/(4B)`
//!   (B=윤활유 체적탄성). 비압축 극한 `B→∞ ⇒ C=0`.
//!   (`C≠0` 정밀형·B(p) 는 상보파 Phase 의 잔여큐 RQ-2; C 민감도는 `compressibility_c_sensitivity`.)
//!
//! ### 유도 개요 (1D 횡거칠기, Couette 지배 중앙부, 비압축·등점도) — 식[2] 근거
//! 식[2] RHS 대류항 `ū ∂ₓh + ∂ₜh`. 특수해는 표면2 거칠기와 함께 `u₂` 로 이동
//! (`δh∝e^{iωₓ(x−u₂t)}`) → `∂ₜh=−iωₓu₂ h`, 따라서 `ū∂ₓh+∂ₜh = iωₓ(ū−u₂)h`.
//! 선형 Reynolds `∂ₓ[h³/(12η)∂ₓp] = (ū−u₂)∂ₓh` 를 모드 `e^{iωₓx}` 로 선형화:
//! - Poiseuille: `−h0³k²/(12η)·p_a`,  Couette: `(ū−u₂)·i·k·h_a`.
//! - ⇒ `p_a = i·M·h_a`,  `M = 12·η·(u₂−ū)/(h0³·k)`  (부호: (u₂−ū)=−Δu/2).
//! - 탄성결합(반무한체 주기압력 표면변위, 식[3] `ν_a=4p_a/(E'κ)=2p_a/(E_red·κ)`):
//!   `h_a = s_a + (2/(E_red·k))·p_a`.
//! - `Q ≡ 2M/(E_red·k) = 24·η·(u₂−ū)/(h0³·k²·E_red)` 로 정의하면
//!   `M = (E_red·k/2)·Q` 이므로 위 표준형이 나온다. 2D 로 일반화하면 `k→|k|`,
//!   `Q` 분자에 구름방향 성분 `k_x`(원논문 Q정의: 분모 `ωₓ²/ηₓ+ωy²/ηy=κ²/η_mode`).
//!
//! ### 유막 리플(진폭감소) — 표준형 부산물
//! ```text
//!   h_a / s_a = 1 + (2/(E_red·κ))·(p_a/s_a) = (1 − i·C·Q)/(1 − i·(1+C)·Q)
//! ```
//! `C=0` 일 때 `|h_a/s_a| = 1/√(1+Q²)` — GW1994 의 유막 진폭감소(장파장/저속에서
//! 평활화, 단파장/고속에서 원거칠기 통과)를 정성 재현한다.
//!
//! ## 반환
//! [`solve_full_film`] 은 특수해분 [`LubResult`] 를 반환한다:
//! - `p_lub` = 압력 리플 특수해장 [Pa] (평균 0, Hertz 평균압 주위 섭동분).
//! - `h_lub` = `h_bar` + 유막 리플장 [m].

use crate::types::{Field2, LubResult, PartialLubInput, PartialLubResult};
use crate::util::fft::{fft2_forward, fft2_inverse};
use rustfft::num_complex::Complex;
use std::f64::consts::PI;

// ─────────────────────────────────────────────────────────────────────────
//  M2 핵심 물리 함수 (공개 — 단위 테스트/후속 모듈 재사용)
// ─────────────────────────────────────────────────────────────────────────

/// Eyring 접선(미분) 유효점도 [Pa·s].
///
/// Eyring 유변: `γ̇ = (τ0/η)·sinh(τ/τ0)`. 평균 전단률 `γ̇` 하의 **미소 섭동 유동에
/// 대한 접선 점도** `dτ/dγ̇` 는
/// ```text
///   η_eff = η / cosh(τ/τ0) = η / √(1 + (η·γ̇/τ0)²)
/// ```
/// (마지막 등식은 `τ = τ0·asinh(η·γ̇/τ0)` 대입). 전단률 0 → `η_eff = η`.
///
/// 이것이 **방향별(directional) 점도**의 근원이다: 구름/미끄럼(x) 방향은 평균 전단으로
/// 점도가 감소, 횡(y) 방향은 평균 전단이 없어 `η` 유지.
///
/// 입력(SI): `eta`[Pa·s], `shear_rate`[1/s], `tau0`[Pa]. `tau0≤0` 이면 감소 없음.
pub fn eyring_eff_visc(eta: f64, shear_rate: f64, tau0: f64) -> f64 {
    if tau0 <= 0.0 || eta <= 0.0 {
        return eta.max(0.0);
    }
    let s = eta * shear_rate.abs() / tau0;
    eta / (1.0 + s * s).sqrt()
}

/// Barus 압점도(piezoviscous) 국소 점도 [Pa·s]: `η = η0·exp(α·p)`.
///
/// 접촉 중앙부의 국소 점도는 기준점도 `eta0` 보다 압도적으로 크며, 진폭감소
/// 파라미터 `Q ∝ η` 를 지배한다. (Roelands 형은 잔여큐 RQ-3.)
#[inline]
pub fn barus_visc(eta0: f64, alpha_visc: f64, p: f64) -> f64 {
    eta0 * (alpha_visc * p).exp()
}

/// Roelands 압력상수 `c_p` [Pa] (Roelands 1966 표준값).
const ROELANDS_CP: f64 = 1.96e8;

/// Roelands 압점도 국소 점도 [Pa·s] — **RQ-3 해소**(Barus exp 폭주를 포화형으로 대체).
///
/// `η = η0·exp{ (ln η0 + 9.67)·[ (1 + p/c_p)^Z − 1 ] }`, `c_p = 1.96e8 Pa` (Roelands 1966).
/// 압점도 지수 `Z = α_visc·c_p/(ln η0 + 9.67)` 로 **저압에서 Barus 기울기**(dη/dp|₀=η0·α_visc)
/// 와 1차 일치 → 입력 `α_visc` 에 소급(자의적 계수 없음). 고압(운전점 1.5 GPa)에서 유계로
/// 포화하여 Barus `exp(30)≈1e11 Pa·s` 비물리를 제거. 원 논문 식[6] Eyring 운전점 η 계열
/// ((30) Ehret; Roelands 압점도). `roelands_bounded_and_low_p_matches_barus` 가 유계·정합 강제.
#[inline]
pub fn roelands_visc(eta0: f64, alpha_visc: f64, p: f64) -> f64 {
    let z_denom = eta0.ln() + 9.67; // η0 [Pa·s]
    let z = alpha_visc * ROELANDS_CP / z_denom;
    eta0 * (z_denom * ((1.0 + p / ROELANDS_CP).powf(z) - 1.0)).exp()
}

/// 무차원 진폭감소 파라미터 `Q` [-] (SSOT, 식[5] 부속 Q정의).
///
/// 원 논문 정의(식[5] 아래):
/// ```text
///   Q = [48·(u₂−ū)·ωx / (E'·h³·κ)] / (ωx²/ηx + ωy²/ηy)
/// ```
/// 방향별 유효점도를 단일 등가점도 `η_mode`(directional_visc, `ωx²/ηx+ωy²/ηy=κ²/η_mode`)로
/// 접고 `E'=2·E_red` 치환(`48/E'=24/E_red`)하면:
/// ```text
///   Q = 24·η_mode·(u₂−ū)·k_x / (h0³·|k|³·E_red)
/// ```
/// - `eta_eff`=η_mode[Pa·s], `u_conv`=(u₂−ū)[m/s] **대류 미끄럼속도(=−Δu/2)**,
/// - `h0`[m]: 중앙유막(h_bar), `kx`[1/m]: 구름방향 파수 성분(부호 포함),
/// - `k_mag`[1/m]: 파수 크기 |k|, `e_red`[Pa]. 비물리(≤0) 분모는 0 반환.
///
/// 계수 24(=48/E')·지수 |k|³ 은 `q_independent_oracle_from_eq2` 가 식[2] 손유도로 독립 강제.
///
/// 대류속도: 특수해는 표면2 거칠기와 함께 `u₂` 로 이동하므로 `(u₂−ū)=−Δu/2`(미끄럼 절반).
/// [`solve_full_film`] 은 이를 `u_conv=−slide_roll·u_mean/2` 로 결선한다(식[2] 대로).
/// 상보파(`ū` 전파·비뉴턴 감쇠, 식[7][8])는 별도 미구현 → 순수 rolling(`u₂→ū`) 근처
/// 특수해 리플이 0 으로 과소예측(잔여: 상보파 Phase). 이때 리플은 상보파가 담당.
pub fn amplitude_q(
    eta_eff: f64,
    u_conv: f64,
    h0: f64,
    kx: f64,
    k_mag: f64,
    e_red: f64,
) -> f64 {
    if h0 <= 0.0 || k_mag <= 0.0 || e_red <= 0.0 {
        return 0.0;
    }
    24.0 * eta_eff * u_conv * kx / (h0.powi(3) * k_mag.powi(3) * e_red)
}

/// 압력 리플 전달함수(특수해) `p_a/s_a` [Pa/m] — SSOT 표준형.
///
/// ```text
///   p_a/s_a = (κ·E_red/2) · i·Q / (1 − i·Q − i·C·Q)
/// ```
/// `kappa`=κ[1/m], `e_red`[Pa], `q`=Q[-], `c`=C[-].
/// 크기 `|p_a/s_a| = (κ·E_red/2)·|Q|/√(1+((1+C)Q)²)`:
/// `Q→0` 이면 0, `|Q|→∞` 이면 `(κ·E_red/2)/(1+C)` 로 포화(GW1994 정성).
pub fn pressure_ripple_transfer(kappa: f64, e_red: f64, q: f64, c: f64) -> Complex<f64> {
    let i = Complex::new(0.0, 1.0);
    let num = i * q; // i·Q
    let den = Complex::new(1.0, 0.0) - i * q - i * (c * q); // 1 − iQ − iCQ
    let pref = Complex::new(kappa * e_red / 2.0, 0.0); // κ·E_red/2 (= κ·E'/4)
    if den.norm_sqr() < f64::MIN_POSITIVE {
        return Complex::new(0.0, 0.0);
    }
    pref * num / den
}

/// 유막 리플 전달함수 `h_a/s_a` [-] — 표준형 부산물.
///
/// `h_a/s_a = 1 + (2/(E_red·κ))·(p_a/s_a) = (1 − iCQ)/(1 − i(1+C)Q)`.
/// `C=0` 이면 `|h_a/s_a| = 1/√(1+Q²)` (진폭감소).
pub fn film_ripple_transfer(kappa: f64, e_red: f64, q: f64, c: f64) -> Complex<f64> {
    if kappa <= 0.0 || e_red <= 0.0 {
        return Complex::new(1.0, 0.0);
    }
    let p = pressure_ripple_transfer(kappa, e_red, q, c);
    Complex::new(1.0, 0.0) + Complex::new(2.0 / (e_red * kappa), 0.0) * p
}

// ─────────────────────────────────────────────────────────────────────────
//  상보파(complementary wave) — 입구 생성·ū 전파·비뉴턴 감쇠 (ref (28) Hooke2007)
// ─────────────────────────────────────────────────────────────────────────
//
//  진폭감소는 **2성분해**다(GW1994=ref(29)): 선형 Reynolds 식[2]의 해 = 특수해
//  (particular, 표면2 거칠기와 함께 u₂ 이동, 위 `pressure_ripple_transfer`) + **상보파**
//  (complementary, 입구 생성·평균속도 ū 전파·비뉴턴 감쇠, 식[7][8]=ref(28) 식(25)~(30)).
//  ★ 순수 rolling(u₂→ū)에서 특수해 Q∝(u₂−ū)→0 → 특수해 압력리플 0 이나, **압력 리플은
//  물리적으로 존재**하며 이를 상보파가 담당(GW1994 Fig.7b). 아래 함수가 그 성분이다.

/// 상보파 유입진폭비 `g = h_c/A` [-] — **G-M2-1 데이터 gap**(자유계수 아님, 문헌 소급).
///
/// 상보파 진폭(입구 pumping)은 접촉 유입부 해석 필요 — ref(28) Hooke 는 이를 지연(별도 논문),
/// ref(29) GW1994 §3.2 는 **"미변형 거칠기 pumping 의 절반으로 가정하면 Venner-Lubrecht(9)와
/// fair agreement"** 로 근사(예제 fit `r₂=0.45`). ref(31) Venner-Lubrecht 뉴턴 진폭감소
/// 보간표 수치는 우리 문서에 **미제공**(G-M2-1). 따라서 GW1994 "half pumping" 을 근거
/// **기본값 0.5** 로 채택. ref(28) Fig.10 상보파 최대≈0.6·GW fit 0.45 범위가 **민감도 대역**
/// (RQ-M2-comp1). 파장의존 정밀 보간은 데이터 부재로 상수근사 + 민감도로 정직 처리.
///
/// **G-M2-1 정량 검증(2026-07-14, ref(15) Venner1997 원문 소급)**: 고하중 순수 rolling 서
/// 정상(특수해) 유막 평탄화·상보파 지배(ref(31) L631) ⇒ A_d/A_i=g. Venner eq(5)
/// `1/(1+0.17∇+0.03∇²)` 를 g 로 역산한 λ/b 가 Table1 실측 A_d/A_i=0.5 교차구간 (0.25,0.5) 안:
/// g=0.5 → λ/b≈0.377(∇≈3.60), GW half-pumping 중앙곡선점과 정합 → **단일점 검증됨**.
/// 오라클 `vc_m2_comp_amplitude_venner`(단일 2단): Part A 가 eq(5)↔Table1 정확도(2~5%)로 외부
/// 기준곡선을 확정하고 Part B 가 모델 g 를 그 곡선에 anchor(g-민감·변이 g×0.5 CAUGHT). 곡선
/// 전체(파장의존 g(∇)) 는 types 동결로 ∇ 산출(b,M,L) 불가 → 잔여 RQ-M2-comp-curve.
pub const COMP_INLET_RATIO: f64 = 0.5;

/// 상보파 유입진폭비 g (기본 [`COMP_INLET_RATIO`]). 민감도/변이게이트용 접근자.
#[inline]
pub fn complementary_inlet_ratio() -> f64 {
    COMP_INLET_RATIO
}

/// 상보파 공칭 파수 `ω_c = ω·(ν/u) = kx·(u₂/ū)` [1/m] (ref(28) 식(25) 부속).
///
/// 거칠기는 유입부에 주파수 `ω·ν`(=kx·u₂) 로 진입, 상보파는 평균속도 `u=ū` 로 이탈 →
/// 공칭 파수 `ω_c=ων/u`. 거친면이 빠르면(ν>u) 파장 단축, 느리면 신장(GW1994 λ'=λū/u₂).
/// 순수 rolling(u₂=ū) 이면 `ω_c=kx`(파수 불변).
#[inline]
pub fn complementary_wavenumber(kx: f64, u2: f64, u_mean: f64) -> f64 {
    if u_mean == 0.0 {
        return kx;
    }
    kx * (u2 / u_mean)
}

/// 상보파 분산관계 `ψ = ω_d + iβ` 를 식(30)[ref(28)] 실/허부 교대 대입으로 수렴.
///
/// ```text
///   ψ = ω_c + i·(E'/τ_e)·(h²·Ω/24)·|1−ν/u|·(ψ² + V·ξ²) / [1 + (E'·h·Ω/(4B))]     [식(30)]
/// ```
/// `Ω²=ψ²+ξ²`(복소 파수 크기), `E'=2·E_red`(SSOT 치환), `τ_e`=Eyring 응력, `V=η_x/η_y`
/// (방향점도비), `ξ=ky`. **비압축 극한 B→∞ ⇒ 분모=1**(특수해 `c_comp=0` 과 정합; C≠0 은
/// RQ-2). `ω_c`=[`complementary_wavenumber`], `nu_over_u`=ν/u=u₂/ū.
///
/// **수렴**: ψ₀=ω_c(뉴턴, β=0) 에서 고정점 반복(식(30) RHS). Hooke: "실부로 실부를, 허부로
/// 허부를 추정" — RHS 가 자연 분해된다. tol=1e-5, 상한 `MAX_IT`. **비수렴 폴백=`None`**:
/// 단파장·고감쇠에서 고정점(ψ³ 항) 발산 → 물리적으로 상보파가 급감쇠(exp(−βx')→0)하여
/// 기여 무시 가능한 영역이므로, 호출부는 `None` 을 **상보파 기여 0**(완전감쇠)으로 처리한다
/// (β=0 폴백은 오히려 무감쇠 오류 → 채택 안 함). β<0(비물리 성장) 은 |β| 로 강제.
///
/// **G-M2-2**: β 초기추정·수렴 tol 은 논문 미제공 → ψ₀=ω_c·tol=1e-5 가정(RQ-M2-comp2).
pub fn dispersion_psi(
    omega_c: f64,
    ky: f64,
    h: f64,
    e_red: f64,
    tau_e: f64,
    v_ratio: f64,
    nu_over_u: f64,
) -> Option<Complex<f64>> {
    let e_prime = 2.0 * e_red; // SSOT: E' = 2·E_red
    let slip = (1.0 - nu_over_u).abs();
    // 뉴턴/퇴화: 감쇠 없음(β=0) — 순수 rolling(slip=0) 포함. Ω=ψ=ω_c 실수.
    if tau_e <= 0.0 || h <= 0.0 || slip == 0.0 {
        return Some(Complex::new(omega_c, 0.0));
    }
    // 계수 = (E'/τ_e)·(h²/24)·|1−ν/u| [분모 1 = 비압축 B→∞].
    let coef = (e_prime / tau_e) * (h * h / 24.0) * slip;
    let ksq = Complex::new(ky * ky, 0.0);
    let vksq = Complex::new(v_ratio * ky * ky, 0.0);
    let mut psi = Complex::new(omega_c, 0.0);
    let scale = omega_c.abs().max(1.0);
    const MAX_IT: usize = 200;
    const TOL: f64 = 1e-5;
    for _ in 0..MAX_IT {
        let big_omega = (psi * psi + ksq).sqrt(); // Ω = √(ψ²+ξ²) (주분지 Re≥0)
        let bracket = psi * psi + vksq; // ψ² + V·ξ²
        // 식(30) RHS: ω_c + i·coef·Ω·(ψ²+Vξ²).
        let rhs = Complex::new(omega_c, 0.0) + Complex::new(0.0, coef) * big_omega * bracket;
        // 발산 감지(단파장 고감쇠) → None(완전감쇠, 상보파 기여 0).
        if !rhs.re.is_finite() || !rhs.im.is_finite() || rhs.norm() > 1.0e8 * scale {
            return None;
        }
        let d = (rhs - psi).norm();
        psi = rhs;
        if d <= TOL * psi.norm().max(1.0) {
            // 물리 감쇠: β≥0 강제(비물리 성장 방지).
            return Some(Complex::new(psi.re, psi.im.abs()));
        }
    }
    None // 비수렴 → 완전감쇠 취급
}

/// 상보파 표면강성 `p_c/h_c = ΩE'/4 = Ω·E_red/2` [Pa/m], `Ω²=ψ²+ξ²` (식(28) ref(28)).
///
/// 정현파 표면 강성(식[3] `w_a=4p_a/(E'κ)`)의 **복소 파수 ψ 확장**. SSOT `E'=2·E_red`
/// 이므로 `ΩE'/4 = Ω·E_red/2`.
#[inline]
pub fn complementary_pressure_stiffness(
    psi: Complex<f64>,
    ky: f64,
    e_red: f64,
) -> Complex<f64> {
    let big_omega = (psi * psi + Complex::new(ky * ky, 0.0)).sqrt();
    big_omega * Complex::new(e_red / 2.0, 0.0) // Ω·E_red/2 = Ω·E'/4
}

/// 상보파 2성분 게인 `(δh_c/A, δp_c/A)` per 단위 거칠기 진폭 A (ref(28) 식(25)(27)(28)).
///
/// ```text
///   δh_c/A = g·exp(iψx')            [식(25): 유입진폭 g, 전파 exp(iψx')=exp(iω_d x')·exp(−βx')]
///   δp_c/A = (ΩE'/4)·(δh_c/A)       [식(28) 표면강성]
/// ```
/// `g`=유입진폭비([`complementary_inlet_ratio`]), `x_transit`=유입→평가점(중앙) 전파거리 [m].
/// 전파위상 `exp(iψx')` 은 **크기 감쇠 exp(−βx')** 와 **위상 exp(iω_d x')** 동시 반영.
///
/// 반환 `(h_gain[-], p_gain[Pa/m])`. `k→−k` 에서 `ψ→−conj(ψ)`(고정점 대칭)이라 두 게인 모두
/// 켤레대칭 → 실공간 IFFT 실수성 보존.
pub fn complementary_wave(
    psi: Complex<f64>,
    ky: f64,
    e_red: f64,
    g_inlet: f64,
    x_transit: f64,
) -> (Complex<f64>, Complex<f64>) {
    let i = Complex::new(0.0, 1.0);
    let phase = (i * psi * Complex::new(x_transit, 0.0)).exp(); // exp(iψx')
    let h_gain = Complex::new(g_inlet, 0.0) * phase;
    let p_gain = complementary_pressure_stiffness(psi, ky, e_red) * h_gain;
    (h_gain, p_gain)
}

// ─────────────────────────────────────────────────────────────────────────
//  방향별 유효점도 (Eyring) — Poiseuille 항 반영
// ─────────────────────────────────────────────────────────────────────────

/// 모드 (kx,ky) 의 유효점도 [Pa·s].
///
/// Poiseuille 항 `∂ₓ((h³/12ηx)∂ₓp) + ∂_y((h³/12ηy)∂_y p)` 를 모드로 쓰면
/// `−(h³/12)·(kx²/ηx + ky²/ηy)·p`. 등가 스칼라 점도는
/// `η_mode = |k|² / (kx²/ηx + ky²/ηy)`.
/// - `ηx = η/√(1+(η·γ̇_s/τ0)²)` (미끄럼 전단 감소; γ̇_s=|Δu|/h0),
/// - `ηy = η` (횡방향 평균전단 없음).
///
/// 1D 횡거칠기(ky=0) → `η_mode=ηx`, 순수 y리지(kx=0) → `η_mode=ηy`.
fn directional_visc(eta_local: f64, gamma_s: f64, tau0: f64, kx: f64, ky: f64) -> f64 {
    let eta_x = eyring_eff_visc(eta_local, gamma_s, tau0);
    let eta_y = eta_local;
    let denom = kx * kx / eta_x + ky * ky / eta_y;
    if denom <= 0.0 {
        return eta_local;
    }
    (kx * kx + ky * ky) / denom
}

// ─────────────────────────────────────────────────────────────────────────
//  주파수 인덱스 → 물리 파수
// ─────────────────────────────────────────────────────────────────────────

/// FFT 인덱스 `a`(0..n) → 부호 파수 `k = 2π·m/L` [1/m]. (m: [-n/2, n/2) 로 접힘)
#[inline]
fn wavenumber(a: usize, n: usize, l: f64) -> f64 {
    let m = if a <= n / 2 {
        a as isize
    } else {
        a as isize - n as isize
    };
    2.0 * PI * m as f64 / l
}

// ─────────────────────────────────────────────────────────────────────────
//  M2 주 진입점
// ─────────────────────────────────────────────────────────────────────────

/// 완전윤활(EHL) 압력 리플 **특수해**를 계산해 [`LubResult`] 반환.
///
/// 절차: 복합거칠기 `s = rough1 + rough2`(평균 제거) → 2D FFT → 각 Fourier 모드에
/// 전달함수 `p_a/s_a`, `h_a/s_a` 적용 → 역FFT 로 압력/유막 리플장 재구성.
///
/// - `p_lub` = 압력 리플장 [Pa] (평균 0, 특수해분).
/// - `h_lub` = `h_bar` + 유막 리플장 [m].
///
/// 상보파(입출구 이동파)는 후속 Phase — 여기서는 국소 정상 특수해만.
pub fn solve_full_film(input: &PartialLubInput) -> LubResult {
    let nx = input.grid.nx;
    let ny = input.grid.ny;
    let n = nx * ny;

    // 퇴화 격자 방어.
    if nx == 0 || ny == 0 {
        return LubResult {
            p_lub: Field2::zeros(nx, ny),
            h_lub: Field2::filled(nx, ny, input.h_bar),
        };
    }

    let lx = input.grid.lx;
    let ly = input.grid.ly;
    let e_red = input.mat.e_red;
    let h0 = input.h_bar;
    let op = &input.op;

    // 국소 압점도(Roelands 포화형; RQ-3 해소) + 미끄럼 전단률. Barus exp(α·p) 는 운전점
    // (α·p≈30→exp(30)) 에서 비물리(η~1e11 Pa·s)이므로 포화형 Roelands 로 유계 평가
    // (원 논문 식[6] Eyring 운전점 η 계열; (30) Ehret).
    let eta_local = roelands_visc(op.eta0, op.alpha_visc, op.p_h);
    let d_u = (op.slide_roll * op.u_mean).abs(); // |Δu| = |SRR|·u_mean
    let gamma_s = if h0 > 0.0 { d_u / h0 } else { 0.0 };
    // 식[2] 특수해 대류속도 (u₂−ū) = −Δu/2 = −slide_roll·u_mean/2 (RQ-vel: 논문대로 정의).
    let u_conv = -0.5 * op.slide_roll * op.u_mean;
    let tau0 = op.tau0;
    let c_comp = 0.0_f64; // 비압축·등점도 극한(상보파 Phase 에서 C≠0 정밀화; RQ-2)

    // ── 상보파(complementary wave) 결선 파라미터 (ref(28) 식(25)~(30)) ──
    // 상보파 대류속도 = 평균속도 ū(op.u_mean) — 특수해 u_conv=(u₂−ū) 과 **분리**(2성분 핵심).
    // 방향점도비 V=η_x/η_y (모드무관: gamma_s·tau0·eta_local 만 의존) — 분산관계 식(30) 입력.
    let eta_x_visc = eyring_eff_visc(eta_local, gamma_s, tau0);
    let v_ratio = if eta_local > 0.0 { eta_x_visc / eta_local } else { 1.0 };
    let nu_over_u = if op.u_mean != 0.0 { op.u2 / op.u_mean } else { 1.0 };
    let g_inlet = complementary_inlet_ratio(); // 유입진폭비 g (G-M2-1: GW1994 0.5 + 민감도)
    // 유입→접촉중앙 전파거리 추정: 도메인(=접촉패치, RQ-M1-win 동일전제)의 절반.
    // ref(28) 감쇠 exp(−βx') 의 x'; b(반접촉폭) 미입력이라 격자기하로 소급(RQ-transit, G-M2-3).
    let x_transit = 0.5 * lx;

    // 복합 거칠기(간극 섭동 s) — 평균 제거.
    let mut s: Vec<Complex<f64>> = Vec::with_capacity(n);
    let mut mean = 0.0;
    for k in 0..n {
        let v = input.rough1.data[k] + input.rough2.data[k];
        mean += v;
    }
    mean /= n as f64;
    for k in 0..n {
        let v = input.rough1.data[k] + input.rough2.data[k] - mean;
        s.push(Complex::new(v, 0.0));
    }

    // 순방향 FFT (in-place).
    fft2_forward(&mut s, nx, ny);

    // 모드별 전달함수 적용 → 압력/유막 스펙트럼.
    let mut p_spec = vec![Complex::new(0.0, 0.0); n];
    let mut h_spec = vec![Complex::new(0.0, 0.0); n];
    for b in 0..ny {
        let ky = wavenumber(b, ny, ly);
        for a in 0..nx {
            let kx = wavenumber(a, nx, lx);
            let idx = a + b * nx;
            let k_mag = (kx * kx + ky * ky).sqrt();
            if k_mag <= 0.0 {
                // DC: 리플 0 (평균 유막은 h_bar 가 담당).
                continue;
            }
            let eta_mode = directional_visc(eta_local, gamma_s, tau0, kx, ky);
            // 식[2] 특수해 대류속도 = (u₂−ū) = −Δu/2 (위 u_conv). 상보파(ū 전파·비뉴턴
            // 감쇠, 식[7][8])는 별도 미구현 → 순수 rolling(u₂→ū) 근처 특수해 리플 과소예측
            // (잔여: 상보파 Phase). amplitude_q 계수24·|k|³ 는 q_independent_oracle_from_eq2 독립검증.
            let q = amplitude_q(eta_mode, u_conv, h0, kx, k_mag, e_red);
            // 성분①: 특수해(particular, u₂ 이동) — 전달함수 식(19)(20) [ref(28)].
            let p_part = pressure_ripple_transfer(k_mag, e_red, q, c_comp);
            let h_part = film_ripple_transfer(k_mag, e_red, q, c_comp);
            // 성분②: 상보파(complementary, ū 전파·β 감쇠) — 식(25)(28)(30) [ref(28)].
            // 분산관계 ψ 수렴 시 게인 합산, 비수렴(단파장 완전감쇠) 시 0(특수해 단독).
            let omega_c = complementary_wavenumber(kx, op.u2, op.u_mean);
            let (h_comp, p_comp) =
                match dispersion_psi(omega_c, ky, h0, e_red, tau0, v_ratio, nu_over_u) {
                    Some(psi) => complementary_wave(psi, ky, e_red, g_inlet, x_transit),
                    None => (Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)),
                };
            // 2성분 합성: p_spec=(p_part+p_comp)·s, h_spec=(h_part+h_comp)·s.
            let mut pt = p_part + p_comp;
            let mut ht = h_part + h_comp;
            // M2-4: x-Nyquist bin(짝수 nx, a=nx/2)은 ±kx 가 동일 bin 으로 접혀 kx 부호가
            // 소실 → Q(∝kx) 부호의존 전달함수가 자기켤레성을 잃어 p_spec 이 비-Hermitian.
            // 전달함수 실수부로 투영해 Hermitian 복원. (Re(IFFT)=IFFT(Herm) 이므로 최종 실수
            // 장 값은 불변이나, 중간 스펙트럼을 물리적으로 유효하게 유지해 재사용을 안전화.
            // 물리적으로도 sin(kx_nyq·x)≡0(격자점상) 이라 Im(T) 성분은 관측불가 → 실수부만
            // 유효. y-Nyquist(b=ny/2)는 Q 가 ky 부호에 무관(κ 만 의존)해 자연 Hermitian → 미처리.)
            if nx % 2 == 0 && a == nx / 2 {
                pt = Complex::new(pt.re, 0.0);
                ht = Complex::new(ht.re, 0.0);
            }
            p_spec[idx] = pt * s[idx];
            h_spec[idx] = ht * s[idx];
        }
    }

    // 역FFT → 실공간 리플.
    fft2_inverse(&mut p_spec, nx, ny);
    fft2_inverse(&mut h_spec, nx, ny);

    let mut p_lub = Field2::zeros(nx, ny);
    let mut h_lub = Field2::zeros(nx, ny);
    for k in 0..n {
        p_lub.data[k] = p_spec[k].re; // 압력 리플(특수해분)
        h_lub.data[k] = h0 + h_spec[k].re; // h_bar + 유막 리플
    }

    LubResult { p_lub, h_lub }
}

/// 부분윤활(mixed/partial) — 유막-아스페리티 하중분담은 **후속 Phase**(M2 범위 밖).
///
/// 인터페이스 안정성을 위해 유지하되, 현 Phase 에서는 완전유막 특수해를 전이장으로
/// 통과시키고 경계윤활 분율 `phi_bl=0` 을 반환한다(하중분담 결합은 잔여큐 RQ-1).
///
/// # ⚠️ 이 함수는 스텁이다 — `partial_lub::solve_partial` 과 혼동 금지
///
/// 두 함수는 **시그니처가 동일**해서 잘못 `use` 해도 조용히 컴파일된다. 그 결과
/// `phi_bl = 0` 이 M5 로 흘러가면 [`crate::m5_wear::wear_coefficient`] 의 건마모 항이
/// 사라져 **마모 없는 결과가 정상처럼 보인다**(무증상 실패). 실제 부분윤활 결합·마찰은
/// [`crate::partial_lub::solve_partial`] 이 담당한다.
///
/// `#[deprecated]` 는 **거동을 바꾸지 않는다** — 오사용을 컴파일 경고로 드러내
/// 함정을 문서가 아니라 컴파일러가 지키게 할 뿐이다(시각화 HTML 계획 R4).
#[deprecated(
    since = "0.1.0",
    note = "패스스루 스텁(phi_bl=0·q_tran=0). 실제 부분윤활은 partial_lub::solve_partial 사용. \
            phi_bl=0 이 M5 로 가면 건마모가 조용히 사라진다."
)]
pub fn solve_partial(input: &PartialLubInput) -> PartialLubResult {
    let full = solve_full_film(input);
    let (nx, ny) = (input.grid.nx, input.grid.ny);
    PartialLubResult {
        p_tran: full.p_lub,
        h_tran: full.h_lub,
        phi_bl: 0.0,
        // 하중분담 미결합 passthrough — 마찰 트랙션도 미산정(0). 실제 결합·마찰은
        // partial_lub::solve_partial 이 담당(M6 실결선).
        q_tran: Field2::zeros(nx, ny),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    fn dummy_op() -> OperatingConditions {
        OperatingConditions {
            p_h: 1.5e9,
            u_mean: 1.0,
            u2: 0.95, // = u_mean − slide_roll·u_mean/2 (u_mean·slide_roll 규약 정합)
            slide_roll: 0.1,
            eta0: 0.01,
            alpha_visc: 2e-8,
            tau0: 5e6,
            temp: 353.0,
            r_x: 0.02,
        }
    }

    fn dummy_mat() -> MaterialProps {
        MaterialProps {
            e_red: E_RED_STEEL_PA,
            nu: NU_STEEL,
            hardness: 7e9,
            p_lim: 4e9,
        }
    }

    // ── 1. 전달함수 크기: Q 에 대해 단조 증가하며 포화 (GW1994 정성) ──
    #[test]
    fn transfer_magnitude_monotone_and_saturates() {
        let kappa = 1.0e6; // 1/m (파장 ~6 μm)
        let e_red = E_RED_STEEL_PA;
        let c = 0.0;
        let sat = kappa * e_red / 2.0; // Q→∞ 포화값 (C=0)

        // Q 를 로그 스윕 (독립 변수로 직접 주입).
        let qs: Vec<f64> = (0..60).map(|i| 10f64.powf(-4.0 + 0.15 * i as f64)).collect();
        let mut prev = 0.0;
        for &q in &qs {
            let mag = pressure_ripple_transfer(kappa, e_red, q, c).norm();
            // 단조 비감소.
            assert!(
                mag >= prev - sat * 1e-9,
                "non-monotone: q={q}, mag={mag}, prev={prev}"
            );
            // 포화값 이하.
            assert!(mag <= sat * (1.0 + 1e-9), "exceeds saturation: q={q}, mag={mag}");
            prev = mag;
        }

        // Q→0 극한: 0.
        let mag_small = pressure_ripple_transfer(kappa, e_red, 1e-6, c).norm();
        assert!(mag_small < sat * 1e-3, "Q→0 should vanish: {mag_small}");

        // Q→∞ 극한: 포화값 접근 (C=0 → sat).
        let mag_big = pressure_ripple_transfer(kappa, e_red, 1e6, c).norm();
        assert!(
            (mag_big - sat).abs() < sat * 1e-3,
            "Q→∞ should saturate at κE/2: mag_big={mag_big}, sat={sat}"
        );
    }

    // ── 2. Q=0 에서 압력 리플 0 ──
    #[test]
    fn transfer_zero_at_zero_q() {
        let t = pressure_ripple_transfer(1e6, E_RED_STEEL_PA, 0.0, 0.0);
        assert!(t.norm() < 1e-6);
    }

    // ── 3. 유막 진폭감소: |h_a/s_a| = 1/√(1+Q²) (C=0), Q 증가 시 감소, ≤1 ──
    #[test]
    fn film_amplitude_reduction() {
        let kappa = 1.0e6;
        let e_red = E_RED_STEEL_PA;
        let mut prev = 2.0;
        for i in 0..40 {
            let q = 10f64.powf(-2.0 + 0.1 * i as f64);
            let mag = film_ripple_transfer(kappa, e_red, q, 0.0).norm();
            let expect = 1.0 / (1.0 + q * q).sqrt();
            assert!((mag - expect).abs() < 1e-9, "q={q}: {mag} vs {expect}");
            assert!(mag <= 1.0 + 1e-9, "reduction ratio >1: q={q}");
            assert!(mag <= prev + 1e-12, "not monotone decreasing: q={q}");
            prev = mag;
        }
        // Q→0 → 1 (원거칠기 통과), Q→∞ → 0 (평활화).
        assert!((film_ripple_transfer(kappa, e_red, 1e-4, 0.0).norm() - 1.0).abs() < 1e-6);
        assert!(film_ripple_transfer(kappa, e_red, 1e4, 0.0).norm() < 1e-3);
    }

    // ── 4. Eyring 방향별 유효점도: 미끄럼전단 → 점도 감소, 무전단 → 불변 ──
    #[test]
    fn eyring_directional_viscosity() {
        let eta = 100.0; // Pa·s (압점도로 커진 국소점도 가정)
        let tau0 = 5e6;
        // 전단률 0 → η 유지.
        assert!((eyring_eff_visc(eta, 0.0, tau0) - eta).abs() < 1e-12);
        // 전단률 증가 → 단조 감소, < η.
        let g1 = 1.0e5;
        let g2 = 1.0e6;
        let e1 = eyring_eff_visc(eta, g1, tau0);
        let e2 = eyring_eff_visc(eta, g2, tau0);
        assert!(e1 < eta && e2 < e1, "eta={eta}, e1={e1}, e2={e2}");
        // 방향성: 횡(ky만) = η_local, 종(kx만) = η_x(<η).
        let gamma_s = 1.0e6;
        let eta_x = directional_visc(eta, gamma_s, tau0, 1.0e6, 0.0);
        let eta_y = directional_visc(eta, gamma_s, tau0, 0.0, 1.0e6);
        assert!((eta_y - eta).abs() < 1e-9, "transverse should keep η");
        assert!(eta_x < eta, "longitudinal should be reduced");
    }

    // ── 5. 필드 재구성: 단일 정현파 거칠기 → 그 모드의 리플 진폭이 해석 전달함수와 일치 ──
    #[test]
    fn field_reconstruction_single_mode() {
        let nx = 64usize;
        let ny = 4usize;
        let lx = 1.0e-4; // 100 μm
        let ly = 1.0e-4;
        // 모드 수 m=4 (Nyquist 아님), kx = 2π·4/lx.
        let m = 4.0;
        let kx = 2.0 * PI * m / lx;
        let k_mag = kx; // ky=0 (횡거칠기)
        let r_a = 1.0e-7; // 0.1 μm 진폭

        let mut r1 = Field2::zeros(nx, ny);
        for b in 0..ny {
            for a in 0..nx {
                let x = a as f64 * (lx / nx as f64);
                r1.set(a, b, r_a * (kx * x).cos());
            }
        }
        let input = PartialLubInput {
            grid: Grid::new(nx, ny, lx, ly),
            rough1: r1,
            rough2: Field2::zeros(nx, ny),
            mat: dummy_mat(),
            op: dummy_op(),
            h_bar: 1.4e-7,
        };
        let res = solve_full_film(&input);

        // 해석 기대값: 모드에 쓰인 것과 동일한 파라미터로 **2성분 전달함수** 재계산(Roelands).
        let eta_local = roelands_visc(input.op.eta0, input.op.alpha_visc, input.op.p_h);
        let gamma_s = (input.op.slide_roll * input.op.u_mean).abs() / input.h_bar;
        let eta_mode = directional_visc(eta_local, gamma_s, input.op.tau0, kx, 0.0);
        let u_conv = -0.5 * input.op.slide_roll * input.op.u_mean; // 식[2] (u₂−ū)
        let q = amplitude_q(eta_mode, u_conv, input.h_bar, kx, k_mag, input.mat.e_red);
        // 성분① 특수해.
        let p_part = pressure_ripple_transfer(k_mag, input.mat.e_red, q, 0.0);
        let h_part = film_ripple_transfer(k_mag, input.mat.e_red, q, 0.0);
        // 성분② 상보파(solve_full_film 과 동일 결선).
        let eta_x_visc = eyring_eff_visc(eta_local, gamma_s, input.op.tau0);
        let v_ratio = eta_x_visc / eta_local;
        let nu_over_u = input.op.u2 / input.op.u_mean;
        let omega_c = complementary_wavenumber(kx, input.op.u2, input.op.u_mean);
        let (h_comp, p_comp) = match dispersion_psi(
            omega_c,
            0.0,
            input.h_bar,
            input.mat.e_red,
            input.op.tau0,
            v_ratio,
            nu_over_u,
        ) {
            Some(psi) => complementary_wave(
                psi,
                0.0,
                input.mat.e_red,
                complementary_inlet_ratio(),
                0.5 * lx,
            ),
            None => (Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)),
        };
        // 상보파가 실제로 유효(수렴·비영)해 검출력 확보 확인.
        assert!(p_comp.norm() > 0.0, "complementary must be active in this mode");
        let p_gain = (p_part + p_comp).norm();
        let h_gain = (h_part + h_comp).norm();

        // 단일 정현파 진폭 = √2·RMS (Parseval, 정수주기 → 샘플링 오차 없이 정확).
        let n = res.p_lub.len() as f64;
        let p_rms = (res.p_lub.data.iter().map(|v| v * v).sum::<f64>() / n).sqrt();
        let p_amp = std::f64::consts::SQRT_2 * p_rms;
        let p_expect = p_gain * r_a;
        assert!(
            (p_amp - p_expect).abs() <= p_expect.max(1.0) * 1e-9,
            "pressure ripple amp: got {p_amp}, expect {p_expect}"
        );

        // 유막 리플 진폭 = √2·RMS(h − h_bar).
        let h_rms = (res
            .h_lub
            .data
            .iter()
            .map(|v| (v - input.h_bar) * (v - input.h_bar))
            .sum::<f64>()
            / n)
            .sqrt();
        let h_amp = std::f64::consts::SQRT_2 * h_rms;
        let h_expect = h_gain * r_a;
        assert!(
            (h_amp - h_expect).abs() <= h_expect.max(1e-12) * 1e-6,
            "film ripple amp: got {h_amp}, expect {h_expect}"
        );
        // 평균 유막은 h_bar (DC=0, 상보파도 DC 미기여).
        let h_mean: f64 = res.h_lub.data.iter().sum::<f64>() / res.h_lub.len() as f64;
        assert!((h_mean - input.h_bar).abs() < input.h_bar * 1e-9);
        // 물리성: 2성분(특수해 u₂-이동 + 상보파 ū-전파)이 한 스냅샷에서 중첩되므로 순간
        // 유막 리플은 원거칠기보다 크거나 작을 수 있다(GW1994 Fig.6 beating). 유계만 확인.
        assert!(h_amp.is_finite() && h_amp > 0.0, "film ripple must be finite/positive");
    }

    // ── 6. 인터페이스 유지: solve_partial 통과 + phi_bl=0 ──
    //
    // 이 테스트는 **스텁이 스텁임을 고정**하는 것이 목적(phi_bl=0 을 명시적으로 assert)이므로
    // deprecated 경고를 의도적으로 허용한다. 다른 곳에서의 사용은 경고로 드러나야 한다.
    #[test]
    #[allow(deprecated)]
    fn partial_passthrough() {
        let input = PartialLubInput {
            grid: Grid::new(8, 8, 1e-4, 1e-4),
            rough1: Field2::zeros(8, 8),
            rough2: Field2::zeros(8, 8),
            mat: dummy_mat(),
            op: dummy_op(),
            h_bar: 1.4e-7,
        };
        let r = solve_partial(&input);
        assert_eq!(r.phi_bl, 0.0);
        // 거칠기 0 → 리플 0 → 유막 = h_bar.
        assert!((r.h_tran.max().unwrap() - 1.4e-7).abs() < 1e-15);
    }

    // ── M2-1(치명): Q 독립 오라클 — 식[2]/[5] 손유도. 검증함수로 기대값 생성 금지. ──
    //
    // 3-4-5 파수(kx≠|k|)로 계수 24 와 지수 |k|³ 를 동시에 강제한다.
    // 손계산(식[5] 부속 Q정의, 48/E' 경로, E'=2·E_red):
    //   Q = [48·V·ωx/(E'·h³·κ)] / (κ²/η_mode) = 48·η·V·kx/(E'·h0³·|k|³)
    //     = 24·η·V·kx/(E_red·h0³·|k|³).
    // 수치: η=2, V=3, h0=1e-7, kx=3e5, |k|=5e5(=√(3e5²+4e5²)), E_red=1e11, E'=2e11.
    //   분자 48·3·3e5·2 = 8.64e7;  분모 E'·h0³·|k|³ = 2e11·1e-21·1.25e17 = 2.5e7.
    //   Q = 8.64e7/2.5e7 = 3.456  (독립 손계산 하드코딩; amplitude_q 미호출).
    #[test]
    fn q_independent_oracle_from_eq2() {
        let eta = 2.0;
        let v = 3.0;
        let h0 = 1.0e-7;
        let kx = 3.0e5;
        let k_mag = 5.0e5;
        let e_red = 1.0e11;
        let q_hand = 3.456_f64; // 식[2]/[5] 손유도값

        let q = amplitude_q(eta, v, h0, kx, k_mag, e_red);
        assert!(
            (q - q_hand).abs() <= q_hand.abs() * 1e-12,
            "Q coeff/exponent mismatch vs eq2 hand-derivation: got {q}, hand {q_hand}"
        );
        // 지수 |k|³ 강제: kx 고정, |k| 2배 → Q ∝ |k|⁻³ → 1/8 (|k|² 오구현이면 1/4 로 검출).
        let q2 = amplitude_q(eta, v, h0, kx, 2.0 * k_mag, e_red);
        assert!(
            (q2 - q / 8.0).abs() <= (q / 8.0).abs() * 1e-12,
            "|k|³ exponent not enforced: q2={q2}, expect q/8={}",
            q / 8.0
        );
        // 계수 24(=48/E') 강제: E_red 2배 → Q 1/2 (계수 12 오구현이면 절대크기 절반으로 검출).
        let q3 = amplitude_q(eta, v, h0, kx, k_mag, 2.0 * e_red);
        assert!((q3 - q / 2.0).abs() <= (q / 2.0).abs() * 1e-12, "1/E_red scaling broken");
        // kx 선형성(구름방향 성분): kx 2배 → Q 2배.
        let q4 = amplitude_q(eta, v, h0, 2.0 * kx, k_mag, e_red);
        assert!((q4 - 2.0 * q).abs() <= (2.0 * q).abs() * 1e-12, "kx linearity broken");
    }

    // ── M2-2: |p_a/r_a| vs Q 닫힌형(식[5], C=0) 독립 대조 [(29)GW1994/(31)Venner-Lubrecht]. ──
    //
    // 식[5] C=0: p_a/r_a = (κE'/4)·iQ/(1−iQ) ; |·| = (κE'/4)·|Q|/√(1+Q²).
    // (κE'/4)=(κE_red/2). 독립 닫힌형(복소구현 미사용)으로 여러 Q 점 대조.
    #[test]
    fn pressure_amplitude_reduction_closed_form() {
        let kappa = 1.0e6;
        let e_red = 1.0e11;
        let pref = kappa * e_red / 2.0; // κE'/4
        for &q in &[0.25_f64, 1.0, 4.0, 25.0] {
            let closed = pref * q.abs() / (1.0 + q * q).sqrt(); // 식[5] 닫힌형(독립)
            let got = pressure_ripple_transfer(kappa, e_red, q, 0.0).norm();
            assert!(
                (got - closed).abs() <= closed * 1e-12,
                "|p_a/r_a| closed-form mismatch @Q={q}: got {got}, closed {closed}"
            );
        }
        // (29)/(31) 진폭감소 대표점: 유막비 |h_a/r_a|=1/√(1+Q²), Q=1 → 1/√2.
        let hr = film_ripple_transfer(kappa, e_red, 1.0, 0.0).norm();
        assert!((hr - 1.0 / std::f64::consts::SQRT_2).abs() < 1e-12, "amp-reduction @Q=1 off");
    }

    // ── M2-3: 압축성 C(식[4] `ρ_a=(ρ/B)p_a` → C=hE'κ/(4B)) 민감도 (=B 스윕 등가). ──
    //
    // |p_a/r_a| = (κE'/4)·Q/√(1+((1+C)Q)²)  → C↑ 시 단조감소.
    // |h_a/r_a| = √(1+(CQ)²)/√(1+((1+C)Q)²) → C↑ 시 단조증가(→1), ≤1.
    // (C=hE'κ/(4B) 이므로 C-스윕은 체적탄성 B 스윕과 일대일; B→∞ ⇒ C→0.)
    #[test]
    fn compressibility_c_sensitivity() {
        let kappa = 1.0e6;
        let e_red = 1.0e11;
        let q = 3.0;
        let pref = kappa * e_red / 2.0;
        let cs = [0.0_f64, 0.25, 0.5, 1.0, 2.0, 4.0];
        let mut prev_p = f64::INFINITY;
        let mut prev_h = 0.0_f64;
        for &c in &cs {
            let pm = pressure_ripple_transfer(kappa, e_red, q, c).norm();
            let hm = film_ripple_transfer(kappa, e_red, q, c).norm();
            // 독립 닫힌형 대조.
            let p_closed = pref * q / (1.0 + ((1.0 + c) * q).powi(2)).sqrt();
            let h_closed =
                (1.0 + (c * q).powi(2)).sqrt() / (1.0 + ((1.0 + c) * q).powi(2)).sqrt();
            assert!((pm - p_closed).abs() <= p_closed * 1e-12, "p closed-form @C={c}");
            assert!((hm - h_closed).abs() <= h_closed * 1e-12, "h closed-form @C={c}");
            // 민감도 방향(단조) + 물리 경계.
            assert!(pm <= prev_p + p_closed * 1e-12, "|p_a/r_a| not decreasing in C @C={c}");
            assert!(hm >= prev_h - 1e-12, "|h_a/r_a| not increasing in C @C={c}");
            assert!(hm <= 1.0 + 1e-12, "|h_a/r_a|>1 @C={c}");
            prev_p = pm;
            prev_h = hm;
        }
        // 유의 민감도: C=4 에서 유막 감소가 완화(|h| 증가), 상한 1 미만 유지.
        let h_c0 = film_ripple_transfer(kappa, e_red, q, 0.0).norm();
        let h_c4 = film_ripple_transfer(kappa, e_red, q, 4.0).norm();
        assert!(h_c4 > h_c0 && h_c4 < 1.0, "C-sensitivity bound: h0={h_c0}, h4={h_c4}");
    }

    // ── M2-3: Roelands 포화형 — 저압 Barus 정합 + 고압 유계(RQ-3 해소). ──
    #[test]
    fn roelands_bounded_and_low_p_matches_barus() {
        // 저압(p=1e6): Roelands ≈ Barus (Z=α·c_p/(ln η0+9.67) 로 기울기 1차 정합).
        let r_lo = roelands_visc(0.01, 2e-8, 1.0e6);
        let b_lo = barus_visc(0.01, 2e-8, 1.0e6);
        assert!(
            (r_lo - b_lo).abs() / b_lo < 1e-2,
            "Roelands should match Barus at low p: {r_lo} vs {b_lo}"
        );
        // 고압 운전점(1.5 GPa): 유계 & Barus exp(30) 폭주보다 훨씬 작음.
        let r_hi = roelands_visc(0.01, 2e-8, 1.5e9);
        let b_hi = barus_visc(0.01, 2e-8, 1.5e9); // exp(30)≈1.07e11 Pa·s
        assert!(r_hi.is_finite(), "Roelands η must be finite");
        assert!(r_hi < b_hi, "Roelands must be below Barus blowup: {r_hi} vs {b_hi}");
        assert!(r_hi < 1.0e9, "Roelands η far below Barus exp(30): {r_hi}");
        // 단조 증가.
        let r_mid = roelands_visc(0.01, 2e-8, 0.5e9);
        assert!(r_lo < r_mid && r_mid < r_hi, "Roelands not monotonic in p: {r_lo},{r_mid},{r_hi}");
    }

    // ── M2-4: 짝수격자 x-Nyquist Hermitian 처리 회귀. ──
    //
    // x-Nyquist 거칠기 r = r_a·(−1)^i·cos(ky·y) (y 는 비-Nyquist). 실수투영된 전달함수
    // 닫힌형 `Re(T)·r_a·(−1)^i·cos(ky y)` 와 재구성장이 일치해야 한다(전 격자점).
    #[test]
    fn nyquist_hermitian_regression() {
        let nx = 8usize; // 짝수 → x-Nyquist m=4
        let ny = 8usize;
        let lx = 1.0e-4;
        let ly = 1.0e-4;
        let kx = 2.0 * PI * (nx as f64 / 2.0) / lx; // x-Nyquist 파수
        let my = 2.0;
        let ky = 2.0 * PI * my / ly; // 비-Nyquist y
        let k_mag = (kx * kx + ky * ky).sqrt();
        let r_a = 1.0e-8;

        let mut r1 = Field2::zeros(nx, ny);
        for b in 0..ny {
            let y = b as f64 * (ly / ny as f64);
            for a in 0..nx {
                let sign = if a % 2 == 0 { 1.0 } else { -1.0 }; // (−1)^i = cos(kx_nyq·x_i)
                r1.set(a, b, r_a * sign * (ky * y).cos());
            }
        }
        let input = PartialLubInput {
            grid: Grid::new(nx, ny, lx, ly),
            rough1: r1,
            rough2: Field2::zeros(nx, ny),
            mat: dummy_mat(),
            op: dummy_op(),
            h_bar: 1.4e-7,
        };
        let res = solve_full_film(&input);

        // 유한성.
        assert!(res.p_lub.data.iter().all(|v| v.is_finite()), "p_lub non-finite");
        assert!(res.h_lub.data.iter().all(|v| v.is_finite()), "h_lub non-finite");

        // 기대: x-Nyquist 실수투영 → p = Re(T)·r_a·(−1)^i·cos(ky y), T=특수해+상보파.
        let eta_local = roelands_visc(input.op.eta0, input.op.alpha_visc, input.op.p_h);
        let gamma_s = (input.op.slide_roll * input.op.u_mean).abs() / input.h_bar;
        let eta_mode = directional_visc(eta_local, gamma_s, input.op.tau0, kx, ky);
        let u_conv = -0.5 * input.op.slide_roll * input.op.u_mean; // 식[2] (u₂−ū)
        let q = amplitude_q(eta_mode, u_conv, input.h_bar, kx, k_mag, input.mat.e_red);
        let p_part = pressure_ripple_transfer(k_mag, input.mat.e_red, q, 0.0);
        // 상보파(solve_full_film 동일 결선).
        let eta_x_visc = eyring_eff_visc(eta_local, gamma_s, input.op.tau0);
        let v_ratio = eta_x_visc / eta_local;
        let nu_over_u = input.op.u2 / input.op.u_mean;
        let omega_c = complementary_wavenumber(kx, input.op.u2, input.op.u_mean);
        let (_h_comp, p_comp) = match dispersion_psi(
            omega_c,
            ky,
            input.h_bar,
            input.mat.e_red,
            input.op.tau0,
            v_ratio,
            nu_over_u,
        ) {
            Some(psi) => complementary_wave(
                psi,
                ky,
                input.mat.e_red,
                complementary_inlet_ratio(),
                0.5 * lx,
            ),
            None => (Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)),
        };
        let t_full = p_part + p_comp; // 2성분 총 전달함수
        let t_re = t_full.re; // 실수투영분
        // Nyquist 는 자명 실수 아님(Im(T)≠0 → 실수투영이 반드시 필요).
        assert!(t_full.im.abs() > t_re.abs() * 1e-6, "test setup: T must be complex at Nyquist");

        let mut max_err = 0.0_f64;
        for b in 0..ny {
            let y = b as f64 * (ly / ny as f64);
            for a in 0..nx {
                let sign = if a % 2 == 0 { 1.0 } else { -1.0 };
                let expect = t_re * r_a * sign * (ky * y).cos();
                let got = res.p_lub.at(a, b);
                max_err = max_err.max((got - expect).abs());
            }
        }
        // 진폭 스케일 = |Re(T)|·r_a.
        let scale = (t_re * r_a).abs().max(1e-30);
        assert!(
            max_err <= scale * 1e-9,
            "nyquist field mismatch vs real-projected closed form: max_err={max_err}, scale={scale}"
        );
    }

    // ════════════════════════════════════════════════════════════════════
    //  상보파(complementary wave) 오라클 — §8.A (독립·실패가능·비-tautology)
    // ════════════════════════════════════════════════════════════════════

    // ── §8.A(e) 분산관계 독립 오라클 — 수렴 ψ 가 식(30)[ref(28)] 잔차 0 을 만족 ──
    //
    // 검증함수로 기대값을 만들지 않는다: 수렴한 ψ 를 식(30) RHS 에 **재대입**해 ψ 로
    // 돌아오는지(고정점 잔차<tol) 독립 확인. + 순수rolling·뉴턴 극한 β=0, 단파장 완전감쇠.
    #[test]
    fn dispersion_psi_satisfies_eq30() {
        let e_red = 1.0e11;
        let e_prime = 2.0 * e_red;
        let tau_e = 5.0e6;
        let h = 1.4e-7;
        let v_ratio = 0.3;
        let nu_over_u = 0.5; // ν/u = 0.5 (강한 미끄럼)
        let ky = 0.0;
        // 중간 파장(수렴역): kx 로 ω_c 설정.
        let kx = 1.2e5;
        let omega_c = kx * nu_over_u;
        let psi = dispersion_psi(omega_c, ky, h, e_red, tau_e, v_ratio, nu_over_u)
            .expect("mid-wavelength dispersion should converge");
        // β>0(비뉴턴 감쇠 존재).
        assert!(psi.im > 0.0, "damping β must be positive: psi={psi:?}");
        // 식(30) 잔차 재대입(독립): ψ_recon = ω_c + i·coef·Ω·(ψ²+Vξ²), Ω=√(ψ²+ξ²).
        let coef = (e_prime / tau_e) * (h * h / 24.0) * (1.0 - nu_over_u).abs();
        let big_omega = (psi * psi + Complex::new(ky * ky, 0.0)).sqrt();
        let bracket = psi * psi + Complex::new(v_ratio * ky * ky, 0.0);
        let recon = Complex::new(omega_c, 0.0) + Complex::new(0.0, coef) * big_omega * bracket;
        assert!(
            (recon - psi).norm() <= 1e-5 * psi.norm().max(1.0),
            "eq(30) residual not satisfied: psi={psi:?}, recon={recon:?}"
        );
        // 순수 rolling(ν/u=1 → slip=0): β=0(감쇠 없음, 특수해 무 → 상보파 무감쇠 전파).
        let psi_roll = dispersion_psi(omega_c, ky, h, e_red, tau_e, v_ratio, 1.0).unwrap();
        assert!(psi_roll.im.abs() < 1e-30, "pure rolling must give β=0: {psi_roll:?}");
        // 뉴턴(τ_e→0): β=0.
        let psi_newt = dispersion_psi(omega_c, ky, h, e_red, 0.0, v_ratio, nu_over_u).unwrap();
        assert!(psi_newt.im.abs() < 1e-30, "Newtonian must give β=0: {psi_newt:?}");
        // 단파장(고감쇠, ψ³ 고정점 발산) → None(완전감쇠 취급).
        let big_kx = 5.0e7;
        let none = dispersion_psi(big_kx * nu_over_u, ky, h, e_red, tau_e, v_ratio, nu_over_u);
        assert!(none.is_none(), "short wavelength must diverge→None(full damping)");
    }

    // ── §8.A 상보파 표면강성 식(28): p_c/h_c=ΩE'/4=Ω·E_red/2 + k→−k 켤레대칭 ──
    #[test]
    fn complementary_stiffness_eq28() {
        let e_red = 1.0e11;
        // ψ 실수(뉴턴): Ω=√(ψ²+ky²).
        let psi = Complex::new(1.0e6, 0.0);
        let ky = 5.0e5;
        let s = complementary_pressure_stiffness(psi, ky, e_red);
        let omega = ((psi * psi + Complex::new(ky * ky, 0.0)).sqrt()).re;
        let expect = omega * e_red / 2.0; // Ω·E_red/2 = Ω·E'/4
        assert!((s.re - expect).abs() <= expect * 1e-12, "stiffness≠ΩE'/4: {s:?} vs {expect}");
        // k→−k: ψ→−conj(ψ), ky→−ky → 게인 켤레대칭(실공간 실수성).
        let psi_c = Complex::new(3.0e5, 4.0e4);
        let (h1, p1) = complementary_wave(psi_c, ky, e_red, 0.5, 2.0e-5);
        let (h2, p2) = complementary_wave(-psi_c.conj(), -ky, e_red, 0.5, 2.0e-5);
        assert!((h2 - h1.conj()).norm() <= h1.norm() * 1e-9, "h_comp not conjugate-symmetric");
        assert!((p2 - p1.conj()).norm() <= p1.norm() * 1e-9, "p_comp not conjugate-symmetric");
    }

    // ── §8.A(c) 순수 rolling 압력 리플 non-zero (치명·구조적 검출력) ──
    //
    // slide_roll→0: 특수해 Q∝(u₂−ū)→0 → p_part→0. 물리적으로 압력 리플은 존재하며
    // **상보파가 담당**(GW1994 Fig.7b). 총 압력 리플>0 을 강제하고, 상보파 없이(특수해 단독)
    // 이면 ~0 임을 대조해 검출력을 영구 고정(변이게이트 (e) 의 permanent 형).
    #[test]
    fn pure_rolling_pressure_ripple_nonzero() {
        let nx = 64usize;
        let ny = 4usize;
        let lx = 1.0e-4;
        let ly = 1.0e-4;
        let m = 4.0;
        let kx = 2.0 * PI * m / lx;
        let k_mag = kx;
        let r_a = 1.0e-7;

        let mut r1 = Field2::zeros(nx, ny);
        for b in 0..ny {
            for a in 0..nx {
                let x = a as f64 * (lx / nx as f64);
                r1.set(a, b, r_a * (kx * x).cos());
            }
        }
        // 순수 rolling(정확): slide_roll=0 → u₂=ū. 특수해 u_conv=(u₂−ū)=0 → Q=0 → p_part=0
        // 정확히 소멸(η 운전점이 커도 Q∝(u₂−ū)=0). 상보파는 slip=0 → β=0 무감쇠 전파.
        let u_mean = 1.0;
        let mut op = dummy_op();
        op.slide_roll = 0.0;
        op.u_mean = u_mean;
        op.u2 = u_mean;
        let input = PartialLubInput {
            grid: Grid::new(nx, ny, lx, ly),
            rough1: r1,
            rough2: Field2::zeros(nx, ny),
            mat: dummy_mat(),
            op,
            h_bar: 1.4e-7,
        };
        let res = solve_full_film(&input);
        let n = res.p_lub.len() as f64;
        let p_rms = (res.p_lub.data.iter().map(|v| v * v).sum::<f64>() / n).sqrt();

        // 특수해 단독 게인(대조): Q→0 → p_part→0.
        let eta_local = roelands_visc(input.op.eta0, input.op.alpha_visc, input.op.p_h);
        let gamma_s = (input.op.slide_roll * input.op.u_mean).abs() / input.h_bar;
        let eta_mode = directional_visc(eta_local, gamma_s, input.op.tau0, kx, 0.0);
        let u_conv = -0.5 * input.op.slide_roll * input.op.u_mean;
        let q = amplitude_q(eta_mode, u_conv, input.h_bar, kx, k_mag, input.mat.e_red);
        let p_part = pressure_ripple_transfer(k_mag, input.mat.e_red, q, 0.0).norm();
        let p_part_amp = p_part * r_a; // 특수해 단독이면 이 값이 진폭

        // 상보파 게인(담당).
        let omega_c = complementary_wavenumber(kx, input.op.u2, input.op.u_mean);
        let (_h, p_comp) = dispersion_psi(
            omega_c, 0.0, input.h_bar, input.mat.e_red, input.op.tau0,
            eyring_eff_visc(eta_local, gamma_s, input.op.tau0) / eta_local,
            input.op.u2 / input.op.u_mean,
        )
        .map(|psi| complementary_wave(psi, 0.0, input.mat.e_red, complementary_inlet_ratio(), 0.5 * lx))
        .unwrap();
        let p_comp_amp = p_comp.norm() * r_a;

        // (1) 특수해 단독은 압력 리플 소멸(검출력): p_part_amp ≪ p_comp_amp.
        assert!(
            p_part_amp < p_comp_amp * 1e-3,
            "detection: particular-only ripple must vanish in pure rolling: part={p_part_amp}, comp={p_comp_amp}"
        );
        // (2) 총 압력 리플(상보파 담당)은 non-zero: √2·RMS ≈ p_comp_amp.
        let p_amp = std::f64::consts::SQRT_2 * p_rms;
        assert!(p_amp > 0.0, "total pressure ripple must be non-zero in pure rolling");
        assert!(
            (p_amp - p_comp_amp).abs() <= p_comp_amp * 1e-6,
            "pure-rolling ripple must equal complementary contribution: got {p_amp}, comp {p_comp_amp}"
        );
    }

    // ── §8.A(a) VC-M2-AR — GW1994 식(15)(16)(17) 진폭감소 닫힌형 [ref(29)] ──
    //
    // 독립 닫힌형(코드 전달함수 미사용): A=2λ/(παh̄E'), P₁/Z₁=−1/(C+A), H₁/Z₁=C/(C+A),
    // Aₙ=A/n. 물리 극한(단파장→거칠기 지속, 장파장→평탄화) + Aₙ 스케일 강제.
    #[test]
    fn vc_m2_ar_gw1994_closed_form() {
        let e_prime = 2.0 * E_RED_STEEL_PA; // GW1994 E'(=2·E_red)
        let alpha = 12.0e-9; // 압점도 [1/Pa]
        let hbar = 0.3e-6; // 평균유막 [m]
        let c = 0.03_f64; // 압축성항(대표값; C=hE'κ/(4B) 계열, 여기선 파라미터로 고정)
        // 식(15) A — reference(leaf) 소유.
        let a_of = |lam: f64| crate::reference::gw1994_a(lam, alpha, hbar, e_prime);
        // 파장 스윕: 단파장→장파장.
        let lams = [1e-5, 3e-5, 1e-4, 3e-4, 1e-3];
        let mut prev_h = -1.0_f64;
        let mut prev_p = f64::INFINITY;
        for &lam in &lams {
            let a = a_of(lam);
            let h_ratio = crate::reference::gw1994_h1_over_z1(c, a); // 식(16) |H₁/Z₁|
            let p_ratio = crate::reference::gw1994_p1_over_z1(c, a).abs(); // 식(15) |P₁/Z₁|
            // 물리: 장파장(A↑) → H₁/Z₁↓(평탄화), P₁/Z₁↓.
            if prev_h >= 0.0 {
                assert!(h_ratio < prev_h, "H₁/Z₁ must decrease with λ(A↑): λ={lam}");
                assert!(p_ratio < prev_p, "P₁/Z₁ must decrease with λ(A↑): λ={lam}");
            }
            assert!(h_ratio > 0.0 && h_ratio < 1.0, "0<H₁/Z₁<1 required");
            prev_h = h_ratio;
            prev_p = p_ratio;
        }
        // 단파장 극한 A→0: H₁/Z₁→1(거칠기 지속), 장파장 A→∞: →0(평탄화).
        let a_tiny = a_of(1e-7);
        let a_huge = a_of(1.0);
        assert!(
            crate::reference::gw1994_h1_over_z1(c, a_tiny) > 0.99,
            "short-λ: roughness should persist (H₁/Z₁→1)"
        );
        assert!(
            crate::reference::gw1994_h1_over_z1(c, a_huge) < 0.01,
            "long-λ: roughness flattened (H₁/Z₁→0)"
        );
        // 식(17) Aₙ=A/n: n 성분 파수 λ/n → A_n = A/n(선형).
        let a1 = a_of(4e-4);
        let a4 = a_of(4e-4 / 4.0);
        assert!((a4 - a1 / 4.0).abs() <= (a1 / 4.0) * 1e-12, "Aₙ=A/n scaling broken");
    }

    // ── §8.A(c) VC-M2-Spot — GW1994 Table 1 정량 [ref(29)] (독립·零자유계수) ──
    //
    // ★ 정직: GW1994 Table 1(50/80℃ 스팟)은 **정상상태(특수해) 진폭감소**로, 상보파(전이해,
    // Fig.7~9)와 무관하다. 검증: 기하 A_data=2λ/(παh̄E') 와, 두 스팟(P₁,h₁)에서 C 를 소거해
    // 얻은 A_inferred=(z₁−h₁)/(αp₁h̄) 를 대조(식(15)(16) 결합). **자유계수 0**(C 소거) ·
    // 특수해 전달함수 형태를 문헌 실측치로 독립 검증. (상보파 진폭 g 는 별도 gap G-M2-1.)
    #[test]
    fn vc_m2_spot_gw1994_table1() {
        // Table 2 입력 + Table 1 "Present theory" 스팟 — 전부 reference(leaf) 소유.
        use crate::reference::{
            gw1994_a, GW1994_E_PRIME_PA, GW1994_LAMBDA_M, GW1994_TABLE1_PRESENT,
            GW1994_TABLE2_KWEH, GW1994_Z1_M,
        };
        let e_prime = GW1994_E_PRIME_PA;
        let lam = GW1994_LAMBDA_M;
        let z1 = GW1994_Z1_M;
        let cases = [
            (GW1994_TABLE2_KWEH[0].0, GW1994_TABLE2_KWEH[0].1, GW1994_TABLE1_PRESENT[0].0, GW1994_TABLE1_PRESENT[0].1), // 50℃
            (GW1994_TABLE2_KWEH[1].0, GW1994_TABLE2_KWEH[1].1, GW1994_TABLE1_PRESENT[1].0, GW1994_TABLE1_PRESENT[1].1), // 80℃
        ];
        for (alpha, hbar, p1, h1) in cases {
            let a_data = gw1994_a(lam, alpha, hbar, e_prime); // 기하 A(식15)
            let a_inf = (z1 - h1) / (alpha * p1 * hbar); // 스팟서 C 소거한 A
            let rel = (a_data - a_inf).abs() / a_data;
            // 대조원 차등오차 내(<1%). 스팟 정량 재현 → 특수해 형태 검증.
            assert!(
                rel < 1.0e-2,
                "VC-M2-Spot: A_data={a_data:.5}, A_inferred={a_inf:.5}, rel={rel:.2e}"
            );
        }
    }

    // ── §8.A(d) VC-M2-Master — Venner2000 식(29) 마스터커브 [ref(31)] ──
    //
    // A_d/A_i = 1/(1+0.15 f̄∇₂ + 0.015(f̄∇₂)²), ∇₂=(λ/a)√(M/L), f̄(r>1)=e^{1−1/r} else 1.
    // 예제(Fig.4/5: λx=λy=a/4, M=1007.6, L=12.05, r=1→f̄=1) → ∇₂=2.2861 → A_d/A_i=0.7036.
    // 문헌 수치해 스팟 A_d=0.739·A_i 와 근사오차 수%(fit↔numerics) 내 정합.
    #[test]
    fn vc_m2_master_venner2000() {
        // 식(29)·∇₂·f̄·예제·문헌 수치해 — 전부 reference(leaf) 소유.
        use crate::reference::{
            venner2000_amplitude_reduction, venner2000_f_bar, venner2000_nabla2, VENNER2000_EXAMPLE,
            VENNER2000_EXAMPLE_NUMERICS,
        };
        let f_bar = venner2000_f_bar;
        let ad_ai = |nab2: f64, f: f64| venner2000_amplitude_reduction(nab2, f);
        // f̄ 정의 검증.
        assert!((f_bar(1.0) - 1.0).abs() < 1e-15, "f̄(1)=1");
        assert!((f_bar(2.0) - (0.5_f64).exp()).abs() < 1e-12, "f̄(2)=e^0.5");
        // 예제 스팟: ∇₂ 계산 → 식(29) 값.
        let (m, l, lam_over_a) = VENNER2000_EXAMPLE;
        let nab2 = venner2000_nabla2(lam_over_a, m, l);
        assert!((nab2 - 2.2861).abs() < 1e-3, "∇₂ example={nab2}");
        let ratio = ad_ai(nab2, 1.0);
        // 식(29) 자체 값(기계정밀): 0.7036.
        assert!((ratio - 0.70358).abs() < 1e-4, "eq(29) value={ratio}");
        // 문헌 수치해 0.739 와 fit 오차 수%(<6%) 내.
        assert!(
            (ratio - VENNER2000_EXAMPLE_NUMERICS).abs() / VENNER2000_EXAMPLE_NUMERICS < 0.06,
            "master vs Venner numerics({VENNER2000_EXAMPLE_NUMERICS}): {ratio}"
        );
        // 물리 극한: ∇₂→0(단파장/고주파) → A_d/A_i→1(불변), ∇₂→∞(장파장) → →0(완전변형).
        assert!(ad_ai(1e-4, 1.0) > 0.999, "high-freq: unchanged");
        assert!(ad_ai(1e4, 1.0) < 0.01, "low-freq: fully deformed");
        // 단조 감소.
        let mut prev = 2.0;
        for i in 0..20 {
            let v = ad_ai(0.1 * (i as f64 + 1.0), 1.0);
            assert!(v < prev, "A_d/A_i must be monotone decreasing in ∇₂");
            prev = v;
        }
    }

    // ════════════════════════════════════════════════════════════════════
    //  VC-M2-Comp-Amplitude — 상보파 절대진폭 g 의 Venner1997 정량 검증 (G-M2-1 해소)
    //  ★ 원문(15) 직접 정독: eq(5) 1/(1+0.17∇+0.03∇²), ∇=(λ/b)M^{3/4}/L^{1/2},
    //    Table1(M=100,L=11) 스팟. (31) eq(29) 교차확인은 vc_m2_master_venner2000 이 담당.
    // ════════════════════════════════════════════════════════════════════

    // ── VC-M2-Comp-Amplitude: 상보파 절대진폭 g 를 Venner1997 데이터로 정량 검증 [ref(15)] ──
    //
    // **단일 오라클 2단**(G-M2-1 해소, 이번 라운드 결선점):
    //  ▸ Part A — 외부 기준곡선 확정(비-tautology): 순수 rolling 진폭감소 마스터커브
    //    A_d/A_i = 1/(1+0.17∇+0.03∇²), ∇=(λ/b)·M^{3/4}/L^{1/2} ((15) §4.4 식(5)·§3.1 M=100·L=11)
    //    가 Table1(수치해) 스팟 λ/b=1.0→0.183·0.5→0.394·0.25→0.660 을 논문 명시 정확도 2~5% 내로
    //    재현하는지 대조. Venner 상수(0.17/0.03)·Table1 수치는 **외부 출판치를 하드코딩**(모델에서
    //    재계산하지 않음) → fit↔numerics 둘 다 외부 grounding.
    //  ▸ Part B — 모델 g 를 그 외부 곡선에 정량 anchor(**g-민감**): (31)Venner2000 L631 2성분해에서
    //    고하중 순수 rolling 시 정상(특수해) 유막은 완전평탄화되고 상보파(입구 excitation)가 유막
    //    A_d 를 지배 → A_d/A_i = 상보파 진폭비 = g. 모델의 상보파 g 를 solve_full_film 과 동일 경로
    //    (complementary_wave, slip=0→β=0→|h_comp|=g)로 추출 → COMP_INLET_RATIO 변이에 직접 민감.
    //    eq(5)를 g 로 역산한 λ/b 가 Table1 실측 A_d/A_i=0.5 교차구간 (0.25,0.5)(λ/b=0.25→0.660>0.5,
    //    0.5→0.394<0.5) 안에 드는지 검증 → g=0.5(GW half-pumping, fit r₂=0.45)와 3자 정합.
    //
    // ★ 비-tautology: 구간 (0.25,0.5)·역산식·2~5% 대조는 **Venner 출판 Table1/eq(5)**(외부)에서만 옴.
    //   모델 g 만 흔들면(Part B) 반드시 검출되고, Part A(순수 외부수치)는 g 와 무관하게 고정.
    // ★ g-민감(변이게이트 (i)): g→g×0.5=0.25 시 λ/b→0.7929 로 구간 이탈 → 이 오라클 FAIL(CAUGHT).
    // ★ 정직 한계: 본 모델은 특수해 유막을 평탄화하지 않아(순수 rolling h_part=1) **총** 유막비는
    //   1±g 로 Venner 곡선과 불일치 — 여기선 (31) 2성분해 정의대로 **상보파 성분 g** 만 Venner A_d
    //   로 앵커한다. 총유막 평탄화·파장의존 g(∇) 는 별도 잔여(RQ-M2-comp-curve; types 동결로
    //   ∇=(λ/b)M^{3/4}/L^{1/2} 의 b,M,L 산출 불가 → 곡선 전체 결선 차단).
    #[test]
    fn vc_m2_comp_amplitude_venner() {
        // ══ Part A — Venner1997 eq(5) 마스터커브 vs Table1 수치해 (외부 기준곡선 확정) ══
        // ∇ = (λ/b)·M^{3/4}/L^{1/2}, M=100, L=11 (Venner1997 §3.1 numerical-accuracy case).
        // 식(5)·상수(0.17/0.03)·Table1·앵커스팟 — 전부 reference(leaf) 소유. 뷰어 참조곡선과 동일 코드.
        use crate::reference::{
            venner1997_amplitude_reduction, venner1997_nabla, venner1997_nabla_from_ratio,
            VENNER1997_ANCHOR_SPOTS, VENNER1997_HALF_CROSSING_BRACKET, VENNER1997_L, VENNER1997_M,
        };
        let m = VENNER1997_M;
        let l = VENNER1997_L;
        let grad = |lob: f64| venner1997_nabla(lob, m, l);
        let ad_ai = |lob: f64| venner1997_amplitude_reduction(grad(lob));
        // Table1 스팟(외부 출판 수치해; (15) L122-124). 앵커역 λ/b∈[0.25,1.0](A_d/A_i≈0.5 교차부)
        // 서 fit↔numerics 2~5% 대조. 큰 ∇ 꼬리 λ/b≥2 은 단일(M,L) fit 산포가 5% 초과(λ/b=2→~9.6%)
        // 라 스팟서 제외. eq(5) 과소예측역은 0.5<A_d/A_i<1(단파장·작은 λ/b), A_d/A_i≤0.5 는
        // 장파장(큰 λ/b·큰 ∇)쪽이다((15) §4.4; L1245 물리극한과 정합).
        let spots = VENNER1997_ANCHOR_SPOTS;
        for (lob, table) in spots {
            let fit = ad_ai(lob);
            let rel = (fit - table).abs() / table;
            assert!(
                rel <= 0.05,
                "eq(5) vs Table1 @λ/b={lob}: fit={fit:.4}, table={table}, rel={rel:.3} (>5%)"
            );
        }
        // ∇·eq(5) 스팟(기계정밀 고정): λ/b=0.5 → ∇=4.76734, A_d/A_i=0.40122.
        assert!((grad(0.5) - 4.76734).abs() < 1e-4, "∇(λ/b=0.5)={}", grad(0.5));
        assert!((ad_ai(0.5) - 0.40122).abs() < 1e-4, "eq(5)(λ/b=0.5)={}", ad_ai(0.5));
        // 물리 극한(식(5) 자명): 단파장(∇→0)→1(거칠기 지속), 장파장(∇→∞)→0(완전평탄화).
        assert!(ad_ai(1e-4) > 0.999, "short-λ: roughness persists (A_d/A_i→1)");
        assert!(ad_ai(1e4) < 1e-3, "long-λ: fully flattened (A_d/A_i→0)");
        // ∇ 에 대해 단조 감소.
        let mut prev = 2.0;
        for i in 1..=40 {
            let v = ad_ai(0.05 * i as f64);
            assert!(v < prev, "A_d/A_i must be monotone decreasing in ∇");
            prev = v;
        }

        // ══ Part B — 모델 상보파 진폭 g 를 위 외부곡선에 정량 anchor (g-민감·비-tautology) ══
        // 모델의 상보파 g 를 solve_full_film 과 동일 경로(complementary_wave, 순수 rolling
        // slip=0 → β=0 → |h_comp|=g_inlet)에서 추출 → COMP_INLET_RATIO 변이에 직접 민감.
        let e_red = E_RED_STEEL_PA;
        let h = 1.4e-7;
        let lx = 1.0e-3;
        let kx = 2.0 * PI * 4.0 / lx; // 임의 모드(순수 rolling |h_comp|=g 는 kx 무관)
        let psi = dispersion_psi(kx, 0.0, h, e_red, 5e6, 1.0, 1.0)
            .expect("pure-rolling dispersion converges (β=0)");
        let (h_comp, _p) =
            complementary_wave(psi, 0.0, e_red, complementary_inlet_ratio(), 0.5 * lx);
        let g_model = h_comp.norm(); // 순수 rolling: |h_comp| = g_inlet (모델 상보파 진폭비)
        assert!(
            (g_model - complementary_inlet_ratio()).abs() < 1e-12,
            "model complementary amplitude must equal g_inlet in pure rolling: {g_model}"
        );

        // Venner1997 eq(5) 역산: A_d/A_i=g 인 ∇ 양근 — reference(leaf) 소유.
        // ∇ = factor·(λ/b), factor≈9.5347 (Part A 와 동일 M,L) → λ/b = ∇/factor.
        let factor = venner1997_nabla(1.0, m, l);
        let invert_lob =
            |g: f64| venner1997_nabla_from_ratio(g).expect("g∈(0,1] 이므로 가역") / factor;
        let lob = invert_lob(g_model);

        // 외부 앵커: (15) Table1 실측 A_d/A_i=0.5 교차구간 (0.25,0.5).
        let (br_lo, br_hi) = VENNER1997_HALF_CROSSING_BRACKET;
        assert!(
            lob > br_lo && lob < br_hi,
            "g={g_model} → λ/b={lob:.4} outside Venner Table1 0.5-crossing bracket ({br_lo},{br_hi})"
        );
        // g=0.5 → λ/b≈0.3773 (∇≈3.598): GW half-pumping 중앙곡선점.
        assert!(
            (lob - 0.3773).abs() < 1e-3,
            "g=0.5 anchor λ/b={lob:.4} (expect 0.3773, ∇≈3.598)"
        );
        // 민감도 대역 [0.45,0.6](RQ-M2-comp1) 은 구간 내(정상).
        let in_bracket = |v: f64| v > br_lo && v < br_hi;
        assert!(in_bracket(invert_lob(0.45)), "g=0.45 in-bracket");
        assert!(in_bracket(invert_lob(0.60)), "g=0.60 in-bracket");
        // 변이게이트 (i): g 반감(0.25) → 구간 이탈(검출력 영구 고정, 이 오라클이 직접 CAUGHT).
        let mutated = invert_lob(g_model * 0.5);
        assert!(
            !in_bracket(mutated),
            "MUTATION UNDETECTED: g×0.5 → λ/b={mutated:.4} still in Venner bracket"
        );
    }
}
