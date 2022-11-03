use colored::Colorize;

use crate::{utils::load_file, flow::wax, conf::get_conf, error};

/** Build a wax project */
pub fn build(path: String) {
  let work_dir = format!("{}/..", &path);

  println!("\n{} loading config", "Wax".green().bold());


  let index_dir;

  // Attempt to read the wax config file:
  match get_conf(&path) {
    Ok(index) => index_dir = index,
    Err(e) => {
      error!(e);
      return;
    },
  }

  // Delete the .wax directory if it exists.
  if std::path::Path::new(format!("{}/.wax", work_dir).as_str()).exists() {
    std::fs::remove_dir_all(format!("{}/.wax", work_dir)).expect("Failed to remove .wax directory");
  }


  println!("{} building '{}'\n", "Wax".green().bold(), path);
  let start = std::time::Instant::now();


  let mut output = String::new();

  let mut dir = Directories { 
    code_dir: path.clone(), 
    work_dir: work_dir.clone(), 
    relative_path: index_dir.clone(), 
    parent_file: "index.html".into() 
  };

  // Attempt to read the index file:
  if let Ok(contents) = load_file(&path, format!("{}/index.html", index_dir).as_str()) {
    match wax(&mut dir, contents) {
      Ok(result) => output = result,
      Err(e) => {
        println!("\n{} failed ({})", "Wax".red().bold(), e);
        return;
      }
    }
  }

  // Write the output to the disk:
  std::fs::create_dir_all(format!("{}/build", work_dir)).expect("Failed to create ./build directory");
  std::fs::write(format!("{}/build/index.html", work_dir), &output).expect("Failed to write output");

  println!("\n{} finished in {}ms", "Wax".green().bold(), start.elapsed().as_millis());
}

#[derive(Clone)]
pub struct Directories {
  pub work_dir: String,
  pub code_dir: String,

  pub relative_path: String,
  pub parent_file: String,
}