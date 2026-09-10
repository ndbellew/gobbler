use crate::models::StagedFile;
use std::fs;
use std::time::{Duration, SystemTime};

pub fn should_delete(
    file: &StagedFile,
    delete_after_days: u64,
) -> bool {
    let delete_after =
        Duration::from_secs(delete_after_days * 24 * 60 * 60);

    match SystemTime::now().duration_since(file.staged_at) {
        Ok(age) => age >= delete_after,
        Err(_) => false,
    }
}

pub fn delete_staged_file(file: &StagedFile) -> Result<(), String> {
    fs::remove_file(&file.staged_path)
        .map_err(|err| err.to_string())
}