use serde::{Deserialize, Deserializer, Serialize};

use crate::error::SolverError;

// ─── Progress Reporting ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverProgress {
    pub stage: String,
    pub detail: String,
    pub percent: f64,        // 0.0 ~ 100.0
}

/// Trait for progress reporting. Implementations can emit Tauri events, log, or no-op.
pub trait ProgressReporter: Send + Sync {
    fn report(&self, progress: SolverProgress);
}

/// No-op reporter for tests and non-UI contexts.
#[cfg(test)]
pub struct NoopReporter;
#[cfg(test)]
impl ProgressReporter for NoopReporter {
    fn report(&self, _progress: SolverProgress) {}
}

// ─── Input Types ────────────────────────────────────────────────────

// CRB (Cylindrical Roller Bearing) — Plan §6 D1~D7 반영:
//   D1: 모든 시리즈에서 rib contact 제외 → h_rib / alpha_rib / h_c 제거
//   D2: 시리즈 분기 없음 (단일 솔버)
//   D3: 단일 row (n_rows = 1 고정, 필드 없음)
//   D4: F_a = 0 강제 (OperatingConditions 에서 처리)
//   D5: 좌표계 X=수평 radial, Y=수직(중력), Z=shaft axis (변경 없음)
//   D6: single-plane misalignment (X축 about, γ_y=0)
//   D7: 평형 DOF = 3 (δx, δy, γx)
// ISO 16281 근거:
//   α = 0 → alpha 제거
//   원통 균일 → D_we_max/D_we_min 통합
//   Clause 4 NOTE 3 (ISO p. 4): L_we 는 roller axis 따라 정의
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroGeometry {
    pub d: f64,           // Bore diameter [mm]
    pub outer_diameter: f64, // Outer diameter [mm]
    pub t: f64,           // Bearing width [mm]
    pub z: u32,           // Number of rollers
    pub d_we: f64,        // Roller diameter (uniform for CRB) [mm]
    pub l_we: f64,        // Roller effective contact length (along roller axis, ISO p.4 NOTE 3) [mm]
    pub d_pw: f64,        // Pitch circle diameter [mm]
    pub g_r: f64,         // Radial internal clearance [μm]
}

impl MacroGeometry {
    pub fn validate(&self) -> Result<(), SolverError> {
        if self.d <= 0.0 {
            return Err(SolverError::InvalidGeometry("Bore diameter must be positive".into()));
        }
        if self.outer_diameter <= self.d {
            return Err(SolverError::InvalidGeometry("Outer diameter must be greater than bore diameter".into()));
        }
        if self.l_we <= 0.0 {
            return Err(SolverError::InvalidGeometry("Effective contact length must be positive".into()));
        }
        if self.z == 0 {
            return Err(SolverError::InvalidGeometry("Number of rollers must be > 0".into()));
        }
        if self.d_we <= 0.0 {
            return Err(SolverError::InvalidGeometry("Roller diameter must be positive".into()));
        }
        Ok(())
    }
}

// CRB raceway — 원통형 (cylindrical bore).  α_i = α_o = 0 → 필드 자체 제거.
// r_rib / r_rib_circ 제거 (D1: rib 없음).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RacewayGeometry {
    pub r_i: f64,         // Inner raceway transverse curvature radius [mm] (일반적으로 ∞, 원통 표준값)
    pub r_o: f64,         // Outer raceway transverse curvature radius [mm]
    pub d_uc: f64,        // Raceway undercut depth [mm]
    pub l_uc: f64,        // Raceway undercut axial extent [mm]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrownType {
    Logarithmic { a_log: f64 },
    Circular { r_crown: f64 },
    Parabolic { c2: f64 },
    Custom { profile: Vec<(f64, f64)> }, // (x_mm, dz_um)
    /// 4th-order polynomial: dz = p1·x⁴ + p2·x³ + p3·x² + p4·x + p5 [μm]
    /// x is centered position [mm], coeffs = [p1, p2, p3, p4, p5]
    Polynomial { coeffs: Vec<f64> },
}

// CRB roller profile — 양 끝 대칭 (Plan 부록 A.1 참조).
// r_sph 제거 (D1: rib 없음 → end sphere 무의미).
// Dub-off: large/small 분리 필드 → 단일 (delta_dub, l_dub) 로 통합, 양쪽 동일 적용.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollerProfile {
    pub crown_type: CrownType,
    pub delta_c: f64,      // Crown drop center-to-end [μm]
    pub delta_dub: f64,    // Dub-off amount (both ends, symmetric) [μm]
    pub l_dub: f64,        // Dub-off length (both ends, symmetric) [mm]
    /// Roller surface roughness Ra [μm]. Default 0.15 for ground steel.
    #[serde(default = "default_sigma_roller")]
    pub sigma_roller: f64,
}

fn default_sigma_roller() -> f64 {
    0.15 // μm, typical roller Ra for ground bearing steel
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RacewayProfile {
    pub delta_rw: f64,     // Raceway crowning [μm]
    pub w_a: f64,          // Axial waviness amplitude [μm]
    pub ra: f64,           // Surface roughness Ra [μm]
    pub custom_profile: Option<Vec<(f64, f64)>>, // (x_mm, dz_um)
    /// 4th-order polynomial: dz = p1·x⁴ + p2·x³ + p3·x² + p4·x + p5 [μm]
    /// x is centered position [mm], coeffs = [p1, p2, p3, p4, p5]
    #[serde(default)]
    pub polynomial_coeffs: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub e_roller: f64,     // Young's modulus roller [GPa]
    pub e_ring: f64,       // Young's modulus rings [GPa]
    pub nu: f64,           // Poisson's ratio
    pub hrc: f64,          // Surface hardness [HRC]
    /// Roller density [g/cm³]. Default 7.85 for bearing steel.
    #[serde(default = "default_density")]
    pub density_roller: f64,
    /// Ring (inner/outer race) density [g/cm³]. Default 7.85 for bearing steel.
    #[serde(default = "default_density")]
    pub density_ring: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            e_roller: 210.0,
            e_ring: 210.0,
            nu: 0.3,
            hrc: 60.0,
            density_roller: 7.85,
            density_ring: 7.85,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LubricationType {
    Oil,
    Grease,
}

impl Default for LubricationType {
    fn default() -> Self {
        LubricationType::Oil
    }
}

/// Lubrication analysis model selection.
///
/// - Method1_DH: Dowson-Higginson (1977) — classic isothermal line-contact EHL
/// - Method2_MK: Masjedi-Khonsari (2015) — roughness-integrated EHL
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LubricationModel {
    /// Method 1: Dowson-Higginson + Greenwood-Tripp + Gupta thermal
    #[serde(alias = "Basic")]
    Method1_DH,
    /// Method 2: Masjedi-Khonsari film + Roelands viscosity + Eyring traction + Murch-Wilson thermal
    #[serde(alias = "Advanced")]
    Method2_MK,
    /// Method 3: Nijenbanning-Venner-Moes (1994) unified 4-regime EHL + M2 traction/mixed
    Method3_NVM,
}

impl Default for LubricationModel {
    fn default() -> Self {
        LubricationModel::Method1_DH
    }
}

/// Surface finish class for micropitting Λ_perm assessment.
/// Based on ISO/TS 6336-22 GF-Class concept adapted for bearings.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SurfaceFinish {
    /// As-ground, Rq ≈ 0.3~0.5 μm. λ_perm_base = 2.0 (GF-Class Low)
    Standard,
    /// Fine ground, Rq ≈ 0.15~0.3 μm. λ_perm_base = 1.0 (GF-Class Medium)
    FineGround,
    /// Superfinish / isotropic, Rq < 0.1 μm. λ_perm_base = 0.5 (GF-Class High)
    Superfinish,
}

impl Default for SurfaceFinish {
    fn default() -> Self { SurfaceFinish::Standard }
}

impl SurfaceFinish {
    /// Base Λ_perm value from ISO/TS 6336-22 GF-Class mapping.
    pub fn lambda_perm_base(&self) -> f64 {
        match self {
            SurfaceFinish::Standard => 2.0,
            SurfaceFinish::FineGround => 1.0,
            SurfaceFinish::Superfinish => 0.5,
        }
    }
}

/// Lubricant additive type for micropitting Λ_perm correction.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AdditiveType {
    /// No specific micropitting-protective additive
    None,
    /// Extreme Pressure additive (sulfur-phosphorus)
    EP,
    /// Anti-Wear additive (ZDDP etc.)
    AW,
}

impl Default for AdditiveType {
    fn default() -> Self { AdditiveType::None }
}

impl AdditiveType {
    /// Λ_perm multiplier. EP/AW additives provide boundary protection.
    pub fn lambda_perm_factor(&self) -> f64 {
        match self {
            AdditiveType::None => 1.0,
            AdditiveType::EP => 0.8,
            AdditiveType::AW => 0.7,
        }
    }
}

/// Non-Newtonian traction model selection.
///
/// - `Eyring`: τ = τ₀·sinh⁻¹(η·γ̇/τ₀). Default. Logarithmic shear-thinning at
///   high shear. Good for raceway-roller (low SRR) contacts.
/// - `CarreauYasuda`: η_eff = η_∞ + (η₀−η_∞)·[1+(λ·γ̇)^a]^((n−1)/a).
///   Plateaus to finite η_∞ at high shear. Recommended for rib-roller-end and
///   gear contacts where SRR > 0.5 (research report §4.3, §5.1).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TractionModel {
    Eyring,
    CarreauYasuda,
}

impl Default for TractionModel {
    fn default() -> Self { TractionModel::Eyring }
}

