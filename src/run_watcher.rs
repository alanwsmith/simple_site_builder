use anyhow::Result;
use chrono::{DateTime, Local};
use itertools::Itertools;
use std::path::PathBuf;
use std::sync::Arc;
// use std::time::Duration;
use tokio::sync::mpsc::Sender;
use watchexec::Watchexec;
use watchexec_events::filekind::*;
use watchexec_events::{Event, Tag};
use watchexec_signals::Signal;

pub async fn run_watcher(tx: Sender<DateTime<Local>>) -> Result<()> {
    println!("Starting watcher");
    tx.send(chrono::Local::now()).await.unwrap();
    let wx = Watchexec::default();
    // wx.config.pathset(vec!["content", "data"]);
    //wx.config.throttle(Duration::from_millis(200));
    wx.config.pathset(vec!["content", "data", "scripts"]);
    wx.config.on_action(move |mut action| {
        let paths = get_paths(&action.events);
        // dbg!(&paths);
        if paths.len() > 0 {
            let tx2 = tx.clone();
            tokio::spawn(async move {
                tx2.send(chrono::Local::now()).await.unwrap();
            });
        }
        if action
            .signals()
            .any(|sig| sig == Signal::Interrupt || sig == Signal::Terminate)
        {
            action.quit(); // Needed for Ctrl+c
        }
        action
    });
    let _ = wx.main().await?;
    println!("Watcher stopped.");
    Ok(())
}

fn get_paths(events: &Arc<[Event]>) -> Vec<PathBuf> {
    events
        .iter()
        .filter(|event| {
            event
                .tags
                .iter()
                .find(|tag| {
                    if let Tag::FileEventKind(kind) = &tag {
                        if let FileEventKind::Modify(mod_kind) = kind {
                            if let ModifyKind::Data(change) = mod_kind {
                                if let DataChange::Content = change {
                                    return true;
                                }
                            }
                        }
                    };
                    false
                })
                .is_some()
        })
        .filter_map(|event| {
            event.tags.iter().find_map(|tag| {
                if let Tag::Path { path, .. } = tag {
                    for component in path.components() {
                        if let std::path::Component::Normal(part) = component {
                            if part.display().to_string().starts_with(".") {
                                return None;
                            }
                        }
                    }
                    if let Some(file_name_path) = path.file_name() {
                        let file_name = file_name_path.display().to_string();
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
