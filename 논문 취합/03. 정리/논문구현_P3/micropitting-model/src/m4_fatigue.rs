//! # M4 — 다축피로 (Dang Van 중간척도 기준 + MCE)
//!
//! M3([`crate::m3_stress`])의 표면하 응력장 [`StressResult`] 을 **시간이력**(정상상태
//! 이동하중에서 x-스냅샷 = 시간, `T = x/ū`)으로 소비해 각 물질점의 Dang Van 위험계수
//! `D` 와 수명 `N` 을 산출한다. 결과는 [`FatigueResult`](crate::types::FatigueResult).
//!
//! ## 근거 원문 유도 (자의계수 금지 — 식·계수는 원문 소급)
//!
//! ### Desimone 2006 (정본 — τ̂/p̂ 계산법)
//! - **식(1)** Dang Van 기준:  `τ_max(t) + a_DV·σ_H(t) = τ_W`
//!   (τ_W = 반전(reversed) 비틀림 피로한계, σ_H = 순간 정수압, τ_max = 순간 Tresca 전단).
//! - **식(2)** 대칭화 편차응력의 Tresca 주값:  `τ_max(t) = (ŝ_I(t) − ŝ_III(t))/2`.
//! - **식(3)** 편차응력:  `s_ij(t) = σ_ij(t) − δ_ij·σ_H(t)`,  `σ_H = tr(σ)/3`.
//! - **식(4)** 대칭화(중간척도) 편차:  `ŝ_ij(t) = s_ij(t) − s_ij,m`.
//! - **식(5)** 상수텐서 `s_ij,m` = 편차응력경로의 **최소외접초구**(min circumscribed
//!   hypersphere, Chebyshev 중심)  =  `argmin_{s'} max_t |s(t) − s'|`.
//! - **식(7)** 기울기(교정):  `a_DV = 3(τ_W/σ_W − 1/2)`.  von Mises `τ_W/σ_W = 1/√3` →
//!   `a_DV = 3(1/√3 − 1/2) ≈ 0.2320508` (본 구현 단일선 채택; 상수 [`ALPHA_DV`]).
//! - **L109/L134**: 대칭화(식5)로 **잔류 전단응력은 불필요**(소거됨); 잔류 정수압만 σ_H 를
//!   평행이동한다. 본 구현은 **무잔류(σ_H,res = 0)** 가정 → macro=meso 정수압(L61).
//!
//! ### Milano/Bernasconi 2006 (닫힌해 오라클)
//! - **식(2)** `max_t[τ_DV(t) + α·σ_H(t)] = β`,  **식(4)** `α = 3(τ_W/σ_W − 1/2)`, `β = τ_W`
//!   (Desimone 식(1)(7)과 1:1 동일 — 상호 확증).
//! - **App A.1**: `τ_DV(t) = Tresca[ŝ(t)] = (ŝ_I − ŝ_III)/2` (Desimone 식2와 동일).
//! - **App A.2–A.4**: `ŝ(t) = S(t) + ρ*`,  `ρ* = −z*`,
//!   `z* = argmin_z max_t √((S(t)−z):(S(t)−z))` (식5 MCE 와 동일; 편차공간 최소외접초구).
//! - **App A.5–A.10** (닫힌해, σ_x=σ_m+σ_a cos ωt, σ_m=−σ_a, τ_xy=τ_a sin ωt):
//!   `ρ* = diag(−2/3·σ_m, σ_m/3, σ_m/3)`  (A.6),
//!   `τ_DV(t) = ½√(σ_a² cos²ωt + 4τ_a² sin²ωt)`  (A.8),
//!   `σ_H(t) = (σ_a/3)(cos ωt − 1)`  (A.10).
//!   → **VC-M4-Milano / VC-M4-MesoA 오라클**(구현의 MCE+Tresca 기계를 이 닫힌해로 독립검증;
//!   닫힌해는 구현에 미사용).
//!
//! ### Wöhler (수명) — G-M4-3 (SO Q4 확정)
//! - `σ_f(N) = A·ln N + B`,  **A = −43.0 MPa, B = 1220 MPa** (2011 재적합값; 상수 확정).
//!   A<0 → N↑ 시 σ_f↓ 정합. `N_ref = 1e6`(가정). `τ_f(N) = σ_f(N)/√3` (von Mises).
//!
//! ## 파이프라인 (물질점 = 고정 (y,z); 시간 = x-스냅샷)
//! 정상상태 이동하중에서 (y,z)에 고정된 물질점이 겪는 응력 시간열 = 그 (y,z)의 x-프로파일
//! `{ σ_ij(x,y,z) : x = 0..nx }`. 각 x 가 시간스텝. Dang Van `D`·수명 `N` 은 한 시간열(= 한
//! 통과)당 **스칼라 1개**(max_t) 이므로, 결과 [`Field2`](crate::types::Field2)(nx×ny)의 한
//! 열(고정 j)은 x축으로 **동일값 broadcast**(모든 x 위치가 동일 이력·시간이동만 다름 → 동일 D).
//!
//! ## SSOT / 규약
//! - 응력 부호 **인장 +** (M3 인계). 접촉하 σ_H < 0(압축) → 식(1) 분모 `τ_f − a_DV·σ_H` 증가
//!   → 압축이 피로한도 상향(Dang Van 물리) 정합.
//! - `alpha_dv`(≈0.232, 무차원) 은 `alpha_wave`(1/m)·`alpha_visc`(1/Pa)와 **혼용 금지**.

use crate::types::{Field2, FatigueResult, StressResult};
#[cfg(feature = "parallel")]
use rayon::prelude::*;

