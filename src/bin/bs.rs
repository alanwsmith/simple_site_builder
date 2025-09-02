use bs_site_builder::config::*;
use bs_site_builder::logger::*;
use std::path::PathBuf;
use tracing::info;
use tracing::metadata::LevelFilter;

#[tokio::main]
async fn main() {
  let config = Config::new(
    PathBuf::from("content"),
    PathBuf::from("logs"),
    PathBuf::from("docs"),
    true,
  );

  let _log_guards = Logger::setup()
    .with_stdout(LevelFilter::INFO)
    .to_json_dir(&config.json_logs(), LevelFilter::INFO)
    .to_txt_dir(&config.txt_logs(), LevelFilter::INFO)
    .init();
  info!("Initilizing");

  println!("starting v0.2.0");
}