/// Bearing-level friction model selection (rolling + sliding integration).
///
/// - `PalmgrenLike`: μ_rr·Q·u_roll per contact + Eyring/Carreau sliding traction.
///   Default — preserves prior solver behavior. Simple, no viscosity dependence.
/// - `BibouletHoupert`: per-contact hydrodynamic rolling resistance from the
///   Biboulet-Houpert 2010 analytical model (IVR/EHL with smooth transition).
///   Replaces Palmgren μ_rr·Q·u with F_R(η, u, Q, R_x, R_y, E*); preserves
///   per-roller breakdown (works with split-contact and non-SKF bearings).
/// - `SkfAdvanced`: SKF Catalogue 2018 bearing-level model. M_rr = G_rr·(νn)^0.6
///   captures viscous-hydrodynamic rolling resistance (industry calibration of
///   Houpert/Biboulet theory). Matches SKF Bearing Calculator. Limited to the
///   SKF series with calibrated R/S constants (others use 'Other' fallback).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FrictionModel {
    PalmgrenLike,
    BibouletHoupert,
    SkfAdvanced,
}

impl Default for FrictionModel {
    fn default() -> Self { FrictionModel::PalmgrenLike }
}

/// Thermal inlet-shear correction model for BH 2010 line-contact rolling
/// friction (only applied when `FrictionModel::BibouletHoupert` is active).
///
/// Both factors take the form φ_T = 1 / (1 + a·L^b) with L = η₀·β·u²/k_fluid;
/// they differ in calibration source:
/// - `Aihara1987` (default): a=0.29, b=0.78.  Calibrated specifically for
///   **TRB rolling torque** under axial load (Aihara, *J. Tribol.* 109:471).
///   Validated against Schwarz 2023 measurements within ±10 % across the
///   full operating range; cited in Tewari 2023 Table 1.
/// - `Wilson1979`: a=0.1, b=0.64.  Originally derived for **film thickness**
///   reduction in EHL line contact.  Matches the φ_T already applied to film
///   thickness in the M1/M2 paths.  Conservative; under-corrects rolling
///   torque at high speed (~+20 % over-prediction in Schwarz 4000 rpm test).
/// - `None`: isothermal — no thermal correction.  Use for academic comparison
///   or when post-processing thermal effects separately.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ThermalCorrection {
    Wilson1979,
    Aihara1987,
    None,
}

impl Default for ThermalCorrection {
    fn default() -> Self { ThermalCorrection::Aihara1987 }
}

/// SKF TRB series (R/S geometric constants) — exposed in `OperatingConditions`
/// when `FrictionModel::SkfAdvanced` is selected and for the SKF reference
/// torque card. The wrapper type lets us serialize the catalogue's series
/// classification cleanly.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SkfTrbSeriesEnum {
    Series302,
    Series303,
    Series313,
    Series320,
    Series322,
    Series322B,
    Series323,
    Series323B,
    Other,
}

impl Default for SkfTrbSeriesEnum {
    fn default() -> Self { SkfTrbSeriesEnum::Series303 }
}

/// SKF lubrication scheme (controls K_rs in the kinematic-starvation factor).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SkfLubricationEnum {
    OilBath,
    OilJet,
    Grease,
    OilAir,
}

impl Default for SkfLubricationEnum {
    fn default() -> Self { SkfLubricationEnum::OilBath }
}

/// Axial preload application mode for TRB.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PreloadMode {
    /// Force-controlled preload via displacement (simple).
    /// Convert f_a to δz_preload (pure axial, Fr=0, all rollers uniform).
    /// Fix δz, solve 2-DOF NR for (δx, δy). Axial reaction is a result.
    /// Matches physical preload setting condition (no radial load).
    DisplacementFromForce,
    /// Force-controlled preload via displacement (iterative).
    /// Step 1: Convert f_a to δz_preload → initial guess.
    /// Step 2: 3-DOF NR for (δx, δy, δz) to satisfy full force equilibrium Σ Q_j·sin(α) = F_a.
    DisplacementFromForceIterative,
    /// Direct displacement input.
    /// Fix δz = delta_preload_um, solve 2-DOF NR for (δx, δy).
    /// Axial reaction is a result, not a constraint.
    Displacement,
}

impl Default for PreloadMode {
    fn default() -> Self {
        PreloadMode::DisplacementFromForce
    }
}

fn default_alpha_pv() -> f64 {
    20.0 // [1/GPa], typical mineral oil
}

fn default_starvation_factor() -> f64 {
    1.0 // fully flooded (oil bath); user should set 0.5-0.8 for grease
}

fn default_density() -> f64 { 7.85 }

fn default_rho_oil() -> f64 {
    850.0 // kg/m³, mineral oil typical
}

fn default_tau_eyring() -> f64 {
    5.0 // MPa, typical mineral oil Eyring stress
}

fn default_z_roelands() -> f64 {
    0.67 // Roelands pressure-viscosity exponent, mineral oil
}

fn default_carreau_eta_inf_ratio() -> f64 {
    0.005 // η_∞ / η_0, typical for mineral oil at very high shear
}

fn default_carreau_lambda_s() -> f64 {
    1.0e-7 // relaxation time [s], Bair (2007) range 1e-9 ~ 1e-6
}

fn default_carreau_n() -> f64 {
    0.5 // power-law exponent, mineral oil typical 0.4 ~ 0.7
}

fn default_carreau_a() -> f64 {
    2.0 // Yasuda transition width; a=2 reduces to original Carreau model
}

fn default_skf_y_factor() -> f64 {
    1.6 // 30306 J2/Q catalogue value (representative for 303-series)
}

fn default_hysteresis_loss_factor() -> f64 {
    0.005 // Johnson α_v, conservative value for hardened bearing steel
}

fn default_k_fluid() -> f64 {
    0.15 // W/(m·K), lubricant thermal conductivity
}

fn default_beta_visc() -> f64 {
    0.04 // 1/K, viscosity-temperature coefficient
}

fn default_rq_inner() -> f64 {
    0.3 // μm, inner raceway RMS roughness
}

fn default_rq_outer() -> f64 {
    0.3 // μm, outer raceway RMS roughness
}

fn default_rq_roller() -> f64 {
    0.15 // μm, roller RMS roughness
}

fn default_design_life_hours() -> f64 {
    100.0
}

// CRB Operating Conditions — Plan §6 D4+D6+D7 반영:
//   D4: f_a 제거 (axial 지지 없음, ISO 16281 A.3.1 NOTE 1)
//   D6: m_y 제거 (single-plane misalignment, X축 about 만 사용)
//   → 평형 DOF = 3: (δx, δy, γx)
// preload_mode, delta_preload_um 제거 (axial preload — CRB 무관).
// skf_trb_series 제거 (TRB 전용 SKF 시리즈 — Phase 7 에서 CRB 대응 검토).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatingConditions {
    pub f_x: f64,          // Radial load X-component (horizontal) [kN]
    #[serde(default)]
    pub f_y: f64,          // Radial load Y-component (vertical, gravity) [kN]
    pub m_x: f64,          // Tilting moment about X (single-plane, D6) [kN·m]
    /// Inner ring rotational speed [rpm]. Default = n_rpm for backward compat.
    #[serde(default, alias = "n_rpm")]
    pub n_inner_rpm: f64,
    /// Outer ring rotational speed [rpm]. Default = 0 (stationary).
    #[serde(default)]
    pub n_outer_rpm: f64,
    pub gamma: f64,        // External misalignment angle (about X-axis, D6) [arcmin]
    pub t_op: f64,         // Operating temperature [°C]
    pub nu_40: f64,        // Kinematic viscosity at 40°C [mm²/s]
    pub nu_100: f64,       // Kinematic viscosity at 100°C [mm²/s]
    /// Pressure-viscosity coefficient α [1/GPa]. Default ≈ 20 for mineral oil.
    #[serde(default = "default_alpha_pv")]
    pub alpha_pv: f64,
    /// Lubricant type: "oil" or "grease"
    #[serde(default)]
    pub lubrication_type: LubricationType,
    /// Starvation factor φ_s ∈ (0,1]. 1.0 = fully flooded, 0.5-0.8 = grease typical.
    #[serde(default = "default_starvation_factor")]
    pub starvation_factor: f64,
    /// Lubricant density [kg/m³]. Default 850 for mineral oil.
    #[serde(default = "default_rho_oil")]
    pub rho_oil: f64,
    /// Design life duration for damage/reliability calculation [hours]. Default 100.
    #[serde(default = "default_design_life_hours")]
    pub design_life_hours: f64,

    // === Advanced lubrication model parameters (serde defaults) ===

    /// Lubrication analysis model selection (Method1_DH / Method2_MK)
    #[serde(default)]
    pub lubrication_model: LubricationModel,

    /// Enable Van Zoelen film thickness decay model.
    /// When true, predicts time-dependent film decay due to side flow.
    #[serde(default)]
    pub film_decay_enabled: bool,
    /// Operating time for film decay calculation [hours]. Default 0 (= fully flooded).
    #[serde(default)]
    pub film_decay_time_hours: f64,
    /// Roller skew angle for decay correction [degrees]. Default 0.
    #[serde(default)]
    pub skew_angle_deg: f64,
    /// Replenishment rate R [nm/s]. Default 0 (= no replenishment, worst-case).
    /// Grease: estimate from bleeding rate. Oil jet: from jet flow rate.
    #[serde(default)]
    pub replenishment_rate_nm_s: f64,
    /// Surface finish class for micropitting assessment.
    /// Determines Λ_perm multiplier. Default: Standard.
    #[serde(default)]
    pub surface_finish: SurfaceFinish,
    /// Lubricant additive type for micropitting Λ_perm correction.
    #[serde(default)]
    pub additive_type: AdditiveType,

    /// Eyring stress τ₀ [MPa]. Default 5 MPa (mineral oil).
    #[serde(default = "default_tau_eyring")]
    pub tau_eyring: f64,
    /// Roelands pressure-viscosity exponent Z_r. Default 0.67 (mineral oil).
    #[serde(default = "default_z_roelands")]
    pub z_roelands: f64,

    /// Non-Newtonian traction model. Default: Eyring (backward-compat).
    #[serde(default)]
    pub traction_model: TractionModel,
    /// Bearing-level friction model. Default: PalmgrenLike (backward-compat).
    #[serde(default)]
    pub friction_model: FrictionModel,
    /// Thermal inlet-shear correction model for BH 2010 rolling friction.
    /// Default: Wilson1979 (matches film thickness φ_T treatment).
    /// `Aihara1987` is recommended for TRB axial-load rolling torque.
    #[serde(default)]
    pub thermal_correction: ThermalCorrection,
    /// Johnson (1985) material hysteresis loss factor α_v [dimensionless].
    /// Captures rolling resistance from incomplete elastic recovery of bearing
    /// steel during cyclic compression — INDEPENDENT of lubricant.
    /// Typical range for hardened bearing steel: 0.005 – 0.05.
    /// Default 0.005 (conservative).  Applied only when `friction_model =
    /// BibouletHoupert` (BH is purely viscous; Palmgren/SKF already include
    /// hysteresis empirically via μ_rr / G_rr calibration).
    /// Per Schwarz 2023 Eq. 20 / Johnson Contact Mechanics §9.6.
    #[serde(default = "default_hysteresis_loss_factor")]
    pub hysteresis_loss_factor: f64,
    // skf_trb_series 제거 (TRB 전용) — Phase 7 에서 SKF CRB 시리즈 대응 검토
    /// SKF lubrication scheme (kinematic starvation factor K_rs).
    #[serde(default)]
    pub skf_lubrication: SkfLubricationEnum,
    /// Axial load factor Y for the SKF model (catalogue-supplied; 1.6 typical
    /// for 30306 J2/Q).
    #[serde(default = "default_skf_y_factor")]
    pub skf_y_factor: f64,
    /// Carreau-Yasuda: η_∞ / η_0 ratio. Default 0.005.
    #[serde(default = "default_carreau_eta_inf_ratio")]
    pub carreau_eta_inf_ratio: f64,
    /// Carreau-Yasuda: relaxation time λ [s]. Default 1e-7.
    #[serde(default = "default_carreau_lambda_s")]
    pub carreau_lambda_s: f64,
    /// Carreau-Yasuda: power-law exponent n. Default 0.5.
    #[serde(default = "default_carreau_n")]
    pub carreau_n: f64,
    /// Carreau-Yasuda: Yasuda transition exponent a. Default 2.0 (= original Carreau).
    #[serde(default = "default_carreau_a")]
    pub carreau_a: f64,
    /// Lubricant thermal conductivity k_fluid [W/(m·K)]. Default 0.15.
    #[serde(default = "default_k_fluid")]
    pub k_fluid: f64,
    /// Viscosity-temperature coefficient β_visc [1/K]. Default 0.04.
    #[serde(default = "default_beta_visc")]
    pub beta_visc: f64,
    /// Surface roughness input mode: Ra or Rq.
    /// When Ra: values below are Ra, internally converted to Rq = 1.25 × Ra.
    /// When Rq: values below are Rq (RMS), used directly.
    #[serde(default)]
    pub roughness_input_mode: RoughnessInputMode,
    /// Inner raceway roughness [μm]. Interpretation depends on roughness_input_mode.
    #[serde(default = "default_rq_inner")]
    pub rq_inner: f64,
    /// Outer raceway roughness [μm]. Interpretation depends on roughness_input_mode.
    #[serde(default = "default_rq_outer")]
    pub rq_outer: f64,
    /// Roller surface roughness [μm]. Interpretation depends on roughness_input_mode.
    #[serde(default = "default_rq_roller")]
    pub rq_roller: f64,
}

