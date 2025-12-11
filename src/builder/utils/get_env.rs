use super::block_highlight_filter;
use super::highlight_code;
use super::highlight_code_with_safe;
use super::inline_highlight_filter;
use super::inline_highlight_function;
use markdown::{CompileOptions, Options};
use minijinja::AutoEscape;
use minijinja::Environment;
use minijinja::Value;
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
  env.set_auto_escape_callback(|name| {
    if matches!(
      name.rsplit('.').next().unwrap_or(""),
      "html" | "htm"
    ) {
      AutoEscape::Html
    } else {
      AutoEscape::None
    }
  });
  env.add_function(
    "inline_highlight",
    inline_highlight_function,
  );
  env.add_filter(
    "inline_highlight",
    inline_highlight_filter,
  );
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
  env.add_filter(
    "block_highlight",
    block_highlight_filter,
  );
  env
}

pub fn highlight_css(code: String) -> Value {
  highlight_code_with_safe(&code, "css")
}

pub fn highlight_html(code: String) -> Value {
  highlight_code_with_safe(&code, "html")
}

pub fn highlight_javascript(code: String) -> Value {
  highlight_code_with_safe(&code, "js")
}

pub fn highlight_json(code: String) -> Value {
  highlight_code_with_safe(&code, "json")
}

pub fn highlight_lua(code: String) -> Value {
  highlight_code_with_safe(&code, "lua")
}

pub fn highlight_python(code: String) -> Value {
  highlight_code_with_safe(&code, "py")
}

pub fn highlight_rust(code: String) -> Value {
  highlight_code_with_safe(&code, "rs")
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