// ─────────────────────────────────────────────────────────────────────────
//  상수 (근거: Desimone 식(7)·Milano 식(4) / G-M4-3 SO Q4)
// ─────────────────────────────────────────────────────────────────────────

/// Wöhler 절편 계수 `A` [Pa].  `σ_f(N) = A·ln N + B`.  A<0 (N↑→σ_f↓; G-M4-3 SO Q4).
///
/// **근거·가정**: A=−43.0/B=1220 MPa 는 **P2-2 §T3-1(SKF p.627 verbatim)** 확정값. 단,
/// A/B 는 Shimizu(19) torsion τ–N 을 2011 저자가 **반-로그 재적합**한 값(원저자 Shimizu 는
/// 반-로그 부적절 명시; **P2-3 §3.4 L137** 귀속주의) — 단순화임을 flag.
pub const WOHLER_A_PA: f64 = -43.0e6;
/// Wöhler 상수항 `B` [Pa] (= 외삽 `N=1` 피로강도).  G-M4-3 SO Q4 (P2-2 §T3-1 verbatim).
pub const WOHLER_B_PA: f64 = 1220.0e6;
/// 위험계수 `D` 평가 기준 수명 [cycles].  **가정 flag**: `N_ref = 1e6` — 원문은 `N_ref>1e6`
/// 만 명시(P2 RP-M4-Nref, Desimone Ref33), 그 **하한 채택**(Q8 승인, 민감도 {1e6,1e7}).
pub const N_REF: f64 = 1.0e6;

/// Dang Van 정수압 민감계수 `a_DV` [-] = `3(τ_W/σ_W − 1/2)` (Desimone 식7 / Milano 식4).
///
/// von Mises `τ_W/σ_W = 1/√3` 채택 → `3(1/√3 − 1/2) = 0.2320508075688772`.
/// **가정 flag**: 단일선(single-line) 채택 + `τ_W/σ_W = 1/√3` 고정(Desimone Table 은 이중기울기
/// 를 제안하나, 본 구현은 SO Q3 로 원 단일선 채택).
pub const ALPHA_DV: f64 = 0.232_050_807_568_877_2;

/// √3 (von Mises `σ_W = √3·τ_W`).
const SQRT3: f64 = 1.732_050_807_568_877_2;
/// √2 (편차텐서 → 등거리 벡터화 시 전단성분 가중).
const SQRT2: f64 = std::f64::consts::SQRT_2;

// ─────────────────────────────────────────────────────────────────────────
//  파라미터 (types.rs 동결 — 모듈-로컬 struct; 상수 기본값 + 민감도 스윕용)
// ─────────────────────────────────────────────────────────────────────────

/// M4 Dang Van/Wöhler 파라미터 (기본값 = 위 확정 상수; 민감도 스윕용 오버라이드 가능).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FatigueParams {
    /// Wöhler A [Pa] (기본 [`WOHLER_A_PA`]).
    pub wohler_a: f64,
    /// Wöhler B [Pa] (기본 [`WOHLER_B_PA`]).
    pub wohler_b: f64,
    /// 위험계수 기준 수명 [cycles] (기본 [`N_REF`]).
    pub n_ref: f64,
    /// 정수압 민감계수 a_DV [-] (기본 [`ALPHA_DV`]).
    pub alpha_dv: f64,
}

impl Default for FatigueParams {
    fn default() -> Self {
        FatigueParams {
            wohler_a: WOHLER_A_PA,
            wohler_b: WOHLER_B_PA,
            n_ref: N_REF,
            alpha_dv: ALPHA_DV,
        }
    }
}

/// 한 물질점(한 시간열)의 Dang Van 중간척도 산출물.
///
/// - `tau_hat[t]` : 대칭화 편차의 Tresca 전단 `(ŝ_I − ŝ_III)/2` [Pa] (식2/A.1).
/// - `p_hat[t]`   : 순간 정수압 `σ_H = tr(σ)/3` [Pa] (식3; 무잔류 macro=meso).
/// - `z_star`     : MCE 중심(편차공간, 6성분 텐서표기 [sxx,syy,szz,sxy,syz,sxz]) [Pa] (식5).
#[derive(Debug, Clone, PartialEq)]
pub struct PointResult {
    /// 대칭화 Tresca 전단 시간열 [Pa].
    pub tau_hat: Vec<f64>,
    /// 정수압 시간열 [Pa].
    pub p_hat: Vec<f64>,
    /// MCE 중심 편차텐서(6성분) [Pa].
    pub z_star: [f64; 6],
}

// ─────────────────────────────────────────────────────────────────────────
//  σ_f / τ_f (Wöhler)
// ─────────────────────────────────────────────────────────────────────────

/// 인장-압축 피로강도 `σ_f(N) = A·ln N + B` [Pa].
#[inline]
pub fn sigma_f(n: f64, a: f64, b: f64) -> f64 {
    a * n.ln() + b
}

/// 반전 비틀림 피로한계 `τ_f(N) = σ_f(N)/√3` [Pa] (von Mises).
#[inline]
pub fn tau_f(n: f64, a: f64, b: f64) -> f64 {
    sigma_f(n, a, b) / SQRT3
}

// ─────────────────────────────────────────────────────────────────────────
//  대칭 3×3 고유값 (Tresca 주값) — 폐형식(자립; nalgebra 회피, hot-loop 고속)
// ─────────────────────────────────────────────────────────────────────────

