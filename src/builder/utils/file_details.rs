use serde::Serialize;
use std::path::Path;
use std::path::PathBuf;
use tokio::sync::SemaphorePermit;

#[derive(Debug, PartialEq, Serialize)]
pub enum FileMoveType {
  Copy,
  Skip,
  Transform,
  TransformHtml,
  TransformTxt,
  TransformCSS,
  TransformAndMinifyJavaScript,
  CopyAndMinifyJavaScript,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct FileDetails {
  pub extension: Option<String>,
  pub file_move_type: FileMoveType,
  pub path_parts: Vec<String>,
  pub path_part_strings: Vec<String>,
  pub folder: PathBuf,
  pub folder_parts: Vec<String>,
  pub input_path: PathBuf,
  pub name: PathBuf,
  pub output_folder: Option<PathBuf>,
  pub output_name: Option<PathBuf>,
  pub parent_folder: Option<String>,
  pub parent: Option<PathBuf>,
  pub stem: PathBuf,
}

impl FileDetails {
  pub fn new(input_path: &Path) -> FileDetails {
    let extension =
      FileDetails::get_extension(input_path);
    let file_move_type =
      FileDetails::get_file_move_type(input_path);
    let path_parts =
      FileDetails::get_path_parts(input_path);
    let path_part_strings =
      FileDetails::get_path_part_strings(input_path);
    let folder = FileDetails::get_input_dir(input_path);
    let folder_parts =
      FileDetails::get_folder_parts(input_path);
    let name = FileDetails::get_input_name(input_path);
    let output_folder =
      FileDetails::get_output_dir(input_path);
    let output_name =
      FileDetails::get_output_name(input_path);
    let parent_folder =
      input_path.parent().and_then(|p| {
        p.file_stem()
          .map(|f| f.to_string_lossy().to_string())
      });
    let stem = FileDetails::get_input_stem(input_path);
    FileDetails {
      extension,
      file_move_type,
      folder,
      folder_parts,
      input_path: input_path.to_path_buf(),
      name,
      output_folder,
      output_name,
      parent: input_path
        .parent()
        .map(|p| p.to_path_buf()),
      parent_folder,
      path_part_strings,
      path_parts,
      stem,
    }
  }

  pub fn dir_path_strings(&self) -> Vec<String> {
    if let Some(parent) = self.input_path.parent() {
      let mut items: Vec<String> = parent
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();
      items.push(self.stem.to_string_lossy().to_string());
      items
    } else {
      vec![]
    }
  }

  pub fn get_extension(
    input_path: &Path
  ) -> Option<String> {
    input_path
      .extension()
      .map(|ext| ext.display().to_string())
  }

  pub fn get_input_name(input_path: &Path) -> PathBuf {
    input_path.file_name().unwrap().into()
  }

  pub fn get_input_dir(input_path: &Path) -> PathBuf {
    input_path.parent().unwrap().into()
  }

  pub fn get_input_stem(input_path: &Path) -> PathBuf {
    input_path.file_stem().unwrap().into()
  }

  pub fn get_file_move_type(
    input_path: &Path
  ) -> FileMoveType {
    let transforms = &["html", "txt", "md"];
    if input_path
      .iter()
      .any(|part| part.to_str().unwrap().starts_with("_"))
    {
      return FileMoveType::Skip;
    }
    if let Some(ext) = input_path.extension() {
      if let Some(stem) = input_path.file_stem() {
        if let Some(ext2) =
          PathBuf::from(stem).extension()
        {
          if ext2 == "on" {
            return FileMoveType::Transform;
          } else if ext2 == "off" {
            return FileMoveType::Copy;
          }
        }
      }
      if transforms.contains(&ext.to_str().unwrap()) {
        return FileMoveType::Transform;
      }
    }
    FileMoveType::Copy
  }

  pub fn get_path_part_strings(
    input_path: &Path
  ) -> Vec<String> {
    let mut parts = input_path
      .ancestors()
      .map(|part| part.to_string_lossy().to_string())
      .collect::<Vec<String>>();
    let _ = parts.pop();
    parts.reverse();
    parts
  }

  pub fn get_path_parts(
    input_path: &Path
  ) -> Vec<String> {
    input_path
      .iter()
      .map(|part| part.to_string_lossy().to_string())
      .collect::<Vec<String>>()
  }

  pub fn get_folder_parts(
    input_path: &Path
  ) -> Vec<String> {
    let mut initial = input_path
      .iter()
      .map(|part| part.to_string_lossy().to_string())
      .collect::<Vec<String>>();
    let _ = initial.pop();
    initial
  }

