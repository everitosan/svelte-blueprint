use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
  static ref SLOT_PATTERN: Regex = Regex::new(r#"<slot([ ]+\n)?[\n ]*(name=)?([\w'"])*"#).unwrap();
  static ref SLOT_NAME_PATTERN: Regex = Regex::new(r#"name=["']?([a-zA-Z0-9!@#\$%\^\&*\)\(+=._ -]+)?['"]?"#).unwrap();
}

#[derive(Debug, Clone)]
pub struct ComponentSlot {
  pub name: String,
  pub description: Option<String>
}

pub fn parse(content: &String) -> Option<Vec<ComponentSlot>> {
  let mut slots: Vec<ComponentSlot> = vec![];

  for cap in SLOT_PATTERN.captures_iter(content) {
    let slot_line = cap.get(0).map_or("", |m| m.as_str());

    match SLOT_NAME_PATTERN.captures(slot_line) {
      Some(name ) => {
        slots.push(ComponentSlot {
          name: String::from(&name[1]), 
          description: None
        });
      },
      None => {
        slots.push(ComponentSlot {
          name: String::from("default"), 
          description: None
        });
      }
    };

  }

  if slots.len() == 0 {
    return None
  }

  return Some(slots);
}


// Run the following tests with cargo test modules::parser::slots

#[cfg(test)]
mod test_slots {
  use super::*;

  #[test]
  fn test_typescript_text() {

    let text = format!("{}", r#"
<slot> // default slot

</slot>


<slot name="one"> // Slot for number one

</slot>

<slot name='two' > // Slot for number two

</slot>


<slot
  name='three' > // Slots for number three

</slot>
    "#);
    
    let slots = parse(&text).unwrap();
    assert_eq!(slots.len(), 4);

    let prop  = slots.get(0).unwrap();
    assert_eq!(prop.name, String::from("default"));

    let prop  = slots.get(1).unwrap();
    assert_eq!(prop.name, String::from("one"));

    let prop  = slots.get(2).unwrap();
    assert_eq!(prop.name, String::from("two"));

    let prop  = slots.get(3).unwrap();
    assert_eq!(prop.name, String::from("three"));
  }
}
