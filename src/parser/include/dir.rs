use std::ops::Range;

use crate::{info, utils::utils::color_file, build::Directories};

/// Function for including a directory with markdown files.
pub fn include_dir(dir: &mut Directories, path: &str, element: &str, range: Range<usize>, output: &mut String) -> Result<(), String> {

  info!("routing", Color::BrightMagenta, 
    "{}", color_file(&dir.parent_file)
  );

  output.replace_range(range.clone(), "");

  Ok(())
}