use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
  pub content_dir: PathBuf,
  pub output_dir: PathBuf,
  pub logs_dir: PathBuf,
  pub debug: bool,
}

impl Config {
  pub fn new(
    content_dir: PathBuf,
    logs_dir: PathBuf,
    output_dir: PathBuf,
    debug: bool,
  ) -> Config {
    Config {
      content_dir,
      logs_dir,
      output_dir,
      debug,
    }
  }

  pub fn json_logs(&self) -> PathBuf {
    self.logs_dir.join("json")
  }

  pub fn txt_logs(&self) -> PathBuf {
    self.logs_dir.join("txt")
  }
}
