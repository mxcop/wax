use std::{ops::Range, path::Path};

use crate::{info, utils::utils::color_file, build::Directories};

/// Function for parsing and then including a markdown file.
pub fn include_md(dir: &mut Directories, contents: &str, path: &str, range: Range<usize>, output: &mut String) -> Result<(), String> {

  // Check if this file has already been generated and cached:
  let cached_path = format!("{}/.wax/{}/{}", dir.code_dir, dir.relative_path, path);
  if let Ok(cached) = std::fs::read_to_string(&cached_path) {
    info!("recycle", Color::Blue, color_file(&path));

    // Include the cached file.
    output.replace_range(range.clone(), &cached);

    return Ok(());
  }

  // Logging:
  info!("waxing ", Color::Green, 
    "{} {} {}", color_file(&path), 
    "->".bright_black(), 
    color_file(&dir.parent_file)
  );
  
  // Create the markdown parser.
  let parser = pulldown_cmark::Parser::new(&contents);

  // Write to a new String buffer.
  let mut html_output = String::new();
  pulldown_cmark::html::push_html(&mut html_output, parser);

  // Include the result.
  output.replace_range(range.clone(), &html_output);

  // Cache the result.
  let cache_path = format!("{}/.wax/{}", dir.code_dir, dir.relative_path);
  let cache_file = format!("{}/{}", cache_path, path);
  let cache_path = Path::new(&cache_file).parent().expect("Failed to get cache path");

  // Write a copy of this markdown file to the disk:
  std::fs::create_dir_all(&cache_path).expect("Failed to create .wax cache directory");
  std::fs::write(&cache_file, &html_output).expect("Failed to write cache");

  Ok(())
}