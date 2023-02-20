use std::path::{PathBuf, Component};
use std::fs::{create_dir_all, write};
use askama::Template;

use crate::modules::errors::{BlueprintResult, BlueprintError};
use crate::modules::parser::{SvelteComponent};

static DEAULT_TEMPLATE: &str = "default-template.svelte";

pub struct DocumentParams<'a> {
  pub destination: &'a PathBuf,
  pub template: &'a Option<PathBuf>,
}

#[derive(Template)]
#[template(path = "component.html", escape = "none")]
struct ComponentTemplate<'a, 'b> {
  template_path: &'a str,
  component_source: &'a str,
  component: &'b SvelteComponent,
}

pub struct Blueprint<'a> {
  params: DocumentParams<'a>,
}

impl<'a> Blueprint<'a> {
  pub fn new(params: DocumentParams<'a>) -> Blueprint<'a> {
    return Blueprint { params };
  }

  pub fn parse(&self, file_path: &PathBuf) -> BlueprintResult<()> {
    // Create dir for output
    if !self.params.destination.exists() {
      match create_dir_all(&self.params.destination) {
        Ok(_) => {},
        Err(e) => {
          return Err(BlueprintError::DestinationDir(format!("{}", e)));
        }
      };
    }

    // Parse component
    let component = SvelteComponent::new(file_path)?;

    // Define Blueprint template to use
    let template_path= match self.params.template {
      Some(t) => {
        if let Some(t) = t.to_str() { t } else { DEAULT_TEMPLATE }
      },
      None => { DEAULT_TEMPLATE }
    };

    let component_source = relative_path(&self.params.destination, file_path)?;
    let component_source = match component_source.to_str() {
      Some(c) => {c},
      None => { return Err(BlueprintError::RelativePath) }
    };

    let c_t = ComponentTemplate {
      template_path: template_path,
      component_source: component_source,
      component: &component.clone()
    };

    // Render output
    let content = match c_t.render() {
      Ok(c) => {c},
      Err(e) => {
        return Err(BlueprintError::OutputFile(format!("{}", e)));
      }
    };

    // Write blueprint file
    let mut output_file = self.params.destination.join(component.name);
    output_file.set_extension("svelte");
    match write(output_file, content) {
      Ok(_) => {},
      Err(e) => { 
        return Err(BlueprintError::OutputFile(format!("{}", e)));
      }
    };

    Ok(())
  }
}


fn relative_path(from: &PathBuf, to: &PathBuf) -> BlueprintResult<PathBuf> {
  if to.is_absolute() {
    if let Some(to_str) = to.to_str() {
      return Ok(PathBuf::from(to_str))
    } else {
      return Err(BlueprintError::RelativePath)
    }
   
  } else {
    let mut result = PathBuf::from("");
    for c in from.components() {
      match c {
        Component::Normal(_) => {
          result = result.join("../");
        },
        _ => {}
      };
    }
    for c in to.components() {
      match c {
        Component::Normal(p) => {
          result = result.join(p);
        },
        _ => {}
      }
    }
    return Ok(result);
  }
}


#[cfg(test)]
mod test_blueprint {
  use super::*;
  use std::path::Path;

  #[test]
  fn test_get_component_path() {
    let from = PathBuf::from("./Docs/");
    let to = PathBuf::from("tests/Table.svelte");
    let result = relative_path(&from, &to).unwrap();

    assert_eq!(result, PathBuf::from("../tests/Table.svelte"))
  }

  #[test]
  fn test_document_component() {
    let params = DocumentParams {
      destination: &PathBuf::from("./Docs/pages/random"),
      template: &None
    };

    let source = PathBuf::from("tests/Table.svelte");
    let bp = Blueprint::new(params);
    let response = bp.parse(&source).unwrap();
    let res_path = Path::new("./Docs/pages/random/Table.svelte").exists();

    assert_eq!(response, ());
    assert!(res_path);
  }

}