/// Conversion factor: Rq ≈ 1.25 × Ra for Gaussian surfaces (ISO 4287).
pub const RA_TO_RQ: f64 = 1.25;

impl OperatingConditions {
    /// Resultant radial load [kN]
    pub fn f_r(&self) -> f64 {
        (self.f_x * self.f_x + self.f_y * self.f_y).sqrt()
    }

    /// Misalignment angle in radians
    pub fn gamma_rad(&self) -> f64 {
        self.gamma * std::f64::consts::PI / (180.0 * 60.0)
    }

    /// Backward-compatible: effective shaft speed [rpm] = |n_inner - n_outer|.
    /// Used where a single representative speed is needed (ν₁ ref viscosity, etc.).
    pub fn n_rpm(&self) -> f64 {
        (self.n_inner_rpm - self.n_outer_rpm).abs()
    }

    /// Inner ring angular velocity [rad/s].
    pub fn omega_inner(&self) -> f64 {
        self.n_inner_rpm * std::f64::consts::TAU / 60.0
    }

    /// Outer ring angular velocity [rad/s].
    pub fn omega_outer(&self) -> f64 {
        self.n_outer_rpm * std::f64::consts::TAU / 60.0
    }

    /// Cage orbital speed [rad/s].
    /// ω_c = (ω_i × (1−γ) + ω_o × (1+γ)) / 2
    pub fn omega_cage(&self, gamma_dw: f64) -> f64 {
        let w_i = self.omega_inner();
        let w_o = self.omega_outer();
        (w_i * (1.0 - gamma_dw) + w_o * (1.0 + gamma_dw)) / 2.0
    }

    /// Roller spin speed [rad/s].
    /// ω_r = |ω_i − ω_o| × d_pw / (2·D_we) × (1 − γ²)
    pub fn omega_roller(&self, gamma_dw: f64, d_pw: f64, d_we: f64) -> f64 {
        let delta_w = (self.omega_inner() - self.omega_outer()).abs();
        delta_w * d_pw / (2.0 * d_we) * (1.0 - gamma_dw * gamma_dw)
    }

    /// Mean entraining velocity at inner raceway contact [m/s].
    ///
    /// EHL entraining velocity = sweeping speed of the contact zone over
    /// the raceway surface.  At pure rolling both surfaces move at the
    /// same speed relative to the contact, so u_m = u_sweep.
    ///
    ///   u_m = |(ω_i − ω_cage)| × R_inner
    ///       = |ω_i − ω_cage| × R_pw × (1 − γ)
    ///       = |ω_i − ω_o| × R_pw × (1 − γ²) / 2
    ///
    /// Ref: Harris & Kotzalas (2006) Ch.12, Hamrock et al. (2004) Ch.18.
    pub fn u_m_inner(&self, r_pw_m: f64, gamma_k: f64) -> f64 {
        let w_i = self.omega_inner();
        let w_c = self.omega_cage(gamma_k);
        let r_i = r_pw_m * (1.0 - gamma_k);
        ((w_i - w_c) * r_i).abs()
    }

    /// Mean entraining velocity at outer raceway contact [m/s].
    ///
    /// EHL entraining velocity = sweeping speed of the contact zone over
    /// the outer raceway surface.
    ///
    ///   u_m = |(ω_cage − ω_o)| × R_outer
    ///       = |ω_cage − ω_o| × R_pw × (1 + γ)
    ///       = |ω_i − ω_o| × R_pw × (1 − γ²) / 2
    ///
    /// At pure rolling: u_m_inner = u_m_outer (same at each slice).
    /// Ref: Harris & Kotzalas (2006) Ch.12, Hamrock et al. (2004) Ch.18.
    pub fn u_m_outer(&self, r_pw_m: f64, gamma_k: f64) -> f64 {
        let w_o = self.omega_outer();
        let w_c = self.omega_cage(gamma_k);
        let r_o = r_pw_m * (1.0 + gamma_k);
        ((w_c - w_o) * r_o).abs()
    }

    /// Effective Rq for roller [μm] — converts from Ra if needed.
    pub fn rq_roller_eff(&self) -> f64 {
        match self.roughness_input_mode {
            RoughnessInputMode::Ra => self.rq_roller * RA_TO_RQ,
            RoughnessInputMode::Rq => self.rq_roller,
        }
    }

    /// Effective Rq for inner raceway [μm] — converts from Ra if needed.
    pub fn rq_inner_eff(&self) -> f64 {
        match self.roughness_input_mode {
            RoughnessInputMode::Ra => self.rq_inner * RA_TO_RQ,
            RoughnessInputMode::Rq => self.rq_inner,
        }
    }

    /// Effective Rq for outer raceway [μm] — converts from Ra if needed.
    pub fn rq_outer_eff(&self) -> f64 {
        match self.roughness_input_mode {
            RoughnessInputMode::Ra => self.rq_outer * RA_TO_RQ,
            RoughnessInputMode::Rq => self.rq_outer,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SolverMode {
    Gen1,
    Gen3,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RunMode {
    Single(SolverMode),
    Dual,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BeamType {
    EulerBernoulli,
    Timoshenko,
}

/// Viscosity ratio κ calculation method.
///
/// Method 1 (default): κ = ν / ν₁ (ISO 281:2007 Eq.27-29)
///   - Standard bearing steel, mineral oil, good manufacturing quality assumed.
///
/// Method 2: κ ≈ Λ^1.3 (ISO 281:2007 §9.3.3.3.2, ISO/TR 1281-2 §11)
///   - Uses EHL film thickness ratio Λ = h_min / σ_composite.
///   - More accurate for: special surface finish, synthetic oils,
///     non-standard pressure-viscosity coefficients.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum KappaMethod {
    /// ISO 281 Eq.(27): κ = ν_actual / ν_reference
    ViscosityRatio,
    /// ISO/TR 1281-2 Eq.(57): κ ≈ Λ^1.3 (film thickness ratio)
    FilmThicknessRatio,
}

impl Default for KappaMethod {
    fn default() -> Self {
        KappaMethod::ViscosityRatio
    }
}

/// Surface roughness input mode.
///
/// Ra (arithmetic mean) and Rq (root mean square) are related by
/// Rq ≈ 1.25 × Ra for Gaussian surfaces (ISO 4287).
/// Internally all calculations use Rq (per ISO/TR 1281-2).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RoughnessInputMode {
    /// Input as Ra — converted to Rq via Rq = 1.25 × Ra
    Ra,
    /// Input as Rq (RMS) — used directly
    Rq,
}

impl Default for RoughnessInputMode {
    fn default() -> Self {
        RoughnessInputMode::Rq
    }
}

/// Fatigue life calculation method.
/// Only ISO 16281 lamina-level method is used; the former "Stirling" variant
/// was identical in lamina capacity and has been merged.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LifeMethod {
    /// ISO 16281:2025 lamina-level life calculation
    Iso16281,
}

impl Default for LifeMethod {
    fn default() -> Self {
        LifeMethod::Iso16281
    }
}

/// Backward-compatible deserializer: accepts "Stirling" as alias for Iso16281.
fn deserialize_life_method<'de, D>(deserializer: D) -> Result<LifeMethod, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Iso16281" | "Stirling" => Ok(LifeMethod::Iso16281),
        _ => Ok(LifeMethod::Iso16281), // fallback
    }
}

