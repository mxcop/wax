use toml::Value;

use crate::utils;

/** Read the wax toml config file */
pub fn get_conf(code_path: &str) -> Result<String, String> {
  // Attempt to read the wax config file:
  if let Ok(conf) = utils::load_file(code_path, "./wax.toml") {
    // Parse the config file.
    let val = conf.parse::<Value>().unwrap();
    let index = val.get("index");

    // Check if the index is none.
    if index.is_none() { return Err(r#"missing 'index = "..."' in wax.toml"#.into()); }

    // Check if the index is a string.
    if let Some(index) = index.unwrap().as_str() {
      return Ok(String::from(index));
    } else {
      return Err("wrong type used for 'index' in wax.toml".into());
    }
  } else {
    return Err("missing wax.config file".into());
  }
}