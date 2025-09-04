use std::{ffi::OsStr, path::PathBuf};

pub fn copy_file_list(
  source_paths: Vec<PathBuf>
) -> Vec<PathBuf> {
  source_paths
    .iter()
    .filter(|path| {
      !path.iter().any(|part| {
        part.to_str().unwrap().starts_with("_")
      })
    })
    .filter(|path| {
      let ext = path.extension();
      Option::is_none(&ext)
        || ext != Some(OsStr::new("html"))
    })
    .cloned()
    .collect()
}

#[cfg(test)]
mod test {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn copy_file_list_test() {
    let source_list = vec![
      "skip.html",
      "sub-dir/skip.html",
      "include.txt",
      "include.json",
      "include-no-extension",
      "sub-dir/include.txt",
      "sub-dir/include.json",
      "sub-dir/include-no-extension",
    ];
    let left_list = vec![
      "include.txt",
      "include.json",
      "include-no-extension",
      "sub-dir/include.txt",
      "sub-dir/include.json",
      "sub-dir/include-no-extension",
    ];
    let source_paths =
      source_list.iter().map(PathBuf::from).collect();
    let left: Vec<PathBuf> =
      left_list.iter().map(PathBuf::from).collect();
    let right = copy_file_list(source_paths);
    assert_eq!(left, right);
  }
}
