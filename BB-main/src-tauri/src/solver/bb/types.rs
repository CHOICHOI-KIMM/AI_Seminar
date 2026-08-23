// BB Contact Analysis — 데이터 모델 (ACBB, Angular Contact Ball Bearing)
//
// BB Phase 1-S2 (2026-08-20): CRB(원통롤러) 데이터 모델을 백지에서 전면 재작성.
//
// ── 규약 (Plan §3.4) ────────────────────────────────────────────────
//  D-7  좌표계: ISO 16281 규약 — X = 회전축, Y·Z = 반경방향, 우수좌표계
//       미지수 (δ_x, δ_y, δ_z, γ_y, γ_z) / 잔차 (F_x, F_y, F_z, M_y, M_z)
//       구속 δ_z = γ_y = 0 → ISO 3-DOF (A.6)(A.7)(A.8) 과 항등
//  D-8  볼 각위치: φ_j = 2π(j−1)/Z 고정 원점. 케이지 위상 스윕은 별도 옵션
//  D-9  틸트 모멘트 팔: R_i (식 A.4). d_pw_mm/2 아님
//  D-10 단위: 본 파일의 모든 값은 **mm · N · rad · MPa** (솔버 내부 단위).
//       μm·kN·° 는 UI 표시 전용이며 이 파일에 등장하지 않는다.
//       **모든 유차원 필드는 이름에 단위 접미사를 붙인다** (`_mm`, `_n`, `_nmm`,
//       `_rad`, `_mpa`, `_per_mm`, `_g_cm3`, `_rpm`, `_c`). 무차원 필드(`nu`,
//       `gamma`, `z`, `f_rho_i`)는 접미사 없음. 프론트엔드·JSON 을 거치며
//       단위가 오독되는 것을 이름 수준에서 차단한다 (P1-S3 결정).
//
// ── 수식 근거 ───────────────────────────────────────────────────────
//  BB_Development_Theory.md. 식 번호는 ISO 16281:2025 (A.x / 숫자) 기준.
//  본 파일은 수식을 구현하지 않는다 — 자료구조와 검증만 담당.

use serde::{Deserialize, Serialize};

use crate::error::SolverError;
use crate::solver::common::types::{Alert, Material};

// ═══════════════════════════════════════════════════════════════════
//  입력 — 기하
// ═══════════════════════════════════════════════════════════════════

/// 클리어런스 / 예압 지정 방식 (D-2).
///
/// ISO 16281 은 예압을 직접 다루지 않는다. 셋 중 무엇을 주든 내부적으로
/// 초기 접촉각 `α₀` (식 A.1) 로 환산해 사용한다.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BbClearanceSpec {
    /// 직경으로 측정한 반경 운전 클리어런스 `G_r op` [mm].
    /// **직경 기준**임에 주의 (식 A.1 의 분모가 2A). 음수면 예압 상태.
    DiametralMm(f64),
    /// 초기 접촉각 `α₀` [rad] 직접 지정. ACBB 사양서가 통상 이 형태.
    InitialAngleRad(f64),
    /// 축방향 예압 하중 `F_a0` [N]. P3 에서 사전 해석으로 `δ_x0` 를 역산한다.
    AxialPreloadN(f64),
}

/// ACBB 매크로 기하. 모든 길이 [mm].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BallBearingGeometry {
    /// 내경 d [mm]
    pub bore_mm: f64,
    /// 외경 D [mm]
    pub outer_diameter_mm: f64,
    /// 폭 B [mm]
    pub width_mm: f64,
    /// 볼 수 Z
    pub z: u32,
    /// 볼 직경 D_w [mm]
    pub d_w_mm: f64,
    /// 볼 세트 피치직경 D_pw [mm]
    pub d_pw_mm: f64,
    /// 내륜 홈 단면 곡률반경 r_i_mm [mm].
    /// ISO 16281 Annex B.2 참조기하 기본값 = 0,52 D_w
    pub r_i_mm: f64,
    /// 외륜 홈 단면 곡률반경 r_e_mm [mm].
    /// ISO 16281 Annex B.2 참조기하 기본값 = 0,53 D_w
    pub r_e_mm: f64,
    /// 공칭 접촉각 α [rad]. **ISO 281 정격하중 계산 전용.**
    /// 내부 하중분포에는 초기 접촉각 α₀ 를 쓴다 (ISO 16281 Clause 3.5 NOTE).
    pub alpha_nom_rad: f64,
    /// 클리어런스 / 예압
    pub clearance: BbClearanceSpec,
}

