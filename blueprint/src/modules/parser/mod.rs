use std::path::Path;
use std::{fs};
use crate::modules::errors::{BlueprintResult, BlueprintError};

pub mod description;
pub mod props;
pub mod slots;
pub mod example;


static DEAULT_COMPONENT: &str = "component"; 

#[derive(Debug, Clone)]

pub struct SvelteComponent{
  pub name: String,
  pub description:  Option<String>,
  pub props: Option<Vec<props::ComponentProp>>,
  pub slots: Option<Vec<slots::ComponentSlot>>,
  pub example: Option<String>,
}


impl SvelteComponent {
  pub fn new(path: &Path) -> BlueprintResult<SvelteComponent> {
    let content = match fs::read_to_string(path) {
      Ok(c) => {c},
      Err(e) => { 
         return Err(BlueprintError::ParserReadSource(format!("{}", e)));
      }
    };

    // Get name of component
    let name = match path.file_stem() {
      Some(n) => {
        if let Some(str_name) = n.to_str() {
          str_name
        } else { DEAULT_COMPONENT }},
      None => { DEAULT_COMPONENT }
    };


    Ok(SvelteComponent {
      name: format!("{}", name), 
      description: description::parse(&content),
      props: props::parse(&content), 
      slots: slots::parse(&content), 
      example: example::parse(&content)
    })

  }

}



#[cfg(test)]
mod test_component {
  use super::*;

  #[test]
  fn test_full_component() {
    let component = SvelteComponent::new(Path::new("tests/Table.svelte")).unwrap();
    assert_eq!(component.name, String::from("Table"));
    assert_eq!(component.description, Some(String::from("Componente para tablas")));
    assert_ne!(component.example, None);
  }
}