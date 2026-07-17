//! # reference — 문헌 폐형식·출판 데이터 (**leaf 모듈**)
//!
//! 계획 정본: `논문구현_P3_시각화_HTML.md` §4.1(Phase 1).
//!
//! ## 무엇이 여기 오는가
//! **우리 모델과 독립인** 외부 문헌의 폐형식과 출판 수치만. 이 모듈의 값은 오라클의 *기대값*
//! 이자 뷰어 참조곡선의 *데이터원*이다. 두 용도가 **같은 코드**를 쓰므로, 뷰어에 그려지는 곡선은
//! 오라클이 지키는 바로 그 곡선이다(SSOT 단일).
//!
//! ## ★ leaf 불변식 — 어기면 오라클이 붕괴한다
//!
//! ```text
//! (1) 이 모듈은 types·units 외 크레이트 내부를 import 하지 않는다.
//! (2) m1~m6·partial_lub 의 **생산 코드**는 이 모듈을 참조하지 않는다.
//!     (각 모듈의 #[cfg(test)] 안에서 기대값으로 쓰는 것은 허용 — 그것이 이 모듈의 용도다.)
//! ```
//!
//! **왜**: 모델이 reference 를 쓰기 시작하면 오라클이 **모델 자신을 검증**하게 되어 순환한다.
//! 이는 가상의 우려가 아니라 본 프로젝트에서 **두 번 발생한 실패 모드**다 —
//! (a) Phase 1 M2 `Q` 오라클 tautology(검증 대상 함수로 기대값 생성 → 계수 24↔12 오류도 통과, G3 fail),
//! (b) M4 대칭 오라클 사각지대(MCE 중심 ≡ centroid 라 centroid 스텁이 전 오라클 통과).
//! 불변식은 `reference_is_leaf_*` 구조 가드 테스트가 기계적으로 강제한다.
//!
//! ## 여기 오지 **않는** 것
//! - **우리 모델이 곧 그 식인 경우**. 예: Archard 식[14] `Δh_w/Δn = k·p·u_s/H` 는
//!   `m5_wear::wear_depth_rate` **자체**다. 이를 여기 복제하면 "같은 식의 복제본끼리 비교"가 되어
//!   위 (a) 와 동형의 tautology 를 새로 만든다. → Archard 는 **실험 데이터(기울기)만** 등재
//!   ([`ARCHARD1953_FIG7_SLOPE_BRASS`] 등), 폐형식은 등재하지 않는다.
//! - 우리 가정·자유계수. 모든 값은 출판 문헌으로 소급되어야 한다(자의적 작업 금지 대원칙).
//!
//! ## 출처
//! - GW 1994 = ref(29) `1994. (GW-SKF) The Behaviour of transverse roughness in EHL contacts.md`
//! - Venner 1997 = ref(15) `1997. (Venner) Amplitude Reduction of Waviness in Transient EHL Line Contacts.md`
//! - Venner 2000 = ref(31) `2000. (Venner) Multigrid techniques.md`
//! - Tripp/ME 2003 = ref(16) `2003. (SKF) Frequency Response Functions and Rough Surface.md`
//! - Milano 2006 = ref(34) · Archard 1953 = ref(17)
//! - McEwen = 고전 선접촉 Hertz 표면하 응력해

use std::f64::consts::PI;

// ═════════════════════════════════════════════════════════════════════════
//  Greenwood–Morales-Espejel (GW) 1994 — 정상상태 진폭감소 폐형식 [ref(29)]
// ═════════════════════════════════════════════════════════════════════════

/// GW1994 식(15) 무차원 파라미터 `A = 2λ/(π·α·h̄·E′)`.
///
/// - `lambda` 파장 [m] · `alpha_visc` 압점도 [1/Pa] · `h_bar` 평균유막 [m]
/// - `e_prime` **논문 E′** [Pa] (= 2·E_red — 규약 조정표 §1.4; 표준 E_red 를 넣지 말 것)
///
/// 식(17) `Aₙ = A/n` 은 파장 λ/n 을 대입하면 자동 성립(A ∝ λ).
#[inline]
pub fn gw1994_a(lambda: f64, alpha_visc: f64, h_bar: f64, e_prime: f64) -> f64 {
    2.0 * lambda / (PI * alpha_visc * h_bar * e_prime)
}

/// GW1994 식(16) 유막 진폭비 `H₁/Z₁ = C/(C+A)`.
///
/// 극한: `A→0`(단파장) → 1(거칠기 지속) · `A→∞`(장파장) → 0(평탄화).
#[inline]
pub fn gw1994_h1_over_z1(c: f64, a: f64) -> f64 {
    c / (c + a)
}

/// GW1994 식(15) 압력 진폭비 `P₁/Z₁ = −1/(C+A)` [1/Pa 스케일은 호출측 규약].
#[inline]
pub fn gw1994_p1_over_z1(c: f64, a: f64) -> f64 {
    -1.0 / (c + a)
}

/// GW1994 Table 2 — Kweh 사례 입력 (L120). `(alpha_visc[1/Pa], h_bar[m])`.
///
/// 50 ℃ / 80 ℃. 공통: `E′ = 227 GPa`([`GW1994_E_PRIME_PA`]), `2λ = 0.4 mm`, `z₁ = 0.25 µm`.
pub const GW1994_TABLE2_KWEH: [(f64, f64); 2] = [
    (14.8e-9, 1.341e-6), // 50 ℃
    (12.3e-9, 0.698e-6), // 80 ℃
];

