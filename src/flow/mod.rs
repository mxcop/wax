mod include;
mod params;

use std::ops::Range;
use regex::{Regex, Captures};

use crate::{build::Directories, info, utils::utils::color_file};

use include::wax_include;

/// Process a html file.
pub fn wax(dir: &mut Directories, mut output: String) -> Result<String, String> {

  // Check if this file has already been generated and cached:
  let cached_path = format!("{}/.wax/{}/{}", dir.code_dir, dir.relative_path, dir.parent_file);
  if let Ok(cached) = std::fs::read_to_string(&cached_path) {
    info!("recycle", Color::Blue, color_file(&dir.parent_file));
    return Ok(cached);
  }

  // Use regex to find all the <wax!> elements:
  let exp = Regex::new(r"<wax!.*?>").expect("Regex failed");

  // Check if this component has child components.
  let has_children = exp.captures(&output).is_some();

  // Wax all the <wax!> components within this component.
  while let Some(caps) = exp.captures(&output) {

    // Get the range and element from the captures:
    let (range, element) = from_captures(caps);

    //output = wax_params()?;
    output = wax_include(dir, &range, &element, output)?;
  }

  // Cache this component if it has child components.
  if has_children {
    let cache_path = format!("{}/.wax/{}", dir.code_dir, dir.relative_path);
    let cache_file = format!("{}/{}", cache_path, dir.parent_file);

    // Write a copy of this component to the disk:
    std::fs::create_dir_all(cache_path).expect("Failed to create .wax cache directory");
    std::fs::write(cache_file, &output).expect("Failed to write cache");
  }

  Ok(output)
}

/// Returns a tuple containing (0th range, 0th as string with no whitespace).
/// 
/// # Examples
/// 
/// Basic usage:
/// ```
/// let re = Regex::new(r"<wax!.*?>").unwrap();
/// let caps = re.captures(r#"<wax! path="./test.html">"#).unwrap();
/// 
/// let (range, element) = from_captures(caps);
/// assert_eq!(r#"<wax!path="./test.html">"#, element);
/// ```
fn from_captures(caps: Captures) -> (Range<usize>, String) {
  (caps.get(0).unwrap().range(), caps.get(0).unwrap().as_str().replace(" ", ""))
}