use std::{path::Path, ops::Range};

use crate::{info, utils::utils::color_file, build::Directories, error, flow::{params::wax_params, self}};

/// Function for including a HTML file using Wax.
pub fn include_html(dir: &mut Directories, mut contents: String, path: &str, element: &str, range: Range<usize>, output: &mut String) -> Result<(), String> {

  // Remove the filename & extension from the path to get the directory of this file.
  // This is important for relative import paths.
  let file_dir = Path::new(&path).ancestors().nth(1).unwrap().to_str().unwrap();
  let file_name = Path::new(&path).file_name().unwrap().to_str().unwrap();

  if file_name == dir.parent_file {
    error!("({}) recursive include detected", dir.parent_file.red());
    return Err("Cannot include component within itself".into());
  }

  let mut new_dir = dir.clone();
  new_dir.relative_path = format!("{}/{}", dir.relative_path, file_dir);
  new_dir.parent_file = file_name.into();

  // Wax all the parameters within this component.
  contents = wax_params(&new_dir, &mut contents, element)?;

  // Remove any whitespace around the content.
  contents = contents.trim().to_string();

  // First handle the <wax!> elements inside this component.
  match flow::wax(&mut new_dir, contents) {
    Ok(result) => contents = result,
    Err(e) => return Err(e)
  }

  // Then include the result.
  output.replace_range(range.clone(), &contents);

  // Logging:
  info!("waxing ", Color::Green, 
    "{} {} {}", color_file(&path), 
    "->".bright_black(), 
    color_file(&dir.parent_file)
  );

  Ok(())
}