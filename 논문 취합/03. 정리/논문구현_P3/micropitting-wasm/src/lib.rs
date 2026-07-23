//! # micropitting-wasm — WASM 경계 셸 (Phase 0 스파이크)
//!
//! 계획 정본: `논문구현_P3_시각화_HTML.md` §3(Phase 0)·§4.2(Phase 2).
//!
//! ## 역할 한정 ★
//! **얇은 경계 어댑터. 물리 로직 금지.** 모든 수치는 [`micropitting_model`] 이 낸다.
//! 여기서 하는 일은 (a) JSON ↔ Rust struct 변환, (b) lifetime·비-serde struct 의 Rust측 조립,
//! (c) 진입점 화이트리스트뿐이다. 계수·식·기본값을 이 파일에서 **새로 만들지 않는다**
//! (자의적 작업 금지 대원칙 — 모든 값은 P2 식[n]/ref 로 소급).
//!
//! ## 왜 셸이 필요한가 (계획 §4.2)
//! - [`WearInput`] 은 `p_tran: &'a Field2` **lifetime 보유** → wasm-bindgen 경계 통과 불가.
//! - [`WearParams`] 는 `Default` 만 있고 **serde 미파생** → JSON 미러 필요.
//! - `types.rs` 의 struct 는 전부 serde 파생 완료 → 그대로 재사용(중복 정의 금지).
//!
//! ## Phase 2 — 세 무증상 실패를 **구조적으로 불가능**하게 (계획 R4·R5 + Phase 0 숙제)
//!
//! 이 셸의 실질은 "진입점 추가"가 아니라, 아래 셋을 **도달 불가**로 만드는 것이다.
//! 셋 다 공통 성질이 **틀린 답이 정상처럼 보인다**는 것(무증상)이라, 검사를 더하는 게 아니라
//! 틀린 상태에 도달할 수 없게 만든다.
//!
//! | # | 무증상 실패 | 이 셸의 구조적 봉쇄 |
//! |---|---|---|
//! | 1 | `grid` ↔ [`Field2`] 차원 불일치 → **조용한 오독** | [`check_dims`] 를 조립 **직전** 통과해야만 `*Input` 이 만들어짐 |
//! | 2 | `m2_lub::solve_partial`(스텁, `phi_bl=0`) 오사용 → 건마모 소멸 | `partial_lub::` 만 import + 구조 가드 테스트 + 모델측 `#[deprecated]` |
//! | 3 | 미수렴 해가 정상 해와 구분 불가 | `_traced` 만 사용 + [`Diagnostics`] 를 **비-`Option`** 필드로 강제 |
//!
//! ### 왜 차원 불일치가 패닉이 아니라 오류 반환인가
//! [`Field2::at`] 은 `idx = i + j*self.nx` 로 **Field2 자신의 nx** 를 쓴다. 호출측은 `grid.nx` 로
//! 루프를 돈다. Field2 가 grid 보다 **크면** 인덱스가 범위 안에 들어와 **조용히 다른 원소를 읽는다**
//! (실측 확인). `debug_assert!(i < self.nx)` 는 `i < grid.nx ≤ self.nx` 라 **항상 통과**해 못 잡고,
//! release(= `wasm-pack --release`)에선 아예 컴파일 아웃된다. → 검사는 **경계에서** 해야 한다.
//!
//! 그리고 WASM 에서 패닉은 abort = 모듈 인스턴스 오염 → 이후 호출까지 죽는다. 사용자가 폼에 숫자를
//! 잘못 넣었다고 페이지를 새로고침하게 할 수는 없다 → 구조적 `{ok:false, error}` 로 반환한다.

use micropitting_model::m3_stress;
use micropitting_model::m4_fatigue::{self, FatigueParams};
use micropitting_model::m5_wear::{self, WearInput, WearParams};
// ★ R4: `partial_lub::` 만 import — `m2_lub::solve_partial`(스텁) 은 이 셸에서 **도달 불가**.
//   시그니처가 동일해 오사용해도 조용히 컴파일되므로, 구조 가드 테스트가 이 불변식을 강제한다.
use micropitting_model::partial_lub;
use micropitting_model::reference as refr;
use micropitting_model::types::{
    Field2, Grid, MaterialProps, OperatingConditions, PartialLubInput, PartialLubResult, WearResult,
};
use serde::{Deserialize, Serialize};

// ─────────────────────────────────────────────────────────────────────────
//  숙제 1 — 차원 검사 (조립 직전 관문)
// ─────────────────────────────────────────────────────────────────────────

/// `grid` 와 [`Field2`] 의 차원 일치 검사. **모든 `*Input` 조립 직전**에 통과해야 한다.
///
/// 불일치 시 조용한 오독(Field2 가 더 큰 경우) 또는 패닉(작은 경우)이 되므로, 여기서 잡아
/// 구조적 오류로 되돌린다(모듈 docstring 참조).
pub fn check_dims(name: &str, grid: &Grid, f: &Field2) -> Result<(), String> {
    if f.nx != grid.nx || f.ny != grid.ny {
        return Err(format!(
            "{name} 차원 불일치: field {}x{} vs grid {}x{} \
             (일치하지 않으면 Field2::at 이 조용히 다른 원소를 읽는다)",
            f.nx, f.ny, grid.nx, grid.ny
        ));
    }
    if f.data.len() != grid.nx * grid.ny {
        return Err(format!(
            "{name} data 길이 불일치: {} != nx*ny={}",
            f.data.len(),
            grid.nx * grid.ny
        ));
    }
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────
//  JSON 미러 — 비-serde / lifetime struct 만 (계획 §4.2)
// ─────────────────────────────────────────────────────────────────────────

/// [`WearParams`] JSON 미러 (원본은 serde 미파생, `m5_wear.rs:94-99`).
///
/// **전 필드 `Option`** = 미지정 시 [`WearParams::default`] 값 사용.
/// 기본값(k_lub·f_w)은 **모델 크레이트가 소유**한다 — 여기 상수를 두면 SSOT 이중화.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct WearParamsArgs {
    /// 윤활 마모계수 `k_lub` [-]. 미지정 → 모델 기본값(가정+민감도, RP-klub).
    pub k_lub: Option<f64>,
    /// 건/윤활 마모계수비 `f_w` [-]. 미지정 → 모델 기본값.
    pub f_w: Option<f64>,
}

impl WearParamsArgs {
    /// 모델 기본값 위에 지정된 필드만 override.
    fn to_params(&self) -> WearParams {
        let mut p = WearParams::default();
        if let Some(k) = self.k_lub {
            p.k_lub = k;
        }
        if let Some(f) = self.f_w {
            p.f_w = f;
        }
        p
    }
}

