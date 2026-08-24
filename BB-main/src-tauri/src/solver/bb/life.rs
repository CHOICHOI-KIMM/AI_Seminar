// BB Contact Analysis — 동정격하중 · 수명 (ISO 281:2007 + ISO 16281:2025)
//
// BB Phase 5-1 S5-1-2 (2026-08-24): **백지 신규**.
// 같은 이름의 구 파일은 P4-S0-1 에서 영구 삭제된 롤러(TRB) 판이라 관련이 없다.
//
// ── 근거 ────────────────────────────────────────────────────────────
// BB_Development_Theory.md §7.1 ~ §7.10.
//   (ISO 281-1)(ISO 281-2) C_r,  Table 1 b_m,  Table 2 f_c
//   Table 3  X · Y · e
//   (1)(2)   Q_ci · Q_ce      — ISO 16281 §5.2.1.2
//   (5)~(8)  Q_ei · Q_ee      — ISO 16281 §5.2.2
//   (9)(11)  L_10r · P_ref r  — ISO 16281 §5.2.3~5.2.4
//   (13)     L_nmr            — ISO 16281 §5.2.5
//   (31)~(33) a_ISO **볼 계수**,  (27)~(29) κ · ν₁
//   (B.1)(B.9)(B.10)(B.11) C_u 정밀법,  (B.18)(B.19) 간이법
//
// ⚠️ **롤러 계수 오용 금지** (Theory §7.8 / §7.9 의 경고).
//    a_ISO : 볼은 (31)~(33) — 계수 2,5671 / 2,2649 / 1,9987, 지수 0,83 · 1/3 · −9,3.
//            롤러 (34)~(36) 의 1,5859 / 1,3993 / 1,2348 · 0,4 · −9,185 를 쓰면 틀린다.
//    C_u   : 볼은 (B.10) 0,228 8 및 (B.18) C_0/22.
//            롤러 (B.14) 0,245 3 · (B.20) C_0/8,2 와 혼동 금지.
//
// ── 범위 (사용자 확정 2026-08-24) ───────────────────────────────────
// **단열 레이디얼 볼만.** 복열 (10)(14)(15) · thrust (3)(4)(12)(16) · 자동조심은 제외.
// 따라서 i = 1 고정이고 L_10r 은 단열식 (9) 만 쓴다.
//
// ── 단위 (D-10) ─────────────────────────────────────────────────────
// mm · N · rad · MPa. 수명은 **10⁶ 회전** 단위(`_mrev`)로만 낸다.
// 시간(h) 환산은 UI 경계에서 한다 — 이 파일에 단위 환산 상수는 없다.

use crate::error::SolverError;
use crate::solver::bb::static_rating;
use crate::solver::bb::types::*;
use crate::solver::common::types::{Alert, AlertLevel, Material};
use crate::solver::common::util;

// ═══════════════════════════════════════════════════════════════════
//  C_r — ISO 281 §5.1.1
// ═══════════════════════════════════════════════════════════════════

/// ISO 281 Table 1 — 레이디얼·앵귤러콘택트 볼베어링(필링슬롯 제외)의 `b_m`.
pub const B_M_BALL: f64 = 1.3;

/// ISO 281 식 (1)/(2) 의 분기점 — 볼 직경 [mm].
pub const D_W_FORMULA_SWITCH_MM: f64 = 25.4;

/// 식 (2) 의 계수 (D_w > 25,4 mm).
pub const C_R_LARGE_BALL_COEFF: f64 = 3.647;

/// ISO 281:2007 Table 2 — `f_c` (단열 레이디얼 + 단열·복열 앵귤러콘택트 열).
///
/// 진입 변수는 `γ = D_w cos α / D_pw`. 중간값은 선형보간한다.
/// ACBB 열은 γ ≈ 0,19 에서 최대 60,0 을 가지므로 **이 표도 비단조**다.
pub const F_C_TABLE: [(f64, f64); 40] = [
    (0.01, 29.1), (0.02, 35.8), (0.03, 40.3), (0.04, 43.8), (0.05, 46.7),
    (0.06, 49.1), (0.07, 51.1), (0.08, 52.8), (0.09, 54.3), (0.10, 55.5),
    (0.11, 56.6), (0.12, 57.5), (0.13, 58.2), (0.14, 58.8), (0.15, 59.3),
    (0.16, 59.6), (0.17, 59.8), (0.18, 59.9), (0.19, 60.0), (0.20, 59.9),
    (0.21, 59.8), (0.22, 59.6), (0.23, 59.3), (0.24, 59.0), (0.25, 58.6),
    (0.26, 58.2), (0.27, 57.7), (0.28, 57.1), (0.29, 56.6), (0.30, 56.0),
    (0.31, 55.3), (0.32, 54.6), (0.33, 53.9), (0.34, 53.2), (0.35, 52.4),
    (0.36, 51.7), (0.37, 50.9), (0.38, 50.0), (0.39, 49.2), (0.40, 48.4),
];

