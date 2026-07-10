//! # M1 — 건식 접촉(dry rough contact) 모듈
//!
//! 반공간(half-space) 탄성 접촉을 **주파수영역(FFT) 스펙트럼법 + Kalker/Polonsky-Keer
//! 변분(공액구배) 반복**으로 해석한다. 입력 거칠기(rough1+rough2)로부터 실제 아스페리티
//! 접촉 압력장 `p_dry` 와 변형 후 간극장 `h_dry` 를 산출한다.
//!
//! 입력: [`PartialLubInput`] → 출력: [`DryResult`]. 좌표·부호·단위 규약은 [`crate::types`].
//!
//! ## 지배식 (SSOT 표준형)
//!
//! ### 식[1] 탄성 표면변위 — bi-sinusoidal 주파수응답
//! 반공간 표면 법선변위는 압력과의 순환 컨볼루션이며, 스펙트럼(주파수)영역에서 대각화된다:
//! ```text
//!   u = IFFT{ W(k) · FFT(p) }
//!   W(k) = 2 / (E_red · k),      k = 2π·√(f_x² + f_y²)   [rad/m]
//! ```
//! `W(k)` 는 bi-sinusoidal 압력 `p = cos(2π f_x x)cos(2π f_y y)` 에 대한 반공간
//! 주파수응답(컴플라이언스). 단위 `[m/Pa]`. DC(k=0)는 강체운동이므로 `W=0`.
//!
//! 유도: Boussinesq 커널 `u(x)=1/(πE_red)∬p(ξ)/|x−ξ|dξ` 의 2D 연속 푸리에변환.
//! `FT{1/r}=2π/k` 이므로 `û = (1/(πE_red))(2π/k)p̂ = (2/(E_red k))p̂`.
//! y-균일(f_y=0) 단면은 선접촉 log 커널 `û=(2/(E_red|k|))p̂ = 1/(πE_red f_x)·p̂` 와 정확히 일치
//! → 매끈 선접촉 Hertz 를 재현한다(오라클 테스트).
//!
//! ### E_red 규약 (SSOT, 전략 B)
//! `1/E_red=(1−ν1²)/E1+(1−ν2²)/E2`. 논문 `E'` 는 `E'=2·E_red` 로 1회 치환.
//! 위 `W(k)` 는 `E_red` 를 직접 사용한다(논문형 `E'` 로 쓰면 `W=4/(E'k)`).
//!
//! ### 변분(상보성) 조건 + 하중구속 + 소성절단
//! ```text
//!   p ≥ 0,  g = h + u − δ ≥ 0,  p·g = 0        (Signorini 상보성)
//!   ∑ p·dA = W  (목표하중; δ 는 라그랑주 승수=강체접근량)
//!   p ≤ p_lim   (탄성-완전소성 절단)
//! ```
//! Polonsky-Keer(1999) 공액구배법으로 반복: 접촉집합 갱신·음압 절단(p←max(p,0))·
//! 하중 재정규화(∑p·dA=W 로 시프트)·중첩(overlap) 점 복구를 결합. 수렴 후 `p≤p_lim` 절단.

use crate::types::{DryResult, Field2, Grid, PartialLubInput};
use crate::util::fft::{fft2_forward, fft2_inverse};
use rustfft::num_complex::Complex;
use std::f64::consts::PI;

/// 공액구배 반복 최대 횟수.
const MAX_ITER: usize = 2000;
/// 압력 상대변화 수렴 임계값.
const TOL: f64 = 1e-8;

