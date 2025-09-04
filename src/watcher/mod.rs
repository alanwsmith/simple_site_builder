use crate::Config;
use anyhow::Result;
use chrono::{DateTime, Local};
use itertools::Itertools;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tracing::info;
use watchexec::Watchexec;
use watchexec_events::filekind::*;
use watchexec_events::{Event, Tag};
use watchexec_signals::Signal;

pub struct Watcher {
  config: Config,
  tx: Sender<DateTime<Local>>,
}

impl Watcher {
  pub fn new(
    config: Config,
    tx: Sender<DateTime<Local>>,
  ) -> Watcher {
    Watcher { config, tx }
  }
  pub async fn start(&self) -> Result<()> {
    info!("Starting watcher");
    let wx = Watchexec::default();
    wx.config.pathset(vec![
      self.config.content_root.display().to_string(),
    ]);
    let tx2 = self.tx.clone();
    wx.config.on_action(move |mut action| {
      let paths = filter_paths(&action.events);
      if !paths.is_empty() {
        let tx3 = tx2.clone();
        tokio::spawn(async move {
          tx3.send(chrono::Local::now()).await.unwrap();
        });
      }
      if action.signals().any(|sig| {
        // action.signals() check required for Ctrl+c
        sig == Signal::Interrupt
          || sig == Signal::Terminate
      }) {
        action.quit();
      }
      action
    });
    let _ = wx.main().await?;
    println!("Watcher stopped.");
    Ok(())
  }
}

fn filter_paths(events: &Arc<[Event]>) -> Vec<PathBuf> {
  events
    .iter()
    .filter(|event| {
      event.tags.iter().any(|tag| {
        matches!(
          tag,
          Tag::FileEventKind(FileEventKind::Modify(
            ModifyKind::Data(DataChange::Content,)
          ),)
        ) || matches!(
          tag,
          Tag::FileEventKind(FileEventKind::Create(
            CreateKind::File
          ),)
        )
      })
    })
    .filter_map(|event| {
      event.tags.iter().find_map(|tag| {
        if let Tag::Path { path, .. } = tag {
          for component in path.components() {
            if let std::path::Component::Normal(part) =
              component
            {
              if part
                .display()
                .to_string()
                .starts_with(".")
              {
                return None;
              }
            }
          }
          if let Some(file_name_path) = path.file_name() {
            let file_name =
              file_name_path.display().to_string();
            if file_name.ends_with("~") {
              return None;
            }
          };
          Some(path.to_path_buf())
        } else {
          None
        }
      })
    })
    .unique()
    .collect()
}
