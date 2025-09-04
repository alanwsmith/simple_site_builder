use anyhow::Result;
use std::path::PathBuf;

pub fn write_file_with_mkdir(
  output_path: &PathBuf,
  content: &str,
) -> Result<()> {
  if let Some(parent_dir) = output_path.parent() {
    std::fs::create_dir_all(parent_dir)?;
  }
  std::fs::write(output_path, content)?;
  Ok(())
}
