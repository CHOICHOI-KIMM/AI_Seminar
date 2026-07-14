//! # 부분윤활 서브시스템 상위 오케스트레이션 (Phase 2 · §8.B)
//!
//! M1(`solve_dry`)·M2(`solve_full_film`)·M6(`combine_share_traced`)를 **하나의 완전
//! 결합 solver** 로 승격한다. 진입점 [`solve_partial`] 은 부분윤활 최종 산출물
//! `p_tran`(식[258])·`h_tran`(식[252])·`phi_bl` 및 M3용 마찰 트랙션 `q_tran=μ·p_tran`
//! 을 자기일관 산출한다.
//!
//! ## 결선 절차 (원 논문 "Combined Model for Partial Lubrication" L247–263; (21)(23) 절차①~⑤)
//! 1. **M1** `solve_dry`(복합거칠기 `rough1+rough2` 내부합산) → `p_dry`·`h_dry`.
//! 2. **M2 두 거친면**([`two_surface_film`]) → `p_lub=p_lub1+p_lub2`(+ Hertz 평균압 `p̄`).
//!    각 표면은 **자기 표면속도**(u₁, u₂)로 리플을 생성한다(식[2] 대류항; §8.B.1-3).
//! 3. **M6** `combine_share_traced` — flow-balance 절차①~⑤(φ_bl 이분법 + 영역 재식별).
//! 4. **외부루프**(절차②④): 평균압 `p̄`(→c_ρ) 자기일관 + 영역 재식별 안정화.
//! 5. **마찰**: `q_tran=μ_eff·p_tran`, `μ_eff=φ_bl·μ_bl+(1−φ_bl)·μ_ehl`(원 논문 Table 1/2).
//!
//! ## 하중보존 (CV-M6-Load, A안)
//! `combine_share_traced` 내부 [`crate::m6_share`] 의 **A안 사후 하중재균형**(강체 오프셋
//! 이분법)이 cavitation 절단 후에도 `∫p_tran·dA=W` 를 ≤0.1% 로 고정한다. 따라서 외부루프의
//! 평균압 고정점은 `p̄=p_h`(절차② 자기일관)로 즉시 수렴한다 — B안(외부 하중루프)의 평균압
//! 조정단계를 A안이 이미 만족시키는 구조. 물리충실 B안은 이 자기일관성 위에서 성립.
//!
//! ## 단위 (SI)
//! 길이/간극/유막 [m], 압력·트랙션 [Pa], 하중 [N], 면적요소 `dA=dx·dy` [m²].

use crate::m1_dry::solve_dry;
use crate::m2_lub::solve_full_film;
use crate::m6_share::{combine_share_traced, ShareTrace, SharePolicy};
use crate::types::{
    Field2, LubResult, OperatingConditions, PartialLubInput, PartialLubResult, EPS,
};

/// 경계윤활(boundary) 마찰계수 μ_bl [-] — 원 논문 Table 1/2 (P2-2 §2.7, T3-5 확인 🅣).
pub const MU_BL: f64 = 0.12;
/// 완전유막(full-film EHL) 마찰계수 μ_ehl [-] — 원 논문 Table 1/2 (P2-2 §2.7).
pub const MU_EHL: f64 = 0.05;

/// 외부(절차②④) 루프 상한.
const OUTER_MAX: usize = 50;
/// 외부 루프 수렴 상대 tol (φ_bl·p̄ 동시).
const OUTER_TOL: f64 = 1e-6;

/// 유효 마찰계수 `μ_eff = φ_bl·μ_bl + (1−φ_bl)·μ_ehl` [-] (원 논문 `q=μp` 가중; Table 1/2).
///
/// 경계윤활 분율 `phi_bl` 로 경계마찰([`MU_BL`])·유막마찰([`MU_EHL`])을 선형 가중.
/// `φ_bl=0`(완전유막) → `μ_ehl`, `φ_bl=1`(완전경계) → `μ_bl`. 상하계 `[μ_ehl, μ_bl]`.
#[inline]
pub fn mu_effective(phi_bl: f64) -> f64 {
    let phi = phi_bl.clamp(0.0, 1.0);
    phi * MU_BL + (1.0 - phi) * MU_EHL
}

