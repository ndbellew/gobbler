use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize)]
pub enum FileStatus {
    Active,
    Staged,
    Deleted,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrackedFile {
    pub path: PathBuf,
    pub first_seen_at: SystemTime,
    pub staged_at: Option<SystemTime>,
    pub deleted_at: Option<SystemTime>,
    pub status: FileStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrackedFileView {
    pub path: String,
    pub status: String,
    pub age_days: u64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchRule {
    pub path: PathBuf,
    pub stage_after_days: u64,
    pub delete_after_days: u64,
}
