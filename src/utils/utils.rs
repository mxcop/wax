// use std::{path::Path, fs::ReadDir};

// use colored::Colorize;

// /** Load a file using relative or absolute path */
// pub fn load_file(code_path: &str, path: &str) -> std::io::Result<String> {
//   let path = format!("{}/{}", code_path, path);
//   let path = Path::new(&path);
//   std::fs::read_to_string(path)
// }