impl BallBearingGeometry {
    /// ISO 16281 Annex B.2 참조 홈 반경 (사양 미상 시 기본값).
    pub fn reference_groove_radii(d_w_mm: f64) -> (f64, f64) {
        (0.52 * d_w_mm, 0.53 * d_w_mm)
    }

    pub fn validate(&self) -> Result<(), SolverError> {
        let e = |m: &str| Err(SolverError::InvalidGeometry(m.to_string()));
        if self.d_w_mm <= 0.0 {
            return e("볼 직경 D_w 는 양수여야 합니다");
        }
        if self.d_pw_mm <= self.d_w_mm {
            return e("피치직경 D_pw 는 볼 직경 D_w 보다 커야 합니다");
        }
        if self.z < 3 {
            return e("볼 수 Z 는 3 이상이어야 합니다");
        }
        // 홈 반경은 볼 반경보다 커야 접촉이 성립 (오목면)
        if self.r_i_mm <= self.d_w_mm / 2.0 || self.r_e_mm <= self.d_w_mm / 2.0 {
            return e("홈 곡률반경 r_i_mm, r_e_mm 는 볼 반경 D_w/2 보다 커야 합니다");
        }
        // A = r_i_mm + r_e_mm − D_w 가 양수여야 α₀ 정의 가능 (식 A.3)
        if self.r_i_mm + self.r_e_mm - self.d_w_mm <= 0.0 {
            return e("A = r_i_mm + r_e_mm − D_w 가 양수가 아닙니다");
        }
        if self.outer_diameter_mm <= self.bore_mm {
            return e("외경 D 는 내경 d 보다 커야 합니다");
        }
        if let BbClearanceSpec::InitialAngleRad(a) = self.clearance {
            if !(0.0..std::f64::consts::FRAC_PI_2).contains(&a) {
                return e("초기 접촉각 α₀ 는 [0, π/2) 범위여야 합니다");
            }
        }
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════
//  입력 — 운전조건
// ═══════════════════════════════════════════════════════════════════

/// 외부 하중과 운전조건. 좌표계는 D-7 (ISO 규약, X = 회전축).
///
/// 부호 규약: 내륜에 작용하는 하중을 양으로 본다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BbOperatingConditions {
    /// 축하중 F_x [N] (X = 회전축). ISO 식 (A.7) 의 F_a
    pub f_x_n: f64,
    /// 반경하중 F_y [N] (Y). ISO 식 (A.6) 의 F_r
    pub f_y_n: f64,
    /// 반경하중 F_z [N] (Z). 5-DOF 확장 성분
    #[serde(default)]
    pub f_z_n: f64,
    /// 모멘트 M_y [N·mm] (about Y). 5-DOF 확장 성분
    #[serde(default)]
    pub m_y_nmm: f64,
    /// 모멘트 M_z [N·mm] (about Z). ISO 식 (A.8) 의 M_z
    #[serde(default)]
    pub m_z_nmm: f64,
    /// 내륜 회전속도 [r/min]
    pub n_inner_rpm: f64,
    /// 외륜 회전속도 [r/min]. 통상 0 (고정)
    #[serde(default)]
    pub n_outer_rpm: f64,
    /// 운전 온도 [°C] — P5 윤활 계산용
    #[serde(default = "default_temperature")]
    pub temperature_c: f64,
}

fn default_temperature() -> f64 {
    70.0
}

impl BbOperatingConditions {
    /// 반경하중 합성 크기 [N]
    pub fn radial_magnitude(&self) -> f64 {
        (self.f_y_n * self.f_y_n + self.f_z_n * self.f_z_n).sqrt()
    }

    /// 반경하중 방향각 [rad] (Y축 기준, Z 방향으로 양)
    pub fn radial_angle(&self) -> f64 {
        self.f_z_n.atan2(self.f_y_n)
    }

