use waxc_errors::error::WaxError;
use waxc_lexer::{token::{Token, TokenKind}, lexer::LexIter};

use crate::{tree::AST, node::{NodeKind, Span}};

/// The Wax parser
pub struct Parser<I: Iterator<Item = Token> + Clone> {
  file: String,
  len_consumed: usize,
  pos_cursor: usize,
  /* Lexical iterator */
  iter: LexIter<I>,

  tree: AST,
  scope: usize,
}

impl<I: Iterator<Item = Token> + Clone> Parser<I> {
  pub fn new(file: String, iter: LexIter<I>) -> Self {
    /* Initialize the abstract syntax tree */
    let mut tree = AST::new();
    let scope = tree.add_node(Span::new(0, 0), NodeKind::Root);

    Self {
      file,
      len_consumed: 0,
      pos_cursor: 0,
      iter,
      tree,
      scope,
    }
  }

  /// Get a clone of the abstract syntax tree.
  // todo: optimize this so we don't need to clone the entire tree...
  pub fn get_tree(&self) -> AST {
    self.tree.clone()
  }

  /// Add a new node to the current scope.
  pub fn add_node(&mut self, node: NodeKind) {
    self.tree.add_child(self.scope, 
      &Span::new(self.pos_cursor, self.len_consumed - self.pos_cursor), 
    node);
  }

  /// Add a new scope to the current scope.
  pub fn add_scope(&mut self, node: NodeKind) {
    self.scope = self.tree.add_child(self.scope, 
      &Span::new(self.pos_cursor, self.len_consumed - self.pos_cursor), 
    node);
  }

  /// Change the current scope to it's parent scope.
  pub fn retreat_scope(&mut self) {
    match self.tree.get_parent(self.scope) {
      Some(scope) => self.scope = scope,
      None => () // println!("went out of scope!!!")
    }
  }

  /// Get the current scope.
  pub fn get_scope(&self) -> usize {
    self.scope
  }

  /// Bump the iterator to the next token after updating the cursor. 
  pub fn next_with_cursor(&mut self) -> Option<Token> {
    self.update_cursor();
    let Some(next) = self.iter.next() else {
      return None;
    };
    self.len_consumed += next.len();
    Some(next)
  }

  /// Bump the iterator to the next token. 
  pub fn next(&mut self) -> Option<Token> {
    let Some(next) = self.iter.next() else {
      return None;
    };
    self.len_consumed += next.len();
    Some(next)
  }

  /// Returns the next element without consuming it.
  pub fn first(&mut self) -> TokenKind {
    self.iter.first()
  }

  /// Returns the next next element without consuming it.
  #[allow(unused)]
  pub fn second(&mut self) -> TokenKind {
    self.iter.second()
  }

  /// Read from the last cursor reset until the end of the last consumed token.
  pub fn read(&self) -> String {
    self.file[self.pos_cursor..self.len_consumed].to_string()
  }

  /// Update the cursor position to the end of the last consumed token.
  pub fn update_cursor(&mut self) {
    self.pos_cursor = self.len_consumed;
  }

  /// Eat tokens while they match a kind of token.
  pub fn eat_while(&mut self, kind: TokenKind) {
    self.len_consumed += self.iter.eat_while(kind);
  }

  /// Eat tokens until one matches a kind of token.
  #[allow(unused)]
  pub fn eat_until(&mut self, kind: TokenKind) {
    self.len_consumed += self.iter.eat_until(kind);
  }

  /// Eat tokens until the end of a scope.<br>
  /// `open_token`: Token that opens a scope.<br>
  /// `close_token`: Token that closes a scope.
  #[allow(unused)]
  pub fn eat_scope(&mut self, open_token: TokenKind, close_token: TokenKind) -> Result<(), WaxError> {
    let Ok(len) = self.iter.eat_scope(open_token, close_token) else {
      todo!();
    };
    self.len_consumed += len;
    Ok(())
  }
}