mod commands;
mod error;
mod presets;
pub mod solver;   // pub: integration tests (tests/geometry_level_a.rs) 접근용

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            // Create default preset if none exist
            let _ = presets::bb_preset_ensure_default(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::bb_compute_geometry,
            commands::bb_compute_contact,
            commands::bb_solve_bearing,
            presets::bb_preset_list,
            presets::bb_preset_save,
            presets::bb_preset_load,
            presets::bb_preset_delete,
            presets::bb_preset_ensure_default,
            presets::bb_preset_get_last,
            presets::bb_preset_save_last,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
