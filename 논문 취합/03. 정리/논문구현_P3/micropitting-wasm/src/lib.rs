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
}
