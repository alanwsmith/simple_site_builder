use anyhow::{Result, anyhow};
use bs_site_builder::*;
use chrono::{DateTime, Local};
use port_check::free_local_port_in_range;
use std::path::PathBuf;
use tokio::sync::mpsc;
use tower_livereload::LiveReloadLayer;
use tracing::info;
use tracing::metadata::LevelFilter;

#[tokio::main]
async fn main() -> Result<()> {
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

  let port = find_port()?;
  info!("Found port for web server: {}", port);

  let live_reload = LiveReloadLayer::new();
  let reloader = live_reload.reloader();
  let (tx, rx) = mpsc::channel::<DateTime<Local>>(32);

  let server = Server::new(config.clone(), port);
  let server_handle = tokio::spawn(async move {
    let _ = server.start(live_reload).await;
  });

  let mut builder =
    Builder::new(config.clone(), reloader, rx, port);
  let builder_handle = tokio::spawn(async move {
    let _ = builder.start().await;
  });

  let watcher = Watcher::new(config.clone(), tx);
  let _ = watcher.start().await;

  server_handle.abort();
  builder_handle.abort();

  Ok(())
}

fn find_port() -> Result<u16> {
  free_local_port_in_range(5444..=6000)
    .ok_or(anyhow!("Could not find port"))
}
