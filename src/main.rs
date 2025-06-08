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

#[derive(Clone)]
struct Site {
    content_dir: PathBuf,
    data_dir: PathBuf,
    docs_dir: PathBuf,
    scripts_dir: PathBuf,
}

impl Site {
    pub fn new() -> Site {
        Site {
            content_dir: PathBuf::from("content"),
            data_dir: PathBuf::from("data"),
            docs_dir: PathBuf::from("docs"),
            scripts_dir: PathBuf::from("scripts"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
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

fn deploy_non_html_files() -> Result<()> {
    let content_dir = PathBuf::from("content");
    let docs_dir = PathBuf::from("docs");
    let file_list: Vec<PathBuf> = WalkDir::new(&content_dir)
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
        .filter(|e| e.path().extension().unwrap() != "html")
        .map(|e| e.path().to_path_buf())
        .map(|e| e.strip_prefix(content_dir.clone()).unwrap().to_path_buf())
        .filter(|e| !e.display().to_string().starts_with("_"))
        .collect();
    for file in file_list.iter() {
        let in_path = content_dir.join(file);
        let out_path = docs_dir.join(file);
        let out_parent = out_path.parent().unwrap();
        fs::create_dir_all(&out_parent)?;
        let data = std::fs::read(in_path)?;
        std::fs::write(out_path, &data)?;
    }
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
        .filter(|e| !e.display().to_string().starts_with("_"))
        .collect();
    dbg!(&file_list);
    Ok(file_list)
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

async fn run_builder(mut rx: Receiver<bool>, reloader: Reloader, site: Site) -> Result<()> {
    println!("Starting builder");
    while let Some(_) = rx.recv().await {
        println!("Building.");
        run_scripts(&site.scripts_dir)?;
        let mut env = Environment::new();
        env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );
        let docs_dir = PathBuf::from("docs");
        empty_dir(&docs_dir)?;
        std::fs::create_dir_all(&docs_dir)?;
        env.set_loader(path_loader("content"));
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
        deploy_non_html_files()?;
        reloader.reload();
    }
    println!("Builder stopped.");
    Ok(())
}

pub fn run_scripts(dir: &PathBuf) -> Result<()> {
    println!("Running scripts");
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
        .filter(|pb| {
            pb.components()
                .find(|part| part.as_os_str().to_string_lossy().starts_with("."))
                .is_none()
        })
        .sorted()
        .collect();
    for file in files {
        if is_executable(&file)? {
            let name = file.file_name().ok_or(anyhow!("Cound not get file name"))?;
            let parent = file.parent().ok_or(anyhow!("Could not get parent"))?;
            let canon_parent = canonicalize(parent)?;
            Command::new(format!("./{}", name.display()))
                .current_dir(canon_parent)
                .spawn()?;
        }
    }
    Ok(())
}

async fn run_server(livereload: LiveReloadLayer) -> Result<()> {
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
