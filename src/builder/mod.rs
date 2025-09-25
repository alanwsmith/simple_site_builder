pub mod prep_build_files;
pub mod utils;

// use self::prep_build_files::*;
use self::utils::*;
use crate::config::Config;
use anyhow::Result;
use chrono::{DateTime, Local};
use markdown::{CompileOptions, Options};
use minijinja::Value;
use minijinja::context;
use std::collections::BTreeMap;
use std::fs;
// use std::io::Write;
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
    info!("Building site on port {}", &self.port);
    empty_dir(&self.config.build_files_dir())?;
    empty_dir(&self.config.output_root)?;
    self.prep_build_files(
      &self.config.content_root,
      &self.config.build_files_dir(),
    )?;
    let file_list =
      file_list(&self.config.build_files_dir());
    let _ = &self.transform_html(&file_list)?;
    let _ = &self.copy_files(&file_list)?;
    let _ = &self.copy_js(&file_list)?;
    empty_dir(&self.config.build_files_dir())?;
    info!("Reloading browser on port {}/", self.port);
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

  pub fn highlight_files(
    &self,
    file_list: &[FileDetails],
  ) -> Value {
    let mut highlights: BTreeMap<String, String> =
      BTreeMap::new();
    file_list
      .iter()
      .filter(|details| {
        details.extension == Some("css".to_string())
          || details.extension == Some("html".to_string())
          || details.extension == Some("js".to_string())
          || details.extension == Some("json".to_string())
          || details.extension == Some("py".to_string())
          || details.extension == Some("rs".to_string())
      })
      .for_each(|details| {
        let content_path = self
          .config
          .build_files_dir()
          .join(&details.folder)
          .join(&details.name);
        let key_path = details.folder.join(&details.name);
        let content =
          fs::read_to_string(content_path).unwrap();
        let highlighted = highlight_code(
          &content,
          details.extension.as_ref().unwrap().as_str(),
        );
        highlights.insert(
          key_path.display().to_string(),
          highlighted,
        );
      });
    Value::from_serialize(highlights)
  }

  pub fn highlight_files_deprecated(
    &self,
    file_list: &[FileDetails],
  ) -> Value {
    let mut highlights: BTreeMap<String, String> =
      BTreeMap::new();
    file_list
      .iter()
      .filter(|details| {
        details.extension == Some("css".to_string())
          || details.extension == Some("html".to_string())
          || details.extension == Some("js".to_string())
          || details.extension == Some("json".to_string())
          || details.extension == Some("py".to_string())
          || details.extension == Some("rs".to_string())
      })
      .for_each(|details| {
        // let content_path = self
        //   .config
        //   .build_files_dir()
        //   .join(&details.folder)
        //   .join(&details.name);
        let key_path = details.folder.join(&details.name);
        // let content =
        //   fs::read_to_string(content_path).unwrap();
        // let highlighted = highlight_code(
        //   &content,
        //   details.extension.as_ref().unwrap().as_str(),
        // );
        highlights.insert(
          key_path.display().to_string(),
          "[minijinja: `highlight` is deprecated. use `highlight_file`]"
            .to_string(),
        );
      });
    Value::from_serialize(highlights)
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

  // TODO: The minify_js crate doesn't work
  // with certain types of imports including
  // the ones that bitty uses. So, This is
  // off until a suitable replacement is found
  //
  // pub fn minify_js(
  //   &self,
  //   file_list: &[FileDetails],
  // ) -> Result<()> {
  //   file_list.iter().for_each(|details| {
  //     if details.file_move_type
  //       == FileMoveType::CopyAndMinifyJavaScript
  //     {
  //       let input_path = &self
  //         .config
  //         .build_files_root()
  //         .join(&details.folder)
  //         .join(&details.name);
  //       let file_stem = &details
  //         .output_name
  //         .as_ref()
  //         .unwrap()
  //         .file_stem()
  //         .unwrap();
  //       let file_name = format!(
  //         "{}.min.js",
  //         file_stem.to_string_lossy()
  //       );
  //       let input_js =
  //         fs::read_to_string(input_path).unwrap();
  //       let code: &[u8] = input_js.as_bytes();
  //       let session = Session::new();
  //       let mut out = Vec::new();
  //       minify(
  //         &session,
  //         TopLevelMode::Global,
  //         code,
  //         &mut out,
  //       )
  //       .unwrap();
  //       dbg!(out.as_slice());
  //       let output_path = &self
  //         .config
  //         .output_root
  //         .join(details.output_folder.as_ref().unwrap())
  //         .join(file_name);
  //       let mut file =
  //         fs::File::create(output_path).unwrap();
  //       file.write_all(out.as_slice());
  //       // let _ =
  //       //   copy_file_with_mkdir(input_path, output_path);
  //     }
  //   });
  //   Ok(())
  // }

  pub async fn start(&mut self) -> Result<()> {
    info!("Starting builder");
    let _ = &self.build_site();
    while (self.rx.recv().await).is_some() {
      let _ = &self.build_site();
    }
    Ok(())
  }

  pub fn load_markdown(
    &self,
    file_list: &[FileDetails],
  ) -> Value {
    let mut markdown_map: BTreeMap<String, String> =
      BTreeMap::new();
    file_list
      .iter()
      .filter(|details| {
        details.extension == Some("md".to_string())
      })
      .for_each(|details| {
        let content_path = self
          .config
          .build_files_dir()
          .join(&details.folder)
          .join(&details.name);
        let key_path = details.folder.join(&details.name);
        let md_content =
          fs::read_to_string(content_path).unwrap();
        match markdown::to_html_with_options(
          &md_content,
          &Options {
            compile: CompileOptions {
              allow_dangerous_html: true,
              ..CompileOptions::default()
            },
            ..Options::default()
          },
        ) {
          Ok(parsed) => {
            markdown_map.insert(
              key_path.display().to_string(),
              parsed,
            );
          }
          Err(e) => {
            dbg!(e);
          }
        }
      });
    Value::from_serialize(markdown_map)
  }

  pub fn load_markdown_deprecated(
    &self,
    file_list: &[FileDetails],
  ) -> Value {
    let mut markdown_map: BTreeMap<String, String> =
      BTreeMap::new();
    file_list
      .iter()
      .filter(|details| {
        details.extension == Some("md".to_string())
      })
      .for_each(|details| {
        let content_path = self
          .config
          .build_files_dir()
          .join(&details.folder)
          .join(&details.name);
        let key_path = details.folder.join(&details.name);
        let md_content =
          fs::read_to_string(content_path).unwrap();
        match markdown::to_html_with_options(
          &md_content,
          &Options {
            compile: CompileOptions {
              allow_dangerous_html: true,
              ..CompileOptions::default()
            },
            ..Options::default()
          },
        ) {
          Ok(_parsed) => {
            markdown_map.insert(
              key_path.display().to_string(),
              "[minijinja: `markdown` is deprecated. use `markdown_file` instead]".to_string()
            );
          }
          Err(e) => {
            dbg!(e);
          }
        }
      });
    Value::from_serialize(markdown_map)
  }

  pub fn transform_html(
    &self,
    file_list: &[FileDetails],
  ) -> Result<()> {
    let folders =
      folder_list(&self.config.build_files_dir());
    let env = get_env(&self.config.build_files_dir());
    let file_list_as_value =
      Value::from_serialize(file_list);
    let folders_as_value = Value::from_serialize(folders);
    let markdown_deprecated =
      self.load_markdown_deprecated(file_list);
    let markdown_files = self.load_markdown(file_list);
    let highlight_deprecated =
      self.highlight_files_deprecated(file_list);
    let highlighted_files =
      self.highlight_files(file_list);
    let data = self.load_data(file_list);
    file_list.iter().for_each(|details| {
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
            highlight_file => highlighted_files,
            highlight => highlight_deprecated,
            markdown_file => markdown_files,
            markdown => markdown_deprecated,
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
            }
          },
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
