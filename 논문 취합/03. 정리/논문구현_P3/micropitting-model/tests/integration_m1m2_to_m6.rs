//! M1+M2 → M6 실결선(real-wiring) 통합 스모크 테스트.
//!
//! 합성 거칠기·운전조건으로:
//!   1. `m1_dry::solve_dry`     → `DryResult { p_dry, h_dry }`
//!   2. `m2_lub::solve_full_film`→ `LubResult { p_lub, h_lub }`
//! 을 산출하고, 두 국부해를 `m6_share::combine_share_traced` 에 **실제 입력**하여
//! `PartialLubResult` 를 얻는다. 하중보존(∫p_tran dA = W)·유한성(NaN/Inf 없음)을 확인한다.

use micropitting_model::m1_dry::solve_dry;
use micropitting_model::m2_lub::solve_full_film;
use micropitting_model::m6_share::{combine_share_traced, SharePolicy};
use micropitting_model::types::{
    Field2, Grid, MaterialProps, OperatingConditions, PartialLubInput, E_RED_STEEL_PA, NU_STEEL,
};
use std::f64::consts::PI;

/// 합성 2방향 정현파 거칠기 + 대표 운전조건으로 통합 입력 구성.
fn build_input() -> PartialLubInput {
    let nx = 64usize;
    let ny = 64usize;
    let lx = 200e-6; // 200 μm
    let ly = 200e-6;
    let grid = Grid::new(nx, ny, lx, ly);
    let dx = grid.dx();
    let dy = grid.dy();

    // 표면 1, 2 거칠기 — 서로 다른 파장/진폭(평균 ~0), 진폭 O(0.2 μm).
    // (진폭을 유막 h̄=10 nm 대비 충분히 크게 잡아 혼합윤활 접촉을 유발.)
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
            rough1.set(i, j, 0.20e-6 * (kx1 * x).cos() * (ky1 * y).cos());
            rough2.set(i, j, 0.16e-6 * (kx2 * x).sin() * (ky2 * y).sin());
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
            // 소성절단 비활성(≫peak): 본 스모크는 M1→M2→M6 하중분담 배선/보존을 격리 검증한다.
            // p_lim 절단은 M1 자체 오라클(plastic_clamp_caps_pressure)이 담당하며, 여기서
            // 절단이 작동하면 M1 이 하중을 잘라 배선 보존검사가 오염되므로 크게 둔다.
            p_lim: 1.0e30,
        },
        op: OperatingConditions {
            p_h: 1.5e9,
            u_mean: 1.0,
            u2: 0.95, // = u_mean − slide_roll·u_mean/2 (u_mean·slide_roll 규약 정합)
            slide_roll: 0.1,
            eta0: 0.01,
            alpha_visc: 2.0e-8,
            tau0: 5.0e6,
            temp: 353.0,
        },
        // 혼합윤활(mixed) 영역: 중앙유막 h̄ 를 거칠기 진폭(≈0.2 μm) 대비 얇게(10 nm) 잡아
        // 아스페리티 접촉이 실제로 발생하도록(interior phi_bl). 완전분리(두꺼운 유막)면
        // 물리적으로 phi_bl→0 이 옳으므로, M6 하중분담의 실검증에는 혼합영역이 필요하다.
        h_bar: 1.0e-8,
    }
}