/// `f_c` 가 최대(60,0)가 되는 γ — 비단조 굴곡점.
pub const F_C_PEAK_GAMMA: f64 = 0.19;

/// ISO 281 Table 2 보간. 표 밖(γ ∉ [0,01, 0,40])이면 **오류**다.
pub fn f_c(gamma: f64) -> Result<f64, SolverError> {
    util::interpolate_linear_table(&F_C_TABLE, gamma).ok_or_else(|| {
        SolverError::InvalidGeometry(format!(
            "γ = {gamma:.6} 가 ISO 281 Table 2 의 정의역 [0,01, 0,40] 밖입니다 — \
             규격이 값을 주지 않아 f_c 를 외삽하지 않습니다"
        ))
    })
}

/// 열 수 i. 단열 고정.
pub const ROW_COUNT: f64 = 1.0;

/// 기본 동정격 반경하중 `C_r` [N] — ISO 281 식 (1)/(2).
///
/// ```text
///   D_w ≤ 25,4 : C_r = b_m f_c (i cos α)^0,7 Z^(2/3) D_w^1,8
///   D_w > 25,4 : C_r = 3,647 b_m f_c (i cos α)^0,7 Z^(2/3) D_w^1,4
/// ```
pub fn basic_dynamic_radial_load_rating_n(
    f_c_value: f64,
    z: u32,
    d_w_mm: f64,
    alpha_nom_rad: f64,
) -> f64 {
    let common = B_M_BALL
        * f_c_value
        * (ROW_COUNT * alpha_nom_rad.cos()).powf(0.7)
        * f64::from(z).powf(2.0 / 3.0);
    if d_w_mm <= D_W_FORMULA_SWITCH_MM {
        common * d_w_mm.powf(1.8)
    } else {
        C_R_LARGE_BALL_COEFF * common * d_w_mm.powf(1.4)
    }
}

// ═══════════════════════════════════════════════════════════════════
//  P — ISO 281 §5.2.1 Table 3 (카탈로그 수명용)
// ═══════════════════════════════════════════════════════════════════

/// ISO 281:2007 Table 3 (단열) — (접촉각 [°], `X`, `Y`, `e`).
///
/// `F_a/F_r > e` 구간의 `X`·`Y` 이며, `F_a/F_r ≤ e` 이면 `X = 1, Y = 0` 이다.
/// α ≥ 20° 는 상대 축하중과 **무관**한 단일 값을 가진다 (Theory §7.3).
///
/// ⚠️ α = 5°/10°/15° 는 `f_0 F_a/C_0r` 에 따라 X·Y·e 가 **또 한 겹 변한다**
///    (ISO 281 Table 3 상단부). 그 상단부는 Theory §7.3 에 전사되어 있지 않으므로
///    본 구현은 α < 20° 를 **거부**한다 — 값을 지어내지 않는다.
pub const XYE_TABLE_DEG: [(f64, f64, f64, f64); 6] = [
    (20.0, 0.43, 1.00, 0.57),
    (25.0, 0.41, 0.87, 0.68),
    (30.0, 0.39, 0.76, 0.80),
    (35.0, 0.37, 0.66, 0.95),
    (40.0, 0.35, 0.57, 1.14),
    (45.0, 0.33, 0.50, 1.34),
];

/// Table 3 단열 구간의 하한 [°] — 이보다 작으면 2중 보간 표가 필요하다.
pub const XYE_MIN_DEG: f64 = 20.0;
/// Table 3 단열 구간의 상한 [°].
pub const XYE_MAX_DEG: f64 = 45.0;

