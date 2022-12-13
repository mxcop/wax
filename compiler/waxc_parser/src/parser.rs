use waxc_lexer::{token::{Token, TokenKind}, iter::LexIter};

use crate::{tree::AST, node::{Node, NodeKind}, span::Span};

/// The Wax parser
pub(crate) struct Parser {
  tree: AST,
  scope: usize,
  iter: LexIter,
  file: String,
}

impl Parser {
  pub fn new(file: String, iter: LexIter) -> Self {

    let mut tree = AST::new();
    let scope = tree.add_node(Span::new(0, 0), NodeKind::Root);

    Self {
      file,
      iter,
      scope,
      tree,
    }
  }

  /// Get a clone of the abstract syntax tree.
  pub fn get_tree(&self) -> AST {
    self.tree.clone()
  }

  /// Add a new node to the current scope.
  pub fn add_node(&mut self, start: usize, node: NodeKind) {
    self.tree.add_child(self.scope, &Span::new(start, self.consumed() - start), node);
  }

  /// Add a new scope to the current scope.
  pub fn add_scope(&mut self, start: usize, node: NodeKind) {
    self.scope = self.tree.add_child(self.scope, &Span::new(start, self.consumed() - start), node);
  }

  pub fn retreat_scope(&mut self) {
    match self.tree.get_parent(self.scope) {
      Some(scope) => self.scope = scope,
      None => todo!()
    }
  }

  /// Bump the iterator to the next token. 
  /// Returns an EOF token if the next token doesn't unwrap.
  pub fn next(&mut self) -> Token {
    self.iter.next()
  }

  /// Returns the next element without consuming it.
  pub fn first(&mut self) -> TokenKind {
    self.iter.first()
  }

  /// Returns the next next element without consuming it.
  pub fn second(&mut self) -> TokenKind {
    self.iter.second()
  }

  /// Read a string from to the end of the last consumed token.
  pub fn read(&self, from: usize) -> &str {
    &self.file[self.iter.range_to_consumed(from)]
  }

  /// Returns the total length of all consumed tokens.
  pub fn consumed(&self) -> usize {
    self.iter.consumed()
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

  /// Eat until the next token matches a token kind.
  pub fn eat_until(&mut self, kind: TokenKind) {
    loop {
      let next = self.first();
      if next != kind {
        return;
      }
      self.next();
    }
  }
}