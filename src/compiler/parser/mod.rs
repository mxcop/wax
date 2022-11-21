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
        Token::ILLEGAL => todo!(),
        Token::EOF => todo!(),
        Token::IDENT(_) => todo!(),
        Token::INT(_) => todo!(),
        Token::EQUALS(_) => todo!(),
        Token::PLUS(_) => todo!(),
        Token::COMMA(_) => todo!(),
        Token::SEMI(_) => todo!(),
        Token::COLON(_) => todo!(),
        Token::LPAREN(_) => todo!(),
        Token::RPAREN(_) => todo!(),
        Token::QUEST(_) => todo!(),
        Token::LBRACE(_) => todo!(),
        Token::RBRACE(_) => todo!(),
        Token::IMPORT => todo!(),
        Token::EXPORT => todo!(),
        Token::FROM => todo!(),
        Token::FUNCTION => todo!(),
        Token::LET => todo!(),
        Token::CONST => todo!(),
        Token::TRUE => todo!(),
        Token::FALSE => todo!(),
        Token::IF => todo!(),
        Token::ELSE => todo!(),
        Token::RETURN => todo!(),
        Token::MINUS(_) => todo!(),
        Token::BANG(_) => todo!(),
        Token::AST(_) => todo!(),
        Token::SLASH(_) => todo!(),
        Token::LT(_) => todo!(),
        Token::GT(_) => todo!(),
        Token::QUOT(_) => todo!(),
        Token::APOS(_) => todo!(),
        Token::DOT(_) => todo!(),
        Token::COMMAT(_) => todo!(),
    }
    self.read_token();
    node
  }
}