/// `(X, Y, e)` — ISO 281 Table 3 (단열), 접촉각 선형보간.
pub fn x_y_e(alpha_nom_rad: f64) -> Result<(f64, f64, f64), SolverError> {
    let alpha_deg = alpha_nom_rad.to_degrees();
    if !(XYE_MIN_DEG..=XYE_MAX_DEG).contains(&alpha_deg) {
        return Err(SolverError::InvalidGeometry(format!(
            "공칭 접촉각 {alpha_deg:.3}° 가 ISO 281 Table 3 단열 구간 \
             [{XYE_MIN_DEG}°, {XYE_MAX_DEG}°] 밖입니다. α < 20° 는 상대 축하중 \
             f_0·F_a/C_0r 에 따른 2중 보간이 필요하며 그 표는 Theory §7.3 에 \
             전사되어 있지 않습니다 (동등가하중 P 만 영향 — L_10r 은 P_ref r 을 씁니다)"
        )));
    }
    let x_grid: Vec<(f64, f64)> = XYE_TABLE_DEG.iter().map(|r| (r.0, r.1)).collect();
    let y_grid: Vec<(f64, f64)> = XYE_TABLE_DEG.iter().map(|r| (r.0, r.2)).collect();
    let e_grid: Vec<(f64, f64)> = XYE_TABLE_DEG.iter().map(|r| (r.0, r.3)).collect();
    let x = util::interpolate_linear_table(&x_grid, alpha_deg).expect("구간 내 보장됨");
    let y = util::interpolate_linear_table(&y_grid, alpha_deg).expect("구간 내 보장됨");
    let e = util::interpolate_linear_table(&e_grid, alpha_deg).expect("구간 내 보장됨");
    Ok((x, y, e))
}

/// 동등가 반경하중 `P_r` [N] — ISO 281 §5.2.1.
///
/// `F_a/F_r ≤ e` 이면 `X = 1, Y = 0` (즉 `P = F_r`), 아니면 Table 3 값을 쓴다.
/// **2중 보간의 바깥 겹**(상대 축하중 분기)이 여기다.
pub fn dynamic_equivalent_radial_load_n(
    alpha_nom_rad: f64,
    f_r_n: f64,
    f_a_n: f64,
) -> Result<f64, SolverError> {
    let (x, y, e) = x_y_e(alpha_nom_rad)?;
    if f_r_n <= 0.0 {
        // 순수 축하중. F_a/F_r → ∞ 이므로 Table 3 분기.
        return Ok(y * f_a_n);
    }
    if f_a_n / f_r_n <= e {
        Ok(f_r_n)
    } else {
        Ok(x * f_r_n + y * f_a_n)
    }
}

// ═══════════════════════════════════════════════════════════════════
//  Q_ci · Q_ce — ISO 16281 §5.2.1.2 식 (1)(2)
// ═══════════════════════════════════════════════════════════════════

/// (1)(2) 의 중괄호 안 공통항.
///
/// ```text
///   T = 1,044 ((1−γ)/(1+γ))^1,72 [ (r_i/r_e)((2r_e − D_w)/(2r_i − D_w)) ]^0,41
/// ```
/// 내륜은 `(1 + T^(10/3))^(3/10)`, 외륜은 `(1 + T^(−10/3))^(3/10)` 로 지수 부호만 반전된다.
pub fn q_c_common_term(gamma: f64, r_i_mm: f64, r_e_mm: f64, d_w_mm: f64) -> f64 {
    let groove = (r_i_mm / r_e_mm)
        * ((2.0 * r_e_mm - d_w_mm) / (2.0 * r_i_mm - d_w_mm));
    1.044 * ((1.0 - gamma) / (1.0 + gamma)).powf(1.72) * groove.powf(0.41)
}

/// 내륜 계수 (식 1).
pub const Q_C_INNER_COEFF: f64 = 0.407;
/// 외륜 계수 (식 2).
pub const Q_C_OUTER_COEFF: f64 = 0.389;

/// 등가 공칭 전동체하중 `Q_ci` [N] — 식 (1).
pub fn q_ci_n(c_r_n: f64, z: u32, alpha_nom_rad: f64, common_term: f64) -> f64 {
    let den = Q_C_INNER_COEFF * f64::from(z) * alpha_nom_rad.cos() * ROW_COUNT.powf(0.7);
    c_r_n / den * (1.0 + common_term.powf(10.0 / 3.0)).powf(3.0 / 10.0)
}

