//! # M6 — 하중분담/공유 결합(share) 모듈 [구현]
//!
//! 건식(M1 `DryResult`)·완전윤활(M2 `LubResult`) 국부해를 결합하여 **부분윤활
//! (partial/mixed) 하중분담**을 산정한다. 아스페리티(경계윤활)와 유막이 총하중
//! `W`를 나누어 지지하며, 그 분율이 경계윤활 분율 `phi_bl` 이다.
//!
//! ## 알고리즘 (flow-balance 반복)
//! 1. `phi_bl` 가정 → 유막하중 `W_film=(1-phi_bl)·W`.
//! 2. 유막하중으로부터 평균 분리간극 `h_sep` 산정. 유막하중이 작을수록(=phi_bl↑)
//!    유막이 두꺼워져 분리간극이 커진다(음성 피드백):
//!    `h_sep = sigma_r · (1-phi_bl)^(-k_film)`  (`sigma_r`=거칠기 표준편차).
//! 3. **영역 재식별**: 거칠기 peak height `r_i > h_sep` 인 점만 아스페리티 접촉영역 A.
//!    침투 적분 `num = Σ_A (r_i - h_sep)`.
//! 4. 압축성 보정된 유막 용량 `cfilm = c_film0·c_rho(p_H)·Σr_i` 로 암시분율
//!    `phi_impl = num/(num+cfilm)`. `h_sep↑ → num↓ → phi_impl↓` 이므로 유일 내부
//!    고정점이 존재(음성 피드백 → 수렴).
//! 5. 완화 `phi ← (1-ω)·phi + ω·phi_impl`, `ω∈[0.3,0.7]`. 수렴까지 반복.
//!
//! ## 전이 유막/간극장 `h_tran`
//! `h_tran = mean(h_lub) + IFFT{ C }`, 여기서 각 주파수 bin 에서
//! `C[k] = (|H_dry[k]| ≥ |H_lub[k]|) ? H_dry[k] : H_lub[k]` (=진폭 max, 위상 보존).
//! **DC 처리(w(1,1)=0)**: 1-index `(1,1)`(=0-index `[0]`) bin 을 0 으로 두어 평균
//! 성분을 스펙트럼 합성에서 제외하고, 강체 분리(평균 유막)로 별도 재부여한다.
//!
//! ## 하중 보존 (SSOT)
//! 최종 국부압력장은 아스페리티/유막 각각을 자기 하중에 맞게 스케일하여
//! `∫ p_dry_local dA + ∫ p_lub_local dA = W` (엄밀 보존)을 만족한다.
//!
//! ## 단위 (SI)
//! 길이/간극/유막 [m], 압력 [Pa], 하중 [N], 면적요소 `dA = dx·dy` [m²].

use crate::types::{DryResult, Field2, Grid, LubResult, OperatingConditions, PartialLubResult, EPS};
use crate::util::fft::{fft2_forward, fft2_inverse};
use rustfft::num_complex::Complex;

/// Dowson–Higginson 압축성 밀도비 `c_rho = ρ/ρ0` [-].
///
/// `ρ/ρ0 = 1 + (C1·p)/(1 + C2·p)`,  `C1 = 0.6e-9`, `C2 = 1.7e-9` [Pa⁻¹]
/// (압력 p 는 [Pa], 인장 + 규약이므로 접촉 압력크기 p≥0 사용).
/// `c_rho(0)=1`, p>0 에서 단조증가, 상한 `1 + C1/C2 ≈ 1.353`.
pub fn c_rho(p: f64) -> f64 {
    const C1: f64 = 0.6e-9;
    const C2: f64 = 1.7e-9;
    let p = p.max(0.0);
    1.0 + (C1 * p) / (1.0 + C2 * p)
}

