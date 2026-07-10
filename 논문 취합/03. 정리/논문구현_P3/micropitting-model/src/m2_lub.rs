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

/// Barus 지수 인수 상한 `α·p` [-] (수치 폭주 방지 캡).
///
/// Barus `η=η0·exp(α·p)` 는 고압(>~1 GPa)에서 급격히 과대예측한다. 본 논문 운전점
/// (`α_visc=2e-8, p_h=1.5 GPa`)에서 `α·p≈30 → exp(30)≈1.07e13` 배 → `η≈1e11 Pa·s`
/// 로 비물리적. 원 논문은 접촉 중앙 점도를 **Eyring 유효점도(식[6]) 운전점**에서 평가하며
/// Barus 를 peak Hertz압에 그대로 적용하지 않는다. 물리적 대안(포화형 Roelands)은 미구현
/// → **잔여큐 RQ-3**. 임시로 지수 인수를 이 상한으로 캡하여 폭주를 억제한다.
///
/// 상한값(=12) 자체는 '가정+민감도'(RQ-3): `exp(12)≈1.6e5` 배 → `η0=0.01→~1.6e3 Pa·s`
/// 로 EHL 중앙 점도 현실범위(10²–10⁴). `barus_arg_cap_sensitivity` 가 캡 발동·유계성을 강제.
const BARUS_ARG_CAP: f64 = 12.0;

