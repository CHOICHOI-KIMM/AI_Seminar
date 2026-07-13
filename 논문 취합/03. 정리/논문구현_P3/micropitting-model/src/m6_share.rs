//! # M6 — 부분윤활 하중분담(load sharing) 모듈 [논문 flow-balance 재구현]
//!
//! 건식(M1 [`DryResult`])·완전윤활(M2 [`LubResult`]) 국부해를 결합하여 부분윤활
//! (partial/mixed) 하중분담을 산정한다. 아스페리티(경계윤활)와 유막이 총하중 `W`를
//! 나누어 지지하며, 경계윤활 분율이 `phi_bl` 이다.
//!
//! ## 근거(원 논문 SKF 2011 "Combined Model for Partial Lubrication", L247–263)
//! 원 논문·(21)Morales-Espejel 2010·(23)Johnson-Greenwood-Poon 1972 의 **flow-balance
//! 반복**을 그대로 구현한다. 이전 구현의 폐합식(`h_sep=σ(1−φ)^−k_film`, `cfilm=c_film0·Σr`,
//! 자유계수 `k_film=0.5`·`c_film0=0.3`)은 **논문에 존재하지 않는 자의적 휴리스틱**이었고
//! 전면 폐기하였다(결함 M6-1).
//!
//! ### 절차 ①~⑤ (원 논문 L247; P2-3 §3.6; [23] App. II / [21] §4.2)
//! ① 매끈 중앙유막 `h̄` 를 EHL 공식(Dowson–Toyoda, 열/기아보정 제외)으로 산정.
//!    본 결합기는 M2 가 부여한 `h̄ = mean(h_lub)`(= 입력 `h_bar`)를 재사용한다.
//! ② 접촉 평균압을 선택, 초기값 = 최대 Hertz압 `p̄ = op.p_h`.
//! ③ 아스페리티가 지지하는 하중분율 `phi_bl` 가정 → 나머지는 유막이 지지.
//! ④ dry(M1) / lub(M2) **실입력** 국부압력·간극으로 dry/lub 영역 재식별,
//!    flow balance(변형 간극 상하 이동)를 유지하며 수렴까지 반복.
//! ⑤ 윤활유 압축성으로 인해 유막 지지분율이 변하면 초기 `h̄` 를 압축성 보정계수
//!    `c_ρ` 로 조정(식 [`c_rho`]).
//!
//! ### 전이 간극/압력장 (원 논문 식 [252]·[258]; 참고문헌 정리 "하중분담" 식)
//! 장파장 특징은 진폭이 커 접촉 가능성이 높다는 근거로, dry/lub **간극**을 주파수영역
//! 에서 비교해 절대최대를 취한다(간극–간극 **동종** 결합; 유막–간극 혼합 금지, M6-4).
//! ```text
//!   h_tran = IFFT{ max(|h̃_dry|, |h̃_lub|) }                     (식 [252])
//!   p_tran = IFFT{ w^{-1} · FFT(r − h_tran) },  r − h_tran = u   (식 [258])
//! ```
//! `w = W(k) = 2/(E_red·k)` (M1 식[1] 영향함수), 역연산 `w^{-1} = E_red·k/2`, DC(k=0)=0.
//! 미변형 거칠기 `r` 은 M6 입력으로 직접 주어지지 않으므로 **dry 해로부터 일관 복원**한다:
//! dry 문제에서 `p̂_dry = w^{-1}(r̂ − ĥ_dry)` 이므로
//! `p̂_tran = w^{-1}(r̂ − ĥ_tran) = p̂_dry + w^{-1}(ĥ_dry − ĥ_tran)` (r 소거, 아래 식).
//! 소성 발생 시 `p_tran ≤ p_lim`, cavitation(음압) 시 `p_tran = 0`(원 논문 L261).
//!
//! ## 잔여 가정(가정+민감도; 침묵 선택 금지)
//! * **RQ-M6-cρ (G-M6-2)**: `c_ρ` 압력인자로 `phi_bl`(dry분율)을 채택 — 두 독립출처
//!   ([21] L411 · 원 논문 nomenclature) 모두 `phi_bl` 로 일치(원문 충실). 물리적으로는
//!   유막부담분율 `(1−phi_bl)` 예상 → **물리해석 미검증**, `(1−phi_bl)` 대안 민감도 대상.
//! * **RQ-M6-tol (G-M6-3)**: flow-balance 수렴 tol·해법(이분법)은 논문 미제공 →
//!   가정([21] §4.2-7 `ε∈[1e-4,1e-3]`; [23] App. II). 이분법은 (23) flow-balance 근을
//!   단조성 기반으로 확정하는 강건형(under-relaxation 상위호환).
//! * **RQ-M6-hbar (G-M6-1)**: 중앙유막 `h̄` 는 Dowson–Toyoda(=`mean(h_lub)`) 재사용.
//!   Hamrock–Dowson 계수·~20% 과소평가 민감도는 M2/입력 `h_bar` 단계 소관.
//!
//! ## 단위 (SI)
//! 길이/간극/유막 [m], 압력 [Pa], 하중 [N], 면적요소 `dA = dx·dy` [m²].

use crate::types::{DryResult, Field2, Grid, LubResult, OperatingConditions, PartialLubResult, EPS};
use crate::util::fft::{fft2_forward, fft2_inverse};
use rustfft::num_complex::Complex;
use std::f64::consts::PI;