/// 하중분담 반복 정책/파라미터.
#[derive(Debug, Clone, Copy)]
pub struct SharePolicy {
    /// 총 접촉하중 W [N] (패치 압력적분의 목표값).
    pub w_total: f64,
    /// 완화계수 ω (SSOT 상 [0.3,0.7] 로 클램프).
    pub omega: f64,
    /// 최대 반복 횟수.
    pub max_iter: usize,
    /// 수렴 판정 |Δphi| 임계값.
    pub tol: f64,
    /// 유막 분리 지수 k_film (>0). h_sep = sigma_r·(1-phi)^(-k_film).
    pub k_film: f64,
    /// 유막 용량 계수 c_film0 [-].
    pub c_film0: f64,
    /// 초기 phi_bl 가정.
    pub phi0: f64,
}

impl Default for SharePolicy {
    fn default() -> Self {
        SharePolicy {
            w_total: 0.0,
            omega: 0.5,
            max_iter: 500,
            tol: 1e-10,
            k_film: 0.5,
            c_film0: 0.3,
            phi0: 0.5,
        }
    }
}

/// flow-balance 반복 추적(진단)용.
#[derive(Debug, Clone)]
pub struct ShareTrace {
    /// 반복별 phi_bl 이력(초기값 포함).
    pub phi_history: Vec<f64>,
    /// 수렴 여부.
    pub converged: bool,
    /// 실제 반복 횟수.
    pub iters: usize,
    /// 최종 분리간극 h_sep [m].
    pub h_sep: f64,
    /// 최종 아스페리티 접촉점 수.
    pub contact_count: usize,
    /// 하중 보존 잔차 |(∫dry+∫lub) - W| / W [-].
    pub load_residual: f64,
}

/// 각 주파수 bin 에서 진폭이 큰 스펙트럼을 취하고 DC(=w(1,1))를 0 으로 둔 결합 스펙트럼.
///
/// `C[k] = (|H_dry[k]| ≥ |H_lub[k]|) ? H_dry[k] : H_lub[k]`, `C[0]=0`.
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
    // w(1,1)=0 : DC(평균) 성분 제거 → 스펙트럼 합성은 순수 변동분만. 평균은 강체분리로 별도.
    if n > 0 {
        c[0] = Complex::new(0.0, 0.0);
    }
    c
}

/// 전이 유막/간극장 h_tran [m] 과 그 변동분(fluct, 평균 0)을 산정.
///
/// `h_tran = mean(h_lub) + fluct`, `fluct = Re(IFFT{combined_spectrum})`.
fn build_h_tran(h_dry: &Field2, h_lub: &Field2) -> (Field2, Vec<f64>) {
    let nx = h_dry.nx;
    let ny = h_dry.ny;
    let n = nx * ny;
    let mut c = combined_spectrum(h_dry, h_lub);
    fft2_inverse(&mut c, nx, ny);
    let fluct: Vec<f64> = c.iter().map(|z| z.re).collect();
    // 강체 분리(평균 유막) 재부여: h_lub 평균.
    let h_mean = if n > 0 {
        h_lub.data.iter().sum::<f64>() / n as f64
    } else {
        0.0
    };
    let data: Vec<f64> = fluct.iter().map(|&f| h_mean + f).collect();
    (Field2::from_vec(nx, ny, data), fluct)
}

