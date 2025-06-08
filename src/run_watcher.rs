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

pub async fn run_watcher(tx: Sender<bool>) -> Result<()> {
    println!("Starting watcher");
    tx.send(true).await.unwrap();
    let wx = Watchexec::default();
    wx.config.pathset(vec!["content"]);
    wx.config.on_action(move |mut action| {
        let tx2 = tx.clone();
        tokio::spawn(async move {
            tx2.send(true).await.unwrap();
        });
        if action
            .signals()
            .any(|sig| sig == Signal::Interrupt || sig == Signal::Terminate)
        {
            action.quit(); // Needed for Ctrl+c
        }
        action
    });
    let _ = wx.main().await?;
    println!("Watcher stopped.");
    Ok(())
}
