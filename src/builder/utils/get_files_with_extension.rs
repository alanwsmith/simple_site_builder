use std::fs;
use std::path::PathBuf;

pub fn get_files_with_extension(
  dir: &PathBuf,
  extension: &str,
) -> Vec<PathBuf> {
  if !dir.exists() {
    // returning empty here if dir doesn't
    // exist instead of an error because it's
    // fine if there are no find/replace files
    vec![]
  } else {
    fs::read_dir(dir)
      .unwrap()
      .filter(|p| p.as_ref().unwrap().path().is_file())
      .filter(|p| {
        match p.as_ref().unwrap().path().extension() {
          Some(ext) => ext == extension,
          None => false,
        }
      })
      .filter_map(|p| {
        match p.as_ref().unwrap().path().strip_prefix(".")
        {
          Ok(_) => None,
          Err(_) => Some(p.as_ref().unwrap().path()),
        }
      })
      .collect()
  }
}
