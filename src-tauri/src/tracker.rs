use crate::scanner::FileEntry;
use sqlx::SqlitePool;

pub async fn track_files(
    pool: &SqlitePool,
    watch_rule_id: i64,
    files: Vec<FileEntry>,
) -> Result<(), sqlx::Error> {
    for file in files {
        sqlx::query(
            "
            INSERT INTO tracked_files (
                watch_rule_id,
                path,
                first_seen_at,
                staged_at,
                deleted_at,
                status,
                size, modified_at
            )
            VALUES (?, ?, strftime('%s','now'), NULL, NULL, 'Active', ?, ?)
            ON CONFLICT(watch_rule_id, path) DO UPDATE SET
                first_seen_at = CASE WHEN tracked_files.status='Missing' OR tracked_files.size != excluded.size OR tracked_files.modified_at != excluded.modified_at THEN strftime('%s','now') ELSE tracked_files.first_seen_at END,
                staged_at = CASE WHEN tracked_files.status='Missing' OR tracked_files.size != excluded.size OR tracked_files.modified_at != excluded.modified_at THEN NULL ELSE tracked_files.staged_at END,
                status = CASE WHEN tracked_files.status='Missing' OR tracked_files.size != excluded.size OR tracked_files.modified_at != excluded.modified_at THEN 'Active' ELSE tracked_files.status END,
                size = excluded.size, modified_at = excluded.modified_at
            ",
        )
        .bind(watch_rule_id)
        .bind(file.path)
        .bind(file.size as i64)
        .bind(file.modified_at)
        .execute(pool)
        .await?;
    }

    Ok(())
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TrackedFileView {
    pub id: i64,
    pub path: String,
    pub status: String,
    pub age_days: i64,
    pub staged_at: Option<i64>,
    pub delete_at: Option<i64>,
    pub size: i64,
}

pub async fn list_files(pool: &SqlitePool) -> Result<Vec<TrackedFileView>, sqlx::Error> {
    sqlx::query_as::<_, TrackedFileView>(
        "SELECT tf.id, tf.path, tf.status,
          CAST((strftime('%s','now') - tf.first_seen_at) / 86400 AS INTEGER) AS age_days,
          tf.staged_at,
          CASE WHEN tf.staged_at IS NOT NULL THEN tf.staged_at + wr.delete_after_days * 86400 END AS delete_at,
          tf.size
         FROM tracked_files tf JOIN watch_rules wr ON wr.id = tf.watch_rule_id
         ORDER BY tf.status DESC, tf.path"
    ).fetch_all(pool).await
}

pub async fn cancel_mark(pool: &SqlitePool, path: &str) -> Result<(), String> {
    sqlx::query("UPDATE tracked_files SET status='Active', staged_at=NULL, first_seen_at=strftime('%s','now') WHERE path=? AND status='Staged'")
        .bind(path).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn reconcile(pool: &SqlitePool) -> Result<(), String> {
    let rules = sqlx::query_as::<_, (i64, String, i64, i64)>(
        "SELECT id, path, stage_after_days, delete_after_days FROM watch_rules"
    ).fetch_all(pool).await.map_err(|e| e.to_string())?;
    for (id, path, stage_days, grace_days) in rules {
        let files = match crate::scanner::scan_folder_internal(&path) {
            Ok(files) => files,
            Err(error) => { eprintln!("Gobbler: cannot scan watched folder {path}: {error}"); continue; }
        };
        let present: std::collections::HashSet<String> = files.iter().map(|file| file.path.clone()).collect();
        let known = sqlx::query_as::<_, (String,)>("SELECT path FROM tracked_files WHERE watch_rule_id=?").bind(id).fetch_all(pool).await.map_err(|e| e.to_string())?;
        for (known_path,) in known {
            if !present.contains(&known_path) {
                sqlx::query("UPDATE tracked_files SET status='Missing', staged_at=NULL WHERE watch_rule_id=? AND path=? AND status != 'Missing'").bind(id).bind(&known_path).execute(pool).await.map_err(|e| e.to_string())?;
            }
        }
        track_files(pool, id, files).await.map_err(|e| e.to_string())?;
        sqlx::query("UPDATE tracked_files SET status='Staged', staged_at=strftime('%s','now') WHERE watch_rule_id=? AND status='Active' AND first_seen_at <= strftime('%s','now') - ? * 86400")
            .bind(id).bind(stage_days).execute(pool).await.map_err(|e| e.to_string())?;
        let due = sqlx::query_as::<_, (i64, String, i64, i64)>("SELECT id, path, size, modified_at FROM tracked_files WHERE watch_rule_id=? AND status='Staged' AND staged_at <= strftime('%s','now') - ? * 86400")
            .bind(id).bind(grace_days).fetch_all(pool).await.map_err(|e| e.to_string())?;
        let deletion_enabled: i64 = sqlx::query_scalar("SELECT delete_enabled FROM app_settings WHERE id=1")
            .fetch_one(pool).await.map_err(|e| e.to_string())?;
        for (file_id, file_path, size, modified_at) in due {
            let metadata = match std::fs::symlink_metadata(&file_path) {
                Ok(metadata) => metadata,
                Err(error) => { eprintln!("Gobbler: cannot inspect due file {file_path}: {error}"); continue; }
            };
            let actual_modified = metadata.modified().ok().and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs() as i64).unwrap_or(0);
            if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() as i64 != size || actual_modified != modified_at {
                eprintln!("Gobbler: due file changed or is not regular; skipped: {file_path}"); continue;
            }
            if deletion_enabled == 0 { eprintln!("Gobbler dry-run: would delete {file_path}"); continue; }
            if let Err(error) = std::fs::remove_file(&file_path) { eprintln!("Gobbler: failed to delete {file_path}: {error}"); continue; }
            let name = std::path::Path::new(&file_path).file_name().and_then(|n| n.to_str()).unwrap_or("");
            sqlx::query("INSERT INTO deleted_files (file_name, path, size, deleted_at) VALUES (?, ?, ?, strftime('%s','now'))")
                .bind(name).bind(&file_path).bind(size).execute(pool).await.map_err(|e| e.to_string())?;
            sqlx::query("DELETE FROM tracked_files WHERE id=?").bind(file_id).execute(pool).await.map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
