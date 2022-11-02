use regex::Regex;

use crate::{error, Directories};

pub fn wax_params(dir: &Directories, contents: &mut String, import: &str) -> Result<String, String> {
  // Use regex to find the first <params!> element:
  let exp = Regex::new(r"<params!(.*?)>").expect("Regex failed");

  // Get the first match of the regex.
  let contents_clone = contents.clone();
  let cap = exp.captures(&contents_clone);

  if let Some(cap) = cap {

    // Get the parameters from the params element.
    let element = cap.get(1).unwrap().as_str().trim_start();

    // Remove the <params!> element from the contents:
    let range = cap.get(0).unwrap().range();
    contents.replace_range(range, "");

    // Split the parameters into a vector.
    let params = element.split(' ').collect::<Vec<&str>>();

    for param in params {
      // Use regex to extract the attribute from the import element.
      let exp = Regex::new(format!(r#"{}="(?s)(.*?)""#, param).as_str()).expect("Regex failed");

      // Try to extract the attribute:
      if let Some(caps) = exp.captures(&import) {
        if let Some(value) = caps.get(1) {
          let value = value.as_str();

          // Use regex to replace the params within the component.
          let exp = Regex::new(format!(r"\{{(\s)*?({})(\s)*?\}}", param).as_str()).expect("Regex failed");

          // Move our way through the params and replace them with their value:
          while let Some(caps) = exp.captures(&contents) {
            let range = caps.get(0).expect("Regex Failed").range();
            contents.replace_range(range, value);
          }
        }
      }
      else { error!("({}) missing parameter '{}'", dir.parent_file.red(), param); return Err("missing parameter".into()); }
    }
  }
  
  Ok(contents.to_string())
}