//use std::{thread, borrow::Borrow};

use colored::Colorize;
use pest::{Parser, iterators::Pairs};
use wax_pest::{WaxParser, Rule};

mod lib;

fn main() {
  // Enable colors in the command prompt.
  colored::control::set_virtual_terminal(true).unwrap();
  println!("");

  //let mut handles = Vec::new();
  let file = std::fs::read_to_string("./examples/example.wx").unwrap();

  let start = std::time::Instant::now();

  let result = WaxParser::parse(Rule::comb, &file);

  match result {
    Ok(pairs) => {
      recurse(pairs, 0);
    }
    Err(e) => {
      println!("Err: {:?}", e);
    }
  }

  let time = start.elapsed().as_millis();
  println!("\nTotal time elapsed {}ms", time);

  // for _ in 0..100 {
  //   let file = file.clone();
  //   let handle = thread::spawn(move || { 
  //     let speed = parse(file.borrow()); 
  //     //recurse(pairs.unwrap(), 0);
  //     speed
  //   });

  //   handles.push(handle);
  // }

  //for handle in handles {
  //  let result = handle.join().unwrap();
  //  println!("Pest took {} to parse the file", result);
  //}

  //let time = start.elapsed().as_millis();
  //println!("Total time elapsed {}ms", time);
}

// fn parse(file: &str) -> String {
//   let start = std::time::Instant::now();

//   if let Ok(pairs) = WaxParser::parse(Rule::html, file) {
//     let time = start.elapsed().as_millis();
//     //recurse(pairs, 0);
//     format!("{}ms", time)
//   } else {
//     "-".into()
//   }
// }

/// Print the whole Abstract Syntax Tree
fn recurse(pairs: Pairs<Rule>, level: usize) {
  for pair in pairs {

    for _ in 0..level {
      print!("  ");
    }

    let rule = format!("{:?}", pair.as_rule());
    let str = pair.as_str();

    if count_newlines(str) > 0 {
      println!("- {:?}", rule);
      println!("{}", format!("({})", pair.as_str()).bright_black());
    } else {
      print!("- {:?}", rule);
      println!("  {}", format!("({})", pair.as_str()).bright_black());
    }
    
    recurse(pair.into_inner(), level+1);
  }
}

fn count_newlines(s: &str) -> usize {
  s.as_bytes().iter().filter(|&&c| c == b'\n').count()
}