use super::super::import::fetch_imports;

/// Test different import formats.
#[test]
fn formatting() -> Result<(), String> {
  let result = fetch_imports(FORMATTING_TEST);
  
  if let Ok(result) = result {

    if result.len() < 9 {
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
    Err(String::from("an error occured during fetching"))
  }
}

/// Test if curly brace imports will panic.
#[test]
#[should_panic]
fn curly_braces() {
  fetch_imports(CURLY_IMPORT_TEST).unwrap();
}

/// Test if imports without a name will be interpreted correctly.
#[test]
fn no_name() -> Result<(), String> {
  let result = fetch_imports(NONAME_IMPORT_TEST);

  if let Ok(result) = result {
    if result.len() == 0 {
      return Err(String::from("no imports were found"));
    }
    if result[0].name == "comp" {
      return Ok(());
    }
    return Err(format!(r#"no name import name was interpreted incorrectly ("{}" should've been "comp")"#, result[0].name));
  } else {
    return Err(String::from("an error occured during fetching"));
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