/// 등가 공칭 전동체하중 `Q_ce` [N] — 식 (2). 중괄호 지수가 **−10/3** 이다.
pub fn q_ce_n(c_r_n: f64, z: u32, alpha_nom_rad: f64, common_term: f64) -> f64 {
    let den = Q_C_OUTER_COEFF * f64::from(z) * alpha_nom_rad.cos() * ROW_COUNT.powf(0.7);
    c_r_n / den * (1.0 + common_term.powf(-10.0 / 3.0)).powf(3.0 / 10.0)
}

// ═══════════════════════════════════════════════════════════════════
//  Q_ei · Q_ee — ISO 16281 §5.2.2 식 (5)~(8)
// ═══════════════════════════════════════════════════════════════════

/// 회전하는 궤도륜의 하중 평균 지수 — 식 (5)(8).
pub const ROTATING_RING_EXPONENT: f64 = 3.0;
/// 정지한 궤도륜의 하중 평균 지수 — 식 (6)(7).
pub const STATIONARY_RING_EXPONENT: f64 = 10.0 / 3.0;

/// `Q_e = ( (1/Z) Σ Q_j^p )^(1/p)` — 식 (5)~(8) 의 공통 형태.
///
/// 합은 **전 볼**에 대해 돈다. 비접촉 볼은 `Q_j = 0` 이라 기여하지 않으나
/// 분모의 `Z` 에는 그대로 들어간다.
pub fn equivalent_ball_load_n(ball_loads_n: &[f64], exponent: f64) -> f64 {
    if ball_loads_n.is_empty() {
        return 0.0;
    }
    let z = ball_loads_n.len() as f64;
    let sum: f64 = ball_loads_n
        .iter()
        .map(|q| if *q > 0.0 { q.powf(exponent) } else { 0.0 })
        .sum();
    (sum / z).powf(1.0 / exponent)
}

// ═══════════════════════════════════════════════════════════════════
//  L_10r · P_ref r — ISO 16281 §5.2.3~5.2.4 식 (9)(11)
// ═══════════════════════════════════════════════════════════════════

/// 기본 기준정격수명 `L_10r` [10⁶ rev] — 단열 식 (9).
///
/// 하중이 0 이면 `+∞` 를 반환한다.
pub fn l_10r_mrev(q_ci_n: f64, q_ei_n: f64, q_ce_n: f64, q_ee_n: f64) -> f64 {
    if q_ei_n <= 0.0 && q_ee_n <= 0.0 {
        return f64::INFINITY;
    }
    let ti = if q_ei_n > 0.0 {
        (q_ci_n / q_ei_n).powf(-10.0 / 3.0)
    } else {
        0.0
    };
    let te = if q_ee_n > 0.0 {
        (q_ce_n / q_ee_n).powf(-10.0 / 3.0)
    } else {
        0.0
    };
    (ti + te).powf(-9.0 / 10.0)
}

/// 동등가 기준하중 `P_ref r` [N] — 식 (11). **`a_ISO` 의 인수는 이 값**이다 (§7.7).
pub fn p_ref_r_n(c_r_n: f64, l_10r: f64) -> f64 {
    if !l_10r.is_finite() || l_10r <= 0.0 {
        return 0.0;
    }
    c_r_n / l_10r.cbrt()
}

// ═══════════════════════════════════════════════════════════════════
//  C_u — ISO 281 Annex B
// ═══════════════════════════════════════════════════════════════════

/// ISO 281 B.3.1 권장 피로한계 접촉응력 [MPa] (양질 재료·양질 가공).
///
/// ⚠️ 정정격의 σ_max = 4 200 MPa(영구변형 한계)와 **다른 기준**이다 (Theory §7.11 말미).
pub const SIGMA_HU_MPA: f64 = crate::solver::bb::hertz::SIGMA_HU_MPA;

/// (B.10)(B.11) 의 **볼** 계수. 롤러 (B.14) 의 0,245 3 과 혼동 금지.
pub const C_U_BALL_COEFF: f64 = 0.2288;

/// (B.18)(B.19) 간이법의 **볼** 제수. 롤러 (B.20) 의 8,2 와 혼동 금지.
pub const C_U_SIMPLE_DIVISOR: f64 = 22.0;

