use crate::helpers::*;
use crate::run_scripts::run_scripts;
use crate::site::Site;
use anyhow::Result;
use chrono::DateTime;
use chrono::Local;
use minijinja::Environment;
use minijinja::Value;
use minijinja::context;
use minijinja::path_loader;
use minijinja::syntax::SyntaxConfig;
use serde_json5;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::mpsc::Receiver;
use tower_livereload::Reloader;
use walkdir::WalkDir;

fn deploy_non_html_files() -> Result<()> {
    let content_dir = PathBuf::from("content");
    let docs_dir = PathBuf::from("docs");
    let file_list: Vec<PathBuf> = get_files_in_tree(&content_dir, None, Some(vec!["html"]))?
        .into_iter()
        .filter(|pb| !pb.display().to_string().starts_with("_"))
        .collect();
    copy_file_list_from_to(&file_list, &content_dir, &docs_dir, true)?;
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

pub async fn run_builder(
    mut rx: Receiver<DateTime<Local>>,
    reloader: Reloader,
    site: Site,
) -> Result<()> {
    let mut first_run = true;
    println!("Starting builder");
    let format = "%-I:%M:%S%p";
    let mut last_update = Instant::now();
    while let Some(_) = rx.recv().await {
        let elapsed = last_update.elapsed();
        if first_run || elapsed > Duration::from_millis(200) {
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
            let data = get_data()?;
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
            for source_file in
                get_files_in_dir(&PathBuf::from("content"), Some(vec!["html"]), None)?.iter()
            {
                if let Some(parent) = source_file.parent() {
                    if parent.display().to_string() != "" {
                        let dir_path = PathBuf::from("docs").join(parent);
                        dbg!(&dir_path);
                        std::fs::create_dir_all(dir_path)?;
                    }
                }
                let current_source =
                    fs::read_to_string(format!("content/{}", source_file.display()))?;
                env.add_template_owned("current-source", current_source)?;
                let template = env.get_template("current-source")?;
                match template.render(context!(data)) {
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
            last_update = Instant::now();
        }
    }
    println!("Builder stopped.");
    Ok(())
}

fn get_data() -> Result<Value> {
    let mut data = BTreeMap::new();
    let root_dir = PathBuf::from("data");
    let file_list: Vec<PathBuf> = WalkDir::new(root_dir.clone())
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
        .filter(|e| {
            e.path().extension().unwrap() == "json" || e.path().extension().unwrap() == "json5"
        })
        .map(|e| e.path().to_path_buf())
        .filter(|e| !e.display().to_string().starts_with("_"))
        .collect();
    for data_file in file_list.iter() {
        let json = fs::read_to_string(data_file)?;
        if let Ok(value) = serde_json5::from_str::<Value>(&json) {
            if let Some(file_stem) = data_file.file_stem() {
                data.insert(file_stem.display().to_string(), value);
            }
        }
    }
    Ok(Value::from_serialize(data))
}
