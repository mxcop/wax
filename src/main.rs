// use clap::Parser;
// use args::Commands;

use compiler::lexer::{token::Token, Lexer};

mod args;
mod build;
mod create;
mod logging;
mod utils;

mod compiler;

fn main() {
  // Enable colors in the command prompt.
  colored::control::set_virtual_terminal(true).unwrap();

  let input = std::fs::read_to_string("./example/src/pages/hive.wx").expect("failed to load file");
  let mut l = Lexer::new(input.chars().collect());
  l.read_char();
  loop {
    let token = l.next_token();
    if token == Token::EOF {
      break;
    } else {
      println!("{:?}", token);
    }
  }
  println!("{} {} {}", char::from(l.ch), l.position, l.read_position);

  // let args = args::Args::parse();

  // match args.cmd {
  //   Commands::Create { name } => create::create(name),
  //   Commands::Build { path } => build::build(path.clone()),
  // }
}
