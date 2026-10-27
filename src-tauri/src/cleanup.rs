use crate::models::StagedFile;
use std::fs;
use std::time::{Duration, SystemTime};

use sqlx::SqlitePool;
use std::fs;

pub async fn delete_tracked_file(
    pool: &SqlitePool,
    tracked_file_id: i64,
    path: &str,
    size: i64,
) -> Result<(), String> {
    fs::remove_file(path)
        .map_err(|err| err.to_string())?;

    let file_name = std::path::Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("Invalid file name")?;

    sqlx::query(
        "
        INSERT INTO deleted_files (
            file_name,
            path,
            size,
            deleted_at
        )
        VALUES (?, ?, ?, strftime('%s','now'))
        ",
    )
    .bind(file_name)
    .bind(path)
    .bind(size)
    .execute(pool)
    .await
    .map_err(|err| err.to_string())?;

    sqlx::query(
        "
        DELETE FROM tracked_files
        WHERE id = ?
        ",
    )
    .bind(tracked_file_id)
    .execute(pool)
    .await
    .map_err(|err| err.to_string())?;

    Ok(())
}

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