/// Rib contact treatment in bearing equilibrium
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RibContactMode {
    /// Rib contact computed as post-processing only (default, legacy)
    PostProcess,
    /// Rib Hertz stiffness included in 5-DOF equilibrium residual
    Coupled,
}

impl Default for RibContactMode {
    fn default() -> Self {
        RibContactMode::PostProcess
    }
}

/// ISO 281:2007 Annex A contamination level.
/// Determines C₁, C₂ constants for e_C calculation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ContaminationLevel {
    /// ISO 4406: —/13/10 (on-line filter β6(c)≥200, β10(c)≥200)
    HighCleanliness,
    /// ISO 4406: —/15/12 (on-line filter β6(c)≥200, β25(c)≥75)
    NormalCleanliness,
    /// ISO 4406: —/17/14 (on-line filter β25(c)≥75)
    SlightContamination,
    /// ISO 4406: —/19/16 (no filter or coarse filter)
    SevereContamination,
    /// ISO 4406: —/21/18 (very heavy contamination)
    VeryHeavyContamination,
}

impl Default for ContaminationLevel {
    fn default() -> Self {
        ContaminationLevel::NormalCleanliness
    }
}

impl ContaminationLevel {
    /// Returns (C₁, C₂) constants for ISO 281 Annex A Eq.(28).
    /// Values for roller bearings, oil bath lubrication.
    pub fn c1_c2_oil_bath(&self) -> (f64, f64) {
        match self {
            ContaminationLevel::HighCleanliness       => (0.1710, 0.3796),
            ContaminationLevel::NormalCleanliness      => (0.0864, 0.6796),
            ContaminationLevel::SlightContamination    => (0.0411, 1.1410),
            ContaminationLevel::SevereContamination    => (0.0178, 1.8570),
            ContaminationLevel::VeryHeavyContamination => (0.0085, 2.6620),
        }
    }

    /// Returns (C₁, C₂) for circulating oil with on-line filter.
    pub fn c1_c2_circulating(&self) -> (f64, f64) {
        match self {
            ContaminationLevel::HighCleanliness       => (0.2288, 0.2700),
            ContaminationLevel::NormalCleanliness      => (0.1148, 0.4920),
            ContaminationLevel::SlightContamination    => (0.0617, 0.8310),
            ContaminationLevel::SevereContamination    => (0.0297, 1.3560),
            ContaminationLevel::VeryHeavyContamination => (0.0133, 1.9970),
        }
    }

    /// Returns (C₁, C₂) for grease lubrication.
    pub fn c1_c2_grease(&self) -> (f64, f64) {
        match self {
            ContaminationLevel::HighCleanliness       => (0.1500, 0.4500),
            ContaminationLevel::NormalCleanliness      => (0.0750, 0.7500),
            ContaminationLevel::SlightContamination    => (0.0380, 1.2500),
            ContaminationLevel::SevereContamination    => (0.0165, 2.0000),
            ContaminationLevel::VeryHeavyContamination => (0.0075, 2.8500),
        }
    }
}

/// Oil supply method — determines which C₁,C₂ table to use for e_C.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum OilSupplyMethod {
    OilBath,
    CirculatingWithFilter,
    Grease,
}

