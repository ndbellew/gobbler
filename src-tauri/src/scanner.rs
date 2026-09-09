use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub size: u64,
}

#[tauri::command]
pub fn scan_folder(path: String) -> Result<Vec<FileEntry>, String> {
    println!("scan_folder called with: {}", path);

    let path = PathBuf::from(path);

    let entries =
        fs::read_dir(&path).map_err(|err| format!("Failed to read directory: {}", err))?;

    let mut files = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|err| err.to_string())?;
        let metadata = entry.metadata().map_err(|err| err.to_string())?;

        if metadata.is_file() {
            files.push(FileEntry {
                name: entry.file_name().to_string_lossy().to_string(),
                path: entry.path().to_string_lossy().to_string(),
                size: metadata.len(),
            });
        }
    }

    println!("Found {} files", files.len());

    Ok(files)
}
