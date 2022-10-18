mod include;
mod utils;
mod conf;

fn main() {

  // Get the code path from the args.
  let args: Vec<String> = std::env::args().collect();
  let default_path = String::from(".");
  let code_path = args.get(1).unwrap_or(&default_path);


  println!("\nWax - Loading Conf...\n");


  let index_dir;

  // Attempt to read the wax config file:
  if let Some(index) = conf::get_conf(&code_path) {
    index_dir = index;
  } else {
    return;
  }

  std::fs::remove_dir_all("./.wax").expect("Failed to remove .wax directory");


  println!("Wax - Building...\n");
  let start = std::time::Instant::now();


  let mut output = String::new();

  // Attempt to read the index file:
  if let Ok(contents) = utils::load_file(code_path, format!("{}/index.html", index_dir).as_str()) {
    output = include::include(code_path, &index_dir, "index.html", contents);
  }

  // Write the output to the disk:
  std::fs::create_dir_all("./build").expect("Failed to create ./build directory");
  std::fs::write("./build/index.html", &output).expect("Failed to write output");


  println!("\nWax - Finished in {}ms", start.elapsed().as_millis());
}