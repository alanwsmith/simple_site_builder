use minijinja::syntax::SyntaxConfig;
use minijinja::{Environment, Value, path_loader};
use std::fs;
use std::path::PathBuf;

pub struct Renderer<'a> {
    pub env: Environment<'a>,
    pub log: Vec<RendererStatus>,
    error_template: Option<String>,
}

impl Renderer<'_> {
    pub fn new() -> Renderer<'static> {
        let mut renderer = Renderer {
            env: Environment::new(),
            log: vec![],
            error_template: None,
        };
        renderer.env.set_debug(true);
        renderer.env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );
        renderer
    }

    pub fn add_template(&mut self, name: &str, content: &str) {
        match self
            .env
            .add_template_owned(name.to_string(), content.to_string())
        {
            Ok(_) => self.log.push(RendererStatus::AddTemplateSuccess {
                path: None,
                name: name.to_string(),
            }),
            Err(e) => self.log.push(RendererStatus::AddTemplateError {
                path: None,
                name: name.to_string(),
                error_text: e.display_debug_info().to_string(),
            }),
        }
    }

    pub fn add_template_dir(&mut self, dir: &PathBuf) {
        if dir.is_dir() {
            self.env.set_loader(path_loader(dir));
            self.log.push(RendererStatus::AddTemplateDirSuccess {
                path: dir.to_path_buf(),
            });
        } else {
            self.log.push(RendererStatus::AddTemplateDirError {
                path: dir.to_path_buf(),
                error_text: format!(
                    "Tried to load tempaltes from missing directory: {}",
                    dir.display()
                ),
            });
        }
    }

    pub fn add_template_from_path(&mut self, name: &str, path: &PathBuf) {
        match fs::read_to_string(path) {
            Ok(content) => {
                self.add_template(name, &content);
            }
            Err(e) => {
                self.log.push(RendererStatus::AddTemplateFileError {
                    name: name.to_string(),
                    path: path.to_path_buf(),
                    error_text: format!("Error: {} - on file: {}", e.to_string(), path.display()),
                });
            }
        }
    }

    fn error_template(&self, error_text: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<style>
body {{ background-color: black; color: #aaa; }}
</style>
</head>
<body><pre>{}</pre></body>
"#,
            error_text
        )
    }

    pub fn errors(&self) -> Vec<&RendererStatus> {
        self.log
            .iter()
            .filter(|item| match item {
                RendererStatus::AddTemplateError { .. } => true,
                RendererStatus::AddTemplateDirError { .. } => true,
                RendererStatus::AddTemplateFileError { .. } => true,
                RendererStatus::GetTemplateError { .. } => true,
                RendererStatus::RenderContentError { .. } => true,
                _ => false,
            })
            .collect()
    }

    pub fn render_content(&mut self, template: &str, context: &Value) -> String {
        match self.env.get_template(template) {
            Ok(tmpl) => match tmpl.render(context) {
                Ok(output) => {
                    self.log.push(RendererStatus::RenderContentSuccess {
                        template: template.to_string(),
                    });
                    output
                }
                Err(e) => {
                    self.log.push(RendererStatus::RenderContentError {
                        error_text: e.display_debug_info().to_string(),
                        template: template.to_string(),
                    });
                    self.error_template(&e.display_debug_info().to_string())
                }
            },
            Err(e) => {
                self.log.push(RendererStatus::GetTemplateError {
                    template: template.to_string(),
                    error_text: e.display_debug_info().to_string(),
                });
                self.error_template(&e.display_debug_info().to_string())
            }
        }
    }

    pub fn set_error_template(&mut self, fmt: &str) {
        self.error_template = Some(fmt.to_string());
    }
}

pub enum RendererStatus {
    AddTemplateError {
        path: Option<PathBuf>,
        name: String,
        error_text: String,
    },
    AddTemplateFileError {
        path: PathBuf,
        name: String,
        error_text: String,
    },
    AddTemplateSuccess {
        path: Option<PathBuf>,
        name: String,
    },
    AddTemplateDirError {
        path: PathBuf,
        error_text: String,
    },
    AddTemplateDirSuccess {
        path: PathBuf,
    },
    GetTemplateError {
        template: String,
        error_text: String,
    },
    RenderContentError {
        template: String,
        error_text: String,
    },
    RenderContentSuccess {
        template: String,
    },
}
