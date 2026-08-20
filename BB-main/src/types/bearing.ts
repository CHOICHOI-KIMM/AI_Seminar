// TypeScript interfaces mirroring Rust solver types

// ─── Input Types ────────────────────────────────────────────────────

// CRB (Cylindrical Roller Bearing) — Plan §6 D1~D7 반영
// D1: rib 제거, D3: 단일 row, D4: F_a 제거, D6: γ_y 제거, D7: 3-DOF (δx, δy, γx)
export interface MacroGeometry {
  d: number;                // Bore diameter [mm]
  outer_diameter: number;   // Outer diameter [mm]
  t: number;                // Bearing width [mm]
  z: number;                // Number of rollers
  d_we: number;             // Roller diameter (uniform for CRB) [mm]
  l_we: number;             // Roller effective contact length (along roller axis, ISO p.4 NOTE 3) [mm]
  d_pw: number;             // Pitch circle diameter [mm]
  g_r: number;              // Radial internal clearance [μm]
}

export interface RacewayGeometry {
  r_i: number;              // Inner raceway transverse curvature radius [mm]
  r_o: number;              // Outer raceway transverse curvature radius [mm]
  d_uc: number;             // Raceway undercut depth [mm]
  l_uc: number;             // Raceway undercut axial extent [mm]
}

export type CrownType =
  | { Logarithmic: { a_log: number } }
  | { Circular: { r_crown: number } }
  | { Parabolic: { c2: number } }
  | { Custom: { profile: [number, number][] } }
  | { Polynomial: { coeffs: number[] } }; // [p1..p5]: p1*x^4 + p2*x^3 + p3*x^2 + p4*x + p5 [um]

// CRB: 양 끝 대칭 (D1: rib 없음, 부록 A.1)
export interface RollerProfile {
  crown_type: CrownType;
  delta_c: number;         // Crown drop center-to-end [μm]
  delta_dub: number;       // Dub-off amount (both ends, symmetric) [μm]
  l_dub: number;           // Dub-off length (both ends, symmetric) [mm]
  sigma_roller: number;    // roller surface roughness Ra [μm]
}

export interface RacewayProfile {
  delta_rw: number;
  w_a: number;
  ra: number;
  custom_profile: [number, number][] | null;
  polynomial_coeffs: number[] | null; // [p1..p5]: 4th-order polynomial coefficients [um]
}

export interface Material {
  e_roller: number;
  e_ring: number;
  nu: number;
  hrc: number;
  density_roller: number;  // roller density [g/cm³]
  density_ring: number;    // ring (inner/outer race) density [g/cm³]
}

export type LubricationType = "Oil" | "Grease";

// CRB: PreloadMode 제거 (D4: axial preload 무관). 필요 시 Phase 7 에서 재검토.

/** Lubrication analysis model selection
 *  - Method1_DH: Dowson-Higginson (1977) — classic isothermal line-contact EHL
 *  - Method2_MK: Masjedi-Khonsari (2015) — roughness-integrated EHL
 */
export type LubricationModel = "Method1_DH" | "Method2_MK" | "Method3_NVM";

