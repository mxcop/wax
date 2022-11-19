#[derive(Debug)]
pub struct SyntaxError {
  pub description: String,
  pub origin: String,
  pub line_number: usize,
  pub char_number: usize,
  pub line: String,

  pub help: Option<String>,
}

impl SyntaxError {
  
  pub fn with_help(desc: &str, origin: &str, line_number: usize, char_number: usize, line: &str, help: &str) -> SyntaxError {
    SyntaxError {
      description: desc.to_string(),
      origin: origin.to_string(),
      line_number,
      char_number,
      line: line.to_string(),
      help: Some(help.to_string())
    }
  }

  pub fn new(desc: &str, origin: &str, line_number: usize, char_number: usize, line: &str) -> SyntaxError {
    SyntaxError {
      description: desc.to_string(),
      origin: origin.to_string(),
      line_number,
      char_number,
      line: line.to_string(),
      help: None
    }
  }
}