use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub fn get_files_in_dir(
  dir: &PathBuf
) -> Result<Vec<PathBuf>> {
  let files = fs::read_dir(dir)?
    .filter(|p| {
      p.as_ref().unwrap().path().is_file()
    })
    .map(|p| p.as_ref().unwrap().path())
    .filter(|p| {
      !p.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .starts_with(".")
    })
    .collect();
  Ok(files)
}
