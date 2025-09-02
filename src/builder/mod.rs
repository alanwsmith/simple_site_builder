use crate::config::Config;
use anyhow::Result;
use chrono::{DateTime, Local};
use tokio::sync::mpsc::Receiver;
use tower_livereload::Reloader;
use tracing::info;

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

  pub fn build_site(&self) -> Result<()> {
    info!("Building site");
    //  let _ = &self.transform_files()?;
    // let _ = &self.copy_files()?;
    info!("Reloading browser");
    let _ = &self.reloader.reload();
    Ok(())
  }

  pub async fn start(&mut self) -> Result<()> {
    info!("Starting builder");
    let _ = &self.build_site();
    while (self.rx.recv().await).is_some() {
      let _ = &self.build_site();
    }
    Ok(())
  }
}