impl Default for OilSupplyMethod {
    fn default() -> Self {
        OilSupplyMethod::OilBath
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverParams {
    pub run_mode: RunMode,
    pub n_slices: usize,
    pub beam_type: BeamType,
    pub convergence_tol: f64,
    pub max_iterations: usize,
    pub angular_increment_deg: f64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_life_method")]
    pub life_method: LifeMethod,
    #[serde(default = "default_e_c")]
    pub e_c: f64,                   // contamination factor override (0=auto, >0=manual)
    /// Contamination level (ISO 4406 cleanliness class) — used when e_c=0 (auto)
    #[serde(default)]
    pub contamination_level: ContaminationLevel,
    /// Oil supply method — determines C₁,C₂ table for e_C calculation
    #[serde(default)]
    pub oil_supply_method: OilSupplyMethod,
    #[serde(default)]
    pub c_r_kn: Option<f64>,        // override basic dynamic load rating [kN]
    /// Basic static load rating C₀ᵣ [kN] — None = auto-calculate from geometry
    #[serde(default)]
    pub c_0r_kn: Option<f64>,
    /// Minimum required static safety factor S₀ = C₀ᵣ / P₀
    #[serde(default = "default_f_s_min")]
    pub f_s_min: f64,
    /// Rib contact mode: PostProcess (default) or Coupled (stiffness in equilibrium)
    #[serde(default)]
    pub rib_contact_mode: RibContactMode,
    /// ISO 15312 coefficient f₀ᵣ for load-independent friction moment.
    /// TRB dimension series 02/03/29/30/20: 3.0 (default); 22/23/13/31/32: 4.5
    #[serde(default = "default_f_0r")]
    pub f_0r: f64,
    /// ISO 15312 coefficient f₁ᵣ for load-dependent friction moment.
    /// All TRB dimension series: 0.0004 (default)
    #[serde(default = "default_f_1r")]
    pub f_1r: f64,
    /// Viscosity ratio κ calculation method.
    /// ViscosityRatio (default): κ = ν/ν₁ (ISO 281 Eq.27)
    /// FilmThicknessRatio: κ ≈ Λ^1.3 (ISO/TR 1281-2 Eq.57)
    #[serde(default)]
    pub kappa_method: KappaMethod,
    /// Use split contact model (independent inner/outer q_k per slice).
    /// Default: true (split is the standard model).
    #[serde(default = "default_split_contact")]
    pub use_split_contact: bool,
}

fn default_split_contact() -> bool { true }

fn default_e_c() -> f64 {
    0.0 // 0 = auto (ISO 281 Annex A), >0 = manual override
}

fn default_f_s_min() -> f64 {
    1.0
}

fn default_f_0r() -> f64 {
    3.0
}

fn default_f_1r() -> f64 {
    0.0004
}

impl Default for SolverParams {
    fn default() -> Self {
        Self {
            run_mode: RunMode::Single(SolverMode::Gen3),
            n_slices: 50,
            beam_type: BeamType::Timoshenko,
            convergence_tol: 1e-6,
            max_iterations: 100,
            angular_increment_deg: 2.0,
            life_method: LifeMethod::Iso16281,
            e_c: 0.0,
            contamination_level: ContaminationLevel::NormalCleanliness,
            oil_supply_method: OilSupplyMethod::OilBath,
            c_r_kn: None,
            c_0r_kn: None,
            f_s_min: 1.0,
            rib_contact_mode: RibContactMode::PostProcess,
            f_0r: 3.0,
            f_1r: 0.0004,
            kappa_method: KappaMethod::ViscosityRatio,
            use_split_contact: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearingInput {
    pub macro_geom: MacroGeometry,
    pub raceway_geom: RacewayGeometry,
    pub roller_profile: RollerProfile,
    pub raceway_profile_inner: RacewayProfile,
    pub raceway_profile_outer: RacewayProfile,
    pub material: Material,
    pub operating: OperatingConditions,
    pub solver: SolverParams,
    /// Transient analysis input (None = steady-state only)
    #[serde(default)]
    pub transient: Option<TransientInput>,
}

impl BearingInput {
    pub fn validate(&self) -> Result<(), SolverError> {
        self.macro_geom.validate()?;
        if self.material.e_roller <= 0.0 || self.material.e_ring <= 0.0 {
            return Err(SolverError::InvalidInput("Young's modulus must be positive".into()));
        }
        if self.material.nu <= 0.0 || self.material.nu >= 0.5 {
            return Err(SolverError::InvalidInput("Poisson's ratio must be in (0, 0.5)".into()));
        }
        if self.solver.n_slices == 0 {
            return Err(SolverError::InvalidInput("Number of slices must be > 0".into()));
        }
        if self.solver.convergence_tol <= 0.0 {
            return Err(SolverError::InvalidInput("Convergence tolerance must be positive".into()));
        }
        Ok(())
    }
}

// ─── Intermediate / Output Types ────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliceGeometry {
    pub k: usize,              // slice index (0-based)
    pub x_axial: f64,          // axial position from small end [mm]
    pub r_roller: f64,         // roller radius at this slice [mm]
    pub r_inner_race: f64,     // inner raceway curvature radius [mm]
    pub r_outer_race: f64,     // outer raceway curvature radius [mm]
    pub r_eq_inner: f64,       // equivalent radius (roller-inner) [mm]
    pub r_eq_outer: f64,       // equivalent radius (roller-outer) [mm]
    pub delta_z_total_inner: f64, // profile correction at inner contact [μm] = Δz_roller + Δz_inner
    pub delta_z_total_outer: f64, // profile correction at outer contact [μm] = Δz_roller + Δz_outer
    pub slice_width: f64,      // l_k = L_we / n [mm]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliceContactResult {
    pub k: usize,
    pub delta_k: f64,          // approach amount [μm]
    pub q_k: f64,              // load per unit length [N/mm] (same for inner/outer)
    pub q_k_outer: f64,        // outer raceway line load [N/mm] (independent)
    pub q_k_inner: f64,        // inner raceway line load [N/mm] (independent)
    // Inner raceway contact
    pub b_k: f64,              // inner contact half-width [mm]
    pub p_max_k: f64,          // inner max Hertz contact stress [MPa]
    pub h_bulk_k: f64,         // inner Weber bulk deformation [μm] — diagnostic only
    pub k_hertz_k: f64,        // inner Hertz contact stiffness [N/mm/μm]
    // Outer raceway contact
    pub b_k_outer: f64,        // outer contact half-width [mm]
    pub p_max_k_outer: f64,    // outer max Hertz contact stress [MPa]
    pub h_bulk_k_outer: f64,   // outer Weber bulk deformation [μm] — diagnostic only
    pub k_hertz_k_outer: f64,  // outer Hertz contact stiffness [N/mm/μm]
    // Combined slice stiffness: 2-spring Hertz series along outer normal.
    //   1/k = 1/k_hertz_k_outer + cos²(α_o−α_i)/k_hertz_k
    // Weber bulk is already embedded in Hertz mutual approach — not a
    // separate spring.
    pub k_combined_k: f64,     // combined slice stiffness [N/mm/μm]
    pub in_contact: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RibContactResult {
    pub f_rib: f64,            // rib contact force [N]
    pub a_ellipse: f64,        // contact ellipse semi-axis a [mm]
    pub b_ellipse: f64,        // contact ellipse semi-axis b [mm]
    pub p_max_rib: f64,        // max contact stress [MPa]
    pub spin_moment: f64,      // spin moment [N·mm]
    pub delta_rib: f64,        // Hertz approach [μm]
    pub k_rib: f64,            // tangent stiffness dF/dδ [N/μm]
    pub r_contact_mm: f64,     // contact point radial position from bearing axis [mm]
    pub r_rib_circ_mm: f64,    // circumferential curvature radius used [mm]
    pub h_c_mm: f64,           // contact height on rib face above rib base [mm] (Liu 2023)
    /// EHL/TEHL evaluation at the rib contact. None when operating conditions
    /// are not provided (e.g. dry analysis) or speed is zero.
    #[serde(default)]
    pub ehl: Option<RibEhlResult>,
}

/// Elastohydrodynamic state of the rib (flange-roller end) contact.
///
/// Computed via Hamrock-Dowson elliptical regression + Murch-Wilson thermal
/// correction + Greenwood-Tripp asperity sharing + dispatched non-Newtonian
/// traction (Eyring or Carreau-Yasuda). Research report §4.3 reference path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RibEhlResult {
    pub h_c_um: f64,                 // central film thickness [μm]
    pub h_min_um: f64,                // minimum film thickness [μm]
    pub sigma_composite_um: f64,      // composite roughness σ_c = √(σ_roller² + σ_rib²) [μm]
    pub lambda_ratio: f64,            // Λ = h_min / σ_c
    pub regime: LubricationRegime,    // Boundary | Mixed | FullEhl
    pub mu_eff: f64,                  // effective traction (mixed)
    pub mu_ehl: f64,                  // fluid-only traction
    pub asperity_load_ratio: f64,     // f_a = F_{5/2}(Λ) / F_{5/2}(0)
    pub p_asperity_mpa: f64,          // asperity-borne pressure
    pub flash_temp_c: f64,            // Blok-Jaeger flash ΔT
    pub srr: f64,                     // slide-roll ratio
    pub u_entrain_m_s: f64,           // mean entrainment velocity
    pub u_slide_m_s: f64,             // sliding velocity
    pub thermal_factor: f64,          // φ_T (Murch-Wilson)
    pub u_param: f64,                 // dimensionless U
    pub g_param: f64,                 // dimensionless G
    pub w_param: f64,                 // dimensionless W
    pub k_ellipse: f64,               // ellipticity a/b
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AngularLoadPoint {
    pub psi_deg: f64,           // angular position [deg]
    pub delta_rigid: f64,       // rigid-body approach [μm]
    pub q_total: f64,           // total roller load at this position [N]
    pub p_max: f64,             // max contact stress [MPa]
    pub slice_p_max: Vec<f64>,        // per-slice inner raceway max stress [MPa]
    pub slice_p_max_outer: Vec<f64>,  // per-slice outer raceway max stress [MPa]
    pub slice_q_k: Vec<f64>,          // per-slice line load [N/mm]
    pub is_roller: bool,              // true if an actual roller sits at this position
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearingEquilibrium {
    pub displacement: [f64; 5], // [δx, δy, δz, γx, γy]
    pub roller_loads: Vec<f64>, // Q_j per roller [N]
    pub roller_results: Vec<RollerResult>,
    pub angular_distribution: Vec<AngularLoadPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollerResult {
    pub psi_deg: f64,
    pub q_normal: f64,        // outer raceway normal load [N]
    pub q_normal_inner: f64,  // inner raceway normal load [N] = q_normal · cos(α_o−α_i)
    pub slice_results: Vec<SliceContactResult>,
    pub rib_result: Option<RibContactResult>,
}

/// Intermediate variables and coefficients for fatigue life calculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeIntermediates {
    // Viscosity parameters
    pub nu_actual: f64,        // kinematic viscosity at T_op [mm²/s]
    pub nu_ref: f64,           // reference viscosity ν₁ [mm²/s]
    // ISO 281 factors
    pub b_m: f64,              // material/manufacturing factor
    pub f_c: f64,              // ISO 281 geometry factor
    pub gamma_bearing: f64,    // D_we·cos(α)/d_pw
    pub c_u_kn: f64,           // fatigue limit load C_u [kN]
    pub c_u_over_p: f64,       // C_u/P ratio
    // Load factors
    pub e_demarcation: f64,    // demarcation factor e = 1.5·tan(α)
    pub x_factor: f64,         // radial load factor X
    pub y_factor: f64,         // axial load factor Y
    pub f_a_over_f_r: f64,     // F_a/F_r ratio
    // Raceway capacity factors
    pub f_ci: f64,             // inner raceway capacity factor
    pub f_co: f64,             // outer raceway capacity factor
    pub q_c_base: f64,         // base element dynamic capacity [N]
    pub q_ci: f64,             // inner raceway element capacity [N]
    pub q_co: f64,             // outer raceway element capacity [N]
    // Equivalent element loads (ISO 16281)
    pub q_ei: f64,             // inner ring Weibull equiv load [N]
    pub q_eo: f64,             // outer ring max element load [N]
    // Life combination
    pub weibull_e: f64,        // Weibull exponent (9/8)
    pub l_nm_mrev: f64,        // modified reference life [10⁶ rev]
    // Lamina capacity
    pub q_c_lamina_inner: f64, // per-lamina inner capacity [N/mm]
    pub q_c_lamina_outer: f64, // per-lamina outer capacity [N/mm]
    pub e_c_used: f64,         // actual contamination factor used (auto or manual)
    pub kappa_method: KappaMethod, // method used for κ calculation
    pub lambda_inner: Option<f64>,  // Λ value for inner raceway (Some when FilmThicknessRatio)
    pub lambda_outer: Option<f64>,  // Λ value for outer raceway (Some when FilmThicknessRatio)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FatigueLifeResult {
    pub method: LifeMethod,
    pub l_10_basic: f64,       // basic rating life [10⁶ rev]
    pub l_nm_hours: f64,       // modified reference life [hours]
    pub l_10_inner: f64,       // inner ring life [10⁶ rev]
    pub l_10_outer: f64,       // outer ring life [10⁶ rev]
    pub weakest_lamina: usize,
    pub a_iso: f64,            // life modification factor
    pub kappa: f64,            // viscosity ratio (representative, = kappa_inner for backward compat)
    pub kappa_inner: f64,      // viscosity ratio for inner raceway
    pub kappa_outer: f64,      // viscosity ratio for outer raceway
    pub c_dyn: f64,            // basic dynamic load rating used [kN]
    pub p_equiv: f64,          // equivalent dynamic bearing load (ISO 281 X,Y) [kN]
    pub p_ref: f64,            // back-calculated: C_r / L_10r(ISO16281)^(3/10) [kN]
    pub p_ref_damage: f64,     // damage-weighted average of lamina P_sk [kN]
    pub intermediates: LifeIntermediates,
    pub lamina_lives: Option<Vec<LaminaLife>>, // per-lamina detail
    /// EHL film thickness analysis (None if insufficient data)
    #[serde(default)]
    pub film_thickness: Option<FilmThicknessResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaminaLife {
    pub k: usize,                  // lamina index
    pub q_equiv_inner: f64,        // inner ring equiv load [N/mm]
    pub q_equiv_outer: f64,        // outer ring equiv load [N/mm]
    pub l_10_inner: f64,           // inner lamina life [10⁶ rev]
    pub l_10_outer: f64,           // outer lamina life [10⁶ rev]
    pub p_sk: f64,                 // ISO 16281 Eq.(31) lamina reference load [kN]
    pub a_iso_k_inner: f64,        // per-lamina life modification factor (inner raceway)
    pub a_iso_k_outer: f64,        // per-lamina life modification factor (outer raceway)
}

/// Lubrication regime classification based on Lambda ratio.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LubricationRegime {
    /// Λ > 3: full EHL film separation
    FullEhl,
    /// 1 ≤ Λ ≤ 3: partial asperity contact
    Mixed,
    /// Λ < 1: predominantly metal-to-metal contact
    Boundary,
}

impl Default for LubricationRegime {
    fn default() -> Self { LubricationRegime::Boundary }
}

/// Per-slice EHL film thickness (computed with slice-local R_eq, u_m, q_k).
/// Inner and outer raceway contacts are computed separately because R_eq differs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SliceFilmThickness {
    /// Inner raceway minimum film thickness [μm]
    pub h_min_um: f64,
    /// Inner raceway central film thickness [μm]
    pub h_central_um: f64,
    /// Inner raceway Lambda ratio
    pub lambda: f64,
    /// Inner raceway lubrication regime
    pub regime: LubricationRegime,
    /// Outer raceway minimum film thickness [μm]
    #[serde(default)]
    pub h_min_um_outer: f64,
    /// Outer raceway central film thickness [μm]
    #[serde(default)]
    pub h_central_um_outer: f64,
    /// Outer raceway Lambda ratio
    #[serde(default)]
    pub lambda_outer: f64,
    /// Outer raceway lubrication regime
    #[serde(default)]
    pub regime_outer: LubricationRegime,
}

/// Per-roller film distribution (vec of per-slice results).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollerFilmDistribution {
    pub roller_idx: usize,
    pub psi_deg: f64,
    pub slices: Vec<SliceFilmThickness>,
}

/// EHL film thickness result per bearing (representative values at max-loaded roller).
/// Inner and outer raceway results are separated because R_eq and σ differ.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilmThicknessResult {
    // ─── Inner raceway ───
    /// Inner raceway minimum EHL film thickness [μm]
    pub h_min_um: f64,
    /// Inner raceway central EHL film thickness [μm]
    pub h_central_um: f64,
    /// Inner composite roughness σ_inner = √(Rq_roller² + Rq_inner²) [μm]
    pub sigma_composite_um: f64,
    /// Inner Lambda ratio Λ = h_min / σ_inner
    pub lambda_ratio: f64,
    /// Inner raceway lubrication regime
    pub regime: LubricationRegime,

    // ─── Outer raceway ───
    /// Outer raceway minimum EHL film thickness [μm]
    #[serde(default)]
    pub h_min_um_outer: f64,
    /// Outer raceway central EHL film thickness [μm]
    #[serde(default)]
    pub h_central_um_outer: f64,
    /// Outer composite roughness σ_outer = √(Rq_roller² + Rq_outer²) [μm]
    #[serde(default)]
    pub sigma_composite_um_outer: f64,
    /// Outer Lambda ratio Λ = h_min_outer / σ_outer
    #[serde(default)]
    pub lambda_ratio_outer: f64,
    /// Outer raceway lubrication regime
    #[serde(default)]
    pub regime_outer: LubricationRegime,

    // ─── Individual roughness values ───
    /// Roller surface roughness [μm] (Ra for Basic, Rq for Advanced)
    #[serde(default)]
    pub rq_roller_um: f64,
    /// Inner raceway surface roughness [μm]
    #[serde(default)]
    pub rq_inner_um: f64,
    /// Outer raceway surface roughness [μm]
    #[serde(default)]
    pub rq_outer_um: f64,

    // ─── Kinematics ───
    /// Entraining velocity at inner raceway contact [m/s]
    pub u_mean_m_s: f64,
    /// Entraining velocity at outer raceway contact [m/s]
    #[serde(default)]
    pub u_mean_m_s_outer: f64,
    /// Cage orbital speed [rpm]
    #[serde(default)]
    pub cage_speed_rpm: f64,
    /// Roller spin speed [rpm]
    #[serde(default)]
    pub roller_spin_rpm: f64,

    // ─── Common parameters ───
    /// Starvation factor applied φ_s
    pub starvation_factor: f64,
    /// Thermal correction factor φ_T (Gupta)
    pub thermal_factor: f64,
    /// Dimensionless parameters for diagnostics (inner raceway)
    pub u_param: f64, // speed parameter U
    pub g_param: f64, // material parameter G
    pub w_param: f64, // load parameter W
    /// Mixed lubrication analysis (Greenwood-Tripp asperity model, inner raceway)
    pub mixed: MixedLubricationResult,
    /// Flash temperature at worst asperity contact [°C] (Method2_MK only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash_temp_c: Option<f64>,

    // ─── Van Zoelen Film Decay results ───
    /// Film decay analysis results (present when film_decay_enabled = true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub film_decay: Option<FilmDecayResult>,

    // ─── Micropitting Safety ───
    /// Micropitting safety factor (present when surface_finish is set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub micropitting: Option<MicropittingSafety>,
}

/// Micropitting safety assessment based on ISO/TS 6336-22 framework.
///
/// Adapted from gear standard to bearing application.
/// S_λ = Λ_min / Λ_perm — safety factor against surface-initiated fatigue.
///
/// **Caveat**: ISO/TS 6336-22 is a gear standard. No equivalent bearing standard exists.
/// Values should be treated as engineering estimates, not certified ratings.
///
/// References:
///   - ISO/TS 6336-22: Micropitting load capacity for gears
///   - FVA 54/7: FZG micropitting test (GF-Class)
///   - Morales-Espejel & Brizmer (2011), Tribology Transactions 54:625-643
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicropittingSafety {
    /// Effective Λ_perm = base × additive_factor
    pub lambda_perm: f64,
    /// Λ_perm base value from surface finish class
    pub lambda_perm_base: f64,
    /// Additive correction factor
    pub additive_factor: f64,
    /// Safety factor inner: S_λ = Λ_min_inner / Λ_perm
    pub s_lambda_inner: f64,
    /// Safety factor outer: S_λ = Λ_min_outer / Λ_perm
    pub s_lambda_outer: f64,
    /// Risk level inner
    pub risk_inner: MicropittingRisk,
    /// Risk level outer
    pub risk_outer: MicropittingRisk,
}

/// Micropitting risk classification.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MicropittingRisk {
    /// S_λ ≥ 2.0 — Full EHL, no micropitting expected
    Safe,
    /// 1.0 ≤ S_λ < 2.0 — Marginal, consider surface treatment or additive
    Marginal,
    /// S_λ < 1.0 — High risk of micropitting
    AtRisk,
}

/// Van Zoelen film thickness decay result.
///
/// Predicts how the central film thickness decreases over operating time
/// due to pressure-driven side flow in starved EHL contacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilmDecayResult {
    /// Operating time [hours]
    pub t_hours: f64,
    /// Decayed central film thickness, inner raceway [μm]
    pub h_c_decayed_inner_um: f64,
    /// Decayed central film thickness, outer raceway [μm]
    pub h_c_decayed_outer_um: f64,
    /// Starvation ratio inner: h_c(t) / h_cff
    pub starvation_ratio_inner: f64,
    /// Starvation ratio outer: h_c(t) / h_cff
    pub starvation_ratio_outer: f64,
    /// Side flow parameter F(0) inner [m⁻² s⁻¹]
    pub f0_inner: f64,
    /// Side flow parameter F(0) outer [m⁻² s⁻¹]
    pub f0_outer: f64,
    /// Lambda ratio at time t, inner
    pub lambda_decayed_inner: f64,
    /// Lambda ratio at time t, outer
    pub lambda_decayed_outer: f64,
    /// Lubrication regime at time t, inner
    pub regime_decayed_inner: LubricationRegime,
    /// Lubrication regime at time t, outer
    pub regime_decayed_outer: LubricationRegime,
    /// Replenishment rate used [nm/s]
    pub replenishment_rate_nm_s: f64,
    /// Equilibrium film thickness inner [μm] (when R > 0, loss = replenishment)
    pub h_c_equilibrium_inner_um: Option<f64>,
    /// Equilibrium film thickness outer [μm]
    pub h_c_equilibrium_outer_um: Option<f64>,
    /// Decay curve data: (time_hours, h_c_inner_nm, h_c_outer_nm)
    pub decay_curve: Vec<(f64, f64, f64)>,
}

/// Greenwood-Tripp mixed lubrication model results.
///
/// Decomposes the total contact load into fluid-borne and asperity-borne
/// fractions using statistical rough-surface contact mechanics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixedLubricationResult {
    /// Asperity load ratio: F_asperity / F_total ∈ [0,1]
    /// 0 = full EHL (no metal contact), 1 = full boundary contact
    pub asperity_load_ratio: f64,
    /// Asperity area ratio: A_real / A_hertz ∈ [0,1]
    pub asperity_area_ratio: f64,
    /// Mean asperity contact pressure [MPa] (on real contact area)
    pub p_asperity_mpa: f64,
    /// Mean fluid film pressure [MPa]
    pub p_fluid_mpa: f64,
    /// EHL traction coefficient (fluid shear) — pure rolling ≈ 0.001–0.01
    pub mu_ehl: f64,
    /// Boundary friction coefficient (metal contact) — typically 0.08–0.15
    pub mu_boundary: f64,
    /// Effective (weighted) friction coefficient μ_eff = (1−γ)μ_ehl + γ·μ_boundary
    pub mu_effective: f64,
    /// Greenwood-Tripp F_{5/2}(Λ) statistical integral value
    pub f_5_2: f64,
    /// Greenwood-Tripp F_2(Λ) statistical integral value (for area)
    pub f_2: f64,
}

