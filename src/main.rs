use colored::Colorize;

mod flow;
mod utils;
mod conf;

#[macro_use]
mod term;

fn main() {

  // Get the code path from the args.
  let args: Vec<String> = std::env::args().collect();
  let default_path = String::from(".");
  let code_path = args.get(1).unwrap_or(&default_path);
  let work_dir = format!("{}/..", &code_path);

  println!("\n{} loading config", "Wax".green().bold());


  let index_dir;

  // Attempt to read the wax config file:
  if let Some(index) = conf::get_conf(&code_path) {
    index_dir = index;
  } else {
    return;
  }

  // Delete the .wax directory if it exists.
  if std::path::Path::new(format!("{}/.wax", work_dir).as_str()).exists() {
    std::fs::remove_dir_all(format!("{}/.wax", work_dir)).expect("Failed to remove .wax directory");
  }


  println!("{} building '{}'\n", "Wax".green().bold(), code_path);
  let start = std::time::Instant::now();


  let mut output = String::new();

  let mut dir = Directories { 
    code_dir: code_path.clone(), 
    work_dir: work_dir.clone(), 
    relative_path: index_dir.clone(), 
    parent_file: "index.html".into() 
  };

  // Attempt to read the index file:
  if let Ok(contents) = utils::load_file(code_path, format!("{}/index.html", index_dir).as_str()) {
    match flow::wax(&mut dir, contents) {
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
  work_dir: String,
  code_dir: String,

  relative_path: String,
  parent_file: String,
}