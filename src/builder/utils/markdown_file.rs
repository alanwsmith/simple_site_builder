use markdown::{CompileOptions, Options};
use std::path::PathBuf;

pub fn markdown_file(file_path: &str) -> String {
  let path = PathBuf::from("content").join(file_path);
  if let Ok(content) = std::fs::read_to_string(path) {
    match markdown::to_html_with_options(
      &content,
      &Options {
        compile: CompileOptions {
          allow_dangerous_html: true,
          ..CompileOptions::default()
        },
        ..Options::default()
      },
    ) {
      Ok(parsed) => parsed.to_string(),
      Err(error) => {
        format!(
          "Could not process markdown file: {}",
          error
        )
      }
    }
  } else {
    format!(
      "Could not load file to process markdown: {}",
      file_path
    )
  }
}