// CRB Operating Conditions — D4 (f_a 제거), D6 (m_y 제거), 평형 DOF = 3
export interface OperatingConditions {
  f_x: number;            // Radial load X (horizontal) [kN]
  f_y: number;            // Radial load Y (vertical, gravity) [kN]
  m_x: number;            // Tilting moment about X (single-plane, D6) [kN·m]
  n_inner_rpm: number;    // Inner ring speed [rpm]
  n_outer_rpm: number;    // Outer ring speed [rpm]
  gamma: number;          // External misalignment (about X-axis, D6) [arcmin]
  t_op: number;
  nu_40: number;
  nu_100: number;
  alpha_pv: number;           // pressure-viscosity coefficient [1/GPa]
  lubrication_type: LubricationType;
  starvation_factor: number;  // φ_s ∈ (0,1], 1.0=fully flooded
  rho_oil: number;            // lubricant density [kg/m³]
  /** Design life duration for damage/reliability calculation [hours] */
  design_life_hours: number;
  // === Advanced lubrication model parameters ===
  lubrication_model: LubricationModel;  // Method1_DH or Method2_MK
  film_decay_enabled: boolean;          // Van Zoelen film decay model ON/OFF
  film_decay_time_hours: number;        // operating time for decay [hours]
  skew_angle_deg: number;               // roller skew angle [degrees]
  replenishment_rate_nm_s: number;      // replenishment rate R [nm/s]
  surface_finish: SurfaceFinish;        // surface finish class
  additive_type: AdditiveType;          // lubricant additive type
  tau_eyring: number;         // Eyring stress τ₀ [MPa] (default 5.0)
  z_roelands: number;        // Roelands pressure-viscosity exponent (default 0.67)
  traction_model: TractionModel;       // Eyring | CarreauYasuda (default Eyring)
  carreau_eta_inf_ratio: number;       // η_∞ / η_0 (default 0.005)
  carreau_lambda_s: number;            // Carreau-Yasuda relaxation time λ [s] (default 1e-7)
  carreau_n: number;                   // Carreau-Yasuda power-law exponent (default 0.5)
  carreau_a: number;                   // Yasuda transition exponent (default 2.0; a=2 = original Carreau)
  friction_model: FrictionModel;        // PalmgrenLike (default) | SkfAdvanced
  thermal_correction: ThermalCorrection;  // Wilson1979 | Aihara1987 (default) | None — for BH rolling friction
  hysteresis_loss_factor: number;       // Johnson 1985 α_v [-], default 0.005 (range 0.005-0.05 hardened bearing steel)
  // skf_trb_series 제거 (TRB 전용, CRB Phase 7 에서 SKF CRB 대응 검토)
  skf_lubrication: SkfLubrication;      // OilBath | OilJet | Grease | OilAir
  skf_y_factor: number;                 // SKF axial load factor Y (catalogue, ~1.6 for 30306)
  k_fluid: number;           // Lubricant thermal conductivity [W/(m·K)] (default 0.15)
  beta_visc: number;         // Viscosity-temperature coefficient [1/K] (default 0.04)
  roughness_input_mode: RoughnessInputMode;  // Ra or Rq input mode
  rq_inner: number;          // Inner raceway roughness [μm] (Ra or Rq per mode)
  rq_outer: number;          // Outer raceway roughness [μm] (Ra or Rq per mode)
  rq_roller: number;         // Roller roughness [μm] (Ra or Rq per mode)
}

export type SolverMode = "Gen1" | "Gen3";
export type LifeMethod = "Iso16281";
export type ContaminationLevel = "HighCleanliness" | "NormalCleanliness" | "SlightContamination" | "SevereContamination" | "VeryHeavyContamination";
export type OilSupplyMethod = "OilBath" | "CirculatingWithFilter" | "Grease";
export type RibContactMode = "PostProcess" | "Coupled";
export type KappaMethod = "ViscosityRatio" | "FilmThicknessRatio";
export type RoughnessInputMode = "Ra" | "Rq";
export type RunMode = { Single: SolverMode } | "Dual";
export type BeamType = "EulerBernoulli" | "Timoshenko";

export interface SolverParams {
  run_mode: RunMode;
  n_slices: number;
  beam_type: BeamType;
  convergence_tol: number;
  max_iterations: number;
  angular_increment_deg: number;
  life_method: LifeMethod;
  e_c: number;                    // contamination factor (0=auto, >0=manual)
  contamination_level: ContaminationLevel;
  oil_supply_method: OilSupplyMethod;
  c_r_kn: number | null;
  c_0r_kn: number | null;
  f_s_min: number;
  rib_contact_mode: RibContactMode;
  /** ISO 15312 f₀ᵣ coefficient (TRB 02/03/29/30: 3.0, 22/23/13/31/32: 4.5) */
  f_0r: number;
  /** ISO 15312 f₁ᵣ coefficient (all TRB series: 0.0004) */
  f_1r: number;
  /** κ calculation method: "ViscosityRatio" (κ=ν/ν₁) or "FilmThicknessRatio" (κ≈Λ^1.3) */
  kappa_method: KappaMethod;
  /** Use split contact model (independent inner/outer q_k per slice) */
  use_split_contact?: boolean;
}

export interface BearingInput {
  macro_geom: MacroGeometry;
  raceway_geom: RacewayGeometry;
  roller_profile: RollerProfile;
  raceway_profile_inner: RacewayProfile;
  raceway_profile_outer: RacewayProfile;
  material: Material;
  operating: OperatingConditions;
  solver: SolverParams;
  /** Transient analysis input (null = steady-state only) */
  transient?: TransientInput | null;
}

