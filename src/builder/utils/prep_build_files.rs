use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn prep_build_files(
  source_dir: &PathBuf,
  dest_dir: &PathBuf,
) -> Result<()> {
  for entry in WalkDir::new(source_dir) {
    let source_path = entry?.into_path();
    let dest_path = dest_dir.join(
      source_path.strip_prefix(source_dir).unwrap(),
    );
    if source_path.is_dir() {
      fs::create_dir_all(dest_path)?;
    } else {
      let data = std::fs::read(source_path)?;
      std::fs::write(dest_path, &data)?;
    }
  }
  Ok(())
}
