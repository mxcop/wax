// Wax Component Logic

use crate::utils::files::load_file;

/// Wax Component
struct Comb {
  contents: String
}

impl Comb {
  /// Create a new Wax component
  pub fn new(file_path: &String) -> Comb {
    // Try to load the component file :
    if let Ok(contents) = load_file(file_path) {
      Comb { contents }
    } else {
      Comb { contents: String::new() }
    }
  }

  /// Compile the Wax component.
  pub fn compile(&self) -> String { String::new() }
}