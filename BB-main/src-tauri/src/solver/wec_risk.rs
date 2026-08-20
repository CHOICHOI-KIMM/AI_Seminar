//! WEC and smearing risk assessment for transient sliding analysis.
//!
//! Implements:
//! - NREL Guo(2021) slip-during-high-load WEC criterion
//! - Cumulative friction energy criterion (Argonne Lab)
//! - ISO 15243 smearing risk with flash temperature

use super::types::*;

/// Critical friction energy density [J/mm²] — literature threshold for WEC initiation.
/// Based on Argonne National Lab experimental data (Evans et al.).
const E_CRITICAL_J_PER_MM2: f64 = 50.0;

/// Flash temperature coefficient (Blok approximation).
/// ΔT = C_flash × μ × Q × V_slide / (b × k_thermal × √(V_slide × b / α_thermal))
/// Simplified: ΔT ≈ μ × p_mean × V_slide × C_blok
const C_BLOK: f64 = 0.1; // simplified coefficient [°C·mm²·s/(N·m)]

/// Evaluate comprehensive risk assessment from transient results.
pub fn evaluate_risk(
    result: &TransientResult,
    contact_area_mm2: f64,
) -> TransientRiskAssessment {
    let wec_guo = evaluate_wec_guo(result);
    let wec_energy_ratio = evaluate_wec_energy(&result.damage_summary, contact_area_mm2);
    let smearing = evaluate_smearing(result);

    let overall_risk_level = worst_risk(&[
        wec_guo.risk_level,
        energy_to_risk(wec_energy_ratio),
        smearing.risk_level,
    ]);

    let recommendations = generate_recommendations(&wec_guo, wec_energy_ratio, &smearing);

    TransientRiskAssessment {
        wec_guo,
        wec_energy_ratio,
        smearing,
        overall_risk_level,
        recommendations,
    }
}

/// NREL Guo(2021) WEC criterion.
///
/// Identifies slip events under high contact load as WEC risk indicators.
/// Risk = fraction of time with simultaneous slip + high load.
fn evaluate_wec_guo(result: &TransientResult) -> WecRiskAssessment {
    if result.snapshots.is_empty() {
        return WecRiskAssessment {
            slip_load_fraction: 0.0,
            q_max_during_slip: 0.0,
            high_load_slip_events: 0,
            risk_index: 0.0,
            risk_level: RiskLevel::Low,
        };
    }

    // Find max load across entire simulation for threshold
    let q_max_overall = result.damage_summary.roller_damage.iter()
        .map(|d| d.max_contact_load_during_slip_n)
        .fold(0.0f64, f64::max);

    // High load threshold: 30% of max observed load
    let q_threshold = q_max_overall * 0.3;

    let mut slip_load_steps = 0usize;
    let mut high_load_slip_events = 0usize;
    let mut q_max_during_slip = 0.0f64;

    for snap in &result.snapshots {
        let has_slip_under_load = snap.roller_kinematics.iter().any(|rk| {
            if !rk.in_slip {
                return false;
            }
            // Find this roller's load
            let q_j = if rk.j < snap.equilibrium.roller_loads.len() {
                snap.equilibrium.roller_loads[rk.j]
            } else {
                0.0
            };
            if q_j > q_threshold {
                true
            } else {
                false
            }
        });

        if has_slip_under_load {
            slip_load_steps += 1;
        }

        // Track max load during any slip
        for rk in &snap.roller_kinematics {
            if rk.in_slip {
                let q_j = if rk.j < snap.equilibrium.roller_loads.len() {
                    snap.equilibrium.roller_loads[rk.j]
                } else {
                    0.0
                };
                q_max_during_slip = q_max_during_slip.max(q_j);
                if q_j > q_threshold {
                    high_load_slip_events += 1;
                }
            }
        }
    }

    let slip_load_fraction = slip_load_steps as f64 / result.snapshots.len() as f64;

    // Risk index: weighted combination of slip fraction and load severity
    let load_severity = if q_max_overall > 0.0 {
        (q_max_during_slip / q_max_overall).min(1.0)
    } else {
        0.0
    };

    let risk_index = (slip_load_fraction * 0.6 + load_severity * 0.4).min(1.0);

    let risk_level = if risk_index > 0.7 {
        RiskLevel::Critical
    } else if risk_index > 0.4 {
        RiskLevel::High
    } else if risk_index > 0.15 {
        RiskLevel::Medium
    } else {
        RiskLevel::Low
    };

    WecRiskAssessment {
        slip_load_fraction,
        q_max_during_slip,
        high_load_slip_events,
        risk_index,
        risk_level,
    }
}

/// Cumulative friction energy criterion (Argonne Lab).
///
/// Risk ratio = E_cumulative / (E_critical × contact_area)
fn evaluate_wec_energy(
    damage: &TransientDamageSummary,
    contact_area_mm2: f64,
) -> f64 {
    if contact_area_mm2 <= 0.0 {
        return 0.0;
    }

    let total_energy: f64 = damage.roller_damage.iter()
        .map(|d| d.cumulative_friction_energy_j)
        .sum();

    // Convert J to J/mm² (energy density)
    let energy_density = total_energy / contact_area_mm2;

    // Ratio to critical threshold
    energy_density / E_CRITICAL_J_PER_MM2
}

