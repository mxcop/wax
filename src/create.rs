use std::{fs::{create_dir_all, write}, path::Path};

use colored::Colorize;

use crate::error;

/** Create a new wax project */
pub fn create(name: String) {

  println!("\n{} creating project '{}'", "Wax".green().bold(), &name);

  let path = format!("./{}/", &name);

  // Check if this path already exists.
  if Path::new(&path).exists() {
    error!("path '{}' already exists", path);
    return;
  }

  let get_path = |add: &str| -> String {
    format!("{}{}", &path, add)
  };

  // Create the directories:
  create_dir_all(get_path("routes")).expect("Failed to create project routes directory");
  create_dir_all(get_path("lib")).expect("Failed to create project lib directory");

  // Create the config file.
  write(get_path("wax.toml"), CONF_TEMPLATE).expect("Failed to create config file");

  // Create the index.html file.
  write(get_path("routes/index.html"), HTML_TEMPLATE).expect("Failed to create index.html file");

  // Create the example component file.
  write(get_path("lib/comp.html"), COMP_TEMPLATE).expect("Failed to create example component file");

  println!("\n{} finished creating project at './{}'", "Wax".green().bold(), &name);
}

// Wax config template.
const CONF_TEMPLATE: &str = 
r#"index = "./routes""#;

// Index html file template.
const HTML_TEMPLATE: &str = 
r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta http-equiv="X-UA-Compatible" content="IE=edge">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>My Wax Site</title>
</head>
<body>

  <wax! src="../lib/comp.html" />

</body>
</html>"#;

// Example component file template.
const COMP_TEMPLATE: &str = 
r#"<h1>
  Hello world !
</h1>"#;