  // pub fn get_output_dir(
  //   input_path: &Path
  // ) -> Option<PathBuf> {
  //   if input_path
  //     .iter()
  //     .any(|part| part.to_str().unwrap().starts_with("_"))
  //   {
  //     None
  //   } else {
  //     Some(PathBuf::from(
  //       input_path.parent().unwrap().to_str().unwrap(),
  //     ))
  //   }
  // }

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
            Some(parent_path)
          }
        }
        None => Some(parent_path),
      }
    }
  }

  // pub fn get_output_name(
  //   input_path: &Path
  // ) -> Option<PathBuf> {
  //   if input_path
  //     .iter()
  //     .any(|part| part.to_str().unwrap().starts_with("_"))
  //   {
  //     None
  //   } else {
  //     Some(input_path.file_name().unwrap().into())
  //   }
  // }

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

  pub fn sort_key(&self) -> (String, String) {
    (
      self.folder.display().to_string(),
      self.name.display().to_string(),
    )
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  // #[rstest]
  // #[case(
  //   "html",
  //   "index.html",
  //   "",
  //   "index.html",
  //   "",
  //   "index.html",
  //   FileMoveType::TransformHtml,
  //   "index"
  // )]
  // fn file_details_integration_test(
  //   #[case] extension: &str,
  //   #[case] input_path: &str,
  //   #[case] folder: &str,
  //   #[case] name: &str,
  //   #[case] output_folder: &str,
  //   #[case] output_name: &str,
  //   #[case] file_move_type: FileMoveType,
  //   #[case] stem: &str,
  // ) {
  //   let left = FileDetails {
  //     extension: Some(extension.to_string()),
  //     folder: PathBuf::from(folder),
  //     name: PathBuf::from(name),
  //     output_folder: Some(PathBuf::from(output_folder)),
  //     output_name: Some(PathBuf::from(output_name)),
  //     stem: PathBuf::from(stem),
  //     file_move_type,
  //     input_path: PathBuf::from(input_path),
  //   };
  //   let right =
  //     FileDetails::new(&PathBuf::from(input_path));
  //   assert_eq!(left, right);
  // }

  #[rstest]
  #[case("test.js", FileMoveType::Copy)]
  #[case("test.html", FileMoveType::Transform)]
  #[case("test.on.js", FileMoveType::Transform)]
  #[case("test.off.html", FileMoveType::Copy)]
  fn get_file_move_type_test(
    #[case] input_path: &str,
    #[case] left: FileMoveType,
  ) {
    let right = FileDetails::get_file_move_type(
      &PathBuf::from(input_path),
    );
    assert_eq!(left, right);
  }

  #[rstest]
  #[case("index.html", "html")]
  #[case("data.json", "json")]
  fn get_extension_test(
    #[case] input_path: &str,
    #[case] target: &str,
  ) {
    let expected = Some(target.to_string());
    let got = FileDetails::get_extension(&PathBuf::from(
      input_path,
    ));
    assert_eq!(expected, got);
  }

  #[rstest]
  #[case("no_extension", None)]
  fn get_extension_none(
    #[case] input_path: &str,
    #[case] expected: Option<String>,
  ) {
    let got = FileDetails::get_extension(&PathBuf::from(
      input_path,
    ));
    assert_eq!(expected, got);
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

  // NOTE: This is old when files moved directly
  // instead of making new like `about.html` to
  // `about.html` instead of to `about/index.html`
  // #[rstest]
  // #[case("index.html", "index.html")]
  // #[case("subdir/index.html", "index.html")]
  // #[case("test.json", "test.json")]
  // #[case("subdir/test.json", "test.json")]
  // #[case(".dotfile", ".dotfile")]
  // #[case(".dotdir/test.json", "test.json")]
  // #[case("about.html", "about.html")]
  // #[case("subdir/about.html", "about.html")]
  // #[case(".subdir/about.html", "about.html")]
  // #[case("subdir/.about.html", ".about.html")]
  // fn get_output_name_to_move(
  //   #[case] input_path: &str,
  //   #[case] output_name: &str,
  // ) {
  //   let expected = Some(PathBuf::from(&output_name));
  //   let got = FileDetails::get_output_name(
  //     &PathBuf::from(input_path),
  //   );
  //   assert_eq!(expected, got);
  // }

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
  fn solo_get_output_name_to_move(
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

  // THIS IS FOR PRIOR VERSION WHEN NOT
  // MAKING `item.html` turn into
  // `item/index.html`
  // #[rstest]
  // #[case("index.html", "")]
  // #[case("sub-dir/index.html", "sub-dir")]
  // #[case("about.html", "")]
  // #[case("valid-dir/about.html", "valid-dir")]
  // fn get_output_dir_valid_test_html(
  //   #[case] input_path: &str,
  //   #[case] target: &str,
  // ) {
  //   let expected = Some(PathBuf::from(target));
  //   let got = FileDetails::get_output_dir(
  //     &PathBuf::from(input_path),
  //   );
  //   assert_eq!(expected, got);
  // }

  #[rstest]
  #[case("index.html", "")]
  #[case("sub-dir/index.html", "sub-dir")]
  #[case("about.html", "about")]
  #[case("valid-dir/about.html", "valid-dir/about")]
  fn get_output_dir_valid_test_html(
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
  #[case("data.json", "")]
  #[case(".data.json", "")]
  #[case("sub-dir/data.json", "sub-dir")]
  #[case("sub-dir/.data.json", "sub-dir")]
  #[case(".sub-dir/data.json", ".sub-dir")]
  fn get_output_dir_valid_test_non_html(
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
  #[case("index.html", vec![])]
  #[case("parent/index.html", vec!["parent".to_string()])]
  #[case("grandparent/parent/index.html", vec!["grandparent".to_string(), "parent".to_string()])]
  fn get_folder_parts_test(
    #[case] input_path: &str,
    #[case] expected: Vec<String>,
  ) {
    let got = FileDetails::get_folder_parts(
      &PathBuf::from(input_path),
    );
    assert_eq!(expected, got);
  }

  #[rstest]
  #[case("_skipped.html", None)]
  #[case("_skipped-dir/index.html", None)]
  #[case("_skipped-dir/about.html", None)]
  #[case("valid-dir/_skip.html", None)]
  #[case(".valid-dir/_skip.html", None)]
  #[case("valid-dir/_skip-dir/file.html", None)]
  #[case("_skipped.json", None)]
  #[case("_skipped-dir/skipped.json", None)]
  #[case("valid-dir/_skipped.json", None)]
  #[case("valid-dir/_skip-dir/file.json", None)]
  fn get_output_dir_skipped_test(
    #[case] input_path: &str,
    #[case] expected: Option<PathBuf>,
  ) {
    let got = FileDetails::get_output_dir(
      &PathBuf::from(input_path),
    );
    assert_eq!(expected, got);
  }

  // #[rstest]
  // #[case(&PathBuf::from("index.html"), vec![])]
  // #[case(&PathBuf::from("test/index.html"), vec!["test".to_string()])]
  // fn solo_dir_path_strings_test(
  //   #[case] given: &PathBuf,
  //   #[case] expected: Vec<String>,
  // ) {
  //   let got = FileDetails::dir_path_strings(given);
  //   assert_eq!(expected, got);
  // }

  // #[rstest]
  // #[case("index.html", FileMoveType::TransformHtml)]
  // #[case("data.json", FileMoveType::Copy)]
  // #[case("no-extension", FileMoveType::Copy)]
  // #[case(".dot-file", FileMoveType::Copy)]
  // #[case(".dot.html", FileMoveType::TransformHtml)]
  // #[case("_skip.html", FileMoveType::Skip)]
  // #[case("_skip-dir/file.html", FileMoveType::Skip)]
  // #[case("valid-dir/_skip.html", FileMoveType::Skip)]
  // #[case(
  //   "valid-dir/_skip-dir/file.html",
  //   FileMoveType::Skip
  // )]
  // #[case(
  //   "subdir/index.html",
  //   FileMoveType::TransformHtml
  // )]
  // #[case("about.html", FileMoveType::TransformHtml)]
  // #[case(
  //   "subdir/about.html",
  //   FileMoveType::TransformHtml
  // )]
  // // #[case("index.md", FileMoveType::TransformMarkdown)]
  // // #[case("about.md", FileMoveType::TransformMarkdown)]
  // // #[case("subdir/index.md", FileMoveType::TransformMarkdown)]
  // // #[case("subdir/about.md", FileMoveType::TransformMarkdown)]
  // fn file_move_type_test(
  //   #[case] input_path: &str,
  //   #[case] expected: FileMoveType,
  // ) {
  //   let got = FileDetails::get_file_move_type(
  //     &PathBuf::from(input_path),
  //   );
  //   assert_eq!(expected, got)
  // }

  //
}
