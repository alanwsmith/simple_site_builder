use std::{ffi::OsStr, path::PathBuf};
use walkdir::WalkDir;

pub fn files_in_folder_with_extension(
  path: &str,
  extension: &str,
) -> Vec<String> {
  WalkDir::new(PathBuf::from(format!("content/{}", path)))
    .min_depth(1)
    .max_depth(1)
    .sort_by_file_name()
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.path().is_file())
    .filter(|e| {
      !e.file_name()
        .to_str()
        .unwrap_or("")
        .starts_with(".")
    })
    .filter(|e| {
      e.path().extension().unwrap_or(OsStr::new(""))
        == extension
    })
    .map(|e| e.path().to_string_lossy().to_string())
    .collect::<Vec<_>>()
}
