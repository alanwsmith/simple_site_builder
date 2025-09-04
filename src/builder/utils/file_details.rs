use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct FileDetails {
  input_dir: PathBuf,
  input_name: PathBuf,
  output_dir: Option<PathBuf>,
  output_name: Option<PathBuf>,
  move_type: FileMoveType,
}

impl FileDetails {
  pub fn new(input_path: &Path) -> FileDetails {
    let input_dir =
      FileDetails::get_input_dir(input_path);
    let input_name =
      FileDetails::get_input_name(input_path);
    let output_dir =
      FileDetails::get_output_dir(input_path);
    let output_name =
      FileDetails::get_output_name(input_path);
    let move_type =
      FileDetails::get_move_type(input_path);
    FileDetails {
      input_dir,
      input_name,
      output_dir,
      output_name,
      move_type,
    }
  }

  pub fn get_input_name(input_path: &Path) -> PathBuf {
    input_path.file_name().unwrap().into()
  }

  pub fn get_input_dir(input_path: &Path) -> PathBuf {
    input_path.parent().unwrap().into()
  }

  // TODO:
  pub fn get_move_type(
    input_path: &Path
  ) -> FileMoveType {
    FileMoveType::Transform
  }

  pub fn get_output_dir(
    input_path: &Path
  ) -> Option<PathBuf> {
    if input_path
      .iter()
      .any(|part| part.to_str().unwrap().starts_with("_"))
    {
      None
    } else {
      let file_stem =
        input_path.file_stem().unwrap().to_str().unwrap();
      let parent_path = PathBuf::from(
        input_path.parent().unwrap().to_str().unwrap(),
      );
      match input_path.extension() {
        Some(ext) => {
          if ext.to_str().unwrap() == "html" {
            if input_path
              .file_stem()
              .unwrap()
              .to_str()
              .unwrap()
              != "index"
            {
              Some(parent_path.join(file_stem))
            } else {
              Some(parent_path)
            }
          } else {
            Some(input_path.file_name().unwrap().into())
          }
        }
        None => {
          Some(input_path.file_name().unwrap().into())
        }
      }
    }
  }

  pub fn get_output_name(
    input_path: &Path
  ) -> Option<PathBuf> {
    if input_path
      .iter()
      .any(|part| part.to_str().unwrap().starts_with("_"))
    {
      None
    } else {
      match input_path.extension() {
        Some(ext) => {
          if ext.to_str().unwrap() == "html" {
            Some(PathBuf::from("index.html"))
          } else {
            Some(input_path.file_name().unwrap().into())
          }
        }
        None => {
          Some(input_path.file_name().unwrap().into())
        }
      }
    }
  }

  //
}

#[derive(Debug, PartialEq)]
pub enum FileMoveType {
  Copy,
  Skip,
  Transform,
}

#[cfg(test)]
mod test {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case(
    "index.html",
    "",
    "index.html",
    "",
    "index.html",
    FileMoveType::Transform
  )]
  fn file_details_integration_test(
    #[case] input_path: &str,
    #[case] input_dir: &str,
    #[case] input_name: &str,
    #[case] output_dir: &str,
    #[case] output_name: &str,
    #[case] move_type: FileMoveType,
  ) {
    let left = FileDetails {
      input_dir: PathBuf::from(input_dir),
      input_name: PathBuf::from(input_name),
      output_dir: Some(PathBuf::from(output_dir)),
      output_name: Some(PathBuf::from(output_name)),
      move_type,
    };
    let right =
      FileDetails::new(&PathBuf::from(input_path));
    assert_eq!(left, right);
  }

  #[rstest]
  #[case("index.html", "index.html")]
  #[case("about.html", "about.html")]
  #[case("test.json", "test.json")]
  #[case("no_extension", "no_extension")]
  #[case(".dot-hidden", ".dot-hidden")]
  #[case("_leading_underscore", "_leading_underscore")]
  fn get_input_name_test(
    #[case] input_path: &str,
    #[case] target: &str,
  ) {
    let expected = PathBuf::from(&target);
    let got = FileDetails::get_input_name(
      &PathBuf::from(input_path),
    );
    assert_eq!(expected, got);
  }

  #[rstest]
  #[case("sub-dir/index.html", "sub-dir")]
  #[case("index.html", "")]
  fn get_input_dir_test(
    #[case] input_path: &str,
    #[case] target: &str,
  ) {
    let expected = PathBuf::from(target);
    let got = FileDetails::get_input_dir(&PathBuf::from(
      input_path,
    ));
    assert_eq!(expected, got);
  }

  #[rstest]
  #[case("index.html", "index.html")]
  #[case("subdir/index.html", "index.html")]
  #[case("test.json", "test.json")]
  #[case("subdir/test.json", "test.json")]
  #[case(".dotfile", ".dotfile")]
  #[case(".dotdir/test.json", "test.json")]
  #[case("about.html", "index.html")]
  #[case("subdir/about.html", "index.html")]
  #[case(".subdir/about.html", "index.html")]
  #[case("subdir/.about.html", "index.html")]
  fn get_output_name_to_move(
    #[case] input_path: &str,
    #[case] output_name: &str,
  ) {
    let expected = Some(PathBuf::from(&output_name));
    let got = FileDetails::get_output_name(
      &PathBuf::from(input_path),
    );
    assert_eq!(expected, got);
  }

  #[rstest]
  #[case("_index.html", None)]
  #[case("_skip.html", None)]
  #[case("_skip-dir/index.html", None)]
  #[case("valid-dir/_index.html", None)]
  #[case("valid-dir/_skip-sub-dir/index.html", None)]
  #[case("_skip-dir/.hidden", None)]
  #[case("_skip-dir/.hidden.html", None)]
  fn get_output_name_to_skip(
    #[case] input_path: &str,
    #[case] expected: Option<PathBuf>,
  ) {
    let got = FileDetails::get_output_name(
      &PathBuf::from(input_path),
    );
    assert_eq!(expected, got);
  }

  #[rstest]
  #[case("index.html", "")]
  #[case("sub-dir/index.html", "sub-dir")]
  #[case("about.html", "about")]
  #[case("valid-dir/about.html", "valid-dir/about")]
  fn get_output_dir_valid_test(
    #[case] input_path: &str,
    #[case] target: &str,
  ) {
    let expected = Some(PathBuf::from(target));
    let got = FileDetails::get_output_dir(
      &PathBuf::from(input_path),
    );
    assert_eq!(expected, got);
  }

  #[rstest]
  #[case("_skipped.html", None)]
  #[case("_skipped-dir/index.html", None)]
  #[case("_skipped-dir/about.html", None)]
  #[case("valid-dir/_skip.html", None)]
  #[case("valid-dir/_skip-dir/file.html", None)]
  fn get_output_dir_skipped_test(
    #[case] input_path: &str,
    #[case] expected: Option<PathBuf>,
  ) {
    let got = FileDetails::get_output_dir(
      &PathBuf::from(input_path),
    );
    assert_eq!(expected, got);
  }

  //
}
