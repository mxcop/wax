pub mod node;

use self::node::{Node, GrammarNode};

use super::lexer::token::Token;

pub struct Parser {
  input: Vec<Token>,
  pub position: usize,
  pub read_position: usize,
  pub tk: Token,
  
  pub html: Node,
  pub css: Node,
  pub js: Node,
}

impl Parser {
  pub fn new(input: Vec<Token>) -> Self {
    Self {
      input: input,
      position: 0,
      read_position: 0,
      tk: Token::ILLEGAL,
      
      html: Node::new(GrammarNode::HTML),
      css: Node::new(GrammarNode::CSS),
      js: Node::new(GrammarNode::JS)
    }
  }

  pub fn read_token(&mut self) {
    if self.read_position >= self.input.len() {
      self.tk = Token::ILLEGAL;
    } else {
      self.tk = self.input[self.read_position].clone();
    }
    self.position = self.read_position;
    self.read_position = self.read_position + 1;
  }

  pub fn parse(&mut self) -> Node {
    let node: Node;
    match self.tk {
        _ => todo!()
    }
    self.read_token();
    node
  }
}
