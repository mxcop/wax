use colored::Colorize;

use crate::{parser::wax, error, utils::{conf::{get_conf, WaxConfig}, files::visit_dirs}};

/** Build a wax project */
pub fn build(path: String) {
  let work_dir = format!("{}/..", &path);

  println!("\n{} loading config", "Wax".green().bold());


  let conf: WaxConfig;

  // Attempt to read the wax config file:
  match get_conf(&path) {
    Ok(c) => conf = c,
    Err(e) => {
      error!(e);
      return;
    },
  }

  // Delete the .wax directory if it exists.
  if std::path::Path::new(format!("{}/.wax", &path).as_str()).exists() {
    std::fs::remove_dir_all(format!("{}/.wax", &path)).expect("Failed to remove .wax directory");
  }


  println!("{} building '{}'", "Wax".green().bold(), &path);
  let start = std::time::Instant::now();

  // Get the path to the pages directory:
  let pages = format!("{}/{}", &path, &conf.website.pages);
  let pages_path = std::path::Path::new(&pages);

  // Check if the pages path points to a directory.
  if pages_path.is_dir() == false {
    println!("\n{} failed ({})", "Wax".red().bold(), "pages path doesn't point to a directory");
    return;
  }

  // Walk through all pages in the pages directory.
  visit_dirs(pages_path, &|file| {

    println!("\n{} {}", "&".bright_black(), file.file_name().to_string_lossy().on_black().bright_magenta().italic());

    let mut output = String::new();

    let dir = Directories { 
      code_dir: path.clone(), 
      work_dir: work_dir.clone(), 
      relative_path: conf.website.pages.clone(), 
      parent_file: file.file_name().to_string_lossy().to_string()
    };

    // Attempt to read the file:
    if let Ok(contents) = std::fs::read_to_string(file.path()) {
      match wax(&mut dir.clone(), contents) {
        Ok(result) => output = result,
        Err(e) => {
          println!("\n{} failed ({})", "Wax".red().bold(), e);
          return;
        }
      }
    }

    // Check the build options in the config:
    if let Some(build) = &conf.build {
      if let Some(true) = build.minify {
        use minify_html::{Cfg, minify};

        // Minify the final document using minify_html crate:
        let cfg = Cfg::new();
        let minified = minify(output.as_bytes(), &cfg);

        output = String::from_utf8_lossy(&minified).to_string();
      }
    }

    // Write the output to the disk:
    std::fs::create_dir_all(format!("{}/dist", &path)).expect("Failed to create ./dist directory");
    std::fs::write(format!("{}/dist/{}", &path, file.file_name().to_string_lossy()), &output).expect("Failed to write output");

  }).unwrap();

  println!("\n{} finished in {}ms", "Wax".green().bold(), start.elapsed().as_millis());
}

#[derive(Clone)]
pub struct Directories {
  pub work_dir: String,
  pub code_dir: String,

  pub relative_path: String,
  pub parent_file: String,
}