pub mod document;

use std::path::PathBuf;
use log::{error, info };
use walkdir::{WalkDir};


pub fn process(source: &PathBuf, params: document::DocumentParams) -> Vec<String> {
  let blueprint = document::Blueprint::new(params);
  let mut blueprint_files :Vec<String> = vec![];

  if source.is_dir() {
    info!("Documenting directory: {}", source.to_str().unwrap());

    let walker = WalkDir::new(source.to_str().unwrap()).into_iter();
  
    for entry in walker.filter_map(|e| e.ok()) {
      let ft = entry.file_name().to_str().unwrap();

      let path_string = format!("{}", entry.path().display());
      let current_source = PathBuf::from(path_string.clone());

      if current_source.is_file() && ft.ends_with("svelte") {
        info!("\tProcessing file {}", path_string);
        match blueprint.parse(&current_source) {
          Ok(_) => {
            blueprint_files.push(path_string)
          },
          Err(e) => { error!("{}", e);}
        };
      }
      
    }
  } else {
    let path_string = source.to_str().unwrap();
    info!("Documenting file: {}", path_string);
    match blueprint.parse(source) {
      Ok(_) => {
        blueprint_files.push(path_string.to_string())
      },
      Err(e) => { error!("{}", e);}
    };
  }

  return blueprint_files;
}