/// (B.10)/(B.11)·(B.18)/(B.19) 의 분기 피치직경 [mm].
pub const C_U_DPW_SWITCH_MM: f64 = 100.0;

/// 접촉 하나의 피로한계 전동체하중 `Q_u` [N] — 식 (B.1).
///
/// ```text
///   Q_u = σ_Hu³ · (32π χ / 3) · [ (1−ν²)/E · E(χ)/Σρ ]²
/// ```
/// 점접촉이라 볼은 **해석적**으로 계산된다 (롤러와 달리 수치해석 불필요).
pub fn q_u_n(chi: f64, e_ellip: f64, sum_rho_per_mm: f64, e_mpa: f64, nu: f64) -> f64 {
    let compliance = (1.0 - nu * nu) / e_mpa;
    let inner = compliance * e_ellip / sum_rho_per_mm;
    SIGMA_HU_MPA.powi(3) * (32.0 * std::f64::consts::PI * chi / 3.0) * inner * inner
}

/// 피로한계하중 `C_u` [N] — 정밀법 (B.9)+(B.10)/(B.11).
pub fn c_u_precise_n(q_u_n_value: f64, z: u32, alpha_nom_rad: f64, d_pw_mm: f64) -> f64 {
    let base = C_U_BALL_COEFF * f64::from(z) * q_u_n_value * ROW_COUNT * alpha_nom_rad.cos();
    if d_pw_mm <= C_U_DPW_SWITCH_MM {
        base
    } else {
        base * (C_U_DPW_SWITCH_MM / d_pw_mm).sqrt()
    }
}

/// 피로한계하중 `C_u` [N] — 간이법 (B.18)/(B.19). `C_0` 는 ISO 76 정정격하중이다.
///
/// > ISO NOTE: 간이법 결과는 정밀법과 상당히 다를 수 있으며 **정밀법이 우선**이다.
pub fn c_u_simplified_n(c_0r_n: f64, d_pw_mm: f64) -> f64 {
    let base = c_0r_n / C_U_SIMPLE_DIVISOR;
    if d_pw_mm <= C_U_DPW_SWITCH_MM {
        base
    } else {
        base * (C_U_DPW_SWITCH_MM / d_pw_mm).sqrt()
    }
}

// ═══════════════════════════════════════════════════════════════════
//  κ · a_ISO — ISO 281 식 (27)~(29), (31)~(33)
// ═══════════════════════════════════════════════════════════════════

/// (28)/(29) 의 분기 회전속도 [r/min].
pub const NU1_SPEED_SWITCH_RPM: f64 = 1000.0;

/// 기준 동점도 `ν₁` [mm²/s] — 식 (28)(29).
pub fn nu_1_mm2_s(n_rpm: f64, d_pw_mm: f64) -> Option<f64> {
    if n_rpm <= 0.0 || d_pw_mm <= 0.0 {
        return None;
    }
    let v = if n_rpm < NU1_SPEED_SWITCH_RPM {
        45_000.0 * n_rpm.powf(-0.83) * d_pw_mm.powf(-0.5)
    } else {
        4_500.0 * n_rpm.powf(-0.5) * d_pw_mm.powf(-0.5)
    };
    Some(v)
}

/// `a_ISO` 계산 가능 하한 (ISO 281 §9.3.3.4).
pub const KAPPA_MIN: f64 = 0.1;
/// `a_ISO` 계산 시 κ 를 이 값으로 클램프한다 (κ > 4 → 4).
pub const KAPPA_MAX: f64 = 4.0;
/// `a_ISO` 의 상한 (ISO 281 §9.3.3.4).
pub const A_ISO_CAP: f64 = 50.0;
/// EP 첨가제 적용 시 `a_ISO` 상한.
pub const A_ISO_CAP_EP: f64 = 3.0;
/// EP 첨가제 규칙이 발동하는 오염계수 하한.
pub const EP_RULE_EC_MIN: f64 = 0.2;

/// `a_ISO` 식 (31)~(33) 의 κ 구간 경계.
const KAPPA_BREAK_LOW: f64 = 0.4;
/// 식 (32)/(33) 의 경계.
const KAPPA_BREAK_HIGH: f64 = 1.0;

