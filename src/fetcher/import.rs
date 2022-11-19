use std::path::Path;

use regex::Regex;

use crate::error::SyntaxError;

/// References an import statement which imports a wax component.
pub struct ComponentImport {
  pub name: String,
  pub path: String,
}

/// Fetch all the import statements within the contents of a script tag.
pub fn fetch_imports(script: &str) -> Result<Vec<ComponentImport>, SyntaxError> {

  // Use regex to find all the import statements:
  let re = Regex::new(IMPORT_REGEX).expect("Regex failed to initialize");

  // Check if anything was found.
  let has_captures = re.captures(script).is_some();

  if has_captures {
    // Get the itterator and create the vector:
    let iter = re.captures_iter(script);
    let mut imports: Vec<ComponentImport> = Vec::with_capacity(iter.count());

    for cap in re.captures_iter(script) {
      let name = cap.get(1);
      let path = cap.get(2);

      if let None = name { println!("warning(fetch_imports) : captures match number 1 was None"); continue; }
      if let None = path { println!("warning(fetch_imports) : captures match number 2 was None"); continue; }

      let mut name_str = name.unwrap().as_str();
      let path = path.unwrap().as_str();

      // If the name isn't one word, or uses curly braces.
      if name_str.trim().contains(' ') {
        return Err(SyntaxError::with_help(
          "component import cannot use curly braces syntax", 
          "folder/file.wx", 
          0, 
          0,
          "<line>",
          "use default import syntax instead, `import <name> from <path>`"
        ));
      }

      // If the name is empty then we set it using the file name.
      if name_str.trim().is_empty() {
        // TODO : use prefix instead of stem here! (otherwise component name might include a '.')
        let file_name = Path::new(path).file_stem().expect("import missing component name and file name in path");
        name_str = file_name.to_str().unwrap();
      }

      let import = ComponentImport { 
        name: name_str.trim().to_string(), 
        // The trim -> trim -> trim is because the result includes \s"\s
        path: path.trim().trim_matches('"').trim().to_string()
      };

      // Add the import into the vector.
      imports.push(import); 
    }

    return Ok(imports);
  }

  Ok(Vec::new())
}

const IMPORT_REGEX: &str = r#"import([\s\S]*?)(?:from|")([\s\S]*?\.wx)[\s\S]*?""#;