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
    let _ = &self.transform_html()?;
    let _ = &self.copy_files()?;
    info!("Reloading browser");
    let _ = &self.reloader.reload();
    Ok(())
  }

  pub fn copy_files(&self) -> Result<()> {
    copy_file_list(get_files(&self.config.content_root))
      .iter()
      .for_each(|input_file| {
        let details = CopyFileDetails::new(
          &self.config.content_root,
          input_file,
          &self.config.output_root,
        );
        let _ = copy_file_with_mkdir(
          &details.input_path(),
          &details.output_path(),
        );
      });

    Ok(())
  }

  pub fn load_data(&self) -> Value {
    let mut data_map = BTreeMap::new();
    json_file_list(get_files(&self.config.content_root))
      .iter()
      .for_each(|input_file| {
        let input_path =
          self.config.content_root.join(input_file);
        match fs::read_to_string(&input_path) {
          Ok(json) => {
            match serde_json::from_str::<Value>(&json) {
              Ok(data) => {
                data_map.insert(
                  input_file.display().to_string(),
                  data,
                );
              }
              Err(e) => {
                // TODO: Add better error handling here
                dbg!(e);
              }
            }
          }
          Err(e) => {
            // TODO: Add better error messaging here
            dbg!(e);
          }
        }
      });
    Value::from_serialize(data_map)
  }

  pub fn load_highlighted_files(&self) -> Value {
    let mut files = BTreeMap::new();
    highlighted_file_list(get_files(
      &self.config.content_root,
    ))
    .iter()
    .for_each(|input_file| {
      let input_path =
        self.config.content_root.join(input_file);
      // Reminder: files are already filter to make sure
      // they have a valid extension so you
      // can just unwrap:
      let ext = input_path.extension().unwrap();
      match fs::read_to_string(&input_path) {
        Ok(code) => {
          let highlighted = highlight_code(
            &code,
            &ext.display().to_string(),
          );
          files.insert(
            input_file.display().to_string(),
            highlighted,
          );
        }
        Err(e) => {
          dbg!(e);
        }
      }
    });
    Value::from_serialize(files)
  }

  pub fn transform_html(&self) -> Result<()> {
    let env = get_env(&self.config.content_root);
    let data = self.load_data();
    let highlighted = self.load_highlighted_files();
    html_file_list(get_files(&self.config.content_root))
      .iter()
      .for_each(|input_path| {
        let details = HtmlFileDetails::new(
          input_path,
          &self.config.output_root.clone(),
        );
        match env.get_template(&details.input_path_str())
        {
          Ok(template) => {
            match template.render(context!(
              data => data,
              highlighted => highlighted,
            )) {
              Ok(output) => {
                let _ = write_file_with_mkdir(
                  &details.output_path(),
                  &output,
                );
              }
              Err(e) => {
                // TODO: Throw here and print error
                dbg!(e);
              }
            }
          }
          Err(e) => {
            // TODO: Throw here and print error
            dbg!(e);
          }
        }
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
}
