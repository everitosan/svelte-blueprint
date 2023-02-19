use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
  static ref EXAMPLE_PATTERN: Regex = Regex::new(r#"<!--E([^#]*)-->"#).unwrap();
}


pub fn parse(content: &String) -> Option<String> {
  match EXAMPLE_PATTERN.captures(content) {
    Some(cap) => {
      let value = &cap[1].to_string();
      return Some(value.trim().to_owned());
    }, 
    None => {
      return None
    }
  }
}

// Run the following tests with cargo test modules::parser::example

#[cfg(test)]
mod test_example {
  use super::*;

  #[test]
  fn test_one_line_text() {
    let text = String::from("<!--E Contenido de ejemplo -->");
    let example = parse(&text);
    assert_eq!(example, Some("Contenido de ejemplo".to_string()));
  }

  #[test]
  fn test_several_lines_text() {
    let text = format!("{}", r#"
<!--E
  <Button  />
  <Button text="One" />
  <Button text='Two' />
  <Button text='Three' />
-->
    "#);
    let desc = parse(&text);
    assert_eq!(desc, Some("<Button  />\n  <Button text=\"One\" />\n  <Button text='Two' />\n  <Button text='Three' />".to_string()));
  }
}