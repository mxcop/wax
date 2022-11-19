use crate::fetcher::import::fetch_imports;

#[test]
fn formatting() -> Result<(), String> {
  let result = fetch_imports(FORMATTING_TEST);
  
  if let Some(result) = result {

    if result.len() < 10 {
      return Err(format!("not all imports were found ({} out of 10)", result.len()));
    }

    for import in result {
      if import.path != "../lib/comp.wx" {
        return Err(format!(r#"import path wasn't fetched correctly ("{}" should've been "../lib/comp.wx")"#, import.path));
      }
      if import.name != "comp" && import.name != "{ comp }" {
        return Err(format!(r#"import name wasn't fetched correctly ("{}" should've been "comp")"#, import.name));
      }
    }

    Ok(())
  } else {
    Err(String::from("no imports were found"))
  }
}

/** Test string for import statement formatting. */
const FORMATTING_TEST: &str = 
r#"
import comp from "../lib/comp.wx";

import comp from 
"../lib/comp.wx";

import comp 
from "../lib/comp.wx";

import 
comp from "../lib/comp.wx";

import 
comp from 
"../lib/comp.wx";

import 
comp 
from 
"../lib/comp.wx";

import comp from " ../lib/comp.wx";

import comp from "../lib/comp.wx ";

import { comp } from "../lib/comp.wx";

import "../lib/comp.wx";
"#;

/** Test string for imports using curly braces. */
const CURLY_IMPORT_TEST: &str = 
r#"
import { comp } from "../lib/comp.wx";
"#;

/** Test string for imports that don't have a given name. */
const NONAME_IMPORT_TEST: &str = 
r#"
import "../lib/comp.wx";
"#;