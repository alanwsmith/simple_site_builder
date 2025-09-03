use crate::builder::trim_empty_leading_lines;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

pub fn highlight_code(
  code: &str,
  lang: &str,
) -> String {
  let syntax_set = SyntaxSet::load_defaults_newlines();
  let syntax =
    syntax_set.find_syntax_by_token(lang).unwrap_or_else(
      || syntax_set.find_syntax_plain_text(),
    );
  let mut html_generator =
    ClassedHTMLGenerator::new_with_class_style(
      syntax,
      &syntax_set,
      ClassStyle::Spaced,
    );
  for line in LinesWithEndings::from(
    &trim_empty_leading_lines(code),
  ) {
    let _ = html_generator
      .parse_html_for_line_which_includes_newline(line);
  }
  let initial_html = html_generator.finalize();
  let output_html: Vec<_> = initial_html
    .lines()
    .map(|line| {
      format!(
        r#"<span class="line-marker"></span>{}"#,
        line
      )
    })
    .collect();
  output_html.join("\n")
}
