use waxc_lexer::lexer::LexIter;

mod args;
mod build;
mod create;
mod utils;

fn main() {
  // Enable colors in the command prompt.
  colored::control::set_virtual_terminal(true).unwrap();

  let index_file = std::fs::read_to_string("./example/src/index.html").expect("failed to load index.html");
  let input = std::fs::read_to_string("./example/src/pages/hive.wx").expect("failed to load file");

  run(&index_file, &input, "src/pages/hive.wx");
}

/// Run a single parsing.
fn run(index_file: &str, input: &str, filename: &str) {
  // Initialize the lexical iterator:
  let lexer = waxc_lexer::lex(input);
  let iter = LexIter::new(lexer);

  // Start the parsing process:
  let parser = waxc_parser::parse(input.to_string(), iter);

  // Check for errors:
  let ast = match parser {
    Err(e) => {
      e.print(input, filename);
      return;
    }
    Ok(ast) => ast
  };
  println!("{}", &ast);

  // Generate the code:
  let comb = waxc_codegen::generate(index_file.to_string(), ast);

  // Check for errors:
  let comb = match comb {
    Err(e) => {
      e.print(input, filename);
      return;
    }
    Ok(comb) => comb
  };
  println!("{}", &comb);

  // Save the files:
  if std::path::Path::new("./build/").is_dir() == false {
    std::fs::create_dir("./build/").expect("failed to create build dir");
  }
  std::fs::write("./build/index.html", comb.html).expect("failed to save html");
  std::fs::write("./build/index.css", comb.css).expect("failed to save css");
}
