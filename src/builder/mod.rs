#![allow(unused)]
pub mod prep_build_files;
pub mod utils;

use self::utils::*;
use crate::config::Config;
use anyhow::Result;
use chrono::{DateTime, Local};
use minijinja::Value;
use minijinja::context;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use tokio::sync::mpsc::Receiver;
use tokio::time::Duration;
use tokio::time::sleep;
use tower_livereload::Reloader;
use tracing::info;

pub struct Builder {
  pub config: Config,
  pub reloader: Reloader,
  pub rx: Receiver<DateTime<Local>>,
  pub port: u16,
  pub json: DataNode,
  pub last_build_started: chrono::DateTime<Local>,
}

impl Builder {
  pub fn new(
    config: Config,
    reloader: Reloader,
    rx: Receiver<DateTime<Local>>,
    port: u16,
    json: DataNode,
  ) -> Builder {
    Builder {
      config,
      reloader,
      rx,
      port,
      json,
      last_build_started: chrono::prelude::Local::now(),
    }
  }

  pub async fn build_site(
    &mut self,
    ts: chrono::DateTime<chrono::Local>,
  ) -> Result<()> {
    if ts > self.last_build_started {
      let _ = clearscreen::clear();
      info!("Building site on port {}.", &self.port);
      sleep(Duration::from_millis(200)).await;
      self.last_build_started =
        chrono::prelude::Local::now();
      self.json = DataNode::new();
      empty_dir(&self.config.build_files_dir())?;
      empty_dir(&self.config.output_root)?;
      info!("Initial Pass: Loading File List...");
      let file_list =
        get_file_list(&self.config.content_root);
      info!("Initial Pass: Copying files...");
      let _ = &self.copy_files(
        &file_list,
        &self.config.content_root.clone(),
        &self.config.build_files_dir(),
      )?;
      info!("Initial Pass: Transform Files...");
      let _ = &self.transform_html_and_txt(
        &file_list,
        &self.config.content_root.clone(),
        &self.config.build_files_dir(),
      )?;
      info!("Output Pass: Loading File List...");
      let output_file_list =
        get_file_list(&self.config.build_files_dir());
      info!("Initial Pass: Copying files...");
      let _ = &self.copy_files(
        &output_file_list,
        &self.config.build_files_dir(),
        &self.config.output_root.clone(),
      )?;
      info!("Initial Pass: Transform Files...");
      let _ = &self.transform_html_and_txt(
        &output_file_list,
        &self.config.build_files_dir(),
        &self.config.output_root.clone(),
      )?;
      empty_dir(&self.config.build_files_dir())?;
      info!(
        r#"Build complete. Reloading browser on port {}."#,
        self.port
      );
      let _ = &self.reloader.reload();
    }
    Ok(())
  }

  pub fn copy_files(
    &self,
    file_list: &[FileDetails],
    input_root: &Path,
    output_root: &Path,
  ) -> Result<()> {
    file_list.iter().for_each(|details| {
      if details.file_move_type == FileMoveType::Copy {
        let input_path = &input_root
          .join(&details.folder)
          .join(&details.name);
        let output_path = &output_root
          .join(details.output_folder.as_ref().unwrap())
          .join(details.output_name.as_ref().unwrap());
        let _ =
          copy_file_with_mkdir(input_path, output_path);
      }
    });
    Ok(())
  }

  // pub fn copy_js(
  //   &self,
  //   file_list: &[FileDetails],
  //   input_root: &Path,
  //   output_root: &Path,
  // ) -> Result<()> {
  //   // TODO: Set up a minifier at some point
  //   // when you get one that works (the rust one
  //   // broke on import statements).
  //   file_list.iter().for_each(|details| {
  //     if details.file_move_type
  //       == FileMoveType::Copy
  //     {
  //       let input_path = &input_root
  //         .join(&details.folder)
  //         .join(&details.name);
  //       let output_path = &output_root
  //         .join(details.output_folder.as_ref().unwrap())
  //         .join(details.output_name.as_ref().unwrap());
  //       let _ =
  //         copy_file_with_mkdir(input_path, output_path);
  //     }
  //   });
  //   Ok(())
  // }

  pub fn load_json(
    &mut self,
    file_list: &[FileDetails],
    input_root: &Path,
  ) {
    file_list
      .iter()
      .filter(|details| {
        details.extension == Some("json".to_string())
      })
      .for_each(|details| {
        let key = details.folder.join(&details.name);
        let input_path = input_root.join(&key);
        match fs::read_to_string(&input_path) {
          Ok(json) => {
            match serde_json::from_str::<Value>(&json) {
              Ok(json) => {
                let _ = &self.json.insert(
                  &details.dir_path_strings(),
                  json,
                );
              }
              Err(err) => {
                println!("{}", &input_path.display());
                // TODO: Add better error handling here
                println!("ERROR: {:#}", err);
              }
            }
          }
          Err(err) => {
            // TODO: Add better error messaging here
            println!("{}", &input_path.display());
            println!("ERROR: {:#}", err);
          }
        }
      });
  }

  pub async fn start(&mut self) -> Result<()> {
    info!("Starting builder");
    let _ = &self
      .build_site(chrono::prelude::Local::now())
      .await;
    while let Some(ts) = self.rx.recv().await {
      let _ = &self.build_site(ts).await;
    }
    Ok(())
  }

  pub fn transform_html_and_txt(
    &mut self,
    file_list: &[FileDetails],
    input_root: &PathBuf,
    output_root: &Path,
  ) -> Result<()> {
    let folders = folder_list(input_root);
    let mut env = get_env(&input_root.clone());
    env.add_function("highlight_code", highlight_code);
    env.add_function("highlight_file", highlight_file);
    env.add_function("markdown_file", markdown_file);
    let file_list_as_value =
      Value::from_serialize(file_list);
    let folders_as_value = Value::from_serialize(folders);
    info!("Loading JSONs.");
    self.load_json(file_list, &input_root.clone());
    let json = Value::from_serialize(self.json.clone());
    file_list.iter().for_each(|details| {
      if details.file_move_type == FileMoveType::Transform
      {
        let template_name = details
          .folder
          .join(&details.name)
          .display()
          .to_string();
        let output_path = &output_root.join(
          details
            .output_folder
            .clone()
            .unwrap()
            .join(details.output_name.clone().unwrap()),
        );
        match env.get_template(&template_name) {
          Ok(template) => match template.render(context!(
            json => json,
            files => file_list_as_value,
            folders => folders_as_value,
            file => Value::from_serialize(details),
          )) {
            Ok(content) => {
              let output_content = content
                .replace("{!", "[!")
                .replace("!}", "!]")
                .replace("{@", "[@")
                .replace("@}", "@]")
                .replace("{#", "[#")
                .replace("#}", "#]");
              let _ = write_file_with_mkdir(
                output_path,
                &output_content,
              );
            }
            Err(err) => {
              println!("{}", error_html(&err));
              let _ = write_file_with_mkdir(
                output_path,
                &error_html(&err),
              );
            }
          },
          Err(err) => {
            println!("{}", error_html(&err));
            let _ = write_file_with_mkdir(
              output_path,
              &error_html(&err),
            );
          }
        }

        //
      }
    });
    Ok(())
  }

  //
}
