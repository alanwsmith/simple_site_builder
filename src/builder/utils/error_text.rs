pub fn error_text(err: &minijinja::Error) -> String {
  let mut output =
    vec!["A MiniJinja error occurred\n".to_string()];
  output.push(format!(
    "Could not render template:\n{:#}",
    err
  ));
  let mut err = &err as &dyn std::error::Error;
  while let Some(next_err) = err.source() {
    output.push(format!("\ncaused by:\n{:#}", next_err));
    err = next_err;
  }
  output.join("\n")
}
