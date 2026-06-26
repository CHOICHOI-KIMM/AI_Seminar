mod commands;
mod error;
mod presets;
mod solver;

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
            let _ = presets::ensure_default_preset(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::compute_slice_geometry,
            commands::compute_hertz_single_slice,
            commands::solve_roller_gen1,
            commands::solve_roller_gen1_for_load,
            commands::solve_roller_gen3,
            commands::solve_roller_gen3_for_load,
            commands::solve_bearing,
            commands::solve_bearing_dual,
            commands::compute_rib,
            commands::solve_transient,
            commands::parse_load_csv,
            commands::run_hmehl,
            presets::list_presets,
            presets::save_preset,
            presets::load_preset,
            presets::delete_preset,
            presets::ensure_default_preset,
            presets::get_last_preset,
            presets::save_last_preset,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
