use std::ops::Range;

use crate::{info, utils::utils::color_file, build::Directories};

pub fn include_md(dir: &mut Directories, contents: &str, path: &str, range: Range<usize>, output: &mut String) -> Result<(), String> {

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

  // Then include the result.
  output.replace_range(range.clone(), &html_output);

  Ok(())
}