/// Convert energy ratio to risk level.
fn energy_to_risk(ratio: f64) -> RiskLevel {
    if ratio > 0.8 {
        RiskLevel::Critical
    } else if ratio > 0.5 {
        RiskLevel::High
    } else if ratio > 0.2 {
        RiskLevel::Medium
    } else {
        RiskLevel::Low
    }
}

/// Smearing risk assessment with flash temperature and SRR directionality.
///
/// WEC literature (Wear, 2022): negative SRR (roller slower than raceway,
/// driven contact regime) is more damaging than positive SRR for WEC initiation.
/// Positive SRR (roller faster) does not produce WEC under otherwise identical conditions.
fn evaluate_smearing(result: &TransientResult) -> SmearingRiskAssessment {
    let mut max_srr = 0.0f64;
    let mut max_negative_srr = 0.0f64; // most negative (worst for WEC)
    let mut max_positive_srr = 0.0f64;
    let mut peak_slide_velocity = 0.0f64;
    let mut max_flash_temp_rise = 0.0f64;

    for snap in &result.snapshots {
        for rk in &snap.roller_kinematics {
            max_srr = max_srr.max(rk.slip_ratio.abs());
            if rk.slip_ratio < 0.0 {
                max_negative_srr = max_negative_srr.max(rk.slip_ratio.abs());
            } else {
                max_positive_srr = max_positive_srr.max(rk.slip_ratio);
            }
            peak_slide_velocity = peak_slide_velocity.max(rk.u_slide_avg.abs());

            // Flash temperature estimate (simplified Blok)
            let q_j = if rk.j < snap.equilibrium.roller_loads.len() {
                snap.equilibrium.roller_loads[rk.j]
            } else {
                0.0
            };
            let mu_est = 0.05; // representative friction coefficient
            let delta_t = C_BLOK * mu_est * q_j * rk.u_slide_avg.abs();
            max_flash_temp_rise = max_flash_temp_rise.max(delta_t);
        }
    }

    // Total sliding distance (worst roller)
    let total_slide_distance = result.damage_summary.roller_damage.iter()
        .map(|d| d.cumulative_slide_distance_m)
        .fold(0.0f64, f64::max);

    // Risk classification: negative SRR gets stricter thresholds (WEC-prone)
    // Negative slip thresholds are 60% of standard (more conservative)
    let neg_factor = 0.6;
    let risk_level = if max_srr > 0.10
        || max_negative_srr > 0.10 * neg_factor
        || max_flash_temp_rise > 100.0
    {
        RiskLevel::Critical
    } else if max_srr > 0.05
        || max_negative_srr > 0.05 * neg_factor
        || max_flash_temp_rise > 50.0
    {
        RiskLevel::High
    } else if max_srr > 0.02
        || max_negative_srr > 0.02 * neg_factor
        || max_flash_temp_rise > 20.0
    {
        RiskLevel::Medium
    } else {
        RiskLevel::Low
    };

    SmearingRiskAssessment {
        max_srr,
        max_negative_srr,
        max_positive_srr,
        max_flash_temp_rise,
        total_slide_distance,
        peak_slide_velocity,
        risk_level,
    }
}

/// Return the worst (highest) risk level from a set.
fn worst_risk(levels: &[RiskLevel]) -> RiskLevel {
    let mut worst = RiskLevel::Low;
    for &level in levels {
        worst = match (worst, level) {
            (RiskLevel::Critical, _) | (_, RiskLevel::Critical) => RiskLevel::Critical,
            (RiskLevel::High, _) | (_, RiskLevel::High) => RiskLevel::High,
            (RiskLevel::Medium, _) | (_, RiskLevel::Medium) => RiskLevel::Medium,
            _ => RiskLevel::Low,
        };
    }
    worst
}

