//! # M5 — 경마모 (수정 Archard)
//!
//! M6 전이압 [`p_tran`](crate::types::PartialLubResult::p_tran)·미끄럼속도 `u_s`·경계윤활
//! 분율 `phi_bl` 을 소비(M3/M4 비의존)해 **사이클당 국소 마모깊이** `Δh_w/Δn` 을 산출한다.
//! 결과는 [`WearResult`](crate::types::WearResult). 시간루프가 `Δn` 스텝마다 형상(양 표면
//! 절반)·손상맵을 갱신한다(§양표면 절반배분).
//!
//! ## 근거 원문 유도 (자의계수 금지 — 식·계수는 Archard 1953 / P2 소급, 불일치 flag)
//!
//! ### Archard 1953 (정본 — 마모법칙 원출처)
//! - **Eq(17)** `W = K·P/(3·p_m)` : 마모율(worn volume / 단위 미끄럼거리), `p_m`=유동압(≈경도),
//!   `P`=하중, `K`=확률인자(무차원). Holm **Eq(12)** `W = Z·P/p_m` 와 동형.
//!   **⚠ OCR flag**: 원문 MD 는 `W=K·P/(3a)` 로 표기되나 `3a`(force/length)는 `W`(volume/distance
//!   =area)와 **차원 불일치** → `p_m` 의 OCR 오독(`p_m→a`)으로 판단. 물리 정본 분모는 `3·p_m`.
//! - **하중-경도형 유도**(위 `3·p_m` 확인): 소성접촉 단일 asperity `P_i = p_m·π·a²`,
//!   반구형 마모입자 체적 `δV=(2/3)π a³`, `δW=K·δV/(2a)=K·π a²/3`. `πa²=P/p_m` 대입하면
//!   **`W = K·P/(3·p_m)`** (Archard Sec. II.3 과정). `p_m`(유동압)≈경도 `H`.
//! - **깊이형 환산**: 양변을 공칭면적 `A` 로 나눔 → `(W/A) = K·(P/A)/(3 p_m) = K·p/(3H)`,
//!   즉 **단위 미끄럼거리당 마모깊이 = K·p/(3H)**. `p=P/A`(공칭압), `W/A`=깊이/거리.
//!
//! ### P2 통합(§3.5 M5) — 수정 Archard 깊이율 식
//! - **식[14]** `Δh_w/Δn = k·p·u_s/H · (ℓ_c/ū)` : 사이클당 마모깊이율 [m/cycle].
//!   Archard 깊이형 `k·p/H` (단위거리당) 에 **사이클당 미끄럼거리** `s_cycle = u_s·(ℓ_c/ū)`
//!   [m/cycle] 를 곱한 형. `k`=무차원 마모계수, `H`=경도[Pa], `u_s`=미끄럼속도[m/s].
//!   **⚠ 계수 불일치 flag**: 식[14]의 `k` 는 Archard 깊이형의 `K/3` 에 대응(입자형상/기하
//!   `1/3` 인자를 경험적 `k` 에 흡수). 식[14]는 명시적 `/3` 없이 `k·p/H` 로 쓰므로,
//!   식[14] `k`(1e-11~5e-10) = Archard `K`/3. 두 상수를 혼동 말 것(VC-M5-Archard 교차검증).
//! - **식[15]** `Δh_w = (u_s/(H·A))·∫_A k·p dA · (ℓ_c/ū)` : 스텝당 면적평균 마모층율.
//!   식[14]를 창(window) 면적 `A` 로 면적평균한 것과 동일(k 스칼라·s_cycle 상수 →
//!   `Δh_w_mean = (1/A)∫ Δh_w dA` = 격자평균). "시료 내 u_s·H 일정" 가정으로 적분 밖 인출.
//!
//! ### Q5 — u_s[m/s] → 사이클당 미끄럼거리 환산 (G-M5-2, SO Q5 확정)
//! 식[14] 좌변은 [m/cycle], 우변 `k·p·u_s/H` 는 [m/s]. **차원 불일치**를 SO Q5 가
//! **환산계수 = 접촉폭/속도 = ℓ_c/ū [m/(m/s)]=[s]** 도입으로 해소(P2-1 §6 L243–250).
//! 물리: 한 물질점이 접촉대(구름방향 길이 `ℓ_c`)를 통과하는 **체류시간** `τ_c = ℓ_c/ū` [s]
//! 동안 두 표면이 상대속도 `u_s` 로 미끄러지므로, **1회 오버롤(=1 cycle)당 미끄럼거리**
//! `s_cycle = u_s·τ_c = u_s·ℓ_c/ū` [m/cycle]. (`ū` = 평균 구름속도 `u_mean`.)
//! - **ℓ_c(접촉폭) = 2b**: `b = 2·r_x·p_h/E_red` (Hertz 선접촉 반폭, types.rs `r_x` 주석;
//!   [`crate::m3_stress::contact_half_width`] 재사용). 물질점은 `x=−b→+b` 전현(全弦) `2b`
//!   를 통과하므로 체류길이 = 전접촉폭 `2b`.
//!   **⚠ 가정 flag(Q5-width)**: P2 는 "접촉폭"만 명기(반폭 b vs 전폭 2b 미구분). 본 구현은
//!   물리적 통과현 = **전폭 2b** 채택(반폭이면 마모율 ½배). 민감도 대상.
//! - `u_s = |slide_roll·u_mean| = |u1−u2|` (SRR=(u1−u2)/u_mean → |u1−u2|=|SRR|·u_mean).
//!
//! ## SSOT / 규약
//! - 단위 SI (m, Pa, m/s, cycle). 좌표 x=구름/y=횡. 식[14] 좌우변 **m/cycle**(CV-M5-Dim).
//! - 압력 규약: `p_tran` 은 압력 크기 양수 저장(M1/M6 인계). 접촉 밖 `p≤0` → `dh_w=0`.
//! - **양표면 절반배분**: 스텝당 `Δh_w` 는 두 접촉면에 절반씩 배분(원문 (a)손상맵 상방이동·
//!   (b)최고돌기 제거). **M5 산출은 총 `dh_w`**(절반 미적용); 형상갱신·절반배분은 **시간루프
//!   소관**(P2-1 §4 M5→갱신). 시간루프가 `dh_w/2` 를 각 표면에 적용.
//! - **RP-Λmax/RP-Slip** (마모↔피로 경쟁, Λ 의존)은 시간루프 손상맵 `A_p` 필요 → **시간루프
//!   G5 이월**(P2: 정량식 미제공, 시간루프 UPD 창발; ref(10) 미보유).

