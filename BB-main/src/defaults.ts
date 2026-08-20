import type { BearingInput } from './types/bearing';

// CRB (Cylindrical Roller Bearing) 기본값 — Phase 1.5
// NU 240 유사 파라미터 (풍력 메인베어링 후속 F4 결정 대기)
//   d = 200 mm, D = 360 mm, T = 58 mm
//   Z = 18 rollers, D_we = 44 mm, L_we = 42 mm
//   Cr ≈ 1520 kN, C0r ≈ 2400 kN (NU 시리즈 catalogue 근사)
// Plan §6 D1~D7 반영:
//   - 시리즈 무관 (D2), 단일 row (D3)
//   - Rib contact 제외 (D1) → h_rib/alpha_rib/R_sph/r_rib 등 필드 없음
//   - F_a = 0 강제 (D4), M_y = 0 강제 (D6)
//   - 평형 DOF = 3 (δx, δy, γx) (D7)
//   - 좌표: X=수평, Y=수직(중력), Z=shaft (D5)
export const defaultInput: BearingInput = {
  macro_geom: {
    d: 200.0,               // Bore diameter [mm]
    outer_diameter: 360.0,  // Outer diameter [mm]
    t: 58.0,                // Bearing width [mm]
    z: 18,                  // Number of rollers
    d_we: 44.0,             // Roller diameter (uniform for CRB) [mm]
    l_we: 42.0,             // Roller effective contact length (roller axis) [mm]
    d_pw: 280.0,            // Pitch circle diameter [mm]
    g_r: 30.0,              // Radial internal clearance [μm]
  },
  raceway_geom: {
    r_i: 1.0e9,             // Inner raceway transverse curvature radius [mm] (원통 근사)
    r_o: 1.0e9,             // Outer raceway transverse curvature radius [mm]
    d_uc: 0.0,              // Undercut depth [mm]
    l_uc: 0.0,              // Undercut axial extent [mm]
  },
  roller_profile: {
    // Phase 1: 단순 대칭 logarithmic (Phase 2 에서 정식 profile 재검토)
    crown_type: { Logarithmic: { a_log: 0.0 } },
    delta_c: 5.0,           // Crown drop [μm]
    delta_dub: 0.0,         // Dub-off (양 끝 대칭, 부록 A.1)
    l_dub: 0.0,
    sigma_roller: 0.15,     // Roller surface roughness Ra [μm]
  },
  raceway_profile_inner: {
    delta_rw: 0.0,
    w_a: 0.0,
    ra: 0.15,
    custom_profile: null,
    polynomial_coeffs: null,
  },
  raceway_profile_outer: {
    delta_rw: 0.0,
    w_a: 0.0,
    ra: 0.15,
    custom_profile: null,
    polynomial_coeffs: null,
  },
  material: {
    e_roller: 210.0,        // SUJ2 bearing steel
    e_ring: 210.0,
    nu: 0.3,
    hrc: 61.0,
    density_roller: 7.85,
    density_ring: 7.85,
  },
  operating: {
    f_x: 100.0,             // Radial load X (horizontal) [kN]
    f_y: -500.0,            // Radial load Y (vertical, gravity) [kN] — 풍력 자중 예시
    m_x: 0.0,               // Tilting moment about X (single-plane, D6) [kN·m]
    n_inner_rpm: 500.0,     // Inner ring speed [rpm]
    n_outer_rpm: 0.0,
    gamma: 0.0,             // External misalignment (about X-axis, D6) [arcmin]
    t_op: 60.0,             // Operating temperature [°C]
    nu_40: 68.0,            // ISO VG 68
    nu_100: 8.0,
    alpha_pv: 20.0,
    lubrication_type: 'Oil' as const,
    starvation_factor: 1.0,
    rho_oil: 850.0,
    design_life_hours: 100,
    // Advanced lubrication parameters (Phase 7 재확인 예정)
    lubrication_model: 'Method1_DH' as const,
    film_decay_enabled: false,
    film_decay_time_hours: 0,
    skew_angle_deg: 0,
    replenishment_rate_nm_s: 0,
    surface_finish: 'Standard' as const,
    additive_type: 'None' as const,
    tau_eyring: 5.0,
    z_roelands: 0.67,
    traction_model: 'Eyring' as const,
    carreau_eta_inf_ratio: 0.005,
    carreau_lambda_s: 1.0e-7,
    carreau_n: 0.5,
    carreau_a: 2.0,
    friction_model: 'PalmgrenLike' as const,
    thermal_correction: 'Aihara1987' as const,
    hysteresis_loss_factor: 0.005,
    // skf_trb_series 제거 (Phase 7 에서 SKF CRB 대응 검토)
    skf_lubrication: 'OilBath' as const,
    skf_y_factor: 1.6,
    k_fluid: 0.15,
    beta_visc: 0.04,
    roughness_input_mode: 'Rq' as const,
    rq_inner: 0.3,
    rq_outer: 0.3,
    rq_roller: 0.15,
  },
  solver: {
    run_mode: { Single: 'Gen1' },
    n_slices: 30,
    beam_type: 'Timoshenko',
    convergence_tol: 1e-4,
    max_iterations: 200,
    angular_increment_deg: 2.0,
    life_method: 'Iso16281',
    e_c: 0,
    contamination_level: 'NormalCleanliness' as const,
    oil_supply_method: 'OilBath' as const,
    c_r_kn: 1520.0,         // NU 240 catalogue 근사 [kN]
    c_0r_kn: 2400.0,        // NU 240 basic static load rating [kN]
    f_s_min: 1.0,
    rib_contact_mode: 'PostProcess',  // 사용 안 함 (D1) — 하위호환용 필드
    f_0r: 1.7,              // ISO 15312 CRB (NU 시리즈) 예시값
    f_1r: 0.00025,
    kappa_method: 'ViscosityRatio' as const,
    use_split_contact: true,
  },
};