/// GW1994 Table 2 — 환산탄성계수 **E′** [Pa] (논문 규약, = 2·E_red).
pub const GW1994_E_PRIME_PA: f64 = 227.0e9;

/// GW1994 Table 2 — 파장 λ [m] (`2λ = 0.4 mm`).
pub const GW1994_LAMBDA_M: f64 = 0.2e-3;

/// GW1994 Table 2 — 미변형 거칠기 진폭 `z₁` [m].
pub const GW1994_Z1_M: f64 = 0.25e-6;

/// GW1994 Table 1 — **"Present theory"** 행 (L112·113). `(P₁[Pa], h₁[m])`, 50 ℃ / 80 ℃.
///
/// ⚠️ 같은 표의 Kweh·Chang 행은 원문 L115 verbatim — *"estimates from graphs"* → 허용오차를
/// 달리해야 한다(총괄계획 §8.A.3(b): GJ ±5 % / Kweh·Chang ±15~18 %).
pub const GW1994_TABLE1_PRESENT: [(f64, f64); 2] = [
    (0.388e9, 0.0331e-6), // 50 ℃
    (0.413e9, 0.0185e-6), // 80 ℃
];

/// GW1994 상보파 입구 pumping 비 `g` 의 문헌 근거값 (L352 verbatim).
///
/// 원문: *"an approximation can be made by estimating the 'pumping' of the oil flow from the
/// inlet. **Assuming it to be half** that due to the undeformed roughness gives fair agreement
/// with Venner and Lubrecht."* → `g = 0.5`. 민감도 범위 [0.45, 0.60](P2-2 재검토 L65).
pub const GW1994_HALF_PUMPING_G: f64 = 0.5;

// ═════════════════════════════════════════════════════════════════════════
//  Venner 1997 — 선접촉 진폭감소 마스터커브 [ref(15)]
// ═════════════════════════════════════════════════════════════════════════

/// Venner 1997 §4.4 식(5) 무차원수 `∇ = (λ/b)·M^{3/4}/L^{1/2}`.
///
/// ★ **선접촉 전용**. 선접촉 `M = W(2U)^{-1/2}` · `L = G(2U)^{1/4}`.
/// Venner **2000**(점접촉)의 `M = W(2U)^{-3/4}` 와 **혼용 금지** — 같은 입력에 line-M 을 넣으면
/// 2.44 ≠ 1007.6 으로 어긋난다(총괄계획 L476: 카테고리 오류 차단). 점접촉은 [`venner2000_nabla2`].
#[inline]
pub fn venner1997_nabla(lambda_over_b: f64, m: f64, l: f64) -> f64 {
    lambda_over_b * m.powf(0.75) / l.sqrt()
}

/// Venner 1997 식(5) 마스터커브 `A_d/A_i = 1/(1 + 0.17∇ + 0.03∇²)`.
///
/// 상수 0.17·0.03 은 **외부 출판치**(모델에서 재계산하지 않는다).
/// 극한: `∇→0` → 1 · `∇→∞` → 0 (원문 명시, 자연 충족).
///
/// ⚠️ **적용 한계(원문 §4.4 verbatim)**: *"closely approximates the computed values for
/// A_d/A_i ≤ 0.5. For larger values, 0.5 < A_d/A_i < 1, however, this curve seems to predict
/// values which are **too small**."* → 뷰어는 이 구간을 "fit degrades" 로 음영 표기할 것
/// ([`venner1997_fit_degrades`]).
#[inline]
pub fn venner1997_amplitude_reduction(nabla: f64) -> f64 {
    1.0 / (1.0 + 0.17 * nabla + 0.03 * nabla * nabla)
}

/// 식(5) 의 과소예측 구간(`0.5 < A_d/A_i < 1`) 판정 — 원문 §4.4 자인.
#[inline]
pub fn venner1997_fit_degrades(ad_ai: f64) -> bool {
    ad_ai > 0.5 && ad_ai < 1.0
}

/// Venner 1997 Table 1 사례의 무차원 하중수 `M`(선접촉) — §3.1.
pub const VENNER1997_M: f64 = 100.0;

/// Venner 1997 Table 1 사례의 무차원 재료수 `L` — §3.1.
pub const VENNER1997_L: f64 = 11.0;

/// Venner 1997 **Table 1** (L117-128) — `A_d/A_i` 수치해. **전 코퍼스 최대 자산(18점)**.
///
/// 행 = `(λ/b, [A_i/H_c=0.1, 0.2, 0.5])`. `M = 100`, `L = 11`, 순수 rolling.
///
/// ★ `A_i/H_c = 0.1` 열이 총괄계획 L476 **anti-fudge 앵커** — 재현 요구 2~5 %, **튜닝 금지**.
pub const VENNER1997_TABLE1: [(f64, [f64; 3]); 6] = [
    (4.0, [0.030, 0.030, 0.029]),
    (2.0, [0.073, 0.073, 0.073]),
    (1.0, [0.183, 0.182, 0.195]),
    (0.5, [0.394, 0.393, 0.378]),
    (0.25, [0.660, 0.670, 0.679]),
    (0.125, [0.839, 0.838, 0.859]),
];