// ─── Contact Traction (Power Loss Readiness) ──────────────────────

/// Per-roller contact traction result — designed for power loss integration.
///
/// Future `power_loss.rs` module can sum these:
///   P_total = Σ_j (inner.power_loss_w + outer.power_loss_w + rib.power_loss_w) + P_drag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollerTractionResult {
    pub roller_idx: usize,
    pub psi_deg: f64,
    /// Inner raceway contact friction
    pub inner: ContactFriction,
    /// Outer raceway contact friction
    pub outer: ContactFriction,
    /// Rib contact friction (if present)
    pub rib: Option<RibFriction>,
}

/// Friction/traction at a single line contact (roller–raceway).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactFriction {
    /// Rolling (entraining) velocity [m/s]
    pub u_rolling: f64,
    /// Sliding velocity [m/s]
    pub u_sliding: f64,
    /// Slide-roll ratio SRR = u_sliding / u_rolling
    pub srr: f64,
    /// Lambda ratio at this contact
    pub lambda: f64,
    /// Asperity load fraction [0,1]
    pub asperity_load_ratio: f64,
    /// Effective friction coefficient (sliding traction)
    pub mu: f64,
    /// Traction force [N] = μ × Q_normal
    pub f_traction_n: f64,
    /// Sliding power dissipation [W] = F_traction × |u_sliding|
    pub power_loss_w: f64,
    /// Rolling resistance power dissipation [W] = μ_rolling × Q_normal × u_rolling
    /// (Palmgren μ_rolling = 0.002 for bearing steel line contact.)
    #[serde(default)]
    pub p_rolling_w: f64,
    /// Johnson (1985) material hysteresis power loss [W] — independent of
    /// lubricant, depends only on Q and Hertz half-width b.
    /// Active only when `friction_model = BibouletHoupert` (BH is purely
    /// viscous, so hysteresis must be added; Palmgren/SKF μ_rr already
    /// include hysteresis empirically).
    #[serde(default)]
    pub p_hysteresis_w: f64,
}

