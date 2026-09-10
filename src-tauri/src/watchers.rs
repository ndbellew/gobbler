use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchRule {
    pub path: PathBuf,
    pub stage_after_days: u64,
    pub delete_after_days: u64,
}

#[tauri::command]
pub fn save_watch_rule(
    path: String,
    stage_after_days: u64,
    delete_after_days: u64,
) -> Result<(), String> {
    let rule = WatchRule {
        path: PathBuf::from(path),
        stage_after_days,
        delete_after_days,
    };

    println!("Saving rule: {:?}", rule);

    Ok(())
}