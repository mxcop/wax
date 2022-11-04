use colored::Colorize;

use crate::{flow::wax, error, utils::{conf::{get_conf, WaxConfig}, utils::load_file}};

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


  println!("{} building '{}'\n", "Wax".green().bold(), &path);
  let start = std::time::Instant::now();


  let mut output = String::new();

  let mut dir = Directories { 
    code_dir: path.clone(), 
    work_dir: work_dir.clone(), 
    relative_path: conf.website.pages.clone(), 
    parent_file: "index.html".into() 
  };

  // Attempt to read the index file:
  if let Ok(contents) = load_file(&path, format!("{}/index.html", &dir.relative_path).as_str()) {
    match wax(&mut dir, contents) {
      Ok(result) => output = result,
      Err(e) => {
        println!("\n{} failed ({})", "Wax".red().bold(), e);
        return;
      }
    }
  }

  // Check the build options in the config:
  if let Some(build) = conf.build {
    if let Some(true) = build.minify {
      // Remove all newlines.
      let re = regex::Regex::new(r"<!--(.*?)-->|\s\B").unwrap();
      output = re.replace_all(&output, "").to_string();
    }
  }

  // Write the output to the disk:
  std::fs::create_dir_all(format!("{}/dist", &path)).expect("Failed to create ./dist directory");
  std::fs::write(format!("{}/dist/index.html", &path), &output).expect("Failed to write output");

  println!("\n{} finished in {}ms", "Wax".green().bold(), start.elapsed().as_millis());
}

#[derive(Clone)]
pub struct Directories {
  pub work_dir: String,
  pub code_dir: String,

  pub relative_path: String,
  pub parent_file: String,
}