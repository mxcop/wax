mod include;
mod utils;
mod conf;

use regex::Regex;

fn main() {

  // Get the code path from the args.
  let args: Vec<String> = std::env::args().collect();
  let default_path = String::from(".");
  let code_path = args.get(1).unwrap_or(&default_path);

  println!("Wax - Loading Conf...\n");

  let index_dir;

  // Attempt to read the wax config file:
  if let Some(index) = conf::get_conf(&code_path) {
    index_dir = index;
  } else {
    return;
  }

  println!("Wax - Building...\n");

  let mut output = String::new();

  // Attempt to read the index file:
  if let Ok(contents) = utils::load_file(code_path, format!("{}/index.html", index_dir).as_str()) {
    output = contents;

    // Use regex to find all the <wax> elements:
    let exp = Regex::new(r"<wax.*?>").expect("Regex failed");

    while let Some(caps) = exp.captures(&output) {
      output = include::include(caps, code_path, &index_dir, output.clone());
    }
  }

  std::fs::create_dir_all("./build").expect("Failed to create ./build directory");
  std::fs::write("./build/index.html", &output).expect("Failed to write output");

  println!("out: \n{}", output);
}