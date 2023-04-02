use std::{path::Path, fs};

use waxc_errors::error::{WaxError, WaxHint};
use crate::page::OutputPage;

/// Recursively walk through all pages in the pages directory.
pub(crate) fn walk_pages(pages: &mut Vec<OutputPage>, dir: &Path, cb: &dyn Fn(&Path) -> Result<OutputPage, WaxError>) -> Result<(), WaxError> {
  if dir.is_dir() {
    let Ok(dir) = fs::read_dir(dir) else {
      // TODO: improve error message:
      return Err(WaxError::new(
        0, 0, 
        "couldn't read the pages directory", 
      WaxHint::None, None));
    };

    for entry in dir {
      let Ok(entry) = entry else {
        // TODO: improve error message:
        return Err(WaxError::new(
          0, 0, 
          "couldn't read entry in pages directory", 
        WaxHint::None, None));
      };
      let path = entry.path();
      if path.is_dir() {
        walk_pages(pages, &path, cb)?;
      } else {
        pages.push(cb(&entry.path().as_path())?);
      }
    }
  }
  Ok(())
}
