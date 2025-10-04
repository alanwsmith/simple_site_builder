use std::path::PathBuf;

use super::highlight_code;

pub fn highlight_file(file_path: &str) -> String {
  // TODO: Add measure to prevent calling outside
  // of the project root
  let path = PathBuf::from("content").join(file_path);
  let lang = match path.extension() {
    Some(e) => e.to_string_lossy().to_string(),
    None => "txt".to_string(),
  };
  if let Ok(code) = std::fs::read_to_string(path) {
    let highlighted = highlight_code(&code, &lang);
    highlighted
  } else {
    "no output".to_string()
  }
}