/// [`WearInput`] JSON 미러 (원본은 `p_tran: &'a Field2` lifetime 보유).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WearArgs {
    /// 계산 격자.
    pub grid: Grid,
    /// M6 전이 압력장 [Pa]. **소유값** — 셸에서 참조로 빌려 `WearInput` 조립.
    pub p_tran: Field2,
    /// 운전조건 (u_mean·slide_roll·r_x·p_h).
    pub op: OperatingConditions,
    /// 재료 물성 (hardness H·e_red).
    pub mat: MaterialProps,
    /// 경계윤활 하중분율 φ_bl [-] (M6 산출, 0~1).
    ///
    /// ⚠️ 계획 §1.3: `m2_lub::solve_partial`(스텁)은 `phi_bl=0` 을 낸다 → 건마모 소멸.
    /// Phase 2 에서 M6 를 결선할 때 **`partial_lub::solve_partial`** 산출만 여기 흘릴 것.
    pub phi_bl: f64,
    /// 마모 파라미터 override (미지정 → 모델 기본값).
    #[serde(default)]
    pub params: WearParamsArgs,
}

/// 성공/실패를 **명시적으로** 실어 나르는 응답 봉투.
///
/// 조용한 실패는 계획 R5(미수렴 해가 정상처럼 보임)와 같은 실패 모드 → `ok` 를 강제한다.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WearResponse {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<WearResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────────
//  코어 — 네이티브/WASM 공용 (기준 ② 값 일치 대조의 근거)
// ─────────────────────────────────────────────────────────────────────────

/// M5 진입점의 JSON 래퍼. **네이티브·wasm32 동일 코드**.
///
/// 이 함수가 `#[wasm_bindgen]` 과 분리되어 있어야 네이티브에서도 호출 가능하고,
/// 그래야 "WASM 값 == 네이티브 값" 을 **동일 코드로** 대조할 수 있다(계획 §3.2 기준 ②).
pub fn run_wear(input_json: &str) -> String {
    let resp = match wear_inner(input_json) {
        Ok(r) => WearResponse { ok: true, result: Some(r), error: None },
        Err(e) => WearResponse { ok: false, result: None, error: Some(e) },
    };
    // 직렬화 실패는 구조상 불가(전 필드 f64/usize) — 그래도 unwrap 대신 명시 폴백.
    serde_json::to_string(&resp)
        .unwrap_or_else(|e| format!(r#"{{"ok":false,"error":"serialize failed: {e}"}}"#))
}

fn wear_inner(input_json: &str) -> Result<WearResult, String> {
    let args: WearArgs =
        serde_json::from_str(input_json).map_err(|e| format!("input parse failed: {e}"))?;
    // ★ 숙제 1: 조립 **직전** 관문. 통과 못하면 WearInput 이 존재하지 않는다.
    check_dims("p_tran", &args.grid, &args.p_tran)?;
    let params = args.params.to_params();
    // lifetime struct 는 여기서 조립 — 경계를 넘지 않는다.
    let input = WearInput {
        grid: args.grid,
        p_tran: &args.p_tran,
        op: args.op,
        mat: args.mat,
        phi_bl: args.phi_bl,
    };
    Ok(m5_wear::solve_wear(&input, &params))
}

// ─────────────────────────────────────────────────────────────────────────
//  숙제 3 — 부분윤활(M1+M2+M6): 진단을 타입으로 강제
// ─────────────────────────────────────────────────────────────────────────

/// 수렴 진단. **`Option` 이 아니다** — 빼먹는 것이 타입상 불가능하다.
///
/// 크레이트에 로깅이 전무하므로 `PartialTrace`/`ShareTrace` 가 **유일한 진단 채널**이고,
/// 이것이 없으면 미수렴 해와 정상 해가 UI 에서 구분되지 않는다(계획 R5).
/// 핵심은 "trace 를 노출한다"가 아니라 **trace 없는 경로를 셸에서 없앤다**는 것 —
/// `partial_lub::solve_partial`(trace 버림) 은 여기서 쓰지 않는다.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostics {
    /// 외부 루프(φ_bl·p̄ 자기일관) 수렴 여부. **`false` 면 UI 는 경고 배지 필수**.
    pub outer_converged: bool,
    /// 내부 flow-balance bisection 수렴 여부. 동일.
    pub share_converged: bool,
    /// 외부 반복 횟수.
    pub outer_iters: usize,
    /// 내부 반복 횟수 (`SharePolicy::max_iter` 기본 200).
    pub share_iters: usize,
    /// 하중 보존 잔차 [-] (**회귀 가드**, 물리검증 아님 — CV-M6-Load).
    pub load_residual: f64,
    /// flow-balance 잔차 [-] (구성상 ≈0; 절차⑤ 항등).
    pub flow_balance_residual: f64,
    /// 유효 마찰계수 μ_eff [-].
    pub mu_eff: f64,
    /// 수렴 평균 접촉압 p̄ [Pa].
    pub p_bar: f64,
    /// 아스페리티 축퇴 플래그(M6-5).
    pub asperity_degenerate: bool,
    /// 아스페리티 접촉점 수.
    pub contact_count: usize,
}

/// [`PartialLubInput`] JSON 미러 (`Field2` 를 소유값으로 받아 차원 검사 후 조립).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PartialArgs {
    pub grid: Grid,
    /// 표면 1 거칠기 [m].
    pub rough1: Field2,
    /// 표면 2 거칠기 [m].
    pub rough2: Field2,
    pub mat: MaterialProps,
    pub op: OperatingConditions,
    /// 평균유막 h̄ [m] (Dowson-Toyoda 등으로 산출한 값).
    pub h_bar: f64,
}

/// 부분윤활 응답 — `diagnostics` 는 성공 시 **항상** 실린다.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialResponse {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<PartialLubResult>,
    /// 성공 시 반드시 존재. 실패 시에만 `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Diagnostics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// M1+M2+M6 부분윤활 완전결합. **`solve_partial_traced` 만 사용** → 진단 누락 불가.
