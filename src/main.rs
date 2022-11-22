// use clap::Parser;
// use args::Commands;

use compiler::lexer::{token::Token, Lexer};
//use compiler::parser::Parser;

use compiler::parser::node::{NodeType, Attribute};
use compiler::parser::tree::ArenaTree;

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
  let mut tree: ArenaTree<NodeType> = ArenaTree::new();
  let curr = tree.add_node("Root".into(), NodeType::Root);

  for (index, token) in tk.iter().enumerate() {
    match token {
      Token::TAG(tag) => parse_tag(index, &tag, &tk, &mut tree, curr),
      _ => {}
    }
    println!("{} : {:?}", index, token);
  }

  fn parse_tag(
    start: usize,
    tag: &String,
    tokens: &Vec<Token>,
    tree: &mut ArenaTree<NodeType>,
    curr: usize,
  ) {
    println!("Found a tag <{}>", tag);

    match tag.as_str() {
      "script" => {
        let mut attrs: Vec<Attribute> = Vec::new();

        let mut j = start;
        loop {
          match &tokens[j] {
            Token::GT(_) => {
              break;
            }
            Token::IDENT(attr) => {
              println!("Found an attribute {}", attr);
              attrs.push(Attribute { name: attr.into(), value: "".into() });
            }
            Token::EOF => {
              break;
            }
            _ => {}
          }
          j += 1;
        }

        let node = NodeType::Script {
          attributes: attrs,
        };

        tree.add_child(curr, "Script".into(), node);
      }
      "style" => {
        let mut attrs: Vec<Attribute> = Vec::new();

        let mut j = start;
        loop {
          match &tokens[j] {
            Token::GT(_) => {
              break;
            }
            Token::IDENT(attr) => {
              println!("Found an attribute {}", attr);
              attrs.push(Attribute { name: attr.into(), value: "".into() });
            }
            Token::EOF => {
              break;
            }
            _ => {}
          }
          j += 1;
        }

        let node = NodeType::Style {
          attributes: attrs,
        };

        tree.add_child(curr, "Style".into(), node);
      }
      _ => {
        let mut attrs: Vec<Attribute> = Vec::new();

        let mut j = start;
        loop {
          match &tokens[j] {
            Token::GT(_) => {
              break;
            }
            Token::IDENT(attr) => {
              println!("Found an attribute {}", attr);
              attrs.push(Attribute { name: attr.into(), value: "".into() });
            }
            Token::EOF => {
              break;
            }
            _ => {}
          }
          j += 1;
        }

        let node = NodeType::Tag {
          attributes: attrs,
        };

        tree.add_child(curr, tag.into(), node);
      }
    }
  }

  println!("\nAST : \n{}", tree);
}
