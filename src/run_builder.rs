use crate::helpers::*;
use crate::renderer::Renderer;
use crate::run_scripts::run_scripts;
use crate::site::Site;
use anyhow::Result;
use chrono::DateTime;
use chrono::Local;
use minijinja::Value;
use minijinja::context;
use serde_json5;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::mpsc::Receiver;
use tower_livereload::Reloader;
use walkdir::WalkDir;

fn check_script_list(files: &Vec<PathBuf>, dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let return_list = get_files_in_tree(dir, None, None)?
        .iter()
        .map(|f| {
            let check_file = f.to_path_buf();
            if !files.contains(&check_file) {
                let file_name = format!("{}", dir.join(&check_file).display());
                let args = vec!["u+x", &file_name];
                let _ = Command::new("chmod").args(args).output();
            }
            check_file
        })
        .collect();
    Ok(return_list)
}

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

pub async fn run_builder(
    mut rx: Receiver<DateTime<Local>>,
    reloader: Reloader,
    site: Site,
) -> Result<()> {
    println!("Starting builder");
    let mut first_run = true;
    let mut last_update = Instant::now();
    // TODO: don't kill the builder here if this fails
    let mut script_list = get_files_in_tree(&site.scripts_dir, None, None)?;
    let mut renderer = Renderer::new();
    while let Some(_) = rx.recv().await {
        let elapsed = last_update.elapsed();
        if !first_run && elapsed < Duration::from_millis(300) {
            continue;
        }
        if !first_run {
            clearscreen::clear()?;
        }
        print_timestamp();
        // TODO: don't kill the builder here if this fails
        script_list = check_script_list(&script_list, &site.scripts_dir)?;
        // TODO: show errors here if they happen
        let _ = run_scripts(&site.scripts_dir);
        let docs_dir = PathBuf::from("docs");
        let _ = empty_dir(&docs_dir);
        if let Ok(_) = std::fs::create_dir_all(&docs_dir) {
            println!("Made directory: {}", docs_dir.display());
            renderer.env.clear_templates();
            renderer.add_template_dir(&site.content_dir);
            if let Ok(data_context) = get_data_context() {
                if let Ok(source_files) =
                    get_files_in_dir(&PathBuf::from("content"), Some(vec!["html"]), None)
                {
                    for source_file in source_files.iter() {
                        let in_path = &site.content_dir.join(source_file);
                        let out_path = &site.docs_dir.join(source_file);
                        renderer.add_template_from_path(&in_path.display().to_string(), &in_path);
                        let content =
                            renderer.render_content(&in_path.display().to_string(), &data_context);
                        match write_file_with_mkdir(&out_path, &content) {
                            Ok(_) => println!("Generated: {}", &out_path.display()),
                            Err(e) => {
                                println!("ERROR: {} with: {}", e.to_string(), &out_path.display())
                            }
                        }
                    }
                } else {
                    println!("Could not load source files");
                }
            } else {
                println!("Could not load data context");
            }
        } else {
            println!("ERROR: Could not make directory: {}", docs_dir.display());
        }

        first_run = false;
        last_update = Instant::now();
        // TODO: show errors here if they happen
        let _ = deploy_non_html_files();
        reloader.reload();

        //     for source_file in
        //         get_files_in_dir(&PathBuf::from("content"), Some(vec!["html"]), None)?.iter()
        //     {
        //         if let Some(parent) = source_file.parent() {
        //             if parent.display().to_string() != "" {
        //                 let dir_path = PathBuf::from("docs").join(parent);
        //                 std::fs::create_dir_all(dir_path)?;
        //             }
        //         }
        //         let current_source =
        //             fs::read_to_string(format!("content/{}", source_file.display()))?;
        //         env.add_template_owned("current-source", current_source)?;
        //         let template = env.get_template("current-source")?;
        //         match template.render(context!(data)) {
        //             Ok(output) => {
        //                 fs::write(format!("docs/{}", source_file.display()), output).unwrap();
        //             }
        //             Err(e) => {
        //                 println!("{}", e);
        //             }
        //         }
        //     }

        //
    }
    println!("ERROR: Builder stopped.");
    Ok(())
}

fn get_data_context() -> Result<Value> {
    let mut data = BTreeMap::new();
    let root_dir = PathBuf::from("content/_data");
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
    Ok(context!(data => Value::from_serialize(data)))
}

fn print_timestamp() {
    let format = "%-I:%M:%S%p";
    println!(
        "Building at {}",
        chrono::Local::now()
            .format(format)
            .to_string()
            .to_lowercase()
    );
}

// fn output_files(site: &Site, env: &mut Environment) -> Result<()> {
//     let data = match get_data() {
//         Ok(d) => d,
//         Err(_) => serde_json5::from_str("{}").unwrap(),
//     };
//     if let Ok(html_files) = get_files_in_dir(&site.content_dir, Some(vec!["html"]), None) {
//         for html_file in html_files {
//             if let Err(e) = output_file(&html_file, &data, env) {
//                 println!("{}", e);
//             }
//         }
//     } else {
//         println!(
//             "ERROR: Cound not load content files from: {}",
//             &site.content_dir.display()
//         );
//     }
//     Ok(())
// }

// fn output_file(html_file: &PathBuf, data: &Value, env: &mut Environment) -> Result<()> {
//     if let Some(parent) = html_file.parent() {
//         if parent.display().to_string() != "" {
//             let dir_path = PathBuf::from("docs").join(parent);
//             std::fs::create_dir_all(dir_path)?;
//         }
//     }
//     let current_source = fs::read_to_string(format!("content/{}", html_file.display()))?;
//     match env.add_template_owned("current-source", current_source) {
//         Ok(_) => {
//             if let Ok(template) = env.get_template("current-source") {
//                 match template.render(context!(data)) {
//                     Ok(output) => {
//                         fs::write(format!("docs/{}", html_file.display()), output).unwrap();
//                     }
//                     Err(e) => {
//                         println!("ERROR IN: {}\n{}", html_file.display(), e);
//                     }
//                 }
//             }
//         }
//         Err(e) => {
//             println!("ERROR IN: {}\n{}", html_file.display(), e);
//         }
//     }
//     Ok(())
// }
