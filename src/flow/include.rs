use std::{path::Path, ops::Range};

use colored::{Colorize, Color};
use regex::Regex;

use crate::{utils::{color_file, load_file}, printpro, Directories, flow};

pub fn wax_include(dir: &mut Directories, range: &Range<usize>, element: &str, mut output: String) -> Result<String, String> {
  // Use regex to extract the path attribute:
  let exp = Regex::new(r#"src="(?s)(.*)""#).expect("Regex failed");
  
  if let Some(path) = exp.captures(element) {

    // Get the path from the regex match.
    let path = path.get(1).unwrap().as_str();

    // Load the contents of the html file given in the path:
    if let Ok(mut subcontents) = load_file(&dir.code_dir, format!("{}/{}", dir.relative_path, path).as_str()) {
      
      // Remove the filename & extension from the path to get the directory of this file.
      // This is important for relative import paths.
      let file_dir = Path::new(&path).ancestors().nth(1).unwrap().to_str().unwrap();

      // DEBUG //
      let file_name = Path::new(&path).file_name().unwrap().to_str().unwrap();

      if file_name == dir.parent_file {
        printpro!("error! ", Color::Red, format!("({}) recursive include detected", dir.parent_file.red()));
        return Err("Cannot include component within itself".into());
      }

      let mut new_dir = dir.clone();
      new_dir.relative_path = format!("{}/{}", dir.relative_path, file_dir);

      // First handle the <wax!> elements inside this component.
      match flow::wax(&mut new_dir, subcontents) {
        Ok(result) => subcontents = result,
        Err(e) => return Err(e)
      }

      // Then include the result.
      output.replace_range(range.clone(), &subcontents);
    } else {
      printpro!("warn!  ", Color::Yellow, format!("({}) failed to load '{}'", dir.parent_file.yellow(), path));
      return Ok(String::new());
    }

    // DEBUG //
    printpro!("waxing ", Color::Green, 
      format!("{} {} {}", color_file(&path), 
      "->".black(), 
      color_file(&dir.parent_file))
    );

  } else {
    printpro!("warn!  ", Color::Yellow, format!("({}) missing a 'path' attribute", dir.parent_file.yellow()));
    return Ok(String::new());
  }

  Ok(output)
}