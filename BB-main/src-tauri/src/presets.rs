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
pub fn bb_preset_list(app: AppHandle) -> Result<Vec<PresetInfo>, String> {
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
pub fn bb_preset_save(app: AppHandle, name: String, input: BbInput) -> Result<(), String> {
    let dir = presets_dir(&app)?;
    let filename = sanitize_filename(&name);
    let path = dir.join(&filename);
    let json = serde_json::to_string_pretty(&input).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn bb_preset_load(app: AppHandle, name: String) -> Result<BbInput, String> {
    let dir = presets_dir(&app)?;
    let filename = sanitize_filename(&name);
    let path = dir.join(&filename);
    let json = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let input: BbInput = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    Ok(input)
}

#[tauri::command]
pub fn bb_preset_get_last(app: AppHandle) -> Result<Option<String>, String> {
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
pub fn bb_preset_save_last(app: AppHandle, name: String) -> Result<(), String> {
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
pub fn bb_preset_delete(app: AppHandle, name: String) -> Result<(), String> {
    let dir = presets_dir(&app)?;
    let filename = sanitize_filename(&name);
    let path = dir.join(&filename);
    fs::remove_file(&path).map_err(|e| e.to_string())?;
    Ok(())
}

/// 기본 프리셋 2종 (Plan §3.6.5.4).
///
/// 성격이 다른 둘을 둔다 — ① 은 **문헌 근거가 있는 유일한 기하**(Level B·B-3 대조용),
/// ② 는 **Level C·D 검증 픽스처와 동일한 기하**(화면 숫자를 검증 결과와 직접 대조).
/// 이름은 `bb_preset_ensure_default` 가 파일명으로도 쓰므로 변경 시 주의.
pub const PRESET_HARRIS_MINDEL_1973: &str = "Harris-Mindel 1973 (ACBB)";
pub const PRESET_VERIFICATION_FIXTURE: &str = "ACBB Verification Fixture (assumed Z, D_w)";

/// 기본 프리셋 전량. `(이름, 입력)` 목록.
///
/// **개별 존재 확인용**이다 — `bb_preset_ensure_default` 은 「프리셋이 하나라도 있으면
/// 아무것도 안 함」이 아니라 **각 항목마다** 파일 존재를 확인해 없는 것만 만든다.
/// 그래야 프리셋이 나중에 추가돼도 기존 사용자에게 전달된다.
pub fn default_presets() -> Vec<(&'static str, BbInput)> {
    vec![
        (PRESET_HARRIS_MINDEL_1973, harris_mindel_1973_input()),
        (PRESET_VERIFICATION_FIXTURE, verification_fixture_input()),
    ]
}

/// ① `Harris-Mindel 1973 (ACBB)` — 문헌 근거가 있는 유일한 기하.
///
/// 출처: **Harris & Mindel, *Wear* 23(3) 311–337 (1973), Fig. 15** — 전량 인쇄된 실기 ACBB.
/// `tests/contact_level_b3.rs` 와 **동일 출처**다. in·lb·psi → mm·N·MPa 환산값:
///
/// | 항목 | 원 자료 | 솔버 단위 |
/// |---|---|---|
/// | `Z` | 21 | 21 |
/// | `D_w` | 0,8125 in | 20,6375 mm |
/// | `D_pw` | 6,2008 in | 157,50032 mm |
/// | `f_i` = `f_e` | 0,5200 | `r` = 0,52 · D_w = 10,7315 mm |
/// | `α₀` | 27,0000° | 0,471239 rad |
/// | bore / OD / width | 4,93 / 7,48 / 1,28 in | 125,222 / 189,992 / 32,512 mm |
/// | `E` (링) | 2,96e7 psi | 204 085 MPa |
/// | `ν` | 0,25 | 0,25 |
///
/// **클리어런스**: 원 자료가 INTERNAL CLEARANCE (MOUNTED) **0,0000** 이므로
/// 클리어런스 0 — 즉 초기 접촉각 α₀ 가 자유 접촉각 27° 와 같다.
/// 그래서 `InitialAngleRad(27°)` 로 지정한다.
///
/// ⚠️ **정직하게 남겨야 할 두 가지** (Plan §3.6.5.4):
///
/// 1. **볼의 `E` 는 Fig. 15 에 인쇄되어 있지 않다.** 인쇄된 것은 HOUSING·OUTER RING·
///    INNER RING·SHAFT 뿐이다. 여기서는 링과 같은 204 085 MPa 를 쓰지만 그것은
///    **가정**이며 문헌값이 아니다. Level **B-3b** 의 접촉타원 `a`·`b` 계통편차
///    **−0,68 %** 의 유력한 원인이기도 하다 (Plan §3.6.3.1 대체판정).
/// 2. **원 예제의 운전조건(24 000 rpm)은 우리 정적 모델의 범위 밖이다.**
///    Level **B-3c** 가 자료로 확인했다 — 원심력/접촉하중 = **1,05**, 즉 원심력이
///    접촉하중과 같은 크기라 정적 평형 가정이 성립하지 않는다.
///    따라서 **기하만 문헌값이고 속도는 정적 범위에서 따로 잡았다**:
///    `n_inner_rpm` = **1 000** (`n·D_pw` = 1,58e5 < 1e6 이라 `HIGH_SPEED` 경고 없음).
///    축하중은 원 예제와 같은 3 000 lbf = **13 345 N** 을 쓴다.
///
/// `hrc` 와 밀도는 Fig. 15 에 없으므로 `Material::default()` (강 기준값) 를 그대로 쓴다.
/// 즉 이 프리셋에서 **문헌값인 것은 기하와 `E`(링)·`ν` 뿐**이다.
fn harris_mindel_1973_input() -> BbInput {
    BbInput {
        kind: BallBearingKind::Acbb,
        geometry: BallBearingGeometry {
            bore_mm: 125.222,           // 4,93 in
            outer_diameter_mm: 189.992, // 7,48 in
            width_mm: 32.512,           // 1,28 in
            z: 21,
            d_w_mm: 20.6375,    // 0,8125 in
            d_pw_mm: 157.50032, // 6,2008 in
            r_i_mm: 10.7315,    // f_i = 0,5200 → 0,52 · D_w
            r_e_mm: 10.7315,    // f_e = 0,5200 → 0,52 · D_w
            alpha_nom_rad: 27.0_f64.to_radians(),
            // 원 자료 INTERNAL CLEARANCE (MOUNTED) = 0,0000 → α₀ = 자유 접촉각 27°
            clearance: BbClearanceSpec::InitialAngleRad(27.0_f64.to_radians()),
        },
        material: Material {
            // 볼의 E 는 Fig. 15 에 없다 — 링과 같은 값을 쓰되 **가정**이다 (위 주석 1).
            e_ball_mpa: 204_085.0,
            e_ring_mpa: 204_085.0, // 2,96e7 psi (문헌값)
            nu: 0.25,              // (문헌값)
            // hrc·밀도는 Fig. 15 에 없다 → 강 기준 기본값 (Material::default()).
            ..Material::default()
        },
        operating: BbOperatingConditions {
            f_x_n: 13_345.0, // 3 000 lbf — 원 예제와 동일
            f_y_n: 0.0,
            f_z_n: 0.0,
            m_y_nmm: 0.0,
            m_z_nmm: 0.0,
            n_inner_rpm: 1_000.0, // 원 예제 24 000 rpm 이 아니다 (위 주석 2)
            n_outer_rpm: 0.0,
            temperature_c: 70.0,
        },
        solver: BbSolverParams::default(),
    }
}

/// ② `ACBB Verification Fixture (assumed Z, D_w)` — 구 `7210 (ACBB Default)` 의 개명판.
///
/// **값은 하나도 바뀌지 않았다. 이름만 바꿨다** (Plan §3.6.5.4 ②).
///
/// 경계치수 d/D/B 는 ISO 15 치수계열 7210 형번 기준이지만,
/// **`Z` 와 `D_w` 는 제조사별 값이라 가정값이다** (실 카탈로그 미확인 — Plan **T-6**).
/// 홈 반경은 ISO 16281 Annex B.2 참조기하 (r_i = 0,52 D_w, r_e = 0,53 D_w).
///
/// 🔑 **개명 사유**: 이 가정 기하(`D_w` 11,5 · `D_pw` 70 · α₀ 40° · `Z` 16)가
/// **Level C·D 검증 픽스처와 정확히 같다.** 즉 실제로는 7210 이 아니라 우리 검증 기준
/// 기하인데 형번 이름을 달고 있었다. 이름을 바꾸면
/// **화면에 뜬 숫자를 Level C·D 검증 결과와 직접 대조할 수 있다** —
/// 이것이 이 프리셋의 진짜 용도다. 실 카탈로그를 확보하면(T-6) 그때
/// **진짜 7210 프리셋을 별도로** 추가한다.
///
/// 단위는 전부 솔버 내부 단위 mm · N · rad (Plan D-10).
fn verification_fixture_input() -> BbInput {
    let d_w_mm = 11.5; // 가정값 (T-6)
    let (r_i_mm, r_e_mm) = BallBearingGeometry::reference_groove_radii(d_w_mm);
    BbInput {
        kind: BallBearingKind::Acbb,
        geometry: BallBearingGeometry {
            bore_mm: 50.0,
            outer_diameter_mm: 90.0,
            width_mm: 20.0,
            z: 16, // 가정값 (T-6)
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

#[tauri::command]
pub fn bb_preset_ensure_default(app: AppHandle) -> Result<(), String> {
    let dir = presets_dir(&app)?;

    // 기본 프리셋 **각각**의 존재를 개별 확인해 없는 것만 만든다 (Plan §3.6.5.4).
    // 예전에는 「프리셋이 하나라도 있으면 아무것도 안 함」이었다 — 그러면 나중에
    // 추가된 프리셋이 기존 사용자에게 영원히 전달되지 않는다.
    //
    // NOTE (BB P1-S2): CRB 시절 프리셋 JSON 은 BbInput 스키마가 달라
    // bb_preset_load 에서 역직렬화 오류가 난다. 사용자 결정에 따라 마이그레이션은
    // 제공하지 않으며, 구 프리셋은 폐기 대상이다 (파일 삭제는 사용자 몫).
    //
    // NOTE (BB P4-S2-1): 구 `7210 (ACBB Default).json` 도 **삭제하지 않는다** —
    // 이미 사용자 데이터다. 개명은 새 파일 생성으로만 이뤄진다.
    for (name, input) in default_presets() {
        let path = dir.join(sanitize_filename(name));
        if path.exists() {
            continue;
        }
        let json = serde_json::to_string_pretty(&input).map_err(|e| e.to_string())?;
        fs::write(&path, json).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 기본 프리셋 2종이 **실제로 저장·로드되는지** 왕복 확인한다.
    ///
    /// `bb_preset_save`/`bb_preset_load` 는 `AppHandle` 이 필요해 단위시험에서 부를 수
    /// 없으므로, 그 둘이 실제로 하는 일(= `serde_json` 왕복 + `sanitize_filename`)을
    /// 그대로 재현한다.
    #[test]
    fn default_presets_round_trip_and_validate() {
        let presets = default_presets();
        assert_eq!(presets.len(), 2, "기본 프리셋은 2종이다 (Plan §3.6.5.4)");

        let mut filenames = Vec::new();
        for (name, input) in &presets {
            // ① 솔버가 받아들이는 입력인가 — UI 가 아니라 Rust validate() 가 판정한다.
            input.validate().unwrap_or_else(|e| {
                panic!("프리셋 '{name}' 이 validate() 를 통과하지 못했다: {e}")
            });

            // ② 저장 → 로드 왕복. bb_preset_save/bb_preset_load 의 직렬화 경로와 동일하다.
            let json = serde_json::to_string_pretty(input).expect("직렬화 실패");
            let back: BbInput = serde_json::from_str(&json).expect("역직렬화 실패");
            assert_eq!(back.geometry.z, input.geometry.z);
            assert_eq!(back.geometry.d_w_mm, input.geometry.d_w_mm);
            assert_eq!(back.geometry.d_pw_mm, input.geometry.d_pw_mm);
            assert_eq!(back.geometry.clearance, input.geometry.clearance);
            assert_eq!(back.material.e_ball_mpa, input.material.e_ball_mpa);
            assert_eq!(back.operating.f_x_n, input.operating.f_x_n);

            // ③ 파일명이 서로 다른가 — 같으면 하나가 다른 하나를 덮어쓴다.
            filenames.push(sanitize_filename(name));
        }
        filenames.sort();
        filenames.dedup();
        assert_eq!(filenames.len(), 2, "프리셋 파일명이 충돌한다");
    }

    /// ① Harris-Mindel 1973 (Fig. 15) 의 환산값이 원 자료와 맞는지.
    ///
    /// 화면 숫자 = 검증 숫자 원칙(Plan §3.6.4.2)상 이 환산이 틀리면
    /// Level B-3 대조가 통째로 무의미해진다.
    #[test]
    fn harris_mindel_preset_matches_source_figure_15() {
        const IN_TO_MM: f64 = 25.4;
        let input = harris_mindel_1973_input();
        let g = &input.geometry;
        assert_eq!(g.z, 21);
        assert!((g.d_w_mm - 0.8125 * IN_TO_MM).abs() < 1e-9);
        assert!((g.d_pw_mm - 6.2008 * IN_TO_MM).abs() < 1e-6);
        // f_i = f_e = 0,5200
        assert!((g.r_i_mm - 0.52 * g.d_w_mm).abs() < 1e-9);
        assert!((g.r_e_mm - 0.52 * g.d_w_mm).abs() < 1e-9);
        assert!((g.alpha_nom_rad - 27.0_f64.to_radians()).abs() < 1e-12);
        // INTERNAL CLEARANCE (MOUNTED) = 0,0000 → α₀ = 자유 접촉각
        assert_eq!(
            g.clearance,
            BbClearanceSpec::InitialAngleRad(27.0_f64.to_radians())
        );

        // 정적 범위로 따로 잡은 속도 — HIGH_SPEED 문턱(n·D_pw = 1e6) 아래여야 한다.
        assert!(
            input.operating.n_inner_rpm * g.d_pw_mm < 1.0e6,
            "n·D_pw 가 HIGH_SPEED 문턱을 넘으면 프리셋의 취지(정적 범위)가 깨진다"
        );
    }

    /// ② 검증 픽스처는 **이름만** 바뀌었고 기하값은 그대로여야 한다.
    ///
    /// 이 값이 바뀌면 Level C·D 검증 결과와 화면 숫자의 대조가 성립하지 않는다.
    #[test]
    fn verification_fixture_geometry_is_unchanged() {
        let g = verification_fixture_input().geometry;
        assert_eq!(g.z, 16);
        assert_eq!(g.d_w_mm, 11.5);
        assert_eq!(g.d_pw_mm, 70.0);
        assert_eq!(g.alpha_nom_rad, 40.0_f64.to_radians());
        assert!(PRESET_VERIFICATION_FIXTURE.contains("Verification Fixture"));
        assert!(
            !PRESET_VERIFICATION_FIXTURE.contains("7210"),
            "형번 이름을 떼는 것이 개명의 목적이다 (T-6)"
        );
    }
}
