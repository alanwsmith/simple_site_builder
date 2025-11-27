use serde::Serialize;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize)]
pub struct FolderDetails {
  pub parent: PathBuf,
  pub name: PathBuf,
  pub folder_parts: Vec<String>,
}

impl FolderDetails {
  pub fn new(input_path: &Path) -> FolderDetails {
    FolderDetails {
      folder_parts: FolderDetails::get_folder_parts(
        input_path,
      ),
      name: match input_path.file_name() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(""),
      },
      parent: match input_path.parent() {
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

  pub fn get_folder_parts(
    input_path: &Path
  ) -> Vec<String> {
    let mut initial = input_path
      .iter()
      .map(|part| part.to_string_lossy().to_string())
      .collect::<Vec<String>>();
    let _ = initial.pop();
    initial
  }
}