use crate::m3_stress::contact_half_width;
use crate::types::{Field2, Grid, MaterialProps, OperatingConditions, WearResult};

// ─────────────────────────────────────────────────────────────────────────
//  상수 (근거: P2 §3.5 / G-M5-1 SO 가정 — 자의계수 아님, 원문 소급·가정 flag)
// ─────────────────────────────────────────────────────────────────────────

/// 무첨가유(base oil) 경계윤활 마모계수 `k_lub` [-]. 기본 4e-10.
///
/// **근거·가정 flag**: 무첨가유 지지범위 `3–5×10⁻¹⁰` 의 중앙값(G-M5-1, MAIN L371/L391;
/// Williams(37)). SO 가정(민감도 대상). 첨가유는 [`K_LUB_ADDITIVE`].
pub const K_LUB: f64 = 4.0e-10;
/// 첨가유(additive) 경계윤활 마모계수 하한 `k_lub` [-]. 1e-11.
///
/// **⚠ 외삽 flag**: 하한 1e-11 은 원문 **직접근거 없는 공학적 외삽**(P2-3 §3.5 L194 명시).
/// 민감도 스윕 하단(≈1.7 decade)으로만 사용.
pub const K_LUB_ADDITIVE: f64 = 1.0e-11;
/// dry/lub 마모계수비 `f_w` [-]. 기본 10 (부착기구).
///
/// **근거·가정 flag**: `f_w≈10` 은 **MAIN L371 경계/전막 마찰비**(SKF 2011) 소급 — 마찰비 기반값.
/// (별개 척도인 **Archard 1953 L308** 은 윤활 시 **마모**가 `10²~10³`(100~1000)배 감소를 제시 →
/// dry/lub 마모비로는 100~1000 지지. 따라서 f_w=10 은 마찰비 기반의 **보수적 하한**이고 100~1000 은
/// 민감도 상단.) `k_dry = f_w·k_lub`. G-M5-1 가정+민감도(스윕 f_w∈{1,10,100,1000}).
pub const F_W: f64 = 10.0;
/// 경화 AISI 52100 강 대표 경도 `H` [Pa]. 7 GPa (참고값; 실사용은 `mat.hardness`).
///
/// **근거 flag**: MAIN L64 (범위 밖 대조값). 압흔력/면적 ≈7 GPa.
pub const H_STEEL_PA: f64 = 7.0e9;

