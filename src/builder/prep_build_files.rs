use super::*;
use anyhow::Result;
use std::path::Path;
use std::{fs, vec};
use walkdir::WalkDir;

impl Builder {
  pub fn prep_build_files(
    &self,
    source_dir: &Path,
    dest_dir: &Path,
  ) -> Result<()> {
    let mut replacements: Vec<Vec<String>> = vec![];
    for file in get_files_with_extension(
      &self.config.find_and_replace_dir(),
      "txt",
    )
    .iter()
    {
      fs::read_to_string(file)?.lines().for_each(
        |line| {
          replacements.push(
            line
              .split("|")
              .map(|part| part.to_string())
              .collect(),
          )
        },
      );
    }
    for entry in WalkDir::new(source_dir) {
      let source_path = entry?.into_path();
      let dest_path = dest_dir.join(
        source_path.strip_prefix(source_dir).unwrap(),
      );
      if source_path.is_dir() {
        fs::create_dir_all(dest_path)?;
      } else {
        let data = std::fs::read(&source_path)?;
        if let Some(ext) = &source_path.extension() {
          if self
            .config
            .find_and_replce_file_extensions()
            .contains(&ext.to_string_lossy().to_string())
          {
            let mut output_string =
              String::from_utf8(data.clone())?;
            for r in replacements.iter() {
              if r.len() >= 2 {
                output_string = output_string
                  .replace(r[0].trim(), r[1].trim());
              }
            }
            std::fs::write(&dest_path, &output_string)?;
          } else {
            // For files with other extensions
            std::fs::write(dest_path, &data)?;
          }
        } else {
          // For files without extensions
          std::fs::write(dest_path, &data)?;
        }
      }
    }
    Ok(())
  }
}