/// `a_ISO` — **레이디얼 볼베어링** ISO 281 식 (31)~(33).
///
/// ```text
///   a_ISO = 0,1 [ 1 − (2,5671 − C/κ^p)^0,83 · (e_C C_u / P)^(1/3) ]^(−9,3)
///     0,1 ≤ κ < 0,4 : C = 2,2649, p = 0,054381        (31)
///     0,4 ≤ κ < 1   : C = 1,9987, p = 0,19087         (32)
///     1   ≤ κ ≤ 4   : C = 1,9987, p = 0,071739        (33)
/// ```
/// 대괄호가 0 이하가 되면 (인수 `e_C C_u/P` 가 매우 클 때) 상한 50 으로 포화시킨다.
///
/// ⚠️ 롤러 식 (34)~(36) 의 계수(1,5859 / 1,3993 / 1,2348, 지수 0,4 · −9,185)를
///    쓰면 수명이 틀린다 (Theory §7.8 경고).
pub fn a_iso_ball(kappa: f64, load_ratio: f64) -> Result<f64, SolverError> {
    if kappa < KAPPA_MIN {
        return Err(SolverError::InvalidInput(format!(
            "점도비 κ = {kappa:.4} 가 ISO 281 식 (31)~(33) 의 적용 하한 {KAPPA_MIN} 미만입니다 — \
             규격·선도의 범위 밖이라 a_ISO 를 계산할 수 없습니다"
        )));
    }
    let k = kappa.min(KAPPA_MAX);
    let (coeff, exponent) = if k < KAPPA_BREAK_LOW {
        (2.2649, 0.054381)
    } else if k < KAPPA_BREAK_HIGH {
        (1.9987, 0.19087)
    } else {
        (1.9987, 0.071739)
    };
    let bracket = 1.0 - (2.5671 - coeff / k.powf(exponent)).powf(0.83) * load_ratio.cbrt();
    if bracket <= 0.0 {
        return Ok(A_ISO_CAP);
    }
    Ok((0.1 * bracket.powf(-9.3)).min(A_ISO_CAP))
}

// ═══════════════════════════════════════════════════════════════════
//  종합
// ═══════════════════════════════════════════════════════════════════

