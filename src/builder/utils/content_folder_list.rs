use crate::builder::*;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn content_folder_list(
  content_dir: &PathBuf
) -> Vec<ContentFolderDetails> {
  let mut folders = WalkDir::new(content_dir)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.path().is_dir())
    .map(|e| e.path().to_path_buf())
    .map(|pb| {
      pb.strip_prefix(format!(
        "{}/",
        content_dir.display()
      ))
      .unwrap()
      .to_path_buf()
    })
    .filter(|pb| pb.file_name().is_some())
    .map(|pb| ContentFolderDetails::new(&pb))
    .collect::<Vec<ContentFolderDetails>>();
  folders.sort_by_key(|k| k.sort_key());
  folders
}
