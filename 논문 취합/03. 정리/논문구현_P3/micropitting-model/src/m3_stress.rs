//! # M3 — 표면하(subsurface) 응력장 모듈: 정현파 폐형식 FRF + 2D-FFT 모드중첩
//!
//! 표면 트랙션(법선 압력 `p` + 접선 트랙션 `q=μ·p`)이 주어졌을 때 반무한 탄성체
//! 내부의 6성분 응력텐서장 + von Mises 등가응력을 깊이층별로 산출한다.
//!
//! ## 정본 근거 (Tripp et al. 2003, "Frequency Response Functions and Rough Surface")
//! 원문 파일: `2003. (SKF) Frequency Response Functions and Rough Surface.md`.
//! **bi-sinusoidal 표면하중 → 6응력 폐형식**을 그대로 채택한다(자의적 계수 없음):
//!
//! ### 식[8]/[10] — 법선 하중 `σ_z(x,y,0)=p0·cos(αx)cos(βy)`  (원문 line 156, 186)
//! `ζ=√(α²+β²)`, `e=e^{−ζz}` 로 두면(원문 verbatim):
//! ```text
//!   σ_x  = p0[ α²/ζ² − α²z/ζ + 2ν(β/ζ)² ]·e·cos(αx)cos(βy)
//!   σ_y  = p0[ β²/ζ² − β²z/ζ + 2ν(α/ζ)² ]·e·cos(αx)cos(βy)
//!   σ_z  = p0( 1 + ζz )·e·cos(αx)cos(βy)
//!   τ_xy = −p0(αβ/ζ²)[ (1−2ν) − ζz ]·e·sin(αx)sin(βy)
//!   τ_yz =  p0(βz)·e·cos(αx)sin(βy)
//!   τ_xz =  p0(αz)·e·sin(αx)cos(βy)
//! ```
//!
//! ### 식[9]/[16] — 접선 하중 `τ_xz(x,y,0)=q0·cos(αx)cos(βy)`  (원문 line 160, 229)
//! ```text
//!   σ_x  = q0(α/ζ)[ 2 + 2ν(β/ζ)² − (α/ζ)(αz) ]·e·sin(αx)cos(βy)
//!   σ_y  = q0(α/ζ)[ 2ν(α/ζ)²     − (β/ζ)(βz) ]·e·sin(αx)cos(βy)
//!   σ_z  = q0(αz)·e·sin(αx)cos(βy)
//!   τ_xy = q0(β/ζ)[ 1 − 2ν(α/ζ)² − (α/ζ)(αz) ]·e·cos(αx)sin(βy)
//!   τ_yz = q0(β/ζ)(αz)·e·sin(αx)sin(βy)
//!   τ_xz = q0[ 1 − (α/ζ)(αz) ]·e·cos(αx)cos(βy)
//! ```
//! > ⚠️ 2011 재인용본 식[13] σ_y 는 **전사 오류**((α/ζ)²→(β/ζ)², (β/ζ)(βz)→(α/ζ)(αz);
//! > P2-1 §2.4 지적). 여기서는 **Tripp 2003 원전 식[16]** 만 사용. `vc_m3_trace_*` 가
//! > trace 항등으로 정본을 확증(2011 손상식은 불성립).
//!
//! ## 복소 전달함수(FRF)로의 변환 — FFT 모드중첩 (convolution 아님)
//! 실 하중장을 2D-FFT 하면 각 bin 이 복소모드 `e^{+i(a·x+b·y)}` (a=kx, b=ky) 로 표현된다.
//! 위 실수 cos/sin 폐형식을 복소모드 전달함수 `H(a,b,z)` 로 소급한다. `cos(αx)cos(βy)` 를
//! 4개 복소모드 `¼Σ_{s1,s2} e^{i(s1αx+s2βy)}` 로 분해하고 출력의 구조(cc/sc/cs/ss)를 대응시키면:
//! - `cc`(cos·cos) 출력 → 실수·짝수 전달함수,
//! - `sc`(sin·cos) → `−i·(a/|a|)`·(홀수 in a),  `cs`(cos·sin) → `−i`(홀수 in b),
//! - `ss`(sin·sin) → 실수(부호 `ab` 결합).
//!
//! 결과(입력 스펙트럼 `P̂=FFT(p)`, `Q̂=FFT(q)` 에 곱해 각 성분 IFFT·중첩):
//! ```text
//! 법선(P̂):   Hxx=(a²/ζ²−a²z/ζ+2νb²/ζ²)e         [실]
//!            Hyy=(b²/ζ²−b²z/ζ+2νa²/ζ²)e         [실]
//!            Hzz=(1+ζz)e                          [실]
//!            Hxy=(ab/ζ²)[(1−2ν)−ζz]e             [실]
//!            Hyz=−i·(bz)e   Hxz=−i·(az)e          [허]
//! 접선(Q̂):   Hxx=−i(a/ζ)[2+2νb²/ζ²−a²z/ζ]e       [허]
//!            Hyy=−i(a/ζ)[2νa²/ζ²−b²z/ζ]e         [허]
//!            Hzz=−i·(az)e                         [허]
//!            Hxy=−i(b/ζ)[1−2νa²/ζ²−a²z/ζ]e        [허]
//!            Hyz=−(abz/ζ)e   Hxz=(1−a²z/ζ)e        [실]
//! ```
//! 각 `H(−a,−b)=conj(H(a,b))` (Hermitian) → 실 입력의 IFFT 는 실수장을 보존
//! (`vc_m3_sin_*` 가 위 폐형식과 <1e-6 일치 강제).
//!
//! ## 알고리즘 (P2-3 §2.1, 원문 RESULTS)
//! `p,q` 각 1회 forward-FFT → bin 별 `H(a,b,z)` 선형적용 → 6성분 IFFT(중첩) → von Mises.
//! 깊이 `z=0~0.25b`, 15층(원문 "Rolling in a Rough Lubricated Groove" 예제 `z/a=0~0.25,
//! 30 등간격점" 의 등간격 절반해상 축약; P2-3 §2.1). 비주기 하중은 large window+zero-pad
//! (주기성 오차 지배, Polonsky-Keer; VC-M3-Hertz 가 매끈 Hertz 로 외부검증).
//!
//! ## DC(ζ=0) 모드 처리 — 균일하중 참응답 + 잔여 FLAG
//! Tripp 2003 은 DC(공간평균) 전달함수를 명시하지 않는다(in-plane 항 `a²/ζ²` 는 (0,0)서 0/0).
//! **균일 표면하중의 고전 참응답**을 채택: 법선 균일하중 → 구속(오이도미터) 상태
//! `σ_zz=p_DC`, `σ_xx=σ_yy=ν/(1−ν)·σ_zz`(Hooke `ε_xx=ε_yy=0` 유도), 접선 → `τ_xz=q_DC`, 나머지 0.
//! (측방항을 0 으로 두면 vM 편향 — 교정 전 VC-M3-Hertz +7.1% → 교정 후 +3.4%@32b창.)
//! **periodic-window 평균 아티팩트**(유한창 평균압이 고립 Hertz 엔 없는 구속하중으로 기여)는
//! 창 크기에 반비례(∝1/창; 실측 32b→+3.4%, 128b→+0.93%, 256b→+0.49%) → **대형창으로 <1% 수렴**
//! (VC-M3-Hertz 는 256b 서 +0.49%). 완전 결선(작은 창 <1%)은 평균분리(P2-3 §3.3 "평균 Hertz
//! 해석해 분리") 로 가능하나 대형창이 등가·단순.
//!
//! **접선(q) DC**: 균일 표면전단 → 반공간 **단순전단** `σ_xz=q_DC`(전 깊이), 나머지 0, vM=√3·q
//! (평형·적합성·BC(σ_zz(0)=0·τ_xz(0)=q) 만족 유일 응력해; u_x∝z 발산이나 응력은 잘 정의).
//! `dc_uniform_traction` 이 검증(구 미검증 gap 폐색). → **RQ-M3-DC 의 법선·접선 DC 응답은 정본 확정**;
//! 잔여는 실사용 매크로 정밀도(평균분리 P2-3 §3.3)뿐 — 리플(비-DC) 목적엔 불요·enhancement.

