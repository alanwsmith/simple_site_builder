use anyhow::Result;
use anyhow::anyhow;
use chrono::DateTime;
use chrono::Local;
use dialoguer::Confirm;
use itertools::Itertools;
use rust_embed::RustEmbed;
use ssbuild::helpers::*;
use ssbuild::run_builder::*;
use ssbuild::run_server::run_server;
use ssbuild::run_watcher::run_watcher;
use ssbuild::site::Site;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tokio::sync::mpsc;
use tower_livereload::LiveReloadLayer;
use walkdir::WalkDir;

#[derive(RustEmbed)]
#[folder = "src/defaults"]
struct DefaultFiles;

#[tokio::main]
async fn main() -> Result<()> {
    clearscreen::clear()?;
    println!("Starting up...");
    let site = Site::new();
    if init_files_and_dirs(&site)? {
        let livereload = LiveReloadLayer::new();
        let reloader = livereload.reloader();
        let (watcher_tx, watcher_rx) = mpsc::channel::<DateTime<Local>>(32);
        let http_handle = tokio::spawn(async move {
            let _ = run_server(livereload).await;
        });
        let builder_handle = tokio::spawn(async move {
            let _ = run_builder(watcher_rx, reloader, site.clone()).await;
            println!("################ ERROR ##################");
            println!("builder crashed");
            println!("#########################################");
        });
        let _ = run_watcher(watcher_tx).await;
        http_handle.abort();
        builder_handle.abort();
        println!("Process complete");
    }
    Ok(())
}

fn init_files_and_dirs(site: &Site) -> Result<bool> {
    if !path_exists(&site.content_dir) {
        let confirmation = Confirm::new()
            .with_prompt("Make this a website directory?")
            .default(false)
            .interact()
            .unwrap();
        if confirmation {
            let output_root = PathBuf::from(".");
            for file in DefaultFiles::iter() {
                let name = file.as_ref();
                let output_path = output_root.join(name);
                if let Some(content) = DefaultFiles::get(name) {
                    if let Some(parent) = output_path.parent() {
                        if !parent.exists() {
                            std::fs::create_dir_all(parent)?
                        }
                    }
                    if !output_path.exists() {
                        let body: Vec<u8> = content.data.into();
                        let mut output = File::create(output_path)?;
                        output.write_all(&body)?;
                    }
                }
            }
            // make scripts executable
            for script in get_files_in_dir(&site.scripts_dir, None, None)?.iter() {
                let script_path = &site.scripts_dir.join(script);
                let file_name = format!("{}", script_path.display());
                let args = vec!["u+x", &file_name];
                let _ = Command::new("chmod").args(args).output();
                dbg!(script_path);
            }
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Ok(true)
    }
}

fn path_exists(path: &PathBuf) -> bool {
    match path.try_exists() {
        Ok(exists) => {
            if exists == true {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

pub fn get_file_list(dir: &PathBuf) -> Result<Vec<PathBuf>> {
    if !dir.is_dir() {
        return Err(anyhow!("Not a directory at: {}", dir.display()));
    }
    let walker = WalkDir::new(dir).into_iter();
    let files: Vec<_> = walker
        .filter_map(|entry| match entry {
            Ok(e) => Some(e.path().to_path_buf()),
            Err(_) => None,
        })
        .filter(|pb| pb.is_file())
        .filter_map(|path| match path.strip_prefix(dir) {
            Ok(p) => Some(p.to_path_buf()),
            Err(_) => None,
        })
        .filter(|pb| {
            pb.components()
                .find(|part| part.as_os_str().to_string_lossy().starts_with("."))
                .is_none()
        })
        .sorted()
        .collect();
    Ok(files)
}