/// 건식 rough 접촉 해석.
///
/// 복합 표면 `s = rough1 + rough2`(평균 0) 로부터 미변형 간극 `h = max(s) − s ≥ 0` 을
/// 구성하고, 목표하중 `W = p_h · Lx · Ly` (평균압력 규약; RQ 참조) 로 FFT 변분 접촉을 푼다.
///
/// 반환: [`DryResult`] { `p_dry` [Pa], `h_dry` [m] }.
pub fn solve_dry(input: &PartialLubInput) -> DryResult {
    let g = &input.grid;
    let nx = g.nx;
    let ny = g.ny;
    if g.is_empty() {
        return DryResult {
            p_dry: Field2::zeros(nx, ny),
            h_dry: Field2::zeros(nx, ny),
        };
    }

    // 복합 표면 높이 s = rough1 + rough2 → 미변형 간극 h = s_max − s ( ≥ 0 ).
    let n = nx * ny;
    let mut s = vec![0.0f64; n];
    for i in 0..n {
        s[i] = input.rough1.data[i] + input.rough2.data[i];
    }
    let s_max = s.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let gap0_data: Vec<f64> = s.iter().map(|&si| s_max - si).collect();
    let gap0 = Field2::from_vec(nx, ny, gap0_data);

    // 목표하중 W = p_h · Lx · Ly  (평균압력 = p_h 규약; 매크로 곡률·R 미제공으로 인한 가정 — RQ)
    let target_load = input.op.p_h * g.lx * g.ly;

    dry_contact(g, &gap0, input.mat.e_red, input.mat.p_lim, target_load)
}

