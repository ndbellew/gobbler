mod scanner;

#[tauri::command]
fn get_app_status() -> String {
    "Gobbler core is alive".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_app_status,
            scanner::scan_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
