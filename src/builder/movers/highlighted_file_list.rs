use std::{ffi::OsStr, path::PathBuf};

pub fn highlighted_file_list(
  source_paths: Vec<PathBuf>
) -> Vec<PathBuf> {
  source_paths
    .iter()
    .filter(|path| path.extension().is_some())
    .filter(|path| {
      let ext = path.extension().unwrap();
      // TODO: pull this into a config
      ext == OsStr::new("rs")
        || ext == OsStr::new("css")
        || ext == OsStr::new("html")
        || ext == OsStr::new("java")
        || ext == OsStr::new("js")
        || ext == OsStr::new("json")
        || ext == OsStr::new("py")
        || ext == OsStr::new("txt")
    })
    .cloned()
    .collect()
}

#[cfg(test)]
mod test {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn highlighted_file_list_test() {
    let source_list = vec![
      "skip-image.jpg",
      "skip-image.jpeg",
      "skip-image.png",
      "skip-image.gif",
      "skip-image.avif",
      "skip-image.webm",
      "skip-image.webp",
      "skip-image.webm",
      "yes.html",
      "yes.js",
      "yes.rs",
      "sub-dir/yes.rs",
      "skip-no-extension",
      "_include-underscore-yes.py",
      "_include-underscore-dir/yes.py",
    ];
    let left_list = vec![
      "yes.html",
      "yes.js",
      "yes.rs",
      "sub-dir/yes.rs",
      "_include-underscore-yes.py",
      "_include-underscore-dir/yes.py",
    ];
    let source_paths =
      source_list.iter().map(PathBuf::from).collect();
    let left: Vec<PathBuf> =
      left_list.iter().map(PathBuf::from).collect();
    let right = highlighted_file_list(source_paths);
    assert_eq!(left, right);
  }
}
