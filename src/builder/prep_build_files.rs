use super::*;
use anyhow::Result;
use itertools::Itertools;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

impl Builder {
  pub fn prep_build_files(
    &self,
    source_dir: &PathBuf,
    dest_dir: &PathBuf,
  ) -> Result<()> {
    let file_prep_scripts: Vec<String> =
      get_files_in_dir(
        &self.config.file_prep_scripts_dir(),
      )?
      .iter()
      .map(|p| p.to_string_lossy().to_string())
      .collect();
    for entry in WalkDir::new(source_dir) {
      let source_path = entry?.into_path();
      let dest_path = dest_dir.join(
        source_path.strip_prefix(source_dir).unwrap(),
      );
      if source_path.is_dir() {
        fs::create_dir_all(dest_path)?;
      } else {
        let data = std::fs::read(&source_path)?;
        dbg!("-----------------");
        if let Some(ext) = &source_path.extension() {
          if self
            .config
            .file_prep_extensions()
            .contains(&ext.to_string_lossy().to_string())
          {
            for script in
              file_prep_scripts.iter().sorted()
            {
              // TODO: Show error message if
              // script fails.
              let prepped_content =
                run_script(script, &data)?;
              dbg!(prepped_content);
            }
          }
        }

        std::fs::write(dest_path, &data)?;
      }
    }
    Ok(())
  }
}