/// M1(p_dry) + M2(p_lub) → M6 결합. 하중보존 + 유한성.
#[test]
fn m1m2_to_m6_load_conservation_and_finite() {
    let input = build_input();
    let grid = input.grid;
    let da = grid.dx() * grid.dy();

    // ── M1: 건식 rough 접촉 ──
    let dry = solve_dry(&input);
    assert_eq!(dry.p_dry.len(), grid.len());
    assert!(dry.p_dry.data.iter().all(|v| v.is_finite()), "p_dry non-finite");
    assert!(dry.h_dry.data.iter().all(|v| v.is_finite()), "h_dry non-finite");
    // M1 은 목표하중 W = p_h·Lx·Ly 로 해를 스케일 → 압력 적분이 그 값을 재현.
    let w_dry: f64 = dry.p_dry.data.iter().map(|&p| p * da).sum();
    let w_target = input.op.p_h * grid.lx * grid.ly;
    assert!(
        (w_dry - w_target).abs() / w_target < 1e-3,
        "M1 load: {w_dry:.4e} vs target {w_target:.4e}"
    );

    // ── M2: EHL 압력 리플 특수해 ──
    let lub = solve_full_film(&input);
    assert_eq!(lub.p_lub.len(), grid.len());
    assert!(lub.p_lub.data.iter().all(|v| v.is_finite()), "p_lub non-finite");
    assert!(lub.h_lub.data.iter().all(|v| v.is_finite()), "h_lub non-finite");
    // 특수해(리플) 평균 ~ 0, 유막 평균 ~ h_bar (섭동분).
    let p_lub_mean: f64 = lub.p_lub.data.iter().sum::<f64>() / grid.len() as f64;
    assert!(p_lub_mean.abs() < 1.0, "p_lub ripple mean not ~0: {p_lub_mean}");
    let h_lub_mean: f64 = lub.h_lub.data.iter().sum::<f64>() / grid.len() as f64;
    assert!((h_lub_mean - input.h_bar).abs() < input.h_bar * 1e-6, "h_lub mean != h_bar");

    // M2 리플만으로는 유막이 하중을 지지하지 않으므로, M6 결합을 위해 유막에
    // Hertz 평균압 오프셋을 실어 물리적 유막 압력장을 구성(리플은 그대로 통과).
    let mut lub_wired = lub.clone();
    for v in lub_wired.p_lub.data.iter_mut() {
        *v += input.op.p_h;
    }

    // ── M6: 실결선 flow-balance 하중분담 결합 ──
    // p_tran 탄성복원(식[258]) 은 E_red·p_lim 을 SharePolicy 로 받는다.
    let policy = SharePolicy {
        w_total: w_target,
        e_red: input.mat.e_red,
        p_lim: input.mat.p_lim,
        ..Default::default()
    };
    let (res, trace) =
        combine_share_traced(&dry, &lub_wired, &grid, &input.op, &policy);

    // 유한성.
    assert!(res.p_tran.data.iter().all(|v| v.is_finite()), "p_tran non-finite");
    assert!(res.h_tran.data.iter().all(|v| v.is_finite()), "h_tran non-finite");
    // cavitation: p_tran ≥ 0.
    assert!(res.p_tran.data.iter().all(|&v| v >= 0.0), "p_tran must be ≥0 (cavitation)");
    // 차원 계약.
    assert_eq!(res.p_tran.nx, grid.nx);
    assert_eq!(res.p_tran.ny, grid.ny);
    assert_eq!(res.h_tran.len(), grid.len());
    // 혼합영역: phi_bl 내부값 + 실제 아스페리티 접촉 발생(영역 재식별 유효).
    assert!(res.phi_bl > 0.0 && res.phi_bl < 1.0, "phi_bl not interior: {}", res.phi_bl);
    assert!(
        trace.contact_count > 0 && trace.contact_count < grid.len(),
        "expected partial asperity contact, got {} / {}",
        trace.contact_count,
        grid.len()
    );

    // ── 하중 보존(A안 재균형): ∫p_tran dA = W (cavitation 절단분을 균일 오프셋으로 복원, ≤0.1%) ──
    let w_tran: f64 = res.p_tran.data.iter().map(|&p| p * da).sum();
    assert!(
        (w_tran - w_target).abs() / w_target < 1e-3,
        "M6 load not conserved: {w_tran:.6e} vs W={w_target:.6e} (resid trace={})",
        trace.load_residual
    );
    assert!(trace.load_residual < 1e-3, "trace load_residual {}", trace.load_residual);
    // flow-balance 절차⑤ 항등: mean(h_tran)=c_ρ·h̄.
    assert!(trace.flow_balance_residual < 1e-9, "flow-balance residual {}", trace.flow_balance_residual);
    assert!(trace.converged, "flow-balance did not converge in {} iters", trace.iters);
    assert!(w_target > 0.0);
}
