use serde::Serialize;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize)]
pub struct FolderDetails {
  pub parent: PathBuf,
  pub name: PathBuf,
}

impl FolderDetails {
  pub fn new(input_path: &Path) -> FolderDetails {
    FolderDetails {
      parent: match input_path.parent() {
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
      self.parent.to_string_lossy().to_string(),
      self.name.to_string_lossy().to_string(),
    )
  }
}