/// 한 거친면(`rough`, 표면속도 `u_surface`)이 생성하는 EHL 리플을 [`solve_full_film`] 로 계산.
///
/// `solve_full_film` 은 표면2(u₂) 대류를 하드코딩(시그니처 동결)하므로, 임의 표면속도
/// `u_s` 의 리플을 얻으려면 `op` 를 재매핑한다:
/// ```text
///   내부 u_conv = −½·slide_roll'·ū = u_s − ū   ⇒  slide_roll' = 2·(ū − u_s)/ū
///   상보파 ω_c  = kx·(u₂'/ū)                     ⇒  u₂' = u_s
/// ```
/// **근거**: 식[2] Reynolds 대류항 — 각 표면 거칠기는 자기 표면속도로 대류한다. SRR 규약
/// `u₁=ū(1+½SRR)`, `u₂=ū(1−½SRR)`. 전단률 `γ̇=|Δu|/h0`·방향점도비 `V` 는 `slide_roll'` 의
/// **부호와 무관**(`|slide_roll'·ū|=|Δu|` 불변)해 두 표면에서 동일 → 점도장 일관.
fn surface_film(input: &PartialLubInput, rough: &Field2, u_surface: f64) -> LubResult {
    let u_mean = input.op.u_mean;
    let slide_roll_p = if u_mean != 0.0 {
        2.0 * (u_mean - u_surface) / u_mean
    } else {
        0.0
    };
    let op_p = OperatingConditions {
        u2: u_surface,
        slide_roll: slide_roll_p,
        ..input.op
    };
    let sub = PartialLubInput {
        grid: input.grid,
        rough1: rough.clone(),
        rough2: Field2::zeros(input.grid.nx, input.grid.ny),
        mat: input.mat,
        op: op_p,
        h_bar: input.h_bar,
    };
    solve_full_film(&sub)
}

/// 두 거친면 결합 유막해 (§8.B.1-3, CV-M6-Load): `p_lub=p̄+리플₁+리플₂`, `h_lub=h̄+Δh₁+Δh₂`.
///
/// 각 표면이 **다른 속도**(u₁, u₂)로 리플을 생성([`surface_film`]) → 두 리플을 합성한다.
/// 압력 리플(평균 0)에 **Hertz 평균압 `p̄=op.p_h`** 를 실어 유막이 하중을 지지하는 물리
/// 압력장으로 만든다(창≪Hertz 반접촉폭 → p̄≈peak; M1·M6 규약 정합, P2-1 §3). 유막두께는
/// `h̄` 주위 두 표면 리플 섭동의 합.
///
/// (선형 전달함수라 동일속도면 `film(r₁)+film(r₂)=film(r₁+r₂)` 이나, u₁≠u₂ 로 두 표면의
/// 대류속도가 반대부호(±Δu/2)라 리플 위상이 달라 **비자명 2표면 합성**이 된다.)
fn two_surface_film(input: &PartialLubInput) -> LubResult {
    let u_mean = input.op.u_mean;
    let srr = input.op.slide_roll;
    let u1 = u_mean * (1.0 + 0.5 * srr); // 표면1 속도 (SRR 규약)
    let u2 = u_mean * (1.0 - 0.5 * srr); // 표면2 속도
    let lub1 = surface_film(input, &input.rough1, u1);
    let lub2 = surface_film(input, &input.rough2, u2);

    let (nx, ny) = (input.grid.nx, input.grid.ny);
    let n = nx * ny;
    let hb = input.h_bar;
    let p_bar = input.op.p_h;
    let mut p = Field2::zeros(nx, ny);
    let mut h = Field2::zeros(nx, ny);
    for k in 0..n {
        // p_lub = p̄ + 리플₁ + 리플₂ (리플 평균 0).
        p.data[k] = p_bar + lub1.p_lub.data[k] + lub2.p_lub.data[k];
        // h_lub = h̄ + (h₁−h̄) + (h₂−h̄) = 리플섭동 합 위의 평균유막.
        h.data[k] = hb + (lub1.h_lub.data[k] - hb) + (lub2.h_lub.data[k] - hb);
    }
    LubResult { p_lub: p, h_lub: h }
}