// ─── Output Types ───────────────────────────────────────────────────

export interface SliceGeometry {
  k: number;
  x_axial: number;
  r_roller: number;
  r_inner_race: number;
  r_outer_race: number;
  r_eq_inner: number;
  r_eq_outer: number;
  delta_z_total_inner: number;
  delta_z_total_outer: number;
  slice_width: number;
}

export interface SliceContactResult {
  k: number;
  delta_k: number;
  q_k: number;
  q_k_outer: number;
  q_k_inner: number;
  b_k: number;
  p_max_k: number;
  h_bulk_k: number;
  b_k_outer: number;
  p_max_k_outer: number;
  h_bulk_k_outer: number;
  k_hertz_k: number;
  k_hertz_k_outer: number;
  k_combined_k: number;
  in_contact: boolean;
}

export interface RibEhlResult {
  h_c_um: number;                  // central film thickness [μm]
  h_min_um: number;                // minimum film thickness [μm]
  sigma_composite_um: number;      // composite roughness √(σ_roller² + σ_rib²)
  lambda_ratio: number;            // Λ = h_min / σ_c
  regime: LubricationRegime;       // FullEhl | Mixed | Boundary
  mu_eff: number;                  // effective traction (mixed)
  mu_ehl: number;                  // fluid-only traction
  asperity_load_ratio: number;     // f_a = F_{5/2}(Λ) / F_{5/2}(0)
  p_asperity_mpa: number;          // asperity-borne pressure [MPa]
  flash_temp_c: number;            // Blok-Jaeger flash ΔT [°C]
  srr: number;                     // slide-roll ratio (≈ 2 at rib pure-sliding)
  u_entrain_m_s: number;           // mean entrainment velocity
  u_slide_m_s: number;             // sliding velocity
  thermal_factor: number;          // φ_T (Murch-Wilson)
  u_param: number;                 // dimensionless U
  g_param: number;                 // dimensionless G
  w_param: number;                 // dimensionless W
  k_ellipse: number;               // ellipticity a/b
}

export interface RibContactResult {
  f_rib: number;
  a_ellipse: number;
  b_ellipse: number;
  p_max_rib: number;
  spin_moment: number;
  delta_rib: number;
  k_rib: number;
  r_contact_mm: number;
  r_rib_circ_mm: number;
  h_c_mm: number;
  ehl: RibEhlResult | null;
}

export interface RollerResult {
  psi_deg: number;
  q_normal: number;        // outer raceway normal load [N]
  q_normal_inner: number;  // inner raceway normal load [N]
  slice_results: SliceContactResult[];
  rib_result: RibContactResult | null;
}

export interface AngularLoadPoint {
  psi_deg: number;
  delta_rigid: number;
  q_total: number;
  p_max: number;
  slice_p_max: number[];
  slice_p_max_outer: number[];
  slice_q_k: number[];
  is_roller: boolean;
}

export interface BearingEquilibrium {
  displacement: [number, number, number, number, number];
  roller_loads: number[];
  roller_results: RollerResult[];
  angular_distribution: AngularLoadPoint[];
}

export interface LaminaLife {
  k: number;
  q_equiv_inner: number;
  q_equiv_outer: number;
  l_10_inner: number;
  l_10_outer: number;
  p_sk: number;        // ISO 16281 Eq.(31) lamina reference load [kN]
  a_iso_k_inner: number;  // per-lamina life modification factor (inner raceway)
  a_iso_k_outer: number;  // per-lamina life modification factor (outer raceway)
}

export interface LifeIntermediates {
  nu_actual: number;
  nu_ref: number;
  b_m: number;
  f_c: number;
  gamma_bearing: number;
  c_u_kn: number;
  c_u_over_p: number;
  e_demarcation: number;
  x_factor: number;
  y_factor: number;
  f_a_over_f_r: number;
  f_ci: number;
  f_co: number;
  q_c_base: number;
  q_ci: number;
  q_co: number;
  q_ei: number;
  q_eo: number;
  weibull_e: number;
  l_nm_mrev: number;
  q_c_lamina_inner: number;
  q_c_lamina_outer: number;
  e_c_used: number;       // actual contamination factor used (auto or manual)
  kappa_method: KappaMethod; // method used for κ calculation
  lambda_inner: number | null; // Λ value for inner raceway (when FilmThicknessRatio method)
  lambda_outer: number | null; // Λ value for outer raceway (when FilmThicknessRatio method)
}

