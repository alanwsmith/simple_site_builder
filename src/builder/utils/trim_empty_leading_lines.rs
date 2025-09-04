use regex::Regex;

pub fn trim_empty_leading_lines(source: &str) -> String {
  let re = Regex::new(r"\S").unwrap();
  let trimmed_front =
    source.split("\n").fold("".to_string(), |acc, l| {
      if !acc.is_empty() {
        acc + l + "\n"
      } else if re.is_match(l) {
        l.to_string() + "\n"
      } else {
        acc
      }
    });
  trimmed_front.trim_end().to_string()
}
