// TODO: Deprecate in favor of file_list
//
//
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn get_files(dir: &PathBuf) -> Vec<PathBuf> {
  WalkDir::new(dir)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.path().is_file())
    .map(|e| e.path().to_path_buf())
    .map(|pb| pb.strip_prefix(dir).unwrap().to_path_buf())
    .collect()
}
