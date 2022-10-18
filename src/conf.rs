use toml::Value;

use crate::utils;

pub fn get_conf(code_path: &str) -> Option<String> {
  // Attempt to read the wax config file:
  if let Ok(conf) = utils::load_file(code_path, "./wax.toml") {
    // Parse the config file.
    let val = conf.parse::<Value>().unwrap();
    let index = val["index"].as_str();

    if let Some(index) = index {
      return Some(String::from(index));
    } else {
      println!("error: missing 'index' from wax.toml");
      return None;
    }
  } else {
    println!("error: missing wax.config file");
    return None;
  }
}