    /// 상대 회전속도 [r/min]. 링 사이의 상대 각속도.
    pub fn relative_speed_rpm(&self) -> f64 {
        self.n_inner_rpm - self.n_outer_rpm
    }
}

// ═══════════════════════════════════════════════════════════════════
//  입력 — 해석 설정
// ═══════════════════════════════════════════════════════════════════

/// 한 자유도의 경계조건.
///
/// - `Free` — 미지수. 그 방향의 외력(또는 외부 모멘트)이 주어지고 변위가 해가 된다 (**하중 제어**)
/// - `Prescribed(v)` — 변위를 `v` 로 고정한다. 그 방향의 반력이 결과가 된다 (**변위 제어**)
///
/// 단위: `δ_x`·`δ_y`·`δ_z` 는 [mm], `γ_y`·`γ_z` 는 [rad].
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BbDof {
    Free,
    Prescribed(f64),
}

impl BbDof {
    pub fn is_free(&self) -> bool {
        matches!(self, BbDof::Free)
    }
    /// 구속값 (자유면 0)
    pub fn value(&self) -> f64 {
        match self {
            BbDof::Free => 0.0,
            BbDof::Prescribed(v) => *v,
        }
    }
}

/// 5-DOF 각각의 경계조건 (D-1).
///
/// `ISO_3DOF` = `δ_z`·`γ_y` 를 0 으로 구속 → ISO 16281 A.2.2 와 항등.
/// Level D-1 (Harris Table 7.4 대조) 은 이 모드에서 수행한다.
///
/// **강체(스페이서) 예압**은 `x: Prescribed(δ_x0)` 로 표현된다 — 별도 기구가 아니라
/// 같은 구속 메커니즘이다 (P3-1 결정).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct BbDofMask {
    pub x: BbDof,
    pub y: BbDof,
    pub z: BbDof,
    pub gy: BbDof,
    pub gz: BbDof,
}

impl BbDofMask {
    /// 5-DOF 전 자유도 해방
    pub const FULL: Self = Self {
        x: BbDof::Free,
        y: BbDof::Free,
        z: BbDof::Free,
        gy: BbDof::Free,
        gz: BbDof::Free,
    };

    /// ISO 16281 Annex A.2 정식화와 항등인 3-DOF 구속 (δ_a, δ_r, ψ)
    pub const ISO_3DOF: Self = Self {
        x: BbDof::Free,
        y: BbDof::Free,
        z: BbDof::Prescribed(0.0),
        gy: BbDof::Prescribed(0.0),
        gz: BbDof::Free,
    };

    pub fn as_array(&self) -> [BbDof; 5] {
        [self.x, self.y, self.z, self.gy, self.gz]
    }

    pub fn count_free(&self) -> usize {
        self.as_array().iter().filter(|d| d.is_free()).count()
    }
}

impl Default for BbDofMask {
    fn default() -> Self {
        Self::FULL
    }
}

/// 축방향 예압 모델 (D-2, P3-1 결정).
///
/// 두 모델은 **하중 조건이 아니라 경계조건**이 다르다.
///
/// | | `Spring` | `Rigid` |
/// |---|---|---|
/// | 실물 | 웨이브 와셔·스프링 | 듀플렉스 조합·스페이서·로크너트 |
/// | 경계조건 | 하중 제어 — `F_a0` 를 외부 축하중에 더한다 | 변위 제어 — `F_a0` 로 역산한 `δ_x0` 를 구속한다 |
/// | 외부 축하중 | 예압에 더해짐 | 예압 변위 고정, 반력이 변함 |
///
/// `Rigid` 는 `δ_x` 를 구속하므로 **외부 축하중을 독립적으로 받을 수 없다**
/// (실물에서 가능한 이유는 짝 베어링이 반력을 받기 때문이며, 그것은 단열 모델 범위 밖이다).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub enum BbPreloadModel {
    /// 정력(스프링) 예압 — 기본값
    #[default]
    Spring,
    /// 강체(스페이서) 예압 — `F_a0` 로부터 `δ_x0` 를 역산해 구속
    Rigid,
}

/// 케이지 위상 스윕 설정 (D-8).
///
/// `φ_j = φ₀ + 2π(j−1)/Z` 의 `φ₀` 를 `[0, 2π/Z)` 로 `n_phase` 분할하여
/// Q_max·p_H·수명의 **최악값**과 발생 위상을 구한다.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BbPhaseSweep {
    pub enabled: bool,
    /// 분할 수. 볼 해석은 O(Z) 라 36 분할도 밀리초 단위.
    pub n_phase: u32,
}

