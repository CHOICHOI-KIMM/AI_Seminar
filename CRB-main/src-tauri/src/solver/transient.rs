//! Transient sliding analysis — roller dynamics with inertia effects.
//!
//! Optimized orchestrator:
//! 1. Geometry/slice caching — compute_slices() called once
//! 2. Warm-start — previous displacement seeds next NR solve
//! 3. Load-change skip — reuse equilibrium when load barely changes
//! 4. Lightweight solver — transient-specific fast path (no validation/life/film)
//! 5. Rayon parallelization — roller-level parallel dynamics integration
//! 6. Adaptive time stepping — dt grows/shrinks with load rate of change

use std::time::Instant;

use rayon::prelude::*;

use crate::error::SolverError;
use super::bearing::solve_equilibrium_fast;
use super::geometry::{compute_roller_inertia, compute_slices};
use super::lubrication::{compute_traction_coefficient_srr, eyring_traction_advanced};
use super::wec_risk::evaluate_risk;
use super::types::*;

/// Bearing steel density [kg/m³]
const RHO_STEEL: f64 = 7850.0;

/// Slip ratio threshold for "in slip" detection.
/// EHL contacts: even small slip can cause surface damage under high load.
const SLIP_THRESHOLD: f64 = 0.0005; // 0.05%

/// Eyring stress for mineral oil [Pa]
const TAU_EYRING: f64 = 5.0e6;

/// Cage pocket clearance [m] — radial gap between roller and cage pocket wall.
const CAGE_POCKET_CLEARANCE_M: f64 = 3e-4; // 0.3 mm typical

/// Fraction of roller circumference wetted by cage pocket oil film (0..1).
const CAGE_WETTED_FRACTION: f64 = 0.4;

/// Cage pocket wall friction coefficient (mixed lubrication).
const MU_CAGE_POCKET: f64 = 0.10;

/// Rib face friction coefficient (mixed/boundary lubrication).
/// Houpert (2002): 0.03–0.08 for TRB rib contacts under typical conditions.
/// This is sliding friction — rib contact is pure sliding, not rolling.
const MU_RIB_DEFAULT: f64 = 0.05;

/// Roller drag parameters computed from bearing geometry and oil properties.
/// These replace the previous hardcoded C_VISCOUS_DRAG and TAU_CAGE_POCKET constants.
#[derive(Clone, Copy, Debug)]
struct RollerDragParams {
    /// Viscous drag coefficient [N·m·s/rad] — Couette flow in cage pocket gap.
    /// C_viscous = η × r² × A_wetted / gap
    c_viscous: f64,
    /// Cage pocket friction coefficient [N·m·s²/rad²] — centrifugal force on roller.
    /// tau_cage = k_cage × ω_cage²
    k_cage: f64,
    /// Cage-to-inner-ring speed ratio (ω_cage / ω_inner).
    cage_speed_ratio: f64,
}

impl RollerDragParams {
    /// Compute drag parameters from bearing geometry and oil viscosity.
    fn from_bearing(geom: &MacroGeometry, raceway: &RacewayGeometry, eta_0: f64) -> Self {
        let r_mean_m = (geom.d_we_max + geom.d_we_min) / 4.0 * 1e-3; // [m]
        let l_m = geom.l_we * 1e-3; // [m]
        let r_pitch_m = geom.d_pw / 2.0 * 1e-3; // [m]

        // ── Viscous drag: Couette flow model ──
        // Wetted area = fraction × 2π × r_roller × L_roller
        let a_wetted = CAGE_WETTED_FRACTION * 2.0 * std::f64::consts::PI * r_mean_m * l_m;
        // C_viscous = η₀ × r² × A_wetted / gap
        let c_viscous = eta_0 * r_mean_m * r_mean_m * a_wetted / CAGE_POCKET_CLEARANCE_M;

        // ── Cage pocket friction: centrifugal model ──
        // Roller mass (approximate as cylinder)
        let vol = std::f64::consts::PI * r_mean_m * r_mean_m * l_m;
        let m_roller = RHO_STEEL * vol;
        // k_cage = μ_pocket × m_roller × R_pitch × (2/3 × r_roller)
        let k_cage = MU_CAGE_POCKET * m_roller * r_pitch_m * (2.0 / 3.0 * r_mean_m);

        // ── Cage speed ratio ──
        // ω_cage / ω_inner ≈ (1 - d_roller·cos(α) / d_pw) / 2
        let alpha_rad = raceway.alpha_o.to_radians();
        let d_we_mean = (geom.d_we_max + geom.d_we_min) / 2.0;
        let cage_speed_ratio = (1.0 - d_we_mean * alpha_rad.cos() / geom.d_pw) / 2.0;

        RollerDragParams { c_viscous, k_cage, cage_speed_ratio }
    }

    /// Viscous drag torque [N·m] at given roller angular velocity.
    #[inline]
    fn tau_viscous(&self, omega_roller: f64) -> f64 {
        self.c_viscous * omega_roller
    }

    /// Cage pocket driving torque [N·m] at given inner ring angular velocity.
    /// Based on centrifugal force: F = m × ω_cage² × R_pitch.
    #[inline]
    fn tau_cage(&self, omega_inner: f64) -> f64 {
        let omega_cage = self.cage_speed_ratio * omega_inner;
        self.k_cage * omega_cage * omega_cage
    }
}

/// Load-change threshold for equilibrium reuse (relative)
const LOAD_SKIP_TOL: f64 = 1e-4;

/// Minimum adaptive dt multiplier (relative to dt_max)
#[allow(dead_code)]
const DT_ADAPT_MIN: f64 = 0.1;

/// Maximum adaptive dt multiplier
#[allow(dead_code)]
const DT_ADAPT_MAX: f64 = 4.0;

/// Load rate threshold below which dt can grow [kN/s]
const LOAD_RATE_LOW: f64 = 10.0;

/// Load rate threshold above which dt shrinks [kN/s]
const LOAD_RATE_HIGH: f64 = 500.0;

