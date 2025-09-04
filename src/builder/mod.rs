#![allow(unused)]
pub mod movers;
pub mod utils;

use self::movers::*;
use self::utils::*;
use crate::config::Config;
use anyhow::Result;
use chrono::{DateTime, Local};
use minijinja::Value;
use minijinja::context;
use std::collections::BTreeMap;
use std::fs;
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
    let file_list = file_list(&self.config.content_root);
    let _ = &self.transform_html(&file_list)?;
    // let _ = &self.copy_files()?;
    info!("Reloading browser");
    let _ = &self.reloader.reload();
    Ok(())
  }

  pub fn transform_html(
    &self,
    file_list: &[FileDetails],
  ) -> Result<()> {
    let env = get_env(&self.config.content_root);
    file_list.iter().for_each(|details| {
      dbg!(details);
    });
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

  //
}
