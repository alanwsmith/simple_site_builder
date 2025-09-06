use crate::builder::*;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn file_list(
  content_dir: &PathBuf
) -> Vec<FileDetails> {
  let mut file_list = WalkDir::new(content_dir)
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
    .filter(|pb| {
      pb.file_name().unwrap().display().to_string()
        != ".DS_Store".to_string()
    })
    .map(|pb| FileDetails::new(&pb))
    .collect::<Vec<FileDetails>>();
  file_list.sort_by_key(|f| f.sort_key());
  file_list
}