/// Friction at rib contact (point contact, pure sliding).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RibFriction {
    /// Sliding velocity at rib [m/s]
    pub u_sliding: f64,
    /// Friction coefficient (boundary, ~0.04–0.08 with EP additive)
    pub mu: f64,
    /// Friction force [N]
    pub f_friction_n: f64,
    /// Power dissipation [W]
    pub power_loss_w: f64,
}

/// Bearing-level traction summary — the interface for power loss model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TractionSummary {
    /// Per-roller traction detail
    pub rollers: Vec<RollerTractionResult>,
    /// Total rolling friction power loss [W] (BH 2010 viscous EHL only)
    pub p_rolling_w: f64,
    /// Total sliding friction power loss (raceway) [W]
    pub p_sliding_w: f64,
    /// Total rib friction power loss [W]
    pub p_rib_w: f64,
    /// Total Johnson (1985) material hysteresis power loss [W].
    /// Bearing-level sum of per-contact `ContactFriction.p_hysteresis_w`.
    /// 0 unless `friction_model = BibouletHoupert`.
    #[serde(default)]
    pub p_hysteresis_w: f64,
    /// Total contact power loss (rolling + sliding + rib) [W]
    /// Note: drag/seal losses are NOT included — computed by power_loss module
    pub p_contact_total_w: f64,
    /// Bearing friction torque estimate [N·mm]
    pub m_friction_nmm: f64,
    /// Active friction model (echoes `OperatingConditions.friction_model`).
    #[serde(default)]
    pub friction_model: FrictionModel,
    /// SKF reference frictional moment (Catalogue 2018 advanced model).
    /// Always populated when bearing rotates and series/Y-factor are valid.
    /// When `friction_model = SkfAdvanced`, the bearing-level totals above
    /// (p_rolling_w, p_sliding_w, p_contact_total_w, m_friction_nmm) are
    /// derived from this; the per-roller breakdown remains Palmgren-style.
    #[serde(default)]
    pub skf_reference: Option<SkfFrictionRef>,
}

/// SKF Catalogue 2018 frictional moment reference exposed to the UI.
/// Mirror of `lubrication::SkfFrictionMoment` but with `Serialize`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SkfFrictionRef {
    pub m_rr_nmm: f64,
    pub m_sl_nmm: f64,
    pub m_total_nmm: f64,
    pub p_rolling_w: f64,
    pub p_sliding_w: f64,
    pub p_total_w: f64,
    pub phi_ish: f64,
    pub phi_rs: f64,
    pub phi_bl: f64,
    pub g_rr: f64,
    pub g_sl: f64,
    pub mu_sl: f64,
    pub d_m_mm: f64,
    pub nu_op_cst: f64,
    pub n_rpm: f64,
    pub series: SkfTrbSeriesEnum,
    pub lubrication: SkfLubricationEnum,
    pub y_factor: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub level: AlertLevel,
    pub category: String,
    pub message: String,
    pub value: f64,
    pub threshold: f64,
}

/// Auto-calculated geometry summary derived from input parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometrySummary {
    pub roller_taper_angle_deg: f64,   // roller half-taper angle [deg]
    pub roller_taper_angle_rad: f64,   // roller half-taper angle [rad]
    pub e_star_gpa: f64,               // combined elastic modulus E* [GPa]
    pub d_we_mean: f64,                // mean roller diameter [mm]
    pub cone_angle_deg: f64,           // full cone angle (2×taper) [deg]
    pub gamma_dw: f64,                 // D_we_mean / d_pw ratio
    pub contact_length_ratio: f64,     // l_we / d_we_mean ratio
    pub f_r_kn: f64,                   // resultant radial load [kN]
    pub f_a_kn: f64,                   // axial load [kN]
    pub gamma_rad: f64,                // misalignment [rad]
    pub slice_geometries: Vec<SliceGeometry>,
    pub mass_roller_g: f64,        // single roller mass [g]
    pub mass_rollers_total_g: f64, // all rollers mass [g]
    pub mass_inner_race_g: f64,    // inner race (cone) mass [g]
    pub mass_outer_race_g: f64,    // outer race (cup) mass [g]
    pub mass_total_g: f64,         // total bearing mass (races + rollers, no cage) [g]
}

/// Static load rating result (ISO 76 + ISO 17956).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticRatingResult {
    // ISO 76
    pub c_0r_kn: f64,              // basic static radial load rating C₀ᵣ [kN]
    pub p_0r_kn: f64,              // static equivalent radial load P₀ᵣ [kN]
    pub s_0: f64,                   // static safety factor S₀ = C₀ᵣ / P₀ᵣ
    pub x_0: f64,                   // static radial load factor X₀
    pub y_0: f64,                   // static axial load factor Y₀
    // ISO 17956
    pub q_0: f64,                   // reference lamina load q₀ [N]
    pub q_max: f64,                 // maximum lamina load q_max [N] (from actual load distribution)
    pub s_0_eff: f64,               // effective static safety factor S₀,eff = q₀ / q_max
    pub q_max_roller_idx: usize,    // roller index where q_max occurs
    pub q_max_lamina_idx: usize,    // lamina index where q_max occurs
    pub s_0_adequate: bool,         // S₀,eff >= f_s_min
}

/// ISO 15312:2018 Thermal speed rating result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalSpeedResult {
    /// Thermal speed rating nθr [min⁻¹]
    pub n_theta_r: f64,
    /// Speed ratio = n_operating / n_theta_r
    pub speed_ratio: f64,
    /// Heat emitting reference surface area Ar [mm²]
    pub a_r: f64,
    /// Mean bearing diameter dm = 0.5·(D+d) [mm]
    pub d_m: f64,
    /// Reference load P₁ᵣ = 0.05·C₀ᵣ [N]
    pub p_1r: f64,
    /// Load-independent frictional moment M₀ᵣ at nθr [N·mm]
    pub m_0r: f64,
    /// Load-dependent frictional moment M₁ᵣ [N·mm]
    pub m_1r: f64,
    /// Bearing power loss Nr at nθr [W]
    pub n_r: f64,
    /// Reference heat flow Φᵣ [W]
    pub phi_r: f64,
    /// Reference heat flow density qᵣ [W/mm²]
    pub q_r: f64,
    /// Coefficient f₀ᵣ used (load-independent friction)
    pub f_0r: f64,
    /// Coefficient f₁ᵣ used (load-dependent friction)
    pub f_1r: f64,
    /// Reference kinematic viscosity vᵣ at θᵣ=70°C [mm²/s]
    pub v_r: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearingResult {
    pub mode: SolverMode,
    pub equilibrium: BearingEquilibrium,
    pub geometry: GeometrySummary,
    pub life: FatigueLifeResult,
    pub static_rating: StaticRatingResult,
    pub thermal_speed: ThermalSpeedResult,
    pub alerts: Vec<Alert>,
    #[serde(default)]
    pub elapsed_ms: f64,
    /// Induced axial thrust from radial loading [kN].
    /// When F_a_input < F_a_induced, the solver uses F_a_induced
    /// (reaction from mating bearing in paired TRB arrangement).
    #[serde(default)]
    pub f_a_induced_kn: f64,
    /// Effective axial load used by the solver [kN] = max(F_a_input, F_a_induced).
    #[serde(default)]
    pub f_a_effective_kn: f64,
    /// Preload mode used for this analysis.
    #[serde(default)]
    pub preload_mode: PreloadMode,
    /// Preload axial displacement δz [μm].
    /// Non-zero when preload_mode = DisplacementFromForce or Displacement.
    #[serde(default)]
    pub delta_preload_um: f64,
    /// Actual axial reaction force [kN] from rollers after equilibrium.
    /// In displacement mode, this differs from input f_a.
    #[serde(default)]
    pub f_a_reaction_kn: f64,
    /// Bearing radial stiffness [N/μm] at the converged state.
    #[serde(default)]
    pub k_radial: f64,
    /// Bearing axial stiffness [N/μm] at the converged state.
    #[serde(default)]
    pub k_axial: f64,
    /// Contact traction summary (for power loss integration).
    /// None if speed is zero (static analysis).
    #[serde(default)]
    pub traction: Option<TractionSummary>,
    /// Per-roller × per-slice EHL film thickness distribution.
    /// Computed with slice-local R_eq_k, u_m_k, q_k for accurate TRB analysis.
    #[serde(default)]
    pub film_distribution: Option<Vec<RollerFilmDistribution>>,
    /// Radial load direction angle [deg]. Roller #0 is aligned here (worst-case).
    /// 0° when pure axial load (F_r ≈ 0).
    #[serde(default)]
    pub load_angle_deg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualModeComparison {
    pub gen1_result: BearingResult,
    pub gen3_result: BearingResult,
    pub delta_p_max_pct: f64,
    pub delta_q_max_pct: f64,
    pub delta_l10_pct: f64,
    pub gen3_recommended: bool,
    pub recommendation_reason: String,
    pub gen1_elapsed_ms: f64,
    pub gen3_elapsed_ms: f64,
    pub total_elapsed_ms: f64,
}

// ─── Transient Analysis Types ──────────────────────────────────────

/// Single time point in a load time series.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTimePoint {
    /// Time [s]
    pub t_s: f64,
    /// Radial load X [kN]
    pub f_x: f64,
    /// Radial load Y [kN]
    pub f_y: f64,
    /// Axial load [kN]
    // CRB: f_a 제거 (D4), m_y 제거 (D6). LoadTimePoint 는 CSV 로드 시계열 입력에도 동일.
    /// Moment X [kN·m]
    pub m_x: f64,
    /// Rotational speed [rpm]
    pub n_rpm: f64,
}