/// 윤활유 압축성 보정계수 `c_ρ` [-] (원 논문 nomenclature; [21] step3).
///
/// ```text
///   c_ρ = (0.59 + 1.34·φ_bl·p̄) / (0.59 + φ_bl·p̄)      (p̄ : GPa 관례)
/// ```
/// Dowson–Higginson 밀도-압력 법칙의 nomenclature 형태(대압극한 계수 2.3/1.7≈1.353→1.34).
/// `φ_bl=0`(무접촉) 또는 `p̄=0` 이면 `c_ρ=1`; `φ_bl·p̄` 증가 시 단조증가, 상한 1.34.
///
/// 인자(SI): `phi_bl`[-], `p_bar_pa`[Pa](내부에서 GPa 로 무차원화).
///
/// 근거: P2-1 L148·P2-3 L203·P2-2 M6-2. 압력인자 `phi_bl` 채택은 두 독립출처 일치
/// (원문 충실); 물리해석 미검증(`(1−phi_bl)` 대안 민감도) — 모듈 doc RQ-M6-cρ.
pub fn c_rho(phi_bl: f64, p_bar_pa: f64) -> f64 {
    let x = phi_bl.max(0.0) * (p_bar_pa.max(0.0) / 1.0e9); // φ_bl·p̄ [GPa 관례]
    (0.59 + 1.34 * x) / (0.59 + x)
}

/// 하중분담 flow-balance 반복 정책/파라미터.
#[derive(Debug, Clone, Copy)]
pub struct SharePolicy {
    /// 총 접촉하중 W [N] (하중분담 대상).
    pub w_total: f64,
    /// 최대 반복 횟수(이분법 bracket).
    pub max_iter: usize,
    /// 수렴 판정 |Δphi_bl| 임계값 (RQ-M6-tol; [21] ε∈[1e-4,1e-3] 범위 내 강화 가능).
    pub tol: f64,
    /// 환산탄성계수 E_red [Pa] — p_tran 탄성복원 `w^{-1}=E_red·k/2` (식[258]).
    pub e_red: f64,
    /// 소성 압력 한계 p_lim [Pa] (>0 이면 `p_tran≤p_lim` 절단; ≤0 이면 소성절단 비활성).
    pub p_lim: f64,
}

impl Default for SharePolicy {
    fn default() -> Self {
        SharePolicy {
            w_total: 0.0,
            max_iter: 200,
            tol: 1e-8,
            e_red: crate::types::E_RED_STEEL_PA,
            p_lim: 0.0,
        }
    }
}

/// flow-balance 반복 추적(진단)용.
#[derive(Debug, Clone)]
pub struct ShareTrace {
    /// 반복별 phi_bl 이력(초기 bracket 중점 포함).
    pub phi_history: Vec<f64>,
    /// 수렴 여부.
    pub converged: bool,
    /// 실제 반복 횟수.
    pub iters: usize,
    /// 최종 압축성보정 평균 분리간극 `h_sep = c_ρ·h̄` [m].
    pub h_sep: f64,
    /// 최종 아스페리티(dry) 접촉점 수.
    pub contact_count: usize,
    /// 하중 보존 잔차 |∫p_tran dA − W| / W [-] (**회귀 가드**, 물리검증 아님 — M6-2).
    pub load_residual: f64,
    /// flow-balance 잔차 |mean(h_tran) − c_ρ·h̄| / (c_ρ·h̄) [-] (구성상 ≈0; 절차⑤ 항등).
    pub flow_balance_residual: f64,
    /// 경계 축퇴 플래그(M6-5): 접촉점이 있으나 아스페리티 하중 ≤EPS → phi_bl 유효 0 처리.
    pub asperity_degenerate: bool,
}

/// DFT 주파수 빈 → 부호있는 공간주파수 f [1/m] (표준 FFT 정렬; M1 `freq` 와 동일).
#[inline]
fn freq(idx: usize, n: usize, length: f64) -> f64 {
    let m = if idx <= n / 2 {
        idx as f64
    } else {
        idx as f64 - n as f64
    };
    if length > 0.0 {
        m / length
    } else {
        0.0
    }
}

/// 각 주파수 bin 에서 진폭이 큰 스펙트럼을 취하고 DC(=w(1,1))를 0 으로 둔 결합 스펙트럼.
///
/// `C[k] = (|H_dry[k]| ≥ |H_lub[k]|) ? H_dry[k] : H_lub[k]`, `C[0]=0` (식 [252]).
/// **간극–간극 동종 결합**: `h_dry`·`h_lub` 는 모두 간극(clearance)장이다(M6-4 준수).
fn combined_spectrum(h_dry: &Field2, h_lub: &Field2) -> Vec<Complex<f64>> {
    let nx = h_dry.nx;
    let ny = h_dry.ny;
    let n = nx * ny;
    let mut hd: Vec<Complex<f64>> = h_dry.data.iter().map(|&v| Complex::new(v, 0.0)).collect();
    let mut hl: Vec<Complex<f64>> = h_lub.data.iter().map(|&v| Complex::new(v, 0.0)).collect();
    fft2_forward(&mut hd, nx, ny);
    fft2_forward(&mut hl, nx, ny);
    let mut c = vec![Complex::new(0.0, 0.0); n];
    for k in 0..n {
        c[k] = if hd[k].norm() >= hl[k].norm() { hd[k] } else { hl[k] };
    }
    // w(1,1)=0 : DC(평균) 성분 제거 → 변동분만. 평균 분리간극은 flow-balance c_ρ·h̄ 로 별도.
    if n > 0 {
        c[0] = Complex::new(0.0, 0.0);
    }
    c
}

