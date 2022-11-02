use regex::Regex;

pub fn wax_params(contents: String, import: &str) -> Result<String, String> {

  // Use regex to find the first <params!> element:
  let exp = Regex::new(r"<params!(.*?)>").expect("Regex failed");

  // Check if this component has child components.
  let cap = exp.captures(&contents);

  if let Some(cap) = cap {

    // Get the parameters from the params element.
    let element = cap.get(1).unwrap().as_str().trim_start();

    // Split the parameters into a vector.
    let params = element.split(' ').collect::<Vec<&str>>();

    for param in params {
      println!("Found Param : {}", param);

      // todo : Find this param inside the 'import' string and replace it within the contents.
    }
  }
  
  Ok(contents)
}