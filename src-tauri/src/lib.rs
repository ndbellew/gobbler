mod models;
mod scanner;
mod watchers;
mod database;

use models::TrackedFileView;
use tauri::Manager;

#[tauri::command]
fn get_tracked_files() -> Vec<TrackedFileView> {
    vec![
        TrackedFileView {
            path: "/home/kaladin/Downloads/example.zip".to_string(),
            status: "Staged".to_string(),
            age_days: 3,
        },
        TrackedFileView {
            path: "/home/kaladin/Downloads/installer.exe".to_string(),
            status: "Active".to_string(),
            age_days: 1,
        },
    ]
}

#[tauri::command]
fn get_app_status() -> String {
    "Gobbler core is alive".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to find app data directory");

            std::fs::create_dir_all(&app_data_dir)
                .expect("Failed to create app data directory");

            let database_path = app_data_dir.join("gobbler.db");

            let pool = tauri::async_runtime::block_on(
                database::initialize_database(&database_path)
            )
            .expect("Failed to initialize database");

            app.manage(pool);

            println!("Database initialized at: {:?}", database_path);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_app_status,
            scanner::scan_folder,
            get_tracked_files,
            watchers::save_watch_rule,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}