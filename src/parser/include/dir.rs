use std::{ops::Range, path::Path};

use crate::{info, utils::utils::{color_file, load_dir}, build::Directories, parser::include::md::include_md, error};

/// Function for including a directory with markdown files.
pub fn include_dir(dir: &mut Directories, path: &str, range: Range<usize>, output: &mut String) -> Result<(), String> {

  let path = path.replace("/*", "/");
  let path = Path::new(&path);

  info!("routing", Color::BrightMagenta, 
    "'{}' {} {}", &path.to_string_lossy(), 
    "->".bright_black(), 
    color_file(&dir.parent_file)
  );

  if let Ok(folder) = load_dir(&dir.code_dir, format!("{}/{}", dir.relative_path, path.to_string_lossy()).as_str()) {

    // Format the dist directory for this directory:
    let file_stem = Path::new(&dir.parent_file).file_stem().unwrap().to_string_lossy();
    let dist_dir = format!("{}/dist/{}/", dir.code_dir, file_stem);
    std::fs::create_dir_all(&dist_dir).expect("Failed to create dist directory");

    // Loop over all markdown files in the folder:
    for file in folder {
      let file = file.unwrap();

      // Load the markdown file from the directory.
      let md = std::fs::read_to_string(&file.path());
      
      if let Ok(md) = md {
        // Process the markdown file into html:
        let mut html = output.clone();
        include_md(dir, &md, &format!("{}/{}", &path.to_string_lossy(), file.file_name().to_string_lossy()), range.clone(), &mut html)?;

        // Save the html as a page.
        std::fs::write(&format!("{}/{}.html", &dist_dir, file.path().file_stem().unwrap().to_string_lossy()), html).expect("Failed to write routed page file");
      }
      else {
        error!("failed to load file '{}'", file.path().to_string_lossy());
      }
    }
  }

  output.replace_range(range.clone(), "");

  Ok(())
}