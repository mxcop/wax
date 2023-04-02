// use serde::Deserialize;

// use super::utils::load_file;

// /** Read the wax toml config file */
// pub fn get_conf(code_path: &str) -> Result<WaxConfig, String> {
//   // Attempt to read the wax config file:
//   if let Ok(conf) = load_file(code_path, "./wax.toml") {
//     // Parse the config file.
//     match toml::from_str(&conf) {
//         Ok(conf) => return Ok(conf),
//         Err(e) => return Err(e.to_string()),
//     }

//     // Check if the pages is none.
//     //if val.pages.is_none() { return Err(r#"missing 'pages = "..."' in wax.toml"#.into()); }
//   } else {
//     return Err("missing wax.toml file".into());
//   }
// }

// /** Wax config struct */
// #[derive(Deserialize)]
// pub struct WaxConfig {
//   pub website: Website,
//   pub build: Option<Build>
// }

// /** Group for generic project related info */
// #[derive(Deserialize)]
// pub struct Website {
//   pub pages: String,
// }

// /** Group for build related options */
// #[derive(Deserialize)]
// pub struct Build {
//   pub minify: Option<bool>
// }