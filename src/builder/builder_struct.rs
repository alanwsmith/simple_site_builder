use crate::config::Config;
use anyhow::Result;
use chrono::{DateTime, Local};
use tokio::sync::mpsc::Receiver;
use tower_livereload::Reloader;

pub struct Builder {
  pub config: Config,
  pub reloader: Reloader,
  pub rx: Receiver<DateTime<Local>>,
}

impl Builder {
  pub fn new(
    config: Config,
    reloader: Reloader,
    rx: Receiver<DateTime<Local>>,
  ) -> Builder {
    Builder {
      config,
      reloader,
      rx,
    }
  }

  pub async fn start(&self) -> Result<()> {
    Ok(())
  }
}
