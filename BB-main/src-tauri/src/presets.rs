use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

use crate::solver::types::*;

#[derive(Serialize)]
pub struct PresetInfo {
    pub name: String,
    pub modified: String,
}

fn presets_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("presets");
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    }
    Ok(dir)
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' || c == '(' || c == ')' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
        + ".json"
}

#[tauri::command]
pub fn list_presets(app: AppHandle) -> Result<Vec<PresetInfo>, String> {
    let dir = presets_dir(&app)?;
    let mut presets = Vec::new();

    let entries = fs::read_dir(&dir).map_err(|e| e.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            let modified = fs::metadata(&path)
                .and_then(|m| m.modified())
                .ok()
                .map(|t| {
                    let duration = t
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default();
                    let secs = duration.as_secs();
                    // Simple date formatting: YYYY-MM-DD HH:MM
                    let days = secs / 86400;
                    let time_of_day = secs % 86400;
                    let hours = time_of_day / 3600;
                    let minutes = (time_of_day % 3600) / 60;
                    // Approximate year/month/day from epoch days
                    // Good enough for display purposes
                    format_epoch_days(days, hours, minutes)
                })
                .unwrap_or_default();

            presets.push(PresetInfo { name, modified });
        }
    }

    presets.sort_by(|a, b| b.modified.cmp(&a.modified));
    Ok(presets)
}

fn format_epoch_days(total_days: u64, hours: u64, minutes: u64) -> String {
    // Simple epoch-to-date conversion (no leap second accuracy needed for display)
    let mut days = total_days as i64;
    let mut year = 1970i64;

    loop {
        let days_in_year = if is_leap(year) { 366 } else { 365 };
        if days < days_in_year {
            break;
        }
        days -= days_in_year;
        year += 1;
    }

    let month_days = if is_leap(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1u64;
    for &md in &month_days {
        if days < md {
            break;
        }
        days -= md;
        month += 1;
    }
    let day = days + 1;

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        year, month, day, hours, minutes
    )
}

fn is_leap(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

#[tauri::command]
pub fn save_preset(app: AppHandle, name: String, input: BearingInput) -> Result<(), String> {
    let dir = presets_dir(&app)?;
    let filename = sanitize_filename(&name);
    let path = dir.join(&filename);
    let json = serde_json::to_string_pretty(&input).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn load_preset(app: AppHandle, name: String) -> Result<BearingInput, String> {
    let dir = presets_dir(&app)?;
    let filename = sanitize_filename(&name);
    let path = dir.join(&filename);
    let json = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let input: BearingInput = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    Ok(input)
}

#[tauri::command]
pub fn get_last_preset(app: AppHandle) -> Result<Option<String>, String> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("last_preset.txt");
    if path.exists() {
        let name = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let name = name.trim().to_string();
        if name.is_empty() {
            Ok(None)
        } else {
            Ok(Some(name))
        }
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub fn save_last_preset(app: AppHandle, name: String) -> Result<(), String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    }
    fs::write(dir.join("last_preset.txt"), name).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_preset(app: AppHandle, name: String) -> Result<(), String> {
    let dir = presets_dir(&app)?;
    let filename = sanitize_filename(&name);
    let path = dir.join(&filename);
    fs::remove_file(&path).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn ensure_default_preset(app: AppHandle) -> Result<(), String> {
    let dir = presets_dir(&app)?;

    // Only create default if no presets exist
    let has_presets = fs::read_dir(&dir)
        .map_err(|e| e.to_string())?
        .any(|e| {
            e.ok()
                .and_then(|e| {
                    e.path()
                        .extension()
                        .and_then(|s| s.to_str())
                        .map(|s| s == "json")
                })
                .unwrap_or(false)
        });

    if has_presets {
        return Ok(());
    }

    let default_input = default_bearing_input();
    let json = serde_json::to_string_pretty(&default_input).map_err(|e| e.to_string())?;
    let path = dir.join("NU 240 (CRB Default).json");
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}

/// Construct the default BearingInput for CRB (Phase 1 stub — NU 240 유사 파라미터).
/// Phase 1.5 (defaults) 에서 정식 값 확정 예정.
fn default_bearing_input() -> BearingInput {
    BearingInput {
        macro_geom: MacroGeometry {
            d: 200.0,
            outer_diameter: 360.0,
            t: 58.0,
            z: 18,
            d_we: 44.0,      // 균일 원통 roller diameter
            l_we: 42.0,
            d_pw: 280.0,
            g_r: 30.0,
        },
        raceway_geom: RacewayGeometry {
            r_i: 1.0e9,       // 원통 raceway (transverse 곡률 무한대 근사)
            r_o: 1.0e9,
            d_uc: 0.0,
            l_uc: 0.0,
        },
        roller_profile: RollerProfile {
            crown_type: CrownType::Logarithmic { a_log: 0.0 },
            delta_c: 5.0,
            delta_dub: 0.0,   // 양쪽 대칭 (D1: rib 없음, 부록 A.1)
            l_dub: 0.0,
            sigma_roller: 0.15,
        },
        raceway_profile_inner: RacewayProfile {
            delta_rw: 0.0,
            w_a: 0.0,
            ra: 0.15,
            custom_profile: None,
            polynomial_coeffs: None,
        },
        raceway_profile_outer: RacewayProfile {
            delta_rw: 0.0,
            w_a: 0.0,
            ra: 0.15,
            custom_profile: None,
            polynomial_coeffs: None,
        },
        material: Material {
            e_roller: 210.0,
            e_ring: 210.0,
            nu: 0.3,
            hrc: 61.0,
            density_roller: 7.85,
            density_ring: 7.85,
        },
        operating: OperatingConditions {
            f_x: 100.0,
            f_y: -500.0,      // -Y = 중력 방향 (D5)
            m_x: 0.0,         // single-plane misalignment 축 (D6)
            n_inner_rpm: 500.0,
            n_outer_rpm: 0.0,
            gamma: 0.0,
            t_op: 60.0,
            nu_40: 68.0,
            nu_100: 8.0,
            alpha_pv: 20.0,
            lubrication_type: LubricationType::Oil,
            starvation_factor: 1.0,
            rho_oil: 850.0,
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
            // skf_trb_series 제거 (CRB Phase 7 에서 SKF 대응 검토)
            skf_lubrication: SkfLubricationEnum::OilBath,
            skf_y_factor: 1.6,
            k_fluid: 0.15,
            beta_visc: 0.04,
            roughness_input_mode: RoughnessInputMode::Rq,
            rq_inner: 0.3,
            rq_outer: 0.3,
            rq_roller: 0.15,
        },
        solver: SolverParams {
            run_mode: RunMode::Single(SolverMode::Gen1),
            n_slices: 30,
            beam_type: BeamType::Timoshenko,
            convergence_tol: 1e-4,
            max_iterations: 200,
            angular_increment_deg: 2.0,
            life_method: LifeMethod::Iso16281,
            e_c: 0.0, // auto (ISO 281 Annex A)
            contamination_level: ContaminationLevel::NormalCleanliness,
            oil_supply_method: OilSupplyMethod::OilBath,
            c_r_kn: Some(59.5),
            c_0r_kn: Some(59.8),
            f_s_min: 1.0,
            rib_contact_mode: RibContactMode::PostProcess,
            f_0r: 3.0,
            f_1r: 0.0004,
            kappa_method: KappaMethod::ViscosityRatio,
            use_split_contact: false,
        },
        transient: None,
    }
}