/// 접촉폭(구름방향) = 전접촉폭 계수 × 반폭 b. `ℓ_c = CONTACT_WIDTH_FACTOR·b`.
///
/// **가정 flag(Q5-width)**: 전현 `2b` 채택(물질점 x=−b→+b 통과). 반폭이면 1.0.
pub const CONTACT_WIDTH_FACTOR: f64 = 2.0;

// ─────────────────────────────────────────────────────────────────────────
//  파라미터 / 입력 (types.rs 동결 — 모듈-로컬 struct)
// ─────────────────────────────────────────────────────────────────────────

/// M5 마모 파라미터 (기본값 = 위 SO 확정 상수; 민감도 스윕용 오버라이드 가능).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WearParams {
    /// 경계윤활 마모계수 k_lub [-] (기본 [`K_LUB`]; 첨가유 [`K_LUB_ADDITIVE`]).
    pub k_lub: f64,
    /// dry/lub 마모계수비 f_w [-] (기본 [`F_W`]).
    pub f_w: f64,
}

impl Default for WearParams {
    fn default() -> Self {
        WearParams {
            k_lub: K_LUB,
            f_w: F_W,
        }
    }
}

impl WearParams {
    /// 건식(dry, 경계) 마모계수 `k_dry = f_w·k_lub` [-] (P2 §3.5; 기본 4e-9).
    #[inline]
    pub fn k_dry(&self) -> f64 {
        self.f_w * self.k_lub
    }
}

/// M5 마모 해석 입력 (types.rs 동결 — 프레인 struct; `p_tran` 은 M6 산출 필드 참조).
///
/// - `p_tran`: M6 전이 압력장 [Pa] (압력 크기 양수; 접촉 밖 ≤0).
/// - `op`: 운전조건 — `u_mean`·`slide_roll`(→u_s), `r_x`·`p_h`(→접촉폭 b).
/// - `mat`: 재료 물성 — `hardness`(H)·`e_red`(→b).
/// - `phi_bl`: 경계윤활 하중분율 [-] (M6 산출; k 가중, 0~1).
pub struct WearInput<'a> {
    /// 계산 격자.
    pub grid: Grid,
    /// M6 전이 압력장 [Pa].
    pub p_tran: &'a Field2,
    /// 운전조건 (u_mean, slide_roll, r_x, p_h).
    pub op: OperatingConditions,
    /// 재료 물성 (hardness H, e_red).
    pub mat: MaterialProps,
    /// 경계윤활 하중분율 phi_bl [-] (0~1).
    pub phi_bl: f64,
}

// ─────────────────────────────────────────────────────────────────────────
//  핵심 커널 (테스트 가능 단위로 분리 — 각 축을 변이로 검출)
// ─────────────────────────────────────────────────────────────────────────

/// 미끄럼속도 `u_s = |slide_roll·u_mean| = |u1−u2|` [m/s] (SRR 정의).
#[inline]
pub fn sliding_speed(op: &OperatingConditions) -> f64 {
    (op.slide_roll * op.u_mean).abs()
}

/// 유효 마모계수 `k = φ_bl·k_dry + (1−φ_bl)·k_lub` [-] (P2 정본, φ_bl 가중).
///
/// dry 패치(φ_bl)는 `k_dry=f_w·k_lub`, lub 패치(1−φ_bl)는 `k_lub`. φ_bl 은 [0,1] 클램프.
#[inline]
pub fn wear_coefficient(phi_bl: f64, params: &WearParams) -> f64 {
    let phi = phi_bl.clamp(0.0, 1.0);
    phi * params.k_dry() + (1.0 - phi) * params.k_lub
}

