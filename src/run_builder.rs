use crate::run_scripts::run_scripts;
use crate::site::Site;
use anyhow::Result;
use minijinja::Environment;
use minijinja::context;
use minijinja::path_loader;
use minijinja::syntax::SyntaxConfig;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use tokio::sync::mpsc::Receiver;
use tower_livereload::Reloader;
use walkdir::WalkDir;

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
    Ok(file_list)
}

pub async fn run_builder(mut rx: Receiver<bool>, reloader: Reloader, site: Site) -> Result<()> {
    let mut first_run = true;
    println!("Starting builder");
    let format = "%-I:%M:%S%p";
    while let Some(_) = rx.recv().await {
        if !first_run {
            clearscreen::clear()?;
        }
        first_run = false;
        println!(
            "Building at {}",
            chrono::Local::now()
                .format(format)
                .to_string()
                .to_lowercase()
        );
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

fn get_json5_data_files() -> Result<BTreeMap<String, Value>> {
    let mut data = BTreeMap::new();
    Ok(data)
}
