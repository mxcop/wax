use waxc_lexer::token::{Token, TokenKind};

use crate::{tree::AST, node::{Node, NodeKind}, span::Span};

/// The Wax parser
pub(crate) struct Parser<'a, I: Iterator<Item = Token> + Clone + 'a> {
  consumed: usize,
  cursor: usize,

  tree: AST,
  tokens: &'a mut I,
  scope: usize,
  pub file: String,
}

impl<'a, I> Parser<'a, I> 
  where I: Iterator<Item = Token> + Clone + 'a 
{
  pub fn new(file: String, tokens: &'a mut I) -> Self {

    let mut tree = AST::new();
    let scope = tree.add_node(Span::new(0, 0), NodeKind::Root);

    Self {
      file,
      scope,
      consumed: 0,
      cursor: 0,
      tree,
      tokens
    }
  }

  /// Read from the input file the current token range.
  pub fn read(&self) -> &str {
    &self.file[self.cursor..self.consumed]
  }

  /// Read from the input file from a starting location until the consumed position.
  pub fn read_from(&self, start: usize) -> &str {
    &self.file[start..self.consumed]
  }

  /// Read the string representation of the next token.
  pub fn read_next(&mut self) -> String {
    let prepos = self.consumed;
    self.next();
    self.file[prepos..self.consumed].to_string()
  }

  pub fn reset_cursor(&mut self) {
    self.cursor = self.consumed;
  }

  /// Get a clone of the abstract syntax tree.
  pub fn get_tree(&self) -> AST {
    self.tree.clone()
  }

  /// Add a new node to the current scope.
  pub fn add_node(&mut self, pos: usize, len: usize, node: NodeKind) {
    self.tree.add_child(self.scope, &Span::new(pos, len), node);
  }

  /// Add a new scope to the current scope.
  pub fn add_scope(&mut self, node: NodeKind) {
    self.scope = self.tree.add_child(self.scope, &Span::new(self.cursor, self.consumed - self.cursor), node);
  }

  /// Moves to the next token.
  pub fn next(&mut self) -> Option<Token> {
    let Some(next) = self.tokens.next() else {
      return None;
    };
    //let s = &self.file[self.consumed..self.consumed+next.get_len()];
    self.consumed += next.get_len();
    Some(next)
  }

  /// Peeks the next token from the tokens iterator without consuming it.
  /// Returns `TokenKind::EOF` if the next token was None.
  pub fn first(&self) -> TokenKind {
    // `.next()` optimizes better than `.nth(0)`
    let Some(token) = self.tokens.clone().next() else {
      return TokenKind::EOF;
    };
    token.kind
  }

  /// Eat while the next token matches a token kind.
  pub fn eat_while(&mut self, kind: TokenKind) {
    loop {
      let next = self.first();
      if next != kind {
        return;
      }
      self.next();
    }
  }

  /// Returns the number of characters consumed.
  pub fn consumed(&self) -> usize {
    self.consumed
  }
}