/// ISO 281/16281 수명 일괄 계산 (Theory §7.1~7.10).
///
/// `ball_loads_n` 은 평형해의 볼별 하중 `Q_j` [N] 전량이다 (비접촉 볼의 0 포함).
pub fn compute_life(
    input: &BbInput,
    derived: &BbGeometryDerived,
    contact: &BbContactDerived,
    ball_loads_n: &[f64],
    conditions: &BbLifeConditions,
) -> Result<BbLifeResult, SolverError> {
    conditions.validate()?;
    let geom = &input.geometry;
    let material: &Material = &input.material;

    // ── 정정격 (ISO 76) ────────────────────────────────────────────
    let static_rating = static_rating::compute_static_rating(
        geom,
        derived,
        &input.operating,
        input.solver.c_0r_n,
    )?;

    // ── C_r (ISO 281) ──────────────────────────────────────────────
    let f_c_value = f_c(derived.gamma)?;
    let c_r_computed_n =
        basic_dynamic_radial_load_rating_n(f_c_value, geom.z, geom.d_w_mm, geom.alpha_nom_rad);
    let c_r_n = input.solver.c_r_n.unwrap_or(c_r_computed_n);

    // ── Q_ci · Q_ce (ISO 16281 (1)(2)) ─────────────────────────────
    let common_term = q_c_common_term(derived.gamma, geom.r_i_mm, geom.r_e_mm, geom.d_w_mm);
    let q_ci = q_ci_n(c_r_n, geom.z, geom.alpha_nom_rad, common_term);
    let q_ce = q_ce_n(c_r_n, geom.z, geom.alpha_nom_rad, common_term);

    // ── Q_ei · Q_ee (ISO 16281 (5)~(8)) ────────────────────────────
    let (exp_i, exp_e) = if conditions.inner_ring_rotating {
        (ROTATING_RING_EXPONENT, STATIONARY_RING_EXPONENT)
    } else {
        (STATIONARY_RING_EXPONENT, ROTATING_RING_EXPONENT)
    };
    let q_ei = equivalent_ball_load_n(ball_loads_n, exp_i);
    let q_ee = equivalent_ball_load_n(ball_loads_n, exp_e);

    // ── L_10r · P_ref r ────────────────────────────────────────────
    let l_10r = l_10r_mrev(q_ci, q_ei, q_ce, q_ee);
    let p_ref_r = p_ref_r_n(c_r_n, l_10r);

    // ── ISO 281 카탈로그 동등가하중 P (α ≥ 20° 에서만) ─────────────
    let p_r_n = dynamic_equivalent_radial_load_n(
        geom.alpha_nom_rad,
        static_rating.f_r_n,
        static_rating.f_a_n,
    )
    .ok();

    // ── C_u (ISO 281 Annex B) ──────────────────────────────────────
    let q_ui = q_u_n(
        contact.chi_inner,
        contact.e_ellip_inner,
        derived.sum_rho_i_per_mm,
        material.e_ring_mpa,
        material.nu,
    );
    let q_ue = q_u_n(
        contact.chi_outer,
        contact.e_ellip_outer,
        derived.sum_rho_e_per_mm,
        material.e_ring_mpa,
        material.nu,
    );
    let q_u = q_ui.min(q_ue); // (B.9)
    let c_u_precise = c_u_precise_n(q_u, geom.z, geom.alpha_nom_rad, geom.d_pw_mm);
    let c_u_simplified = c_u_simplified_n(static_rating.c_0r_n, geom.d_pw_mm);
    let c_u_n = match conditions.c_u_method {
        BbFatigueLimitMethod::Precise => c_u_precise,
        BbFatigueLimitMethod::Simplified => c_u_simplified,
    };

    let mut alerts = Vec::new();

    // ── κ · a_ISO · L_nmr ──────────────────────────────────────────
    let speed_rpm = input.operating.relative_speed_rpm().abs();
    let nu_1 = nu_1_mm2_s(speed_rpm, geom.d_pw_mm);
    let mut kappa = match (nu_1, conditions.nu_mm2_s > 0.0) {
        (Some(n1), true) => Some(conditions.nu_mm2_s / n1),
        _ => None,
    };

    // EP 첨가제 규칙 (ISO 281 §9.3.3.4): κ < 1 이고 e_C ≥ 0,2 이면 κ = 1, a_ISO ≤ 3
    let mut a_iso_cap = A_ISO_CAP;
    let mut ep_rule_applied = false;
    if conditions.ep_additive {
        if let Some(k) = kappa {
            if k < KAPPA_BREAK_HIGH && conditions.e_c >= EP_RULE_EC_MIN {
                kappa = Some(KAPPA_BREAK_HIGH);
                a_iso_cap = A_ISO_CAP_EP;
                ep_rule_applied = true;
            }
        }
    }

    let mut a_iso = None;
    if let Some(k) = kappa {
        if p_ref_r > 0.0 {
            let ratio = conditions.e_c * c_u_n / p_ref_r;
            match a_iso_ball(k, ratio) {
                Ok(a) => a_iso = Some(a.min(a_iso_cap)),
                Err(e) => alerts.push(Alert {
                    level: AlertLevel::Warning,
                    code: "A_ISO_OUT_OF_RANGE".into(),
                    message: format!("{e}"),
                }),
            }
        }
    } else {
        alerts.push(Alert {
            level: AlertLevel::Info,
            code: "KAPPA_UNAVAILABLE".into(),
            message: "운전 동점도 ν 또는 회전속도가 지정되지 않아 점도비 κ 와 a_ISO 를 \
                      산출하지 못했습니다 — 수정정격수명 L_nmr 이 비어 있습니다"
                .into(),
        });
    }

    let l_nmr_mrev = a_iso.map(|a| conditions.a_1 * a * l_10r);

    if ep_rule_applied {
        alerts.push(Alert {
            level: AlertLevel::Info,
            code: "EP_ADDITIVE_RULE_APPLIED".into(),
            message: format!(
                "EP 첨가제 규칙(ISO 281 §9.3.3.4)을 적용했습니다: κ < 1 이고 e_C ≥ {EP_RULE_EC_MIN} \
                 이므로 κ = 1 로 두고 a_ISO 를 {A_ISO_CAP_EP} 로 제한했습니다"
            ),
        });
    }
    if a_iso == Some(A_ISO_CAP) {
        alerts.push(Alert {
            level: AlertLevel::Info,
            code: "A_ISO_CAPPED".into(),
            message: format!(
                "a_ISO 가 ISO 281 §9.3.3.4 의 상한 {A_ISO_CAP} 에 걸렸습니다 \
                 (e_C·C_u/P_ref r 가 큼) — 수명이 상한으로 포화되었습니다"
            ),
        });
    }

    // 정밀법 ↔ 간이법 편차 기록 (Plan P5-1 Level E 「C_u 2경로」)
    if c_u_precise > 0.0 {
        let deviation = (c_u_simplified - c_u_precise) / c_u_precise;
        if deviation.abs() > 0.5 {
            alerts.push(Alert {
                level: AlertLevel::Info,
                code: "C_U_METHOD_DEVIATION".into(),
                message: format!(
                    "피로한계하중 C_u 의 정밀법 {c_u_precise:.0} N 과 간이법 {c_u_simplified:.0} N 이 \
                     {:.0} % 차이납니다. ISO 281 Annex B NOTE 는 정밀법을 우선합니다",
                    deviation * 100.0
                ),
            });
        }
    }

    alerts.extend(static_rating.alerts.iter().cloned());

    Ok(BbLifeResult {
        f_c: f_c_value,
        b_m: B_M_BALL,
        c_r_n,
        c_r_computed_n,
        q_ci_n: q_ci,
        q_ce_n: q_ce,
        q_ei_n: q_ei,
        q_ee_n: q_ee,
        l_10r_mrev: l_10r,
        p_ref_r_n: p_ref_r,
        p_r_n,
        q_u_n: q_u,
        c_u_n,
        c_u_precise_n: c_u_precise,
        c_u_simplified_n: c_u_simplified,
        nu_1_mm2_s: nu_1,
        kappa,
        a_iso,
        a_1: conditions.a_1,
        l_nmr_mrev,
        static_rating,
        alerts,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f_c_grid_points_are_reproduced() {
        for (gamma, expected) in F_C_TABLE {
            assert!((f_c(gamma).unwrap() - expected).abs() < 1e-12, "γ={gamma}");
        }
    }

    #[test]
    fn f_c_peak_is_at_gamma_0_19() {
        let peak = f_c(F_C_PEAK_GAMMA).unwrap();
        assert!((peak - 60.0).abs() < 1e-12);
        assert!(f_c(0.18).unwrap() < peak);
        assert!(f_c(0.20).unwrap() < peak);
        assert!(f_c(0.005).is_err());
        assert!(f_c(0.41).is_err());
    }

    #[test]
    fn x_y_e_grid_points_and_range() {
        for (deg, x, y, e) in XYE_TABLE_DEG {
            let (gx, gy, ge) = x_y_e(deg.to_radians()).unwrap();
            assert!((gx - x).abs() < 1e-12 && (gy - y).abs() < 1e-12 && (ge - e).abs() < 1e-12);
        }
        assert!(x_y_e(15.0_f64.to_radians()).is_err());
        assert!(x_y_e(50.0_f64.to_radians()).is_err());
    }

    #[test]
    fn equivalent_ball_load_of_uniform_set_equals_the_load() {
        let loads = [500.0; 12];
        for e in [ROTATING_RING_EXPONENT, STATIONARY_RING_EXPONENT] {
            assert!((equivalent_ball_load_n(&loads, e) - 500.0).abs() < 1e-9);
        }
    }

    #[test]
    fn a_iso_is_capped_and_range_checked() {
        assert!(a_iso_ball(0.05, 1.0).is_err());
        // 매우 깨끗·저하중 → 상한 50
        assert!((a_iso_ball(2.0, 10.0).unwrap() - A_ISO_CAP).abs() < 1e-12);
        // κ > 4 는 4 로 클램프되므로 결과가 같아야 한다
        let a4 = a_iso_ball(4.0, 0.5).unwrap();
        let a9 = a_iso_ball(9.0, 0.5).unwrap();
        assert!((a4 - a9).abs() < 1e-12);
    }

    #[test]
    fn nu_1_switches_at_1000_rpm() {
        assert!(nu_1_mm2_s(0.0, 70.0).is_none());
        let below = nu_1_mm2_s(999.0, 70.0).unwrap();
        let above = nu_1_mm2_s(1001.0, 70.0).unwrap();
        // 분기 양쪽이 같은 오더여야 한다 (규격 곡선이 이어진다)
        assert!((below / above - 1.0).abs() < 0.05, "{below} vs {above}");
    }
}
