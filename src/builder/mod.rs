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
use std::io::empty;
use std::path::PathBuf;
use tokio::sync::mpsc::Receiver;
use tower_livereload::Reloader;
use tracing::info;

pub struct Builder {
  pub config: Config,
  pub reloader: Reloader,
  pub rx: Receiver<DateTime<Local>>,
  pub port: u16,
}

impl Builder {
  pub fn new(
    config: Config,
    reloader: Reloader,
    rx: Receiver<DateTime<Local>>,
    port: u16,
  ) -> Builder {
    Builder {
      config,
      reloader,
      rx,
      port,
    }
  }

  pub fn build_site(&self) -> Result<()> {
    let _ = clearscreen::clear();
    info!("Building site");
    &self.empty_dir();
    let file_list = file_list(&self.config.content_root);
    let _ = &self.transform_html(&file_list)?;
    // let _ = &self.copy_files()?;
    info!(
      "Reloading browser for: http://localhost:{}/",
      self.port
    );
    let _ = &self.reloader.reload();
    Ok(())
  }

  pub fn empty_dir(&self) -> Result<()> {
    empty_dir(&self.config.output_root);
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

  pub fn transform_html(
    &self,
    file_list: &[FileDetails],
  ) -> Result<()> {
    let env = get_env(&self.config.content_root);
    file_list.iter().for_each(|details| {
      if details.file_move_type
        == FileMoveType::TransformHtml
      {
        let template_name = details
          .input_dir
          .join(&details.input_name)
          .display()
          .to_string();
        let output_path = &self.config.output_root.join(
          details
            .output_dir
            .clone()
            .unwrap()
            .join(details.output_name.clone().unwrap()),
        );
        match env.get_template(&template_name) {
          Ok(template) => {
            match template.render(context!()) {
              Ok(content) => {
                let _ = write_file_with_mkdir(
                  output_path,
                  &content,
                );
              }
              Err(e) => {
                println!("{}", e);
              }
            }
          }
          Err(e) => {
            dbg!(e);
          }
        }
      }
    });
    Ok(())
  }

  //
}
