use std::sync::Arc;

use peekmore::PeekMore;
use wax_lexer::{token::Token, Lexer};
use wax_parser::Parser;
use wax_parser::{node::SyntaxNode, tree::ArenaTree};

mod args;
mod build;
mod create;
mod utils;

fn main() {
  // Enable colors in the command prompt.
  colored::control::set_virtual_terminal(true).unwrap();

  let input = std::fs::read_to_string("./example/src/pages/hive.wx").expect("failed to load file");
  //let chars: Vec<char> = input.chars().collect();

  //run_threads(input);
  run(input);
}

#[allow(unused)]
/// Run multiple parsings on separate threads.
fn run_threads(input: String) {
  let input = Arc::new(input);
  let mut handles = Vec::new();

  let start_super = std::time::Instant::now();
  
  for _ in 0..32 {
    let input = input.clone();

    let handle = std::thread::spawn(move || {
      let start = std::time::Instant::now();
      let chars = input.chars().collect();
      let tree = run_thread_safe(input, chars);
      (tree, start.elapsed().as_micros())
    });

    handles.push(handle);
  }

  let mut c = 0;
  for handle in handles {
    let (_, time) = handle.join().unwrap();

    println!("thread {} took {}ms ({}µs)", c, time as f32 / 1_000.0, time);

    c += 1;
  }

  let time = start_super.elapsed().as_micros();
  println!("all computations took {}ms ({}us)", time as f32 / 1_000.0, time);
}

#[allow(unused)]
/// Run a thread safe version of the parsing.
fn run_thread_safe(input: Arc<String>, chars: Vec<char>) -> ArenaTree<SyntaxNode> {
  // Tokenize :
  let mut lexer = Lexer::new(chars.iter().peekable());
  let tokens: Vec<Token> = lexer.lex();

  // Parse :
  let mut parser = Parser::new(input, "src/pages/hive.wx".into(), tokens.iter().peekmore());
  let tree: ArenaTree<SyntaxNode> = parser.parse();

  tree
}

#[allow(unused)]
/// Run a single parsing.
fn run(input: String) {
  let chars: Vec<char> = input.chars().collect();
  let start = std::time::Instant::now();

  // Tokenize :
  let mut lexer = Lexer::new(chars.iter().peekable());
  let tokens: Vec<Token> = lexer.lex();

  let lex_time = start.elapsed().as_micros();

  println!("\nTokens : \n{:?}", tokens);

  let start = std::time::Instant::now();

  // Parse :
  let mut parser = Parser::new(Arc::new(input), "src/pages/hive.wx".into(), tokens.iter().peekmore());
  let tree: ArenaTree<SyntaxNode> = parser.parse();

  let parse_time = start.elapsed().as_micros();

  println!("\nAST : \n{}", tree);

  println!(
    "\nLexing took {}ms ({}µs)\nParsing took {}ms ({}µs)",
    lex_time as f64 / 1_000.0,
    lex_time,
    parse_time as f64 / 1_000.0,
    parse_time
  );
}
