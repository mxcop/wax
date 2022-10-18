use regex::Regex;

use crate::utils;

pub fn include(code_path: &str, relative_path: &str, parent_file: &str, mut output: String) -> String {

  // Check if this file has already been generated and cached:
  let cached_path = format!("./.wax/{}/{}", relative_path, parent_file);
  if let Ok(cached) = std::fs::read_to_string(&cached_path) {
    // DEBUG //
    println!("~ loading '{}' from cache", parent_file);

    return cached;
  }

  // Use regex to find all the <wax> elements:
  let exp = Regex::new(r"<wax.*?>").expect("Regex failed");

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
      if let Ok(mut subcontents) = utils::load_file(code_path, format!("{}/{}", relative_path, path).as_str()) {
        
        // Remove the filename & extension from the path to get the directory of this file.
        // This is important for relative import paths.
        let file_dir = std::path::Path::new(&path).ancestors().nth(1).unwrap().to_str().unwrap();

        // DEBUG //
        let file_name = std::path::Path::new(&path).file_name().unwrap().to_str().unwrap();

        // First handle the <wax> elements inside this component.
        subcontents = include(code_path, format!("{}/{}", relative_path, file_dir).as_str(), file_name, subcontents);

        // Then include the result.
        output.replace_range(range, &subcontents);
      } else {
        println!("warn: failed to load '{}'", format!("{} / {}", relative_path, path));
        break;
      }

      // DEBUG //
      let file_name = std::path::Path::new(&path).file_name().unwrap().to_str().unwrap();
      println!("~ waxing '{}' into '{}'", file_name, parent_file);

    } else {
      println!("warn: missing 'path' attribute on wax element");
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

  output
}