/// 대칭 3×3 행렬 `[[m00,m01,m02],[m01,m11,m12],[m02,m12,m22]]` 의 (최대, 최소) 고유값.
///
/// Smith(1961) 삼각함수 폐형식: `λ = q + 2p·cos(φ + 2kπ/3)`, `q = tr/3`,
/// `p = √(Σ(dev²)/6)`, `φ = ⅓·acos(det(B)/2)`, `B = (M−qI)/p`. 대칭행렬이라 실근 보장.
#[inline]
fn eig_sym3_minmax(m00: f64, m11: f64, m22: f64, m01: f64, m02: f64, m12: f64) -> (f64, f64) {
    let p1 = m01 * m01 + m02 * m02 + m12 * m12;
    if p1 <= 1e-30 {
        // 대각 → 성분이 곧 고유값.
        let mx = m00.max(m11).max(m22);
        let mn = m00.min(m11).min(m22);
        return (mx, mn);
    }
    let q = (m00 + m11 + m22) / 3.0;
    let p2 = (m00 - q).powi(2) + (m11 - q).powi(2) + (m22 - q).powi(2) + 2.0 * p1;
    let p = (p2 / 6.0).sqrt();
    let b00 = (m00 - q) / p;
    let b11 = (m11 - q) / p;
    let b22 = (m22 - q) / p;
    let b01 = m01 / p;
    let b02 = m02 / p;
    let b12 = m12 / p;
    // det(B)/2
    let detb = b00 * (b11 * b22 - b12 * b12) - b01 * (b01 * b22 - b12 * b02)
        + b02 * (b01 * b12 - b11 * b02);
    let mut r = detb / 2.0;
    if r > 1.0 {
        r = 1.0;
    } else if r < -1.0 {
        r = -1.0;
    }
    let phi = r.acos() / 3.0;
    // λ_k = q + 2p·cos(φ + 2kπ/3): k=0 최대, k=1(=φ+2π/3) 최소, k=2 중간(Smith 정렬).
    let e_max = q + 2.0 * p * phi.cos();
    let e_min = q + 2.0 * p * (phi + 2.0 * std::f64::consts::FRAC_PI_3).cos();
    (e_max, e_min)
}

// ─────────────────────────────────────────────────────────────────────────
//  MCE — 최소외접초구 중심 (식5; Bădoiu–Clarkson 반복 1-center)
// ─────────────────────────────────────────────────────────────────────────

/// 6D 가중벡터 집합의 최소외접구(MEB) 중심 — Bădoiu–Clarkson 반복법.
///
/// **알고리즘**: 중심 `c` 를 중심점(centroid)으로 초기화 → 매 반복 `i` 에서 `c` 로부터 가장 먼
/// 점 `p_far` 를 찾아 `c ← c + (p_far − c)/(i+1)` (Frank–Wolfe on the MEB dual과 동치).
/// **수렴**: 반경오차 `R_i ≤ (1+1/i)·R*`. 관측 외접반경이 최소인 중심(유효 상한)을 보존하고,
/// 64회 비개선 시 조기종료(캡 2000). 매끈 주기경로(RCF·Milano 타원)에서는 중심이 centroid
/// 로 빠르게 수렴(대칭 → centroid 가 정확), VC-M4-Milano(≤1%)로 검증.
///
/// **편차공간 유지**: 입력점이 모두 trace=0 부분공간(가중 첫 3성분 합=0)에 있으면 MEB 중심도
/// 그 부분공간에 있음(부분공간 밖 성분은 모든 거리를 늘리므로) → z* 는 자동으로 편차텐서.
fn meb_center6(points: &[[f64; 6]]) -> [f64; 6] {
    let n = points.len();
    if n == 0 {
        return [0.0; 6];
    }
    if n == 1 {
        return points[0];
    }
    // centroid 초기화.
    let mut c = [0.0f64; 6];
    for p in points {
        for k in 0..6 {
            c[k] += p[k];
        }
    }
    let inv = 1.0 / n as f64;
    for k in 0..6 {
        c[k] *= inv;
    }

    let farthest = |c: &[f64; 6]| -> (usize, f64) {
        let mut bi = 0usize;
        let mut br2 = -1.0f64;
        for (idx, p) in points.iter().enumerate() {
            let mut d2 = 0.0;
            for k in 0..6 {
                let dv = p[k] - c[k];
                d2 += dv * dv;
            }
            if d2 > br2 {
                br2 = d2;
                bi = idx;
            }
        }
        (bi, br2)
    };

    let max_iter = 2000usize;
    let mut best_c = c;
    let (_, mut best_r2) = farthest(&c);
    let mut stall = 0usize;
    for i in 1..=max_iter {
        let (far, r2) = farthest(&c);
        if r2 < best_r2 {
            best_r2 = r2;
            best_c = c;
            stall = 0;
        } else {
            stall += 1;
            if stall > 64 {
                break;
            }
        }
        let step = 1.0 / (i as f64 + 1.0);
        for k in 0..6 {
            c[k] += step * (points[far][k] - c[k]);
        }
    }
    best_c
}

// ─────────────────────────────────────────────────────────────────────────
//  단일 물질점 Dang Van (식3→식5→식4→식2)
// ─────────────────────────────────────────────────────────────────────────

