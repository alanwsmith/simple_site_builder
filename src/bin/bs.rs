use bs_site_builder::config::*;
use bs_site_builder::logger::*;
use bs_site_builder::server::*;
use chrono::{DateTime, Local};
use std::path::PathBuf;
use tokio::sync::mpsc;
use tower_livereload::LiveReloadLayer;
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

  let live_reload = LiveReloadLayer::new();
  // let reloader = livereload.reloader();
  // let (tx, rx) = mpsc::channel::<DateTime<Local>>(32);

  let server = Server::new(config.clone());
  let server_handle = tokio::spawn(async move {
    let _ = server.start(live_reload).await;
  });

  server_handle.abort();
}
