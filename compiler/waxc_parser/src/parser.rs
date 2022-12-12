use waxc_lexer::token::{Token, TokenKind};

use crate::{tree::AST, node::{Node, NodeKind}, span::Span};

/// The Wax parser
pub(crate) struct Parser<T: Iterator<Item = Token> + Clone> {
  consumed: usize,
  cursor: usize,

  tree: AST,
  tokens: T,
  scope: usize,
  pub file: String,
}

impl<T: Iterator<Item = Token> + Clone> Parser<T> {
  pub fn new(file: String, input: T) -> Self {

    let mut tree = AST::new();
    let scope = tree.add_node(Span::new(0, 0), NodeKind::Root);

    Self {
      file,
      scope,
      consumed: 0,
      cursor: 0,
      tree,
      tokens: input
    }
  }

  /// Read from the input file from a starting location until the consumed position.
  pub fn read(&self) -> &str {
    &self.file[self.cursor..self.consumed]
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
  pub fn add_scope(&mut self, pos: usize, len: usize, node: NodeKind) {
    self.scope = self.tree.add_child(self.scope, &Span::new(pos, len), node);
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

  /// Returns the number of characters consumed.
  pub fn consumed(&self) -> usize {
    self.consumed
  }
}