pub fn run_partial(input_json: &str) -> String {
    let resp = match partial_inner(input_json) {
        // Diagnostics 가 비-Option 이라, 결과를 만들면 진단이 반드시 따라온다.
        Ok((r, d)) => PartialResponse {
            ok: true,
            result: Some(r),
            diagnostics: Some(d),
            error: None,
        },
        Err(e) => PartialResponse {
            ok: false,
            result: None,
            diagnostics: None,
            error: Some(e),
        },
    };
    serde_json::to_string(&resp)
        .unwrap_or_else(|e| format!(r#"{{"ok":false,"error":"serialize failed: {e}"}}"#))
}

fn partial_inner(input_json: &str) -> Result<(PartialLubResult, Diagnostics), String> {
    let args: PartialArgs =
        serde_json::from_str(input_json).map_err(|e| format!("input parse failed: {e}"))?;
    check_dims("rough1", &args.grid, &args.rough1)?;
    check_dims("rough2", &args.grid, &args.rough2)?;
    if !(args.h_bar > 0.0) {
        return Err(format!("h_bar 는 양수여야 한다: {}", args.h_bar));
    }
    let input = PartialLubInput {
        grid: args.grid,
        rough1: args.rough1,
        rough2: args.rough2,
        mat: args.mat,
        op: args.op,
        h_bar: args.h_bar,
    };
    // ★ R4: partial_lub::(진짜 오케스트레이터). m2_lub::solve_partial(스텁)이 아니다.
    // ★ R5: _traced — 진단을 버리는 경로를 애초에 쓰지 않는다.
    let (res, tr) = partial_lub::solve_partial_traced(&input);
    let d = Diagnostics {
        outer_converged: tr.outer_converged,
        share_converged: tr.share.converged,
        outer_iters: tr.outer_iters,
        share_iters: tr.share.iters,
        load_residual: tr.share.load_residual,
        flow_balance_residual: tr.share.flow_balance_residual,
        mu_eff: tr.mu_eff,
        p_bar: tr.p_bar,
        asperity_degenerate: tr.share.asperity_degenerate,
        contact_count: tr.share.contact_count,
    };
    Ok((res, d))
}

// ─────────────────────────────────────────────────────────────────────────
//  M3→M4 계량 (Phase 0 §3.3) — Phase 3 그리드 사이징 근거
// ─────────────────────────────────────────────────────────────────────────

/// M3→M4 체인 입력. **계량 목적** — 벽시계는 호출측(JS)이 잰다(크레이트에 시계 없음).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StressFatigueArgs {
    pub grid: Grid,
    /// 압력장 [Pa].
    pub p_tran: Field2,
    /// 접선 트랙션장 [Pa].
    pub q_tran: Field2,
    pub op: OperatingConditions,
    pub mat: MaterialProps,
    /// 깊이 층수. `solve_stress` 는 `NZ_DEFAULT` 고정이므로 `solve_stress_at_depths` 사용
    /// (계획 §4.2: 깊이 해상도 슬라이더는 이쪽으로만 가능).
    pub nz: usize,
}

/// M3→M4 요약 응답. 필드 전체를 반환하면 JSON 이 수십 MB → **요약만**(계량 목적).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StressFatigueSummary {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// 접촉 반폭 b [m] (`m3_stress::contact_half_width`).
    pub b: f64,
    pub nz: usize,
    /// 층별 최대 von Mises 중 최대 [Pa].
    pub max_von_mises: f64,
    /// 층별 최대 Dang Van 손상 D 중 최대 [-].
    pub max_dang_van_d: f64,
}

/// M3(응력) → M4(피로) 체인.
pub fn run_stress_fatigue(input_json: &str) -> String {
    match stress_fatigue_inner(input_json) {
        Ok(s) => serde_json::to_string(&s).unwrap_or_default(),
        Err(e) => serde_json::to_string(&StressFatigueSummary {
            ok: false,
            error: Some(e),
            b: 0.0,
            nz: 0,
            max_von_mises: 0.0,
            max_dang_van_d: 0.0,
        })
        .unwrap_or_default(),
    }
}

fn stress_fatigue_inner(input_json: &str) -> Result<StressFatigueSummary, String> {
    let args: StressFatigueArgs =
        serde_json::from_str(input_json).map_err(|e| format!("input parse failed: {e}"))?;
    // ★ 숙제 1: 두 하중장 모두 관문 통과.
    check_dims("p_tran", &args.grid, &args.p_tran)?;
    check_dims("q_tran", &args.grid, &args.q_tran)?;
    if args.nz == 0 {
        return Err("nz 는 1 이상이어야 한다".to_string());
    }

    // b·깊이는 모델이 소유(`contact_half_width` pub). 셸에서 재유도 금지 = SSOT.
    let b = m3_stress::contact_half_width(&args.op, &args.mat);
    // 0~0.25b 등간격 — `m3_stress::DEPTH_FRAC` 규약과 동일 의미. nz 만 가변.
    let z_depths: Vec<f64> = (0..args.nz)
        .map(|k| {
            let f = if args.nz <= 1 {
                0.0
            } else {
                k as f64 / (args.nz - 1) as f64
            };
            m3_stress::DEPTH_FRAC * b * f
        })
        .collect();

    let stress = m3_stress::solve_stress_at_depths(
        &args.grid,
        &args.p_tran,
        &args.q_tran,
        args.mat.nu,
        &z_depths,
    );
    let fatigue = m4_fatigue::solve_fatigue(&stress, &FatigueParams::default());

    // `Field2::max()` → Option<f64> (빈 격자 방어). 빈 층은 건너뛰고, 전 층이 비면 0.
    let max_vm = stress
        .von_mises
        .iter()
        .filter_map(|f| f.max())
        .fold(0.0_f64, f64::max);
    let max_d = fatigue
        .dang_van_d
        .iter()
        .filter_map(|f| f.max())
        .fold(0.0_f64, f64::max);

    Ok(StressFatigueSummary {
        ok: true,
        error: None,
        b,
        nz: args.nz,
        max_von_mises: max_vm,
        max_dang_van_d: max_d,
    })
}

// ─────────────────────────────────────────────────────────────────────────
//  S③ — 전체 체인(탭3): partial_lub → M3 → M4 → M5, 뷰어 슬라이스 반환
//
//  ★ 크기 대책(Phase 3 숙제 3): 6성분 전체 필드(수십 MB JSON)는 반환하지 않는다.
//    탭3 이 그리는 것만 — y₀ 슬라이스 σ_vM(x,z)·x-프로파일(p·h·q·Δh_w)·(y,z) Dang Van 맵.
//    (M4 의 D 는 x 방향 broadcast — x=시간이력이라 한 열당 스칼라 → (y,z) 맵이 정보 전부.)
//  ★ 정직성: `unwornGeometry: true` 플래그 동봉 — 원논문 Fig 6/15 는 "last wear step"(마모 후)
//    이므로 정적 체인 결과는 **미마모 형상·정성 전용(RP-Field)** 캡션 필수(불가침 5).
// ─────────────────────────────────────────────────────────────────────────

/// [`FatigueParams`] JSON 미러 (serde 미파생 → Option override, 기본값은 모델 소유).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FatigueParamsArgs {
    pub wohler_a: Option<f64>,
    pub wohler_b: Option<f64>,
    pub n_ref: Option<f64>,
    pub alpha_dv: Option<f64>,
}

impl FatigueParamsArgs {
    fn to_params(&self) -> FatigueParams {
        let mut p = FatigueParams::default();
        if let Some(v) = self.wohler_a { p.wohler_a = v; }
        if let Some(v) = self.wohler_b { p.wohler_b = v; }
        if let Some(v) = self.n_ref { p.n_ref = v; }
        if let Some(v) = self.alpha_dv { p.alpha_dv = v; }
        p
    }
}

/// 전체 체인 입력 = 부분윤활 입력 + 깊이/슬라이스/파라미터 옵션.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChainArgs {
    pub grid: Grid,
    pub rough1: Field2,
    pub rough2: Field2,
    pub mat: MaterialProps,
    pub op: OperatingConditions,
    pub h_bar: f64,
    /// 깊이층 수 (기본 = 모델 `NZ_DEFAULT`).
    #[serde(default = "d_nz")]
    pub nz: usize,
    /// σ_vM(x,z) 슬라이스의 y 인덱스 (기본 ny/2).
    #[serde(default)]
    pub slice_j: Option<usize>,
    #[serde(default)]
    pub fatigue: FatigueParamsArgs,
    #[serde(default)]
    pub wear: WearParamsArgs,
}
fn d_nz() -> usize { m3_stress::NZ_DEFAULT }

