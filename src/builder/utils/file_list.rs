use crate::builder::*;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn file_list(
  content_dir: &PathBuf
) -> Vec<FileDetails> {
  WalkDir::new(content_dir)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.path().is_file())
    .map(|e| e.path().to_path_buf())
    .map(|pb| {
      pb.strip_prefix(format!(
        "{}/",
        content_dir.display()
      ))
      .unwrap()
      .to_path_buf()
    })
    .map(|pb| FileDetails::new(&pb))
    .collect()
}