/// 변분 FFT 건식 접촉 코어(테스트용 공개).
///
/// * `grid`      — 계산 격자(주기 도메인).
/// * `gap0`      — 미변형 간극장 h [m] (h ≥ 0, 최고점에서 0). 매끈 선접촉이면 포물선.
/// * `e_red`     — 환산탄성계수 [Pa].
/// * `p_lim`     — 소성 압력 한계 [Pa] (≤0 이면 절단 비활성).
/// * `target_load` — 목표 총하중 W [N] = ∑ p·dA.
///
/// 반환: [`DryResult`] (수렴 압력·변형 후 간극).
pub fn dry_contact(
    grid: &Grid,
    gap0: &Field2,
    e_red: f64,
    p_lim: f64,
    target_load: f64,
) -> DryResult {
    let nx = grid.nx;
    let ny = grid.ny;
    let n = nx * ny;
    let da = grid.dx() * grid.dy();

    if n == 0 || target_load <= 0.0 || e_red <= 0.0 || da <= 0.0 {
        return DryResult {
            p_dry: Field2::zeros(nx, ny),
            h_dry: gap0.clone(),
        };
    }

    let h = &gap0.data;
    let w = build_influence(grid, e_red); // W(k) [m/Pa], DC=0

    let a_total = n as f64 * da;
    let p_uniform = target_load / a_total;
    let mut p = vec![p_uniform; n];
    let mut t = vec![0.0f64; n];
    let mut p_prev = p.clone();

    let mut g_norm_old = 1.0f64; // 이전 Σg² (공액계수 분모)
    let mut cg_beta_flag = 1.0f64; // 1: 공액유지, 0: (overlap 발생) 재시작

    for iter in 0..MAX_ITER {
        // ── 식[1] 탄성변위 u = IFFT{W·FFT(p)} ──
        let u = apply_influence(&p, &w, nx, ny);
        // 총 간극 g = h + u (강체접근 δ 는 아래 하중구속으로 흡수)
        let mut g = vec![0.0f64; n];
        for i in 0..n {
            g[i] = h[i] + u[i];
        }

        // 접촉집합 Ic = {p>0}
        let il: Vec<usize> = (0..n).filter(|&i| p[i] > 0.0).collect();
        if il.is_empty() {
            // 접촉 소실 → 최소 간극점 재시드
            let mut imin = 0usize;
            let mut gm = f64::INFINITY;
            for i in 0..n {
                if g[i] < gm {
                    gm = g[i];
                    imin = i;
                }
            }
            p[imin] = p_uniform;
            continue;
        }

        // 접촉집합 평균 간극(=강체접근량 δ) 차감 → 잔차 g←g−δ (0 으로 몰아갈 대상).
        // (이 시프트가 P-K 핵심: 없으면 상수 오프셋이 방향을 지배해 단일점으로 붕괴.)
        let mut g_bar = 0.0;
        for &i in &il {
            g_bar += g[i];
        }
        g_bar /= il.len() as f64;
        for gi in g.iter_mut() {
            *gi -= g_bar;
        }

        // Σ g² (접촉집합 위)
        let mut g_norm = 0.0;
        for &i in &il {
            g_norm += g[i] * g[i];
        }

        // 공액 탐색방향 t = g + β·t (Ic 위), β = flag·(G/G_old)
        let beta = if g_norm_old > 0.0 {
            cg_beta_flag * (g_norm / g_norm_old)
        } else {
            0.0
        };
        for i in 0..n {
            if p[i] > 0.0 {
                t[i] = g[i] + beta * t[i];
            } else {
                t[i] = 0.0;
            }
        }
        g_norm_old = g_norm;

        // r = K·t, 접촉집합 평균 차감(하중중립 갱신 보장)
        let mut r = apply_influence(&t, &w, nx, ny);
        let mut r_bar = 0.0;
        for &i in &il {
            r_bar += r[i];
        }
        r_bar /= il.len() as f64;
        for &i in &il {
            r[i] -= r_bar;
        }

        // 스텝 길이 τ = Σ_Ic g·t / Σ_Ic r·t
        let mut num = 0.0;
        let mut den = 0.0;
        for &i in &il {
            num += g[i] * t[i];
            den += r[i] * t[i];
        }
        let tau = if den.abs() > 1e-300 { num / den } else { 0.0 };

        // 압력 갱신(Ic) + 음압 절단
        for &i in &il {
            p[i] -= tau * t[i];
        }
        for i in 0..n {
            if p[i] < 0.0 {
                p[i] = 0.0;
            }
        }

        // 중첩집합 Iol = {p==0 & g<0} : 접촉 진입 복구
        let iol: Vec<usize> = (0..n).filter(|&i| p[i] == 0.0 && g[i] < 0.0).collect();
        if !iol.is_empty() {
            cg_beta_flag = 0.0; // CG 재시작
            for &i in &iol {
                p[i] -= tau * g[i]; // g<0 → p 증가
            }
        } else {
            cg_beta_flag = 1.0;
        }

        // 하중 재정규화 ∑ p·dA = W
        let mut cur = 0.0;
        for &pi in p.iter() {
            cur += pi;
        }
        cur *= da;
        if cur > 0.0 {
            let scale = target_load / cur;
            for pi in p.iter_mut() {
                *pi *= scale;
            }
        }

        // 수렴: 압력 L2 상대변화
        let mut dnum = 0.0;
        let mut dden = 0.0;
        for i in 0..n {
            let d = p[i] - p_prev[i];
            dnum += d * d;
            dden += p[i] * p[i];
        }
        let err = if dden > 0.0 { (dnum / dden).sqrt() } else { 1.0 };
        p_prev.copy_from_slice(&p);
        if iter > 3 && err < TOL {
            break;
        }
    }

    // 탄성-완전소성 절단 p ≤ p_lim
    if p_lim > 0.0 {
        for pi in p.iter_mut() {
            if *pi > p_lim {
                *pi = p_lim;
            }
        }
    }

    // 변형 후 간극장 h_dry = (h + u − δ)₊,  δ = 접촉집합 위 (h+u) 평균
    let u = apply_influence(&p, &w, nx, ny);
    let il: Vec<usize> = (0..n).filter(|&i| p[i] > 0.0).collect();
    let delta = if !il.is_empty() {
        let mut acc = 0.0;
        for &i in &il {
            acc += h[i] + u[i];
        }
        acc / il.len() as f64
    } else {
        0.0
    };
    let gap: Vec<f64> = (0..n).map(|i| (h[i] + u[i] - delta).max(0.0)).collect();

    DryResult {
        p_dry: Field2::from_vec(nx, ny, p),
        h_dry: Field2::from_vec(nx, ny, gap),
    }
}

