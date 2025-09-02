use std::path::PathBuf;

pub struct HtmlFileDetails {
  input_path: PathBuf,
  output_root: PathBuf,
}

impl HtmlFileDetails {
  pub fn new(
    input_path: &PathBuf,
    output_root: &PathBuf,
  ) -> HtmlFileDetails {
    HtmlFileDetails {
      input_path: input_path.clone(),
      output_root: output_root.clone(),
    }
  }

  pub fn output_path(&self) -> PathBuf {
    let file_name = &self
      .input_path
      .file_name()
      .unwrap()
      .to_str()
      .unwrap();
    if *file_name == "index.html" {
      self.output_root.join(&self.input_path)
    } else {
      let file_stem =
        &self.input_path.file_stem().unwrap();
      let mut input_path_clone = self.input_path.clone();
      input_path_clone.pop();
      self
        .output_root
        .join(input_path_clone)
        .join(file_stem)
        .join("index.html")
    }
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
  #[case("index.html", "out", "out/index.html")]
  #[case("about.html", "out", "out/about/index.html")]
  #[case(
    "sub-path/widget.html",
    "out",
    "out/sub-path/widget/index.html"
  )]
  fn output_path_test(
    #[case] input_path: &str,
    #[case] output_root: &str,
    #[case] output_path: &str,
  ) {
    let transform = HtmlFileDetails::new(
      &PathBuf::from(input_path),
      &PathBuf::from(output_root),
    );
    let left = PathBuf::from(output_path);
    let right = transform.output_path();
    assert_eq!(left, right);
  }
}