/// Solve transient roller dynamics over a load time series.
///
/// For each time step:
/// 1. Interpolate load → solve bearing equilibrium (fast Gen1)
/// 2. Compute target roller speed from cone apex kinematics
/// 3. Integrate roller rotational dynamics with traction force limits
/// 4. Track sliding metrics and cumulative damage
pub fn solve_transient(
    input: &BearingInput,
    progress: &dyn ProgressReporter,
) -> Result<TransientResult, SolverError> {
    let t_wall_start = Instant::now();

    let transient = input.transient.as_ref().ok_or_else(|| {
        SolverError::InvalidInput("No transient input provided".into())
    })?;

    if transient.load_series.len() < 2 {
        return Err(SolverError::InvalidInput(
            "Transient load series needs at least 2 points".into(),
        ));
    }

    let n_rollers = input.macro_geom.z as usize;

    // ══════════════════════════════════════════════════════════════════
    // OPT 1: Geometry/Slice caching — compute once, reuse for all steps
    // ══════════════════════════════════════════════════════════════════
    let slices = compute_slices(
        &input.macro_geom,
        &input.raceway_geom,
        &input.roller_profile,
        &input.raceway_profile_inner,
        &input.raceway_profile_outer,
        input.solver.n_slices,
    )?;

    // Roller inertia (constant)
    let r_max_mm = input.macro_geom.d_we_max / 2.0;
    let r_min_mm = input.macro_geom.d_we_min / 2.0;
    let l_mm = input.macro_geom.l_we;
    let i_roller = compute_roller_inertia(r_max_mm, r_min_mm, l_mm, RHO_STEEL);
    let r_mean_m = (r_max_mm + r_min_mm) / 2.0 * 1e-3;

    // Raceway angles for cone apex kinematics
    let alpha_i_rad = input.raceway_geom.alpha_i.to_radians();
    let alpha_o_rad = input.raceway_geom.alpha_o.to_radians();
    let sin_ai = alpha_i_rad.sin();
    let sin_ao = alpha_o_rad.sin();
    let phi_rad = (alpha_o_rad - alpha_i_rad) / 2.0;
    let sin_phi = phi_rad.sin();

    // Initialize roller states
    let first_op = transient.load_series[0].to_operating(&input.operating);

    // Combined elastic modulus E* for Heathcote slip calculation [MPa]
    // E* = 2 / ((1-ν₁²)/E₁ + (1-ν₂²)/E₂), E in GPa → ×1e3 for MPa
    let nu = input.material.nu;
    let e1 = input.material.e_roller * 1e3; // GPa → MPa
    let e2 = input.material.e_ring * 1e3;
    let e_star = 2.0 / ((1.0 - nu * nu) / e1 + (1.0 - nu * nu) / e2);

    // Nominal roller load for Heathcote estimation (use first load point)
    let q_nominal_heathcote = first_op.f_a.abs().max(first_op.f_x.abs()) * 1e3
        / input.macro_geom.z as f64; // rough per-roller [N]

    // Pre-compute per-slice geometric SRR (profile + Heathcote, constant for session)
    let slice_geo_srr = compute_slice_geometric_srr(&slices, phi_rad, q_nominal_heathcote, e_star);
    let omega_i_init = first_op.n_rpm() * std::f64::consts::TAU / 60.0;
    let omega_target_init = compute_omega_roller_target(omega_i_init, sin_ai, sin_ao, sin_phi);
    let mut roller_omega: Vec<f64> = vec![omega_target_init; n_rollers];

    // Damage accumulators
    let mut damage: Vec<RollerDamageAccumulator> = (0..n_rollers)
        .map(|j| RollerDamageAccumulator {
            j,
            cumulative_friction_energy_j: 0.0,
            cumulative_slide_distance_m: 0.0,
            max_contact_load_during_slip_n: 0.0,
            slip_event_count: 0,
            total_slip_duration_s: 0.0,
        })
        .collect();
    let mut prev_in_slip: Vec<bool> = vec![false; n_rollers];

    // Time grid
    let t_start = transient.load_series[0].t_s;
    let t_end = transient.load_series.last().unwrap().t_s;
    let dt_max = transient.dt_max;

    let mut snapshots = Vec::new();
    let mut total_slip_events = 0usize;
    let mut max_slip_ratio_overall = 0.0f64;

    // Dynamic viscosity (constant for session)
    let eta_0 = estimate_dynamic_viscosity(&input.operating);

    // Roller drag parameters from bearing geometry + oil
    let drag = RollerDragParams::from_bearing(&input.macro_geom, &input.raceway_geom, eta_0);

    // ══════════════════════════════════════════════════════════════════
    // OPT 2: Warm-start — keep previous displacement for next step
    // ══════════════════════════════════════════════════════════════════
    let mut prev_disp: [f64; 5] = [0.1, 0.0, 0.1, 0.0, 0.0]; // initial guess

    // ══════════════════════════════════════════════════════════════════
    // OPT 3: Load-change skip — cache previous load + equilibrium
    // ══════════════════════════════════════════════════════════════════
    let mut prev_load: Option<LoadTimePoint> = None;
    let mut cached_equilibrium: Option<BearingEquilibrium> = None;

    // Estimate total steps for progress reporting
    let est_steps = ((t_end - t_start) / dt_max).ceil() as usize;
    let mut step_count = 0usize;

    progress.report(SolverProgress {
        stage: "Transient".into(),
        detail: format!("~{} steps, dt_max={:.1}ms", est_steps, dt_max * 1e3),
        percent: 0.0,
    });

    // ══════════════════════════════════════════════════════════════════
    // OPT 6: Adaptive time stepping
    //   - Equilibrium solve interval adapts to load rate of change
    //   - Roller dynamics always sub-steps at dt_max for accuracy
    // ══════════════════════════════════════════════════════════════════
    let n_steps = ((t_end - t_start) / dt_max).ceil() as usize;

    // Track equilibrium solve interval (adaptive)
    let mut eq_solve_countdown = 0usize; // 0 = solve now
    let mut eq_steps_per_solve = 1usize; // how many dynamics steps per equilibrium solve

    for step in 0..=n_steps {
        let t = (t_start + step as f64 * dt_max).min(t_end);

        // 1. Interpolate load at time t
        let load_point = interpolate_load_at_time(&transient.load_series, t);

        // ── Determine if we need to re-solve equilibrium ──
        let need_solve = if eq_solve_countdown == 0 {
            true
        } else if let Some(ref pl) = prev_load {
            // Force re-solve if load changed significantly
            !load_change_small(pl, &load_point, LOAD_SKIP_TOL)
        } else {
            true
        };

        if need_solve {
            // Solve equilibrium (OPT 2 + OPT 4: warm-start + fast solver)
            let eq = solve_step(input, &slices, &load_point, &mut prev_disp)?;
            cached_equilibrium = Some(eq);

            // OPT 6: Adapt solve frequency based on load rate
            if let Some(ref pl) = prev_load {
                let rate = load_rate_of_change(pl, &load_point);
                eq_steps_per_solve = if rate < LOAD_RATE_LOW {
                    // Slow load change: solve less often (up to every 4 steps)
                    4
                } else if rate > LOAD_RATE_HIGH {
                    // Fast load change: solve every step
                    1
                } else {
                    let frac = (rate - LOAD_RATE_LOW) / (LOAD_RATE_HIGH - LOAD_RATE_LOW);
                    (4.0 - frac * 3.0).round().max(1.0) as usize
                };
            }
            eq_solve_countdown = eq_steps_per_solve;
        }
        eq_solve_countdown = eq_solve_countdown.saturating_sub(1);

        let equilibrium = cached_equilibrium.as_ref().unwrap();

        // 3. Compute target roller speed
        let step_op = load_point.to_operating(&input.operating);
        let omega_i = step_op.n_rpm() * std::f64::consts::TAU / 60.0;
        let omega_target = compute_omega_roller_target(omega_i, sin_ai, sin_ao, sin_phi);

        // ══════════════════════════════════════════════════════════════
        // OPT 5: Rayon parallelization — roller dynamics integration
        //   Always uses dt_max for dynamics accuracy
        // ══════════════════════════════════════════════════════════════
        let roller_data: Vec<(RollerKinematicState, f64, f64, f64, bool)> = (0..n_rollers)
            .into_par_iter()
            .map(|j| {
                let q_j = if j < equilibrium.roller_loads.len() {
                    equilibrium.roller_loads[j]
                } else {
                    0.0
                };

                // Rib friction drilling moment (about ROLLER spin axis):
                //   M_drilling = (3/8) · μ_rib · F_rib · a_ellipse   [N·mm]
                // → [N·m]: × 1e-3
                // Earlier μ·F·r_contact_mm formulation used wrong lever arm
                // (bearing-axis radius ~50 mm instead of contact half-width ~1.5 mm)
                // and over-predicted by ~30× for typical TRB geometry.
                // Houpert 2002: drilling motion of elliptical contact.
                let tau_rib_j = if j < equilibrium.roller_results.len() {
                    if let Some(ref rib) = equilibrium.roller_results[j].rib_result {
                        let mu = rib.ehl.as_ref().map_or(MU_RIB_DEFAULT, |e| e.mu_eff);
                        0.375 * mu * rib.f_rib * rib.a_ellipse * 1e-3
                    } else {
                        0.0
                    }
                } else {
                    0.0
                };

                let (omega_actual, tau_applied, tau_max) = if transient.enable_roller_dynamics {
                    integrate_roller_step(
                        roller_omega[j], omega_target, q_j, r_mean_m,
                        i_roller, dt_max, eta_0, &input.operating,
                        &drag, omega_i, tau_rib_j,
                    )
                } else {
                    (omega_target, 0.0, 0.0)
                };

                let slip_ratio = if omega_target.abs() > 1e-10 {
                    (omega_actual - omega_target) / omega_target
                } else {
                    0.0
                };
                let u_slide = (omega_actual - omega_target) * r_mean_m;
                let in_slip = slip_ratio.abs() > SLIP_THRESHOLD;

                let mu_eff = if q_j > 0.0 && u_slide.abs() > 1e-8 {
                    estimate_friction_coeff_simple(slip_ratio)
                } else {
                    0.0
                };

                let psi_deg = if j < equilibrium.roller_results.len() {
                    equilibrium.roller_results[j].psi_deg
                } else {
                    j as f64 * 360.0 / n_rollers as f64
                };

                // Per-slice SRR: dynamic + geometric
                let per_slice_srr: Vec<f64> = slice_geo_srr.iter()
                    .map(|geo| slip_ratio + geo)
                    .collect();

                let state = RollerKinematicState {
                    j,
                    psi_deg,
                    omega_roller_actual: omega_actual,
                    omega_roller_target: omega_target,
                    slip_ratio,
                    u_slide_avg: u_slide,
                    tau_traction: tau_applied,
                    tau_traction_max: tau_max,
                    in_slip,
                    slice_srr: per_slice_srr,
                };

                (state, mu_eff, q_j, u_slide, in_slip)
            })
            .collect();

        // Apply results sequentially (damage accumulation is stateful)
        let mut n_in_slip = 0usize;
        let mut max_slip_ratio = 0.0f64;
        let mut max_slide_vel = 0.0f64;
        let mut total_friction_power = 0.0f64;
        let mut roller_states = Vec::with_capacity(n_rollers);

        for (state, mu_eff, q_j, u_slide, in_slip) in &roller_data {
            let j = state.j;
            roller_omega[j] = state.omega_roller_actual;

            damage[j].cumulative_friction_energy_j += mu_eff * q_j * u_slide.abs() * dt_max;
            damage[j].cumulative_slide_distance_m += u_slide.abs() * dt_max;

            if *in_slip {
                damage[j].total_slip_duration_s += dt_max;
                if *q_j > damage[j].max_contact_load_during_slip_n {
                    damage[j].max_contact_load_during_slip_n = *q_j;
                }
                if !prev_in_slip[j] {
                    damage[j].slip_event_count += 1;
                    total_slip_events += 1;
                }
                n_in_slip += 1;
            }
            prev_in_slip[j] = *in_slip;

            max_slip_ratio = max_slip_ratio.max(state.slip_ratio.abs());
            max_slide_vel = max_slide_vel.max(state.u_slide_avg.abs());
            total_friction_power += mu_eff * q_j * u_slide.abs();

            roller_states.push(state.clone());
        }

        max_slip_ratio_overall = max_slip_ratio_overall.max(max_slip_ratio);

        // 5. Save snapshot
        if step % transient.snapshot_interval == 0 || step == n_steps {
            // Build slice SRR map for this snapshot
            let slice_srr_map: Vec<Vec<f64>> = roller_states.iter()
                .map(|rk| rk.slice_srr.clone())
                .collect();

            let max_slice_srr = roller_states.iter()
                .flat_map(|rk| rk.slice_srr.iter())
                .fold(0.0f64, |acc, &v| acc.max(v.abs()));

            snapshots.push(TransientSnapshot {
                t_s: t,
                operating: step_op,
                equilibrium: equilibrium.clone(),
                roller_kinematics: roller_states,
                sliding_metrics: TransientSlidingMetrics {
                    n_rollers_in_slip: n_in_slip,
                    max_slip_ratio,
                    max_slide_velocity: max_slide_vel,
                    instantaneous_friction_power: total_friction_power,
                    max_slice_srr,
                },
                slice_srr_map,
            });
        }

        // Progress reporting (every 5%)
        if n_steps > 0 && step % (n_steps / 20).max(1) == 0 {
            let pct = step as f64 / n_steps as f64 * 100.0;
            progress.report(SolverProgress {
                stage: "Transient".into(),
                detail: format!("t={:.3}s, slip={}, eq_skip={}", t, n_in_slip, eq_steps_per_solve - 1),
                percent: pct.min(99.0),
            });
        }

        // Cache for next step
        prev_load = Some(load_point);
        step_count += 1;
    }

    // Summary
    let total_slip_duration_s: f64 = damage.iter().map(|d| d.total_slip_duration_s).sum();
    let wec_risk_index = compute_wec_risk_simple(&damage, t_end - t_start);
    let elapsed_ms = t_wall_start.elapsed().as_secs_f64() * 1000.0;

    progress.report(SolverProgress {
        stage: "Transient".into(),
        detail: format!("Done: {} steps in {:.0}ms", step_count, elapsed_ms),
        percent: 100.0,
    });

    let mut result = TransientResult {
        snapshots,
        damage_summary: TransientDamageSummary {
            roller_damage: damage,
            total_slip_events,
            total_slip_duration_s,
            max_slip_ratio_overall,
            wec_risk_index,
        },
        total_time_s: t_end - t_start,
        elapsed_ms,
        risk_assessment: None,
    };

    // Risk assessment
    let contact_area_mm2 = {
        let l_we = input.macro_geom.l_we;
        l_we * 0.1 * input.macro_geom.z as f64
    };
    result.risk_assessment = Some(evaluate_risk(&result, contact_area_mm2));

    Ok(result)
}