/// 한 물질점의 응력 시간열(각 `[σxx,σyy,σzz,σxy,σyz,σxz]` [Pa])로부터 Dang Van 중간척도량.
///
/// (a) 편차 `s(t)=σ−σ_H·I`(식3) → (b) MCE 중심 `z*`(식5, 편차공간 6→가중벡터 최소외접구,
/// 전단성분 √2 가중으로 `|·|² = s:s` 보존) → (c) `ŝ=s−z*`(식4) → (d) `τ̂=(ŝ_I−ŝ_III)/2`(식2),
/// `p̂=σ_H`(식3). 닫힌해 미사용 → MCE+Tresca 기계의 독립검증 대상.
pub fn dang_van_point(history: &[[f64; 6]]) -> PointResult {
    let n = history.len();
    let mut devs: Vec<[f64; 6]> = Vec::with_capacity(n);
    let mut p_hat: Vec<f64> = Vec::with_capacity(n);
    let mut wpts: Vec<[f64; 6]> = Vec::with_capacity(n);
    for s in history {
        let sh = (s[0] + s[1] + s[2]) / 3.0; // σ_H = tr/3 (식3)
        let d = [s[0] - sh, s[1] - sh, s[2] - sh, s[3], s[4], s[5]]; // 편차 s (식3)
        devs.push(d);
        p_hat.push(sh);
        // 가중벡터: 전단 ×√2 → 유클리드 |w|² = s:s (이중축약 노름 보존).
        wpts.push([d[0], d[1], d[2], SQRT2 * d[3], SQRT2 * d[4], SQRT2 * d[5]]);
    }

    let cw = meb_center6(&wpts);
    // z* 편차텐서(전단 성분은 √2 역가중).
    let z = [cw[0], cw[1], cw[2], cw[3] / SQRT2, cw[4] / SQRT2, cw[5] / SQRT2];

    let mut tau_hat: Vec<f64> = Vec::with_capacity(n);
    for d in &devs {
        // ŝ = s − z* (식4).
        let sxx = d[0] - z[0];
        let syy = d[1] - z[1];
        let szz = d[2] - z[2];
        let sxy = d[3] - z[3];
        let syz = d[4] - z[4];
        let sxz = d[5] - z[5];
        // Tresca 주값: eig_sym3(m00,m11,m22, m01=xy, m02=xz, m12=yz).
        let (emax, emin) = eig_sym3_minmax(sxx, syy, szz, sxy, sxz, syz);
        tau_hat.push(0.5 * (emax - emin)); // (식2)
    }

    PointResult {
        tau_hat,
        p_hat,
        z_star: z,
    }
}

/// 위험계수 `D = max_t{ τ̂(t) / (τ_f − a_DV·p̂(t)) }` (식1 형).  D≥1 → 피로한계 초과.
///
/// `τ_f = σ_f(N_ref)/√3`. 분모 ≤0 (비물리 강인장) 이면 해당 순간 확실 파손 → +∞.
pub fn damage_from_point(pr: &PointResult, params: &FatigueParams) -> f64 {
    let tf = tau_f(params.n_ref, params.wohler_a, params.wohler_b);
    let mut d = 0.0f64;
    for t in 0..pr.tau_hat.len() {
        let denom = tf - params.alpha_dv * pr.p_hat[t];
        let ratio = if denom > 0.0 {
            pr.tau_hat[t] / denom
        } else {
            f64::INFINITY
        };
        if ratio > d {
            d = ratio;
        }
    }
    d
}

/// 수명 `N` [cycles] — `D(N)=1` 해(식[11]).
///
/// D=1 ⟺ `τ_f(N) = max_t{τ̂ + a_DV·p̂}`(식1 접선; 두 형이 D=1 등고선에서 일치) →
/// `σ_f_req = √3·max_t{τ̂+a_DV·p̂}` → `N = exp((σ_f_req − B)/A)` (A<0). 순수압축(τ̂+a_DV·p̂≤0)
/// 이면 무한수명(+∞).
pub fn life_from_point(pr: &PointResult, params: &FatigueParams) -> f64 {
    let mut dv = f64::NEG_INFINITY;
    for t in 0..pr.tau_hat.len() {
        let v = pr.tau_hat[t] + params.alpha_dv * pr.p_hat[t];
        if v > dv {
            dv = v;
        }
    }
    if dv <= 0.0 {
        return f64::INFINITY;
    }
    let sigma_req = SQRT3 * dv;
    let n = ((sigma_req - params.wohler_b) / params.wohler_a).exp();
    if n.is_finite() {
        n
    } else {
        f64::MAX
    }
}

// ─────────────────────────────────────────────────────────────────────────
//  진입점 — 응력장 → FatigueResult
// ─────────────────────────────────────────────────────────────────────────