/// 캡이 적용된 Barus 국소 점도 [Pa·s]: `η = η0·exp(min(α·p, cap))`.
///
/// 고압 폭주(exp(30)) 억제. `cap` 은 RQ-3 가정(민감도 대상, [`BARUS_ARG_CAP`]).
/// `cap` 미도달(저압)에서는 순수 [`barus_visc`] 와 동일.
#[inline]
fn barus_visc_capped(eta0: f64, alpha_visc: f64, p: f64, cap: f64) -> f64 {
    let arg = (alpha_visc * p).min(cap);
    eta0 * arg.exp()
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
/// 주의(잔여큐 RQ-vel): 특수해는 표면2 거칠기와 함께 `u₂` 로 이동하므로 대류속도는
/// 평균구름 `ū` 가 아니라 `(u₂−ū)=−Δu/2`(미끄럼 절반)이다. 현 [`solve_full_film`] 은
/// op 속도필드(`u2`,`u_mean`,`slide_roll`)의 상호일관성 미확정 + 상보파(`ū` 전파) 미구현
/// 으로 잠정 `op.u_mean` 을 대입한다(식[2] 근거값은 `(u₂−ū)`; 오케스트레이터 판단 대기).
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

    // 국소 압점도(Barus, 폭주 캡 BARUS_ARG_CAP; RQ-3) + 미끄럼 전단률.
    // full p_h 에 exp(α·p) 오적용 시 α·p≈30→exp(30) 비물리(η~1e11 Pa·s). 지수 인수 캡으로
    // 억제(원 논문은 식[6] Eyring 운전점 η 사용; Roelands 포화형 미구현 → RQ-3).
    let eta_local = barus_visc_capped(op.eta0, op.alpha_visc, op.p_h, BARUS_ARG_CAP);
    let d_u = (op.slide_roll * op.u_mean).abs(); // |Δu| = |SRR|·u_mean
    let gamma_s = if h0 > 0.0 { d_u / h0 } else { 0.0 };
    let tau0 = op.tau0;
    let c_comp = 0.0_f64; // 비압축·등점도 극한(상보파 Phase 에서 C≠0 정밀화; RQ-2)

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
            // NOTE(RQ-vel): 식[2] 근거 대류속도는 (u₂−ū)=−Δu/2 이나, op 속도필드 상호
            // 일관성 미확정+상보파(ū 전파) 미구현으로 잠정 op.u_mean 대입(오케스트레이터
            // 판단 대기). amplitude_q 계수24·|k|³ 는 q_independent_oracle_from_eq2 로 독립검증.
            let q = amplitude_q(eta_mode, op.u_mean, h0, kx, k_mag, e_red);
            let mut pt = pressure_ripple_transfer(k_mag, e_red, q, c_comp);
            let mut ht = film_ripple_transfer(k_mag, e_red, q, c_comp);
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
pub fn solve_partial(input: &PartialLubInput) -> PartialLubResult {
    let full = solve_full_film(input);
    PartialLubResult {
        p_tran: full.p_lub,
        h_tran: full.h_lub,
        phi_bl: 0.0,
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
            u2: 0.9,
            slide_roll: 0.1,
            eta0: 0.01,
            alpha_visc: 2e-8,
            tau0: 5e6,
            temp: 353.0,
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

        // 해석 기대값: 모드에 쓰인 것과 동일한 파라미터로 전달함수 재계산(캡 반영).
        let eta_local =
            barus_visc_capped(input.op.eta0, input.op.alpha_visc, input.op.p_h, BARUS_ARG_CAP);
        let gamma_s = (input.op.slide_roll * input.op.u_mean).abs() / input.h_bar;
        let eta_mode = directional_visc(eta_local, gamma_s, input.op.tau0, kx, 0.0);
        let q = amplitude_q(eta_mode, input.op.u_mean, input.h_bar, kx, k_mag, input.mat.e_red);
        let p_gain = pressure_ripple_transfer(k_mag, input.mat.e_red, q, 0.0).norm();
        let h_gain = film_ripple_transfer(k_mag, input.mat.e_red, q, 0.0).norm();

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
        // 평균 유막은 h_bar.
        let h_mean: f64 = res.h_lub.data.iter().sum::<f64>() / res.h_lub.len() as f64;
        assert!((h_mean - input.h_bar).abs() < input.h_bar * 1e-9);
        // 물리성: 유막 리플이 원거칠기보다 감소(진폭감소).
        assert!(h_amp < r_a, "film ripple should be reduced below roughness");
    }

    // ── 6. 인터페이스 유지: solve_partial 통과 + phi_bl=0 ──
    #[test]
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

    // ── M2-3: Barus 폭주 캡 — full p_h 오적용(exp(30)) 억제·유계(RQ-3). ──
    #[test]
    fn barus_arg_cap_sensitivity() {
        // 무캡 Barus 는 운전점(α·p=30)에서 폭주.
        let raw = barus_visc(0.01, 2e-8, 1.5e9); // exp(30)≈1.07e13 배
        assert!(raw > 1e9, "uncapped Barus should blow up: {raw}");
        // 캡 적용 → 지수 인수 = BARUS_ARG_CAP, 유계.
        let capped = barus_visc_capped(0.01, 2e-8, 1.5e9, BARUS_ARG_CAP);
        assert!(
            (capped - 0.01 * BARUS_ARG_CAP.exp()).abs() <= capped * 1e-12,
            "capped value != η0·exp(cap)"
        );
        assert!(capped < raw, "cap must reduce viscosity");
        assert!(capped.is_finite() && capped < 1e6, "capped η out of physical range: {capped}");
        // 저압(캡 미발동, α·p=2<cap)에서는 순수 Barus 와 동일.
        let low = barus_visc_capped(0.01, 2e-8, 1.0e8, BARUS_ARG_CAP);
        assert!(
            (low - barus_visc(0.01, 2e-8, 1.0e8)).abs() <= low * 1e-12,
            "cap must be inactive below threshold"
        );
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

        // 기대: x-Nyquist 실수투영 → p = Re(T)·r_a·(−1)^i·cos(ky y).
        let eta_local = barus_visc_capped(
            input.op.eta0,
            input.op.alpha_visc,
            input.op.p_h,
            BARUS_ARG_CAP,
        );
        let gamma_s = (input.op.slide_roll * input.op.u_mean).abs() / input.h_bar;
        let eta_mode = directional_visc(eta_local, gamma_s, input.op.tau0, kx, ky);
        let q = amplitude_q(eta_mode, input.op.u_mean, input.h_bar, kx, k_mag, input.mat.e_red);
        let t_re = pressure_ripple_transfer(k_mag, input.mat.e_red, q, 0.0).re; // 실수투영분
        // Nyquist 는 자명 실수(Im(T)=0 아님 → 실수투영이 반드시 필요).
        let t_full = pressure_ripple_transfer(k_mag, input.mat.e_red, q, 0.0);
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
}