/// Venner 1997 Table 1 의 `A_i/H_c = 0.1` 열에서 **fit 산포 5 % 내** 앵커 스팟만 추출.
///
/// `(λ/b, A_d/A_i)`. 큰 ∇ 꼬리(`λ/b ≥ 2`)는 단일 (M,L) fit 산포가 5 % 를 넘어(λ/b=2 → ~9.6 %)
/// 제외한다 — 원문 식(5)가 fit 이지 수치해가 아니기 때문이며, 이는 **fit 의 한계**이지
/// 우리 모델의 오차가 아니다.
pub const VENNER1997_ANCHOR_SPOTS: [(f64, f64); 3] = [(1.0, 0.183), (0.5, 0.394), (0.25, 0.660)];

/// Venner 1997 Table 1 의 `A_d/A_i = 0.5` **교차구간** `(λ/b_lo, λ/b_hi)`.
///
/// `λ/b = 0.25 → 0.660 > 0.5` · `λ/b = 0.5 → 0.394 < 0.5` → 교차점은 (0.25, 0.5) 안.
/// 모델의 상보파 `g` 를 이 외부 구간에 앵커한다(G-M2-1 해소, 단일점).
pub const VENNER1997_HALF_CROSSING_BRACKET: (f64, f64) = (0.25, 0.5);

/// 식(5) 역산: `A_d/A_i = ad_ai` 를 주는 `∇` (양근).
///
/// `0.03∇² + 0.17∇ + (1 − 1/ad_ai) = 0` 의 양근. `ad_ai ∈ (0,1]` 밖이면 `None`.
pub fn venner1997_nabla_from_ratio(ad_ai: f64) -> Option<f64> {
    if !(ad_ai > 0.0 && ad_ai <= 1.0) {
        return None;
    }
    let cc = 1.0 - 1.0 / ad_ai;
    let disc = 0.17 * 0.17 - 4.0 * 0.03 * cc;
    if disc < 0.0 {
        return None;
    }
    Some((-0.17 + disc.sqrt()) / (2.0 * 0.03))
}

// ═════════════════════════════════════════════════════════════════════════
//  Venner 2000 — 점접촉 진폭감소 마스터커브 [ref(31)]
// ═════════════════════════════════════════════════════════════════════════

/// Venner 2000 식(29) 이방성 인자 `f̄(r) = e^{1−1/r}` (r>1), 그 외 1. `r = λ_x/λ_y`.
#[inline]
pub fn venner2000_f_bar(r: f64) -> f64 {
    if r > 1.0 {
        (1.0 - 1.0 / r).exp()
    } else {
        1.0
    }
}

/// Venner 2000 무차원수 `∇₂ = (λ/a)·√(M/L)`, `λ = min(λ_x, λ_y)`.
///
/// ★ **점접촉 전용** (`M = W(2U)^{-3/4}`). 선접촉은 [`venner1997_nabla`].
#[inline]
pub fn venner2000_nabla2(lambda_over_a: f64, m: f64, l: f64) -> f64 {
    lambda_over_a * (m / l).sqrt()
}

/// Venner 2000 식(29) `A_d/A_i = 1/(1 + 0.15·f̄∇₂ + 0.015·(f̄∇₂)²)`.
#[inline]
pub fn venner2000_amplitude_reduction(nabla2: f64, f_bar: f64) -> f64 {
    let x = f_bar * nabla2;
    1.0 / (1.0 + 0.15 * x + 0.015 * x * x)
}

/// Venner 2000 Table 1 worked example — `(M, L, λ/a)`. **점접촉**.
pub const VENNER2000_EXAMPLE: (f64, f64, f64) = (1007.6, 12.05, 0.25);

/// Venner 2000 문헌 수치해 스팟: 위 example 에서 `A_d = 0.739·A_i`.
///
/// 식(29)(fit) 는 0.7036 → fit↔numerics 차 **<6 %**. 이 차이는 fit 의 성질이지 오차가 아니다.
pub const VENNER2000_EXAMPLE_NUMERICS: f64 = 0.739;

// ═════════════════════════════════════════════════════════════════════════
//  Tripp / Morales-Espejel 2003 — 표면하 6응력 폐형식 [ref(16)] 식[10]/[16]
// ═════════════════════════════════════════════════════════════════════════
//
// ★ SSOT: 2011 원논문 식[12]/[13] 은 **OCR 손상**(σ_y·e^{−ζz}→e^{−z}·sin/cos 오류)이라
//   사용 금지. 정본은 Tripp 2003 식[10](법선)/[16](트랙션). P2-2 재검토 R-M3-Sin.
//
// 반환 순서는 `types::StressTensor6` 와 동일: `[σxx, σyy, σzz, σxy, σyz, σxz]`. 인장 +.

/// 식[10] — **법선** 이중정현 하중 `p = p₀·cos(αx)·cos(βy)` 의 표면하 6응력 [Pa].
///
/// `zeta = √(α²+β²)`. 깊이 감쇠 `e^{−ζz}`.
///
/// 이 폐형식은 **모델과 독립**이다 — `m3_stress` 는 2D-FFT + 복소 FRF 모드중첩으로 같은 장을
/// 계산하므로, 이 실수식과의 대조는 복소↔실 변환·FFT 정규화·부호를 독립 검증한다.
pub fn tripp2003_normal_bisin(
    p0: f64,
    alpha: f64,
    beta: f64,
    nu: f64,
    x: f64,
    y: f64,
    z: f64,
) -> [f64; 6] {
    let zeta = (alpha * alpha + beta * beta).sqrt();
    let e = (-zeta * z).exp();
    let cc = (alpha * x).cos() * (beta * y).cos();
    let ss = (alpha * x).sin() * (beta * y).sin();
    let cs = (alpha * x).cos() * (beta * y).sin();
    let sc = (alpha * x).sin() * (beta * y).cos();
    let z2 = zeta * zeta;

    let sxx = p0 * (alpha * alpha / z2 - alpha * alpha * z / zeta + 2.0 * nu * beta * beta / z2) * e * cc;
    let syy = p0 * (beta * beta / z2 - beta * beta * z / zeta + 2.0 * nu * alpha * alpha / z2) * e * cc;
    let szz = p0 * (1.0 + zeta * z) * e * cc;
    let sxy = -p0 * (alpha * beta / z2) * ((1.0 - 2.0 * nu) - zeta * z) * e * ss;
    let syz = p0 * (beta * z) * e * cs;
    let sxz = p0 * (alpha * z) * e * sc;
    [sxx, syy, szz, sxy, syz, sxz]
}