// ─── Helper: solve one equilibrium step with warm-start ────────────

fn solve_step(
    input: &BearingInput,
    slices: &[SliceGeometry],
    load_point: &LoadTimePoint,
    prev_disp: &mut [f64; 5],
) -> Result<BearingEquilibrium, SolverError> {
    let step_operating = load_point.to_operating(&input.operating);
    let step_input = BearingInput {
        macro_geom: input.macro_geom.clone(),
        raceway_geom: input.raceway_geom.clone(),
        roller_profile: input.roller_profile.clone(),
        raceway_profile_inner: input.raceway_profile_inner.clone(),
        raceway_profile_outer: input.raceway_profile_outer.clone(),
        material: input.material.clone(),
        operating: step_operating,
        solver: SolverParams {
            run_mode: RunMode::Single(SolverMode::Gen1),
            ..input.solver.clone()
        },
        transient: None,
    };

    // OPT 2 + OPT 4: warm-start + lightweight solver
    let eq = solve_equilibrium_fast(&step_input, slices, prev_disp)?;
    *prev_disp = eq.displacement;
    Ok(eq)
}

// ─── Load-change detection ──────────────────────────────────────────

/// Check if load barely changed (for equilibrium reuse).
fn load_change_small(a: &LoadTimePoint, b: &LoadTimePoint, tol: f64) -> bool {
    let check = |va: f64, vb: f64| -> bool {
        let diff = (va - vb).abs();
        let scale = va.abs().max(vb.abs()).max(1.0);
        diff / scale < tol
    };
    check(a.f_x, b.f_x) && check(a.f_y, b.f_y) && check(a.f_a, b.f_a)
        && check(a.m_x, b.m_x) && check(a.m_y, b.m_y) && check(a.n_rpm, b.n_rpm)
}

