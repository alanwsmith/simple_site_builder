#![allow(unused)]
use anyhow::Result;
use anyhow::anyhow;
use axum::Router;
use axum::response::Html;
use axum::routing::get;
use minijinja::path_loader;
use minijinja::syntax::SyntaxConfig;
use minijinja::{Environment, Value, context};
use port_check::free_local_port_in_range;
use std::fs;
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

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting up...");
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();
    let (watcher_tx, mut watcher_rx) = mpsc::channel::<bool>(32);
    let http_handle = tokio::spawn(async move {
        run_server(livereload).await;
    });
    let builder_handle = tokio::spawn(async move {
        run_builder(watcher_rx, reloader).await;
    });
    run_watcher(watcher_tx).await;
    http_handle.abort();
    builder_handle.abort();
    println!("Process complete");
    Ok(())
}

fn empty_dir(dir: &PathBuf) -> Result<()> {
    if let Ok(exists) = dir.try_exists() {
        if exists {
            for entry in dir.read_dir()? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    fs::remove_dir_all(path)?;
                } else {
                    fs::remove_file(path)?;
                }
            }
        }
    }
    Ok(())
}

fn find_port() -> Result<u16> {
    free_local_port_in_range(5444..=6000).ok_or(anyhow!("Could not find port"))
}

pub fn get_source_html_files(root_dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let file_list: Vec<PathBuf> = WalkDir::new(root_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| {
            !e.file_name()
                .to_str()
                .map(|s| s.starts_with("."))
                .unwrap_or(false)
        })
        .filter(|e| e.path().extension().is_some())
        .filter(|e| e.path().extension().unwrap() == "html")
        .map(|e| e.path().to_path_buf())
        .map(|e| e.strip_prefix(root_dir).unwrap().to_path_buf())
        .collect();
    Ok(file_list)
}

fn launch_browser(port: usize) -> Result<()> {
    let args: Vec<String> = vec![format!("http://localhost:{}", port)];
    Command::new("open").args(args).output()?;
    Ok(())
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

async fn run_builder(mut rx: Receiver<bool>, reloader: Reloader) -> Result<()> {
    println!("Starting builder");
    let mut env = Environment::new();
    env.set_syntax(
        SyntaxConfig::builder()
            .block_delimiters("[!", "!]")
            .variable_delimiters("[@", "@]")
            .comment_delimiters("[#", "#]")
            .build()
            .unwrap(),
    );
    while let Some(message) = rx.recv().await {
        println!("Building.");
        let docs_dir = PathBuf::from("docs");
        empty_dir(&docs_dir);
        std::fs::create_dir_all(&docs_dir);
        env.set_loader(path_loader("templates"));
        for source_file in get_source_html_files(&PathBuf::from("content"))?.iter() {
            if let Some(parent) = source_file.parent() {
                if parent.display().to_string() != "" {
                    let dir_path = PathBuf::from("docs").join(parent);
                    dbg!(&dir_path);
                    std::fs::create_dir_all(dir_path)?;
                }
            }
            let current_source = fs::read_to_string(format!("content/{}", source_file.display()))?;
            env.add_template_owned("current-source", current_source)?;
            let template = env.get_template("current-source")?;
            match template.render(context!()) {
                Ok(output) => {
                    fs::write(format!("docs/{}", source_file.display()), output).unwrap();
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        reloader.reload();
    }
    println!("Builder stopped.");
    Ok(())
}

async fn run_server(livereload: LiveReloadLayer) -> Result<()> {
    let port = find_port()?;
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

async fn run_watcher(tx: Sender<bool>) -> Result<()> {
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
