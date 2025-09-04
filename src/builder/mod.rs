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
use std::path::PathBuf;
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
    let _ = clearscreen::clear();
    info!("Building site");
    let file_list = file_list(&self.config.content_root);
    let _ = &self.transform_html(&file_list)?;
    // let _ = &self.copy_files()?;
    info!("Reloading browser");
    let _ = &self.reloader.reload();
    Ok(())
  }

  pub fn load_data(&self) -> Value {
    let mut data_map: BTreeMap<String, Value> =
      BTreeMap::new();

    // json_file_list(get_files(&self.config.content_root))
    //   .iter()
    //   .for_each(|input_file| {
    //     let input_path =
    //       self.config.content_root.join(input_file);
    //     match fs::read_to_string(&input_path) {
    //       Ok(json) => {
    //         match serde_json::from_str::<Value>(&json) {
    //           Ok(data) => {
    //             data_map.insert(
    //               input_file.display().to_string(),
    //               data,
    //             );
    //           }
    //           Err(e) => {
    //             // TODO: Add better error handling here
    //             dbg!(e);
    //           }
    //         }
    //       }
    //       Err(e) => {
    //         // TODO: Add better error messaging here
    //         dbg!(e);
    //       }
    //     }
    //   });

    Value::from_serialize(data_map)
  }

  pub async fn start(&mut self) -> Result<()> {
    info!("Starting builder");
    let _ = &self.build_site();
    while (self.rx.recv().await).is_some() {
      let _ = &self.build_site();
    }
    Ok(())
  }

  pub fn tmp_output_dir(&self) -> PathBuf {
    let dir_name = format!(
      "{}_tmp",
      &self
        .config
        .output_root
        .file_name()
        .unwrap()
        .display()
    );
    PathBuf::from(
      &self.config.output_root.parent().unwrap(),
    )
    .join(dir_name)
  }

  pub fn transform_html(
    &self,
    file_list: &[FileDetails],
  ) -> Result<()> {
    let env = get_env(&self.config.content_root);
    file_list.iter().for_each(|details| {
      if details.file_move_type
        == FileMoveType::TransformHtml
      {
        let template_name =
          details.input_dir.join(&details.input_name);
        let output_path = &self.tmp_output_dir().join(
          details
            .output_dir
            .clone()
            .unwrap()
            .join(details.output_name.clone().unwrap()),
        );
        dbg!(template_name);
        dbg!(output_path);
      }
    });
    Ok(())
  }

  //
}
