//! 거칠기 부호 규약 프로브 (M1 dry ↔ M2 lub ↔ M6 merge).
//!
//! 목적: `rough` 필드의 부호 해석이 M1(높이: +rough = 돌출 아스페리티 = 간극 감소)과
//! M2(간극 섭동: +rough = 유막 증가)에서 서로 반대인지 **공개 API 만으로** 측정한다.
//! 기존 오라클은 평균 0 정현파(부호 반전 = 위상 이동)와 크기/보존만 검사하므로 이 결함을
//! 볼 수 없다. 여기서는 (1) 일방향(one-sided) 양의 범프, (2) 정현파의 상관 부호로 판정한다.
//!
//! 규칙: 생산코드 무변경. 물리적으로 옳은 단정이 실패하면 `#[ignore]` 로 표시하고 측정값은
//! 비-ignore 특성화 테스트가 `--nocapture` 로 재현한다.
//!
//! 실행:  cargo test --test sign_convention_probe -- --nocapture
//!        cargo test --test sign_convention_probe -- --ignored --nocapture

use micropitting_model::m1_dry::solve_dry;
use micropitting_model::m2_lub::solve_full_film;
use micropitting_model::m6_share::{combine_share_traced, SharePolicy, ShareTrace};
use micropitting_model::partial_lub::solve_partial_traced;
use micropitting_model::types::{
    Field2, Grid, MaterialProps, OperatingConditions, PartialLubInput, PartialLubResult,
};
use std::f64::consts::PI;

// ─────────────────────────────────────────────────────────────────────────
//  공통 입력
// ─────────────────────────────────────────────────────────────────────────

fn steel(p_lim: f64) -> MaterialProps {
    MaterialProps {
        e_red: 1.15e11,
        nu: 0.3,
        hardness: 7.0e9,
        p_lim,
    }
}

/// 순수 구름(S=0): u2 = u_mean, slide_roll = 0.
fn op_pure_rolling() -> OperatingConditions {
    OperatingConditions {
        p_h: 1.5e9,
        u_mean: 1.0,
        u2: 1.0,
        slide_roll: 0.0,
        eta0: 0.0094,
        alpha_visc: 2.078e-8,
        tau0: 3.0e6,
        temp: 348.15,
        r_x: 0.01,
    }
}

fn make_input(grid: Grid, rough1: Field2, h_bar: f64, p_lim: f64) -> PartialLubInput {
    PartialLubInput {
        grid,
        rough1,
        rough2: Field2::zeros(grid.nx, grid.ny),
        mat: steel(p_lim),
        op: op_pure_rolling(),
        h_bar,
    }
}

/// 정규화 상관 ⟨(f−f̄)(g−ḡ)⟩/(σ_f σ_g). σ=0 이면 NaN.
fn corr(f: &[f64], g: &[f64]) -> f64 {
    assert_eq!(f.len(), g.len());
    let n = f.len() as f64;
    let mf = f.iter().sum::<f64>() / n;
    let mg = g.iter().sum::<f64>() / n;
    let mut sfg = 0.0;
    let mut sff = 0.0;
    let mut sgg = 0.0;
    for k in 0..f.len() {
        let a = f[k] - mf;
        let b = g[k] - mg;
        sfg += a * b;
        sff += a * a;
        sgg += b * b;
    }
    sfg / (sff.sqrt() * sgg.sqrt())
}

fn argmax(v: &[f64]) -> usize {
    let mut k = 0;
    for i in 1..v.len() {
        if v[i] > v[k] {
            k = i;
        }
    }
    k
}

