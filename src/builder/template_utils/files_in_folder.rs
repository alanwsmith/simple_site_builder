use std::path::PathBuf;
use walkdir::WalkDir;

pub fn files_in_folder(path: &str) -> Vec<String> {
  WalkDir::new(PathBuf::from(format!("content/{}", path)))
    .min_depth(1)
    .max_depth(1)
    .sort_by_file_name()
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.path().is_file())
    .map(|e| {
      e.path()
        .strip_prefix("content/")
        .expect("could not remove 'content' from path")
        .to_string_lossy()
        .to_string()
    })
    .collect::<Vec<_>>()
}