export type LubricationRegime = "FullEhl" | "Mixed" | "Boundary";

export interface SliceFilmThickness {
  h_min_um: number;
  h_central_um: number;
  lambda: number;
  regime: LubricationRegime;
  h_min_um_outer: number;
  h_central_um_outer: number;
  lambda_outer: number;
  regime_outer: LubricationRegime;
}

export interface RollerFilmDistribution {
  roller_idx: number;
  psi_deg: number;
  slices: SliceFilmThickness[];
}

export interface MixedLubricationResult {
  asperity_load_ratio: number;   // F_asperity / F_total [0,1]
  asperity_area_ratio: number;   // A_real / A_hertz [0,1]
  p_asperity_mpa: number;
  p_fluid_mpa: number;
  mu_ehl: number;                // EHL traction coefficient
  mu_boundary: number;           // boundary friction coefficient
  mu_effective: number;          // weighted effective μ
  f_5_2: number;                 // GT F_{5/2}(Λ) integral
  f_2: number;                   // GT F_2(Λ) integral
}

export interface FilmThicknessResult {
  // Inner raceway
  h_min_um: number;
  h_central_um: number;
  sigma_composite_um: number;
  lambda_ratio: number;
  regime: LubricationRegime;
  // Outer raceway
  h_min_um_outer: number;
  h_central_um_outer: number;
  sigma_composite_um_outer: number;
  lambda_ratio_outer: number;
  regime_outer: LubricationRegime;
  // Individual roughness values
  rq_roller_um: number;
  rq_inner_um: number;
  rq_outer_um: number;
  // Kinematics
  u_mean_m_s: number;           // inner entraining velocity [m/s]
  u_mean_m_s_outer: number;     // outer entraining velocity [m/s]
  cage_speed_rpm: number;       // cage orbital speed [rpm]
  roller_spin_rpm: number;      // roller spin speed [rpm]
  // Common
  starvation_factor: number;
  thermal_factor: number;
  u_param: number;
  g_param: number;
  w_param: number;
  mixed: MixedLubricationResult;
  flash_temp_c?: number;           // flash temperature [°C] (Method2_MK only)
  micropitting?: MicropittingSafety;  // micropitting safety assessment
  film_decay?: FilmDecayResult;    // Van Zoelen film decay (when enabled)
}

/** Van Zoelen film thickness decay result */
export interface FilmDecayResult {
  t_hours: number;
  h_c_decayed_inner_um: number;
  h_c_decayed_outer_um: number;
  starvation_ratio_inner: number;
  starvation_ratio_outer: number;
  f0_inner: number;
  f0_outer: number;
  lambda_decayed_inner: number;
  lambda_decayed_outer: number;
  regime_decayed_inner: LubricationRegime;
  regime_decayed_outer: LubricationRegime;
  replenishment_rate_nm_s: number;
  h_c_equilibrium_inner_um?: number;
  h_c_equilibrium_outer_um?: number;
  decay_curve: [number, number, number][];  // [t_hours, h_c_inner_nm, h_c_outer_nm]
}

/** Micropitting safety assessment (ISO/TS 6336-22 framework adapted for bearings) */
export interface MicropittingSafety {
  lambda_perm: number;
  lambda_perm_base: number;
  additive_factor: number;
  s_lambda_inner: number;
  s_lambda_outer: number;
  risk_inner: MicropittingRisk;
  risk_outer: MicropittingRisk;
}

export type MicropittingRisk = "Safe" | "Marginal" | "AtRisk";

export type SurfaceFinish = "Standard" | "FineGround" | "Superfinish";
export type AdditiveType = "None" | "EP" | "AW";
export type TractionModel = "Eyring" | "CarreauYasuda";
export type FrictionModel = "PalmgrenLike" | "BibouletHoupert" | "SkfAdvanced";

/**
 * Thermal inlet-shear correction model for BH 2010 rolling friction.
 * - Wilson1979: φ_T = 1/(1+0.1·L^0.64) — film-thickness-derived, conservative.
 * - Aihara1987: φ_T = 1/(1+0.29·L^0.78) — calibrated for TRB rolling torque,
 *   matches Schwarz 2023 measurements within 8% at high speed.
 * - None: isothermal.
 */
