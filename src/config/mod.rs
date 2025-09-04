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
      content_root,
      logs_root,
      output_root,
      debug,
    }
  }

  pub fn json_logs(&self) -> PathBuf {
    self.logs_root.join("json")
  }

  pub fn txt_logs(&self) -> PathBuf {
    self.logs_root.join("txt")
  }
}
