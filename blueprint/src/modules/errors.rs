use std::result;

#[derive(Debug)]
pub enum BlueprintError {
  ParserReadSource(String),
  DestinationDir(String),
  OutputFile(String),
  RelativePath,
}

impl std::fmt::Display for BlueprintError {
  fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    let content = match self {
      Self::ParserReadSource(c) => format!("Parser error reading source: {}", c),
      Self::DestinationDir(c) => format!("Destination write error: {}", c),
      Self::OutputFile(c) => format!("Blueprint ouput error: {}", c),
      Self::RelativePath => format!("Relative path calculation error"),
    };

    write!(formatter, "{}", content)
  }
}


pub type BlueprintResult<T> = result::Result<T, BlueprintError>;