export type ThermalCorrection = "Wilson1979" | "Aihara1987" | "None";
export type SkfTrbSeries =
  | "Series302" | "Series303" | "Series313" | "Series320"
  | "Series322" | "Series322B" | "Series323" | "Series323B" | "Other";
export type SkfLubrication = "OilBath" | "OilJet" | "Grease" | "OilAir";

export interface SkfFrictionRef {
  m_rr_nmm: number;
  m_sl_nmm: number;
  m_total_nmm: number;
  p_rolling_w: number;
  p_sliding_w: number;
  p_total_w: number;
  phi_ish: number;
  phi_rs: number;
  phi_bl: number;
  g_rr: number;
  g_sl: number;
  mu_sl: number;
  d_m_mm: number;
  nu_op_cst: number;
  n_rpm: number;
  series: SkfTrbSeries;
  lubrication: SkfLubrication;
  y_factor: number;
}

/** HMEHL (Micro-EHL) solver result — Tier 3 manual analysis */
export interface HMEHLResult {
  mu: number;
  mu_fluid: number;
  mu_asperity: number;
  h_central: number;     // [m]
  h_min: number;         // [m]
  p_asp_mean: number;    // [Pa]
  p_max: number;         // [Pa]
  power_loss_per_m: number; // [W/m]
  tau_surf_max: number;  // [Pa]
  pressure: number[];    // pressure distribution [Pa]
  hertz_pressure_ref: number[]; // Hertz contact pressure for comparison [Pa]
  film: number[];        // film distribution [m]
  temperature: number[]; // TEHL temperature distribution [°C]
  t_max: number;         // max film temperature [°C]
  t_mean_contact: number; // mean temperature in contact zone [°C]
  iterations: number;
  converged: boolean;
}

// ─── Contact Traction (Power Loss Readiness) ───

export interface ContactFriction {
  u_rolling: number;             // rolling velocity [m/s]
  u_sliding: number;             // sliding velocity [m/s]
  srr: number;                   // slide-roll ratio
  lambda: number;                // local Lambda ratio
  asperity_load_ratio: number;   // local asperity fraction
  mu: number;                    // effective friction coefficient
  f_traction_n: number;          // traction force [N]
  power_loss_w: number;          // sliding power dissipation [W]
  p_rolling_w: number;           // rolling resistance power [W] (Palmgren μ_rr × Q × u_rolling; BH viscous EHL)
  p_hysteresis_w: number;        // Johnson 1985 material hysteresis power [W] (BH only; 0 for Palmgren/SKF)
}

export interface RibFriction {
  u_sliding: number;
  mu: number;
  f_friction_n: number;
  power_loss_w: number;
}

export interface RollerTractionResult {
  roller_idx: number;
  psi_deg: number;
  inner: ContactFriction;
  outer: ContactFriction;
  rib: RibFriction | null;
}

export interface TractionSummary {
  rollers: RollerTractionResult[];
  p_rolling_w: number;           // total rolling friction power [W] (BH 2010 viscous EHL only)
  p_sliding_w: number;           // total sliding friction power [W]
  p_rib_w: number;               // total rib friction power [W]
  p_hysteresis_w: number;        // total Johnson 1985 material hysteresis power [W] (BH only)
  p_contact_total_w: number;     // total contact power loss [W] = rolling + sliding + rib + hysteresis
  m_friction_nmm: number;        // bearing friction torque [N·mm]
  friction_model: FrictionModel;       // active friction model
  skf_reference: SkfFrictionRef | null; // SKF Catalogue 2018 reference
}

/** ISO 15312:2018 Thermal speed rating result */
export interface ThermalSpeedResult {
  /** Thermal speed rating nθr [min⁻¹] */
  n_theta_r: number;
  /** Speed ratio = n_operating / n_theta_r */
  speed_ratio: number;
  /** Heat emitting reference surface area Ar [mm²] */
  a_r: number;
  /** Mean bearing diameter dm [mm] */
  d_m: number;
  /** Reference load P₁ᵣ = 0.05·C₀ᵣ [N] */
  p_1r: number;
  /** Load-independent frictional moment M₀ᵣ at nθr [N·mm] */
  m_0r: number;
  /** Load-dependent frictional moment M₁ᵣ [N·mm] */
  m_1r: number;
  /** Bearing power loss Nr at nθr [W] */
  n_r: number;
  /** Reference heat flow Φᵣ [W] */
  phi_r: number;
  /** Reference heat flow density qᵣ [W/mm²] */
  q_r: number;
  /** f₀ᵣ coefficient used */
  f_0r: number;
  /** f₁ᵣ coefficient used */
  f_1r: number;
  /** Reference viscosity vᵣ at θᵣ=70°C [mm²/s] */
  v_r: number;
}