use crate::types::{Field2, Grid, MaterialProps, OperatingConditions, StressResult, StressTensor6};
use crate::util::fft::{fft2_forward, fft2_inverse};
use rustfft::num_complex::Complex;
use std::f64::consts::PI;

// ─────────────────────────────────────────────────────────────────────────
//  모듈-로컬 입력 계약 (types.rs 동결 — 여기서만 정의)
// ─────────────────────────────────────────────────────────────────────────

/// M3 표면하 응력 해석 입력.
///
/// - `p_tran`: 법선 압력장 [Pa] (부호: 압력 크기 양수; 원문 `σ_z(x,y,0)=p`).
/// - `q_tran`: 접선 트랙션장 [Pa] (`q=μ·p`, Coulomb; 원문 `τ_xz(x,y,0)=q`).
/// - `mat.nu`: 푸아송비, `op.r_x`·`op.p_h`·`mat.e_red`: 접촉반폭 `b` 유도 입력.
///
/// 깊이 범위는 [`solve_stress`] 가 `b = 2·r_x·p_h/e_red` (Hertz 선접촉; E'=2·E_red 치환,
/// types.rs `r_x` 주석) 로부터 `z=0~0.25b`, 15층으로 자동 산출한다.
pub struct StressInput {
    /// 계산 격자.
    pub grid: Grid,
    /// 법선 압력장 [Pa].
    pub p_tran: Field2,
    /// 접선 트랙션장 [Pa] (= μ·p_tran).
    pub q_tran: Field2,
    /// 재료 물성 (nu 사용).
    pub mat: MaterialProps,
    /// 운전 조건 (r_x, p_h 로 b 산출).
    pub op: OperatingConditions,
}

/// 기본 깊이층 수 (원문 예제 30등간격점의 절반해상 축약; P2-3 §2.1).
pub const NZ_DEFAULT: usize = 15;
/// 기본 깊이범위 계수: `z_max = DEPTH_FRAC·b` (원문 예제 `z/a=0~0.25`).
pub const DEPTH_FRAC: f64 = 0.25;

// ─────────────────────────────────────────────────────────────────────────
//  von Mises
// ─────────────────────────────────────────────────────────────────────────

/// von Mises 등가응력 [Pa] — 표준식.
///
/// `σ_vM = √( ½[(σx−σy)²+(σy−σz)²+(σz−σx)²] + 3(τxy²+τyz²+τxz²) )`.
#[inline]
pub fn von_mises(sxx: f64, syy: f64, szz: f64, sxy: f64, syz: f64, sxz: f64) -> f64 {
    let dev = (sxx - syy).powi(2) + (syy - szz).powi(2) + (szz - sxx).powi(2);
    let shear = sxy * sxy + syz * syz + sxz * sxz;
    (0.5 * dev + 3.0 * shear).sqrt()
}

// ─────────────────────────────────────────────────────────────────────────
//  주파수 인덱스 → 물리 파수 (m2_lub.rs 패턴 재사용)
// ─────────────────────────────────────────────────────────────────────────

/// FFT 인덱스 `a`(0..n) → 부호 파수 `k = 2π·m/L` [1/m]. (m: [−n/2, n/2) 로 접힘)
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
//  bin 별 6성분 복소 스펙트럼 = H_normal·P̂ + H_tangential·Q̂
// ─────────────────────────────────────────────────────────────────────────

/// 6성분 복소 응력 스펙트럼(한 bin, 한 깊이).
#[derive(Clone, Copy)]
struct Spec6 {
    xx: Complex<f64>,
    yy: Complex<f64>,
    zz: Complex<f64>,
    xy: Complex<f64>,
    yz: Complex<f64>,
    xz: Complex<f64>,
}

