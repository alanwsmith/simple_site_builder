use std::path::PathBuf;
use walkdir::WalkDir;

pub fn folders_in_folder(path: &str) -> Vec<String> {
  WalkDir::new(PathBuf::from(format!("content/{}", path)))
    .min_depth(1)
    .max_depth(1)
    .sort_by_file_name()
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.path().is_dir())
    .map(|e| e.path().to_string_lossy().to_string())
    .collect::<Vec<_>>()
}