impl Default for BbPhaseSweep {
    fn default() -> Self {
        Self {
            enabled: false,
            n_phase: 36,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BbSolverParams {
    /// 수렴 판정 상대 잔차
    pub convergence_tol: f64,
    /// Newton-Raphson 최대 반복
    pub max_iterations: u32,
    /// 자유도 구속 (D-1)
    #[serde(default)]
    pub dof_mask: BbDofMask,
    /// 위상 스윕 (D-8)
    #[serde(default)]
    pub phase_sweep: BbPhaseSweep,
    /// 축방향 예압 모델 (D-2). `BbClearanceSpec::AxialPreloadN` 일 때만 의미가 있다
    #[serde(default)]
    pub preload_model: BbPreloadModel,
    /// 기본 동정격 반경하중 C_r [N]. `None` 이면 ISO 281 식으로 자체 산출 (P4)
    #[serde(default)]
    pub c_r_n: Option<f64>,
    /// 기본 정정격 반경하중 C_0r [N]. `None` 이면 ISO 76 식으로 자체 산출 (P4)
    #[serde(default)]
    pub c_0r_n: Option<f64>,
}

impl Default for BbSolverParams {
    fn default() -> Self {
        Self {
            convergence_tol: 1e-8,
            max_iterations: 100,
            dof_mask: BbDofMask::default(),
            phase_sweep: BbPhaseSweep::default(),
            preload_model: BbPreloadModel::default(),
            c_r_n: None,
            c_0r_n: None,
        }
    }
}

impl BbSolverParams {
    pub fn validate(&self) -> Result<(), SolverError> {
        if self.convergence_tol <= 0.0 {
            return Err(SolverError::InvalidInput(
                "수렴 판정값은 양수여야 합니다".into(),
            ));
        }
        if self.max_iterations == 0 {
            return Err(SolverError::InvalidInput(
                "최대 반복 횟수는 1 이상이어야 합니다".into(),
            ));
        }
        if self.phase_sweep.enabled && self.phase_sweep.n_phase == 0 {
            return Err(SolverError::InvalidInput(
                "위상 스윕 분할 수는 1 이상이어야 합니다".into(),
            ));
        }
        Ok(())
    }
}

/// 볼베어링 변종 (계열 = BB). 기하·접촉·평형을 공유하고 변종은 데이터로 구분한다
/// (Plan §3.6.1.3). 변종별 폴더·모듈을 파지 않는다.
///
/// ⚠ **선언값이지 추론값이 아니다.** α₀ 로 변종을 자동 판정하지 않는다 —
///    `Acbb` 로 선언된 α₀ = 0 입력은 그대로 통과한다 (Level C-7 픽스처).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BallBearingKind {
    /// 각접촉 — α₀ ≠ 0. **현재 검증 완료 범위**
    #[default]
    Acbb,
    /// 심구 — α₀ = 0 인 Acbb 의 특수해. 솔버 코어는 동일하나 수명 계수 미확보
    Dgbb,
    /// 4점접촉 — 궤도당 곡률중심 2개, 볼당 최대 4접촉. 평형 모듈 신규 필요
    FourPoint,
}

/// 최상위 입력 래퍼.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BbInput {
    /// 볼베어링 변종. 기본값 `Acbb` (§3.6.1.3)
    #[serde(default)]
    pub kind: BallBearingKind,
    pub geometry: BallBearingGeometry,
    #[serde(default)]
    pub material: Material,
    pub operating: BbOperatingConditions,
    #[serde(default)]
    pub solver: BbSolverParams,
}

impl BbInput {
    pub fn validate(&self) -> Result<(), SolverError> {
        // 변종 게이트 (§3.6.1.3 ⑤) — 「되는 줄 알았는데 안 되는」 상황을 막는다.
        match self.kind {
            BallBearingKind::Acbb => {}
            BallBearingKind::Dgbb => {
                return Err(SolverError::InvalidInput(
                    "DGBB(심구)는 아직 지원하지 않습니다: 솔버 코어(기하·접촉·5-DOF 평형)는                      ACBB 와 동일하게 동작하나, ISO 281 X/Y 계수(α = 0 행)를 아직 확보하지                      못해 수명 산출이 불가능합니다 — 신 P5(수명) 범위입니다."
                        .into(),
                ));
            }
            BallBearingKind::FourPoint => {
                return Err(SolverError::InvalidInput(
                    "4점접촉(4PCBB)은 아직 지원하지 않습니다: 궤도당 곡률중심이 2개이고                      볼당 접촉이 최대 4점(2점 ↔ 4점 전환 판정 포함)이라 평형 모듈이                      미구현입니다 — 기하·접촉(Hertz)은 그대로 재사용 가능합니다."
                        .into(),
                ));
            }
        }
        self.geometry.validate()?;
        self.material.validate()?;
        self.solver.validate()?;
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════
//  파생 — 기하 전처리 (하중 무관, 해석당 1회)
// ═══════════════════════════════════════════════════════════════════

/// 하중과 무관하게 기하만으로 확정되는 값들 (Theory §2, §8 1단계).
///
/// 해석 시작 시 1회 계산해 캐시한다. CRB 가 매 반복마다 슬라이스를 순회하던
/// 것과 달리, 볼은 이 구조체가 확정되면 반복 비용이 O(Z) 로 끝난다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BbGeometryDerived {
    /// A = r_i_mm + r_e_mm − D_w [mm] — 식 (A.3). 곡률중심 간 거리
    pub a_mm: f64,
    /// 초기 접촉각 α₀ [rad] — 식 (A.1)
    pub alpha_0_rad: f64,
    /// R_i [mm] — 식 (A.4). **틸트 모멘트 팔** (D-9, d_pw_mm/2 아님)
    pub r_i_center_mm: f64,
    /// γ = D_w cos α / D_pw — Clause 4
    pub gamma: f64,
    /// 내륜 접촉 곡률합 Σρ_i [1/mm] — 식 (E.4)
    pub sum_rho_i_per_mm: f64,
    /// 외륜 접촉 곡률합 Σρ_e [1/mm] — 식 (E.5)
    pub sum_rho_e_per_mm: f64,
    /// 내륜 상대 곡률차 F_i(ρ) — 식 (E.6)
    pub f_rho_i: f64,
    /// 외륜 상대 곡률차 F_e(ρ) — 식 (E.7)
    pub f_rho_e: f64,
    /// 등가 반경 클리어런스 G_r op [mm] (입력 지정 방식과 무관하게 환산된 값)
    pub g_r_op_mm: f64,
}

/// 점접촉 전처리 결과 (Theory §3, §6). **하중과 무관**하다.
///
/// `χ` 는 기하만으로 결정되고 `c_P` 도 그로부터 나오므로, 해석 시작 시 1회만
/// 계산해 캐시한다. CRB 의 슬라이스 강성이 하중 의존이던 것과 근본적으로 다르다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BbContactDerived {
    /// 내륜 접촉타원 형상비 χ = a/b — 식 (E.1) 의 해
    pub chi_inner: f64,
    /// 외륜 접촉타원 형상비 χ = a/b
    pub chi_outer: f64,
    /// 제1종 완전타원적분 K(χ_i) — 식 (E.2)
    pub k_ellip_inner: f64,
    /// 제2종 완전타원적분 E(χ_i) — 식 (E.3)
    pub e_ellip_inner: f64,
    /// 제1종 완전타원적분 K(χ_e)
    pub k_ellip_outer: f64,
    /// 제2종 완전타원적분 E(χ_e)
    pub e_ellip_outer: f64,
    /// 무차원 장반경 계수 a* — Harris 식 (6.44)
    pub a_star_inner: f64,
    /// 무차원 단반경 계수 b* — Harris 식 (6.45)
    pub b_star_inner: f64,
    /// 무차원 접근량 계수 δ* — Harris 식 (6.46)
    pub delta_star_inner: f64,
    pub a_star_outer: f64,
    pub b_star_outer: f64,
    pub delta_star_outer: f64,
    /// 등가 탄성계수 E* [MPa]
    pub e_star_mpa: f64,
    /// 점접촉 스프링상수 c_P [N/mm^(3/2)] — 식 (40). `Q = c_P δ^(3/2)`
    pub c_p_n_per_mm15: f64,
}

// ═══════════════════════════════════════════════════════════════════
//  결과
// ═══════════════════════════════════════════════════════════════════

/// 볼 1개의 해석 결과.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BallResult {
    /// 각위치 φ_j [rad] (D-8: φ_1 = 0 이 Y축 방향)
    pub phi_rad: f64,
    /// 총 탄성변형 δ_j [mm] — 식 (A.2). 비접촉이면 0
    pub delta_mm: f64,
    /// 운전 접촉각 α_j [rad] — 식 (A.5)
    pub alpha_rad: f64,
    /// 볼 하중 Q_j [N] — 식 (39)
    pub q_n: f64,
    /// 접촉 여부 (식 A.2 우변 > 0)
    pub loaded: bool,
    /// 내륜 접촉타원 장반경 a [mm] — Harris (6.38). P2 이전에는 0
    #[serde(default)]
    pub a_inner_mm: f64,
    /// 내륜 접촉타원 단반경 b [mm] — Harris (6.40)
    #[serde(default)]
    pub b_inner_mm: f64,
    /// 내륜 최대 접촉응력 [MPa] — Harris (6.25)
    #[serde(default)]
    pub p_max_inner_mpa: f64,
    /// 외륜 접촉타원 장반경 a [mm]
    #[serde(default)]
    pub a_outer_mm: f64,
    /// 외륜 접촉타원 단반경 b [mm]
    #[serde(default)]
    pub b_outer_mm: f64,
    /// 외륜 최대 접촉응력 [MPa]
    #[serde(default)]
    pub p_max_outer_mpa: f64,
}

