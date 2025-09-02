use std::{ffi::OsStr, path::PathBuf};

pub fn html_file_list(
  source_paths: Vec<PathBuf>
) -> Vec<PathBuf> {
  source_paths
    .iter()
    .filter(|path| {
      !path
        .iter()
        .any(|part| part.to_str().unwrap().starts_with("_"))
    })
    .filter(|path| path.extension().is_some())
    .filter(|path| {
      let ext = path.extension().unwrap();
      ext == OsStr::new("html")
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
      "index.html",
      "sub-dir/file.html",
      "skip.txt",
      "no-extension",
      "_skip-dir/text.html",
      "sub-dir/_skip.html",
    ];
    let left_list = vec!["index.html", "sub-dir/file.html"];
    let source_paths =
      source_list.iter().map(PathBuf::from).collect();
    let left: Vec<PathBuf> =
      left_list.iter().map(PathBuf::from).collect();
    let right = html_file_list(source_paths);
    assert_eq!(left, right);
  }
}
