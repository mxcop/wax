/* 
  todo : Simple Proof of concept :
  x 1. Read index.html file
  x 2. Find any <wax> elements
  x 3. Read the <wax path="..."> attribute
  x 4. Read the html file at that path
  x 5. Insert the contents of the file into index.html
  x 6. Write the index.html file into a ./build folder 
*/

use std::{path::Path, io};

use regex::{Regex, Captures};
use toml::Value;

fn main() {

  // Get the code path from the args.
  let args: Vec<String> = std::env::args().collect();
  let default_path = String::from(".");
  let code_path = args.get(1).unwrap_or(&default_path);

  println!("Wax - Loading Conf...\n");

  let index_dir;

  // Attempt to read the wax config file:
  if let Ok(conf) = load_file(code_path, "./wax.toml") {
    // Parse the config file.
    let val = conf.parse::<Value>().unwrap();
    let index = val["index"].as_str();

    if let Some(index) = index {
      index_dir = String::from(index);
    } else {
      println!("error: missing 'index' from wax.toml");
      return;
    }
  } else {
    println!("error: missing wax.config file");
    return;
  }

  println!("Wax - Building...\n");

  let mut output = String::new();

  // Attempt to read the index file:
  if let Ok(contents) = load_file(code_path, format!("{}/index.html", index_dir).as_str()) {
    output = contents;

    // Use regex to find all the <wax> elements:
    let exp = Regex::new(r"<wax.*?>").expect("Regex failed");

    while let Some(caps) = exp.captures(&output) {
      output = include_wax(caps, code_path, &index_dir, output.clone());
    }
  }

  std::fs::create_dir_all("./build").expect("Failed to create ./build directory");
  std::fs::write("./build/index.html", &output).expect("Failed to write output");

  println!("out: \n{}", output);
}

fn include_wax(caps: Captures, code_path: &str, relative_path: &String, mut output: String) -> String {

  // Get the range and element from the captures:
  let range = caps.get(0).unwrap().range();
  let element = caps.get(0).unwrap().as_str().replace(" ", "");

  // Use regex to extract the path attribute:
  let exp = Regex::new(r#"path="(?s)(.*)""#).expect("Regex failed");
  
  if let Some(path) = exp.captures(element.as_str()) {

    // Get the path from the regex match.
    let path = path.get(1).unwrap().as_str();
    println!("found <wax> with path '{}'", path);

    // Load the contents of the html file given in the path:
    if let Ok(subcontents) = load_file(code_path, format!("{}/{}", relative_path, path).as_str()) {
      output.replace_range(range, &subcontents);
    } else {
      println!("warn: failed to load '{}'", path);
    }

  } else {
    println!("warn: missing 'path' attribute on wax element");
  }

  output
}

/** Load a file using relative or absolute path */
fn load_file(code_path: &str, path: &str) -> io::Result<String> {
  let path = format!("{}/{}", code_path, path);
  let path = Path::new(&path);
  std::fs::read_to_string(path)
}