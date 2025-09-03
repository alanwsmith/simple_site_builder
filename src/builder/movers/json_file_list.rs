use std::{ffi::OsStr, path::PathBuf};

pub fn json_file_list(
  source_paths: Vec<PathBuf>
) -> Vec<PathBuf> {
  source_paths
    .iter()
    .filter(|path| path.extension().is_some())
    .filter(|path| {
      let ext = path.extension().unwrap();
      ext == OsStr::new("json")
    })
    .cloned()
    .collect()
}

#[cfg(test)]
mod test {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn html_file_list_test() {
    let source_list = vec![
      "data.json",
      "sub-dir/data.json",
      "skip.html",
      "skip-no-extension",
      "_include-underscore-data.json",
      "_include-underscore-dir/data.json",
    ];
    let left_list = vec![
      "data.json",
      "sub-dir/data.json",
      "_include-underscore-data.json",
      "_include-underscore-dir/data.json",
    ];
    let source_paths =
      source_list.iter().map(PathBuf::from).collect();
    let left: Vec<PathBuf> =
      left_list.iter().map(PathBuf::from).collect();
    let right = json_file_list(source_paths);
    assert_eq!(left, right);
  }
}