/// M4 진입점: M3 응력장 [`StressResult`] → Dang Van 위험계수·수명 [`FatigueResult`].
///
/// 각 깊이층 `l`·y-열 `j` 마다 x-프로파일을 시간열로 삼아 [`dang_van_point`] → D·N 을 구하고,
/// 결과 [`Field2`](nx×ny)의 열 `j` 를 x축으로 동일값 broadcast(§파이프라인). 열(물질점) 병렬(rayon).
pub fn solve_fatigue(stress: &StressResult, params: &FatigueParams) -> FatigueResult {
    let nz = stress.z.len();
    // 퇴화 방어.
    let (nx, ny) = if nz == 0 {
        (0, 0)
    } else {
        (stress.stress[0].sxx.nx, stress.stress[0].sxx.ny)
    };
    if nz == 0 || nx == 0 || ny == 0 {
        return FatigueResult {
            z: stress.z.clone(),
            dang_van_d: (0..nz).map(|_| Field2::zeros(nx, ny)).collect(),
            life_n: (0..nz).map(|_| Field2::zeros(nx, ny)).collect(),
        };
    }

    let mut dang_van_d: Vec<Field2> = Vec::with_capacity(nz);
    let mut life_n: Vec<Field2> = Vec::with_capacity(nz);

    for l in 0..nz {
        let st = &stress.stress[l];
        // 열(물질점 (y=j))별 (D,N) 산출.
        // 병렬/직렬 경로는 **동일 물리**: 열 j 는 상호 독립이고 rayon `collect::<Vec<_>>()` 는
        // 순서를 보존하므로 `cols` 가 동일하다. `--no-default-features` 로 오라클 green 유지가
        // 이 등가성의 기계 증명(시각화 HTML 계획 §3.2 기준 ④).
        #[cfg(feature = "parallel")]
        let col_iter = (0..ny).into_par_iter();
        #[cfg(not(feature = "parallel"))]
        let col_iter = 0..ny;

        let cols: Vec<(f64, f64)> = col_iter
            .map(|j| {
                let hist: Vec<[f64; 6]> = (0..nx)
                    .map(|i| {
                        [
                            st.sxx.at(i, j),
                            st.syy.at(i, j),
                            st.szz.at(i, j),
                            st.sxy.at(i, j),
                            st.syz.at(i, j),
                            st.sxz.at(i, j),
                        ]
                    })
                    .collect();
                let pr = dang_van_point(&hist);
                (
                    damage_from_point(&pr, params),
                    life_from_point(&pr, params),
                )
            })
            .collect();

        let mut df = Field2::zeros(nx, ny);
        let mut nf = Field2::zeros(nx, ny);
        for j in 0..ny {
            let (d, n) = cols[j];
            for i in 0..nx {
                df.set(i, j, d);
                nf.set(i, j, n);
            }
        }
        dang_van_d.push(df);
        life_n.push(nf);
    }

    FatigueResult {
        z: stress.z.clone(),
        dang_van_d,
        life_n,
    }
}

