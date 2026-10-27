mod models;
mod scanner;
mod watchers;
mod database;
mod tracker;

use tauri::Manager;

#[tauri::command]
async fn get_tracked_files(pool: tauri::State<'_, sqlx::SqlitePool>) -> Result<Vec<tracker::TrackedFileView>, String> {
    tracker::list_files(pool.inner()).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn cancel_file_mark(path: String, pool: tauri::State<'_, sqlx::SqlitePool>) -> Result<(), String> {
    tracker::cancel_mark(pool.inner(), &path).await
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
            let worker_pool = app.state::<sqlx::SqlitePool>().inner().clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Err(error) = tracker::reconcile(&worker_pool).await {
                        eprintln!("Gobbler background scan failed: {error}");
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                }
            });

            println!("Database initialized at: {:?}", database_path);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_app_status,
            scanner::scan_folder,
            get_tracked_files,
            cancel_file_mark,
            watchers::save_watch_rule,
            watchers::get_watch_rules,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}