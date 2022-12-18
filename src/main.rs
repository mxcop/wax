use waxc_lexer::lexer::LexIter;

mod args;
mod build;
mod create;
mod utils;

fn main() {
  // Enable colors in the command prompt.
  colored::control::set_virtual_terminal(true).unwrap();

  let input = std::fs::read_to_string("./example/src/pages/hive.wx").expect("failed to load file");

  run(&input, "src/pages/hive.wx");
}

/// Run a single parsing.
fn run(input: &str, filename: &str) {
  // Initialize the lexical iterator:
  let lexer = waxc_lexer::lex(input);
  let iter = LexIter::new(lexer);

  // Start the parsing process:
  let parser = waxc_parser::parse(input.to_string(), iter);

  match parser {
    Err(e) => {
      e.print(input, filename);
    }
    Ok(ast) => { 
      println!("{}", &ast);
      let comb = waxc_codegen::generate(ast).unwrap();
      println!("{}", comb);
    }
  }
}