/// 부분윤활 서브시스템 반복 추적(진단).
#[derive(Debug, Clone)]
pub struct PartialTrace {
    /// 외부(절차②④) 반복 횟수.
    pub outer_iters: usize,
    /// 외부 루프 수렴 여부(φ_bl·p̄ 자기일관).
    pub outer_converged: bool,
    /// 수렴 유효 마찰계수 μ_eff [-].
    pub mu_eff: f64,
    /// 수렴 평균 접촉압 p̄ [Pa] (절차② 자기일관 고정점 = ∫p_tran·dA/A).
    pub p_bar: f64,
    /// 내부 flow-balance 추적([`crate::m6_share::ShareTrace`]).
    pub share: ShareTrace,
}

/// 부분윤활 완전결합 solver (추적 포함) — M1+M2+M6 실결선.
///
/// 반환: `PartialLubResult { p_tran, h_tran, phi_bl, q_tran }` 및 반복 추적 [`PartialTrace`].
pub fn solve_partial_traced(input: &PartialLubInput) -> (PartialLubResult, PartialTrace) {
    let grid = input.grid;
    let op = &input.op;

    // ── ① M1 건식 접촉(복합거칠기 내부합산) ──
    let dry = solve_dry(input);
    // ── ② M2 두 거친면 → p_lub=p_lub1+p_lub2 (+ Hertz 평균압) ──
    let lub = two_surface_film(input);

    // 목표 창하중 W = p_h·Lx·Ly (M1·규약 정합).
    let w_total = op.p_h * grid.lx * grid.ly;
    let policy = SharePolicy {
        w_total,
        e_red: input.mat.e_red,
        p_lim: input.mat.p_lim,
        ..Default::default()
    };

    let da = grid.dx() * grid.dy();
    let area = grid.lx * grid.ly;

    // ── 절차②④ 외부루프: 평균압 p̄(→c_ρ) 자기일관 + 영역 재식별 안정화 ──
    // A안(recover_p_tran)이 ∫p_tran=W 를 강체오프셋으로 고정 ⇒ mean(p_tran)=p̄₀=p_h
    // ⇒ p̄ 고정점 = p_h (즉시 수렴). 루프는 절차②(평균압 반복)·④(영역 재식별) 자기일관 검증.
    let mut p_bar = op.p_h;
    let mut prev_phi = f64::NAN;
    let mut outer_iters = 0usize;
    let mut outer_converged = false;
    let mut result = PartialLubResult {
        p_tran: Field2::zeros(grid.nx, grid.ny),
        h_tran: Field2::zeros(grid.nx, grid.ny),
        phi_bl: 0.0,
        q_tran: Field2::zeros(grid.nx, grid.ny),
    };
    let mut share = ShareTrace {
        phi_history: Vec::new(),
        converged: false,
        iters: 0,
        h_sep: 0.0,
        contact_count: 0,
        load_residual: 0.0,
        flow_balance_residual: 0.0,
        asperity_degenerate: false,
    };

    while outer_iters < OUTER_MAX {
        outer_iters += 1;
        let op_iter = OperatingConditions { p_h: p_bar, ..*op };
        let (res, tr) = combine_share_traced(&dry, &lub, &grid, &op_iter, &policy);
        // 절차②: 평균압 갱신 = 실제 전이압 평균(∫p_tran·dA/A).
        let p_bar_new = if area > 0.0 {
            res.p_tran.data.iter().map(|&p| p * da).sum::<f64>() / area
        } else {
            p_bar
        };
        let d_phi = (res.phi_bl - prev_phi).abs();
        let d_p = if p_bar.abs() > EPS {
            (p_bar_new - p_bar).abs() / p_bar.abs()
        } else {
            (p_bar_new - p_bar).abs()
        };
        result = res;
        share = tr;
        let stable = prev_phi.is_finite() && d_phi < OUTER_TOL && d_p < OUTER_TOL;
        prev_phi = result.phi_bl;
        p_bar = p_bar_new;
        if stable {
            outer_converged = true;
            break;
        }
    }

    // ── ⑤ 마찰 트랙션 q_tran = μ_eff·p_tran (원 논문 q=μp, Coulomb) ──
    let mu_eff = mu_effective(result.phi_bl);
    let mut q = Field2::zeros(grid.nx, grid.ny);
    for k in 0..grid.len() {
        q.data[k] = mu_eff * result.p_tran.data[k];
    }
    result.q_tran = q;

    let ptrace = PartialTrace {
        outer_iters,
        outer_converged,
        mu_eff,
        p_bar,
        share,
    };
    (result, ptrace)
}

