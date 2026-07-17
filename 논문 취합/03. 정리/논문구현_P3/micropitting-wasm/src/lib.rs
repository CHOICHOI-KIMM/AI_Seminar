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
//! ## Phase 0 범위
//! `solve_wear` **1개만**. 목적은 기능이 아니라 **전제 반증 시도**:
//! rustfft·nalgebra 의 wasm32 빌드와 경계 왕복 값 무결성을 최소 비용으로 확인한다.

use micropitting_model::m3_stress;
use micropitting_model::m4_fatigue::{self, FatigueParams};
use micropitting_model::m5_wear::{self, WearInput, WearParams};
use micropitting_model::types::{Field2, Grid, MaterialProps, OperatingConditions, WearResult};
use serde::{Deserialize, Serialize};

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
    let resp = match serde_json::from_str::<WearArgs>(input_json) {
        Ok(args) => {
            let params = args.params.to_params();
            // lifetime struct 는 여기서 조립 — 경계를 넘지 않는다.
            let input = WearInput {
                grid: args.grid,
                p_tran: &args.p_tran,
                op: args.op,
                mat: args.mat,
                phi_bl: args.phi_bl,
            };
            WearResponse {
                ok: true,
                result: Some(m5_wear::solve_wear(&input, &params)),
                error: None,
            }
        }
        Err(e) => WearResponse {
            ok: false,
            result: None,
            error: Some(format!("input parse failed: {e}")),
        },
    };
    // 직렬화 실패는 구조상 불가(전 필드 f64/usize) — 그래도 unwrap 대신 명시 폴백.
    serde_json::to_string(&resp)
        .unwrap_or_else(|e| format!(r#"{{"ok":false,"error":"serialize failed: {e}"}}"#))
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

/// M3(응력) → M4(피로) 체인. **계량/스파이크용** — Phase 2 에서 정식 진입점으로 확장.
pub fn run_stress_fatigue(input_json: &str) -> String {
    let args: StressFatigueArgs = match serde_json::from_str(input_json) {
        Ok(a) => a,
        Err(e) => {
            return serde_json::to_string(&StressFatigueSummary {
                ok: false,
                error: Some(format!("input parse failed: {e}")),
                b: 0.0,
                nz: 0,
                max_von_mises: 0.0,
                max_dang_van_d: 0.0,
            })
            .unwrap_or_default()
        }
    };

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

    serde_json::to_string(&StressFatigueSummary {
        ok: true,
        error: None,
        b,
        nz: args.nz,
        max_von_mises: max_vm,
        max_dang_van_d: max_d,
    })
    .unwrap_or_default()
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

/// M3→M4 체인 (계량/스파이크): `StressFatigueArgs` JSON → `StressFatigueSummary` JSON.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn solve_stress_fatigue_json(input_json: &str) -> String {
    run_stress_fatigue(input_json)
}
