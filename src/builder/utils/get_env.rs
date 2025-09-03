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
  env
}
