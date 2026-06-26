import type { BearingInput } from './types/bearing';

// NSK HR30306J Tapered Roller Bearing (actual internal geometry)
// d=30mm, D=72mm, T=20.75mm, B=19mm, C=16mm
// Cr=59.5kN, C0r=59.8kN, Z=14, mass=0.38kg
// e=0.31, Y2=1.9, Y0=1.05
export const defaultInput: BearingInput = {
  macro_geom: {
    d: 30.0,                // Bore diameter [mm]
    outer_diameter: 72.0,   // Outer diameter [mm]
    t: 20.75,               // Bearing overall width [mm]
    alpha: 11.859,          // Contact angle [deg] (= outer raceway angle αe)
    z: 14,                  // Number of rollers
    d_we_max: 10.9371,      // Roller large-end diameter [mm]
    d_we_min: 10.123273,    // Roller small-end diameter [mm]
    l_we: 11.65,            // Roller effective contact length [mm]
    d_pw: 51.0,             // Pitch circle diameter [mm] ≈ (d+D)/2
    h_rib: 2.5,             // Large-end rib height [mm]
    alpha_rib: 9.855,       // Rib angle [deg] (= 90° - αf, αf=80.145° from axis)
    g_r: 0.0,               // Radial internal clearance [μm] (TRB: preloaded)
    h_c: null,              // Contact height on rib [mm] (null = auto: h_rib/2)
  },
  raceway_geom: {
    alpha_i: 7.85,          // Inner raceway taper angle [deg] (cone)
    alpha_o: 11.859,        // Outer raceway taper angle [deg] (cup)
    r_i: 300.0,             // Inner raceway transverse curvature radius [mm] (flat)
    r_o: 300.0,             // Outer raceway transverse curvature radius [mm] (flat)
    r_rib: 1500.0,          // Large-end rib fillet radius (meridional) [mm]
    r_rib_circ: null,       // Rib circumferential radius [mm] (null = auto: r_contact/sin(α_rib), γ=(αi+αo)/2, r_c=d_pw/2+l_we/2·sinγ−d_we_max/2·cosγ)
    d_uc: 0.0,              // Undercut depth [mm]
    l_uc: 0.0,              // Undercut axial extent [mm]
  },
  roller_profile: {
    // 4th-order polynomial fit of NSK HR30306J measured profile [μm]
    crown_type: { Polynomial: { coeffs: [-0.001713, 0.007566, -0.1307, -0.1991, -0.04019] } },
    delta_c: 0.0,           // Not used for Polynomial type
    delta_dub_l: 0.0,       // Dub-off disabled (polynomial includes edge shape)
    delta_dub_s: 0.0,
    l_dub_l: 0.0,
    l_dub_s: 0.0,
    r_sph: 35.0,            // Roller large-end sphere radius [mm]
    sigma_roller: 0.15,     // Roller surface roughness Ra [μm]
  },
  raceway_profile_inner: {
    delta_rw: 0.0,          // Raceway crowning [μm] (flat for TRB)
    w_a: 0.0,               // Axial waviness [μm]
    ra: 0.15,               // Surface roughness Ra [μm] (HR series: fine finish)
    custom_profile: null,
    // 4th-order polynomial fit of inner raceway measured profile [μm]
    polynomial_coeffs: [-0.01255, -0.01808, -0.01308, 0.1398, -0.2076],
  },
  raceway_profile_outer: {
    delta_rw: 0.0,
    w_a: 0.0,
    ra: 0.15,
    custom_profile: null,
    // 4th-order polynomial fit of outer raceway measured profile [μm]
    polynomial_coeffs: [-0.0006185, 0.001334, -0.2418, -0.08751, -0.1606],
  },
  material: {
    e_roller: 210.0,        // Young's modulus [GPa] (SUJ2 bearing steel)
    e_ring: 210.0,          // Young's modulus [GPa]
    nu: 0.3,                // Poisson's ratio
    hrc: 61.0,              // Surface hardness [HRC] (NSK HR: high hardness)
    density_roller: 7.85,   // Roller density [g/cm³] (bearing steel)
    density_ring: 7.85,     // Ring density [g/cm³] (bearing steel)
  },
  operating: {
    f_x: 5.0,               // Radial load X [kN]
    f_y: 0.0,               // Radial load Y [kN]
    f_a: 2.0,               // Axial load [kN]
    m_x: 0.0,               // Tilting moment X [kN·m]
    m_y: 0.0,               // Tilting moment Y [kN·m]
    n_inner_rpm: 1500.0,    // Inner ring speed [rpm]
    n_outer_rpm: 0.0,       // Outer ring speed [rpm]
    gamma: 0.0,             // Misalignment [arcmin]
    t_op: 70.0,             // Operating temperature [°C]
    nu_40: 68.0,            // Kinematic viscosity at 40°C [mm²/s] (ISO VG 68)
    nu_100: 8.0,            // Kinematic viscosity at 100°C [mm²/s]
    alpha_pv: 20.0,         // Pressure-viscosity coefficient [1/GPa] (mineral oil)
    lubrication_type: 'Oil' as const,
    starvation_factor: 1.0, // φ_s: 1.0=fully flooded, 0.5-0.8=grease typical
    rho_oil: 850.0,         // Lubricant density [kg/m³] (mineral oil)
    preload_mode: 'DisplacementFromForce' as const,
    delta_preload_um: 0.0,
    design_life_hours: 100,         // Design life [hours]
    // Advanced lubrication model parameters
    lubrication_model: 'Method1_DH' as const,
    film_decay_enabled: false,
    film_decay_time_hours: 0,
    skew_angle_deg: 0,
    replenishment_rate_nm_s: 0,
    surface_finish: 'Standard' as const,
    additive_type: 'None' as const,
    tau_eyring: 5.0,        // Eyring stress [MPa] (mineral oil)
    z_roelands: 0.67,       // Roelands exponent (mineral oil)
    traction_model: 'Eyring' as const,    // Eyring | CarreauYasuda
    carreau_eta_inf_ratio: 0.005,         // η_∞ / η_0
    carreau_lambda_s: 1.0e-7,             // Carreau-Yasuda relaxation time [s]
    carreau_n: 0.5,                       // power-law exponent
    carreau_a: 2.0,                       // Yasuda transition exponent (a=2 = original Carreau)
    friction_model: 'PalmgrenLike' as const,
    thermal_correction: 'Aihara1987' as const,
    hysteresis_loss_factor: 0.005,
    skf_trb_series: 'Series303' as const,
    skf_lubrication: 'OilBath' as const,
    skf_y_factor: 1.6,
    k_fluid: 0.15,          // Thermal conductivity [W/(m·K)]
    beta_visc: 0.04,        // Viscosity-temp coefficient [1/K]
    roughness_input_mode: 'Rq' as const,  // Ra or Rq input mode
    rq_inner: 0.3,          // Inner raceway roughness [μm]
    rq_outer: 0.3,          // Outer raceway roughness [μm]
    rq_roller: 0.15,        // Roller roughness [μm]
  },
  solver: {
    run_mode: { Single: 'Gen1' },
    n_slices: 30,
    beam_type: 'Timoshenko',
    convergence_tol: 1e-4,
    max_iterations: 200,
    angular_increment_deg: 2.0,
    life_method: 'Iso16281',
    e_c: 0,                 // Contamination factor (0=auto ISO 281 Annex A, >0=manual)
    contamination_level: 'NormalCleanliness' as const,  // ISO 4406 cleanliness
    oil_supply_method: 'OilBath' as const,               // Oil supply for e_C table
    c_r_kn: 59.5,           // NSK HR30306J dynamic load rating [kN]
    c_0r_kn: 59.8,              // NSK HR30306J basic static load rating [kN]
    f_s_min: 1.0,               // Minimum static safety factor S0
    rib_contact_mode: 'PostProcess',  // PostProcess (default) or Coupled (stiffness in equilibrium)
    f_0r: 3.0,                  // ISO 15312 f₀ᵣ (TRB dim series 02: 3.0)
    f_1r: 0.0004,               // ISO 15312 f₁ᵣ (all TRB series: 0.0004)
    kappa_method: 'ViscosityRatio' as const,  // κ=ν/ν₁ (default) or Λ^1.3
    use_split_contact: true,                    // Independent inner/outer q_k (default ON)
  },
};