fn argmin(v: &[f64]) -> usize {
    let mut k = 0;
    for i in 1..v.len() {
        if v[i] < v[k] {
            k = i;
        }
    }
    k
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn ij(k: usize, nx: usize) -> (usize, usize) {
    (k % nx, k / nx)
}

/// **What-if(가설 수정 시뮬레이션, 생산코드 무변경)**: M2 에 `−rough` 를 먹여 부호를 맞춘
/// 유막해(h_lub−h̄ 및 p_lub 리플 모두 반전)를 M1 dry 해(+rough)와 함께 M6 에 실결선.
/// `partial_lub::two_surface_film` 과 동일하게 p_lub 에 Hertz 평균압 p̄ 를 싣는다(S=0·rough2=0
/// 이므로 two_surface_film ≡ solve_full_film + p̄).
fn whatif_m2_sign_fixed(inp: &PartialLubInput) -> (PartialLubResult, ShareTrace) {
    let dry = solve_dry(inp);
    let mut neg = inp.clone();
    for v in neg.rough1.data.iter_mut() {
        *v = -*v;
    }
    let mut lub = solve_full_film(&neg);
    for v in lub.p_lub.data.iter_mut() {
        *v += inp.op.p_h;
    }
    let policy = SharePolicy {
        w_total: inp.op.p_h * inp.grid.lx * inp.grid.ly,
        e_red: inp.mat.e_red,
        p_lim: inp.mat.p_lim,
        ..Default::default()
    };
    combine_share_traced(&dry, &lub, &inp.grid, &inp.op, &policy)
}

// ─────────────────────────────────────────────────────────────────────────
//  1. 일방향 범프 프로브
// ─────────────────────────────────────────────────────────────────────────

const BUMP_H: f64 = 0.3e-6; // 범프 높이 0.3 µm (+ = 돌출)
const BUMP_SIG: f64 = 2.0e-6; // Gaussian σ = 2 cell

struct BumpCase {
    input: PartialLubInput,
    apex: usize, // row-major index of the apex
}

/// 128×16, dx=dy=1 µm, 중앙에 양의 2D Gaussian 범프 하나(rough2=0).
fn bump_case(h_bar: f64, p_lim: f64) -> BumpCase {
    let nx = 128usize;
    let ny = 16usize;
    let lx = 128.0e-6;
    let ly = 16.0e-6;
    let grid = Grid::new(nx, ny, lx, ly);
    let (dx, dy) = (grid.dx(), grid.dy());
    let (i0, j0) = (nx / 2, ny / 2);
    let (x0, y0) = (i0 as f64 * dx, j0 as f64 * dy);
    let mut r = Field2::zeros(nx, ny);
    for j in 0..ny {
        for i in 0..nx {
            let x = i as f64 * dx - x0;
            let y = j as f64 * dy - y0;
            r.set(i, j, BUMP_H * (-(x * x + y * y) / (2.0 * BUMP_SIG * BUMP_SIG)).exp());
        }
    }
    let apex = r.idx(i0, j0);
    BumpCase {
        input: make_input(grid, r, h_bar, p_lim),
        apex,
    }
}

/// **M1 특성화(비-ignore)**: 양의 범프 → p_dry 최대·h_dry 최소가 apex. (M1 = 높이 규약)
#[test]
fn m1_dry_bump_pressure_peaks_at_apex() {
    let c = bump_case(0.15e-6, 4.0e9);
    let dry = solve_dry(&c.input);
    let nx = c.input.grid.nx;
    let pmax = dry.p_dry.max().unwrap();
    let k_p = argmax(&dry.p_dry.data);
    let hmin = dry.h_dry.min().unwrap();
    let da = c.input.grid.dx() * c.input.grid.dy();
    let w_target = c.input.op.p_h * c.input.grid.lx * c.input.grid.ly;
    let w_dry: f64 = dry.p_dry.data.iter().map(|&p| p * da).sum();
    println!("[M1 bump] apex={:?} argmax(p_dry)={:?} p_dry(apex)={:.4e} pmax={:.4e}",
        ij(c.apex, nx), ij(k_p, nx), dry.p_dry.data[c.apex], pmax);
    println!("[M1 bump] h_dry(apex)={:.4e} h_dry.min={:.4e} h_dry.max={:.4e}",
        dry.h_dry.data[c.apex], hmin, dry.h_dry.max().unwrap());
    println!("[M1 bump] load after p_lim clamp: {:.4e} / target {:.4e} ({:.1}%)",
        w_dry, w_target, 100.0 * w_dry / w_target);
    // apex 가 최대압 plateau 위(p_lim 절단 시 plateau 가능).
    assert!(dry.p_dry.data[c.apex] >= pmax * (1.0 - 1e-9), "p_dry not maximal at apex");
    // apex 간극은 0(접촉).
    assert!(dry.h_dry.data[c.apex] <= hmin + 1e-15, "h_dry not minimal at apex");
    assert!(pmax > c.input.op.p_h, "no asperity concentration");
}

/// **M2 특성화(비-ignore)**: apex 에서 h_lub − h̄ 의 부호·크기를 측정·출력. 구조만 단정.
#[test]
fn m2_film_at_bump_apex_characterization() {
    let c = bump_case(0.15e-6, 4.0e9);
    let lub = solve_full_film(&c.input);
    let hb = c.input.h_bar;
    let nx = c.input.grid.nx;
    let dh: Vec<f64> = lub.h_lub.data.iter().map(|v| v - hb).collect();
    let dh_apex = dh[c.apex];
    let k_max = argmax(&dh);
    let k_min = argmin(&dh);
    println!("[M2 bump] h_bar={:.3e}  h_lub(apex)-h_bar = {:+.4e} m  (= {:+.3} × bump height)",
        hb, dh_apex, dh_apex / BUMP_H);
    println!("[M2 bump] argmax(h_lub-h_bar)={:?} val={:+.4e}; argmin={:?} val={:+.4e}; apex={:?}",
        ij(k_max, nx), dh[k_max], ij(k_min, nx), dh[k_min], ij(c.apex, nx));
    println!("[M2 bump] corr(h_lub-h_bar, rough) = {:+.4}", corr(&dh, &c.input.rough1.data));
    println!("[M2 bump] p_lub(apex)={:+.4e} p_lub.max={:+.4e} p_lub.min={:+.4e}",
        lub.p_lub.data[c.apex], lub.p_lub.max().unwrap(), lub.p_lub.min().unwrap());
    assert!(dh.iter().all(|v| v.is_finite()));
    assert!((mean(&lub.h_lub.data) - hb).abs() < hb * 1e-9, "mean(h_lub) != h_bar");
    // 리플이 실제로 존재(측정 유효성).
    assert!(dh_apex.abs() > 0.1 * BUMP_H, "no film ripple at apex: {dh_apex:e}");
}

/// **물리 단정**: 돌출 아스페리티 위에서는 유막 간극이 평균보다 **작아야** 한다
/// (h_lub(apex) − h̄ < 0). M2 가 rough 를 간극 섭동으로 읽으면 부호가 반대가 되어 실패.
#[test]
#[ignore = "suspected defect: M2 reads rough as gap perturbation (h_lub = h_bar + rough), \
            so film is LARGER at the asperity apex; see sign_convention probe report"]
fn m2_film_smaller_at_asperity_apex() {
    let c = bump_case(0.15e-6, 4.0e9);
    let lub = solve_full_film(&c.input);
    let dh_apex = lub.h_lub.data[c.apex] - c.input.h_bar;
    assert!(
        dh_apex < 0.0,
        "film must be thinner over a protruding asperity: h_lub(apex)-h_bar = {dh_apex:+e}"
    );
}

/// **전체 결선 특성화(비-ignore)**: partial_lub 체인에서 범프가 접촉하는지 측정·출력.
#[test]
fn partial_chain_bump_characterization() {
    let c = bump_case(0.15e-6, 4.0e9); // h̄ = 0.5 × bump height → 범프는 반드시 접촉해야 함
    let (res, tr) = solve_partial_traced(&c.input);
    let nx = c.input.grid.nx;
    let hs = tr.share.h_sep;
    let k_pmax = argmax(&res.p_tran.data);
    let k_hmin = argmin(&res.h_tran.data);
    let dht: Vec<f64> = res.h_tran.data.iter().map(|v| v - hs).collect();
    println!("[chain bump] phi_bl={:.4} contact_count={} h_sep={:.3e} outer_conv={} ",
        res.phi_bl, tr.share.contact_count, hs, tr.outer_converged);
    println!("[chain bump] apex={:?}  argmax(p_tran)={:?} p_tran(apex)={:.4e} p_tran.max={:.4e}",
        ij(c.apex, nx), ij(k_pmax, nx), res.p_tran.data[c.apex], res.p_tran.data[k_pmax]);
    println!("[chain bump] h_tran(apex)={:.4e} (h_tran(apex)-h_sep={:+.4e})  argmin(h_tran)={:?} h_tran.min={:.4e}",
        res.h_tran.data[c.apex], dht[c.apex], ij(k_hmin, nx), res.h_tran.data[k_hmin]);
    println!("[chain bump] corr(h_tran-h_sep, rough)={:+.4}  corr(p_tran, rough)={:+.4}",
        corr(&dht, &c.input.rough1.data), corr(&res.p_tran.data, &c.input.rough1.data));
    // 중앙 행 x-프로파일(apex 주변 ±6 cell): rough / h_tran−h_sep / p_tran
    let j0 = c.input.grid.ny / 2;
    let i0 = nx / 2;
    println!("[chain bump]   i   rough[m]      h_tran-h_sep[m]  p_tran[Pa]");
    for i in (i0 - 6)..=(i0 + 6) {
        let k = i + j0 * nx;
        println!("[chain bump] {:3} {:+.3e}   {:+.3e}     {:.3e}",
            i, c.input.rough1.data[k], dht[k], res.p_tran.data[k]);
    }
    let k_ghost = (i0 + nx / 2) % nx + j0 * nx;
    println!("[chain bump] ghost (x+lx/2): rough={:+.3e} h_tran-h_sep={:+.3e} p_tran={:.3e}",
        c.input.rough1.data[k_ghost], dht[k_ghost], res.p_tran.data[k_ghost]);
    assert!(res.p_tran.data.iter().all(|v| v.is_finite()));
    assert!(res.h_tran.data.iter().all(|v| v.is_finite()));
    assert!(tr.outer_converged);
}

/// **What-if 특성화(비-ignore)**: M2 부호를 맞춘 유막해로 M6 결선 시 범프가 apex 에서 접촉하는지.
/// 가설 수정의 효과 측정이며, 통과하면 "최소 수정 위치 = M2 입력 부호" 의 수치 근거가 된다.
#[test]
fn whatif_bump_contacts_at_apex_when_m2_sign_fixed() {
    let c = bump_case(0.15e-6, 4.0e9);
    let (res, tr) = whatif_m2_sign_fixed(&c.input);
    let nx = c.input.grid.nx;
    let hs = tr.h_sep;
    let k_pmax = argmax(&res.p_tran.data);
    let k_hmin = argmin(&res.h_tran.data);
    let dht: Vec<f64> = res.h_tran.data.iter().map(|v| v - hs).collect();
    println!("[whatif bump] phi_bl={:.4} contact_count={} h_sep={:.3e}", res.phi_bl, tr.contact_count, hs);
    println!("[whatif bump] apex={:?} argmax(p_tran)={:?} p_tran(apex)={:.4e} p_tran.max={:.4e}",
        ij(c.apex, nx), ij(k_pmax, nx), res.p_tran.data[c.apex], res.p_tran.data[k_pmax]);
    println!("[whatif bump] h_tran(apex)-h_sep={:+.4e} argmin(h_tran)={:?} h_tran.min={:.4e}",
        dht[c.apex], ij(k_hmin, nx), res.h_tran.data[k_hmin]);
    println!("[whatif bump] corr(h_tran-h_sep, rough)={:+.4}  corr(p_tran, rough)={:+.4}",
        corr(&dht, &c.input.rough1.data), corr(&res.p_tran.data, &c.input.rough1.data));
    assert!(res.p_tran.data.iter().all(|v| v.is_finite()));
    // 가설 수정 하에서의 물리: apex 접촉 + 전이압 최대가 apex(plateau 허용).
    assert!(res.phi_bl > 0.0 && tr.contact_count > 0, "what-if: apex must contact");
    assert!(dht[c.apex] < 0.0, "what-if: h_tran must close at apex");
    assert!(
        res.p_tran.data[c.apex] >= res.p_tran.data[k_pmax] * (1.0 - 1e-9),
        "what-if: p_tran must be maximal at apex"
    );
}

/// **물리 단정**: h̄ = 0.5×범프높이 이면 범프 apex 는 반드시 접촉(전이 간극 최소·전이압 최대).
#[test]
#[ignore = "suspected defect: M2 gap-sign flip propagates through M6 freq-max merge so the \
            apex is not the p_tran maximum / h_tran minimum; see sign_convention probe report"]
fn partial_chain_bump_contacts_at_apex() {
    let c = bump_case(0.15e-6, 4.0e9);
    let (res, tr) = solve_partial_traced(&c.input);
    let k_pmax = argmax(&res.p_tran.data);
    let k_hmin = argmin(&res.h_tran.data);
    assert!(res.phi_bl > 0.0, "bump 0.3 µm vs h_bar 0.15 µm must give asperity contact, phi_bl={}", res.phi_bl);
    assert!(tr.share.contact_count > 0);
    assert_eq!(k_hmin, c.apex, "h_tran minimum must be at the asperity apex");
    assert_eq!(k_pmax, c.apex, "p_tran maximum must be at the asperity apex");
}

// ─────────────────────────────────────────────────────────────────────────
//  2. 정현파 위상(상관 부호) 프로브
// ─────────────────────────────────────────────────────────────────────────

/// 0.16 µm. 완전접촉 임계 p* = π·E_red·A/λ: m=4(λ=32 µm) 1.81 GPa, m=5(λ=25.6 µm) 2.26 GPa
/// > p̄=1.5 GPa → **부분접촉**(Westergaard p_max ≈ 3.3/3.7 GPa < p_lim=4 GPa, 절단 없음).
const SIN_A: f64 = 0.16e-6;

/// 128×8, lx=128 µm, rough1 = A·cos(2π m x/lx) (평균 0, y-균일), rough2=0.
fn sinusoid_case(m: usize, h_bar: f64) -> PartialLubInput {
    let nx = 128usize;
    let ny = 8usize;
    let lx = 128.0e-6;
    let ly = 8.0e-6;
    let grid = Grid::new(nx, ny, lx, ly);
    let mut r = Field2::zeros(nx, ny);
    for j in 0..ny {
        for i in 0..nx {
            let x = i as f64 / nx as f64;
            r.set(i, j, SIN_A * (2.0 * PI * m as f64 * x).cos());
        }
    }
    make_input(grid, r, h_bar, 4.0e9)
}

struct SinReport {
    c_hdry: f64,
    c_pdry: f64,
    c_hlub: f64,
    c_plub: f64,
    c_htran: f64,
    c_ptran: f64,
    /// what-if(M2 부호 수정) 결선의 h_tran / p_tran 상관.
    c_htran_fix: f64,
    c_ptran_fix: f64,
    phi: f64,
    contact: usize,
    /// 부분접촉 여부(h_dry.max > 0.01·A): 완전접촉이면 h_dry≡0 이라 상관이 무의미.
    partial_contact: bool,
}

fn sinusoid_report(m: usize, h_bar: f64) -> SinReport {
    let inp = sinusoid_case(m, h_bar);
    let r = &inp.rough1.data;
    let dry = solve_dry(&inp);
    let lub = solve_full_film(&inp);
    let (res, tr) = solve_partial_traced(&inp);
    let (res_fix, tr_fix) = whatif_m2_sign_fixed(&inp);
    let hb = inp.h_bar;
    let dh_lub: Vec<f64> = lub.h_lub.data.iter().map(|v| v - hb).collect();
    let rep = SinReport {
        c_hdry: corr(&dry.h_dry.data, r),
        c_pdry: corr(&dry.p_dry.data, r),
        c_hlub: corr(&dh_lub, r),
        c_plub: corr(&lub.p_lub.data, r),
        c_htran: corr(&res.h_tran.data, r),
        c_ptran: corr(&res.p_tran.data, r),
        c_htran_fix: corr(&res_fix.h_tran.data, r),
        c_ptran_fix: corr(&res_fix.p_tran.data, r),
        phi: res.phi_bl,
        contact: tr.share.contact_count,
        partial_contact: dry.h_dry.max().unwrap() > 0.01 * SIN_A,
    };
    let amp = |v: &[f64]| (v.iter().map(|x| (x - mean(v)).powi(2)).sum::<f64>() / v.len() as f64).sqrt() * 2f64.sqrt();
    println!("[sin m={m} h_bar={hb:.1e}] p_dry max={:.3e} min={:.3e} (clamp {}), h_dry max={:.3e}, h_lub ripple amp={:.3e} (={:.2}×A), phi_bl={:.4}, contact={}/{}",
        dry.p_dry.max().unwrap(), dry.p_dry.min().unwrap(),
        if dry.p_dry.max().unwrap() >= inp.mat.p_lim * (1.0 - 1e-9) { "YES" } else { "no" },
        dry.h_dry.max().unwrap(), amp(&dh_lub), amp(&dh_lub) / SIN_A, rep.phi, rep.contact, inp.grid.len());
    println!("[sin m={m}] corr vs rough:  h_dry={:+.4}  p_dry={:+.4} | h_lub-h_bar={:+.4}  p_lub={:+.4} | h_tran={:+.4}  p_tran={:+.4}",
        rep.c_hdry, rep.c_pdry, rep.c_hlub, rep.c_plub, rep.c_htran, rep.c_ptran);
    println!("[sin m={m}] what-if M2 sign fixed: h_tran={:+.4}  p_tran={:+.4}  phi_bl={:.4} contact={}",
        rep.c_htran_fix, rep.c_ptran_fix, res_fix.phi_bl, tr_fix.contact_count);
    rep
}

/// **정현파 특성화(비-ignore)**: 짝수/홀수 모드에서 6개 상관 부호 출력. M1 물리만 단정.
#[test]
fn sinusoid_phase_characterization() {
    for &m in &[4usize, 5usize] {
        let rep = sinusoid_report(m, 0.5 * SIN_A);
        assert!(rep.c_hdry.is_finite() && rep.c_ptran.is_finite());
        assert!(rep.partial_contact, "m={m}: probe needs partial dry contact (h_dry not ≡ 0)");
        // M1 물리(높이 규약): 아스페리티 → 간극 최소·압력 최대.
        assert!(rep.c_hdry < -0.5, "m={m}: h_dry must be anti-correlated with rough: {}", rep.c_hdry);
        assert!(rep.c_pdry > 0.5, "m={m}: p_dry must be correlated with rough: {}", rep.c_pdry);
    }
}

/// **What-if 특성화(비-ignore)**: M2 부호를 맞춘 유막해로 M6 결선하면 정현파 전이압 위상이
/// 물리적(아스페리티에서 최대)으로 복원되는지 — 짝수/홀수 모드 모두.
#[test]
fn whatif_sinusoid_phase_restored_when_m2_sign_fixed() {
    for &m in &[4usize, 5usize] {
        let rep = sinusoid_report(m, 0.5 * SIN_A);
        assert!(rep.c_htran_fix < 0.0, "m={m}: what-if corr(h_tran, rough)={:+.4}", rep.c_htran_fix);
        assert!(rep.c_ptran_fix > 0.0, "m={m}: what-if corr(p_tran, rough)={:+.4}", rep.c_ptran_fix);
    }
}

/// **물리 단정**: 완전유막 리플은 아스페리티에서 유막이 얇아져야 함 → corr(h_lub−h̄, rough) < 0.
#[test]
#[ignore = "suspected defect: M2 film ripple is POSITIVELY correlated with rough (gap-perturbation \
            reading) for both even and odd modes; see sign_convention probe report"]
fn sinusoid_m2_film_anticorrelated_with_rough() {
    for &m in &[4usize, 5usize] {
        let rep = sinusoid_report(m, 0.5 * SIN_A);
        assert!(rep.c_hlub < 0.0, "m={m}: corr(h_lub-h_bar, rough)={:+.4} must be < 0", rep.c_hlub);
    }
}

/// **물리 단정**: 전이압은 아스페리티에서 최대 → corr(p_tran, rough) > 0, corr(h_tran, rough) < 0.
#[test]
#[ignore = "suspected defect: merged p_tran phase corrupted (measured signs in probe report); \
            un-ignore once M1/M2 roughness sign convention is unified"]
fn sinusoid_p_tran_correlated_with_rough() {
    for &m in &[4usize, 5usize] {
        let rep = sinusoid_report(m, 0.5 * SIN_A);
        assert!(rep.c_htran < 0.0, "m={m}: corr(h_tran, rough)={:+.4} must be < 0", rep.c_htran);
        assert!(rep.c_ptran > 0.0, "m={m}: corr(p_tran, rough)={:+.4} must be > 0", rep.c_ptran);
    }
}