/// **Q5 환산**: 사이클당 미끄럼거리 `s_cycle = u_s·(ℓ_c/ū)` [m/cycle].
///
/// `ℓ_c`=접촉폭[m], `ū=u_mean`=평균 구름속도[m/s]. 체류시간 `τ_c=ℓ_c/ū`[s]×미끄럼속도.
/// `ū≤0`(구름 없음) 또는 `ℓ_c≤0`(비물리 접촉) → 0 (마모 없음, 사용처 방어).
#[inline]
pub fn sliding_distance_per_cycle(u_s: f64, u_mean: f64, l_c: f64) -> f64 {
    if u_mean <= 0.0 || l_c <= 0.0 {
        return 0.0;
    }
    u_s * (l_c / u_mean)
}

/// **Archard 깊이 커널** `Δh_w/Δn = k·p·s_cycle/H` [m/cycle] (식[14] 격자점 형).
///
/// `k`[-]·`p`[Pa]·`s_cycle`[m/cycle] / `H`[Pa] = [m/cycle] (CV-M5-Dim). 접촉 밖(`p≤0`)
/// 또는 `H≤0`(비물리) → 0. `p` 는 압축(양수)만 마모 기여.
#[inline]
pub fn wear_depth_rate(k: f64, p: f64, s_cycle: f64, h: f64) -> f64 {
    if h <= 0.0 || p <= 0.0 {
        return 0.0;
    }
    k * p * s_cycle / h
}

// ─────────────────────────────────────────────────────────────────────────
//  진입점 — WearInput → WearResult
// ─────────────────────────────────────────────────────────────────────────

/// M5 진입점: M6 전이압 [`WearInput`] → 사이클당 마모깊이 [`WearResult`].
///
/// 파이프라인: (1) `b=2·r_x·p_h/e_red` → `ℓ_c=2b`([`CONTACT_WIDTH_FACTOR`]) →
/// `s_cycle=u_s·ℓ_c/ū`(Q5) → (2) `k=φ_bl·k_dry+(1−φ_bl)·k_lub` → (3) 격자점별
/// `dh_w=k·max(p,0)·s_cycle/H`(식[14]) → (4) `dh_w_mean=(1/A)∫dh_w dA`(식[15], 격자평균).
pub fn solve_wear(input: &WearInput, params: &WearParams) -> WearResult {
    let nx = input.grid.nx;
    let ny = input.grid.ny;
    // 퇴화 방어(빈 격자).
    if nx == 0 || ny == 0 {
        return WearResult {
            dh_w: Field2::zeros(nx, ny),
            dh_w_mean: 0.0,
        };
    }

    // (1) 접촉폭 → 사이클당 미끄럼거리 (Q5). CRB 재사용: contact_half_width.
    let b = contact_half_width(&input.op, &input.mat);
    let l_c = CONTACT_WIDTH_FACTOR * b;
    let u_s = sliding_speed(&input.op);
    let s_cycle = sliding_distance_per_cycle(u_s, input.op.u_mean, l_c);

    // (2) 유효 마모계수 (φ_bl 가중; 시료 내 균일 가정 → 스칼라).
    let k = wear_coefficient(input.phi_bl, params);
    let h = input.mat.hardness;

    // (3) 격자점별 식[14].
    let mut dh_w = Field2::zeros(nx, ny);
    let mut acc = 0.0f64;
    for j in 0..ny {
        for i in 0..nx {
            let p = input.p_tran.at(i, j);
            let v = wear_depth_rate(k, p, s_cycle, h);
            dh_w.set(i, j, v);
            acc += v;
        }
    }

    // (4) 식[15] 면적평균 = (1/A)∫dh_w dA = 균일격자 단순평균(창 전면적 기준, 접촉밖 0 포함).
    let dh_w_mean = acc / (nx * ny) as f64;

    WearResult { dh_w, dh_w_mean }
}

