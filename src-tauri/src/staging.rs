use crate::models::StagedFile;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub fn stage_file(
    source: &Path,
    staging_directory: &Path,
) -> Result<StagedFile, String> {
    let file_name = source
        .file_name()
        .ok_or("Source path does not contain a valid file name")?;

    let destination: PathBuf = staging_directory.join(file_name);

    fs::create_dir_all(staging_directory)
        .map_err(|err| err.to_string())?;

    fs::rename(source, &destination)
        .map_err(|err| err.to_string())?;

    Ok(StagedFile {
        original_path: source.to_path_buf(),
        staged_path: destination,
        staged_at: SystemTime::now(),
    })
}