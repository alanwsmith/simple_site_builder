#![allow(unused)]
use anyhow::Result;
use anyhow::anyhow;
use axum::Router;
use axum::response::Html;
use axum::routing::get;
use itertools::Itertools;
use minijinja::path_loader;
use minijinja::syntax::SyntaxConfig;
use minijinja::{Environment, Value, context};
use permissions::is_executable;
use port_check::free_local_port_in_range;
use rust_embed::RustEmbed;
use ssbuild::run_builder::*;
use ssbuild::run_server::run_server;
use ssbuild::run_watcher::run_watcher;
use ssbuild::site::Site;
use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::fs::canonicalize;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use tower_livereload::Reloader;
use walkdir::WalkDir;
use watchexec::Watchexec;
use watchexec_signals::Signal;

#[derive(RustEmbed)]
#[folder = "src/defaults"]
struct DefaultFiles;

#[tokio::main]
async fn main() -> Result<()> {
    clearscreen::clear()?;
    println!("Starting up...");
    let site = Site::new();
    init_files_and_dirs(&site)?;
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();
    let (watcher_tx, watcher_rx) = mpsc::channel::<bool>(32);
    let http_handle = tokio::spawn(async move {
        let _ = run_server(livereload).await;
    });
    let builder_handle = tokio::spawn(async move {
        let _ = run_builder(watcher_rx, reloader, site.clone()).await;
    });
    let _ = run_watcher(watcher_tx).await;
    http_handle.abort();
    builder_handle.abort();
    println!("Process complete");
    Ok(())
}

fn init_files_and_dirs(site: &Site) -> Result<()> {
    if !path_exists(&site.content_dir) {
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
    }
    Ok(())
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