/// 전체 체인 응답 — 뷰어가 그릴 것만.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainResp {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// ★ 정직성: 정적 체인 = 미마모 형상. 뷰어 캡션 필수(RP-Field 정성 전용).
    pub unworn_geometry: bool,
    /// 진단 — 성공 시 항상(R5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Diagnostics>,
    pub b: f64,
    pub phi_bl: f64,
    pub dh_w_mean: f64,
    pub slice_j: usize,
    /// x 좌표 [m] (nx).
    pub x: Vec<f64>,
    /// 깊이 [m] (nz).
    pub z_depths: Vec<f64>,
    /// y₀ x-프로파일 (nx): 전이압/유막/트랙션/마모율.
    pub p_tran_profile: Vec<f64>,
    pub h_tran_profile: Vec<f64>,
    pub q_tran_profile: Vec<f64>,
    pub dh_w_profile: Vec<f64>,
    /// σ_vM(x,z) @ y₀ — row-major [z][x] (nz×nx).
    pub vm_xz: Vec<Vec<f64>>,
    /// Dang Van D(y,z) — row-major [z][y] (nz×ny; x broadcast 라 (y,z) 가 정보 전부).
    pub dv_yz: Vec<Vec<f64>>,
    /// 수명 N(y,z) — 동일 배치.
    pub life_yz: Vec<Vec<f64>>,
}

fn chain_err(e: String) -> String {
    serde_json::to_string(&serde_json::json!({ "ok": false, "error": e })).unwrap_or_default()
}

/// 전체 체인: 부분윤활 → 응력 → 피로 → 마모. **모든 물리는 모델 크레이트**(셸은 배선·슬라이스만).
pub fn run_chain(input_json: &str) -> String {
    match chain_inner(input_json) {
        Ok(r) => serde_json::to_string(&r).unwrap_or_default(),
        Err(e) => chain_err(e),
    }
}

fn chain_inner(input_json: &str) -> Result<ChainResp, String> {
    let args: ChainArgs =
        serde_json::from_str(input_json).map_err(|e| format!("input parse failed: {e}"))?;
    check_dims("rough1", &args.grid, &args.rough1)?;
    check_dims("rough2", &args.grid, &args.rough2)?;
    if !(args.h_bar > 0.0) {
        return Err(format!("h_bar 는 양수여야 한다: {}", args.h_bar));
    }
    if args.nz == 0 {
        return Err("nz 는 1 이상".into());
    }
    let (nx, ny) = (args.grid.nx, args.grid.ny);
    let j0 = args.slice_j.unwrap_or(ny / 2);
    if j0 >= ny {
        return Err(format!("slice_j={j0} >= ny={ny}"));
    }

    // ① 부분윤활 (R4: partial_lub:: 진짜 오케스트레이터 · R5: _traced)
    let input = PartialLubInput {
        grid: args.grid,
        rough1: args.rough1,
        rough2: args.rough2,
        mat: args.mat,
        op: args.op,
        h_bar: args.h_bar,
    };
    let (part, tr) = partial_lub::solve_partial_traced(&input);

    // ② M3 — 깊이는 모델 소유 상수·contact_half_width 로 (셸 재유도 금지 = SSOT)
    let b = m3_stress::contact_half_width(&args.op, &args.mat);
    let z_depths: Vec<f64> = (0..args.nz)
        .map(|k| {
            let f = if args.nz <= 1 { 0.0 } else { k as f64 / (args.nz - 1) as f64 };
            m3_stress::DEPTH_FRAC * b * f
        })
        .collect();
    let stress = m3_stress::solve_stress_at_depths(
        &args.grid,
        &part.p_tran,
        &part.q_tran,
        args.mat.nu,
        &z_depths,
    );

    // ③ M4
    let fat = m4_fatigue::solve_fatigue(&stress, &args.fatigue.to_params());

    // ④ M5
    let wear = m5_wear::solve_wear(
        &WearInput {
            grid: args.grid,
            p_tran: &part.p_tran,
            op: args.op,
            mat: args.mat,
            phi_bl: part.phi_bl,
        },
        &args.wear.to_params(),
    );

    // ⑤ 슬라이스 (그릴 것만)
    let dx = args.grid.dx();
    let x: Vec<f64> = (0..nx).map(|i| i as f64 * dx).collect();
    let prof = |f: &Field2| -> Vec<f64> { (0..nx).map(|i| f.at(i, j0)).collect() };
    let vm_xz: Vec<Vec<f64>> = stress
        .von_mises
        .iter()
        .map(|layer| (0..nx).map(|i| layer.at(i, j0)).collect())
        .collect();
    // D·N 은 x broadcast → x=0 열로 (y,z) 맵 구성.
    let dv_yz: Vec<Vec<f64>> =
        fat.dang_van_d.iter().map(|layer| (0..ny).map(|j| layer.at(0, j)).collect()).collect();
    let life_yz: Vec<Vec<f64>> =
        fat.life_n.iter().map(|layer| (0..ny).map(|j| layer.at(0, j)).collect()).collect();

    Ok(ChainResp {
        ok: true,
        error: None,
        unworn_geometry: true,
        diagnostics: Some(Diagnostics {
            outer_converged: tr.outer_converged,
            share_converged: tr.share.converged,
            outer_iters: tr.outer_iters,
            share_iters: tr.share.iters,
            load_residual: tr.share.load_residual,
            flow_balance_residual: tr.share.flow_balance_residual,
            mu_eff: tr.mu_eff,
            p_bar: tr.p_bar,
            asperity_degenerate: tr.share.asperity_degenerate,
            contact_count: tr.share.contact_count,
        }),
        b,
        phi_bl: part.phi_bl,
        dh_w_mean: wear.dh_w_mean,
        slice_j: j0,
        x,
        z_depths,
        p_tran_profile: prof(&part.p_tran),
        h_tran_profile: prof(&part.h_tran),
        q_tran_profile: prof(&part.q_tran),
        dh_w_profile: prof(&wear.dh_w),
        vm_xz,
        dv_yz,
        life_yz,
    })
}

// ─────────────────────────────────────────────────────────────────────────
//  S② — reference(leaf) 노출: 곡선 샘플러 + 정적 데이터 (계획 탭1 참조곡선)
//
//  ★ 역할: 수식 평가는 전부 `micropitting_model::reference` 호출. 셸은 **샘플링 루프만**
//    (x 격자 생성·반복은 물리가 아니다). JS 에는 완성된 배열만 넘어간다(R8: JS 물리 0건).
//  ★ leaf 무관: 금지는 "m1~m6 생산코드 → reference" 방향이다. 셸(소비자)의 사용이 용도다.
//  ★ 스칼라 함수 개별 노출 대신 곡선 단위 반환 = 경계 호출 수천회 회피(계획 Phase3 숙제 1).
// ─────────────────────────────────────────────────────────────────────────

