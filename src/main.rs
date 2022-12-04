use peekmore::PeekMore;
use wax_lexer::{Lexer, token::Token};
use wax_parser::Parser;
use wax_parser::{tree::ArenaTree, node::SyntaxNode};

mod args;
mod build;
mod create;
mod utils;

fn main() {
  // Enable colors in the command prompt.
  colored::control::set_virtual_terminal(true).unwrap();

  let input = std::fs::read_to_string("./example/src/pages/hive.wx").expect("failed to load file");
  let chars: Vec<char> = input.chars().collect();

  // Tokenize :
  let mut lexer = Lexer::new(chars.iter().peekmore());
  let tokens: Vec<Token> = lexer.lex();

  println!("\nTokens : \n{:?}", tokens);

  // Parse :
  let mut parser = Parser::new(input, "src/pages/hive.wx".into(), tokens.iter().peekmore());
  let tree: ArenaTree<SyntaxNode> = parser.parse();

  println!("\nAST : \n{}", tree);
}