/// 한 Fourier bin (파수 ka,kb)·깊이 z 에서 6성분 응력 스펙트럼을 계산.
///
/// 식[10](법선, `phat`)·식[16](접선, `qhat`) 의 복소 FRF 를 선형 결합한다.
/// `x_nyq`/`y_nyq`: 짝수격자 Nyquist bin 여부(부호 소실 → 홀수패리티 전달함수 마스킹).
#[allow(clippy::too_many_arguments)]
fn stress_spectrum(
    ka: f64,
    kb: f64,
    z: f64,
    nu: f64,
    phat: Complex<f64>,
    qhat: Complex<f64>,
    x_nyq: bool,
    y_nyq: bool,
) -> Spec6 {
    let zeta = (ka * ka + kb * kb).sqrt();

    // ── DC (ζ=0): 균일 표면하중의 참 응답(고전 탄성) ──
    // 법선 균일하중 → **구속(오이도미터) 상태**: 깊이·횡 무변화라 ε_xx=ε_yy=0 →
    // Hooke `ε_xx=(σ_xx−ν(σ_yy+σ_zz))/E=0`·대칭(σ_xx=σ_yy) → `σ_xx=σ_yy=ν/(1−ν)·σ_zz`,
    // `σ_zz=p_DC`(평형, 깊이불변). 접선 균일하중 → `τ_xz=q_DC`. 나머지 0.
    // (RQ-M3-DC 잔여: 접선 DC 의 완전형·전 평균분리(P2-3 §3.3)는 상단 참조. 법선 측방항은 본 교정으로 해소.)
    if zeta <= 0.0 {
        let z0 = Complex::new(0.0, 0.0);
        let lat = if nu < 1.0 { nu / (1.0 - nu) } else { 0.0 }; // ν/(1−ν)
        return Spec6 {
            xx: phat * Complex::new(lat, 0.0), // σ_xx(DC) = ν/(1−ν)·σ_zz
            yy: phat * Complex::new(lat, 0.0), // σ_yy(DC) = ν/(1−ν)·σ_zz
            zz: phat,                          // σ_zz(DC) = p_DC
            xy: z0,
            yz: z0,
            xz: qhat, // τ_xz(DC) = q_DC
        };
    }

    let e = (-zeta * z).exp();
    let inv = 1.0 / zeta;
    let inv2 = inv * inv;
    let a2 = ka * ka;
    let b2 = kb * kb;

    // 법선(P̂) 전달함수 — 식[10].
    let n_xx = (a2 * inv2 - a2 * z * inv + 2.0 * nu * b2 * inv2) * e; // 실
    let n_yy = (b2 * inv2 - b2 * z * inv + 2.0 * nu * a2 * inv2) * e; // 실
    let n_zz = (1.0 + zeta * z) * e; // 실
    let mut n_xy = (ka * kb * inv2) * ((1.0 - 2.0 * nu) - zeta * z) * e; // 실 (홀수 in a·b)
    let mut n_yz_im = -kb * z * e; // 허 (홀수 in b)
    let mut n_xz_im = -ka * z * e; // 허 (홀수 in a)

    // 접선(Q̂) 전달함수 — 식[16].
    let mut t_xx_im = -(ka * inv) * (2.0 + 2.0 * nu * b2 * inv2 - a2 * z * inv) * e; // 허 (홀수 in a)
    let mut t_yy_im = -(ka * inv) * (2.0 * nu * a2 * inv2 - b2 * z * inv) * e; // 허 (홀수 in a)
    let mut t_zz_im = -ka * z * e; // 허 (홀수 in a)
    let mut t_xy_im = -(kb * inv) * (1.0 - 2.0 * nu * a2 * inv2 - a2 * z * inv) * e; // 허 (홀수 in b)
    let mut t_yz = -(ka * kb * z * inv) * e; // 실 (홀수 in a·b)
    let t_xz = (1.0 - a2 * z * inv) * e; // 실 (짝수)

    // ── Nyquist 마스킹 (짝수격자): 접힌 방향의 부호가 소실되므로 그 방향에 **홀수**인
    //    전달함수를 0 으로 투영한다. 격자점에서 sin(k_nyq·x)≡0 이라 물리기여 0 이며(관측
    //    불가), 이 마스킹으로 스펙트럼의 Hermitian 성을 보존해 IFFT 실수성/정확성을 확보한다.
    //    (m2_lub.rs x-Nyquist 실수부투영의 일반화 — 성분별 패리티 반영.) ──
    if x_nyq {
        n_xz_im = 0.0; // 홀수 in a
        n_xy = 0.0; // 홀수 in a
        t_xx_im = 0.0;
        t_yy_im = 0.0;
        t_zz_im = 0.0;
        t_yz = 0.0;
    }
    if y_nyq {
        n_yz_im = 0.0; // 홀수 in b
        n_xy = 0.0; // 홀수 in b
        t_xy_im = 0.0;
        t_yz = 0.0;
    }

    Spec6 {
        xx: phat * n_xx + qhat * Complex::new(0.0, t_xx_im),
        yy: phat * n_yy + qhat * Complex::new(0.0, t_yy_im),
        zz: phat * n_zz + qhat * Complex::new(0.0, t_zz_im),
        xy: phat * n_xy + qhat * Complex::new(0.0, t_xy_im),
        yz: phat * Complex::new(0.0, n_yz_im) + qhat * t_yz,
        xz: phat * Complex::new(0.0, n_xz_im) + qhat * t_xz,
    }
}

// ─────────────────────────────────────────────────────────────────────────
//  핵심 진입점 (임의 깊이 배열)
// ─────────────────────────────────────────────────────────────────────────

