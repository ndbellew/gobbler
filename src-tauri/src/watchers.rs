use crate::models::WatchRule;
use crate::scanner;
use crate::tracker;
use sqlx::SqlitePool;
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub async fn get_watch_rules(pool: State<'_, SqlitePool>) -> Result<Vec<WatchRule>, String> {
    let rows = sqlx::query_as::<_, (String, i64, i64)>(
        "SELECT path, stage_after_days, delete_after_days FROM watch_rules ORDER BY path"
    ).fetch_all(pool.inner()).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(path, stage_after_days, delete_after_days)| WatchRule {
        path: PathBuf::from(path), stage_after_days: stage_after_days as u64, delete_after_days: delete_after_days as u64,
    }).collect())
}

#[tauri::command]
pub async fn save_watch_rule(
    path: String,
    stage_after_days: u64,
    delete_after_days: u64,
    pool: State<'_, SqlitePool>,
) -> Result<WatchRule, String> {
    let result = sqlx::query(
        "
        INSERT INTO watch_rules (
            path,
            stage_after_days,
            delete_after_days
        )
        VALUES (?, ?, ?)
        ON CONFLICT(path) DO UPDATE SET
            stage_after_days = excluded.stage_after_days,
            delete_after_days = excluded.delete_after_days
        RETURNING id
        ",
    )
    .bind(&path)
    .bind(stage_after_days as i64)
    .bind(delete_after_days as i64)
    .fetch_one(pool.inner())
    .await
    .map_err(|err| err.to_string())?;

    let watch_rule_id: i64 = sqlx::Row::get(&result, "id");

    let files = scanner::scan_folder_internal(&path)?;

    tracker::track_files(
        pool.inner(),
        watch_rule_id,
        files,
    )
    .await
    .map_err(|err| err.to_string())?;

    Ok(WatchRule {
        path: PathBuf::from(path),
        stage_after_days,
        delete_after_days,
    })
}