use colored::Colorize;
use pest::{Parser, iterators::Pairs};
use wax_pest::{WaxParser, Rule};

mod lib;

fn main() {
  // Enable colors in the command prompt.
  colored::control::set_virtual_terminal(true).unwrap();
  println!("");

  if let Ok(file) = std::fs::read_to_string("./examples/example.wx") {
    let start = std::time::Instant::now();

    if let Ok(pairs) = WaxParser::parse(Rule::html, &file) {
      recurse(pairs, 0);
    }

    println!("\nPest took {}ms to parse the file", start.elapsed().as_millis());
  }
}

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