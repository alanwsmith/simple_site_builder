use anyhow::Result;
use std::fs;
use std::path::PathBuf;

fn empty_dir(dir: &PathBuf) -> Result<()> {
  if let Ok(exists) = dir.try_exists() {
    if exists {
      for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
          fs::remove_dir_all(path)?;
        } else {
          fs::remove_file(path)?;
        }
      }
    }
  }
  Ok(())
}
