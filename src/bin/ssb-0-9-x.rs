use anyhow::{Result, anyhow};
use chrono::{DateTime, Local};
use itertools::Itertools;
use notify::{EventKind, RecursiveMode};
use notify_debouncer_full::{
  DebounceEventResult, new_debouncer,
};
use port_check::free_local_port_in_range;
use simple_site_builder::builder::utils::DataNode;
use simple_site_builder::*;
use std::path::PathBuf;
use tokio::sync::mpsc;
use tokio::time::Duration;
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

  tokio::spawn(async move {
    let _ =
      file_watcher(&PathBuf::from("content"), tx).await;
  });

  let mut builder = Builder::new(
    config.clone(),
    reloader,
    rx,
    port,
    DataNode::new(),
  );

  tokio::spawn(async move {
    let _ = builder.start().await;
  });

  // let mut site_builder =
  //   SiteBuilder::new(rx_file_change, reloader);
  // tokio::spawn(async move {
  //   let _ = site_builder.start().await;
  // });

  let server = Server::new(config.clone(), port);
  let server_handle = tokio::spawn(async move {
    let _ = server.start(live_reload).await;
  });

  server_handle.await.unwrap();

  // let mut builder = Builder::new(
  //   config.clone(),
  //   reloader,
  //   rx,
  //   port,
  //   DataNode::new(),
  // );

  // let builder_handle = tokio::spawn(async move {
  //   let _ = builder.start().await;
  // });

  // let watcher = Watcher::new(config.clone(), tx);
  // let _ = watcher.start().await;

  // server_handle.abort();
  // builder_handle.abort();

  Ok(())
}

fn find_port() -> Result<u16> {
  free_local_port_in_range(5444..=6000)
    .ok_or(anyhow!("Could not find port"))
}

async fn file_watcher(
  source_docroot: &PathBuf,
  tx_file_change: mpsc::Sender<
    chrono::DateTime<chrono::Local>,
  >,
) -> Result<()> {
  println!("starting watcher");
  let (watcher_internal_tx, mut watcher_internal_rx) =
    mpsc::channel::<chrono::DateTime<chrono::Local>>(1);
  let mut debouncer = new_debouncer(
    Duration::from_millis(200),
    None,
    move |result: DebounceEventResult| match result {
      Ok(events) => {
        let paths: Vec<_> = events
          .iter()
          .filter_map(|e| match e.event.kind {
            EventKind::Create(..) => {
              Some(e.paths.clone())
            }
            EventKind::Modify(..) => {
              Some(e.paths.clone())
            }
            EventKind::Remove(..) => {
              Some(e.paths.clone())
            }
            _ => None,
          })
          .flatten()
          .unique()
          .collect();
        if !paths.is_empty() {
          let tx = watcher_internal_tx.clone();
          futures::executor::block_on(async {
            if let Err(e) =
              tx.send(chrono::prelude::Local::now()).await
            {
              println!(
                "Error sending event result: {:?}",
                e
              );
            }
          })
        }
      }
      Err(e) => println!("{:?}", e),
    },
  )
  .unwrap();
  debouncer
    .watch(source_docroot, RecursiveMode::Recursive)
    .unwrap();
  // This is the way I'm keeping the debouncer
  // alive. Another approach I've seen is
  // just to throw a `loop {}` at this position
  // instead, but that feels weird since it
  // feels like that would burn cpu. So,
  // I'm catching events here and then
  // trigging another send.
  while watcher_internal_rx.recv().await.is_some() {
    //println!("saw change");
    let tx = tx_file_change.clone();
    if let Err(e) =
      tx.send(chrono::prelude::Local::now()).await
    {
      println!("Error sending event result: {:?}", e);
    }
  }
  Ok(())
}
