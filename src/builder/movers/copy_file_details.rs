use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CopyFileDetails {
  content_root: PathBuf,
  input_file: PathBuf,
  output_root: PathBuf,
}

impl CopyFileDetails {
  pub fn new(
    content_root: &Path,
    input_file: &Path,
    output_root: &Path,
  ) -> CopyFileDetails {
    CopyFileDetails {
      content_root: content_root.to_path_buf(),
      input_file: input_file.to_path_buf(),
      output_root: output_root.to_path_buf(),
    }
  }

  pub fn input_path(&self) -> PathBuf {
    self.content_root.join(&self.input_file)
  }

  pub fn output_path(&self) -> PathBuf {
    self.output_root.join(&self.input_file)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  // Reminder: Not testing absolute paths because
  // they aren't sent in this implementation
  #[case(
    "content",
    "file.txt",
    "out",
    "content/file.txt",
    "out/file.txt"
  )]
  #[case(
    "content",
    "sub-path/file.txt",
    "out",
    "content/sub-path/file.txt",
    "out/sub-path/file.txt"
  )]
  fn path_tests(
    #[case] content_root: &str,
    #[case] input_file: &str,
    #[case] output_root: &str,
    #[case] input_path: &str,
    #[case] output_path: &str,
  ) {
    let details = CopyFileDetails::new(
      &PathBuf::from(content_root),
      &PathBuf::from(input_file),
      &PathBuf::from(output_root),
    );
    let left_in = PathBuf::from(input_path);
    let right_in = details.input_path();
    assert_eq!(left_in, right_in);
    let left_out = PathBuf::from(output_path);
    let right_out = details.output_path();
    assert_eq!(left_out, right_out);
  }

  //
}
