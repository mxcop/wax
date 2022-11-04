use toml::Value;

use super::utils::load_file;

/** Read the wax toml config file */
pub fn get_conf(code_path: &str) -> Result<String, String> {
  // Attempt to read the wax config file:
  if let Ok(conf) = load_file(code_path, "./wax.toml") {
    // Parse the config file.
    let val = conf.parse::<Value>().unwrap();
    let index = val.get("pages");

    // Check if the pages is none.
    if index.is_none() { return Err(r#"missing 'pages = "..."' in wax.toml"#.into()); }

    // Check if the pages is a string.
    if let Some(index) = index.unwrap().as_str() {
      return Ok(String::from(index));
    } else {
      return Err("wrong type used for 'pages' in wax.toml".into());
    }
  } else {
    return Err("missing wax.toml file".into());
  }
}