// ═════════════════════════════════════════════════════════════════════════
//  오라클 / 테스트
// ═════════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::E_RED_STEEL_PA;
    use approx::assert_relative_eq;

    /// 테스트용 표준 운전조건 (마모 관련 필드만 유의미; 나머지는 더미).
    fn op_std(u_mean: f64, slide_roll: f64, r_x: f64, p_h: f64) -> OperatingConditions {
        OperatingConditions {
            p_h,
            u_mean,
            u2: 0.0,
            slide_roll,
            eta0: 0.01,
            alpha_visc: 20e-9,
            tau0: 5e6,
            temp: 353.0,
            r_x,
        }
    }

    fn mat_std(hardness: f64) -> MaterialProps {
        MaterialProps {
            e_red: E_RED_STEEL_PA,
            nu: 0.30,
            hardness,
            p_lim: 4.3e9,
        }
    }

    // ── VC-M5-Archard(1, 해석해): 식[14] 손유도 정확값 + Archard 원식 교차 ──
    //
    // 알려진 k·p·s_cycle·H 로 Δh_w 정확값 대조. 계수·H위치(분모)·u_s 선형성·환산 각 축을
    // 손유도 값으로 고정 → 어느 축의 변이(예 H를 분자로, k 누락, /3 삽입)든 FAIL(검출력).
    #[test]
    fn vc_m5_archard_closed_form() {
        // 손유도 입력.
        let k = 1.0e-9;
        let p = 1.0e9; // Pa
        let h = 7.0e9; // Pa
        let s_cycle = 1.0e-5; // m/cycle
        // Δh_w = k·p·s_cycle/H = 1e-9·1e9·1e-5/7e9 = 1e-5/7e9 = 1.428571…e-15 m/cycle.
        let expect = k * p * s_cycle / h;
        assert_relative_eq!(expect, 1.428_571_43e-15, max_relative = 1e-6);
        assert_relative_eq!(wear_depth_rate(k, p, s_cycle, h), expect, max_relative = 1e-12);

        // H 위치(분모) 검출: H 2배 → dh_w 절반.
        assert_relative_eq!(
            wear_depth_rate(k, p, s_cycle, 2.0 * h),
            0.5 * expect,
            max_relative = 1e-12
        );
        // k·p 곱 선형: k 3배 → 3배, p 3배 → 3배.
        assert_relative_eq!(wear_depth_rate(3.0 * k, p, s_cycle, h), 3.0 * expect, max_relative = 1e-12);
        assert_relative_eq!(wear_depth_rate(k, 3.0 * p, s_cycle, h), 3.0 * expect, max_relative = 1e-12);

        // k=K/3 관계식 **문서화(대수 항등, 검출력 無)**: 식[14] k 는 Archard 깊이형 K/3 에
        // 대응(기하 /3 인자가 경험적 k 에 흡수 — 모듈 docstring flag). /3 은 관측불가 흡수상수라
        // 외부 검증 아님. 실제 물리 앵커 = 위 독립 손계산 리터럴(1.428e-15)·H위치·k·p 선형성.
        let big_k = 3.0e-9; // Archard K
        assert_relative_eq!(
            big_k * p / (3.0 * h) * s_cycle,
            wear_depth_rate(big_k / 3.0, p, s_cycle, h),
            max_relative = 1e-12
        );
    }

    // ── CV-M5-Dim(2, 차원): 식[14] 우변 = m/cycle; Q5 환산계수 적용 항등 ──
    //
    // 환산계수 ℓ_c/ū 적용 시 차원 정합. **환산 누락**(u_s[m/s] 직접 사용)이면 결과가
    // ℓ_c/ū 배 어긋남 → 환산 누락/오위치 검출. s_cycle 이 실제로 u_s·ℓ_c/ū 임을 항등검증.
    #[test]
    fn cv_m5_dim_conversion() {
        let u_s = 0.6; // m/s
        let u_mean = 12.0; // m/s
        let l_c = 2.4e-4; // m (=2b)
        let s_cycle = sliding_distance_per_cycle(u_s, u_mean, l_c);
        // s_cycle = 0.6·(2.4e-4/12) = 0.6·2e-5 = 1.2e-5 m/cycle (손계산).
        assert_relative_eq!(s_cycle, 1.2e-5, max_relative = 1e-12);

        // 환산계수 = ℓ_c/ū = s_cycle/u_s [s]. 환산 누락(=u_s 직접)이면 비율 = ℓ_c/ū ≠ 1.
        let conv = s_cycle / u_s; // ℓ_c/ū
        assert_relative_eq!(conv, l_c / u_mean, max_relative = 1e-12);
        assert!((conv - 1.0).abs() > 1e-9, "conversion factor must differ from 1 (dim change)");

        // 차원 항등: dh_w[m/cycle] = k·p[Pa]·s_cycle[m/cycle]/H[Pa]. p/H 무차원 → m/cycle 유지.
        let dh = wear_depth_rate(2.0e-10, 1.5e9, s_cycle, 7.0e9);
        let manual = 2.0e-10 * (1.5e9 / 7.0e9) * s_cycle; // (p/H)·k·s_cycle
        assert_relative_eq!(dh, manual, max_relative = 1e-12);

        // 구름 없음(ū≤0)·비물리 접촉폭(ℓ_c≤0) → s_cycle=0.
        assert_eq!(sliding_distance_per_cycle(u_s, 0.0, l_c), 0.0);
        assert_eq!(sliding_distance_per_cycle(u_s, u_mean, 0.0), 0.0);
    }

    // ── RP-klub(4, 정량): k_lub∈[1e-11,5e-10]·f_w≈10 → k_dry 범위·φ_bl 가중 극값 ──
    #[test]
    fn rp_klub_coefficients() {
        let p = WearParams::default();
        // 상수 확정(0% 허용).
        assert_relative_eq!(p.k_lub, 4.0e-10, max_relative = 0.0);
        assert_relative_eq!(p.f_w, 10.0, max_relative = 0.0);
        assert_relative_eq!(p.k_dry(), 4.0e-9, max_relative = 1e-15);
        // k_lub 범위 [1e-11(외삽 하한), 5e-10].
        assert!(K_LUB_ADDITIVE >= 1.0e-11 - 1e-13 && K_LUB <= 5.0e-10);
        // φ_bl 가중 극값: φ=0 → k_lub, φ=1 → k_dry.
        assert_relative_eq!(wear_coefficient(0.0, &p), p.k_lub, max_relative = 1e-15);
        assert_relative_eq!(wear_coefficient(1.0, &p), p.k_dry(), max_relative = 1e-15);
        // φ=0.5 → 중간값.
        assert_relative_eq!(
            wear_coefficient(0.5, &p),
            0.5 * (p.k_lub + p.k_dry()),
            max_relative = 1e-15
        );
        // 클램프: φ<0·φ>1 방어.
        assert_relative_eq!(wear_coefficient(-0.3, &p), p.k_lub, max_relative = 1e-15);
        assert_relative_eq!(wear_coefficient(1.4, &p), p.k_dry(), max_relative = 1e-15);
    }

    // ── RP-klub 크기차수: 대표 베어링 조건 마모율 크기 (물리 타당성) ──
    //
    // p_h=1.5 GPa, u_mean=10 m/s, SRR=0.1(u_s=1 m/s), r_x=20 mm, H=7 GPa, φ_bl=0.3.
    // b=2·0.02·1.5e9/E_red. s_cycle=u_s·2b/u_mean. dh_w~k·p·s_cycle/H → nm/cycle 이하 규모.
    #[test]
    fn rp_klub_order_of_magnitude() {
        let op = op_std(10.0, 0.1, 0.02, 1.5e9);
        let mat = mat_std(7.0e9);
        let grid = Grid::new(8, 4, 1e-4, 5e-5);
        let p_tran = Field2::filled(8, 4, 1.5e9); // 균일 접촉압
        let input = WearInput {
            grid,
            p_tran: &p_tran,
            op,
            mat,
            phi_bl: 0.3,
        };
        let r = solve_wear(&input, &WearParams::default());

        // 손유도 대조.
        let b = 2.0 * 0.02 * 1.5e9 / E_RED_STEEL_PA;
        let s_cycle = 1.0 * (2.0 * b / 10.0); // u_s=|0.1·10|=1
        let k = wear_coefficient(0.3, &WearParams::default());
        let expect = k * 1.5e9 * s_cycle / 7.0e9;
        assert_relative_eq!(r.dh_w.at(0, 0), expect, max_relative = 1e-10);
        // 균일압 → 평균 = 점값.
        assert_relative_eq!(r.dh_w_mean, expect, max_relative = 1e-10);
        // 크기차수(기본 파라미터): 실측 ~3.3e-14 m/cycle. 밴드 [1e-15, 1e-12] 로 10²+ 배 오차
        // 포착(정밀값은 위 hand-composed expect 가 pin; 이 밴드는 gross OoM sanity, 종전 <1e-9 강화).
        assert!(
            r.dh_w_mean > 1e-15 && r.dh_w_mean < 1e-12,
            "wear rate off scale: {}",
            r.dh_w_mean
        );
    }

    // ── 정성(3): p·u_s 단조증가, φ_bl↑(dry↑) 마모↑, 접촉 밖 0 ──
    #[test]
    fn qualitative_monotonic() {
        let mat = mat_std(7.0e9);
        let grid = Grid::new(4, 2, 1e-4, 5e-5);
        let params = WearParams::default();
        let mk = |p_val: f64, slide_roll: f64, phi: f64| -> f64 {
            let op = op_std(10.0, slide_roll, 0.02, 1.5e9);
            let p_tran = Field2::filled(4, 2, p_val);
            let input = WearInput {
                grid,
                p_tran: &p_tran,
                op,
                mat,
                phi_bl: phi,
            };
            solve_wear(&input, &params).dh_w_mean
        };
        let base = mk(1.0e9, 0.1, 0.3);
        // (a) 압력 2배 → 마모 2배.
        assert_relative_eq!(mk(2.0e9, 0.1, 0.3), 2.0 * base, max_relative = 1e-9);
        // (b) u_s(SRR) 2배 → 마모 2배(선형).
        assert_relative_eq!(mk(1.0e9, 0.2, 0.3), 2.0 * base, max_relative = 1e-9);
        // (c) φ_bl↑(0.3→0.8, dry 패치↑) → 마모↑ (k_dry>k_lub).
        assert!(mk(1.0e9, 0.1, 0.8) > base, "more dry patches must raise wear");
        // (d) 접촉 밖 p=0 → dh_w=0.
        assert_eq!(mk(0.0, 0.1, 0.3), 0.0);
        assert_eq!(mk(-1.0e9, 0.1, 0.3), 0.0); // 인장(음압)도 0
    }

    // ── 면적평균(식[15]): 부분접촉(절반만 접촉) → 평균 = 접촉점 평균 ──
    #[test]
    fn area_average_partial_contact() {
        let op = op_std(10.0, 0.1, 0.02, 1.5e9);
        let mat = mat_std(7.0e9);
        let grid = Grid::new(4, 1, 1e-4, 5e-5);
        // 4점 중 2점만 접촉(p=1 GPa), 2점 비접촉(0).
        let p_tran = Field2::from_vec(4, 1, vec![1.0e9, 1.0e9, 0.0, 0.0]);
        let input = WearInput {
            grid,
            p_tran: &p_tran,
            op,
            mat,
            phi_bl: 0.3,
        };
        let r = solve_wear(&input, &WearParams::default());
        // 접촉점 값.
        let vc = r.dh_w.at(0, 0);
        assert!(vc > 0.0);
        assert_eq!(r.dh_w.at(2, 0), 0.0);
        // 면적평균 = (vc+vc+0+0)/4 = vc/2 (창 전면적 기준).
        assert_relative_eq!(r.dh_w_mean, 0.5 * vc, max_relative = 1e-12);
    }

    // ── 퇴화 방어: 빈 격자 → 0 결과 ──
    #[test]
    fn degenerate_empty() {
        let op = op_std(10.0, 0.1, 0.02, 1.5e9);
        let mat = mat_std(7.0e9);
        let grid = Grid::new(0, 0, 0.0, 0.0);
        let p_tran = Field2::zeros(0, 0);
        let input = WearInput {
            grid,
            p_tran: &p_tran,
            op,
            mat,
            phi_bl: 0.3,
        };
        let r = solve_wear(&input, &WearParams::default());
        assert!(r.dh_w.is_empty() && r.dh_w_mean == 0.0);
    }

    // ── sliding_speed: u_s = |SRR·u_mean| ──
    #[test]
    fn sliding_speed_srr() {
        let op = op_std(8.0, -0.25, 0.02, 1.5e9);
        assert_relative_eq!(sliding_speed(&op), 2.0, max_relative = 1e-12); // |−0.25·8|=2
    }
}
