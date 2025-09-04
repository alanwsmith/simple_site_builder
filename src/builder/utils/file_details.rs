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
    // REMINDER: only files are send in so
    // unwrapping directly can be used
    let input_dir = input_path.parent().unwrap().into();
    let input_name =
      FileDetails::get_input_name(input_path);
    let output_dir =
      Some(input_path.parent().unwrap().into());
    let output_name =
      Some(input_path.file_name().unwrap().into());
    let move_type = FileMoveType::Transform;
    FileDetails {
      input_dir,
      input_name,
      output_dir,
      output_name,
      move_type,
    }
  }

  // pub fn get_input_name(input_path: &Path) -> PathBuf {
  //   FileDetails::get_input_name_dev(input_path)
  // }

  pub fn get_input_name(input_path: &Path) -> PathBuf {
    input_path.file_name().unwrap().into()
  }

  pub fn get_input_dir(input_path: &Path) -> PathBuf {
    input_path.parent().unwrap().into()
  }

  // TODO: This is really for output name
  // pub fn get_input_name_dev2(
  //   input_path: &Path
  // ) -> PathBuf {
  //   match input_path.extension() {
  //     Some(ext) => {
  //       if ext.to_str().unwrap() == "html" {
  //         if input_path
  //           .file_stem()
  //           .unwrap()
  //           .to_str()
  //           .unwrap()
  //           != "index"
  //         {
  //           PathBuf::from("index.html")
  //         } else {
  //           input_path.file_name().unwrap().into()
  //         }
  //       } else {
  //         input_path.file_name().unwrap().into()
  //       }
  //     }
  //     None => input_path.file_name().unwrap().into(),
  //   }
  // }

  //
}

#[derive(Debug, PartialEq)]
pub enum FileMoveType {
  Copy,
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
  fn input_name_test(
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
  fn input_dir_test(
    #[case] input_path: &str,
    #[case] target: &str,
  ) {
    let expected = PathBuf::from(target);
    let got = FileDetails::get_input_dir(&PathBuf::from(
      input_path,
    ));
    assert_eq!(expected, got);
  }

  // #[rstest]
  // fn dev(
  //   #[case] input_path: &str,
  //   #[case] input_name: &str,
  // ) {
  //   let expected = PathBuf::from(&input_name);
  //   let got = FileDetails::get_input_name_dev(
  //     &PathBuf::from(input_path),
  //   );
  //   assert_eq!(expected, got);
  // }

  // #[rstest]
  // #[case("about.html", "index.html")]
  // fn dev2(
  //   #[case] input_path: &str,
  //   #[case] input_name: &str,
  // ) {
  //   let expected = PathBuf::from(&input_name);
  //   let got = FileDetails::get_input_name_dev2(
  //     &PathBuf::from(input_path),
  //   );
  //   assert_eq!(expected, got);
  // }

  //
}