/// 전이 간극 변동분 `fluct`(평균 0) = `Re(IFFT{combined_spectrum})` [m] (식 [252] AC 성분).
///
/// 최종 `h_tran = h_sep + fluct`, `h_sep = c_ρ·h̄`(flow-balance 평균 분리간극).
fn transition_fluct(h_dry: &Field2, h_lub: &Field2) -> Vec<f64> {
    let nx = h_dry.nx;
    let ny = h_dry.ny;
    let mut c = combined_spectrum(h_dry, h_lub);
    fft2_inverse(&mut c, nx, ny);
    c.iter().map(|z| z.re).collect()
}

/// 전이 압력장 `p_tran` [Pa] 탄성복원 (식 [258]) + 하중 재균형(CV-M6-Load A안).
///
/// `p_tran = p_dry + IFFT{ w^{-1}·FFT(h_dry − h_tran) }`, `w^{-1}(k)=E_red·k/2`, DC=0.
/// (dry 해 `p̂_dry=w^{-1}(r̂−ĥ_dry)` 로 미변형 거칠기 `r` 일관 소거 — 모듈 doc 참조.)
/// cavitation: 음압 → 0 (L261); 소성: `p_lim>0` 이면 `p_tran≤p_lim`.
///
/// **하중 재균형(A안)**: `w_total>0` 이면 cavitation 절단이 깬 하중보존을 복원하기 위해
/// 균일 오프셋 δ(강체접근량)를 이분법으로 조정해 `∫p_tran·dA = w_total` 을 강제한다
/// (`Σclip(raw+δ)` 는 δ 에 단조증가 → 유일근). M1 Polonsky–Keer 하중정규화와 동일 원리.
/// `w_total≤0` 이면 재균형 없이 순수 복원(식[258] 오라클용).
fn recover_p_tran(
    p_dry: &Field2,
    h_dry: &Field2,
    h_tran: &Field2,
    grid: &Grid,
    e_red: f64,
    p_lim: f64,
    w_total: f64,
    da: f64,
) -> Field2 {
    let nx = grid.nx;
    let ny = grid.ny;
    let n = nx * ny;
    if n == 0 {
        return Field2::zeros(nx, ny);
    }
    // Δh = h_dry − h_tran → 주파수영역에서 w^{-1} 곱 → 실공간 변형압 보정.
    let mut d: Vec<Complex<f64>> = (0..n)
        .map(|k| Complex::new(h_dry.data[k] - h_tran.data[k], 0.0))
        .collect();
    fft2_forward(&mut d, nx, ny);
    for j in 0..ny {
        let fy = freq(j, ny, grid.ly);
        for i in 0..nx {
            let fx = freq(i, nx, grid.lx);
            let k = 2.0 * PI * (fx * fx + fy * fy).sqrt(); // 각파수 [rad/m]
            let w_inv = if k > 0.0 { e_red * k / 2.0 } else { 0.0 }; // 역영향함수, DC=0
            d[i + j * nx] *= w_inv;
        }
    }
    fft2_inverse(&mut d, nx, ny);

    // 클리핑 전 원압(raw) = p_dry + 탄성복원 보정.
    let raw: Vec<f64> = (0..n).map(|k| p_dry.data[k] + d[k].re).collect();
    // 클리핑: cavitation(≥0) + 소성(p_lim>0 이면 ≤p_lim).
    let clip = |v: f64| -> f64 {
        let mut v = if v < 0.0 { 0.0 } else { v };
        if p_lim > 0.0 && v > p_lim {
            v = p_lim;
        }
        v
    };

    // 하중 재균형(A안): `Σclip(raw+δ)·da = w_total` 되는 균일 오프셋 δ 를 이분법으로.
    let offset = if w_total > EPS && da > 0.0 {
        let sum_at = |dlt: f64| -> f64 { raw.iter().map(|&r| clip(r + dlt)).sum::<f64>() * da };
        let max_raw = raw.iter().cloned().fold(f64::MIN, f64::max);
        let lo0 = -(max_raw.abs() + 1.0); // Σ(lo0)=0 < w_total
        let mut lo = lo0;
        let mut hi = 0.0_f64;
        // Σ(hi) < w_total 이면(소성절단 등) hi 를 위로 확장.
        let step = max_raw.abs().max(1.0);
        let mut guard = 0;
        while sum_at(hi) < w_total && guard < 200 {
            hi += step;
            guard += 1;
        }
        for _ in 0..100 {
            let mid = 0.5 * (lo + hi);
            if sum_at(mid) < w_total {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        0.5 * (lo + hi)
    } else {
        0.0
    };

    let mut p = Field2::zeros(nx, ny);
    for k in 0..n {
        p.data[k] = clip(raw[k] + offset);
    }
    p
}

/// 부분윤활 하중분담 결합(추적 포함) — 원 논문 flow-balance 반복.
///
/// 입력: dry(M1)·lub(M2) 국부해 실입력, 격자, 운전조건(`op.p_h`→c_ρ), 정책(W·E_red·p_lim).
/// 반환: `PartialLubResult { p_tran, h_tran, phi_bl }` 및 반복 추적 `ShareTrace`.
///
/// φ_bl 는 이분법(단조 flow-balance 근)으로 수렴: `phi ↑ → c_ρ ↑ → h_sep ↑ → 접촉↓ →
/// 아스페리티 하중분율↓`(음성 피드백 → 유일 고정점).
pub fn combine_share_traced(
    dry: &DryResult,
    lub: &LubResult,
    grid: &Grid,
    op: &OperatingConditions,
    policy: &SharePolicy,
) -> (PartialLubResult, ShareTrace) {
    let nx = grid.nx;
    let ny = grid.ny;
    let n = nx * ny;
    let da = grid.dx() * grid.dy();
    let w = policy.w_total;
    let e_red = policy.e_red;
    let p_lim = policy.p_lim;

    // ── 절차① 매끈 중앙유막 h̄ = mean(h_lub) (Dowson–Toyoda 재사용; = 입력 h_bar) ──
    let h_bar = if n > 0 {
        lub.h_lub.data.iter().sum::<f64>() / n as f64
    } else {
        0.0
    };
    // ── 절차② 접촉 평균압 p̄ = op.p_h (초기 최대 Hertz압) ──
    let p_bar = op.p_h;

    // ── 전이 간극 변동분(φ 무관): 주파수별 절대최대(식[252] AC). DC=0. ──
    let fluct = transition_fluct(&dry.h_dry, &lub.h_lub);

    // 주어진 평균 분리간극 h_sep 에서 아스페리티/유막 하중분율 g(h_sep).
    //   접촉집합 C = { i : h_sep + fluct_i < 0 } (전이 간극이 닫혀 dry 접촉).
    //   W_asp = Σ_C p_dry·dA (아스페리티 지지),  W_film = Σ_{¬C} max(p_lub,0)·dA (유막 지지).
    //   phi = W_asp / (W_asp + W_film).  ⇒ dry(M1)·lub(M2) 실압력을 모두 사용(결선 검증).
    let load_fraction = |h_sep: f64| -> (f64, f64, usize) {
        let mut w_asp = 0.0;
        let mut w_film = 0.0;
        let mut cc = 0usize;
        for k in 0..n {
            if h_sep + fluct[k] < 0.0 {
                w_asp += dry.p_dry.data[k] * da;
                cc += 1;
            } else {
                w_film += lub.p_lub.data[k].max(0.0) * da;
            }
        }
        (w_asp, w_film, cc)
    };
    // φ 로부터의 암시분율 g(φ): flow-balance 절차③~⑤.
    let phi_implicit = |phi: f64| -> f64 {
        let cr = c_rho(phi, p_bar);
        let h_sep = cr * h_bar;
        let (w_asp, w_film, _) = load_fraction(h_sep);
        let tot = w_asp + w_film;
        if tot > EPS {
            (w_asp / tot).clamp(0.0, 1.0)
        } else {
            0.0
        }
    };

    // ── 절차③④⑤ flow-balance 근 F(φ)=g(φ)−φ=0 (단조감소 → 이분법; RQ-M6-tol) ──
    // g 단조감소(φ↑→c_ρ↑→h_sep↑→W_asp↓→g↓) ⇒ F 단조감소 ⇒ 유일 근. 이분법 수렴 보장.
    let mut lo = 0.0_f64;
    let mut hi = 1.0_f64;
    let mut history: Vec<f64> = Vec::new();
    let f_lo = phi_implicit(lo) - lo; // ≥0
    let f_hi = phi_implicit(hi) - hi; // ≤0
    let mut phi;
    let mut converged = false;
    let mut iters = 0usize;
    if f_lo <= 0.0 {
        // g(0)=0 → 완전분리(무접촉) 고정점 φ=0.
        phi = 0.0;
        history.push(phi);
        converged = true;
    } else if f_hi >= 0.0 {
        // g(1)=1 → 완전건식 고정점 φ=1.
        phi = 1.0;
        history.push(phi);
        converged = true;
    } else {
        phi = 0.5 * (lo + hi);
        for _ in 0..policy.max_iter {
            iters += 1;
            phi = 0.5 * (lo + hi);
            history.push(phi);
            let f = phi_implicit(phi) - phi;
            if hi - lo < policy.tol {
                converged = true;
                break;
            }
            if f > 0.0 {
                lo = phi; // 근은 오른쪽
            } else {
                hi = phi; // 근은 왼쪽
            }
        }
    }

    // ── 최종량: h_sep, h_tran, p_tran ──
    let cr = c_rho(phi, p_bar);
    let h_sep = cr * h_bar;
    let h_tran_data: Vec<f64> = fluct.iter().map(|&f| h_sep + f).collect();
    let h_tran = Field2::from_vec(nx, ny, h_tran_data);
    let p_tran = recover_p_tran(&dry.p_dry, &dry.h_dry, &h_tran, grid, e_red, p_lim, w, da);

    // ── 진단(회귀 가드·플래그) ──
    let (w_asp_final, _w_film_final, contact_count) = load_fraction(h_sep);
    let asperity_degenerate = contact_count > 0 && w_asp_final <= EPS;
    let w_tran: f64 = p_tran.data.iter().map(|&p| p * da).sum();
    let load_residual = if w.abs() > EPS {
        (w_tran - w).abs() / w.abs()
    } else {
        w_tran.abs()
    };
    let h_tran_mean = if n > 0 {
        h_tran.data.iter().sum::<f64>() / n as f64
    } else {
        0.0
    };
    let flow_balance_residual = if h_sep.abs() > EPS {
        (h_tran_mean - h_sep).abs() / h_sep.abs()
    } else {
        h_tran_mean.abs()
    };

    let result = PartialLubResult {
        p_tran,
        h_tran,
        phi_bl: phi,
    };
    let trace = ShareTrace {
        phi_history: history,
        converged,
        iters,
        h_sep,
        contact_count,
        load_residual,
        flow_balance_residual,
        asperity_degenerate,
    };
    (result, trace)
}

/// 부분윤활 하중분담 결합 (표준 진입점).
///
/// [`combine_share_traced`] 의 결과만 반환.
pub fn combine_share(
    dry: &DryResult,
    lub: &LubResult,
    grid: &Grid,
    op: &OperatingConditions,
    policy: &SharePolicy,
) -> PartialLubResult {
    combine_share_traced(dry, lub, grid, op, policy).0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;
    use approx::assert_relative_eq;
    use std::f64::consts::PI;

    fn op_ref() -> OperatingConditions {
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

    /// 합성 패치: Hertz형 dry/lub 압력 + 서로 다른 스펙트럼의 거칠기 간극/유막.
    /// `h_bar` 인자로 유막-거칠기 비를 조절(완전분리/혼합/완전건식 극한 구성용).
    fn synthetic(h_bar: f64) -> (DryResult, LubResult, Grid, OperatingConditions, f64) {
        let nx = 32usize;
        let ny = 32usize;
        let lx = 200e-6;
        let ly = 200e-6;
        let grid = Grid::new(nx, ny, lx, ly);
        let dx = grid.dx();
        let dy = grid.dy();
        let b = 60e-6;
        let p_h = 1.5e9;
        let cx = lx / 2.0;
        let cy = ly / 2.0;

        let mut p_dry = Field2::zeros(nx, ny);
        let mut h_dry = Field2::zeros(nx, ny);
        let mut p_lub = Field2::zeros(nx, ny);
        let mut h_lub = Field2::zeros(nx, ny);

        let kx1 = 2.0 * PI * 4.0 / lx;
        let ky1 = 2.0 * PI * 3.0 / ly;
        let kx2 = 2.0 * PI * 6.0 / lx;
        let ky2 = 2.0 * PI * 5.0 / ly;

        for j in 0..ny {
            for i in 0..nx {
                let x = i as f64 * dx;
                let y = j as f64 * dy;
                let xr = x - cx;
                let yr = y - cy;
                let r2 = xr * xr + yr * yr;

                let ud = 1.0 - r2 / (b * b);
                let pd = if ud > 0.0 { p_h * ud.sqrt() } else { 0.0 };
                p_dry.set(i, j, pd);

                let bl = 1.15 * b;
                let ul = 1.0 - r2 / (bl * bl);
                let pl = if ul > 0.0 { 0.9 * p_h * ul.sqrt() } else { 0.0 };
                p_lub.set(i, j, pl);

                let rd = 0.60e-7 * (kx1 * x).cos() * (ky1 * y).cos();
                let rl = 0.90e-7 * (kx2 * x).sin() * (ky2 * y).cos();

                // dry 간극: 접촉영역 ~0, 외곽 증가 + 거칠기.
                let gap = if ud > 0.0 {
                    0.0
                } else {
                    0.5e-7 * (r2 / (b * b) - 1.0)
                };
                h_dry.set(i, j, gap + rd);

                // lub 유막두께: h_bar + 거칠기(간극 변동).
                h_lub.set(i, j, h_bar + rl);
            }
        }

        let da = dx * dy;
        let w: f64 = p_dry.data.iter().map(|&p| p * da).sum();
        (
            DryResult { p_dry, h_dry },
            LubResult { p_lub, h_lub },
            grid,
            op_ref(),
            w,
        )
    }

    fn policy_for(w: f64) -> SharePolicy {
        SharePolicy {
            w_total: w,
            e_red: E_RED_STEEL_PA,
            ..Default::default()
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  실패가능 물리 오라클 (M6-2) — 극한 거동 + 지배효과 방향
    // ═══════════════════════════════════════════════════════════════════════

    /// **완전분리 극한**: 유막 h̄ ≫ 거칠기 → 아스페리티 접촉 없음 → phi_bl → 0.
    /// (자유계수로 interior 를 강제하던 구 휴리스틱이면 실패.)
    #[test]
    fn complete_separation_limit_phi_to_zero() {
        // h_bar 를 거칠기 진폭(≈0.9e-7)의 수배로 → h_sep=c_ρ·h̄ 가 fluct 골보다 큼.
        let (dry, lub, grid, op, w) = synthetic(5.0e-7);
        let (res, trace) = combine_share_traced(&dry, &lub, &grid, &op, &policy_for(w));
        assert!(
            res.phi_bl < 1e-6,
            "complete separation must give phi_bl→0, got {}",
            res.phi_bl
        );
        assert_eq!(trace.contact_count, 0, "no asperity contact expected");
        assert!(trace.converged);
    }

    /// **완전건식 극한**: 유막 h̄→0 & 유막압→0 → 유막 지지분율→0 → phi_bl → 1.
    #[test]
    fn complete_dry_limit_phi_to_one() {
        // 유막 두께 0 근방 + 유막압 제거 → 전량 아스페리티 지지.
        let (dry, mut lub, grid, op, w) = synthetic(1.0e-10);
        for v in lub.p_lub.data.iter_mut() {
            *v = 0.0; // 유막 압력 붕괴(완전건식)
        }
        let (res, trace) = combine_share_traced(&dry, &lub, &grid, &op, &policy_for(w));
        assert!(
            res.phi_bl > 1.0 - 1e-6,
            "complete dry must give phi_bl→1, got {}",
            res.phi_bl
        );
        assert!(trace.contact_count > 0);
        assert!(trace.converged);
    }

    /// **지배효과(M6-3)**: 유막 두께 h̄ 증가 → 접촉↓ → phi_bl 단조감소.
    /// 하중분담의 1차 지배인자는 유막두께(M1·M2 실물리)임을 실입력으로 검증.
    #[test]
    fn thicker_film_lowers_phi_bl_dominant() {
        let mut prev = 2.0_f64;
        for &hb in &[2.0e-8_f64, 4.0e-8, 6.0e-8, 8.0e-8] {
            let (dry, lub, grid, op, w) = synthetic(hb);
            let phi = combine_share(&dry, &lub, &grid, &op, &policy_for(w)).phi_bl;
            assert!(
                phi <= prev + 1e-9,
                "phi_bl not decreasing with film thickness: hb={hb}, phi={phi}, prev={prev}"
            );
            prev = phi;
        }
    }

    /// **flow-balance 고정점**: 수렴 φ 가 g(φ)=φ (절차③~⑤ 자기일관) 를 만족.
    /// (검증대상이 아닌 근-정의로부터 잔차 확인 → tautology 아님.)
    #[test]
    fn flow_balance_fixed_point_consistency() {
        // 혼합 영역(interior).
        let (dry, lub, grid, op, w) = synthetic(2.5e-8);
        let (res, trace) = combine_share_traced(&dry, &lub, &grid, &op, &policy_for(w));
        assert!(trace.converged);
        assert!(
            res.phi_bl > 0.0 && res.phi_bl < 1.0,
            "expected interior mixed regime, phi_bl={}",
            res.phi_bl
        );
        assert!(trace.contact_count > 0 && trace.contact_count < grid.len());
        // g(φ*) 재평가 → φ* 와 일치(고정점). 이분법 bracket tol 규모.
        let cr = c_rho(res.phi_bl, op.p_h);
        let h_sep = cr * (lub.h_lub.data.iter().sum::<f64>() / grid.len() as f64);
        // 재구성 g(φ*): 접촉 하중분율.
        let da = grid.dx() * grid.dy();
        let (mut wa, mut wf) = (0.0, 0.0);
        let fluct = super::transition_fluct(&dry.h_dry, &lub.h_lub);
        for k in 0..grid.len() {
            if h_sep + fluct[k] < 0.0 {
                wa += dry.p_dry.data[k] * da;
            } else {
                wf += lub.p_lub.data[k].max(0.0) * da;
            }
        }
        let g = wa / (wa + wf);
        assert!(
            (g - res.phi_bl).abs() < 1e-3,
            "flow-balance not self-consistent: g(φ*)={g}, φ*={}",
            res.phi_bl
        );
        // 절차⑤ 항등: mean(h_tran)=c_ρ·h̄ (구성상).
        assert!(trace.flow_balance_residual < 1e-9);
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  회귀 가드 (물리검증 아님 — M6-2 재분류)
    // ═══════════════════════════════════════════════════════════════════════

    /// **회귀 가드**(물리검증 아님 — M6-2 재분류): p_tran 탄성복원(식[258])의 보정항이
    /// 순수 AC 이므로 `mean(p_tran)=mean(p_dry)=W/A` → 총하중 보존. 유막을 매끈(constant
    /// h_lub)하게 두면 freq-max 가 h_dry 를 선택 → 보정 DC 만 남아 소거 → cavitation 절단
    /// 없이 **엄밀 보존**. (구 `load_conservation` 은 s_dry/s_lub 를 재스케일해 φ·w 를 구성상
    /// 항상 만족시키던 tautology 였음 → 폐기.)
    #[test]
    fn load_conservation_regression_guard() {
        let nx = 32usize;
        let ny = 32usize;
        let lx = 200e-6;
        let ly = 200e-6;
        let grid = Grid::new(nx, ny, lx, ly);
        let dx = grid.dx();
        let dy = grid.dy();
        let da = dx * dy;
        let b = 60e-6;
        let p_h = 1.5e9;
        let (cx, cy) = (lx / 2.0, ly / 2.0);
        let kx = 2.0 * PI * 4.0 / lx;
        let ky = 2.0 * PI * 3.0 / ly;

        let mut p_dry = Field2::zeros(nx, ny);
        let mut h_dry = Field2::zeros(nx, ny);
        // 매끈 유막(변동 없음) → freq-max 는 항상 h_dry 선택.
        let h_lub = Field2::filled(nx, ny, 3.0e-8);
        let p_lub = Field2::filled(nx, ny, 0.0);
        for j in 0..ny {
            for i in 0..nx {
                let x = i as f64 * dx;
                let y = j as f64 * dy;
                let xr = x - cx;
                let yr = y - cy;
                let ud = 1.0 - (xr * xr + yr * yr) / (b * b);
                p_dry.set(i, j, if ud > 0.0 { p_h * ud.sqrt() } else { 0.0 });
                // 임의의 변동 간극(정합 불필요: h_lub 가 매끈이라 h_tran 변동=h_dry 변동 → 보정 DC 소거).
                h_dry.set(i, j, 5.0e-8 + 1.0e-8 * (kx * x).cos() * (ky * y).cos());
            }
        }
        let w: f64 = p_dry.data.iter().map(|&p| p * da).sum();
        let dry = DryResult { p_dry, h_dry };
        let lub = LubResult { p_lub, h_lub };
        let (res, trace) = combine_share_traced(&dry, &lub, &grid, &op_ref(), &policy_for(w));
        let total: f64 = res.p_tran.data.iter().map(|&p| p * da).sum();
        // 매끈 유막 → p_tran = p_dry → 엄밀 보존.
        assert!(
            trace.load_residual < 1e-9,
            "load residual {} !< 1e-9 (smooth-film exact conservation)",
            trace.load_residual
        );
        assert_relative_eq!(total, w, max_relative = 1e-9);
        assert!(w > 0.0);
    }

    /// **반환형/차원 계약** + 유한성.
    #[test]
    fn returns_partial_lub_contract() {
        let (dry, lub, grid, op, w) = synthetic(2.5e-8);
        let res = combine_share(&dry, &lub, &grid, &op, &policy_for(w));
        assert_eq!(res.p_tran.nx, grid.nx);
        assert_eq!(res.p_tran.ny, grid.ny);
        assert_eq!(res.h_tran.len(), grid.len());
        assert!(res.p_tran.data.iter().all(|v| v.is_finite()));
        assert!(res.h_tran.data.iter().all(|v| v.is_finite()));
        // cavitation: p_tran ≥ 0.
        assert!(res.p_tran.data.iter().all(|&v| v >= 0.0));
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  전이장 식 독립 오라클 (식 [252]·[258])
    // ═══════════════════════════════════════════════════════════════════════

    /// h_tran 스펙트럼: 각 bin 진폭 = max(|H_dry|,|H_lub|), DC=0 (식 [252]).
    #[test]
    fn h_tran_spectrum_max_and_dc_zero() {
        let nx = 8usize;
        let ny = 8usize;
        let mut hd = Field2::zeros(nx, ny);
        let mut hl = Field2::zeros(nx, ny);
        for j in 0..ny {
            for i in 0..nx {
                hd.set(i, j, 3.0 + (i as f64).sin() + 0.5 * (j as f64).cos());
                hl.set(i, j, 2.0 + (2.0 * i as f64).cos() + (0.7 * j as f64).sin());
            }
        }
        let mut fd: Vec<Complex<f64>> = hd.data.iter().map(|&v| Complex::new(v, 0.0)).collect();
        let mut fl: Vec<Complex<f64>> = hl.data.iter().map(|&v| Complex::new(v, 0.0)).collect();
        fft2_forward(&mut fd, nx, ny);
        fft2_forward(&mut fl, nx, ny);

        let c = combined_spectrum(&hd, &hl);
        assert_relative_eq!(c[0].norm(), 0.0, epsilon = 1e-12);
        for k in 1..nx * ny {
            let expected = fd[k].norm().max(fl[k].norm());
            assert_relative_eq!(c[k].norm(), expected, max_relative = 1e-9);
        }
        // fluct 평균 ≈ 0 (DC 제거 결과).
        let fluct = transition_fluct(&hd, &hl);
        let mean = fluct.iter().sum::<f64>() / fluct.len() as f64;
        assert_relative_eq!(mean, 0.0, epsilon = 1e-9);
    }

    /// **p_tran 독립 오라클 (식 [258])**: 단일 모드에서 역영향함수 `w^{-1}=E_red·k/2` 를
    /// 닫힌형으로 강제. 검증대상(recover_p_tran) 아닌 손유도값과 대조 (tautology 회피).
    ///
    /// 구성: 모드 kx 에서 h_lub 진폭 > h_dry 진폭 → h_tran 은 h_lub 모드 선택.
    ///   p_tran = p_dry + w^{-1}·(h_dry − h_tran)
    ///          = p_dc + [p1 + (E_red·kx/2)(hd − hl)]·cos(kx·x)   (전 격자 닫힌형)
    #[test]
    fn p_tran_recovery_closed_form_winv() {
        let nx = 64usize;
        let ny = 4usize;
        let lx = 1.0e-4;
        let ly = 1.0e-4;
        let grid = Grid::new(nx, ny, lx, ly);
        let m = 2.0;
        let kx = 2.0 * PI * m / lx; // ky=0
        let e_red = 1.0e11;

        let p_dc = 2.0e8;
        let p1 = 1.0e8;
        let hd = 1.0e-8; // dry 간극 모드 진폭
        let hl = 2.0e-8; // lub 간극 모드 진폭(> hd → freq-max 가 lub 선택)
        let h_sep = 5.0e-8; // 임의 평균 분리간극(DC → w^{-1} 소거)

        let mut p_dry = Field2::zeros(nx, ny);
        let mut h_dry = Field2::zeros(nx, ny);
        let mut h_lub = Field2::zeros(nx, ny);
        for j in 0..ny {
            for i in 0..nx {
                let x = i as f64 * (lx / nx as f64);
                let cph = (kx * x).cos();
                p_dry.set(i, j, p_dc + p1 * cph);
                h_dry.set(i, j, 3.0e-8 + hd * cph); // DC 3e-8 (임의, 소거됨)
                h_lub.set(i, j, h_sep + hl * cph);
            }
        }
        // 전이 간극: freq-max(여기선 lub 진폭 우세) → fluct = hl·cos.
        let fluct = transition_fluct(&h_dry, &h_lub);
        let h_tran_data: Vec<f64> = fluct.iter().map(|&f| h_sep + f).collect();
        let h_tran = Field2::from_vec(nx, ny, h_tran_data);

        // p_lim=0(비활성). p_dc 크게 잡아 cavitation 절단 없음.
        let p_tran = recover_p_tran(&p_dry, &h_dry, &h_tran, &grid, e_red, 0.0, 0.0, 0.0);

        // 손유도 닫힌형(recover 미사용): 계수 E_red·kx/2.
        let amp = p1 + (e_red * kx / 2.0) * (hd - hl);
        for j in 0..ny {
            for i in 0..nx {
                let x = i as f64 * (lx / nx as f64);
                let expect = p_dc + amp * (kx * x).cos();
                assert!(expect > 0.0, "test setup: keep p_tran positive (no cavitation)");
                assert!(
                    (p_tran.at(i, j) - expect).abs() <= expect.abs() * 1e-9,
                    "p_tran w^{{-1}} closed-form mismatch at ({i},{j}): got {}, expect {expect}",
                    p_tran.at(i, j)
                );
            }
        }
    }

    /// p_tran dry 극한: h_tran 변동분 = h_dry 변동분(freq-max 가 dry 선택)이면 p_tran=p_dry.
    #[test]
    fn p_tran_reduces_to_dry_when_gap_matches() {
        let nx = 16usize;
        let ny = 16usize;
        let grid = Grid::new(nx, ny, 1e-4, 1e-4);
        let e_red = E_RED_STEEL_PA;
        let mut p_dry = Field2::zeros(nx, ny);
        let mut h_dry = Field2::zeros(nx, ny);
        for j in 0..ny {
            for i in 0..nx {
                let x = i as f64;
                let y = j as f64;
                p_dry.set(i, j, 1.0e9 + 1.0e8 * (x * 0.3).sin() * (y * 0.2).cos());
                h_dry.set(i, j, 1.0e-7 + 2.0e-8 * (x * 0.3).sin() * (y * 0.2).cos());
            }
        }
        // h_tran 변동분 = h_dry 변동분(같은 평균이든 다르든 DC 는 소거).
        let hd_mean = h_dry.data.iter().sum::<f64>() / (nx * ny) as f64;
        let h_tran_data: Vec<f64> = h_dry.data.iter().map(|&v| v - hd_mean + 4.0e-8).collect();
        let h_tran = Field2::from_vec(nx, ny, h_tran_data);
        let p_tran = recover_p_tran(&p_dry, &h_dry, &h_tran, &grid, e_red, 0.0, 0.0, 0.0);
        for k in 0..nx * ny {
            assert!(
                (p_tran.data[k] - p_dry.data[k]).abs() <= p_dry.data[k].abs() * 1e-9 + 1.0,
                "p_tran should equal p_dry when gap fluctuation matches: {} vs {}",
                p_tran.data[k],
                p_dry.data[k]
            );
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  c_ρ 압축성 보정 (원 논문 nomenclature 식)
    // ═══════════════════════════════════════════════════════════════════════

    /// c_ρ: `c_ρ(0,·)=1`, `φ·p̄` 단조증가, 상한 1.34 (식 nomenclature).
    #[test]
    fn c_rho_properties() {
        assert_relative_eq!(c_rho(0.0, 1.5e9), 1.0, max_relative = 1e-15);
        assert_relative_eq!(c_rho(1.0, 0.0), 1.0, max_relative = 1e-15);
        let a = c_rho(1.0, 0.5e9);
        let b = c_rho(1.0, 1.5e9);
        let c = c_rho(1.0, 3.0e9);
        assert!(a > 1.0 && b > a && c > b, "c_rho not monotonic increasing in φ·p̄");
        assert!(c < 1.34 + 1e-9, "c_rho upper bound 1.34 violated: {c}");
        // 독립 손계산: φ=1, p̄=1.5GPa → (0.59+1.34·1.5)/(0.59+1.5)=2.6/2.09.
        assert_relative_eq!(c_rho(1.0, 1.5e9), 2.6 / 2.09, max_relative = 1e-12);
    }

    /// **c_ρ 2차효과 격리(M6-3)**: p̄↑ → c_ρ↑ → h_sep↑ → phi_bl↓ (밀도 2차경로).
    /// 주의: 이는 압축성 밀도효과의 격리이며, 지배효과(하중↑→h̄↓→phi_bl↑)는
    /// [`thicker_film_lowers_phi_bl_dominant`] 및 M1·M2 실결선(통합테스트)이 담당.
    #[test]
    fn compressibility_second_order_isolation() {
        let (dry, lub, grid, mut op, w) = synthetic(2.5e-8);
        op.p_h = 0.5e9;
        let phi_lo_p = combine_share(&dry, &lub, &grid, &op, &policy_for(w)).phi_bl;
        op.p_h = 3.0e9;
        let phi_hi_p = combine_share(&dry, &lub, &grid, &op, &policy_for(w)).phi_bl;
        assert!(
            phi_hi_p <= phi_lo_p + 1e-12,
            "higher p̄ → c_ρ↑ → h_sep↑ → phi_bl should not increase: {phi_hi_p} vs {phi_lo_p}"
        );
    }
}
