use std::path::Path;

use colored::{Colorize, Color};
use regex::Regex;

use crate::{utils::{color_file, load_file}, printpro};

pub fn include(code_path: &str, relative_path: &str, parent_file: &str, mut output: String) -> Result<String, String> {

  // Check if this file has already been generated and cached:
  let cached_path = format!("./.wax/{}/{}", relative_path, parent_file);
  if let Ok(cached) = std::fs::read_to_string(&cached_path) {
    // DEBUG //
    printpro!("recycle", Color::Blue, color_file(&parent_file));

    return Ok(cached);
  }

  // Use regex to find all the <wax!> elements:
  let exp = Regex::new(r"<wax!.*?>").expect("Regex failed");

  // Check if this component has child components.
  let has_children = exp.captures(&output).is_some();

  while let Some(caps) = exp.captures(&output) {
    // Get the range and element from the captures:
    let range = caps.get(0).unwrap().range();
    let element = caps.get(0).unwrap().as_str().replace(" ", "");

    // Use regex to extract the path attribute:
    let exp = Regex::new(r#"path="(?s)(.*)""#).expect("Regex failed");
    
    if let Some(path) = exp.captures(element.as_str()) {

      // Get the path from the regex match.
      let path = path.get(1).unwrap().as_str();

      // Load the contents of the html file given in the path:
      if let Ok(mut subcontents) = load_file(code_path, format!("{}/{}", relative_path, path).as_str()) {
        
        // Remove the filename & extension from the path to get the directory of this file.
        // This is important for relative import paths.
        let file_dir = Path::new(&path).ancestors().nth(1).unwrap().to_str().unwrap();

        // DEBUG //
        let file_name = Path::new(&path).file_name().unwrap().to_str().unwrap();

        if file_name == parent_file {
          printpro!("error! ", Color::Red, format!("({}) recursive include detected", parent_file.red()));
          return Err("Cannot include component within itself".into());
        }

        // First handle the <wax!> elements inside this component.
        match include(code_path, format!("{}/{}", relative_path, file_dir).as_str(), file_name, subcontents) {
          Ok(result) => subcontents = result,
          Err(e) => return Err(e)
        }

        // Then include the result.
        output.replace_range(range, &subcontents);
      } else {
        printpro!("warn!  ", Color::Yellow, format!("({}) failed to load '{}'", parent_file.yellow(), path));
        break;
      }

      // DEBUG //
      printpro!("waxing ", Color::Green, 
        format!("{} {} {}", color_file(&path), 
        "->".black(), 
        color_file(&parent_file))
      );

    } else {
      printpro!("warn!  ", Color::Yellow, format!("({}) missing a 'path' attribute", parent_file.yellow()));
      break;
    }
  }

  if has_children {
    let cache_path = format!("./.wax/{}", relative_path);
    let cache_file = format!("{}/{}", cache_path, parent_file);

    // Write a copy of this component to the disk:
    std::fs::create_dir_all(cache_path).expect("Failed to create .wax cache directory");
    std::fs::write(cache_file, &output).expect("Failed to write cache");
  }

  Ok(output)
}