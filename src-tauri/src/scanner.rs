use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub modified_at: i64,
}

pub fn scan_folder_internal(path: &str) -> Result<Vec<FileEntry>, String> {
    let path = PathBuf::from(path);

    let entries = fs::read_dir(&path)
        .map_err(|err| format!("Failed to read directory: {}", err))?;

    let mut files = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|err| err.to_string())?;
        let metadata = entry.metadata().map_err(|err| err.to_string())?;

        if metadata.is_file() {
            let modified_at = metadata.modified().ok().and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs() as i64).unwrap_or(0);
            files.push(FileEntry {
                name: entry.file_name().to_string_lossy().to_string(),
                path: entry.path().to_string_lossy().to_string(),
                size: metadata.len(),
                modified_at,
            });
        }
    }

    Ok(files)
}

#[tauri::command]
pub fn scan_folder(path: String) -> Result<Vec<FileEntry>, String> {
    scan_folder_internal(&path)
}