/// 5-DOF 평형 변위 (D-7 좌표계: X = 회전축).
///
/// ⚠ CRB 의 `[f64;5]` 는 `[δx, δy, δz=0, γx, γy=0]` 로 **인덱스 3의 의미가 다르다**
///    (CRB 는 γx, BB 는 γy). 배열을 쓰면 타입 검사를 통과하면서 조용히 틀린다
///    (Plan §3.6.1.2 충돌 3).
///
/// ⚠ 위치는 `bb/` 다. BB 가 이 규약의 SSOT 이나(§3.6.1.8) CRB·TRB 가 실제로
///    채택하는 시점에 `common/` 으로 승격한다 (§3.6.1.6 주석).
///
/// 필드명의 단위 접미사는 필수다 — 배열이던 시절엔 A-8 단위검사를 우회했다.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Displacement {
    pub dx_mm: f64,
    pub dy_mm: f64,
    pub dz_mm: f64,
    pub ry_rad: f64,
    pub rz_rad: f64,
}

/// 5-DOF 평형해 (D-7 좌표계).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BbEquilibrium {
    /// 평형 변위 (δ_x, δ_y, δ_z [mm] · γ_y, γ_z [rad])
    pub displacement: Displacement,
    /// 볼별 결과 (φ_j 오름차순)
    pub ball_results: Vec<BallResult>,
    /// 최대 볼 하중 Q_max [N]
    pub q_max_n: f64,
    /// 하중을 받는 볼 수
    pub loaded_count: u32,
    pub converged: bool,
    pub iterations: u32,
    /// 최종 상대 잔차
    pub residual_norm: f64,
}

