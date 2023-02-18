use std::path::Path;

use waxc_errors::error::{WaxError, WaxHint};
use waxc_lexer::{token::{Token, TokenKind, EOF_TOKEN}, lexer::LexIter};

use crate::{tree::AST, node::{NodeKind, Span, Node}};

/// The Wax parser
pub struct Parser<'a, I: Iterator<Item = Token> + Clone> {
  file: String,
  filepath: &'a Path,
  len_consumed: usize,
  pos_cursor: usize,
  pos_checkpoint: usize,
  /* Lexical iterator */
  iter: LexIter<I>,

  tree: AST,
  scope: usize,
}

impl<'a, I: Iterator<Item = Token> + Clone> Parser<'a, I> {
  pub fn new(file: String, filepath: &'a Path, iter: LexIter<I>) -> Self {
    /* Initialize the abstract syntax tree */
    let mut tree = AST::new();
    let scope = tree.add_node(Span::new(0, 0), NodeKind::Root);

    Self {
      file,
      filepath,
      len_consumed: 0,
      pos_cursor: 0,
      pos_checkpoint: 0,
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

  /// Get a node from the abstract syntax tree.
  // pub fn get_node(&self, idx: usize) -> &Node {
  //   self.tree.get(idx)
  // }

  /// Get the node that is currently in scope.
  pub fn scoped_node(&self) -> &Node {
    self.tree.get(self.scope)
  }

  /// Add a new node to the current scope.
  pub fn add_node(&mut self, node: NodeKind) {
    self.tree.add_child(self.scope, 
      &Span::new(self.pos_cursor, self.len_consumed - self.pos_cursor), 
    node);
  }

  /// Add a new node to the current scope using the checkpoint for it's position.
  pub fn add_node_with_checkpoint(&mut self, node: NodeKind) {
    self.tree.add_child(self.scope, 
      &Span::new(self.pos_checkpoint, self.len_consumed - self.pos_checkpoint),
    node);
  }

  /// Add a new scope to the current scope.
  pub fn add_scope(&mut self, node: NodeKind) {
    self.scope = self.tree.add_child(self.scope, 
      &Span::new(self.pos_cursor, self.len_consumed - self.pos_cursor), 
    node);
  }

  /// Add a new scope to the current scope using the checkpoint for it's position.
  pub fn add_scope_with_checkpoint(&mut self, node: NodeKind) {
    self.scope = self.tree.add_child(self.scope, 
      &Span::new(self.pos_checkpoint, self.len_consumed - self.pos_checkpoint),
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
  pub fn next_with_cursor(&mut self) -> Token {
    self.update_cursor();
    let next = self.iter.next().unwrap_or(EOF_TOKEN);
    self.len_consumed += next.len();
    next
  }

  /// Bump the iterator to the next token. 
  pub fn next(&mut self) -> Token {
    let next = self.iter.next().unwrap_or(EOF_TOKEN);
    self.len_consumed += next.len();
    next
  }

  /// Returns the next element without consuming it.
  pub fn first(&mut self) -> TokenKind {
    self.iter.first()
  }

  /// Returns the next next element without consuming it.
  // pub fn second(&mut self) -> TokenKind {
  //   self.iter.second()
  // }

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

  /// Create a new wax error using a node.
  pub fn err_node(&self, node: &Node, msg: &str, hint: WaxHint) -> WaxError {
    let span = node.get_span();
    WaxError::new(span.pos, span.len, msg, hint, Some(self.filepath))
  }

  /// Create a new wax error with an example.
  pub fn err_example(&self, msg: &str, example: &str) -> WaxError {
    WaxError::new(self.pos_cursor, self.len_consumed - self.pos_cursor, msg, WaxHint::Example(example.into()), Some(self.filepath))
  }

  /// Create a new wax error with a hint.
  pub fn err_hint(&self, msg: &str, hint: &str) -> WaxError {
    WaxError::new(self.pos_cursor, self.len_consumed - self.pos_cursor, msg, WaxHint::Hint(hint.into()), Some(self.filepath))
  }

  /// Create a new wax error with a hint.
  pub fn err(&self, msg: &str) -> WaxError {
    WaxError::new(self.pos_cursor, self.len_consumed - self.pos_cursor, msg, WaxHint::None, Some(self.filepath))
  }

  /// Set a checkpoint (useful for throwing more accurate errors)
  pub fn checkpoint(&mut self) {
    self.pos_checkpoint = self.pos_cursor;
  }
}