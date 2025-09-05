use markdown::{CompileOptions, Options};
use minijinja::Environment;
use minijinja::path_loader;
use minijinja::syntax::SyntaxConfig;
use std::path::Path;

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
  env.add_filter("markdown", mj_markdown);
  env
}
