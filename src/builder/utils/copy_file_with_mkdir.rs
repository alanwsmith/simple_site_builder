use std::path::PathBuf;

pub fn copy_file_with_mkdir(
  input_path: &PathBuf,
  output_path: &PathBuf,
) -> Result<(), std::io::Error> {
  if let Some(parent_dir) = output_path.parent() {
    std::fs::create_dir_all(parent_dir)?;
  }
  let data = std::fs::read(input_path)?;
  std::fs::write(output_path, &data)?;
  Ok(())
}
