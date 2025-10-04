use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
  pub content_root: PathBuf,
  pub output_root: PathBuf,
  pub logs_root: PathBuf,
  pub debug: bool,
}

impl Config {
  pub fn new(
    content_root: PathBuf,
    logs_root: PathBuf,
    output_root: PathBuf,
    debug: bool,
  ) -> Config {
    Config {
      // TODO: Move these directories over to methods
      content_root,
      logs_root,
      output_root,
      debug,
    }
  }

  pub fn build_files_dir(&self) -> PathBuf {
    PathBuf::from(".build-files")
  }

  // Which file extensions have find and replace
  // run over them.
  pub fn find_and_replce_file_extensions(
    &self
  ) -> Vec<String> {
    [
      "css", "data", "html", "js", "json", "md", "neo",
      "neoj", "txt", "xml",
    ]
    .iter()
    .map(|ext| ext.to_string())
    .collect()
  }

  pub fn find_and_replace_dir(&self) -> PathBuf {
    self.support_dir().join("find-replace")
  }

  pub fn json_logs(&self) -> PathBuf {
    self.logs_root.join("json")
  }

  pub fn support_dir(&self) -> PathBuf {
    PathBuf::from("support")
  }

  pub fn txt_logs(&self) -> PathBuf {
    self.logs_root.join("txt")
  }
}
