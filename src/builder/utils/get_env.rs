use super::highlight_code;
use markdown::{CompileOptions, Options};
use minijinja::Environment;
use minijinja::path_loader;
use minijinja::syntax::SyntaxConfig;
use std::path::Path;

pub fn get_env(
  content_dir: &Path
) -> Environment<'static> {
  let mut env = Environment::new();
  env.set_syntax(
    SyntaxConfig::builder()
      .block_delimiters("[!", "!]")
      .variable_delimiters("[@", "@]")
      .comment_delimiters("[#", "#]")
      .build()
      .unwrap(),
  );
  env.set_lstrip_blocks(true);
  env.set_trim_blocks(true);
  env.set_loader(path_loader(
    content_dir.display().to_string(),
  ));
  env.add_filter("highlight_css", highlight_css);
  env.add_filter("highlight_html", highlight_html);
  env.add_filter(
    "highlight_javascript",
    highlight_javascript,
  );
  env.add_filter("highlight_json", highlight_json);
  env.add_filter("highlight_lua", highlight_lua);
  env.add_filter("highlight_python", highlight_python);
  env.add_filter("highlight_rust", highlight_rust);
  env.add_filter("markdown", mj_markdown);
  env
}

pub fn highlight_css(code: String) -> String {
  highlight_code(&code, "css")
}

pub fn highlight_html(code: String) -> String {
  highlight_code(&code, "html")
}

pub fn highlight_javascript(code: String) -> String {
  highlight_code(&code, "js")
}

pub fn highlight_json(code: String) -> String {
  highlight_code(&code, "json")
}

pub fn highlight_lua(code: String) -> String {
  highlight_code(&code, "lua")
}

pub fn highlight_python(code: String) -> String {
  highlight_code(&code, "py")
}

pub fn highlight_rust(code: String) -> String {
  highlight_code(&code, "rs")
}

pub fn mj_markdown(value: String) -> String {
  match markdown::to_html_with_options(
    &value,
    &Options {
      compile: CompileOptions {
        allow_dangerous_html: true,
        ..CompileOptions::default()
      },
      ..Options::default()
    },
  ) {
    Ok(parsed) => parsed.to_string(),
    Err(_e) => "[unable to parse markdown]".to_string(),
  }
}