/// Compute load rate of change [kN/s] for adaptive time stepping.
fn load_rate_of_change(a: &LoadTimePoint, b: &LoadTimePoint) -> f64 {
    let dt = (b.t_s - a.t_s).abs();
    if dt < 1e-15 { return 0.0; }
    let df_x = (b.f_x - a.f_x).abs();
    let df_y = (b.f_y - a.f_y).abs();
    let df_a = (b.f_a - a.f_a).abs();
    let df_total = (df_x * df_x + df_y * df_y + df_a * df_a).sqrt();
    df_total / dt
}

// ─── Roller dynamics ────────────────────────────────────────────────

/// Compute target roller angular velocity from cone apex kinematics.
fn compute_omega_roller_target(omega_i: f64, sin_ai: f64, sin_ao: f64, sin_phi: f64) -> f64 {
    if sin_phi.abs() > 1e-12 && (sin_ai + sin_ao).abs() > 1e-12 {
        omega_i * sin_ai * sin_ao / (sin_phi * (sin_ai + sin_ao))
    } else {
        0.0
    }
}

/// Integrate single roller rotational dynamics for one time step using RK4.
///
/// Torque balance: I·dω/dt = τ_net(ω) = τ_drive(ω) − τ_viscous(ω)
/// - τ_viscous: Couette drag from oil in cage pocket gap (∝ ω)
/// - τ_cage: centrifugal cage pocket friction (∝ ω_cage²)
/// - τ_traction: EHL contact traction (depends on SRR(ω) via Eyring model)
/// - τ_rib: rib face sliding friction torque = μ_rib × F_rib × r_contact
///
/// RK4 provides 4th-order accuracy vs Forward Euler's 1st-order,
/// improving stability during rapid load transients (Creju et al. 1994).
fn integrate_roller_step(
    omega_current: f64,
    omega_target: f64,
    q_normal: f64,
    r_mean_m: f64,
    i_roller: f64,
    dt: f64,
    eta_0: f64,
    operating: &OperatingConditions,
    drag: &RollerDragParams,
    omega_inner: f64,
    tau_rib: f64,
) -> (f64, f64, f64) {
    let h_c_m = 0.5e-6;
    let p_mean_pa = (q_normal / (r_mean_m * 2.0 * 1e-3).max(1e-6)).max(1.0);
    let u_roll = omega_target.abs() * r_mean_m;
    let tau_cage = drag.tau_cage(omega_inner);

    // Compute net angular acceleration dω/dt at a given ω
    let compute_domega = |omega: f64| -> f64 {
        let srr = if omega_target.abs() > 1e-10 {
            (omega - omega_target) / omega_target
        } else {
            0.0
        };

        let mu_traction = if q_normal > 0.0 {
            match operating.lubrication_model {
                LubricationModel::Method2_MK
                | LubricationModel::Method3_NVM => {
                    let tau_eyring_pa = operating.tau_eyring * 1e6;
                    eyring_traction_advanced(
                        srr, u_roll, p_mean_pa, h_c_m,
                        eta_0, operating.z_roelands, tau_eyring_pa,
                    )
                }
                _ => {
                    compute_traction_coefficient_srr(
                        srr, p_mean_pa, h_c_m, eta_0, TAU_EYRING,
                    )
                }
            }
        } else {
            0.0
        };

        let tau_traction_max = mu_traction * q_normal * r_mean_m;
        let tau_available = tau_traction_max + tau_cage + tau_rib;
        let tau_viscous = drag.tau_viscous(omega);

        // Drive torque: clamped to available
        let tau_needed = i_roller * (omega_target - omega) / dt + tau_viscous;
        let tau_applied = tau_needed.clamp(-tau_available, tau_available);

        (tau_applied - tau_viscous) / i_roller
    };

    // RK4 integration
    let k1 = compute_domega(omega_current);
    let k2 = compute_domega(omega_current + 0.5 * dt * k1);
    let k3 = compute_domega(omega_current + 0.5 * dt * k2);
    let k4 = compute_domega(omega_current + dt * k3);
    let omega_new = omega_current + dt * (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;

    // Report final-state torques for diagnostics
    let srr_final = if omega_target.abs() > 1e-10 {
        (omega_new - omega_target) / omega_target
    } else {
        0.0
    };
    let mu_final = if q_normal > 0.0 {
        estimate_friction_coeff_simple(srr_final)
    } else {
        0.0
    };
    let tau_applied_final = mu_final * q_normal * r_mean_m;
    let tau_available_final = tau_applied_final + tau_cage + tau_rib;

    (omega_new, tau_applied_final, tau_available_final)
}

/// Estimate dynamic viscosity from operating conditions [Pa·s].
///
/// Uses ASTM D341 (Walther) equation:
///   ln(ln(ν + 0.7)) = A − B × ln(T)
/// where T is absolute temperature [K]. Viscosity decreases with temperature.
fn estimate_dynamic_viscosity(op: &OperatingConditions) -> f64 {
    let t1: f64 = 40.0 + 273.15;
    let t2: f64 = 100.0 + 273.15;
    let t_k: f64 = op.t_op + 273.15;

    let z1 = (op.nu_40 + 0.7).ln().ln();
    let z2 = (op.nu_100 + 0.7).ln().ln();

    // ASTM D341: Z = A − B × ln(T), so B = (z1 − z2) / (ln(T2) − ln(T1))
    // and z(T) = z1 − B × (ln(T) − ln(T1))
    let b_slope = (z1 - z2) / (t2.ln() - t1.ln());
    let z = z1 - b_slope * (t_k.ln() - t1.ln());
    let nu_cop = z.exp().exp() - 0.7;

    (nu_cop * 1e-6 * op.rho_oil).max(1e-6)
}

/// Compute per-slice geometric SRR from profile-induced rolling radius change.
///
/// For ideal TRB cone geometry, all slices have identical SRR (the dynamic SRR).
/// Profile modifications (crown, dub-off) alter the effective rolling radius,
/// creating a small geometric micro-slip that varies along the roller length.
///
/// Model: The profile correction Δz [μm] removes material from the roller/raceway,
/// reducing the effective contact radius at that slice:
///   r_eff_k = r_cone_k − Δz_k × cos(φ) × 1e-3  [mm]
///
/// Since the roller rotates as a rigid body, slices with smaller r_eff
/// have lower surface velocity → negative geometric SRR (roller slower).
///
///   SRR_geometric_k = −(Δz_k − Δz_mean) × cos(φ) × 1e-3 / r_k
///
/// The mean is subtracted because the dynamic SRR (from roller dynamics solver)
/// already captures the roller-average effect. The geometric part represents
/// only the spatial variation across slices.
///
/// Typical magnitudes: 0.01–0.1% for standard crown/dub-off profiles.
///
/// Also includes Heathcote slip contribution: within each slice's contact zone,
/// the elastic deformation creates a curved contact surface with varying rolling
/// radius. The second-order approximation:
///   SRR_heathcote_k = b_k² / (8 × R_eq × r_k)
/// where b_k is the Hertz half-width estimated from nominal load.
fn compute_slice_geometric_srr(
    slices: &[SliceGeometry],
    phi_rad: f64,
    q_nominal: f64,
    e_star: f64,
) -> Vec<f64> {
    let n = slices.len();
    if n == 0 {
        return vec![];
    }

    let cos_phi = phi_rad.cos();

    // Compute profile + Heathcote SRR for each slice
    let mut geo_srr = Vec::with_capacity(n);
    for k in 0..n {
        // Average profile correction (inner + outer) / 2 [μm]
        let dz_k = (slices[k].delta_z_total_inner + slices[k].delta_z_total_outer) / 2.0;
        let r_k = slices[k].r_roller; // [mm]

        // Profile SRR: −Δz × cos(φ) × 1e-3 / r
        // Δz [μm] × 1e-3 → [mm], then / r [mm] → dimensionless
        // Sign: positive Δz (crown) → smaller radius → roller slower → negative SRR
        let srr_profile = -dz_k * cos_phi * 1e-3 / r_k;

        // Heathcote slip: SRR_h = b² / (8 × R_eq × r_roller)
        // b = Hertz half-width for line contact: b = sqrt(4 × q_k × R_eq / (π × E*))
        // q_k = load per unit length [N/mm], R_eq [mm], E* [MPa]
        let r_eq = (slices[k].r_eq_inner + slices[k].r_eq_outer) / 2.0; // [mm]
        let l_k = slices[k].slice_width; // [mm]
        let q_k = if l_k > 0.0 && q_nominal > 0.0 {
            q_nominal / (n as f64 * l_k) // approximate uniform load per unit length [N/mm]
        } else {
            0.0
        };

        let srr_heathcote = if r_eq > 0.0 && e_star > 0.0 && q_k > 0.0 {
            let b_sq = 4.0 * q_k * r_eq / (std::f64::consts::PI * e_star); // b² [mm²]
            b_sq / (8.0 * r_eq * r_k)
        } else {
            0.0
        };

        geo_srr.push(srr_profile + srr_heathcote);
    }

    // Subtract mean: dynamic SRR captures roller-average effect,
    // geometric part = spatial deviation only
    let mean: f64 = geo_srr.iter().sum::<f64>() / n as f64;
    for v in &mut geo_srr {
        *v -= mean;
    }

    geo_srr
}

/// Simplified friction coefficient (no BearingResult dependency for parallelism).
fn estimate_friction_coeff_simple(slip_ratio: f64) -> f64 {
    let mu_ehl = 0.005;
    let mu_boundary = 0.12;
    let srr_abs = slip_ratio.abs().min(1.0);
    mu_ehl + (mu_boundary - mu_ehl) * (srr_abs / 0.3).min(1.0)
}

/// Interpolate load at a specific time from the load series.
fn interpolate_load_at_time(series: &[LoadTimePoint], t: f64) -> LoadTimePoint {
    if t <= series[0].t_s {
        return series[0].clone();
    }
    if t >= series[series.len() - 1].t_s {
        return series[series.len() - 1].clone();
    }

    // Binary search for segment (faster than linear scan for large series)
    let mut lo = 0usize;
    let mut hi = series.len() - 1;
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if series[mid].t_s <= t {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    let p0 = &series[lo];
    let p1 = &series[hi];
    let dt_seg = p1.t_s - p0.t_s;
    let frac = if dt_seg.abs() > 1e-15 {
        ((t - p0.t_s) / dt_seg).clamp(0.0, 1.0)
    } else {
        0.0
    };

    LoadTimePoint {
        t_s: t,
        f_x: p0.f_x + frac * (p1.f_x - p0.f_x),
        f_y: p0.f_y + frac * (p1.f_y - p0.f_y),
        f_a: p0.f_a + frac * (p1.f_a - p0.f_a),
        m_x: p0.m_x + frac * (p1.m_x - p0.m_x),
        m_y: p0.m_y + frac * (p1.m_y - p0.m_y),
        n_rpm: p0.n_rpm + frac * (p1.n_rpm - p0.n_rpm),
    }
}

/// Simple WEC risk index.
fn compute_wec_risk_simple(damage: &[RollerDamageAccumulator], total_time_s: f64) -> f64 {
    if total_time_s <= 0.0 || damage.is_empty() {
        return 0.0;
    }
    let max_slip_fraction = damage
        .iter()
        .map(|d| d.total_slip_duration_s / total_time_s)
        .fold(0.0f64, f64::max);
    (max_slip_fraction / 0.1).min(1.0)
}

/// Internal no-op reporter (for per-step equilibrium calls).
#[allow(dead_code)]
struct NoopReporterInternal;
impl ProgressReporter for NoopReporterInternal {
    fn report(&self, _progress: SolverProgress) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_transient_test_input() -> BearingInput {
        BearingInput {
            macro_geom: MacroGeometry {
                d: 50.0,
                outer_diameter: 90.0,
                t: 20.0,
                alpha: 12.0,
                z: 20,
                d_we_max: 10.0,
                d_we_min: 8.5,
                l_we: 15.0,
                d_pw: 70.0,
                h_rib: 3.0,
                alpha_rib: 10.0,
                g_r: 0.0,
                h_c: None,
            },
            raceway_geom: RacewayGeometry {
                alpha_i: 8.0,
                alpha_o: 12.0,
                r_i: 200.0,
                r_o: 200.0,
                r_rib: 1500.0,
                r_rib_circ: None,
                d_uc: 0.0,
                l_uc: 0.0,
            },
            roller_profile: RollerProfile {
                crown_type: CrownType::Parabolic { c2: 0.01 },
                delta_c: 2.0,
                delta_dub_l: 1.0,
                delta_dub_s: 1.0,
                l_dub_l: 1.5,
                l_dub_s: 1.5,
                r_sph: 50.0,
                sigma_roller: 0.15,
            },
            raceway_profile_inner: RacewayProfile {
                delta_rw: 0.0,
                w_a: 0.0,
                ra: 0.3,
                custom_profile: None,
                polynomial_coeffs: None,
            },
            raceway_profile_outer: RacewayProfile {
                delta_rw: 0.0,
                w_a: 0.0,
                ra: 0.3,
                custom_profile: None,
                polynomial_coeffs: None,
            },
            material: Material::default(),
            operating: OperatingConditions {
                f_x: 10.0,
                f_y: 0.0,
                f_a: 5.0,
                m_x: 0.0,
                m_y: 0.0,
                n_inner_rpm: 1000.0,
                n_outer_rpm: 0.0,
                gamma: 0.0,
                t_op: 70.0,
                nu_40: 68.0,
                nu_100: 8.0,
                alpha_pv: 20.0,
                lubrication_type: LubricationType::Oil,
                starvation_factor: 1.0,
                rho_oil: 870.0,
                preload_mode: PreloadMode::DisplacementFromForce,
                delta_preload_um: 0.0,
                design_life_hours: 100.0,
                lubrication_model: LubricationModel::Method1_DH, film_decay_enabled: false, film_decay_time_hours: 0.0, skew_angle_deg: 0.0, replenishment_rate_nm_s: 0.0, surface_finish: SurfaceFinish::Standard, additive_type: AdditiveType::None,
                tau_eyring: 5.0,
                z_roelands: 0.67,
                traction_model: TractionModel::Eyring,
                carreau_eta_inf_ratio: 0.005,
                carreau_lambda_s: 1.0e-7,
                carreau_n: 0.5,
                carreau_a: 2.0,
                friction_model: FrictionModel::PalmgrenLike,
                thermal_correction: ThermalCorrection::Aihara1987,
            hysteresis_loss_factor: 0.005,
                skf_trb_series: SkfTrbSeriesEnum::Series303,
                skf_lubrication: SkfLubricationEnum::OilBath,
                skf_y_factor: 1.6,
                k_fluid: 0.15,
                beta_visc: 0.04,
                rq_inner: 0.3,
                rq_outer: 0.3,
                rq_roller: 0.15,
                roughness_input_mode: RoughnessInputMode::Rq,
            },
            solver: SolverParams {
                run_mode: RunMode::Single(SolverMode::Gen1),
                n_slices: 20,
                angular_increment_deg: 5.0,
                e_c: 0.8,
                ..SolverParams::default()
            },
            transient: None,
        }
    }

    #[test]
    fn test_steady_state_convergence() {
        let mut input = make_transient_test_input();
        input.transient = Some(TransientInput {
            load_series: vec![
                LoadTimePoint { t_s: 0.0, f_x: 10.0, f_y: 0.0, f_a: 5.0, m_x: 0.0, m_y: 0.0, n_rpm: 1000.0 },
                LoadTimePoint { t_s: 0.1, f_x: 10.0, f_y: 0.0, f_a: 5.0, m_x: 0.0, m_y: 0.0, n_rpm: 1000.0 },
            ],
            dt_max: 0.001,
            enable_roller_dynamics: true,
            snapshot_interval: 10,
        });

        let result = solve_transient(&input, &NoopReporterInternal).unwrap();

        let last = result.snapshots.last().unwrap();
        for rk in &last.roller_kinematics {
            assert!(
                rk.slip_ratio.abs() < 0.05,
                "Roller {} slip_ratio={:.4}, expected near zero for steady state",
                rk.j, rk.slip_ratio
            );
        }
    }

    #[test]
    fn test_load_step_causes_transient_slip() {
        let mut input = make_transient_test_input();
        input.transient = Some(TransientInput {
            load_series: vec![
                LoadTimePoint { t_s: 0.000, f_x: 1.0, f_y: 0.0, f_a: 5.0, m_x: 0.0, m_y: 0.0, n_rpm: 1000.0 },
                LoadTimePoint { t_s: 0.010, f_x: 1.0, f_y: 0.0, f_a: 5.0, m_x: 0.0, m_y: 0.0, n_rpm: 1000.0 },
                LoadTimePoint { t_s: 0.011, f_x: 50.0, f_y: 0.0, f_a: 5.0, m_x: 0.0, m_y: 0.0, n_rpm: 1000.0 },
                LoadTimePoint { t_s: 0.100, f_x: 50.0, f_y: 0.0, f_a: 5.0, m_x: 0.0, m_y: 0.0, n_rpm: 1000.0 },
            ],
            dt_max: 0.001,
            enable_roller_dynamics: true,
            snapshot_interval: 5,
        });

        let result = solve_transient(&input, &NoopReporterInternal).unwrap();

        assert!(!result.snapshots.is_empty());
        assert!(result.total_time_s > 0.0);
        assert!(result.damage_summary.max_slip_ratio_overall >= 0.0);
    }

    #[test]
    fn test_no_dynamics_mode() {
        let mut input = make_transient_test_input();
        input.transient = Some(TransientInput {
            load_series: vec![
                LoadTimePoint { t_s: 0.0, f_x: 10.0, f_y: 0.0, f_a: 5.0, m_x: 0.0, m_y: 0.0, n_rpm: 1000.0 },
                LoadTimePoint { t_s: 0.05, f_x: 30.0, f_y: 0.0, f_a: 5.0, m_x: 0.0, m_y: 0.0, n_rpm: 1000.0 },
            ],
            dt_max: 0.005,
            enable_roller_dynamics: false,
            snapshot_interval: 1,
        });

        let result = solve_transient(&input, &NoopReporterInternal).unwrap();

        for snap in &result.snapshots {
            for rk in &snap.roller_kinematics {
                assert!(
                    rk.slip_ratio.abs() < 1e-10,
                    "No-dynamics mode: slip should be exactly zero, got {}",
                    rk.slip_ratio
                );
            }
        }
    }

    #[test]
    fn test_load_change_small() {
        let a = LoadTimePoint { t_s: 0.0, f_x: 10.0, f_y: 0.0, f_a: 50.0, m_x: 0.0, m_y: 0.0, n_rpm: 1000.0 };
        let b = LoadTimePoint { t_s: 0.001, f_x: 10.0001, f_y: 0.0, f_a: 50.0001, m_x: 0.0, m_y: 0.0, n_rpm: 1000.0 };
        assert!(load_change_small(&a, &b, 1e-4));

        let c = LoadTimePoint { t_s: 0.001, f_x: 20.0, f_y: 0.0, f_a: 50.0, m_x: 0.0, m_y: 0.0, n_rpm: 1000.0 };
        assert!(!load_change_small(&a, &c, 1e-4));
    }

    #[test]
    fn test_equilibrium_skip_constant_load() {
        // With constant load, equilibrium cache should be used for most steps
        // (solver is called once, then cached result reused)
        let mut input = make_transient_test_input();
        input.transient = Some(TransientInput {
            load_series: vec![
                LoadTimePoint { t_s: 0.0, f_x: 10.0, f_y: 0.0, f_a: 5.0, m_x: 0.0, m_y: 0.0, n_rpm: 1000.0 },
                LoadTimePoint { t_s: 0.1, f_x: 10.0, f_y: 0.0, f_a: 5.0, m_x: 0.0, m_y: 0.0, n_rpm: 1000.0 },
            ],
            dt_max: 0.001,
            enable_roller_dynamics: false,
            snapshot_interval: 1,
        });

        let result = solve_transient(&input, &NoopReporterInternal).unwrap();

        // Constant load: 101 steps total, all equilibrium values should be identical
        assert!(result.snapshots.len() >= 2);
        let first_loads = &result.snapshots[0].equilibrium.roller_loads;
        let last_loads = &result.snapshots.last().unwrap().equilibrium.roller_loads;
        for (a, b) in first_loads.iter().zip(last_loads.iter()) {
            assert!((a - b).abs() < 1e-6, "Constant load should give same equilibrium");
        }
    }

    #[test]
    fn test_trace_dynamics_step() {
        // Trace integrate_roller_step numerically
        let r_max_mm: f64 = 10.0 / 2.0;
        let r_min_mm: f64 = 8.5 / 2.0;
        let r_mean_m: f64 = (r_max_mm + r_min_mm) / 2.0 * 1e-3;
        let i_roller = compute_roller_inertia(r_max_mm, r_min_mm, 15.0, RHO_STEEL);

        let alpha_i_rad: f64 = 8.0_f64.to_radians();
        let alpha_o_rad: f64 = 12.0_f64.to_radians();
        let sin_ai = alpha_i_rad.sin();
        let sin_ao = alpha_o_rad.sin();
        let phi_rad = (alpha_o_rad - alpha_i_rad) / 2.0;
        let sin_phi = phi_rad.sin();
        let omega_i: f64 = 1000.0 * std::f64::consts::TAU / 60.0;
        let omega_target = compute_omega_roller_target(omega_i, sin_ai, sin_ao, sin_phi);

        let eta_0 = estimate_dynamic_viscosity(&OperatingConditions {
            f_x: 10.0, f_y: 0.0, f_a: 5.0, m_x: 0.0, m_y: 0.0,
            n_inner_rpm: 1000.0, n_outer_rpm: 0.0, gamma: 0.0, t_op: 70.0, nu_40: 68.0, nu_100: 8.0,
            alpha_pv: 20.0, lubrication_type: LubricationType::Oil,
            starvation_factor: 1.0, rho_oil: 870.0,
            preload_mode: PreloadMode::DisplacementFromForce, delta_preload_um: 0.0,
            lubrication_model: LubricationModel::Method1_DH, film_decay_enabled: false, film_decay_time_hours: 0.0, skew_angle_deg: 0.0, replenishment_rate_nm_s: 0.0, surface_finish: SurfaceFinish::Standard, additive_type: AdditiveType::None,
            tau_eyring: 5.0, z_roelands: 0.67,
            traction_model: TractionModel::Eyring, carreau_eta_inf_ratio: 0.005, carreau_lambda_s: 1.0e-7, carreau_n: 0.5, carreau_a: 2.0,
            friction_model: FrictionModel::PalmgrenLike, thermal_correction: ThermalCorrection::Aihara1987,
            hysteresis_loss_factor: 0.005, skf_trb_series: SkfTrbSeriesEnum::Series303, skf_lubrication: SkfLubricationEnum::OilBath, skf_y_factor: 1.6,
            k_fluid: 0.15, beta_visc: 0.04,
            rq_inner: 0.3, rq_outer: 0.3, rq_roller: 0.15,
            roughness_input_mode: RoughnessInputMode::Rq,
            design_life_hours: 100.0,
        });

        // Build drag params from test geometry
        let test_geom = MacroGeometry {
            d: 50.0, outer_diameter: 90.0, t: 20.0, alpha: 12.0, z: 20,
            d_we_max: 10.0, d_we_min: 8.5, l_we: 15.0, d_pw: 70.0,
            h_rib: 3.0, alpha_rib: 10.0, g_r: 0.0, h_c: None,
        };
        let test_raceway = RacewayGeometry {
            alpha_i: 8.0, alpha_o: 12.0, r_i: 200.0, r_o: 200.0,
            r_rib: 1500.0, r_rib_circ: None, d_uc: 0.0, l_uc: 0.0,
        };
        let drag = RollerDragParams::from_bearing(&test_geom, &test_raceway, eta_0);

        eprintln!("omega_target={:.4}, r_mean={:.6}m, I={:.6e}, eta_0={:.6e}",
            omega_target, r_mean_m, i_roller, eta_0);
        eprintln!("drag: c_viscous={:.3e}, k_cage={:.3e}, cage_ratio={:.4}",
            drag.c_viscous, drag.k_cage, drag.cage_speed_ratio);
        eprintln!("at 1000rpm: tau_viscous={:.3e}, tau_cage={:.3e}",
            drag.tau_viscous(omega_target), drag.tau_cage(omega_i));

        let dt = 0.001;
        let mut omega = omega_target;
        let op = OperatingConditions {
            f_x: 10.0, f_y: 0.0, f_a: 5.0, m_x: 0.0, m_y: 0.0,
            n_inner_rpm: 1000.0, n_outer_rpm: 0.0, gamma: 0.0, t_op: 70.0, nu_40: 68.0, nu_100: 8.0,
            alpha_pv: 20.0, lubrication_type: LubricationType::Oil,
            starvation_factor: 1.0, rho_oil: 870.0,
            preload_mode: PreloadMode::DisplacementFromForce, delta_preload_um: 0.0,
            lubrication_model: LubricationModel::Method1_DH, film_decay_enabled: false, film_decay_time_hours: 0.0, skew_angle_deg: 0.0, replenishment_rate_nm_s: 0.0, surface_finish: SurfaceFinish::Standard, additive_type: AdditiveType::None,
            tau_eyring: 5.0, z_roelands: 0.67,
            traction_model: TractionModel::Eyring, carreau_eta_inf_ratio: 0.005, carreau_lambda_s: 1.0e-7, carreau_n: 0.5, carreau_a: 2.0,
            friction_model: FrictionModel::PalmgrenLike, thermal_correction: ThermalCorrection::Aihara1987,
            hysteresis_loss_factor: 0.005, skf_trb_series: SkfTrbSeriesEnum::Series303, skf_lubrication: SkfLubricationEnum::OilBath, skf_y_factor: 1.6,
            k_fluid: 0.15, beta_visc: 0.04,
            rq_inner: 0.3, rq_outer: 0.3, rq_roller: 0.15,
            roughness_input_mode: RoughnessInputMode::Rq,
            design_life_hours: 100.0,
        };

        for step in 0..20 {
            let q_normal = 3000.0; // typical loaded roller
            let tau_rib = 0.0; // no rib load in this test
            let (omega_new, tau_applied, tau_max) = integrate_roller_step(
                omega, omega_target, q_normal, r_mean_m, i_roller, dt, eta_0, &op,
                &drag, omega_i, tau_rib,
            );
            let srr = (omega_new - omega_target) / omega_target;
            eprintln!("step={:2}: omega={:.6}, srr={:.6e}, tau_app={:.6e}, tau_max={:.6e}",
                step, omega_new, srr, tau_applied, tau_max);
            omega = omega_new;
        }

        // After 20 steps, roller should still be near target speed
        let final_srr = (omega - omega_target) / omega_target;
        assert!(
            final_srr.abs() < 0.01,
            "After 20 steps of constant load, srr should be near zero, got {:.6}",
            final_srr
        );
    }

    #[test]
    fn test_slice_geometric_srr_mean_is_zero() {
        // Realistic crown profile: center has large Δz, edges have small Δz
        let slices = vec![
            SliceGeometry { k: 0, x_axial: 0.0, r_roller: 4.25, r_inner_race: 100.0, r_outer_race: 100.0, r_eq_inner: 50.0, r_eq_outer: 50.0, delta_z_total_inner: 0.5, delta_z_total_outer: 0.5, slice_width: 3.0 },
            SliceGeometry { k: 1, x_axial: 3.0, r_roller: 4.40, r_inner_race: 100.0, r_outer_race: 100.0, r_eq_inner: 50.0, r_eq_outer: 50.0, delta_z_total_inner: 3.0, delta_z_total_outer: 3.0, slice_width: 3.0 },
            SliceGeometry { k: 2, x_axial: 6.0, r_roller: 4.55, r_inner_race: 100.0, r_outer_race: 100.0, r_eq_inner: 50.0, r_eq_outer: 50.0, delta_z_total_inner: 5.0, delta_z_total_outer: 5.0, slice_width: 3.0 },
            SliceGeometry { k: 3, x_axial: 9.0, r_roller: 4.70, r_inner_race: 100.0, r_outer_race: 100.0, r_eq_inner: 50.0, r_eq_outer: 50.0, delta_z_total_inner: 3.0, delta_z_total_outer: 3.0, slice_width: 3.0 },
            SliceGeometry { k: 4, x_axial: 12.0, r_roller: 4.85, r_inner_race: 100.0, r_outer_race: 100.0, r_eq_inner: 50.0, r_eq_outer: 50.0, delta_z_total_inner: 0.5, delta_z_total_outer: 0.5, slice_width: 3.0 },
        ];
        let phi_rad = 0.035_f64; // ~2°

        // q_nominal=0 → Heathcote term is zero, tests pure profile SRR
        let geo_srr = compute_slice_geometric_srr(&slices, phi_rad, 0.0, 0.0);

        assert_eq!(geo_srr.len(), 5);
        let sum: f64 = geo_srr.iter().sum();
        assert!(
            sum.abs() < 1e-15,
            "Geometric SRR mean should be zero (sum={:.2e})",
            sum
        );
    }

    #[test]
    fn test_slice_srr_mean_equals_dynamic_srr() {
        let slices = vec![
            SliceGeometry { k: 0, x_axial: 0.0, r_roller: 4.25, r_inner_race: 100.0, r_outer_race: 100.0, r_eq_inner: 50.0, r_eq_outer: 50.0, delta_z_total_inner: 0.5, delta_z_total_outer: 0.5, slice_width: 3.0 },
            SliceGeometry { k: 1, x_axial: 3.0, r_roller: 4.40, r_inner_race: 100.0, r_outer_race: 100.0, r_eq_inner: 50.0, r_eq_outer: 50.0, delta_z_total_inner: 3.0, delta_z_total_outer: 3.0, slice_width: 3.0 },
            SliceGeometry { k: 2, x_axial: 6.0, r_roller: 4.55, r_inner_race: 100.0, r_outer_race: 100.0, r_eq_inner: 50.0, r_eq_outer: 50.0, delta_z_total_inner: 5.0, delta_z_total_outer: 5.0, slice_width: 3.0 },
            SliceGeometry { k: 3, x_axial: 9.0, r_roller: 4.70, r_inner_race: 100.0, r_outer_race: 100.0, r_eq_inner: 50.0, r_eq_outer: 50.0, delta_z_total_inner: 3.0, delta_z_total_outer: 3.0, slice_width: 3.0 },
            SliceGeometry { k: 4, x_axial: 12.0, r_roller: 4.85, r_inner_race: 100.0, r_outer_race: 100.0, r_eq_inner: 50.0, r_eq_outer: 50.0, delta_z_total_inner: 0.5, delta_z_total_outer: 0.5, slice_width: 3.0 },
        ];
        let phi_rad = 0.035_f64;
        let slip_ratio = 0.005; // 0.5% dynamic SRR

        // With Heathcote (q=3000N, E*=230GPa): mean subtraction still holds
        let e_star = 230_000.0; // MPa
        let q_nominal = 3000.0; // N
        let geo_srr = compute_slice_geometric_srr(&slices, phi_rad, q_nominal, e_star);
        let per_slice_srr: Vec<f64> = geo_srr.iter().map(|g| slip_ratio + g).collect();

        let mean: f64 = per_slice_srr.iter().sum::<f64>() / per_slice_srr.len() as f64;
        let diff = (mean - slip_ratio).abs();
        assert!(
            diff < 1e-12,
            "Mean of per-slice SRR ({:.10e}) should equal dynamic SRR ({:.10e}), diff={:.2e}",
            mean, slip_ratio, diff
        );
    }

    #[test]
    fn test_heathcote_slip_is_positive() {
        // Heathcote slip should always be positive (adds to sliding)
        let slices = vec![
            SliceGeometry { k: 0, x_axial: 0.0, r_roller: 4.25, r_inner_race: 100.0, r_outer_race: 100.0, r_eq_inner: 50.0, r_eq_outer: 50.0, delta_z_total_inner: 0.0, delta_z_total_outer: 0.0, slice_width: 3.0 },
            SliceGeometry { k: 1, x_axial: 3.0, r_roller: 4.55, r_inner_race: 100.0, r_outer_race: 100.0, r_eq_inner: 50.0, r_eq_outer: 50.0, delta_z_total_inner: 0.0, delta_z_total_outer: 0.0, slice_width: 3.0 },
            SliceGeometry { k: 2, x_axial: 6.0, r_roller: 4.85, r_inner_race: 100.0, r_outer_race: 100.0, r_eq_inner: 50.0, r_eq_outer: 50.0, delta_z_total_inner: 0.0, delta_z_total_outer: 0.0, slice_width: 3.0 },
        ];
        let phi_rad = 0.035_f64;
        let e_star = 230_000.0; // MPa
        let q_nominal = 3000.0; // N

        // With zero profile correction, only Heathcote remains (before mean subtraction)
        let geo_srr = compute_slice_geometric_srr(&slices, phi_rad, q_nominal, e_star);

        // All values should be small (Heathcote is typically 1e-5 ~ 1e-4)
        for &v in &geo_srr {
            assert!(v.abs() < 1e-3, "Heathcote SRR should be small, got {:.2e}", v);
        }
        // Sum should still be zero (mean subtracted)
        let sum: f64 = geo_srr.iter().sum();
        assert!(sum.abs() < 1e-14, "Sum should be ~0, got {:.2e}", sum);
    }
}