/// 식[16] — **접선(트랙션)** 이중정현 하중 `q = q₀·cos(αx)·cos(βy)` 의 표면하 6응력 [Pa].
pub fn tripp2003_tangential_bisin(
    q0: f64,
    alpha: f64,
    beta: f64,
    nu: f64,
    x: f64,
    y: f64,
    z: f64,
) -> [f64; 6] {
    let zeta = (alpha * alpha + beta * beta).sqrt();
    let e = (-zeta * z).exp();
    let cc = (alpha * x).cos() * (beta * y).cos();
    let ss = (alpha * x).sin() * (beta * y).sin();
    let cs = (alpha * x).cos() * (beta * y).sin();
    let sc = (alpha * x).sin() * (beta * y).cos();
    let aoz = alpha / zeta;
    let boz = beta / zeta;

    let sxx = q0 * aoz * (2.0 + 2.0 * nu * boz * boz - aoz * (alpha * z)) * e * sc;
    let syy = q0 * aoz * (2.0 * nu * aoz * aoz - boz * (beta * z)) * e * sc;
    let szz = q0 * (alpha * z) * e * sc;
    let sxy = q0 * boz * (1.0 - 2.0 * nu * aoz * aoz - aoz * (alpha * z)) * e * cs;
    let syz = q0 * boz * (alpha * z) * e * ss;
    let sxz = q0 * (1.0 - aoz * (alpha * z)) * e * cc;
    [sxx, syy, szz, sxy, syz, sxz]
}

/// 식[10] **trace 항등** — `σx+σy+σz = 2(1+ν)·p₀·e^{−ζz}·cos(αx)·cos(βy)`.
///
/// ★ **정본 판별자**(P2-1 L116). 2011 손상식은 이 항등을 만족하지 못한다 → 어느 식이 정본인지
/// 기계적으로 가른다. 손유도: 대괄호합 `= (α²+β²)/ζ² − (α²+β²)z/ζ + 2ν(α²+β²)/ζ² + (1+ζz)`
/// `= 1 − ζz + 2ν + 1 + ζz = 2(1+ν)`.
pub fn tripp2003_trace_normal(p0: f64, alpha: f64, beta: f64, nu: f64, x: f64, y: f64, z: f64) -> f64 {
    let zeta = (alpha * alpha + beta * beta).sqrt();
    2.0 * (1.0 + nu) * p0 * (-zeta * z).exp() * (alpha * x).cos() * (beta * y).cos()
}

/// 식[16] **trace 항등** — `σx+σy+σz = 2(1+ν)·q₀·(α/ζ)·e^{−ζz}·sin(αx)·cos(βy)` (P2-1 L117).
pub fn tripp2003_trace_tangential(
    q0: f64,
    alpha: f64,
    beta: f64,
    nu: f64,
    x: f64,
    y: f64,
    z: f64,
) -> f64 {
    let zeta = (alpha * alpha + beta * beta).sqrt();
    2.0 * (1.0 + nu) * q0 * (alpha / zeta) * (-zeta * z).exp() * (alpha * x).sin() * (beta * y).cos()
}

// ═════════════════════════════════════════════════════════════════════════
//  McEwen — 고전 선접촉 Hertz 표면하 응력 (평면변형)
// ═════════════════════════════════════════════════════════════════════════

/// McEwen 축상(`x=0`) 무차원 응력 `(σx/p₀, σy/p₀, σz/p₀)`, `zeta = z/b`. 평면변형.
///
/// `σx/p₀ = −[(1+2ζ²)/√(1+ζ²) − 2ζ]` · `σz/p₀ = −1/√(1+ζ²)` · `σy = ν(σx+σz)`.
#[inline]
pub fn mcewen_axial_stresses(zeta: f64, nu: f64) -> (f64, f64, f64) {
    let m = (1.0 + zeta * zeta).sqrt();
    let sx = -((1.0 + 2.0 * zeta * zeta) / m - 2.0 * zeta);
    let sz = -1.0 / m;
    let sy = nu * (sx + sz);
    (sx, sy, sz)
}

/// McEwen 축상 von Mises `σ_vM/p₀`, `zeta = z/b`.
///
/// **최대 ≈ 0.557 @ z/b ≈ 0.70** (ν=0.3) — M3 의 **가장 강한 독립 외부 앵커**
/// (P2-2 재검토 VC-M3-Hertz, A등급).
#[inline]
pub fn mcewen_von_mises_over_p0(zeta: f64, nu: f64) -> f64 {
    let (sx, sy, sz) = mcewen_axial_stresses(zeta, nu);
    (0.5 * ((sx - sy).powi(2) + (sy - sz).powi(2) + (sz - sx).powi(2))).sqrt()
}

