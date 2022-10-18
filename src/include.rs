use regex::{Captures, Regex};

use crate::utils;

pub fn include(caps: Captures, code_path: &str, relative_path: &String, mut output: String) -> String {

  // Get the range and element from the captures:
  let range = caps.get(0).unwrap().range();
  let element = caps.get(0).unwrap().as_str().replace(" ", "");

  // Use regex to extract the path attribute:
  let exp = Regex::new(r#"path="(?s)(.*)""#).expect("Regex failed");
  
  if let Some(path) = exp.captures(element.as_str()) {

    // Get the path from the regex match.
    let path = path.get(1).unwrap().as_str();
    println!("found <wax> with path '{}'", path);

    // Load the contents of the html file given in the path:
    if let Ok(subcontents) = utils::load_file(code_path, format!("{}/{}", relative_path, path).as_str()) {
      output.replace_range(range, &subcontents);
    } else {
      println!("warn: failed to load '{}'", path);
    }

  } else {
    println!("warn: missing 'path' attribute on wax element");
  }

  output
}