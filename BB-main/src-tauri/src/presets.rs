use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

use crate::solver::bb::types::*;
use crate::solver::common::types::*;

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
pub fn save_preset(app: AppHandle, name: String, input: BbInput) -> Result<(), String> {
    let dir = presets_dir(&app)?;
    let filename = sanitize_filename(&name);
    let path = dir.join(&filename);
    let json = serde_json::to_string_pretty(&input).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn load_preset(app: AppHandle, name: String) -> Result<BbInput, String> {
    let dir = presets_dir(&app)?;
    let filename = sanitize_filename(&name);
    let path = dir.join(&filename);
    let json = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let input: BbInput = serde_json::from_str(&json).map_err(|e| e.to_string())?;
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

    // 프리셋이 하나도 없을 때만 기본값 생성.
    // NOTE (BB P1-S2): CRB 시절 프리셋 JSON 은 BbInput 스키마가 달라
    // load_preset 에서 역직렬화 오류가 난다. 사용자 결정에 따라 마이그레이션은
    // 제공하지 않으며, 구 프리셋은 폐기 대상이다 (파일 삭제는 사용자 몫).
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
    let path = dir.join("7210 (ACBB Default).json");
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}

/// ACBB 기본 프리셋 (BB Phase 1-S2, 2026-08-20).
///
/// 경계치수 d/D/B 는 ISO 15 치수계열 7210 형번 기준.
/// **Z 와 D_w 는 제조사별 값이라 가정값이다** (실 카탈로그 미확인 — Plan T-6 참조).
/// 홈 반경은 ISO 16281 Annex B.2 참조기하 (r_i = 0,52 D_w, r_e = 0,53 D_w).
/// 단위는 전부 솔버 내부 단위 mm · N · rad (Plan D-10).
fn default_bearing_input() -> BbInput {
    let d_w_mm = 11.5; // [mm] 가정값
    let (r_i_mm, r_e_mm) = BallBearingGeometry::reference_groove_radii(d_w_mm);
    BbInput {
        kind: BallBearingKind::Acbb,
        geometry: BallBearingGeometry {
            bore_mm: 50.0,
            outer_diameter_mm: 90.0,
            width_mm: 20.0,
            z: 16, // 가정값
            d_w_mm,
            d_pw_mm: 70.0, // (d + D) / 2
            r_i_mm,
            r_e_mm,
            alpha_nom_rad: 40.0_f64.to_radians(),
            clearance: BbClearanceSpec::InitialAngleRad(40.0_f64.to_radians()),
        },
        material: Material::default(),
        operating: BbOperatingConditions {
            f_x_n: 5_000.0, // [N] 축하중
            f_y_n: 2_000.0, // [N] 반경하중 (Y)
            f_z_n: 0.0,
            m_y_nmm: 0.0,
            m_z_nmm: 0.0,
            n_inner_rpm: 1_500.0,
            n_outer_rpm: 0.0,
            temperature_c: 70.0,
        },
        solver: BbSolverParams::default(),
    }
}