/// 위상 스윕 결과 (D-8). `BbPhaseSweep::enabled` 일 때만 채워진다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BbPhaseSweepResult {
    /// 최악 Q_max [N] 와 그때의 φ₀ [rad]
    pub worst_q_max_n: f64,
    pub worst_q_max_phase_rad: f64,
    /// 최악 최대접촉응력 [MPa] 와 그때의 φ₀ [rad]
    pub worst_p_max_mpa: f64,
    pub worst_p_max_phase_rad: f64,
    /// (φ₀, Q_max) 전 이력
    pub curve: Vec<(f64, f64)>,
}

/// 자동 산출된 기하 요약 (UI 표시·검산용).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BbGeometrySummary {
    pub a_mm: f64,
    /// 초기 접촉각 [rad] (표시는 UI 에서 ° 로 변환)
    pub alpha_0_rad: f64,
    pub r_i_center_mm: f64,
    pub gamma: f64,
    pub sum_rho_i_per_mm: f64,
    pub sum_rho_e_per_mm: f64,
    pub f_rho_i: f64,
    pub f_rho_e: f64,
    pub g_r_op_mm: f64,
    /// 오스큘레이션 f_i = r_i_mm / D_w
    pub osculation_inner: f64,
    /// 오스큘레이션 f_e = r_e_mm / D_w
    pub osculation_outer: f64,
    /// 볼 1개 질량 [g]
    pub ball_mass_g: f64,
    /// n·D_pw [mm/min] — ISO 16281 A.4 의 고속 판정 지표 (D-3)
    pub n_dpw_mm_per_min: f64,
}

/// 정상상태 해석 결과 최상위.
///
/// 수명(P4)·윤활(P5) 결과는 해당 Phase 에서 필드를 추가한다 (P1-S2 결정).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BbResult {
    pub geometry: BbGeometrySummary,
    pub equilibrium: BbEquilibrium,
    #[serde(default)]
    pub phase_sweep: Option<BbPhaseSweepResult>,
    pub alerts: Vec<Alert>,
    pub elapsed_ms: f64,
}

