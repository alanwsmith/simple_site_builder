pub fn error_html(err: &minijinja::Error) -> String {
  let mut output = vec![
    r#"<!DOCTYPE html><html lang="en"><head><style>
    body { background-color: black; color: goldenrod; }
    pre {
      white-space: pre-wrap; 
      overflow-wrap: anywhere;
      overflow-x: auto;
      overscroll-behavior-x: auto;
    }
    </style></head><body><pre>"#
      .to_string(),
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