/// 부분윤활 하중분담 결합(추적 포함).
///
/// 입력: 건식/윤활 국부해(mock 가능), 격자, 운전조건(p_H→c_rho), 정책.
/// 반환: `PartialLubResult { p_tran, h_tran, phi_bl }` 및 반복 추적 `ShareTrace`.
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
    let omega = policy.omega.clamp(0.3, 0.7); // SSOT: ω∈[0.3,0.7]
    let k_film = policy.k_film.max(EPS);

    // ── 1) 전이 유막/간극장 h_tran (스펙트럼 진폭 max, DC=0) ──
    let (h_tran, fluct) = build_h_tran(&dry.h_dry, &lub.h_lub);

    // 거칠기 peak height r_i (유막 최박 지점=큰 값). r_i = max(fluct) - fluct_i ≥ 0.
    let fmax = fluct.iter().cloned().fold(f64::MIN, f64::max);
    let r: Vec<f64> = fluct.iter().map(|&f| fmax - f).collect();
    let sum_r: f64 = r.iter().sum();
    let mean_r = if n > 0 { sum_r / n as f64 } else { 0.0 };
    let var_r = if n > 0 {
        r.iter().map(|&x| (x - mean_r) * (x - mean_r)).sum::<f64>() / n as f64
    } else {
        0.0
    };
    let sigma_r = var_r.sqrt().max(EPS);

    // 압축성 보정된 유막 용량 (c_rho: Dowson–Higginson).
    let cfilm = (policy.c_film0 * c_rho(op.p_h) * sum_r).max(EPS);

    // ── 2) flow-balance 반복 (phi 가정 → h_sep → 영역 재식별 → 수렴) ──
    let mut phi = policy.phi0.clamp(1e-6, 1.0 - 1e-6);
    let mut history = vec![phi];
    let mut converged = false;
    let mut iters = 0usize;
    for _ in 0..policy.max_iter {
        iters += 1;
        // 유막하중↓(phi↑) → 유막 두꺼워짐 → 분리간극↑ → 접촉↓ (음성 피드백).
        let h_sep = sigma_r * (1.0 - phi).powf(-k_film);
        // 영역 재식별 + 침투 적분.
        let mut num = 0.0;
        for &ri in &r {
            let pen = ri - h_sep;
            if pen > 0.0 {
                num += pen;
            }
        }
        let phi_impl = (num / (num + cfilm)).clamp(0.0, 1.0);
        let phi_new = (1.0 - omega) * phi + omega * phi_impl;
        history.push(phi_new);
        if (phi_new - phi).abs() < policy.tol {
            phi = phi_new;
            converged = true;
            break;
        }
        phi = phi_new.clamp(1e-6, 1.0 - 1e-6);
    }

    // ── 3) 하중 배분 & 국부압력장 (엄밀 보존) ──
    let h_sep = sigma_r * (1.0 - phi).powf(-k_film);
    let mut contact_count = 0usize;
    let mut i_dry_a = 0.0; // 접촉영역 A 에서의 dry 압력적분 [N]
    for k in 0..n {
        if r[k] > h_sep {
            contact_count += 1;
            i_dry_a += dry.p_dry.data[k] * da;
        }
    }
    let w_asp_target = phi * w;
    // 접촉영역이 실재할 때만 아스페리티 하중을 부여(없으면 전량 유막으로).
    let (s_dry, actual_w_asp) = if i_dry_a > EPS {
        (w_asp_target / i_dry_a, w_asp_target)
    } else {
        (0.0, 0.0)
    };
    let w_film = w - actual_w_asp;
    let i_lub: f64 = lub.p_lub.data.iter().map(|&p| p * da).sum();
    let s_lub = if i_lub > EPS { w_film / i_lub } else { 0.0 };

    let mut p_tran = Field2::zeros(nx, ny);
    for k in 0..n {
        let pd = if r[k] > h_sep {
            dry.p_dry.data[k] * s_dry
        } else {
            0.0
        };
        let pl = lub.p_lub.data[k] * s_lub;
        p_tran.data[k] = pd + pl;
    }

    // 하중 보존 잔차 진단.
    let w_dry_int: f64 = if i_dry_a > EPS { s_dry * i_dry_a } else { 0.0 };
    let w_lub_int: f64 = s_lub * i_lub;
    let load_residual = if w.abs() > EPS {
        ((w_dry_int + w_lub_int) - w).abs() / w.abs()
    } else {
        (w_dry_int + w_lub_int).abs()
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
    fn synthetic() -> (DryResult, LubResult, Grid, OperatingConditions, f64) {
        let nx = 32usize;
        let ny = 32usize;
        let lx = 200e-6;
        let ly = 200e-6;
        let grid = Grid::new(nx, ny, lx, ly);
        let dx = grid.dx();
        let dy = grid.dy();
        let b = 60e-6;
        let p_h = 1.5e9;
        let h_bar = 1.4e-7;
        let cx = lx / 2.0;
        let cy = ly / 2.0;

        let mut p_dry = Field2::zeros(nx, ny);
        let mut h_dry = Field2::zeros(nx, ny);
        let mut p_lub = Field2::zeros(nx, ny);
        let mut h_lub = Field2::zeros(nx, ny);

        // dry 와 lub 은 파장/진폭이 다른 거칠기 → 스펙트럼 max 규칙이 유의미.
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

                // dry 압력: Hertz 반타원.
                let ud = 1.0 - r2 / (b * b);
                let pd = if ud > 0.0 { p_h * ud.sqrt() } else { 0.0 };
                p_dry.set(i, j, pd);

                // lub 압력: 약간 넓고 낮게.
                let bl = 1.15 * b;
                let ul = 1.0 - r2 / (bl * bl);
                let pl = if ul > 0.0 { 0.9 * p_h * ul.sqrt() } else { 0.0 };
                p_lub.set(i, j, pl);

                // 거칠기.
                let rd = 0.40e-7 * (kx1 * x).cos() * (ky1 * y).cos();
                let rl = 0.60e-7 * (kx2 * x).sin() * (ky2 * y).cos();

                // dry 간극: 접촉영역 ~0, 외곽 증가 + 거칠기.
                let gap = if ud > 0.0 {
                    0.0
                } else {
                    0.5e-7 * (r2 / (b * b) - 1.0)
                };
                h_dry.set(i, j, gap + rd);

                // lub 유막두께: h_bar + 거칠기.
                h_lub.set(i, j, h_bar + rl);
            }
        }

        let da = dx * dy;
        // 적용 총하중 W = full dry 압력적분 [N].
        let w: f64 = p_dry.data.iter().map(|&p| p * da).sum();
        (
            DryResult { p_dry, h_dry },
            LubResult { p_lub, h_lub },
            grid,
            op_ref(),
            w,
        )
    }

    // ── 하중 보존: ∫dry + ∫lub = W (<= 0.1%) ──
    #[test]
    fn load_conservation() {
        let (dry, lub, grid, op, w) = synthetic();
        let policy = SharePolicy {
            w_total: w,
            ..Default::default()
        };
        let (res, trace) = combine_share_traced(&dry, &lub, &grid, &op, &policy);
        // 엄밀 보존(구성상) → 사실상 반올림 오차만.
        assert!(
            trace.load_residual < 1e-3,
            "load residual {} !< 0.1%",
            trace.load_residual
        );
        // 직접 재적분으로도 확인.
        let da = grid.dx() * grid.dy();
        let total: f64 = res.p_tran.data.iter().map(|&p| p * da).sum();
        assert_relative_eq!(total, w, max_relative = 1e-3);
        assert!(w > 0.0);
    }

    // ── phi_bl 반복 수렴 & 내부값 ──
    #[test]
    fn phi_bl_converges_interior() {
        let (dry, lub, grid, op, w) = synthetic();
        let policy = SharePolicy {
            w_total: w,
            tol: 1e-9,
            ..Default::default()
        };
        let (res, trace) = combine_share_traced(&dry, &lub, &grid, &op, &policy);
        assert!(trace.converged, "flow-balance did not converge in {} iters", trace.iters);
        assert!(
            res.phi_bl > 0.0 && res.phi_bl < 1.0,
            "phi_bl={} not interior",
            res.phi_bl
        );
        // 마지막 스텝 잔차가 tol 미만.
        let last = trace.phi_history.len();
        let d = (trace.phi_history[last - 1] - trace.phi_history[last - 2]).abs();
        assert!(d < policy.tol, "final |Δphi|={} !< tol", d);
        // 접촉영역이 실재(영역 재식별 유효).
        assert!(trace.contact_count > 0 && trace.contact_count < grid.len());
    }

    // ── 완화계수 무관 동일 고정점 (ω∈[0.3,0.7]) ──
    #[test]
    fn fixed_point_independent_of_omega() {
        let (dry, lub, grid, op, w) = synthetic();
        let mut phis = Vec::new();
        for &omega in &[0.3_f64, 0.5, 0.7] {
            let policy = SharePolicy {
                w_total: w,
                omega,
                tol: 1e-11,
                ..Default::default()
            };
            let (res, tr) = combine_share_traced(&dry, &lub, &grid, &op, &policy);
            assert!(tr.converged);
            phis.push(res.phi_bl);
        }
        assert_relative_eq!(phis[0], phis[1], max_relative = 1e-5);
        assert_relative_eq!(phis[1], phis[2], max_relative = 1e-5);
    }

    // ── 반환형/차원 계약 ──
    #[test]
    fn returns_partial_lub_contract() {
        let (dry, lub, grid, op, w) = synthetic();
        let policy = SharePolicy {
            w_total: w,
            ..Default::default()
        };
        let res = combine_share(&dry, &lub, &grid, &op, &policy);
        assert_eq!(res.p_tran.nx, grid.nx);
        assert_eq!(res.p_tran.ny, grid.ny);
        assert_eq!(res.h_tran.len(), grid.len());
        assert!(res.p_tran.data.iter().all(|v| v.is_finite()));
        assert!(res.h_tran.data.iter().all(|v| v.is_finite()));
    }

    // ── h_tran: 스펙트럼 진폭 max & DC(w(1,1))=0 ──
    #[test]
    fn h_tran_spectrum_max_and_dc_zero() {
        let nx = 8usize;
        let ny = 8usize;
        // 서로 다른 두 패턴.
        let mut hd = Field2::zeros(nx, ny);
        let mut hl = Field2::zeros(nx, ny);
        for j in 0..ny {
            for i in 0..nx {
                hd.set(i, j, 3.0 + (i as f64).sin() + 0.5 * (j as f64).cos());
                hl.set(i, j, 2.0 + (2.0 * i as f64).cos() + (0.7 * j as f64).sin());
            }
        }
        // 참조 스펙트럼.
        let mut fd: Vec<Complex<f64>> = hd.data.iter().map(|&v| Complex::new(v, 0.0)).collect();
        let mut fl: Vec<Complex<f64>> = hl.data.iter().map(|&v| Complex::new(v, 0.0)).collect();
        fft2_forward(&mut fd, nx, ny);
        fft2_forward(&mut fl, nx, ny);

        let c = combined_spectrum(&hd, &hl);
        // DC = 0.
        assert_relative_eq!(c[0].norm(), 0.0, epsilon = 1e-12);
        // 그 외 bin: 진폭 = max(|Hd|,|Hl|).
        for k in 1..nx * ny {
            let expected = fd[k].norm().max(fl[k].norm());
            assert_relative_eq!(c[k].norm(), expected, max_relative = 1e-9);
        }

        // fluct 평균 ≈ 0 (DC 제거 결과).
        let (_htran, fluct) = build_h_tran(&hd, &hl);
        let mean = fluct.iter().sum::<f64>() / fluct.len() as f64;
        assert_relative_eq!(mean, 0.0, epsilon = 1e-9);
    }

    // ── c_rho 압축성 보정: c_rho(0)=1, 단조증가, 상한 ──
    #[test]
    fn c_rho_properties() {
        assert_relative_eq!(c_rho(0.0), 1.0, max_relative = 1e-15);
        let a = c_rho(0.5e9);
        let b = c_rho(1.5e9);
        let c = c_rho(3.0e9);
        assert!(a > 1.0 && b > a && c > b, "c_rho not monotonic increasing");
        // 상한 1 + C1/C2 ≈ 1.3529.
        assert!(c < 1.0 + 0.6 / 1.7 + 1e-9);
    }

    // ── c_rho 가 유막 용량↑ → phi_bl↓ (물리적 방향) ──
    #[test]
    fn compressibility_lowers_phi_bl() {
        let (dry, lub, grid, mut op, w) = synthetic();
        let policy = SharePolicy {
            w_total: w,
            ..Default::default()
        };
        op.p_h = 0.5e9;
        let phi_lo_p = combine_share(&dry, &lub, &grid, &op, &policy).phi_bl;
        op.p_h = 3.0e9;
        let phi_hi_p = combine_share(&dry, &lub, &grid, &op, &policy).phi_bl;
        // 높은 p_H → c_rho↑ → 유막 용량↑ → 아스페리티 분율↓.
        assert!(
            phi_hi_p < phi_lo_p,
            "expected phi_bl to drop with compressibility: {} !< {}",
            phi_hi_p,
            phi_lo_p
        );
    }
}