/// McEwen 축상 vM 최대와 그 깊이 `(σ_vM/p₀, z/b)` — `zeta ∈ (0, 1.2)` 를 `dz` 간격 탐색.
pub fn mcewen_von_mises_peak(nu: f64, dz: f64) -> (f64, f64) {
    let mut best = (0.0_f64, 0.0_f64);
    let mut z = dz;
    while z < 1.2 {
        let v = mcewen_von_mises_over_p0(z, nu);
        if v > best.0 {
            best = (v, z);
        }
        z += dz;
    }
    best
}

/// Hertz 선접촉 압력분포 `p(x) = p_h·√(1 − (x/b)²)` (|x|<b), 그 외 0.
#[inline]
pub fn hertz_line_pressure(p_h: f64, b: f64, x_rel: f64) -> f64 {
    let xr = x_rel / b;
    if xr.abs() < 1.0 {
        p_h * (1.0 - xr * xr).sqrt()
    } else {
        0.0
    }
}

// ═════════════════════════════════════════════════════════════════════════
//  Milano 2006 — Dang Van 닫힌해 (Appendix A) [ref(34)]
// ═════════════════════════════════════════════════════════════════════════
//
// 하중(A.5): σ_x = σ_m + σ_a·cos ωt (σ_m = −σ_a), τ_xy = τ_a·sin ωt, 그 외 0.
// ★ 이 닫힌해는 `m4_fatigue` 구현에 **미사용** → MCE(식5)+Tresca(식2) 기계를 독립검증한다.

/// Milano 식(A.8) `τ_DV = ½·√(σ_a²cos²ωt + 4τ_a²sin²ωt)` [Pa].
#[inline]
pub fn milano2006_tau_dv(sigma_a: f64, tau_a: f64, wt: f64) -> f64 {
    0.5 * ((sigma_a * wt.cos()).powi(2) + 4.0 * (tau_a * wt.sin()).powi(2)).sqrt()
}

/// Milano 식(A.10) `σ_H = (σ_a/3)·(cos ωt − 1)` [Pa].
#[inline]
pub fn milano2006_sigma_h(sigma_a: f64, wt: f64) -> f64 {
    (sigma_a / 3.0) * (wt.cos() - 1.0)
}

/// Milano 식(A.6) 잔류편차 `ρ* = diag(−⅔σ_m, σ_m/3, σ_m/3)` 의 반대텐서 `z* = −ρ*`.
///
/// 단축 맥동하중에서 MCE 중심(식5)이 이를 재현해야 한다.
#[inline]
pub fn milano2006_z_star_uniaxial(sigma_m: f64) -> [f64; 3] {
    [2.0 / 3.0 * sigma_m, -sigma_m / 3.0, -sigma_m / 3.0]
}

// ═════════════════════════════════════════════════════════════════════════
//  Desimone 2006 — Dang Van 재료상수·실험 [ref(33)]
// ═════════════════════════════════════════════════════════════════════════

/// Desimone 식(7) `a_DV = 3·(τ_W/σ_W − ½)`.
///
/// `τ_W/σ_W = 1/√3` → `a_DV ≈ 0.2320508` (SKF 단일선 채택값과 일치 = 문헌 교차검증).
#[inline]
pub fn desimone2006_alpha_dv(tau_w_over_sigma_w: f64) -> f64 {
    3.0 * (tau_w_over_sigma_w - 0.5)
}

/// Desimone Table — R비별 실험 `τ_W/σ_W`. `(R, τ_W/σ_W)`.
///
/// ⚠️ **상태갱신(P2-2 재검토 VC-M4-DesiTab)**: 코드의 단일선은 0.859/0.801 을 예측해 실험과
/// 어긋나지만 이는 **정상 거동**이다 — Desimone 가 **2-slope locus** 를 제안한 바로 그 근거이며,
/// 우리는 SKF 단일선을 채택했다. 뷰어에서 **실패로 표시하지 말 것**.
pub const DESIMONE2006_R_RATIO_TABLE: [(f64, f64); 2] = [(0.1, 0.76), (0.3, 0.68)];

// ═════════════════════════════════════════════════════════════════════════
//  Archard 1953 — 실험 데이터 **만** [ref(17)]
// ═════════════════════════════════════════════════════════════════════════
//
// ★ 폐형식(`Δh_w/Δn = k·p·u_s/H`)은 **여기 두지 않는다** — 그것은 `m5_wear::wear_depth_rate`
//   자체이므로, 복제하면 "같은 식의 복제본끼리 비교" = tautology 를 새로 만든다(모듈 docstring).
//   M5 의 물리 앵커는 테스트 안의 **손유도 리터럴**(1.428e-15)·H 위치·k·p 선형성이다.

/// Archard Fig 7 — 마모율/하중 log-log **기울기**, 황동. 원문 L306 verbatim.
///
/// *"The slopes of these wear rate/load graphs are **1.00 for brass** and 0.98 for stellite
/// (standard error 0.015 in each case)."* → 마모율 ∝ 하중^1.00 = Archard 선형 법칙의 실험 근거.
pub const ARCHARD1953_FIG7_SLOPE_BRASS: f64 = 1.00;

/// Archard Fig 7 — 동 기울기, 스텔라이트 (원문 L306).
pub const ARCHARD1953_FIG7_SLOPE_STELLITE: f64 = 0.98;