export interface FatigueLifeResult {
  method: LifeMethod;
  l_10_basic: number;
  l_nm_hours: number;
  l_10_inner: number;
  l_10_outer: number;
  weakest_lamina: number;
  a_iso: number;
  kappa: number;
  kappa_inner: number;
  kappa_outer: number;
  c_dyn: number;
  p_equiv: number;
  /** Back-calculated: C_r / L_10r(ISO16281)^(3/10) [kN] */
  p_ref: number;
  /** Damage-weighted average of lamina P_sk [kN] */
  p_ref_damage: number;
  intermediates: LifeIntermediates;
  lamina_lives: LaminaLife[] | null;
  film_thickness: FilmThicknessResult | null;
}

export type AlertLevel = "Info" | "Warning" | "Critical";

export interface Alert {
  level: AlertLevel;
  category: string;
  message: string;
  value: number;
  threshold: number;
}

export interface GeometrySummary {
  roller_taper_angle_deg: number;
  roller_taper_angle_rad: number;
  e_star_gpa: number;
  d_we_mean: number;
  cone_angle_deg: number;
  gamma_dw: number;
  contact_length_ratio: number;
  f_r_kn: number;
  f_a_kn: number;
  gamma_rad: number;
  slice_geometries: SliceGeometry[];
  mass_roller_g: number;
  mass_rollers_total_g: number;
  mass_inner_race_g: number;
  mass_outer_race_g: number;
  mass_total_g: number;
}

export interface StaticRatingResult {
  c_0r_kn: number;
  p_0r_kn: number;
  s_0: number;
  x_0: number;
  y_0: number;
  q_0: number;
  q_max: number;
  s_0_eff: number;
  q_max_roller_idx: number;
  q_max_lamina_idx: number;
  s_0_adequate: boolean;
}

export interface BearingResult {
  mode: SolverMode;
  equilibrium: BearingEquilibrium;
  geometry: GeometrySummary;
  life: FatigueLifeResult;
  static_rating: StaticRatingResult;
  thermal_speed: ThermalSpeedResult;
  alerts: Alert[];
  elapsed_ms: number;
  /** Induced axial thrust from radial loading [kN] */
  f_a_induced_kn: number;
  /** Effective axial load used by solver [kN] = max(F_a_input, F_a_induced) */
  f_a_effective_kn: number;
  // CRB: preload_mode / delta_preload_um 제거 (D4: axial preload 무관).
  // BearingResult 는 backend serde 에서 optional 이므로 클라이언트 인터페이스에서만 제거.
  /** Actual axial reaction force [kN] from rollers after equilibrium */
  f_a_reaction_kn: number;
  /** Bearing radial stiffness [N/μm] at converged state */
  k_radial: number;
  /** Bearing axial stiffness [N/μm] at converged state */
  k_axial: number;
  /** Contact traction summary for power loss integration */
  traction: TractionSummary | null;
  /** Per-roller × per-slice EHL film thickness distribution */
  film_distribution: RollerFilmDistribution[] | null;
  /** Radial load direction angle [deg]. Roller #0 aligned here (worst-case). */
  load_angle_deg: number;
}

export interface DualModeComparison {
  gen1_result: BearingResult;
  gen3_result: BearingResult;
  delta_p_max_pct: number;
  delta_q_max_pct: number;
  delta_l10_pct: number;
  gen3_recommended: boolean;
  recommendation_reason: string;
  gen1_elapsed_ms: number;
  gen3_elapsed_ms: number;
  total_elapsed_ms: number;
}

// ─── Transient Analysis Types ──────────────────────────────────────

export type LoadSourceType = 'sine' | 'csv';

export interface SineChannelParams {
  mean: number;
  amplitude: number;
}

// CRB: SineWaveConfig / LoadTimePoint 에서 f_a, m_y 제거 (D4, D6)
export interface SineWaveConfig {
  frequency_hz: number;
  duration_s: number;
  points_per_cycle: number;
  f_x: SineChannelParams;
  f_y: SineChannelParams;
  m_x: SineChannelParams;
  n_rpm: SineChannelParams;
}