/// 임의 깊이배열 `z_depths`[m] 에서 표면하 6응력장 + von Mises 를 계산.
///
/// `p`(법선)·`q`(접선) 각 1회 2D-FFT → bin·깊이별 식[10]/[16] FRF 적용 → 6성분 IFFT 중첩.
/// [`solve_stress`] 가 `z=0~0.25b·15층` 으로 이 함수를 호출한다(오라클은 깊은 범위 사용).
pub fn solve_stress_at_depths(
    grid: &Grid,
    p: &Field2,
    q: &Field2,
    nu: f64,
    z_depths: &[f64],
) -> StressResult {
    let nx = grid.nx;
    let ny = grid.ny;
    let n = nx * ny;
    let nz = z_depths.len();

    // 퇴화 방어.
    if nx == 0 || ny == 0 || n == 0 {
        return StressResult {
            z: z_depths.to_vec(),
            stress: (0..nz)
                .map(|_| StressTensor6 {
                    sxx: Field2::zeros(nx, ny),
                    syy: Field2::zeros(nx, ny),
                    szz: Field2::zeros(nx, ny),
                    sxy: Field2::zeros(nx, ny),
                    syz: Field2::zeros(nx, ny),
                    sxz: Field2::zeros(nx, ny),
                })
                .collect(),
            von_mises: (0..nz).map(|_| Field2::zeros(nx, ny)).collect(),
        };
    }

    // 하중장 forward-FFT (1회).
    let mut pf: Vec<Complex<f64>> = p.data.iter().map(|&v| Complex::new(v, 0.0)).collect();
    let mut qf: Vec<Complex<f64>> = q.data.iter().map(|&v| Complex::new(v, 0.0)).collect();
    fft2_forward(&mut pf, nx, ny);
    fft2_forward(&mut qf, nx, ny);

    // 파수 사전계산.
    let kxs: Vec<f64> = (0..nx).map(|a| wavenumber(a, nx, grid.lx)).collect();
    let kys: Vec<f64> = (0..ny).map(|b| wavenumber(b, ny, grid.ly)).collect();
    let x_nyq_col = if nx % 2 == 0 { Some(nx / 2) } else { None };
    let y_nyq_row = if ny % 2 == 0 { Some(ny / 2) } else { None };

    let mut stress = Vec::with_capacity(nz);
    let mut vms = Vec::with_capacity(nz);

    // 재사용 스펙트럼 버퍼(성분별).
    let (mut sxx, mut syy, mut szz) = (
        vec![Complex::new(0.0, 0.0); n],
        vec![Complex::new(0.0, 0.0); n],
        vec![Complex::new(0.0, 0.0); n],
    );
    let (mut sxy, mut syz, mut sxz) = (
        vec![Complex::new(0.0, 0.0); n],
        vec![Complex::new(0.0, 0.0); n],
        vec![Complex::new(0.0, 0.0); n],
    );

    for &z in z_depths {
        for ib in 0..ny {
            let kb = kys[ib];
            let y_nyq = y_nyq_row == Some(ib);
            for ia in 0..nx {
                let ka = kxs[ia];
                let idx = ia + ib * nx;
                let x_nyq = x_nyq_col == Some(ia);
                let s = stress_spectrum(ka, kb, z, nu, pf[idx], qf[idx], x_nyq, y_nyq);
                sxx[idx] = s.xx;
                syy[idx] = s.yy;
                szz[idx] = s.zz;
                sxy[idx] = s.xy;
                syz[idx] = s.yz;
                sxz[idx] = s.xz;
            }
        }

        // 성분별 역FFT → 실공간 응력장.
        fft2_inverse(&mut sxx, nx, ny);
        fft2_inverse(&mut syy, nx, ny);
        fft2_inverse(&mut szz, nx, ny);
        fft2_inverse(&mut sxy, nx, ny);
        fft2_inverse(&mut syz, nx, ny);
        fft2_inverse(&mut sxz, nx, ny);

        let mut fxx = Field2::zeros(nx, ny);
        let mut fyy = Field2::zeros(nx, ny);
        let mut fzz = Field2::zeros(nx, ny);
        let mut fxy = Field2::zeros(nx, ny);
        let mut fyz = Field2::zeros(nx, ny);
        let mut fxz = Field2::zeros(nx, ny);
        let mut fvm = Field2::zeros(nx, ny);
        for k in 0..n {
            // 실수성: Hermitian 스펙트럼 → Re(IFFT) 가 물리장(허수부는 수치오차/Nyquist 잔여).
            let (a, b, c) = (sxx[k].re, syy[k].re, szz[k].re);
            let (d, e, f) = (sxy[k].re, syz[k].re, sxz[k].re);
            fxx.data[k] = a;
            fyy.data[k] = b;
            fzz.data[k] = c;
            fxy.data[k] = d;
            fyz.data[k] = e;
            fxz.data[k] = f;
            fvm.data[k] = von_mises(a, b, c, d, e, f);
        }

        stress.push(StressTensor6 {
            sxx: fxx,
            syy: fyy,
            szz: fzz,
            sxy: fxy,
            syz: fyz,
            sxz: fxz,
        });
        vms.push(fvm);
    }

    StressResult {
        z: z_depths.to_vec(),
        stress,
        von_mises: vms,
    }
}

// ─────────────────────────────────────────────────────────────────────────
//  주 진입점: z=0~0.25b, 15층
// ─────────────────────────────────────────────────────────────────────────

/// Hertz 접촉반폭 `b = 2·r_x·p_h/e_red` [m] (선접촉; E'=2·E_red 치환, types.rs `r_x` 주석).
///
/// 비물리 입력(≤0)은 0 반환(사용처 방어).
#[inline]
pub fn contact_half_width(op: &OperatingConditions, mat: &MaterialProps) -> f64 {
    if op.r_x <= 0.0 || op.p_h <= 0.0 || mat.e_red <= 0.0 {
        return 0.0;
    }
    2.0 * op.r_x * op.p_h / mat.e_red
}

/// 기본 깊이배열 `z_l = DEPTH_FRAC·b·l/(nz−1)`, l=0..nz−1 (등간격, 원문 정합).
fn default_depths(b: f64) -> Vec<f64> {
    let zmax = DEPTH_FRAC * b;
    (0..NZ_DEFAULT)
        .map(|l| zmax * l as f64 / (NZ_DEFAULT - 1) as f64)
        .collect()
}

/// M3 주 진입점 — `z=0~0.25b`, 15층 표면하 응력장 + von Mises.
///
/// `b = 2·r_x·p_h/e_red` (Hertz 선접촉). `b≤0`(비물리 r_x/p_h/e_red)이면 모든 층을 z=0
/// 으로 두어 안전 반환(퇴화). 깊은 범위가 필요한 검증(매끈 Hertz 최대 vM 는 z≈0.7b)은
/// [`solve_stress_at_depths`] 를 직접 호출한다.
pub fn solve_stress(input: &StressInput) -> StressResult {
    let b = contact_half_width(&input.op, &input.mat);
    let depths = if b > 0.0 {
        default_depths(b)
    } else {
        vec![0.0; NZ_DEFAULT] // 퇴화: b 미정의 → 표면층만(사용처 방어)
    };
    solve_stress_at_depths(
        &input.grid,
        &input.p_tran,
        &input.q_tran,
        input.mat.nu,
        &depths,
    )
}

