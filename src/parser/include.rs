use std::{path::Path, ops::Range, ffi::OsStr};

use regex::Regex;

use crate::{utils::utils::{load_file}, build::Directories, warn};

mod html;
mod md;
mod dir;

pub fn wax_include(dir: &mut Directories, range: &Range<usize>, element: &str, output: &mut String) -> Result<String, String> {
  
  // Use regex to extract the path attribute:
  let exp = Regex::new(r#"src="(?s)(.*?)""#).expect("Regex failed");
  
  if let Some(path) = exp.captures(element) {

    // Get the path from the regex match.
    let path = path.get(1).unwrap().as_str();

    // Get the file extension.
    let extension = Path::new(&path).extension().unwrap_or(OsStr::new("")).to_str().unwrap();

    // Load the contents of the html file given in the path:
    if let Ok(subcontents) = load_file(&dir.code_dir, format!("{}/{}", dir.relative_path, path).as_str()) {
      
      if extension == "html" || extension == "htm" {
        html::include_html(dir, subcontents, path, element, range.clone(), output)?;
      }
      else if extension == "md" {
        md::include_md(dir, &subcontents, path, range.clone(), output)?;
      }

    } 
    else if path.ends_with("/*") {
      dir::include_dir(dir, path, element, range.clone(), output)?;
    }
    else {
      warn!("({}) failed to load '{}'", dir.parent_file.yellow(), path);
      return Ok(String::new());
    }

  } else {
    warn!("({}) missing a 'src' attribute", dir.parent_file.yellow());
    return Ok(String::new());
  }

  Ok(output.to_string())
}