/// bi-sinusoidal 주파수응답 W(k)=2/(E_red·k) 사전계산 [m/Pa], row-major, DC=0.
fn build_influence(grid: &Grid, e_red: f64) -> Vec<f64> {
    let nx = grid.nx;
    let ny = grid.ny;
    let mut w = vec![0.0f64; nx * ny];
    for j in 0..ny {
        let fy = freq(j, ny, grid.ly); // 공간주파수 [1/m]
        for i in 0..nx {
            let fx = freq(i, nx, grid.lx);
            let k = 2.0 * PI * (fx * fx + fy * fy).sqrt(); // 각파수 [rad/m]
            w[i + j * nx] = if k > 0.0 { 2.0 / (e_red * k) } else { 0.0 };
        }
    }
    w
}

/// DFT 주파수 빈 → 부호있는 공간주파수 f [1/m] (표준 FFT 정렬).
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

/// 식[1] 연산자: u = IFFT{ W(k) · FFT(p) }. 실수 필드 in→out.
fn apply_influence(p: &[f64], w: &[f64], nx: usize, ny: usize) -> Vec<f64> {
    let n = nx * ny;
    let mut buf: Vec<Complex<f64>> = p.iter().map(|&x| Complex::new(x, 0.0)).collect();
    fft2_forward(&mut buf, nx, ny);
    for i in 0..n {
        buf[i] *= w[i];
    }
    fft2_inverse(&mut buf, nx, ny);
    buf.iter().map(|c| c.re).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;
    use crate::util::hertz::hertz_line;

    fn dummy_input() -> PartialLubInput {
        let grid = Grid::new(4, 4, 1e-4, 1e-4);
        PartialLubInput {
            grid,
            rough1: Field2::zeros(4, 4),
            rough2: Field2::zeros(4, 4),
            mat: MaterialProps {
                e_red: E_RED_STEEL_PA,
                nu: NU_STEEL,
                hardness: 7e9,
                p_lim: 4e9,
            },
            op: OperatingConditions {
                p_h: 1.5e9,
                u_mean: 1.0,
                u2: 0.9,
                slide_roll: 0.1,
                eta0: 0.01,
                alpha_visc: 2e-8,
                tau0: 5e6,
                temp: 353.0,
            },
            h_bar: 1.4e-7,
        }
    }

    /// solve_dry 스모크: 평탄 표면(거칠기 0) → 균일 압력 = 평균압력 규약(p_h).
    #[test]
    fn solve_dry_flat_uniform_pressure() {
        let inp = dummy_input();
        let r = solve_dry(&inp);
        assert_eq!(r.p_dry.len(), 16);
        // 평탄 접촉: 전점 압력 ≈ p_h (목표하중/면적)
        let pmax = r.p_dry.max().unwrap();
        let pmin = r.p_dry.min().unwrap();
        assert!((pmax - inp.op.p_h).abs() / inp.op.p_h < 1e-3);
        assert!((pmin - inp.op.p_h).abs() / inp.op.p_h < 1e-3);
    }

    /// **오라클**: 매끈 선접촉 → FFT 변분해가 Hertz p0(최대압) 를 재현(상대오차 ≤ 2%).
    #[test]
    fn smooth_line_contact_matches_hertz() {
        // 물리 조건 (util::hertz_line 회귀와 동일): R=0.01 m, w'=1e5 N/m, E_red=115.4e9
        let e_red = E_RED_STEEL_PA;
        let r_eq = 0.01_f64;
        let w_per_len = 1.0e5_f64; // [N/m]
        let (p0_ref, b_ref) = hertz_line(w_per_len, e_red, r_eq);

        // 격자: x=구름방향에 포물선, y=횡방향 균일(선접촉/평면변형).
        // 접촉폭 2b 를 충분히 분해하고, 도메인은 접촉 대비 넓게(주기오차 억제).
        let nx = 1024usize;
        let ny = 4usize;
        let lx = 40.0 * b_ref; // ≈ ±20b
        let ly = 1.0e-4_f64; // 임의(횡방향 균일이라 선하중에서 상쇄)
        let grid = Grid::new(nx, ny, lx, ly);

        // 미변형 간극 h = xc²/(2R), xc: 도메인 중앙 기준. y 균일. min=0(중앙).
        let dx = grid.dx();
        let mut h = vec![0.0f64; nx * ny];
        for j in 0..ny {
            for i in 0..nx {
                let xc = (i as f64 - nx as f64 / 2.0) * dx;
                h[i + j * nx] = xc * xc / (2.0 * r_eq);
            }
        }
        let gap0 = Field2::from_vec(nx, ny, h);

        // 목표 총하중 W = w'·Ly  (횡방향 균일)
        let target_load = w_per_len * ly;
        // 소성 절단 비활성(충분히 큰 p_lim)
        let res = dry_contact(&grid, &gap0, e_red, 1.0e30, target_load);

        // 최대 접촉압 재현(≤2%)
        let p0_num = res.p_dry.max().unwrap();
        let p0_err = (p0_num - p0_ref).abs() / p0_ref;
        assert!(
            p0_err <= 0.02,
            "p0: num={:.4e} ref={:.4e} rel_err={:.3}% (>2%)",
            p0_num,
            p0_ref,
            p0_err * 100.0
        );

        // 접촉 반폭 b 재현(느슨한 sanity, ≤10%): 중앙 행에서 p>0.02·p0 범위
        let j0 = 0usize;
        let thr = 0.02 * p0_num;
        let mut imin = nx;
        let mut imax = 0usize;
        for i in 0..nx {
            if res.p_dry.at(i, j0) > thr {
                if i < imin {
                    imin = i;
                }
                if i > imax {
                    imax = i;
                }
            }
        }
        let width = (imax as f64 - imin as f64) * dx;
        let b_num = width / 2.0;
        let b_err = (b_num - b_ref).abs() / b_ref;
        assert!(
            b_err <= 0.10,
            "b: num={:.4e} ref={:.4e} rel_err={:.3}% (>10%)",
            b_num,
            b_ref,
            b_err * 100.0
        );

        // 하중 보존 확인
        let mut wsum = 0.0;
        for v in &res.p_dry.data {
            wsum += *v;
        }
        wsum *= grid.dx() * grid.dy();
        assert!(
            (wsum - target_load).abs() / target_load < 1e-3,
            "load not conserved: {:.4e} vs {:.4e}",
            wsum,
            target_load
        );
    }

    /// 소성 절단: p_lim 이 낮으면 최대압이 p_lim 로 상한.
    #[test]
    fn plastic_clamp_caps_pressure() {
        let e_red = E_RED_STEEL_PA;
        let r_eq = 0.01_f64;
        let w_per_len = 1.0e5_f64;
        let (p0_ref, b_ref) = hertz_line(w_per_len, e_red, r_eq);
        let p_lim = 0.5 * p0_ref;

        let nx = 512usize;
        let ny = 4usize;
        let lx = 40.0 * b_ref;
        let ly = 1.0e-4_f64;
        let grid = Grid::new(nx, ny, lx, ly);
        let dx = grid.dx();
        let mut h = vec![0.0f64; nx * ny];
        for j in 0..ny {
            for i in 0..nx {
                let xc = (i as f64 - nx as f64 / 2.0) * dx;
                h[i + j * nx] = xc * xc / (2.0 * r_eq);
            }
        }
        let gap0 = Field2::from_vec(nx, ny, h);
        let res = dry_contact(&grid, &gap0, e_red, p_lim, w_per_len * ly);
        let pmax = res.p_dry.max().unwrap();
        assert!(pmax <= p_lim * (1.0 + 1e-9), "pmax {:.3e} > p_lim {:.3e}", pmax, p_lim);
    }

    /// 비물리/빈 입력 → 0 압력.
    #[test]
    fn nonphysical_returns_zero() {
        let grid = Grid::new(8, 8, 1e-4, 1e-4);
        let gap0 = Field2::zeros(8, 8);
        let r = dry_contact(&grid, &gap0, E_RED_STEEL_PA, 4e9, 0.0);
        assert_eq!(r.p_dry.max().unwrap(), 0.0);
    }
}