// ═════════════════════════════════════════════════════════════════════════
//  오라클 (독립·비-tautology·손유도 하드코딩)
// ═════════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{NU_STEEL, E_RED_STEEL_PA};

    // ── 공용: 정수주기 bi-sinusoid 압력/트랙션장 생성 ──
    fn cos_cos_field(nx: usize, ny: usize, lx: f64, ly: f64, mx: f64, my: f64, amp: f64) -> Field2 {
        let alpha = 2.0 * PI * mx / lx;
        let beta = 2.0 * PI * my / ly;
        let mut f = Field2::zeros(nx, ny);
        for j in 0..ny {
            let y = j as f64 * (ly / ny as f64);
            for i in 0..nx {
                let x = i as f64 * (lx / nx as f64);
                f.set(i, j, amp * (alpha * x).cos() * (beta * y).cos());
            }
        }
        f
    }

    fn dummy_mat() -> MaterialProps {
        MaterialProps {
            e_red: E_RED_STEEL_PA,
            nu: NU_STEEL,
            hardness: 7e9,
            p_lim: 4e9,
        }
    }

    fn dummy_op() -> OperatingConditions {
        OperatingConditions {
            p_h: 1.5e9,
            u_mean: 1.0,
            u2: 0.95,
            slide_roll: 0.1,
            eta0: 0.01,
            alpha_visc: 2e-8,
            tau0: 5e6,
            temp: 353.0,
            r_x: 0.02,
        }
    }

    // ── VC-M3-Sin(1): 단일 정현파 → 6응력 재구성이 식[10]/[16] 손유도 폐형식과 일치 ──
    //
    // 검증함수로 기대값 생성 금지 → 원문 식[10]/[16] 실수 cos/sin 폐형식을 **직접 전사**해
    // 격자점에서 평가, 솔버 재구성장과 대조. (전달함수는 이 폐형식에서 소급했으나, 테스트는
    // 복소 FRF 를 쓰지 않고 실수식만 쓰므로 복소↔실 변환·FFT 정규화·부호를 독립 검증.)
    #[test]
    fn vc_m3_sin_normal() {
        let (nx, ny) = (32usize, 16usize);
        let (lx, ly) = (1.0e-4, 1.0e-4);
        let (mx, my) = (3.0, 2.0);
        let p0 = 1.0e9;
        let alpha = 2.0 * PI * mx / lx;
        let beta = 2.0 * PI * my / ly;
        let zeta = (alpha * alpha + beta * beta).sqrt();
        let nu = 0.3;

        let p = cos_cos_field(nx, ny, lx, ly, mx, my, p0);
        let q = Field2::zeros(nx, ny);
        let grid = Grid::new(nx, ny, lx, ly);
        let z_depths = [0.0, 0.15 / zeta, 0.4 / zeta, 0.9 / zeta];
        let res = solve_stress_at_depths(&grid, &p, &q, nu, &z_depths);

        for (l, &z) in z_depths.iter().enumerate() {
            let e = (-zeta * z).exp();
            let st = &res.stress[l];
            for j in 0..ny {
                let y = j as f64 * (ly / ny as f64);
                for i in 0..nx {
                    let x = i as f64 * (lx / nx as f64);
                    let cc = (alpha * x).cos() * (beta * y).cos();
                    let ss = (alpha * x).sin() * (beta * y).sin();
                    let cs = (alpha * x).cos() * (beta * y).sin();
                    let sc = (alpha * x).sin() * (beta * y).cos();
                    // 식[10] verbatim
                    let ex_xx = p0
                        * (alpha * alpha / (zeta * zeta) - alpha * alpha * z / zeta
                            + 2.0 * nu * beta * beta / (zeta * zeta))
                        * e
                        * cc;
                    let ex_yy = p0
                        * (beta * beta / (zeta * zeta) - beta * beta * z / zeta
                            + 2.0 * nu * alpha * alpha / (zeta * zeta))
                        * e
                        * cc;
                    let ex_zz = p0 * (1.0 + zeta * z) * e * cc;
                    let ex_xy = -p0 * (alpha * beta / (zeta * zeta)) * ((1.0 - 2.0 * nu) - zeta * z)
                        * e
                        * ss;
                    let ex_yz = p0 * (beta * z) * e * cs;
                    let ex_xz = p0 * (alpha * z) * e * sc;
                    let tol = p0 * 1e-6;
                    assert!((st.sxx.at(i, j) - ex_xx).abs() < tol, "sxx l={l} i={i} j={j}");
                    assert!((st.syy.at(i, j) - ex_yy).abs() < tol, "syy l={l} i={i} j={j}");
                    assert!((st.szz.at(i, j) - ex_zz).abs() < tol, "szz l={l} i={i} j={j}");
                    assert!((st.sxy.at(i, j) - ex_xy).abs() < tol, "sxy l={l} i={i} j={j}");
                    assert!((st.syz.at(i, j) - ex_yz).abs() < tol, "syz l={l} i={i} j={j}");
                    assert!((st.sxz.at(i, j) - ex_xz).abs() < tol, "sxz l={l} i={i} j={j}");
                }
            }
        }
    }

    #[test]
    fn vc_m3_sin_tangential() {
        let (nx, ny) = (32usize, 16usize);
        let (lx, ly) = (1.0e-4, 1.0e-4);
        let (mx, my) = (3.0, 2.0);
        let q0 = 1.0e9;
        let alpha = 2.0 * PI * mx / lx;
        let beta = 2.0 * PI * my / ly;
        let zeta = (alpha * alpha + beta * beta).sqrt();
        let nu = 0.3;

        let q = cos_cos_field(nx, ny, lx, ly, mx, my, q0);
        let p = Field2::zeros(nx, ny);
        let grid = Grid::new(nx, ny, lx, ly);
        let z_depths = [0.0, 0.15 / zeta, 0.4 / zeta, 0.9 / zeta];
        let res = solve_stress_at_depths(&grid, &p, &q, nu, &z_depths);

        let (aoz, boz) = (alpha / zeta, beta / zeta);
        for (l, &z) in z_depths.iter().enumerate() {
            let e = (-zeta * z).exp();
            let st = &res.stress[l];
            for j in 0..ny {
                let y = j as f64 * (ly / ny as f64);
                for i in 0..nx {
                    let x = i as f64 * (lx / nx as f64);
                    let cc = (alpha * x).cos() * (beta * y).cos();
                    let ss = (alpha * x).sin() * (beta * y).sin();
                    let cs = (alpha * x).cos() * (beta * y).sin();
                    let sc = (alpha * x).sin() * (beta * y).cos();
                    // 식[16] verbatim
                    let ex_xx = q0 * aoz
                        * (2.0 + 2.0 * nu * boz * boz - aoz * (alpha * z))
                        * e
                        * sc;
                    let ex_yy =
                        q0 * aoz * (2.0 * nu * aoz * aoz - boz * (beta * z)) * e * sc;
                    let ex_zz = q0 * (alpha * z) * e * sc;
                    let ex_xy = q0 * boz
                        * (1.0 - 2.0 * nu * aoz * aoz - aoz * (alpha * z))
                        * e
                        * cs;
                    let ex_yz = q0 * boz * (alpha * z) * e * ss;
                    let ex_xz = q0 * (1.0 - aoz * (alpha * z)) * e * cc;
                    let tol = q0 * 1e-6;
                    assert!((st.sxx.at(i, j) - ex_xx).abs() < tol, "sxx l={l} i={i} j={j}");
                    assert!((st.syy.at(i, j) - ex_yy).abs() < tol, "syy l={l} i={i} j={j}");
                    assert!((st.szz.at(i, j) - ex_zz).abs() < tol, "szz l={l} i={i} j={j}");
                    assert!((st.sxy.at(i, j) - ex_xy).abs() < tol, "sxy l={l} i={i} j={j}");
                    assert!((st.syz.at(i, j) - ex_yz).abs() < tol, "syz l={l} i={i} j={j}");
                    assert!((st.sxz.at(i, j) - ex_xz).abs() < tol, "sxz l={l} i={i} j={j}");
                }
            }
        }
    }

    // ── VC-M3-Trace(2, 핵심 정본판별): 원문서 항등 손유도 후 재구성장 만족 확인 ──
    //
    // 식[10] 합: σx+σy+σz = 2(1+ν)p0·e·cos·cos  (P2-1 line 116; 2011 손상식 불성립).
    // 식[16] 합: σx+σy+σz = 2(1+ν)q0(α/ζ)·e·sin·cos  (P2-1 line 117).
    // (손유도: 식[10] 대괄호합 = (α²+β²)/ζ²−(α²+β²)z/ζ+2ν(α²+β²)/ζ² + (1+ζz)
    //         = 1 − ζz + 2ν + 1 + ζz = 2(1+ν).)
    #[test]
    fn vc_m3_trace_normal() {
        let (nx, ny) = (24usize, 12usize);
        let (lx, ly) = (1.2e-4, 0.8e-4);
        let (mx, my) = (2.0, 3.0);
        let p0 = 1.3e9;
        let alpha = 2.0 * PI * mx / lx;
        let beta = 2.0 * PI * my / ly;
        let zeta = (alpha * alpha + beta * beta).sqrt();
        let nu = 0.28;
        let p = cos_cos_field(nx, ny, lx, ly, mx, my, p0);
        let q = Field2::zeros(nx, ny);
        let grid = Grid::new(nx, ny, lx, ly);
        let z_depths = [0.05 / zeta, 0.5 / zeta, 1.2 / zeta];
        let res = solve_stress_at_depths(&grid, &p, &q, nu, &z_depths);
        for (l, &z) in z_depths.iter().enumerate() {
            let e = (-zeta * z).exp();
            let st = &res.stress[l];
            for j in 0..ny {
                let y = j as f64 * (ly / ny as f64);
                for i in 0..nx {
                    let x = i as f64 * (lx / nx as f64);
                    let cc = (alpha * x).cos() * (beta * y).cos();
                    let trace = st.sxx.at(i, j) + st.syy.at(i, j) + st.szz.at(i, j);
                    let expect = 2.0 * (1.0 + nu) * p0 * e * cc;
                    assert!(
                        (trace - expect).abs() < p0 * 1e-6,
                        "trace(normal) l={l} i={i} j={j}: {trace} vs {expect}"
                    );
                }
            }
        }
    }

    #[test]
    fn vc_m3_trace_tangential() {
        let (nx, ny) = (24usize, 12usize);
        let (lx, ly) = (1.2e-4, 0.8e-4);
        let (mx, my) = (2.0, 3.0);
        let q0 = 1.3e9;
        let alpha = 2.0 * PI * mx / lx;
        let beta = 2.0 * PI * my / ly;
        let zeta = (alpha * alpha + beta * beta).sqrt();
        let nu = 0.28;
        let q = cos_cos_field(nx, ny, lx, ly, mx, my, q0);
        let p = Field2::zeros(nx, ny);
        let grid = Grid::new(nx, ny, lx, ly);
        let z_depths = [0.05 / zeta, 0.5 / zeta, 1.2 / zeta];
        let res = solve_stress_at_depths(&grid, &p, &q, nu, &z_depths);
        for (l, &z) in z_depths.iter().enumerate() {
            let e = (-zeta * z).exp();
            let st = &res.stress[l];
            for j in 0..ny {
                let y = j as f64 * (ly / ny as f64);
                for i in 0..nx {
                    let x = i as f64 * (lx / nx as f64);
                    let sc = (alpha * x).sin() * (beta * y).cos();
                    let trace = st.sxx.at(i, j) + st.syy.at(i, j) + st.szz.at(i, j);
                    let expect = 2.0 * (1.0 + nu) * q0 * (alpha / zeta) * e * sc;
                    assert!(
                        (trace - expect).abs() < q0 * 1e-6,
                        "trace(tangential) l={l} i={i} j={j}: {trace} vs {expect}"
                    );
                }
            }
        }
    }

    // ── VC-M3-Limit(3): β=0 → 1D(식[4]), τ_xy·τ_yz→0; z=0 경계조건 ──
    #[test]
    fn vc_m3_limit_beta0() {
        let (nx, ny) = (32usize, 8usize);
        let (lx, ly) = (1.0e-4, 1.0e-4);
        let p0 = 1.0e9;
        let alpha = 2.0 * PI * 3.0 / lx; // β=0 (my=0)
        let nu = 0.3;
        let p = cos_cos_field(nx, ny, lx, ly, 3.0, 0.0, p0); // y-불변
        let q = Field2::zeros(nx, ny);
        let grid = Grid::new(nx, ny, lx, ly);
        let z_depths = [0.0, 0.3 / alpha, 0.8 / alpha];
        let res = solve_stress_at_depths(&grid, &p, &q, nu, &z_depths);
        for (l, &z) in z_depths.iter().enumerate() {
            let e = (-alpha * z).exp();
            let st = &res.stress[l];
            for j in 0..ny {
                for i in 0..nx {
                    let x = i as f64 * (lx / nx as f64);
                    let c = (alpha * x).cos();
                    let s = (alpha * x).sin();
                    // τ_xy, τ_yz → 0
                    assert!(st.sxy.at(i, j).abs() < p0 * 1e-9, "sxy!=0 β=0");
                    assert!(st.syz.at(i, j).abs() < p0 * 1e-9, "syz!=0 β=0");
                    // 식[4] 1D 법선
                    let ex_xx = p0 * (1.0 - alpha * z) * e * c;
                    let ex_yy = 2.0 * nu * p0 * e * c;
                    let ex_zz = p0 * (1.0 + alpha * z) * e * c;
                    let ex_xz = p0 * (alpha * z) * e * s;
                    assert!((st.sxx.at(i, j) - ex_xx).abs() < p0 * 1e-6, "sxx eq4 l={l}");
                    assert!((st.syy.at(i, j) - ex_yy).abs() < p0 * 1e-6, "syy eq4 l={l}");
                    assert!((st.szz.at(i, j) - ex_zz).abs() < p0 * 1e-6, "szz eq4 l={l}");
                    assert!((st.sxz.at(i, j) - ex_xz).abs() < p0 * 1e-6, "sxz eq4 l={l}");
                }
            }
        }
    }

    #[test]
    fn vc_m3_boundary_z0() {
        // z=0 경계: 법선 → σ_zz=p, τ_xz=0; 접선 → τ_xz=q, σ_zz=0.
        let (nx, ny) = (16usize, 16usize);
        let (lx, ly) = (1.0e-4, 1.0e-4);
        let amp = 1.0e9;
        let nu = 0.3;
        let grid = Grid::new(nx, ny, lx, ly);
        let field = cos_cos_field(nx, ny, lx, ly, 3.0, 2.0, amp);
        let zero = Field2::zeros(nx, ny);

        // 법선
        let rn = solve_stress_at_depths(&grid, &field, &zero, nu, &[0.0]);
        for k in 0..nx * ny {
            assert!(
                (rn.stress[0].szz.data[k] - field.data[k]).abs() < amp * 1e-9,
                "z=0 normal σ_zz != p"
            );
            assert!(rn.stress[0].sxz.data[k].abs() < amp * 1e-9, "z=0 normal τ_xz != 0");
        }
        // 접선
        let rt = solve_stress_at_depths(&grid, &zero, &field, nu, &[0.0]);
        for k in 0..nx * ny {
            assert!(
                (rt.stress[0].sxz.data[k] - field.data[k]).abs() < amp * 1e-9,
                "z=0 tangential τ_xz != q"
            );
            assert!(rt.stress[0].szz.data[k].abs() < amp * 1e-9, "z=0 tangential σ_zz != 0");
        }
    }

    // ── VC-M3-Hertz(4, 외부앵커): 매끈 Hertz 선접촉 → vM 최대 ≈0.557·p_h @ z≈0.70b ──
    //
    // 독립 기준 = McEwen 선접촉 z-축 폐형식(원문 아님·고전 탄성해; τ_max=0.30p0@0.78b 로 교차확인):
    //   σ_x/p0 = −[(1+2ζ²)/√(1+ζ²) − 2ζ],  σ_z/p0 = −1/√(1+ζ²),  σ_y=ν(σ_x+σ_z) (평면변형)
    //   → σ_vM/p0 최대 ≈0.5575 @ ζ=z/b≈0.70 (ν=0.3, 손계산).
    // 3D-FFT 솔버에 y-불변 압력 → ky=0 모드만 → 식[10] 평면변형(σ_y=ν(σx+σz)) 재현.
    #[test]
    fn vc_m3_hertz_line_von_mises() {
        let nu = 0.3;
        let p_h = 1.0e9;
        let b = 1.0e-4; // 접촉반폭 100 μm (규모만 중요; 무차원 결과)
        // 창 크기 = 편향의 지배원. periodic-window 평균(DC) 아티팩트 ∝ 1/창 → 창을 키워
        // 고립 Hertz 로 수렴시킨다(측정: 32b→+3.4%, 64b→+1.8%, 128b→+0.93%, 256b→+0.49%;
        // dx 세분은 무영향 → 편향은 순수 창 효과). 256b·nx8192(dx=b/32)로 <1% 달성.
        let nx = 8192usize;
        let ny = 4usize; // y-불변(횡방향 균일) → 최소 격자로 비용 절감(결과 불변)
        let lx = 256.0 * b; // 큰 창(±128b) + zero-pad → 주기성/DC 아티팩트 <1% (수렴 실측)
        let ly = 4.0 * b;
        let grid = Grid::new(nx, ny, lx, ly);
        let dx = lx / nx as f64;
        let x0 = 0.5 * lx; // bump 중심 = 격자 중앙열
        let i0 = nx / 2;

        // Hertz 선접촉 압력 p(x)=p_h·√(1−((x−x0)/b)²), |x−x0|<b, 외부 0 (창·zero-pad).
        let mut p = Field2::zeros(nx, ny);
        for j in 0..ny {
            for i in 0..nx {
                let xr = (i as f64 * dx - x0) / b;
                let v = if xr.abs() < 1.0 {
                    p_h * (1.0 - xr * xr).sqrt()
                } else {
                    0.0
                };
                p.set(i, j, v);
            }
        }
        let q = Field2::zeros(nx, ny);

        // 깊은 z 범위(피크 z≈0.7b 부근 집중) — 주 진입점(0.25b)이 아닌 커스텀 배열. 피크가
        // 견고히 0.70b 이므로 [0.4,1.0]b 를 조밀 탐색(비용 절감; 깊이 assertion [0.55,0.90] 포함).
        let nz = 31usize;
        let z_depths: Vec<f64> =
            (0..nz).map(|l| b * (0.4 + 0.6 * l as f64 / (nz - 1) as f64)).collect();
        let res = solve_stress_at_depths(&grid, &p, &q, nu, &z_depths);

        // 솔버 on-axis(중심열 i0) vM 최대·깊이.
        let mut vm_max = 0.0;
        let mut z_at = 0.0;
        for l in 0..nz {
            let vm = res.von_mises[l].at(i0, 0);
            if vm > vm_max {
                vm_max = vm;
                z_at = z_depths[l];
            }
        }
        let vm_max_norm = vm_max / p_h;
        let z_norm = z_at / b;

        // McEwen 독립 폐형식 최대(고전값).
        let mcewen_vm = |zeta: f64| -> f64 {
            let m = (1.0 + zeta * zeta).sqrt();
            let sx = -((1.0 + 2.0 * zeta * zeta) / m - 2.0 * zeta);
            let sz = -1.0 / m;
            let sy = nu * (sx + sz);
            (0.5 * ((sx - sy).powi(2) + (sy - sz).powi(2) + (sz - sx).powi(2))).sqrt()
        };
        let mut mc_max = 0.0;
        let mut mc_z = 0.0;
        {
            let mut z = 0.01;
            while z < 1.2 {
                let v = mcewen_vm(z);
                if v > mc_max {
                    mc_max = v;
                    mc_z = z;
                }
                z += 0.001;
            }
        }
        // 고전값 자체 sanity (≈0.557 @ ≈0.70).
        assert!((mc_max - 0.557).abs() < 0.01, "McEwen ref off: {mc_max}");
        assert!((mc_z - 0.70).abs() < 0.06, "McEwen depth off: {mc_z}");

        // 솔버 vs 고전값: 256b 창서 실측 vm_max_norm≈0.5602 = 고전값의 **+0.49%**(<1%).
        // DC 측방 교정(σ_xx=σ_yy=ν/(1−ν)σ_zz) + 대형창(창→고립 수렴, ∝1/창)의 결합으로
        // periodic-window 평균 아티팩트를 <1% 로 억제. 허용 ≤1%(실측 0.49%), 깊이 [0.55,0.90]b.
        assert!(
            (vm_max_norm - mc_max).abs() <= 0.01 * mc_max,
            "Hertz vM peak {vm_max_norm} vs classical {mc_max} (>1%)"
        );
        assert!(
            (0.55..=0.90).contains(&z_norm),
            "Hertz vM peak depth z/b={z_norm} out of [0.55,0.90]"
        );

        // 판별력: 평면변형 σ_y 누락 시 vM≈0.30(=τ_max·√3 근방)로 떨어짐 → 0.5 하한이 포착.
        assert!(vm_max_norm > 0.50, "vM peak too low (plane-strain σ_y missing?)");
    }

    // ── RP-Depth(4, 정량): 거칠기 리플 → 최대 vM 깊이 마이크로미터대(원논문 order 5μm) ──
    //
    // 단일 파장 λ 리플 법선압 → 최대 vM 은 전단 τ_xz∝(αz)e^{−αz} 이 지배해 z≈1/α=λ/2π
    // 에서 발생(식[4] 손계산: √3·p0·αz·e^{−αz} 최대 @ αz=1). λ=40μm → z≈6.4μm.
    #[test]
    fn rp_depth_micron_scale() {
        let nu = 0.3;
        let lam = 40.0e-6; // 40 μm 거칠기 파장
        let k = 2.0 * PI / lam;
        let p0 = 0.2e9; // 리플 진폭
        let periods = 8usize;
        let lx = periods as f64 * lam;
        let nx = 256usize;
        let ny = 8usize;
        let ly = lx;
        let grid = Grid::new(nx, ny, lx, ly);

        // 순수 리플(평균 0) — DC 균일 offset 배제해 깊이피크 판별력 확보.
        let mut p = Field2::zeros(nx, ny);
        for j in 0..ny {
            for i in 0..nx {
                let x = i as f64 * (lx / nx as f64);
                p.set(i, j, p0 * (k * x).cos());
            }
        }
        let q = Field2::zeros(nx, ny);

        // 미세 z 배열(0~30μm) 로 마이크론 피크 해상.
        let nz = 60usize;
        let z_depths: Vec<f64> = (0..nz).map(|l| 30.0e-6 * l as f64 / (nz - 1) as f64).collect();
        let res = solve_stress_at_depths(&grid, &p, &q, nu, &z_depths);

        // 전역 최대 vM 깊이.
        let mut vm_max = 0.0;
        let mut z_at = 0.0;
        for l in 0..nz {
            let m = res.von_mises[l].max().unwrap();
            if m > vm_max {
                vm_max = m;
                z_at = z_depths[l];
            }
        }
        // 마이크론대·λ/2π(≈6.4μm) 근방.
        assert!(vm_max > 0.0, "vM zero");
        assert!(
            (2.0e-6..=12.0e-6).contains(&z_at),
            "max vM depth {z_at} m not micron-scale (expect ~λ/2π≈6.4μm)"
        );
    }

    // ── DC 처리: 균일 압력 → σ_zz=p_DC(전깊이), 타성분 0 (RQ-M3-DC 채택안 검증) ──
    #[test]
    fn dc_uniform_pressure() {
        let (nx, ny) = (16usize, 16usize);
        let grid = Grid::new(nx, ny, 1e-4, 1e-4);
        let p = Field2::filled(nx, ny, 5.0e8); // 균일
        let q = Field2::zeros(nx, ny);
        let res = solve_stress_at_depths(&grid, &p, &q, 0.3, &[0.0, 1e-5, 5e-5]);
        for l in 0..3 {
            let st = &res.stress[l];
            let lat = 0.3 / 0.7 * 5.0e8; // ν/(1−ν)·σ_zz (구속 균일하중, ν=0.3)
            for k in 0..nx * ny {
                assert!((st.szz.data[k] - 5.0e8).abs() < 1.0, "σ_zz DC != p");
                assert!((st.sxx.data[k] - lat).abs() < 1.0, "σ_xx DC != ν/(1−ν)p");
                assert!((st.syy.data[k] - lat).abs() < 1.0, "σ_yy DC != ν/(1−ν)p");
            }
        }
    }

    // ── 접선 DC 검증(RQ-M3-DC 접선 완전형): 균일 트랙션 q → **단순전단** σ_xz=q(전 깊이),
    //    나머지 5성분 0, vM=√3·q. (균일 표면전단 → 반공간 단순전단; 평형·적합성·BC
    //    (σ_zz(0)=0·τ_xz(0)=q) 만족 유일 응력해. 변위 u_x∝z 발산이나 응력은 잘 정의.
    //    dc_uniform_pressure(법선 구속하중) 의 접선 대응 — 접선 DC 미검증 gap 폐색.) ──
    #[test]
    fn dc_uniform_traction() {
        let (nx, ny) = (16usize, 16usize);
        let grid = Grid::new(nx, ny, 1e-4, 1e-4);
        let p = Field2::zeros(nx, ny);
        let q = Field2::filled(nx, ny, 4.0e8); // 균일 접선 트랙션
        let res = solve_stress_at_depths(&grid, &p, &q, 0.3, &[0.0, 1e-5, 5e-5]);
        for l in 0..3 {
            let st = &res.stress[l];
            for k in 0..nx * ny {
                assert!((st.sxz.data[k] - 4.0e8).abs() < 1.0, "τ_xz DC != q (단순전단)");
                assert!(st.sxx.data[k].abs() < 1.0, "σ_xx DC != 0");
                assert!(st.syy.data[k].abs() < 1.0, "σ_yy DC != 0");
                assert!(st.szz.data[k].abs() < 1.0, "σ_zz DC != 0");
                assert!(st.sxy.data[k].abs() < 1.0, "τ_xy DC != 0");
                assert!(st.syz.data[k].abs() < 1.0, "τ_yz DC != 0");
            }
            // vM = √3·q (순수전단).
            let vm = res.von_mises[l].at(0, 0);
            assert!((vm - 3f64.sqrt() * 4.0e8).abs() < 1.0, "vM DC != √3·q");
        }
    }

    // ── 주 진입점: b=2·r_x·p_h/e_red, z=0~0.25b·15층 형상 ──
    #[test]
    fn solve_stress_shape_and_depth() {
        let mat = dummy_mat();
        let op = dummy_op();
        let b = contact_half_width(&op, &mat);
        assert!(b > 0.0);
        let (nx, ny) = (16usize, 16usize);
        let input = StressInput {
            grid: Grid::new(nx, ny, 1e-4, 1e-4),
            p_tran: cos_cos_field(nx, ny, 1e-4, 1e-4, 2.0, 2.0, 1e9),
            q_tran: Field2::zeros(nx, ny),
            mat,
            op,
        };
        let res = solve_stress(&input);
        assert_eq!(res.z.len(), NZ_DEFAULT);
        assert_eq!(res.stress.len(), NZ_DEFAULT);
        assert_eq!(res.von_mises.len(), NZ_DEFAULT);
        assert!((res.z[0]).abs() < 1e-18, "z[0]=0");
        assert!(
            (res.z[NZ_DEFAULT - 1] - 0.25 * b).abs() < 0.25 * b * 1e-9,
            "z_max=0.25b"
        );
        assert_eq!(res.von_mises[0].nx, nx);
    }

    // ── von Mises 표준식 sanity: 단축 → |σ|, 순수전단 → √3·τ ──
    #[test]
    fn von_mises_standard() {
        assert!((von_mises(100.0, 0.0, 0.0, 0.0, 0.0, 0.0) - 100.0).abs() < 1e-9);
        assert!((von_mises(0.0, 0.0, 0.0, 50.0, 0.0, 0.0) - 3f64.sqrt() * 50.0).abs() < 1e-9);
        // 정수압 → 0
        assert!(von_mises(30.0, 30.0, 30.0, 0.0, 0.0, 0.0).abs() < 1e-9);
    }
}