/// 부분윤활 완전결합 solver (표준 진입점, §8.B) — 결과만 반환.
pub fn solve_partial(input: &PartialLubInput) -> PartialLubResult {
    solve_partial_traced(input).0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;
    use crate::util::fft::fft2_forward;
    use rustfft::num_complex::Complex;
    use std::f64::consts::PI;

    // 합성 혼합윤활 입력 (통합테스트와 동형; interior phi 유발).
    fn mixed_input(h_bar: f64, amp1: f64, amp2: f64) -> PartialLubInput {
        let nx = 64usize;
        let ny = 64usize;
        let lx = 200e-6;
        let ly = 200e-6;
        let grid = Grid::new(nx, ny, lx, ly);
        let dx = grid.dx();
        let dy = grid.dy();
        let kx1 = 2.0 * PI * 5.0 / lx;
        let ky1 = 2.0 * PI * 4.0 / ly;
        let kx2 = 2.0 * PI * 7.0 / lx;
        let ky2 = 2.0 * PI * 3.0 / ly;
        let mut rough1 = Field2::zeros(nx, ny);
        let mut rough2 = Field2::zeros(nx, ny);
        for j in 0..ny {
            for i in 0..nx {
                let x = i as f64 * dx;
                let y = j as f64 * dy;
                rough1.set(i, j, amp1 * (kx1 * x).cos() * (ky1 * y).cos());
                rough2.set(i, j, amp2 * (kx2 * x).sin() * (ky2 * y).sin());
            }
        }
        PartialLubInput {
            grid,
            rough1,
            rough2,
            mat: MaterialProps {
                e_red: E_RED_STEEL_PA,
                nu: NU_STEEL,
                hardness: 7.0e9,
                p_lim: 1.0e30, // 소성절단 비활성(배선/보존 격리).
            },
            op: OperatingConditions {
                p_h: 1.5e9,
                u_mean: 1.0,
                u2: 0.95, // = u_mean(1−½·SRR), SRR=0.1
                slide_roll: 0.1,
                eta0: 0.01,
                alpha_visc: 2.0e-8,
                tau0: 5.0e6,
                temp: 353.0,
                r_x: 0.02,
            },
            h_bar,
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  (a) CV-M6-Load — 하중보존 ≤0.1% · φ_bl∈[0,1] · flow_balance_residual<1e-9
    // ═══════════════════════════════════════════════════════════════════════
    #[test]
    fn cv_m6_load_conservation_partial() {
        let input = mixed_input(1.0e-8, 0.20e-6, 0.16e-6);
        let (res, tr) = solve_partial_traced(&input);
        let da = input.grid.dx() * input.grid.dy();
        let w = input.op.p_h * input.grid.lx * input.grid.ly;
        let w_tran: f64 = res.p_tran.data.iter().map(|&p| p * da).sum();
        // CV-M6-Load: ∫p_tran·dA = W, ≤0.1%.
        assert!(
            (w_tran - w).abs() / w < 1e-3,
            "load not conserved ≤0.1%: {w_tran:.6e} vs W={w:.6e}"
        );
        assert!(tr.share.load_residual < 1e-3, "load_residual {}", tr.share.load_residual);
        // φ_bl∈[0,1].
        assert!(res.phi_bl >= 0.0 && res.phi_bl <= 1.0, "phi_bl out of [0,1]: {}", res.phi_bl);
        // 절차⑤ 항등: mean(h_tran)=c_ρ·h̄.
        assert!(tr.share.flow_balance_residual < 1e-9, "flow-balance residual {}", tr.share.flow_balance_residual);
        // cavitation: p_tran≥0, 유한.
        assert!(res.p_tran.data.iter().all(|&v| v >= 0.0), "p_tran must be ≥0");
        assert!(res.p_tran.data.iter().all(|v| v.is_finite()), "p_tran non-finite");
        assert!(res.h_tran.data.iter().all(|v| v.is_finite()), "h_tran non-finite");
        assert!(res.q_tran.data.iter().all(|v| v.is_finite()), "q_tran non-finite");
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  (b) 극한·지배효과·고정점 — 실결선 재실행
    // ═══════════════════════════════════════════════════════════════════════

    /// **완전분리 극한**(φ→0): 두꺼운 유막 h̄≫거칠기 → 아스페리티 접촉 없음 → phi_bl→0.
    #[test]
    fn limit_phi_to_zero_thick_film() {
        let input = mixed_input(8.0e-7, 0.15e-6, 0.12e-6);
        let (res, tr) = solve_partial_traced(&input);
        assert!(res.phi_bl < 1e-6, "thick film must give phi_bl→0, got {}", res.phi_bl);
        assert_eq!(tr.share.contact_count, 0, "no asperity contact expected");
    }

    /// **지배효과**: 유막 h̄ 증가 → 접촉↓ → phi_bl 단조감소 (M1·M2·M6 실결선).
    #[test]
    fn dominant_effect_thicker_film_lowers_phi() {
        let mut prev = 2.0_f64;
        for &hb in &[1.0e-8_f64, 2.0e-8, 4.0e-8, 8.0e-8] {
            let input = mixed_input(hb, 0.20e-6, 0.16e-6);
            let phi = solve_partial(&input).phi_bl;
            assert!(
                phi <= prev + 1e-9,
                "phi_bl not decreasing with film thickness: hb={hb}, phi={phi}, prev={prev}"
            );
            prev = phi;
        }
    }

    /// **flow-balance 고정점 + 외부루프 수렴**: 혼합영역 interior + 절차② 평균압 자기일관.
    #[test]
    fn flow_balance_fixed_point_partial() {
        let input = mixed_input(1.0e-8, 0.20e-6, 0.16e-6);
        let (res, tr) = solve_partial_traced(&input);
        // 혼합영역(interior).
        assert!(res.phi_bl > 0.0 && res.phi_bl < 1.0, "expected interior mixed, phi={}", res.phi_bl);
        assert!(
            tr.share.contact_count > 0 && tr.share.contact_count < input.grid.len(),
            "partial asperity contact expected: {}/{}",
            tr.share.contact_count,
            input.grid.len()
        );
        // 외부(절차②④) 루프 수렴 + 평균압 고정점 p̄=p_h (A안 하중고정 자기일관).
        assert!(tr.outer_converged, "outer loop must converge in {} iters", tr.outer_iters);
        assert!(
            (tr.p_bar - input.op.p_h).abs() / input.op.p_h < 1e-3,
            "mean contact pressure fixed point must be p_h: p̄={:.4e} vs p_h={:.4e}",
            tr.p_bar,
            input.op.p_h
        );
        assert!(tr.share.converged, "inner flow-balance must converge");
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  마찰 트랙션 q_tran = μ_eff·p_tran (원 논문 q=μp; Table 1/2)
    // ═══════════════════════════════════════════════════════════════════════
    #[test]
    fn friction_traction_q_equals_mu_p() {
        let input = mixed_input(1.0e-8, 0.20e-6, 0.16e-6);
        let (res, tr) = solve_partial_traced(&input);
        let mu = tr.mu_eff;
        // μ_eff ∈ [μ_ehl, μ_bl].
        assert!(mu >= MU_EHL - 1e-12 && mu <= MU_BL + 1e-12, "mu_eff out of [ehl,bl]: {mu}");
        // q = μ·p 정확(전 격자).
        for k in 0..res.p_tran.len() {
            let expect = mu * res.p_tran.data[k];
            assert!(
                (res.q_tran.data[k] - expect).abs() <= expect.abs() * 1e-12 + 1e-6,
                "q_tran≠μ·p_tran at {k}: {} vs {expect}",
                res.q_tran.data[k]
            );
        }
        // 극한 정합(독립·하드코딩): φ=0→μ_ehl=0.05, φ=1→μ_bl=0.12.
        assert!((mu_effective(0.0) - MU_EHL).abs() < 1e-15, "phi=0 → μ_ehl");
        assert!((mu_effective(1.0) - MU_BL).abs() < 1e-15, "phi=1 → μ_bl");
        assert!((mu_effective(0.5) - 0.085).abs() < 1e-12, "phi=0.5 → 0.085 midpoint");
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  (c) RP-Field — 정성 불변식만 (정량 임계 금지; 이미지 대조 SKF Fig.6/15)
    // ═══════════════════════════════════════════════════════════════════════

    /// **RP-Field 정성**: (i) p_tran 리플 파장이 거칠기 파장과 정합(주입 모드에 스펙트럼 피크),
    /// (ii) 트랙션 q_tran 이 p_tran 에 비례(q=μp), (iii) p_tran≥0(cavitation). 정량 임계 없음.
    #[test]
    fn rp_field_qualitative_invariants() {
        let input = mixed_input(1.0e-8, 0.20e-6, 0.16e-6);
        let (res, _tr) = solve_partial_traced(&input);
        let nx = input.grid.nx;
        let ny = input.grid.ny;

        // (i) 리플 파장정합: p_tran 변동분(평균 제거) 스펙트럼이 거칠기 모드(비-DC)에 에너지.
        let mean_p = res.p_tran.data.iter().sum::<f64>() / (nx * ny) as f64;
        let mut spec: Vec<Complex<f64>> = res
            .p_tran
            .data
            .iter()
            .map(|&v| Complex::new(v - mean_p, 0.0))
            .collect();
        fft2_forward(&mut spec, nx, ny);
        // 주입 거칠기 x-모드(m=5, m=7)에 유의 에너지가 있어야(리플 존재·파장 정합).
        let bin = |mx: usize, my: usize| spec[mx + my * nx].norm();
        let e_rough = bin(5, 4) + bin(7, 3) + bin(nx - 5, ny - 4) + bin(nx - 7, ny - 3);
        // 비-거칠기 임의 모드(예: (2,9))의 에너지보다 훨씬 큼 → 리플이 거칠기 파장을 따름.
        let e_other = bin(2, 9) + bin(9, 2) + 1.0;
        assert!(
            e_rough > 10.0 * e_other,
            "p_tran ripple must follow roughness wavelength: e_rough={e_rough:.3e}, e_other={e_other:.3e}"
        );

        // (ii) 트랙션 비례(q=μp): 부호·형태가 p_tran 을 따름(정성).
        let mu = mu_effective(res.phi_bl);
        assert!(mu > 0.0);
        // p_tran 최대점에서 q_tran 도 최대(형태 정합).
        let kmax = (0..res.p_tran.len())
            .max_by(|&a, &b| res.p_tran.data[a].partial_cmp(&res.p_tran.data[b]).unwrap())
            .unwrap();
        let kmax_q = (0..res.q_tran.len())
            .max_by(|&a, &b| res.q_tran.data[a].partial_cmp(&res.q_tran.data[b]).unwrap())
            .unwrap();
        assert_eq!(kmax, kmax_q, "q_tran peak must coincide with p_tran peak (q=μp)");

        // (iii) cavitation: p_tran≥0.
        assert!(res.p_tran.data.iter().all(|&v| v >= 0.0), "p_tran≥0 (cavitation)");
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  두 거친면 결합 (§8.B.1-3, CV-M6-Load) — p_lub=p_lub1+p_lub2 검출력
    // ═══════════════════════════════════════════════════════════════════════

    /// **두 표면 모드 공존**: 유막 리플 스펙트럼에 표면1(m=5,4)·표면2(m=7,3) 모드가 **둘 다**
    /// 유의 기여 → `p_lub=p_lub1+p_lub2` 합성 검증(단일화하면 한 모드가 소멸 → 검출).
    #[test]
    fn two_surface_film_contains_both_modes() {
        let input = mixed_input(1.4e-7, 0.10e-6, 0.08e-6);
        let lub = two_surface_film(&input);
        let nx = input.grid.nx;
        let ny = input.grid.ny;
        let mean = lub.p_lub.data.iter().sum::<f64>() / (nx * ny) as f64;
        let mut spec: Vec<Complex<f64>> = lub
            .p_lub
            .data
            .iter()
            .map(|&v| Complex::new(v - mean, 0.0))
            .collect();
        fft2_forward(&mut spec, nx, ny);
        let bin = |mx: usize, my: usize| spec[mx + my * nx].norm();
        let e1 = bin(5, 4) + bin(nx - 5, ny - 4); // 표면1 모드
        let e2 = bin(7, 3) + bin(nx - 7, ny - 3); // 표면2 모드
        let noise = bin(2, 9) + bin(9, 2) + 1.0;
        assert!(e1 > 10.0 * noise, "surface-1 ripple mode must be present: e1={e1:.3e}");
        assert!(e2 > 10.0 * noise, "surface-2 ripple mode must be present: e2={e2:.3e}");
    }

    /// **표면별 대류속도 결선**: 동일 거칠기라도 표면속도(u₁ vs u₂)가 다르면 리플이 다르다
    /// (식[2] 대류항 부호 (u_s−ū)=±Δu/2). 두 표면을 같은 속도로 하드코딩하면 차이=0 → 검출.
    #[test]
    fn per_surface_velocity_distinct() {
        let input = mixed_input(1.4e-7, 0.10e-6, 0.0);
        let u_mean = input.op.u_mean;
        let srr = input.op.slide_roll;
        let u1 = u_mean * (1.0 + 0.5 * srr);
        let u2 = u_mean * (1.0 - 0.5 * srr);
        let f1 = surface_film(&input, &input.rough1, u1);
        let f2 = surface_film(&input, &input.rough1, u2);
        let diff: f64 = f1
            .p_lub
            .data
            .iter()
            .zip(&f2.p_lub.data)
            .map(|(a, b)| (a - b).abs())
            .sum();
        let scale: f64 = f1.p_lub.data.iter().map(|v| v.abs()).sum::<f64>().max(1.0);
        assert!(
            diff > scale * 1e-6,
            "different surface velocities must give different ripples: diff={diff:.3e}"
        );
    }

    // ── 반환형/차원 계약 + serde 왕복 ──
    #[test]
    fn returns_contract_and_serde_roundtrip() {
        let input = mixed_input(1.0e-8, 0.20e-6, 0.16e-6);
        let res = solve_partial(&input);
        assert_eq!(res.p_tran.nx, input.grid.nx);
        assert_eq!(res.q_tran.len(), input.grid.len());
        assert_eq!(res.h_tran.len(), input.grid.len());
        // serde 왕복(시간루프 직렬화 계약).
        let json = serde_json::to_string(&res).expect("serialize");
        let back: PartialLubResult = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.p_tran.data.len(), res.p_tran.data.len());
        assert!((back.phi_bl - res.phi_bl).abs() < 1e-15);
    }
}