/// Generate human-readable recommendations based on risk assessment.
fn generate_recommendations(
    wec: &WecRiskAssessment,
    energy_ratio: f64,
    smearing: &SmearingRiskAssessment,
) -> Vec<String> {
    let mut recs = Vec::new();

    // WEC recommendations
    match wec.risk_level {
        RiskLevel::Critical | RiskLevel::High => {
            recs.push("최소 하중 확보: 예압 증가 또는 스프링 예압 적용 검토".into());
            recs.push("WEC 저항 윤활유(WEC-resistant additive) 적용 권장".into());
            if wec.slip_load_fraction > 0.3 {
                recs.push("운전 조건 검토: 하중 변동 주기 및 크기 감소 방안 검토".into());
            }
        }
        RiskLevel::Medium => {
            recs.push("윤활 조건 모니터링 강화 권장".into());
        }
        RiskLevel::Low => {}
    }

    // Energy recommendations
    if energy_ratio > 0.5 {
        recs.push("누적 마찰 에너지 과다: 베어링 표면 처리(Black oxide, DLC 코팅) 검토".into());
    }

    // Smearing recommendations
    match smearing.risk_level {
        RiskLevel::Critical | RiskLevel::High => {
            recs.push("스미어링 위험: 윤활유 점도 등급 상향 또는 EP 첨가제 적용".into());
            if smearing.max_flash_temp_rise > 50.0 {
                recs.push("국부 온도 상승 과다: 냉각 시스템 개선 또는 윤활 방식 변경 검토".into());
            }
        }
        RiskLevel::Medium => {
            recs.push("주기적 윤활유 상태 분석(페로그래피) 권장".into());
        }
        RiskLevel::Low => {}
    }

    // Directional SRR warning (negative slip is WEC-prone)
    if smearing.max_negative_srr > 0.03 {
        recs.push(format!(
            "음의 SRR 감지(max {:.2}%): 롤러 감속 방향 슬립은 WEC 발생 위험이 높음 — 최소 하중 및 가감속 조건 검토",
            smearing.max_negative_srr * 100.0
        ));
    }

    if recs.is_empty() {
        recs.push("현재 운전 조건에서 위험도 낮음 — 정상 유지보수 계획 유지".into());
    }

    recs
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_empty_result() -> TransientResult {
        TransientResult {
            snapshots: vec![],
            damage_summary: TransientDamageSummary {
                roller_damage: vec![],
                total_slip_events: 0,
                total_slip_duration_s: 0.0,
                max_slip_ratio_overall: 0.0,
                wec_risk_index: 0.0,
            },
            total_time_s: 0.0,
            elapsed_ms: 0.0,
            risk_assessment: None,
        }
    }

    #[test]
    fn test_empty_result_low_risk() {
        let result = make_empty_result();
        let risk = evaluate_risk(&result, 100.0);
        assert_eq!(risk.overall_risk_level, RiskLevel::Low);
        assert!((risk.wec_guo.risk_index - 0.0).abs() < 1e-10);
        assert!((risk.wec_energy_ratio - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_energy_ratio_scaling() {
        let damage = TransientDamageSummary {
            roller_damage: vec![RollerDamageAccumulator {
                j: 0,
                cumulative_friction_energy_j: 500.0, // 500 J
                cumulative_slide_distance_m: 1.0,
                max_contact_load_during_slip_n: 1000.0,
                slip_event_count: 5,
                total_slip_duration_s: 0.5,
            }],
            total_slip_events: 5,
            total_slip_duration_s: 0.5,
            max_slip_ratio_overall: 0.05,
            wec_risk_index: 0.5,
        };

        // 500 J / 100 mm² = 5 J/mm²; ratio = 5/50 = 0.1
        let ratio = evaluate_wec_energy(&damage, 100.0);
        assert!((ratio - 0.1).abs() < 1e-10);
    }

    #[test]
    fn test_worst_risk() {
        assert_eq!(worst_risk(&[RiskLevel::Low, RiskLevel::Low]), RiskLevel::Low);
        assert_eq!(worst_risk(&[RiskLevel::Low, RiskLevel::Medium]), RiskLevel::Medium);
        assert_eq!(worst_risk(&[RiskLevel::High, RiskLevel::Medium]), RiskLevel::High);
        assert_eq!(worst_risk(&[RiskLevel::Low, RiskLevel::Critical]), RiskLevel::Critical);
    }

    #[test]
    fn test_smearing_high_srr() {
        let mut result = make_empty_result();
        result.snapshots.push(TransientSnapshot {
            t_s: 0.0,
            operating: OperatingConditions {
                f_x: 10.0, f_y: 0.0, f_a: 5.0, m_x: 0.0, m_y: 0.0,
                n_inner_rpm: 1000.0, n_outer_rpm: 0.0, gamma: 0.0, t_op: 70.0, nu_40: 68.0, nu_100: 8.0,
                alpha_pv: 20.0, lubrication_type: LubricationType::Oil,
                starvation_factor: 1.0, rho_oil: 870.0,
                preload_mode: PreloadMode::DisplacementFromForce, delta_preload_um: 0.0,
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
                design_life_hours: 100.0,
            },
            equilibrium: BearingEquilibrium {
                displacement: [0.0; 5],
                roller_loads: vec![1000.0],
                roller_results: vec![],
                angular_distribution: vec![],
            },
            roller_kinematics: vec![RollerKinematicState {
                j: 0,
                psi_deg: 0.0,
                omega_roller_actual: 100.0,
                omega_roller_target: 105.0,
                slip_ratio: 0.08, // > 0.05 threshold
                u_slide_avg: 0.5,
                tau_traction: 1.0,
                tau_traction_max: 2.0,
                in_slip: true,
                slice_srr: vec![],
            }],
            sliding_metrics: TransientSlidingMetrics {
                n_rollers_in_slip: 1,
                max_slip_ratio: 0.08,
                max_slide_velocity: 0.5,
                instantaneous_friction_power: 50.0,
                max_slice_srr: 0.0,
            },
            slice_srr_map: vec![],
        });

        let smearing = evaluate_smearing(&result);
        assert!(smearing.risk_level == RiskLevel::High || smearing.risk_level == RiskLevel::Critical);
        assert!((smearing.max_srr - 0.08).abs() < 1e-10);
    }
}