impl LoadTimePoint {
    /// Convert to steady-state OperatingConditions using reference lubrication parameters.
    pub fn to_operating(&self, ref_op: &OperatingConditions) -> OperatingConditions {
        OperatingConditions {
            f_x: self.f_x,
            f_y: self.f_y,
            m_x: self.m_x,
            n_inner_rpm: self.n_rpm,
            n_outer_rpm: 0.0,
            // Inherit lubrication/temperature from reference
            gamma: ref_op.gamma,
            t_op: ref_op.t_op,
            nu_40: ref_op.nu_40,
            nu_100: ref_op.nu_100,
            alpha_pv: ref_op.alpha_pv,
            lubrication_type: ref_op.lubrication_type,
            starvation_factor: ref_op.starvation_factor,
            rho_oil: ref_op.rho_oil,
            lubrication_model: ref_op.lubrication_model,
            tau_eyring: ref_op.tau_eyring,
            z_roelands: ref_op.z_roelands,
            traction_model: ref_op.traction_model,
            carreau_eta_inf_ratio: ref_op.carreau_eta_inf_ratio,
            carreau_lambda_s: ref_op.carreau_lambda_s,
            carreau_n: ref_op.carreau_n,
            carreau_a: ref_op.carreau_a,
            friction_model: ref_op.friction_model,
            thermal_correction: ref_op.thermal_correction,
            hysteresis_loss_factor: ref_op.hysteresis_loss_factor,
            // skf_trb_series 제거 (Phase 7 CRB 대응 검토)
            skf_lubrication: ref_op.skf_lubrication,
            skf_y_factor: ref_op.skf_y_factor,
            k_fluid: ref_op.k_fluid,
            beta_visc: ref_op.beta_visc,
            roughness_input_mode: ref_op.roughness_input_mode,
            rq_inner: ref_op.rq_inner,
            rq_outer: ref_op.rq_outer,
            rq_roller: ref_op.rq_roller,
            design_life_hours: ref_op.design_life_hours,
            film_decay_enabled: ref_op.film_decay_enabled,
            film_decay_time_hours: ref_op.film_decay_time_hours,
            skew_angle_deg: ref_op.skew_angle_deg,
            replenishment_rate_nm_s: ref_op.replenishment_rate_nm_s,
            surface_finish: ref_op.surface_finish,
            additive_type: ref_op.additive_type,
        }
    }
}

/// Transient analysis input configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransientInput {
    /// Time-load series
    pub load_series: Vec<LoadTimePoint>,
    /// Maximum time step [s] (adaptive stepping may use smaller)
    #[serde(default = "default_dt_max")]
    pub dt_max: f64,
    /// Enable roller inertia dynamics model
    #[serde(default = "default_true")]
    pub enable_roller_dynamics: bool,
    /// Snapshot save interval (save every N steps; 1 = all)
    #[serde(default = "default_snapshot_interval")]
    pub snapshot_interval: usize,
}

fn default_dt_max() -> f64 {
    1e-3 // 1 ms
}

fn default_true() -> bool {
    true
}

fn default_snapshot_interval() -> usize {
    1
}

/// Per-roller kinematic state at a single time step (including inertia effects).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollerKinematicState {
    /// Roller index
    pub j: usize,
    /// Orbital position [deg]
    pub psi_deg: f64,
    /// Actual roller angular velocity [rad/s] (with inertia lag)
    pub omega_roller_actual: f64,
    /// Pure rolling target angular velocity [rad/s]
    pub omega_roller_target: f64,
    /// Slip ratio: (actual − target) / target
    pub slip_ratio: f64,
    /// Average sliding velocity [m/s]
    pub u_slide_avg: f64,
    /// Applied traction torque [N·m]
    pub tau_traction: f64,
    /// Maximum available traction torque [N·m]
    pub tau_traction_max: f64,
    /// True if roller is in slip (|slip_ratio| > threshold)
    pub in_slip: bool,
    /// Per-slice SRR values (geometric + dynamic) for this roller.
    /// Empty if not computed (e.g., no profile modification).
    #[serde(default)]
    pub slice_srr: Vec<f64>,
}

/// Summary sliding metrics for a single time step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransientSlidingMetrics {
    /// Number of rollers currently in slip
    pub n_rollers_in_slip: usize,
    /// Maximum absolute slip ratio across all rollers
    pub max_slip_ratio: f64,
    /// Maximum sliding velocity [m/s]
    pub max_slide_velocity: f64,
    /// Instantaneous total friction power [W]
    pub instantaneous_friction_power: f64,
    /// Maximum absolute per-slice SRR across all rollers and slices
    #[serde(default)]
    pub max_slice_srr: f64,
}

/// Transient analysis result — single time step snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransientSnapshot {
    /// Time [s]
    pub t_s: f64,
    /// Operating conditions at this step
    pub operating: OperatingConditions,
    /// Bearing equilibrium solution
    pub equilibrium: BearingEquilibrium,
    /// Per-roller kinematic state (with inertia)
    pub roller_kinematics: Vec<RollerKinematicState>,
    /// Sliding metrics summary
    pub sliding_metrics: TransientSlidingMetrics,
    /// Per-slice SRR contour data: slice_srr_map[j][k] = SRR for roller j, slice k.
    /// Only populated at snapshot intervals. Empty vec if slice sliding disabled.
    #[serde(default)]
    pub slice_srr_map: Vec<Vec<f64>>,
}

/// Per-roller cumulative damage accumulator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollerDamageAccumulator {
    /// Roller index
    pub j: usize,
    /// Cumulative friction energy ∫μQ|V_slide|dt [J]
    pub cumulative_friction_energy_j: f64,
    /// Cumulative sliding distance ∫|V_slide|dt [m]
    pub cumulative_slide_distance_m: f64,
    /// Maximum contact load during slip events [N]
    pub max_contact_load_during_slip_n: f64,
    /// Number of slip events
    pub slip_event_count: usize,
    /// Total slip duration [s]
    pub total_slip_duration_s: f64,
}

/// Bearing-level cumulative damage summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransientDamageSummary {
    /// Per-roller damage accumulators
    pub roller_damage: Vec<RollerDamageAccumulator>,
    /// Total slip events across all rollers
    pub total_slip_events: usize,
    /// Total slip duration [s]
    pub total_slip_duration_s: f64,
    /// Maximum slip ratio observed overall
    pub max_slip_ratio_overall: f64,
    /// WEC risk index (NREL Guo criterion, 0–1 scale)
    pub wec_risk_index: f64,
}

/// Complete transient analysis result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransientResult {
    /// Time step snapshots (may be subsampled per snapshot_interval)
    pub snapshots: Vec<TransientSnapshot>,
    /// Cumulative damage summary
    pub damage_summary: TransientDamageSummary,
    /// Total simulation time [s]
    pub total_time_s: f64,
    /// Elapsed wall-clock computation time [ms]
    pub elapsed_ms: f64,
    /// Risk assessment (computed after transient solve)
    #[serde(default)]
    pub risk_assessment: Option<TransientRiskAssessment>,
}

// ─── WEC / Smearing Risk Assessment Types ──────────────────────────

/// Risk level classification.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// NREL Guo(2021) WEC risk assessment result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WecRiskAssessment {
    /// Fraction of time with slip under high contact load
    pub slip_load_fraction: f64,
    /// Maximum contact load during slip [N]
    pub q_max_during_slip: f64,
    /// Number of high-load slip events (Q > 0.5×C_dyn)
    pub high_load_slip_events: usize,
    /// Guo risk index (0–1 scale)
    pub risk_index: f64,
    /// Risk level
    pub risk_level: RiskLevel,
}

/// Smearing risk assessment result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmearingRiskAssessment {
    /// Maximum SRR observed (absolute value)
    pub max_srr: f64,
    /// Maximum negative SRR (roller slower than raceway — WEC-prone direction)
    /// Wear 2022: WEC initiates under negative slip, not positive.
    #[serde(default)]
    pub max_negative_srr: f64,
    /// Maximum positive SRR (roller faster than raceway)
    #[serde(default)]
    pub max_positive_srr: f64,
    /// Maximum flash temperature rise [°C]
    pub max_flash_temp_rise: f64,
    /// Total sliding distance [m]
    pub total_slide_distance: f64,
    /// Peak sliding velocity [m/s]
    pub peak_slide_velocity: f64,
    /// Risk level
    pub risk_level: RiskLevel,
}

/// Comprehensive transient risk assessment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransientRiskAssessment {
    /// NREL Guo WEC criterion
    pub wec_guo: WecRiskAssessment,
    /// Cumulative friction energy ratio (E_cumulative / E_critical)
    pub wec_energy_ratio: f64,
    /// Smearing risk
    pub smearing: SmearingRiskAssessment,
    /// Overall risk level (worst of all criteria)
    pub overall_risk_level: RiskLevel,
    /// Recommendations
    pub recommendations: Vec<String>,
}
