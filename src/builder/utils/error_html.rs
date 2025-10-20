pub fn error_html(err: &minijinja::Error) -> String {
  let mut output = vec![
    r#"<html><head><style>body {{ background-color: black; color: goldenrod; }}</style></head><body><pre>"#.to_string()
  ];
  output.push("A MiniJinja error occurred\n".to_string());
  output.push(format!(
    "Could not render template:\n{:#}</p>",
    err
  ));
  let mut err = &err as &dyn std::error::Error;
  while let Some(next_err) = err.source() {
    output.push(format!("\ncaused by:\n{:#}", next_err));
    err = next_err;
  }
  output.push("</pre></body></html>".to_string());
  output.join("\n")
}