// ═════════════════════════════════════════════════════════════════════════
//  오라클 / 테스트
// ═════════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;
    use crate::m3_stress::solve_stress_at_depths;
    use crate::types::Grid;
    use approx::assert_relative_eq;
    use std::f64::consts::PI;

    // ── RP-M4-Wöhler(4, 정량; 0% 허용): 상수 확정 ──
    //
    // 근거: G-M4-3 SO Q4. A=−43.0 MPa, B=1220 MPa, N_ref=1e6. a_DV=3(1/√3−1/2).
    #[test]
    fn rp_m4_wohler_constants() {
        assert_relative_eq!(WOHLER_A_PA, -43.0e6, max_relative = 0.0);
        assert_relative_eq!(WOHLER_B_PA, 1220.0e6, max_relative = 0.0);
        assert_relative_eq!(N_REF, 1.0e6, max_relative = 0.0);
        // a_DV = 3(1/√3 − 1/2) (식7, von Mises).
        let expect = 3.0 * (1.0 / 3.0_f64.sqrt() - 0.5);
        assert_relative_eq!(ALPHA_DV, expect, max_relative = 1e-12);
        // σ_f(1e6) = −43e6·ln(1e6)+1220e6 ≈ 625.93 MPa; τ_f ≈ 361.38 MPa (손계산).
        let sf = sigma_f(N_REF, WOHLER_A_PA, WOHLER_B_PA);
        assert_relative_eq!(sf, 625.933e6, max_relative = 1e-4);
        assert_relative_eq!(tau_f(N_REF, WOHLER_A_PA, WOHLER_B_PA), 361.38e6, max_relative = 1e-4);
    }

    // ── MCE 기하 단위검증: 알려진 점집합의 최소외접구 중심 ──
    #[test]
    fn meb_center_known_geometry() {
        // 대칭 대척점 두 개 → 중심 = 원점.
        let pts = [[3.0, 0.0, 0.0, 0.0, 0.0, 0.0], [-3.0, 0.0, 0.0, 0.0, 0.0, 0.0]];
        let c = meb_center6(&pts);
        for v in c {
            assert!(v.abs() < 1e-6, "antipodal center not origin: {v}");
        }
        // 정삼각형(2D 평면) → 외심 = centroid(대칭).
        let r = 5.0;
        let tri = [
            [r, 0.0, 0.0, 0.0, 0.0, 0.0],
            [r * (2.0 * PI / 3.0).cos(), r * (2.0 * PI / 3.0).sin(), 0.0, 0.0, 0.0, 0.0],
            [r * (4.0 * PI / 3.0).cos(), r * (4.0 * PI / 3.0).sin(), 0.0, 0.0, 0.0, 0.0],
        ];
        let c = meb_center6(&tri);
        assert!(c[0].abs() < 1e-3 && c[1].abs() < 1e-3, "triangle circumcenter off: {c:?}");
        // 지배쌍(MEB≠centroid): 극점 [6,−3,−3] + 원점 3개 → MEB중심=midpoint=[3,−1.5,−1.5]
        // (centroid=[1.5,−0.75,−0.75] 와 상이). centroid 축약이면 FAIL(검출력).
        let dom = [
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [6.0, -3.0, -3.0, 0.0, 0.0, 0.0],
        ];
        let c = meb_center6(&dom);
        assert_relative_eq!(c[0], 3.0, max_relative = 1e-3); // MEB midpoint(3), NOT centroid(1.5)
        assert_relative_eq!(c[1], -1.5, max_relative = 1e-3);
        assert_relative_eq!(c[2], -1.5, max_relative = 1e-3);
    }

    // ── VC-M4-MCE(비대칭 MEB≠centroid): MCE 축 검출력 확보(대칭 사각지대 폐색, human-gate) ──
    //
    // Dang Van 핵심은 비비례 하중서 최소외접구 중심 z*≠centroid. `dang_van_point` 전 경로를
    // 손유도 지배쌍으로 통과시켜 z*·τ̂ 을 정확값과 대조 — **centroid 스텁이면 값이 달라 FAIL**.
    // (tautology/fabrication 크리틱 human-gate: 기존 오라클 전수 대칭이라 centroid=MEB 사각지대.)
    #[test]
    fn vc_m4_mce_nonproportional() {
        let u = 1.0e8;
        // (A) 법선 지배쌍: 편차경로 3점 원점 + 1점 diag(6,−3,−3)·u.
        //   MEB중심 z*=[3,−1.5,−1.5]·u (centroid [1.5,−0.75,−0.75]·u 아님).
        //   ŝ 극점 ±diag(3,−1.5,−1.5)u → τ̂=½(3−(−1.5))u=2.25u (centroid면 3.375u).
        let hist_a = [
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [6.0 * u, -3.0 * u, -3.0 * u, 0.0, 0.0, 0.0],
        ];
        let pr = dang_van_point(&hist_a);
        assert_relative_eq!(pr.z_star[0], 3.0 * u, max_relative = 1e-3); // ≠ centroid 1.5u
        assert_relative_eq!(pr.z_star[1], -1.5 * u, max_relative = 1e-3);
        assert_relative_eq!(pr.z_star[2], -1.5 * u, max_relative = 1e-3);
        let tmax_a = pr.tau_hat.iter().cloned().fold(0.0, f64::max);
        assert_relative_eq!(tmax_a, 2.25 * u, max_relative = 1e-3); // ≠ centroid 3.375u

        // (B) 전단 지배쌍(√2 가중 검증): 3점 0 + 1점 σxy=q. z*_xy=q/2 (centroid q/4 아님),
        //   τ̂_max=q/2 (centroid 3q/4 아님). √2 노름보존이 틀리면 z*_xy·τ̂ 어긋남.
        let q = 2.0e8;
        let hist_b = [
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, q, 0.0, 0.0],
        ];
        let pr = dang_van_point(&hist_b);
        assert_relative_eq!(pr.z_star[3], 0.5 * q, max_relative = 1e-3); // ≠ centroid 0.25q
        let tmax_b = pr.tau_hat.iter().cloned().fold(0.0, f64::max);
        assert_relative_eq!(tmax_b, 0.5 * q, max_relative = 1e-3); // ≠ centroid 0.75q
    }

    // ── VC-M4-Milano(1, 핵심 오라클): MCE+Tresca 기계 vs Milano 닫힌해 A.8/A.10 (≤1%) ──
    //
    // 하중(A.5): σ_x=σ_m+σ_a cos ωt (σ_m=−σ_a), τ_xy=τ_a sin ωt, 그 외 0.
    // 닫힌해: τ_DV=½√(σ_a²cos²+4τ_a²sin²) (A.8), σ_H=(σ_a/3)(cos−1) (A.10).
    // 닫힌해는 구현에 미사용 → MCE(식5)+Tresca(식2) 기계를 독립검증.
    #[test]
    fn vc_m4_milano_closed_form() {
        let sa = 200.0e6;
        let ta = 150.0e6;
        let sm = -sa;
        let m = 720usize;
        let hist: Vec<[f64; 6]> = (0..m)
            .map(|k| {
                let wt = 2.0 * PI * k as f64 / m as f64;
                let sx = sm + sa * wt.cos();
                let txy = ta * wt.sin();
                [sx, 0.0, 0.0, txy, 0.0, 0.0]
            })
            .collect();

        let pr = dang_van_point(&hist);

        // 닫힌해 피크(정규화 기준).
        let tau_peak = ta.max(0.5 * sa); // sin=1 → τ_a; cos=1 → σ_a/2
        for k in 0..m {
            let wt = 2.0 * PI * k as f64 / m as f64;
            // Milano A.8/A.10 — reference(leaf) 소유. 구현(MCE+Tresca)과 독립.
            let tau_cf = crate::reference::milano2006_tau_dv(sa, ta, wt);
            let p_cf = crate::reference::milano2006_sigma_h(sa, wt);
            // τ̂: MCE 정확도 지배 → 피크 대비 ≤1%.
            assert!(
                (pr.tau_hat[k] - tau_cf).abs() <= 0.01 * tau_peak,
                "Milano τ̂ mismatch @k={k}: impl={} closed={} (>1%)",
                pr.tau_hat[k],
                tau_cf
            );
            // p̂ = tr/3 (해석적으로 정확).
            assert!(
                (pr.p_hat[k] - p_cf).abs() <= 1e-3 * sa,
                "Milano σ_H mismatch @k={k}: impl={} closed={}",
                pr.p_hat[k],
                p_cf
            );
        }
    }

    // ── VC-M4-MesoA(1): 단축 맥동하중 MCE 중심 vs Milano ρ*(A.6) + Tresca ──
    //
    // σ_x=σ_m+σ_a cos ωt (σ_m=−σ_a), τ=0. z*=−ρ*=diag(2/3 σ_m, −σ_m/3, −σ_m/3) (A.6).
    // MCE 중심(식5)이 이 잔류편차 반대텐서를 재현하는지 = 중심 자체의 독립검증.
    #[test]
    fn vc_m4_meso_a_center() {
        let sa = 200.0e6;
        let sm = -sa;
        let m = 720usize;
        let hist: Vec<[f64; 6]> = (0..m)
            .map(|k| {
                let wt = 2.0 * PI * k as f64 / m as f64;
                [sm + sa * wt.cos(), 0.0, 0.0, 0.0, 0.0, 0.0]
            })
            .collect();
        let pr = dang_van_point(&hist);
        // z* = diag(2/3 σ_m, −σ_m/3, −σ_m/3), 전단 0 (A.6 부호).
        let z = pr.z_star;
        assert!((z[0] - 2.0 / 3.0 * sm).abs() <= 0.01 * sa, "z*_xx off: {}", z[0]);
        assert!((z[1] + sm / 3.0).abs() <= 0.01 * sa, "z*_yy off: {}", z[1]);
        assert!((z[2] + sm / 3.0).abs() <= 0.01 * sa, "z*_zz off: {}", z[2]);
        for &s in &z[3..6] {
            assert!(s.abs() <= 1e-3 * sa, "z* shear nonzero: {s}");
        }
        // τ̂ 최대 = σ_a/2 (ŝ_xx=2/3 σ_a cos → Tresca=½|σ_a cos|).
        let tmax = pr.tau_hat.iter().cloned().fold(0.0, f64::max);
        assert_relative_eq!(tmax, 0.5 * sa, max_relative = 1e-2);
    }

    // ── VC-M4-DesiTab(4, 문헌): 단축 R-비 정규화 피로한계 — 단일선 Dang Van 예측 ──
    //
    // 손유도(독립): σ_a(R) 는 max_t{τ̂+a_DV·p̂}=τ_W 를 만족 → 정규화비
    //   σ_a(R)/σ_a(−1) = [½+a/3] / [½ + 2a/(3(1−R))]   (a=a_DV).
    // → R=−2:1.047, −1:1.000, 0.1:0.859, 0.3:0.801 (손계산).
    // **불일치 flag(Desimone Table)**: 실험값 = {−2:1, −1:1, 0.1:0.76, 0.3:0.68}. 원 단일선
    // Dang Van 은 양(+)R 한도를 과대(비보수)예측(0.859 vs 0.76)하고 R=−2 서 증가(1.047 vs 1.0)를
    // 예측 → Desimone 이 이중기울기 궤적을 제안한 바로 그 사유. 본 테스트는 **구현이 단일선
    // 닫힌해를 재현**함을 확인(실험 재현이 아님; 문헌 불일치는 예상된 정상거동).
    #[test]
    fn vc_m4_desitab_rratio() {
        let a = ALPHA_DV;
        let params = FatigueParams::default();
        // 단위진폭 σ_a=1 로 Dang Van 등가 DV=max_t{τ̂+a·p̂} 산출 → σ_a(R) ∝ 1/DV.
        let dv_unit = |r: f64| -> f64 {
            let sa = 1.0;
            let sm = sa * (1.0 + r) / (1.0 - r); // σ_m/σ_a = (1+R)/(1−R)
            let m = 720usize;
            let hist: Vec<[f64; 6]> = (0..m)
                .map(|k| {
                    let wt = 2.0 * PI * k as f64 / m as f64;
                    [sm + sa * wt.cos(), 0.0, 0.0, 0.0, 0.0, 0.0]
                })
                .collect();
            let pr = dang_van_point(&hist);
            pr.tau_hat
                .iter()
                .zip(pr.p_hat.iter())
                .map(|(t, p)| t + params.alpha_dv * p)
                .fold(f64::NEG_INFINITY, f64::max)
        };
        let ref_dv = dv_unit(-1.0);
        let closed = |r: f64| -> f64 {
            (0.5 + a / 3.0) / (0.5 + 2.0 * a / (3.0 * (1.0 - r)))
        };
        for &r in &[-2.0, -1.0, 0.1, 0.3] {
            let ratio_impl = ref_dv / dv_unit(r); // σ_a(R)/σ_a(−1)
            let ratio_closed = closed(r);
            assert!(
                (ratio_impl - ratio_closed).abs() <= 2e-2 * ratio_closed,
                "DesiTab R={r}: impl={ratio_impl} closed={ratio_closed}"
            );
        }
        // 단일선 예측이 실험표를 재현하지 못함(불일치 flag) — 양(+)R 과대예측 확인.
        let exp_table = [(0.1_f64, 0.76_f64), (0.3, 0.68)];
        for &(r, exp) in &exp_table {
            let ratio_impl = ref_dv / dv_unit(r);
            assert!(
                ratio_impl > exp,
                "single-line DV should over-predict +R limit (R={r}): impl={ratio_impl} exp={exp}"
            );
        }
    }

    // ── VC-M4-Desi(3, 문헌·정성): M3→M4 통합, 구름/미끄럼 접촉 RCF 거동 ──
    //
    // Desimone Fig.3a: 구반경1mm·μ=0.1, p_o/k=3.5 서 **원 단일선 Dang Van 이 파손예측 안함**.
    // **가정/불일치 flag**: 정확한 p_o/k=3.5 사상은 순환항복전단 k(shakedown)를 요하며 피로한계
    // 모델 범위 밖 → 본 테스트는 **정성/단조성**만 검증: (a) D 유한·양, (b) 하중↑→D↑·N↓,
    // (c) 마찰↑→D↑, (d) 중간하중서 단일선 D<1(무파손) = Desimone "원 궤적 과소예측"과 정성일치.
    #[test]
    fn vc_m4_desi_rcf_qualitative() {
        let nu = 0.3;
        let b = 1.0e-4; // Hertz 반접촉폭
        let nx = 256usize;
        let ny = 4usize;
        let lx = 16.0 * b;
        let ly = 4.0 * b;
        let grid = Grid::new(nx, ny, lx, ly);
        let dx = lx / nx as f64;
        let x0 = 0.5 * lx;

        // Hertz 선접촉 압력 p(x)=p_h√(1−((x−x0)/b)²), 트랙션 q=μ·p (Coulomb).
        let build = |p_h: f64, mu: f64| -> (Field2, Field2) {
            let mut p = Field2::zeros(nx, ny);
            let mut q = Field2::zeros(nx, ny);
            for j in 0..ny {
                for i in 0..nx {
                    let xr = (i as f64 * dx - x0) / b;
                    let v = if xr.abs() < 1.0 {
                        p_h * (1.0 - xr * xr).sqrt()
                    } else {
                        0.0
                    };
                    p.set(i, j, v);
                    q.set(i, j, mu * v);
                }
            }
            (p, q)
        };

        let z_depths: Vec<f64> =
            (0..15).map(|l| b * (0.02 + 1.3 * l as f64 / 14.0)).collect();
        let params = FatigueParams::default();

        // 중간하중 p_h=0.8 GPa, μ=0.1.
        let (p1, q1) = build(0.8e9, 0.1);
        let s1 = solve_stress_at_depths(&grid, &p1, &q1, nu, &z_depths);
        let f1 = solve_fatigue(&s1, &params);

        // (a) 전 층 D·N 유한·양수.
        let mut d1_max = 0.0f64;
        for l in 0..z_depths.len() {
            for v in &f1.dang_van_d[l].data {
                assert!(v.is_finite() && *v >= 0.0, "D not finite/pos: {v}");
                if *v > d1_max {
                    d1_max = *v;
                }
            }
            for v in &f1.life_n[l].data {
                assert!(v.is_finite() && *v > 0.0, "N not finite/pos: {v}");
            }
        }
        assert!(d1_max > 0.0, "no damage at all");
        // (d) 중간하중서 원 단일선 D<1 (무파손) — Desimone Fig.3a 정성일치.
        assert!(d1_max < 1.0, "single-line DV should not predict failure at moderate load: {d1_max}");

        // (b) 하중 2배 → D↑.
        let (p2, q2) = build(1.6e9, 0.1);
        let s2 = solve_stress_at_depths(&grid, &p2, &q2, nu, &z_depths);
        let f2 = solve_fatigue(&s2, &params);
        let d2_max = f2
            .dang_van_d
            .iter()
            .flat_map(|f| f.data.iter().cloned())
            .fold(0.0, f64::max);
        assert!(d2_max > d1_max, "D should increase with load: {d1_max} -> {d2_max}");

        // (c) 마찰 0.1→0.3 → D↑ (표면 트랙션 손상 가중).
        let (p3, q3) = build(0.8e9, 0.3);
        let s3 = solve_stress_at_depths(&grid, &p3, &q3, nu, &z_depths);
        let f3 = solve_fatigue(&s3, &params);
        let d3_max = f3
            .dang_van_d
            .iter()
            .flat_map(|f| f.data.iter().cloned())
            .fold(0.0, f64::max);
        assert!(d3_max > d1_max, "friction should raise D: {d1_max} -> {d3_max}");

        // 열(x축) broadcast 확인: 한 열 내 x-값 동일.
        let df = &f1.dang_van_d[7];
        for j in 0..ny {
            let v0 = df.at(0, j);
            for i in 1..nx {
                assert_relative_eq!(df.at(i, j), v0, max_relative = 1e-12);
            }
        }
    }

    // ── D=1 등고선 일치: DV=τ_f(N_ref)로 스케일한 이력 → D≈1 ∧ N≈N_ref ──
    //
    // **역할 한정(정직)**: 이는 damage(비율형 식1)↔life(가산형 식11)의 **자기정합 왕복**만
    // 검증한다(구현 dv_unit 로 역산→같은 경로 재평가). 선형성상 MCE/Tresca 코어 버그엔 둔감
    // (그건 `vc_m4_mce_nonproportional`·`vc_m4_milano_closed_form` 가 담당). 실효 검증범위 =
    // life_from_point 의 Wöhler 역산 대수 정합뿐. (tautology 크리틱 지적 반영: 보조 오라클.)
    #[test]
    fn damage_life_consistency() {
        let params = FatigueParams::default();
        // 단축 반전(R=−1) 이력. DV_unit = max_t{τ̂+a·p̂} (σ_a=1).
        let sa0 = 1.0;
        let m = 720usize;
        let mk = |sa: f64| -> Vec<[f64; 6]> {
            (0..m)
                .map(|k| {
                    let wt = 2.0 * PI * k as f64 / m as f64;
                    [sa * wt.cos(), 0.0, 0.0, 0.0, 0.0, 0.0]
                })
                .collect()
        };
        let pr0 = dang_van_point(&mk(sa0));
        let dv_unit = pr0
            .tau_hat
            .iter()
            .zip(pr0.p_hat.iter())
            .map(|(t, p)| t + params.alpha_dv * p)
            .fold(f64::NEG_INFINITY, f64::max);
        // τ_f(N_ref) 를 맞추는 σ_a → D=1, N=N_ref.
        let tf = tau_f(params.n_ref, params.wohler_a, params.wohler_b);
        let sa_lim = sa0 * tf / dv_unit;
        let pr = dang_van_point(&mk(sa_lim));
        let d = damage_from_point(&pr, &params);
        let n = life_from_point(&pr, &params);
        assert_relative_eq!(d, 1.0, max_relative = 1e-6);
        assert_relative_eq!(n, params.n_ref, max_relative = 1e-6);
    }

    // ── 퇴화 방어: 빈 응력장 → 빈/0 결과 ──
    #[test]
    fn degenerate_empty() {
        let empty = StressResult {
            z: vec![],
            stress: vec![],
            von_mises: vec![],
        };
        let r = solve_fatigue(&empty, &FatigueParams::default());
        assert!(r.z.is_empty() && r.dang_van_d.is_empty() && r.life_n.is_empty());
    }
}