/// Archard Fig 7 — 위 두 기울기의 표준오차 (원문 L306: *"standard error 0.015 in each case"*).
pub const ARCHARD1953_FIG7_SLOPE_STD_ERR: f64 = 0.015;

// ═════════════════════════════════════════════════════════════════════════
//  구조 가드 — leaf 불변식의 기계적 강제 (계획 §4.1)
// ═════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    /// 소스에서 `#[cfg(test)]` 이전 = **생산 코드** 부분만 잘라낸다.
    ///
    /// 테스트 코드가 reference 를 import 하는 것은 **허용**(그것이 용도)이므로 검사에서 뺀다.
    fn production_part(src: &str) -> &str {
        match src.find("#[cfg(test)]") {
            Some(i) => &src[..i],
            None => src,
        }
    }

    /// 주석을 제외한 생산 코드 줄.
    fn production_code_lines(src: &str) -> impl Iterator<Item = &str> {
        production_part(src)
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.starts_with("//"))
    }

    /// 모델 모듈 소스 — 컴파일 타임 임베드(파일 I/O 없음 → wasm/CI 무관하게 동작).
    const MODEL_SRCS: [(&str, &str); 7] = [
        ("m1_dry.rs", include_str!("m1_dry.rs")),
        ("m2_lub.rs", include_str!("m2_lub.rs")),
        ("m3_stress.rs", include_str!("m3_stress.rs")),
        ("m4_fatigue.rs", include_str!("m4_fatigue.rs")),
        ("m5_wear.rs", include_str!("m5_wear.rs")),
        ("m6_share.rs", include_str!("m6_share.rs")),
        ("partial_lub.rs", include_str!("partial_lub.rs")),
    ];

    /// ★ leaf 불변식 (2): **모델 생산코드가 reference 를 참조하지 않는다.**
    ///
    /// 위반 시 오라클이 모델 자신을 검증하게 되어 순환한다(M2 Q oracle tautology 와 동형).
    #[test]
    fn reference_is_leaf_not_used_by_model_production_code() {
        for (name, src) in MODEL_SRCS {
            for line in production_code_lines(src) {
                assert!(
                    !line.contains("reference"),
                    "leaf 불변식 위반: {name} 의 생산코드가 reference 를 참조 \
                     → 오라클 순환(tautology). 위반 줄: {line}"
                );
            }
        }
    }

    /// ★ leaf 불변식 (1): **reference 는 types·units 외 크레이트 내부를 import 하지 않는다.**
    #[test]
    fn reference_imports_nothing_but_types_and_units() {
        for line in production_code_lines(include_str!("reference.rs")) {
            if line.starts_with("use crate::") {
                assert!(
                    line.starts_with("use crate::types") || line.starts_with("use crate::units"),
                    "leaf 불변식 위반: reference 가 모델 모듈을 import — {line}"
                );
            }
        }
    }

    // ── 문헌값 자체 sanity (전사 오류 검출; 물리 검증은 각 모듈 VC 가 담당) ──

    #[test]
    fn venner1997_eq5_reproduces_table1_anchor_spots() {
        // 총괄계획 L476 anti-fudge 앵커: A_i/H_c=0.1 열, 2~5 % 내, 튜닝 금지.
        for (lob, table) in VENNER1997_ANCHOR_SPOTS {
            let nabla = venner1997_nabla(lob, VENNER1997_M, VENNER1997_L);
            let fit = venner1997_amplitude_reduction(nabla);
            let rel = (fit - table).abs() / table;
            assert!(rel <= 0.05, "eq(5) vs Table1 @λ/b={lob}: fit={fit:.4} table={table} rel={rel:.3}");
        }
        // 앵커 스팟이 Table1 의 A_i/H_c=0.1 열과 실제로 일치하는지(전사 오류 방지).
        for (lob, ad) in VENNER1997_ANCHOR_SPOTS {
            let row = VENNER1997_TABLE1.iter().find(|(l, _)| *l == lob).expect("anchor λ/b in Table1");
            assert_relative_eq!(row.1[0], ad, max_relative = 1e-12);
        }
    }

    #[test]
    fn venner1997_eq5_limits_and_monotonicity() {
        assert!(venner1997_amplitude_reduction(1e-4) > 0.999, "∇→0 → 1");
        assert!(venner1997_amplitude_reduction(1e4) < 1e-3, "∇→∞ → 0");
        let mut prev = 2.0;
        for i in 1..=40 {
            let v = venner1997_amplitude_reduction(0.5 * i as f64);
            assert!(v < prev, "단조 감소 위반");
            prev = v;
        }
    }

    #[test]
    fn venner1997_inversion_roundtrips() {
        for &g in &[0.2_f64, 0.4, 0.5, 0.66, 0.9] {
            let nabla = venner1997_nabla_from_ratio(g).expect("가역");
            assert_relative_eq!(venner1997_amplitude_reduction(nabla), g, max_relative = 1e-9);
        }
        assert!(venner1997_nabla_from_ratio(0.0).is_none());
        assert!(venner1997_nabla_from_ratio(1.5).is_none());
    }

    #[test]
    fn venner1997_half_crossing_bracket_matches_table1() {
        // 구간이 Table1 에서 실제로 0.5 를 교차하는지 = 상수의 근거 확인.
        let (lo, hi) = VENNER1997_HALF_CROSSING_BRACKET;
        let at = |lob: f64| VENNER1997_TABLE1.iter().find(|(l, _)| *l == lob).unwrap().1[0];
        assert!(at(lo) > 0.5, "λ/b={lo} → {} 는 0.5 초과여야", at(lo));
        assert!(at(hi) < 0.5, "λ/b={hi} → {} 는 0.5 미만이어야", at(hi));
    }

    #[test]
    fn venner2000_eq29_matches_example_and_numerics() {
        let (m, l, lam_over_a) = VENNER2000_EXAMPLE;
        let nab2 = venner2000_nabla2(lam_over_a, m, l);
        assert!((nab2 - 2.2861).abs() < 1e-3, "∇₂={nab2}");
        let ratio = venner2000_amplitude_reduction(nab2, venner2000_f_bar(1.0));
        assert!((ratio - 0.70358).abs() < 1e-4, "eq(29)={ratio}");
        assert!(
            (ratio - VENNER2000_EXAMPLE_NUMERICS).abs() / VENNER2000_EXAMPLE_NUMERICS < 0.06,
            "fit↔numerics <6 % 여야: {ratio}"
        );
    }

    #[test]
    fn venner2000_f_bar_definition() {
        assert_relative_eq!(venner2000_f_bar(1.0), 1.0, max_relative = 1e-15);
        assert_relative_eq!(venner2000_f_bar(0.5), 1.0, max_relative = 1e-15); // r≤1 → 1
        assert_relative_eq!(venner2000_f_bar(2.0), (0.5_f64).exp(), max_relative = 1e-12);
    }

    /// ★ 선접촉(1997) ↔ 점접촉(2000) **카테고리 오류 차단**(총괄계획 L476).
    ///
    /// 같은 (λ/b, M, L) 에 두 ∇ 를 넣으면 서로 다른 값이 나온다 — 두 축을 겹쳐 그리면 안 되는
    /// 이유를 기계적으로 고정한다.
    #[test]
    fn line_and_point_nabla_are_not_interchangeable() {
        let (m, l) = (1007.6, 12.05);
        let line = venner1997_nabla(0.25, m, l);
        let point = venner2000_nabla2(0.25, m, l);
        assert!(
            (line - point).abs() / point > 0.5,
            "선접촉 ∇={line} 와 점접촉 ∇₂={point} 가 유사 — 혼용 위험"
        );
    }

    #[test]
    fn gw1994_closed_form_limits_and_scaling() {
        let e_prime = GW1994_E_PRIME_PA;
        let (alpha, hbar) = GW1994_TABLE2_KWEH[0];
        let c = 0.03_f64;
        // 극한: 단파장 A→0 → H₁/Z₁→1, 장파장 A→∞ → 0.
        let a_tiny = gw1994_a(1e-7, alpha, hbar, e_prime);
        let a_huge = gw1994_a(1.0, alpha, hbar, e_prime);
        assert!(gw1994_h1_over_z1(c, a_tiny) > 0.99, "단파장: 거칠기 지속");
        assert!(gw1994_h1_over_z1(c, a_huge) < 0.01, "장파장: 평탄화");
        // 식(17) Aₙ = A/n (A ∝ λ).
        let a1 = gw1994_a(4e-4, alpha, hbar, e_prime);
        let a4 = gw1994_a(4e-4 / 4.0, alpha, hbar, e_prime);
        assert_relative_eq!(a4, a1 / 4.0, max_relative = 1e-12);
        // 단조성: λ↑ → A↑ → H₁/Z₁↓, |P₁/Z₁|↓.
        let mut prev_h = f64::INFINITY;
        let mut prev_p = f64::INFINITY;
        for &lam in &[1e-5, 3e-5, 1e-4, 3e-4, 1e-3] {
            let a = gw1994_a(lam, alpha, hbar, e_prime);
            let h = gw1994_h1_over_z1(c, a);
            let p = gw1994_p1_over_z1(c, a).abs();
            assert!(h < prev_h && p < prev_p, "λ={lam} 단조성 위반");
            assert!(h > 0.0 && h < 1.0, "0<H₁/Z₁<1");
            prev_h = h;
            prev_p = p;
        }
    }

    /// GW1994 Table 1 "Present theory" 스팟 — C 를 소거한 A 와 기하 A 대조(자유계수 0).
    #[test]
    fn gw1994_table1_present_theory_spots() {
        for (i, (p1, h1)) in GW1994_TABLE1_PRESENT.iter().enumerate() {
            let (alpha, hbar) = GW1994_TABLE2_KWEH[i];
            let a_data = gw1994_a(GW1994_LAMBDA_M, alpha, hbar, GW1994_E_PRIME_PA);
            let a_inf = (GW1994_Z1_M - h1) / (alpha * p1 * hbar);
            let rel = (a_data - a_inf).abs() / a_data;
            assert!(rel < 1.0e-2, "GW Table1 스팟[{i}]: A_data={a_data:.5} A_inf={a_inf:.5} rel={rel:.2e}");
        }
    }

    /// Tripp 식[10]/[16] 이 **자신의 trace 항등**을 만족 — 전사 오류 검출.
    ///
    /// 항등은 6성분식과 **독립 유도**(P2-1 L116/L117)이므로, 성분 전사가 틀리면 여기서 깨진다.
    #[test]
    fn tripp2003_components_satisfy_trace_identity() {
        let (p0, q0, nu) = (1.0e9, 0.7e9, 0.3);
        let (alpha, beta) = (2.0 * PI * 3.0 / 1e-4, 2.0 * PI * 2.0 / 1e-4);
        for &(x, y, z) in &[
            (0.0, 0.0, 0.0),
            (1e-5, 2e-5, 1e-6),
            (3e-5, 1e-5, 5e-6),
            (7e-6, 9e-6, 2e-6),
        ] {
            let s = tripp2003_normal_bisin(p0, alpha, beta, nu, x, y, z);
            let tr = s[0] + s[1] + s[2];
            let expect = tripp2003_trace_normal(p0, alpha, beta, nu, x, y, z);
            assert!((tr - expect).abs() <= p0 * 1e-9, "식[10] trace: {tr} vs {expect}");

            let t = tripp2003_tangential_bisin(q0, alpha, beta, nu, x, y, z);
            let tr_t = t[0] + t[1] + t[2];
            let expect_t = tripp2003_trace_tangential(q0, alpha, beta, nu, x, y, z);
            assert!((tr_t - expect_t).abs() <= q0 * 1e-9, "식[16] trace: {tr_t} vs {expect_t}");
        }
    }

    /// 표면 경계조건 `z=0`: 법선 → `σzz = p`, 접선 → `σxz = q`.
    #[test]
    fn tripp2003_surface_boundary_conditions() {
        let (p0, q0, nu) = (1.0e9, 1.0e9, 0.3);
        let (alpha, beta) = (2.0 * PI * 3.0 / 1e-4, 2.0 * PI * 2.0 / 1e-4);
        let (x, y) = (1.3e-5, 0.7e-5);
        let cc = (alpha * x).cos() * (beta * y).cos();
        let s = tripp2003_normal_bisin(p0, alpha, beta, nu, x, y, 0.0);
        assert_relative_eq!(s[2], p0 * cc, max_relative = 1e-12); // σzz = p
        let t = tripp2003_tangential_bisin(q0, alpha, beta, nu, x, y, 0.0);
        assert_relative_eq!(t[5], q0 * cc, max_relative = 1e-12); // σxz = q
    }

    #[test]
    fn mcewen_peak_is_classical_value() {
        let (vm, z) = mcewen_von_mises_peak(0.3, 0.001);
        assert!((vm - 0.557).abs() < 0.01, "McEwen 최대 vM/p₀ = {vm} (고전값 ≈0.557)");
        assert!((z - 0.70).abs() < 0.06, "McEwen 피크 깊이 z/b = {z} (고전값 ≈0.70)");
    }

    #[test]
    fn hertz_line_pressure_shape() {
        let (p_h, b) = (1.0e9, 1e-4);
        assert_relative_eq!(hertz_line_pressure(p_h, b, 0.0), p_h, max_relative = 1e-12);
        assert_relative_eq!(hertz_line_pressure(p_h, b, 0.5 * b), p_h * 0.75_f64.sqrt(), max_relative = 1e-12);
        assert_eq!(hertz_line_pressure(p_h, b, 1.5 * b), 0.0, "접촉 밖은 0");
    }

    #[test]
    fn milano2006_closed_form_spot_values() {
        let (sa, ta) = (200.0e6, 150.0e6);
        // ωt=0: cos=1, sin=0 → τ_DV=σ_a/2, σ_H=0.
        assert_relative_eq!(milano2006_tau_dv(sa, ta, 0.0), 0.5 * sa, max_relative = 1e-12);
        assert_relative_eq!(milano2006_sigma_h(sa, 0.0), 0.0, epsilon = 1e-6);
        // ωt=π/2: cos=0, sin=1 → τ_DV=τ_a, σ_H=−σ_a/3.
        assert_relative_eq!(milano2006_tau_dv(sa, ta, 0.5 * PI), ta, max_relative = 1e-12);
        assert_relative_eq!(milano2006_sigma_h(sa, 0.5 * PI), -sa / 3.0, max_relative = 1e-12);
        // ωt=π: cos=−1 → τ_DV=σ_a/2, σ_H=−2σ_a/3.
        assert_relative_eq!(milano2006_tau_dv(sa, ta, PI), 0.5 * sa, max_relative = 1e-12);
        assert_relative_eq!(milano2006_sigma_h(sa, PI), -2.0 * sa / 3.0, max_relative = 1e-12);
    }

    #[test]
    fn milano2006_z_star_is_traceless_deviator() {
        let z = milano2006_z_star_uniaxial(-200.0e6);
        assert_relative_eq!(z[0] + z[1] + z[2], 0.0, epsilon = 1e-6);
    }

    #[test]
    fn desimone2006_alpha_dv_matches_skf_value() {
        // τ_W/σ_W = 1/√3 → a_DV = 3(1/√3 − ½) ≈ 0.2320508 (SKF 채택값과 문헌 교차검증).
        let a = desimone2006_alpha_dv(1.0 / 3.0_f64.sqrt());
        assert_relative_eq!(a, 0.232_050_807_568_877_2, max_relative = 1e-12);
    }

    #[test]
    fn archard1953_slopes_are_linear_law_evidence() {
        // 기울기 ≈1 = 마모율 ∝ 하중^1 = Archard 선형법칙. 표준오차 0.015 내에서 1.00 과 정합.
        assert!((ARCHARD1953_FIG7_SLOPE_BRASS - 1.0).abs() <= ARCHARD1953_FIG7_SLOPE_STD_ERR);
        assert!((ARCHARD1953_FIG7_SLOPE_STELLITE - 1.0).abs() <= 2.0 * ARCHARD1953_FIG7_SLOPE_STD_ERR);
    }
}