/// 곡선 한 줄 (플로팅 시리즈).
#[derive(Debug, Serialize)]
pub struct Series {
    pub name: String,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

/// 참조곡선 응답. `meta` 는 곡선별 부속 데이터(표·앵커·피크 등).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurveResp {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub kind: String,
    pub series: Vec<Series>,
    pub meta: serde_json::Value,
}

fn curve_err(kind: &str, e: String) -> String {
    serde_json::to_string(&CurveResp {
        ok: false,
        error: Some(e),
        kind: kind.to_string(),
        series: vec![],
        meta: serde_json::Value::Null,
    })
    .unwrap_or_default()
}

/// x 샘플 격자(선형/로그). 물리 아님 — 순수 좌표 생성.
fn sample_axis(min: f64, max: f64, n: usize, log: bool) -> Result<Vec<f64>, String> {
    if n < 2 || !(max > min) {
        return Err(format!("잘못된 축: min={min}, max={max}, n={n}"));
    }
    if log && !(min > 0.0) {
        return Err(format!("로그축은 min>0 필요: {min}"));
    }
    Ok((0..n)
        .map(|i| {
            let t = i as f64 / (n - 1) as f64;
            if log {
                (min.ln() + t * (max.ln() - min.ln())).exp()
            } else {
                min + t * (max - min)
            }
        })
        .collect())
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Venner97Req {
    #[serde(default = "d_lob_min")]
    lob_min: f64,
    #[serde(default = "d_lob_max")]
    lob_max: f64,
    #[serde(default = "d_n")]
    n: usize,
    /// 선접촉 M (기본 = Venner1997 Table1 사례).
    #[serde(default = "d_m97")]
    m: f64,
    #[serde(default = "d_l97")]
    l: f64,
}
fn d_lob_min() -> f64 { 0.05 }
fn d_lob_max() -> f64 { 8.0 }
fn d_n() -> usize { 200 }
fn d_m97() -> f64 { refr::VENNER1997_M }
fn d_l97() -> f64 { refr::VENNER1997_L }

fn curve_venner1997(params: &str) -> Result<CurveResp, String> {
    let q: Venner97Req = serde_json::from_str(params).map_err(|e| e.to_string())?;
    let lob = sample_axis(q.lob_min, q.lob_max, q.n, true)?;
    let nabla: Vec<f64> = lob.iter().map(|&v| refr::venner1997_nabla(v, q.m, q.l)).collect();
    let y: Vec<f64> = nabla.iter().map(|&nb| refr::venner1997_amplitude_reduction(nb)).collect();
    // 원문 자인 과소예측역(0.5<A_d/A_i<1) 마스크 — 뷰어 음영용.
    let degrade: Vec<f64> =
        y.iter().map(|&v| if refr::venner1997_fit_degrades(v) { 1.0 } else { 0.0 }).collect();
    Ok(CurveResp {
        ok: true,
        error: None,
        kind: "venner1997".into(),
        series: vec![
            Series { name: "eq5".into(), x: lob.clone(), y },
            Series { name: "fitDegradesMask".into(), x: lob.clone(), y: degrade },
        ],
        meta: serde_json::json!({
            "m": q.m, "l": q.l,
            "nabla": nabla,
            "table1": refr::VENNER1997_TABLE1.iter()
                .map(|(lb, a)| serde_json::json!({"lob": lb, "adAi": a}))
                .collect::<Vec<_>>(),
            "table1Cols": [0.1, 0.2, 0.5],
            "anchorSpots": refr::VENNER1997_ANCHOR_SPOTS,
            "halfCrossingBracket": refr::VENNER1997_HALF_CROSSING_BRACKET,
            "lineContactOnly": true, // ★ 점접촉(venner2000)과 축 겹침 금지 (총괄계획 L476)
        }),
    })
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Venner2000Req {
    #[serde(default = "d_n2_min")]
    nabla2_min: f64,
    #[serde(default = "d_n2_max")]
    nabla2_max: f64,
    #[serde(default = "d_n")]
    n: usize,
    /// 이방성비 r = λx/λy (기본 1 = 등방).
    #[serde(default = "d_r")]
    r: f64,
}
fn d_n2_min() -> f64 { 0.01 }
fn d_n2_max() -> f64 { 100.0 }
fn d_r() -> f64 { 1.0 }

fn curve_venner2000(params: &str) -> Result<CurveResp, String> {
    let q: Venner2000Req = serde_json::from_str(params).map_err(|e| e.to_string())?;
    let x = sample_axis(q.nabla2_min, q.nabla2_max, q.n, true)?;
    let fb = refr::venner2000_f_bar(q.r);
    let y: Vec<f64> = x.iter().map(|&nb| refr::venner2000_amplitude_reduction(nb, fb)).collect();
    let (m, l, loa) = refr::VENNER2000_EXAMPLE;
    let ex_nb = refr::venner2000_nabla2(loa, m, l);
    Ok(CurveResp {
        ok: true,
        error: None,
        kind: "venner2000".into(),
        series: vec![Series { name: "eq29".into(), x, y }],
        meta: serde_json::json!({
            "fBar": fb, "r": q.r,
            "example": { "m": m, "l": l, "lamOverA": loa, "nabla2": ex_nb,
                          "eq29": refr::venner2000_amplitude_reduction(ex_nb, 1.0),
                          "numerics": refr::VENNER2000_EXAMPLE_NUMERICS },
            "pointContactOnly": true, // ★ 선접촉(venner1997)과 축 겹침 금지
        }),
    })
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Gw1994Req {
    #[serde(default = "d_lam_min")]
    lambda_min: f64,
    #[serde(default = "d_lam_max")]
    lambda_max: f64,
    #[serde(default = "d_n")]
    n: usize,
    /// 압축성항 C (VC-M2-AR 대표값 기본).
    #[serde(default = "d_c")]
    c: f64,
    /// 압점도 [1/Pa] (기본 = GW Table2 Kweh 50℃).
    #[serde(default = "d_alpha")]
    alpha_visc: f64,
    /// 평균유막 [m] (동).
    #[serde(default = "d_hbar")]
    h_bar: f64,
    /// 논문 E' [Pa] (= 2·E_red).
    #[serde(default = "d_ep")]
    e_prime: f64,
}
fn d_lam_min() -> f64 { 1e-6 }
fn d_lam_max() -> f64 { 1e-3 }
fn d_c() -> f64 { 0.03 }
fn d_alpha() -> f64 { refr::GW1994_TABLE2_KWEH[0].0 }
fn d_hbar() -> f64 { refr::GW1994_TABLE2_KWEH[0].1 }
fn d_ep() -> f64 { refr::GW1994_E_PRIME_PA }

fn curve_gw1994(params: &str) -> Result<CurveResp, String> {
    let q: Gw1994Req = serde_json::from_str(params).map_err(|e| e.to_string())?;
    let lam = sample_axis(q.lambda_min, q.lambda_max, q.n, true)?;
    let a: Vec<f64> =
        lam.iter().map(|&v| refr::gw1994_a(v, q.alpha_visc, q.h_bar, q.e_prime)).collect();
    let h: Vec<f64> = a.iter().map(|&av| refr::gw1994_h1_over_z1(q.c, av)).collect();
    let p: Vec<f64> = a.iter().map(|&av| refr::gw1994_p1_over_z1(q.c, av).abs()).collect();
    Ok(CurveResp {
        ok: true,
        error: None,
        kind: "gw1994".into(),
        series: vec![
            Series { name: "h1OverZ1".into(), x: lam.clone(), y: h },
            Series { name: "absP1OverZ1".into(), x: lam.clone(), y: p },
        ],
        meta: serde_json::json!({
            "c": q.c, "alphaVisc": q.alpha_visc, "hBar": q.h_bar, "ePrime": q.e_prime,
            "table1Present": refr::GW1994_TABLE1_PRESENT,
            "table2Kweh": refr::GW1994_TABLE2_KWEH,
            "lambdaM": refr::GW1994_LAMBDA_M, "z1M": refr::GW1994_Z1_M,
        }),
    })
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct McewenReq {
    #[serde(default = "d_nu")]
    nu: f64,
    #[serde(default = "d_zb_max")]
    zb_max: f64,
    #[serde(default = "d_n")]
    n: usize,
}
fn d_nu() -> f64 { 0.3 }
fn d_zb_max() -> f64 { 1.2 }

fn curve_mcewen(params: &str) -> Result<CurveResp, String> {
    let q: McewenReq = serde_json::from_str(params).map_err(|e| e.to_string())?;
    let x = sample_axis(1e-3, q.zb_max, q.n, false)?;
    let y: Vec<f64> = x.iter().map(|&z| refr::mcewen_von_mises_over_p0(z, q.nu)).collect();
    let (pk_vm, pk_z) = refr::mcewen_von_mises_peak(q.nu, 1e-3);
    Ok(CurveResp {
        ok: true,
        error: None,
        kind: "mcewen".into(),
        series: vec![Series { name: "vmOverP0".into(), x, y }],
        meta: serde_json::json!({ "nu": q.nu, "peak": { "vm": pk_vm, "zOverB": pk_z } }),
    })
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct MilanoReq {
    /// σ_a [Pa] (기본 = VC-M4-Milano 오라클 값).
    #[serde(default = "d_sa")]
    sigma_a: f64,
    #[serde(default = "d_ta")]
    tau_a: f64,
    #[serde(default = "d_n")]
    n: usize,
}
fn d_sa() -> f64 { 200.0e6 }
fn d_ta() -> f64 { 150.0e6 }

fn curve_milano(params: &str) -> Result<CurveResp, String> {
    let q: MilanoReq = serde_json::from_str(params).map_err(|e| e.to_string())?;
    let x = sample_axis(0.0, 2.0 * std::f64::consts::PI, q.n, false)?;
    let tau: Vec<f64> =
        x.iter().map(|&wt| refr::milano2006_tau_dv(q.sigma_a, q.tau_a, wt)).collect();
    let sh: Vec<f64> = x.iter().map(|&wt| refr::milano2006_sigma_h(q.sigma_a, wt)).collect();
    Ok(CurveResp {
        ok: true,
        error: None,
        kind: "milano".into(),
        series: vec![
            Series { name: "tauDv".into(), x: x.clone(), y: tau },
            Series { name: "sigmaH".into(), x: x.clone(), y: sh },
        ],
        meta: serde_json::json!({ "sigmaA": q.sigma_a, "tauA": q.tau_a }),
    })
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct TrippReq {
    #[serde(default = "d_p0")]
    p0: f64,
    #[serde(default)]
    q0: f64,
    /// 파수 α, β [1/m] (기본 = VC-M3-Sin 오라클 격자).
    #[serde(default = "d_alpha_w")]
    alpha: f64,
    #[serde(default = "d_beta_w")]
    beta: f64,
    #[serde(default = "d_nu")]
    nu: f64,
    /// 평가 위상 (기본: cc=1 위치 x=y=0 — 법선 성분 최대 가시화).
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    /// 깊이 최대 [1/ζ 배수] (기본 3/ζ).
    #[serde(default = "d_zmul")]
    z_max_over_zeta: f64,
    #[serde(default = "d_n")]
    n: usize,
}
fn d_p0() -> f64 { 1.0e9 }
fn d_alpha_w() -> f64 { 2.0 * std::f64::consts::PI * 3.0 / 1.0e-4 }
fn d_beta_w() -> f64 { 2.0 * std::f64::consts::PI * 2.0 / 1.0e-4 }
fn d_zmul() -> f64 { 3.0 }

fn curve_tripp(params: &str) -> Result<CurveResp, String> {
    let q: TrippReq = serde_json::from_str(params).map_err(|e| e.to_string())?;
    let zeta = (q.alpha * q.alpha + q.beta * q.beta).sqrt();
    if !(zeta > 0.0) {
        return Err("alpha·beta 파수가 0".into());
    }
    let z = sample_axis(0.0, q.z_max_over_zeta / zeta, q.n, false)?;
    const NAMES: [&str; 6] = ["sxx", "syy", "szz", "sxy", "syz", "sxz"];
    let mut comp: [Vec<f64>; 6] = Default::default();
    let mut trace: Vec<f64> = Vec::with_capacity(z.len());
    for &zz in &z {
        let sn = refr::tripp2003_normal_bisin(q.p0, q.alpha, q.beta, q.nu, q.x, q.y, zz);
        let st = refr::tripp2003_tangential_bisin(q.q0, q.alpha, q.beta, q.nu, q.x, q.y, zz);
        for c in 0..6 {
            comp[c].push(sn[c] + st[c]);
        }
        trace.push(
            refr::tripp2003_trace_normal(q.p0, q.alpha, q.beta, q.nu, q.x, q.y, zz)
                + refr::tripp2003_trace_tangential(q.q0, q.alpha, q.beta, q.nu, q.x, q.y, zz),
        );
    }
    let mut series: Vec<Series> = comp
        .into_iter()
        .zip(NAMES)
        .map(|(y, nm)| Series { name: nm.into(), x: z.clone(), y })
        .collect();
    series.push(Series { name: "traceIdentity".into(), x: z.clone(), y: trace });
    Ok(CurveResp {
        ok: true,
        error: None,
        kind: "tripp2003".into(),
        series,
        meta: serde_json::json!({ "zeta": zeta, "p0": q.p0, "q0": q.q0, "nu": q.nu }),
    })
}

/// 참조곡선 디스패처: `kind` ∈ venner1997 | venner2000 | gw1994 | mcewen | milano | tripp2003.
pub fn run_reference_curve(kind: &str, params_json: &str) -> String {
    let params = if params_json.trim().is_empty() { "{}" } else { params_json };
    let res = match kind {
        "venner1997" => curve_venner1997(params),
        "venner2000" => curve_venner2000(params),
        "gw1994" => curve_gw1994(params),
        "mcewen" => curve_mcewen(params),
        "milano" => curve_milano(params),
        "tripp2003" => curve_tripp(params),
        other => Err(format!("unknown kind: {other}")),
    };
    match res {
        Ok(r) => serde_json::to_string(&r).unwrap_or_default(),
        Err(e) => curve_err(kind, e),
    }
}

/// 정적 문헌 데이터 일괄(JSON) — 표·상수·실험치. 곡선이 아닌 것 전부.
pub fn run_reference_tables() -> String {
    serde_json::json!({
        "ok": true,
        "venner1997": {
            "m": refr::VENNER1997_M, "l": refr::VENNER1997_L,
            "table1": refr::VENNER1997_TABLE1.iter()
                .map(|(lb, a)| serde_json::json!({"lob": lb, "adAi": a}))
                .collect::<Vec<_>>(),
            "table1Cols": [0.1, 0.2, 0.5],
            "anchorSpots": refr::VENNER1997_ANCHOR_SPOTS,
            "halfCrossingBracket": refr::VENNER1997_HALF_CROSSING_BRACKET,
        },
        "venner2000": {
            "example": refr::VENNER2000_EXAMPLE, "numerics": refr::VENNER2000_EXAMPLE_NUMERICS,
        },
        "gw1994": {
            "table1Present": refr::GW1994_TABLE1_PRESENT,
            "table2Kweh": refr::GW1994_TABLE2_KWEH,
            "ePrime": refr::GW1994_E_PRIME_PA, "lambda": refr::GW1994_LAMBDA_M,
            "z1": refr::GW1994_Z1_M,
            "halfPumpingG": refr::GW1994_HALF_PUMPING_G,
        },
        "archard1953": {
            "fig7SlopeBrass": refr::ARCHARD1953_FIG7_SLOPE_BRASS,
            "fig7SlopeStellite": refr::ARCHARD1953_FIG7_SLOPE_STELLITE,
            "fig7SlopeStdErr": refr::ARCHARD1953_FIG7_SLOPE_STD_ERR,
        },
        "desimone2006": {
            "rRatioTable": refr::DESIMONE2006_R_RATIO_TABLE,
            "alphaDvVonMises": refr::desimone2006_alpha_dv(1.0 / 3.0_f64.sqrt()),
        },
    })
    .to_string()
}

// ─────────────────────────────────────────────────────────────────────────
//  WASM 경계 — wasm32 에서만
// ─────────────────────────────────────────────────────────────────────────

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// M5 경마모: `WearArgs` JSON → `WearResponse` JSON.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn solve_wear_json(input_json: &str) -> String {
    run_wear(input_json)
}

/// M3→M4 체인: `StressFatigueArgs` JSON → `StressFatigueSummary` JSON.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn solve_stress_fatigue_json(input_json: &str) -> String {
    run_stress_fatigue(input_json)
}

/// 부분윤활 M1+M2+M6: `PartialArgs` JSON → `PartialResponse` JSON (**diagnostics 항상 포함**).
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn solve_partial_json(input_json: &str) -> String {
    run_partial(input_json)
}

/// 참조곡선: kind + params JSON → CurveResp JSON.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn reference_curve_json(kind: &str, params_json: &str) -> String {
    run_reference_curve(kind, params_json)
}

/// 정적 문헌 데이터 일괄 JSON.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn reference_tables_json() -> String {
    run_reference_tables()
}

/// 전체 체인(탭3): ChainArgs JSON → ChainResp JSON (뷰어 슬라이스만, unwornGeometry 플래그).
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn solve_chain_json(input_json: &str) -> String {
    run_chain(input_json)
}

// ═════════════════════════════════════════════════════════════════════════
//  구조 가드 — 무증상 실패의 봉쇄를 기계적으로 강제
// ═════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    /// 생산 코드(주석 제외) 줄. Phase 1 `reference.rs` 가드와 동일 패턴(검증된 것 재사용).
    fn production_code_lines(src: &str) -> impl Iterator<Item = &str> {
        let prod = match src.find("#[cfg(test)]") {
            Some(i) => &src[..i],
            None => src,
        };
        prod.lines().map(|l| l.trim()).filter(|l| !l.starts_with("//"))
    }

    /// ★ R4 봉쇄: 셸 생산코드가 **스텁 `m2_lub::solve_partial`** 에 도달하지 않는다.
    ///
    /// 두 `solve_partial` 은 시그니처가 같아 오사용해도 조용히 컴파일된다. `phi_bl=0` 이 M5 로
    /// 흘러가면 건마모가 사라진 결과가 정상처럼 보인다(무증상). 모델측 `#[deprecated]` 가
    /// 경고를 내지만, 경고는 무시될 수 있으므로 여기서 **기계적으로 막는다**.
    #[test]
    fn shell_never_reaches_m2_lub_stub() {
        for line in production_code_lines(include_str!("lib.rs")) {
            assert!(
                !line.contains("m2_lub"),
                "R4 위반: 셸 생산코드가 m2_lub 를 참조 — solve_partial 스텁(phi_bl=0) 도달 위험. 줄: {line}"
            );
        }
    }

    /// ★ R5 봉쇄: 셸은 **진단을 버리는 경로**(`solve_partial` 비-traced)를 쓰지 않는다.
    #[test]
    fn shell_uses_only_traced_solver() {
        for line in production_code_lines(include_str!("lib.rs")) {
            assert!(
                !line.contains("partial_lub::solve_partial(") ,
                "R5 위반: trace 를 버리는 solve_partial 사용 — 미수렴이 UI 에서 안 보인다. 줄: {line}"
            );
        }
    }

    // ── 숙제 1: 차원 검사 ──

    #[test]
    fn check_dims_accepts_matching() {
        let g = Grid::new(4, 2, 1e-4, 1e-4);
        assert!(check_dims("p", &g, &Field2::zeros(4, 2)).is_ok());
    }

    /// Field2 가 grid 보다 **작은** 경우 — 방치하면 인덱스 초과 → 패닉(시끄러운 실패).
    #[test]
    fn check_dims_rejects_smaller_field() {
        let g = Grid::new(64, 16, 1e-4, 1e-4);
        let e = check_dims("p_tran", &g, &Field2::zeros(2, 1)).unwrap_err();
        assert!(e.contains("차원 불일치"), "{e}");
    }

    /// ★ Field2 가 grid 보다 **큰** 경우 — 방치하면 **조용히 다른 원소를 읽는다**(무증상).
    ///
    /// `debug_assert!(i < self.nx)` 는 `i < grid.nx ≤ self.nx` 라 통과하고, release 에선
    /// 아예 컴파일 아웃된다 → 경계 검사만이 유일한 방어.
    #[test]
    fn check_dims_rejects_larger_field_silent_misread() {
        let g = Grid::new(2, 2, 1e-4, 1e-4);
        let e = check_dims("p_tran", &g, &Field2::zeros(4, 4)).unwrap_err();
        assert!(e.contains("차원 불일치"), "{e}");
    }

    /// 차원은 맞지만 `data` 길이가 깨진 경우(수동 JSON 조립 사고).
    #[test]
    fn check_dims_rejects_bad_data_len() {
        let g = Grid::new(2, 2, 1e-4, 1e-4);
        let bad = Field2 { nx: 2, ny: 2, data: vec![0.0; 3] };
        let e = check_dims("p", &g, &bad).unwrap_err();
        assert!(e.contains("data 길이"), "{e}");
    }

    /// 진입점이 실제로 관문을 통과시키는지 — 패닉이 아니라 `{ok:false}` 여야 한다.
    #[test]
    fn run_wear_rejects_dim_mismatch_without_panic() {
        let json = r#"{
            "grid": {"nx":64,"ny":16,"lx":4e-5,"ly":2e-5},
            "p_tran": {"nx":2,"ny":1,"data":[1.5e9,0.75e9]},
            "op": {"p_h":1.5e9,"u_mean":1.0,"u2":1.01,"slide_roll":0.02,"eta0":0.0094,
                   "alpha_visc":2.078e-8,"tau0":3e6,"temp":348.15,"r_x":0.01},
            "mat": {"e_red":1.15384615384615e11,"nu":0.3,"hardness":7e9,"p_lim":4e9},
            "phi_bl": 0.3,
            "params": {}
        }"#;
        let out = run_wear(json);
        assert!(out.contains("\"ok\":false"), "{out}");
        assert!(out.contains("차원 불일치"), "{out}");
    }
    // ── S②: reference 노출 스모크 (수식 자체는 reference.rs 오라클 19건이 담당;
    //        여기는 경계 pass-through 등가성·오류경로만) ──

    #[test]
    fn s2_venner1997_curve_matches_reference_fn() {
        let out = run_reference_curve("venner1997", r#"{"lobMin":0.5,"lobMax":1.0,"n":2}"#);
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["ok"], true, "{out}");
        // 경계 pass-through 등가성: y[0] == reference 직접 호출값 (bit-exact).
        let y0 = v["series"][0]["y"][0].as_f64().unwrap();
        let expect = refr::venner1997_amplitude_reduction(refr::venner1997_nabla(
            0.5,
            refr::VENNER1997_M,
            refr::VENNER1997_L,
        ));
        assert_eq!(y0, expect, "pass-through 불일치");
        // 앵커 스팟·브래킷 meta 동봉 확인 (탭1 데이터 완비성).
        assert_eq!(v["meta"]["anchorSpots"].as_array().unwrap().len(), 3);
        assert_eq!(v["meta"]["lineContactOnly"], true);
    }

    #[test]
    fn s2_mcewen_peak_in_meta() {
        let out = run_reference_curve("mcewen", "{}");
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["ok"], true, "{out}");
        let vm = v["meta"]["peak"]["vm"].as_f64().unwrap();
        assert!((vm - 0.557).abs() < 0.01, "McEwen peak {vm}");
    }

    #[test]
    fn s2_tables_json_complete() {
        let out = run_reference_tables();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["ok"], true);
        assert_eq!(v["venner1997"]["table1"].as_array().unwrap().len(), 6, "Table1 6행(18점)");
        assert_eq!(v["archard1953"]["fig7SlopeBrass"], 1.0);
        // a_DV 값이 SKF 채택값과 일치(문헌 교차검증 값 그대로 노출되는지).
        let a = v["desimone2006"]["alphaDvVonMises"].as_f64().unwrap();
        assert!((a - 0.232_050_807_568_877_2).abs() < 1e-12);
    }

    #[test]
    fn s2_unknown_kind_and_bad_axis_are_structured_errors() {
        let v: serde_json::Value =
            serde_json::from_str(&run_reference_curve("nope", "{}")).unwrap();
        assert_eq!(v["ok"], false);
        // 로그축 min<=0 거부.
        let v2: serde_json::Value = serde_json::from_str(&run_reference_curve(
            "venner1997",
            r#"{"lobMin":0.0,"lobMax":1.0,"n":10}"#,
        ))
        .unwrap();
        assert_eq!(v2["ok"], false);
    }
    // ── S③: 전체 체인 스모크 ──

    fn chain_json(nx: usize, ny: usize) -> String {
        let rough: Vec<f64> = (0..nx * ny)
            .map(|k| 0.23e-6 * (2.0 * std::f64::consts::PI * 6.0 * (k % nx) as f64 / nx as f64).sin())
            .collect();
        serde_json::json!({
            "grid": {"nx": nx, "ny": ny, "lx": 5.2e-4, "ly": 1.3e-4},
            "rough1": {"nx": nx, "ny": ny, "data": rough},
            "rough2": {"nx": nx, "ny": ny, "data": (0..nx * ny)
                .map(|k| 0.06e-6 * (2.0 * std::f64::consts::PI * 6.0 * (k % nx) as f64 / nx as f64).sin())
                .collect::<Vec<f64>>()},
            "mat": {"e_red": 1.15384615384615e11, "nu": 0.3, "hardness": 7e9, "p_lim": 4e9},
            "op": {"p_h": 1.5e9, "u_mean": 1.0, "u2": 1.01, "slide_roll": 0.02,
                    "eta0": 0.0094, "alpha_visc": 2.078e-8, "tau0": 3e6, "temp": 348.15, "r_x": 0.01},
            "h_bar": 1.4e-7
        })
        .to_string()
    }

    #[test]
    fn s3_chain_returns_slices_with_honesty_flags() {
        let out = run_chain(&chain_json(32, 8));
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["ok"], true, "{out}");
        // 정직성 플래그·진단 필수.
        assert_eq!(v["unwornGeometry"], true, "미마모 캡션 플래그");
        assert_eq!(v["diagnostics"]["outerConverged"], true);
        // 차원: vm_xz = nz×nx, dv_yz = nz×ny.
        let nz = v["zDepths"].as_array().unwrap().len();
        assert_eq!(nz, m3_stress::NZ_DEFAULT);
        assert_eq!(v["vmXz"].as_array().unwrap().len(), nz);
        assert_eq!(v["vmXz"][0].as_array().unwrap().len(), 32);
        assert_eq!(v["dvYz"][0].as_array().unwrap().len(), 8);
        // phi_bl ≠ 0 (스텁 아님, R4).
        assert!(v["phiBl"].as_f64().unwrap() != 0.0);
        // 프로파일 길이.
        assert_eq!(v["pTranProfile"].as_array().unwrap().len(), 32);
    }

    #[test]
    fn s3_chain_rejects_bad_inputs_structurally() {
        // slice_j 범위 밖.
        let mut v: serde_json::Value = serde_json::from_str(&chain_json(16, 4)).unwrap();
        v["sliceJ"] = serde_json::json!(99);
        // ChainArgs 는 snake_case 필드 → sliceJ 는 unknown → 구조적 거부(deny_unknown_fields).
        let out = run_chain(&v.to_string());
        let r: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(r["ok"], false, "{out}");
        // 올바른 필드명으로 범위 밖.
        let mut v2: serde_json::Value = serde_json::from_str(&chain_json(16, 4)).unwrap();
        v2["slice_j"] = serde_json::json!(99);
        let r2: serde_json::Value = serde_json::from_str(&run_chain(&v2.to_string())).unwrap();
        assert_eq!(r2["ok"], false);
    }
}
