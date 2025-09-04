use serde::Serialize;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize)]
pub struct FolderDetails {
  pub parent_dir: PathBuf,
  pub name: PathBuf,
}

impl FolderDetails {
  pub fn new(input_path: &Path) -> FolderDetails {
    FolderDetails {
      parent_dir: match input_path.parent() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(""),
      },
      name: match input_path.file_name() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(""),
      },
    }
  }

  pub fn sort_key(&self) -> (String, String) {
    (
      self.parent_dir.to_string_lossy().to_string(),
      self.name.to_string_lossy().to_string(),
    )
  }
}
