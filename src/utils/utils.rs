use std::path::Path;

use colored::Colorize;

/** Load a file using relative or absolute path */
pub fn load_file(code_path: &str, path: &str) -> std::io::Result<String> {
  let path = format!("{}/{}", code_path, path);
  let path = Path::new(&path);
  std::fs::read_to_string(path)
}

/** Get the file name & file extension from a path (can panic) */
pub fn name_and_ext(path: &str) -> (&str, &str) {
  let path = Path::new(path);
  let name = path.file_stem().unwrap();
  let ext = path.extension().unwrap();
  (name.to_str().unwrap(), ext.to_str().unwrap())
}

/** Convert a file path to a colorful debug representation (can panic) */
pub fn color_file(path: &str) -> String {
  let (name, ext) = name_and_ext(path);
  format!("{}{}{}", name, ".".bright_black(), ext.bright_black())
}