use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
  static ref DESCRIPTION_PATTERN: Regex = Regex::new(r#"<!--D([a-zA-Z \n áÁéÉíÍóÓúÚ]+)?-->"#).unwrap();
}

pub fn parse(content: &String) -> Option<String> {
  match DESCRIPTION_PATTERN.captures(content) {
    Some(cap) => {
      let value = &cap[1].to_string();
      return Some(value.trim().to_owned());
    }, 
    None => {
      return None
    }
  }
}

// Run the following tests with cargo test modules::parser::description

#[cfg(test)]
mod test_description {
  use super::*;

  #[test]
  fn test_one_line_text() {
    let text = String::from("<!--D Componente de botón -->");
    let desc = parse(&text);
    assert_eq!(desc, Some("Componente de botón".to_string()));
  }

  #[test]
  fn test_several_lines_text() {
    let text = format!("{}", r#"
<!--D
  Componente de botón
-->
    "#);
    let desc = parse(&text);
    assert_eq!(desc, Some("Componente de botón".to_string()));
  }
}