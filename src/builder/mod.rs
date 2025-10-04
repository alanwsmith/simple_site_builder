pub mod prep_build_files;
pub mod utils;

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
    info!("Building site on port {}.", &self.port);
    empty_dir(&self.config.build_files_dir())?;
    empty_dir(&self.config.output_root)?;
    self.prep_build_files(
      &self.config.content_root,
      &self.config.build_files_dir(),
    )?;
    let file_list =
      file_list(&self.config.build_files_dir());
    info!("Transforming HTML.");
    let _ = &self.transform_html(&file_list)?;
    info!("Copying files.");
    let _ = &self.copy_files(&file_list)?;
    info!("Copying JavaScript Files.");
    let _ = &self.copy_js(&file_list)?;
    // NOTE: Keeping the .build-files directory
    // around for now to help with debugging
    // the builder.
    // empty_dir(&self.config.build_files_dir())?;
    info!(
      r#"Build complete. Reloading browser on port {}."#,
      self.port
    );
    let _ = &self.reloader.reload();

    Ok(())
  }

  pub fn copy_files(
    &self,
    file_list: &[FileDetails],
  ) -> Result<()> {
    file_list.iter().for_each(|details| {
      if details.file_move_type == FileMoveType::Copy {
        let input_path = &self
          .config
          .build_files_dir()
          .join(&details.folder)
          .join(&details.name);
        let output_path = &self
          .config
          .output_root
          .join(details.output_folder.as_ref().unwrap())
          .join(details.output_name.as_ref().unwrap());
        let _ =
          copy_file_with_mkdir(input_path, output_path);
      }
    });
    Ok(())
  }

  pub fn copy_js(
    &self,
    file_list: &[FileDetails],
  ) -> Result<()> {
    // TODO: Set up a minifier at some point
    // when you get one that works (the rust one
    // broke on import statements).
    file_list.iter().for_each(|details| {
      if details.file_move_type
        == FileMoveType::CopyAndMinifyJavaScript
      {
        let input_path = &self
          .config
          .build_files_dir()
          .join(&details.folder)
          .join(&details.name);
        let output_path = &self
          .config
          .output_root
          .join(details.output_folder.as_ref().unwrap())
          .join(details.output_name.as_ref().unwrap());
        let _ =
          copy_file_with_mkdir(input_path, output_path);
      }
    });
    Ok(())
  }

  pub fn load_data(
    &self,
    file_list: &[FileDetails],
  ) -> Value {
    let mut data_map: BTreeMap<String, Value> =
      BTreeMap::new();
    file_list
      .iter()
      .filter(|details| {
        details.extension == Some("json".to_string())
      })
      .for_each(|details| {
        let key = details.folder.join(&details.name);
        let input_path =
          self.config.build_files_dir().join(&key);
        match fs::read_to_string(&input_path) {
          Ok(json) => {
            match serde_json::from_str::<Value>(&json) {
              Ok(data) => {
                data_map.insert(
                  key.display().to_string(),
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
    let folders =
      folder_list(&self.config.build_files_dir());
    // TODO: Hoist get_env so you only call it
    // once per build (e.g. not again in the copy
    // javascript or other files stuff)
    let mut env = get_env(&self.config.build_files_dir());
    env.add_function("highlight_file", highlight_file);
    env.add_function("markdown_file", markdown_file);
    let file_list_as_value =
      Value::from_serialize(file_list);
    let folders_as_value = Value::from_serialize(folders);
    info!("Loading data.");
    let data = self.load_data(file_list);
    file_list.iter().for_each(|details| {
      // dbg!(&details.file_move_type);
      if details.file_move_type
        == FileMoveType::TransformHtml
      {
        let template_name = details
          .folder
          .join(&details.name)
          .display()
          .to_string();
        let output_path = &self.config.output_root.join(
          details
            .output_folder
            .clone()
            .unwrap()
            .join(details.output_name.clone().unwrap()),
        );
        match env.get_template(&template_name) {
          Ok(template) => match template.render(context!(
            data => data,
            files => file_list_as_value,
            folders => folders_as_value,
            file => Value::from_serialize(details),
          )) {
            Ok(content) => {
              let _ = write_file_with_mkdir(
                output_path,
                &content,
              );
            }
            Err(e) => {
              println!("{}", e);
              let _ = write_file_with_mkdir(
                output_path,
                format!(r#"<html><head><style>body {{ background-color: black; color: goldenrod; }}</style></head><body>A MiniJinja error occurred. <pre>{}</pre></body></html>"#, e).as_str()
              );
            }
          },
          Err(e) => {
            println!("{}", e);
              let _ = write_file_with_mkdir(
                output_path,
                format!(r#"<html><head><style>body {{ background-color: black; color: goldenrod; }}</style></head><body>A MiniJinja error occurred. <pre>{}</pre></body></html>"#, e).as_str()
              );
          }
        }
      }
    });
    Ok(())
  }

  //
}
