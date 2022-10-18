/* 
  todo : Simple Proof of concept :
  x 1. Read index.html file
  x 2. Find any <wax> elements
  x 3. Read the <wax path="..."> attribute
  4. Read the html file at that path
  5. Insert the contents of the file into index.html
  6. Write the index.html file into a ./build folder 
*/

use std::{path::Path, io};

use regex::Regex;

fn main() {

  println!("Wax - Building...\n");

  let mut output = String::new();

  // Attempt to read the index file:
  if let Ok(contents) = load_file("./assets/index.html") {
    output = contents.clone();

    // Use regex to find all the <wax> elements:
    let exp = Regex::new(r"<wax.*?>").expect("Regex failed");

    for caps in exp.captures_iter(&contents.as_str()) {
      // Get the matched <wax> element without whitespace.
      let mat = caps.get(0).unwrap();
      let el = mat.as_str().replace(" ", "");

      // Use regex to extract the path attribute:
      let exp = Regex::new(r#"path="(?s)(.*)""#).expect("Regex failed");
      let caps = exp.captures_iter(el.as_str()).nth(0);

      if let Some(caps) = caps {

        // Get the path from the regex match.
        let path = caps.get(1).unwrap().as_str();
        println!("found <wax> with path '{}'", path);

        // Load the contents of the html file given in the path:
        if let Ok(subcontents) = load_file(format!("./assets/{}", path).as_str()) {
          output.replace_range(mat.range(), &subcontents);
        } else {
          println!("warn: failed to load '{}'", path);
        }

      } else {
        println!("warn: missing 'path' attribute on wax element");
      }

      //println!("wax: {}", caps.get(0).unwrap().as_str().replace(" ", ""));
    }
  }

  std::fs::create_dir_all("./build").expect("Failed to create ./build directory");
  std::fs::write("./build/index.html", &output).expect("Failed to write output");

  println!("out: \n{}", output);
}

/** Load a file using relative or absolute path */
fn load_file(path: &str) -> io::Result<String> {
  let path = Path::new(path);
  std::fs::read_to_string(path)
}