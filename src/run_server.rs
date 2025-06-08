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

pub async fn run_server(livereload: LiveReloadLayer) -> Result<()> {
    let port = find_port()?;
    // launch_browser(port.into())?;
    println!("Starting web server on port: {}", port);
    let service = ServeDir::new("docs")
        .append_index_html_on_directories(true)
        .not_found_service(get(|| missing_page()));
    let app = Router::new().fallback_service(service).layer(livereload);
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

fn find_port() -> Result<u16> {
    free_local_port_in_range(5444..=6000).ok_or(anyhow!("Could not find port"))
}

async fn missing_page() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html lang="en">
<head><style>body { background: black; color: white;}</style></head>
<body>Page Not Found</body>
</html>"#,
    )
}