// ═══════════════════════════════════════════════════════════════════
//  테스트
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    /// 검증용 ACBB 픽스처.
    /// 경계치수 d/D/B 는 ISO 15 치수계열 7210 형번 기준이나,
    /// Z 와 D_w 는 제조사별 값이라 **가정값**이다 (실 카탈로그 미확인).
    fn fixture() -> BallBearingGeometry {
        let d_w_mm = 11.5;
        let (r_i_mm, r_e_mm) = BallBearingGeometry::reference_groove_radii(d_w_mm);
        BallBearingGeometry {
            bore_mm: 50.0,
            outer_diameter_mm: 90.0,
            width_mm: 20.0,
            z: 16,
            d_w_mm,
            d_pw_mm: 70.0,
            r_i_mm,
            r_e_mm,
            alpha_nom_rad: 40.0_f64.to_radians(),
            clearance: BbClearanceSpec::InitialAngleRad(40.0_f64.to_radians()),
        }
    }

    #[test]
    fn reference_groove_radii_matches_annex_b2() {
        // ISO 16281 Annex B.2: r_i_mm = 0,52 D_w, r_e_mm = 0,53 D_w
        let (r_i_mm, r_e_mm) = BallBearingGeometry::reference_groove_radii(10.0);
        assert!((r_i_mm - 5.2).abs() < 1e-12);
        assert!((r_e_mm - 5.3).abs() < 1e-12);
    }

    #[test]
    fn fixture_is_valid() {
        assert!(fixture().validate().is_ok());
    }

    #[test]
    fn rejects_ball_larger_than_pitch() {
        let mut g = fixture();
        g.d_pw_mm = g.d_w_mm;
        assert!(g.validate().is_err());
    }

    #[test]
    fn rejects_groove_radius_below_ball_radius() {
        let mut g = fixture();
        g.r_i_mm = g.d_w_mm / 2.0 - 1e-9;
        assert!(g.validate().is_err());
    }

    #[test]
    fn rejects_too_few_balls() {
        let mut g = fixture();
        g.z = 2;
        assert!(g.validate().is_err());
    }

    #[test]
    fn a_distance_is_positive_for_reference_geometry() {
        // A = r_i_mm + r_e_mm − D_w = (0,52 + 0,53 − 1) D_w = 0,05 D_w
        let g = fixture();
        let a = g.r_i_mm + g.r_e_mm - g.d_w_mm;
        assert!((a - 0.05 * g.d_w_mm).abs() < 1e-12);
        assert!(a > 0.0);
    }

    #[test]
    fn material_default_matches_iso_notes() {
        // ISO 16281 Clause 4 NOTE 1 / NOTE 6
        let m = Material::default();
        assert!((m.e_ball_mpa - 207_000.0).abs() < 1e-9);
        assert!((m.nu - 0.3).abs() < 1e-12);
        assert!(m.validate().is_ok());
    }

    #[allow(clippy::field_reassign_with_default)]
    #[test]
    fn material_rejects_invalid_poisson() {
        let mut m = Material::default();
        m.nu = 0.5;
        assert!(m.validate().is_err());
    }

    // 상수 마스크의 회귀 가드. clippy 는 const 폴딩이라 경고하지만,
    // 누가 ISO_3DOF 의 자유도를 바꾸면 여기서 잡힌다.
    #[allow(clippy::assertions_on_constants)]
    #[test]
    fn iso_3dof_mask_constrains_two_dof() {
        assert_eq!(BbDofMask::ISO_3DOF.count_free(), 3);
        assert_eq!(BbDofMask::FULL.count_free(), 5);
        assert!(!BbDofMask::ISO_3DOF.z.is_free());
        assert!(!BbDofMask::ISO_3DOF.gy.is_free());
        assert_eq!(BbDofMask::ISO_3DOF.z.value(), 0.0);
    }

    #[test]
    fn prescribed_dof_carries_value() {
        let d = BbDof::Prescribed(0.012);
        assert!(!d.is_free());
        assert!((d.value() - 0.012).abs() < 1e-15);
        assert!(BbDof::Free.is_free());
        assert_eq!(BbDof::Free.value(), 0.0);
    }

    #[test]
    fn preload_model_defaults_to_spring() {
        assert_eq!(BbPreloadModel::default(), BbPreloadModel::Spring);
        assert_eq!(BbSolverParams::default().preload_model, BbPreloadModel::Spring);
    }

    #[test]
    fn radial_magnitude_and_angle_are_consistent() {
        let op = BbOperatingConditions {
            f_x_n: 0.0,
            f_y_n: 3.0,
            f_z_n: 4.0,
            m_y_nmm: 0.0,
            m_z_nmm: 0.0,
            n_inner_rpm: 1000.0,
            n_outer_rpm: 0.0,
            temperature_c: 70.0,
        };
        assert!((op.radial_magnitude() - 5.0).abs() < 1e-12);
        assert!((op.radial_angle() - (4.0_f64).atan2(3.0)).abs() < 1e-12);
        assert!((op.relative_speed_rpm() - 1000.0).abs() < 1e-12);
    }

    #[allow(clippy::field_reassign_with_default)]
    #[test]
    fn solver_params_reject_bad_settings() {
        let mut p = BbSolverParams::default();
        p.convergence_tol = 0.0;
        assert!(p.validate().is_err());

        let mut p = BbSolverParams::default();
        p.max_iterations = 0;
        assert!(p.validate().is_err());

        let mut p = BbSolverParams::default();
        p.phase_sweep = BbPhaseSweep {
            enabled: true,
            n_phase: 0,
        };
        assert!(p.validate().is_err());
    }

    /// D-10: 이 파일에는 단위 환산 상수가 없어야 한다.
    /// 값이 전부 mm·N·rad·MPa 이므로 1000.0 이 등장할 이유가 없다.
    /// (207_000.0 은 MPa 물성값이지 환산 상수가 아니다.)
    #[test]
    fn serde_roundtrip_preserves_input() {
        let input = BbInput {
            kind: BallBearingKind::Acbb,
            geometry: fixture(),
            material: Material::default(),
            operating: BbOperatingConditions {
                f_x_n: 5000.0,
                f_y_n: 2000.0,
                f_z_n: 0.0,
                m_y_nmm: 0.0,
                m_z_nmm: 0.0,
                n_inner_rpm: 1500.0,
                n_outer_rpm: 0.0,
                temperature_c: 70.0,
            },
            solver: BbSolverParams::default(),
        };
        let json = serde_json::to_string(&input).unwrap();
        let back: BbInput = serde_json::from_str(&json).unwrap();
        assert!((back.operating.f_x_n - 5000.0).abs() < 1e-12);
        assert!((back.geometry.d_w_mm - 11.5).abs() < 1e-12);
        assert_eq!(back.solver.dof_mask, BbDofMask::FULL);
        assert!(back.validate().is_ok());
    }

    /// P4-S0-4: 변종 게이트 (§3.6.1.3 ⑤).
    /// 미지원 변종은 `validate()` 가 **이유와 함께** 거부해야 한다.
    #[test]
    fn rejects_unverified_ball_bearing_kinds() {
        let base = |kind| BbInput {
            kind,
            geometry: fixture(),
            material: Material::default(),
            operating: BbOperatingConditions {
                f_x_n: 5000.0,
                f_y_n: 2000.0,
                f_z_n: 0.0,
                m_y_nmm: 0.0,
                m_z_nmm: 0.0,
                n_inner_rpm: 1500.0,
                n_outer_rpm: 0.0,
                temperature_c: 70.0,
            },
            solver: BbSolverParams::default(),
        };
        assert!(base(BallBearingKind::Acbb).validate().is_ok());
        for kind in [BallBearingKind::Dgbb, BallBearingKind::FourPoint] {
            assert!(
                base(kind).validate().is_err(),
                "미검증 변종 {kind:?} 가 통과했다"
            );
        }
        // 기본값은 검증 완료 범위여야 한다 (serde default 포함).
        assert_eq!(BallBearingKind::default(), BallBearingKind::Acbb);
    }

    /// `kind` 는 **선언값이지 추론값이 아니다** — α₀ = 0 이어도 `Acbb` 선언이면
    /// 통과한다 (Level C-7 픽스처가 α₀ = 0 을 쓴다).
    #[test]
    fn acbb_with_zero_initial_angle_is_accepted() {
        let mut g = fixture();
        g.alpha_nom_rad = 0.0;
        g.clearance = BbClearanceSpec::DiametralMm(0.0);
        let input = BbInput {
            kind: BallBearingKind::Acbb,
            geometry: g,
            material: Material::default(),
            operating: BbOperatingConditions {
                f_x_n: 0.0,
                f_y_n: 3000.0,
                f_z_n: 0.0,
                m_y_nmm: 0.0,
                m_z_nmm: 0.0,
                n_inner_rpm: 1500.0,
                n_outer_rpm: 0.0,
                temperature_c: 70.0,
            },
            solver: BbSolverParams::default(),
        };
        assert!(input.validate().is_ok());
    }
}