export const DEFAULT_SINE_CONFIG: SineWaveConfig = {
  frequency_hz: 10,
  duration_s: 0.5,
  points_per_cycle: 50,
  f_x: { mean: 0, amplitude: 0 },
  f_y: { mean: -500, amplitude: 100 },   // -Y = 중력 방향
  m_x: { mean: 0, amplitude: 0 },
  n_rpm: { mean: 1000, amplitude: 0 },
};

export function generateSineLoadSeries(config: SineWaveConfig): LoadTimePoint[] {
  const { frequency_hz, duration_s, points_per_cycle } = config;
  const n_cycles = frequency_hz * duration_s;
  const n_points = Math.max(2, Math.round(n_cycles * points_per_cycle) + 1);
  const dt = duration_s / (n_points - 1);

  const points: LoadTimePoint[] = [];
  for (let i = 0; i < n_points; i++) {
    const t = i * dt;
    const phase = 2 * Math.PI * frequency_hz * t;
    const s = Math.sin(phase);
    points.push({
      t_s: t,
      f_x: config.f_x.mean + config.f_x.amplitude * s,
      f_y: config.f_y.mean + config.f_y.amplitude * s,
      m_x: config.m_x.mean + config.m_x.amplitude * s,
      n_rpm: config.n_rpm.mean + config.n_rpm.amplitude * s,
    });
  }
  return points;
}

export interface LoadTimePoint {
  t_s: number;
  f_x: number;
  f_y: number;
  m_x: number;
  n_rpm: number;
}

export interface TransientInput {
  load_series: LoadTimePoint[];
  dt_max: number;
  enable_roller_dynamics: boolean;
  snapshot_interval: number;
}

export interface RollerKinematicState {
  j: number;
  psi_deg: number;
  omega_roller_actual: number;
  omega_roller_target: number;
  slip_ratio: number;
  u_slide_avg: number;
  tau_traction: number;
  tau_traction_max: number;
  in_slip: boolean;
  slice_srr?: number[];
}

export interface TransientSlidingMetrics {
  n_rollers_in_slip: number;
  max_slip_ratio: number;
  max_slide_velocity: number;
  instantaneous_friction_power: number;
  max_slice_srr?: number;
}

export interface TransientSnapshot {
  t_s: number;
  operating: OperatingConditions;
  equilibrium: BearingEquilibrium;
  roller_kinematics: RollerKinematicState[];
  sliding_metrics: TransientSlidingMetrics;
  slice_srr_map?: number[][];
}

export interface RollerDamageAccumulator {
  j: number;
  cumulative_friction_energy_j: number;
  cumulative_slide_distance_m: number;
  max_contact_load_during_slip_n: number;
  slip_event_count: number;
  total_slip_duration_s: number;
}

export interface TransientDamageSummary {
  roller_damage: RollerDamageAccumulator[];
  total_slip_events: number;
  total_slip_duration_s: number;
  max_slip_ratio_overall: number;
  wec_risk_index: number;
}

export interface TransientResult {
  snapshots: TransientSnapshot[];
  damage_summary: TransientDamageSummary;
  total_time_s: number;
  elapsed_ms: number;
  risk_assessment?: TransientRiskAssessment | null;
}

// ─── WEC / Smearing Risk Assessment Types ──────────────────────────

export type RiskLevel = "Low" | "Medium" | "High" | "Critical";

export interface WecRiskAssessment {
  slip_load_fraction: number;
  q_max_during_slip: number;
  high_load_slip_events: number;
  risk_index: number;
  risk_level: RiskLevel;
}

export interface SmearingRiskAssessment {
  max_srr: number;
  /** Maximum negative SRR (roller slower — WEC-prone direction) */
  max_negative_srr?: number;
  /** Maximum positive SRR (roller faster) */
  max_positive_srr?: number;
  max_flash_temp_rise: number;
  total_slide_distance: number;
  peak_slide_velocity: number;
  risk_level: RiskLevel;
}

export interface TransientRiskAssessment {
  wec_guo: WecRiskAssessment;
  wec_energy_ratio: number;
  smearing: SmearingRiskAssessment;
  overall_risk_level: RiskLevel;
  recommendations: string[];
}
