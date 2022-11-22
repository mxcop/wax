// use clap::Parser;
// use args::Commands;

use compiler::lexer::{token::Token, Lexer};
use compiler::parser::Parser;

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

  // Tokenize :
  let mut l = Lexer::new(input.chars().collect());
  l.read_char();
  let mut tk: Vec<Token> = Vec::new();

  loop {
    let token = l.next_token();
    if token == Token::EOF {
      tk.push(token);
      break;
    }
    tk.push(token);
  }
  println!("{} {} {}", char::from(l.ch), l.position, l.read_position);

  // Parse :
  //let mut p = Parser::new(tk);
  //p.read_token();

  enum NodeType<'a> {
    Root,
    Linear(Vec<Node<'a>>),
    Branch{ eq: &'a Node<'a>, neq: &'a Node<'a>},
    Script(&'a Node<'a>),
    DefaultImport(&'a Node<'a>),
  }

  struct Node<'a> {
    parent: Option<&'a Node<'a>>,
    entry: NodeType<'a>
  }

  let mut root = Node { parent: None, entry: NodeType::Root };
  let mut curr = &mut root;

  for (index, token) in tk.iter().enumerate() {
    match token {
      Token::TAG(tag) => parse_tag(index, &tag, &tk, &mut curr),
      _ => {}
    }
    println!("{} : {:?}", index, token);
  }

  fn parse_tag(start: usize, tag: &String, tokens: &Vec<Token>, curr: &mut Node) {
    println!("Found a tag <{}>", tag);

    let mut j = start;
    loop {
      match &tokens[j] {
        Token::GT(_) => {
          return;
        },
        Token::IDENT(attr) => {
          println!("Found an attribute {}", attr);
        },
        Token::EOF => {
          return;
        }
        _ => {}
      }
      j += 1;
    }
  }
  
  // let mut i = tk.iter().peekable();
  // loop {
  //   if let Some(token) = i.next() {
  //     match &token {
  //       Token::IDENT(_) => {},
  //       Token::SLASH(_) => {},
  //       Token::LT(_) => {
  //         if let Some(peek) = i.peek() {
  //           if let Token::IDENT(ident) = peek {
  //             println!("<");
  //             println!("{}", ident);
  //           } else if let Token::SLASH(_) = peek {
  //             println!("/>");
  //           }
  //         }
  //       },
  //       Token::GT(_) => {},
  //       _ => {},
  //     }
  //   } else {
  //     break;
  //   }
  // }
  //println!("\n{:?}", p.);

  // let args = args::Args::parse();

  // match args.cmd {
  //   Commands::Create { name } => create::create(name),
  //   Commands::Build { path } => build::build(path.clone()),
  // }
}
