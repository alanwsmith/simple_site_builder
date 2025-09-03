use std::{ffi::OsStr, path::PathBuf};

pub fn txt_file_list(
  source_paths: Vec<PathBuf>
) -> Vec<PathBuf> {
  source_paths
    .iter()
    .filter(|path| path.extension().is_some())
    .filter(|path| {
      let ext = path.extension().unwrap();
      ext == OsStr::new("txt")
    })
    .cloned()
    .collect()
}

#[cfg(test)]
mod test {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn txt_file_list_test() {
    let source_list = vec![
      "file.txt",
      "sub-dir/file.txt",
      "skip.html",
      "skip-no-extension",
      "_include-underscore-file.txt",
      "_include-underscore-dir/file.txt",
    ];
    let left_list = vec![
      "file.txt",
      "sub-dir/file.txt",
      "_include-underscore-file.txt",
      "_include-underscore-dir/file.txt",
    ];
    let source_paths =
      source_list.iter().map(PathBuf::from).collect();
    let left: Vec<PathBuf> =
      left_list.iter().map(PathBuf::from).collect();
    let right = txt_file_list(source_paths);
    assert_eq!(left, right);
  }
}
