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

