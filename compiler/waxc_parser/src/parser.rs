use std::slice::Iter;

use waxc_lexer::token::{Token, TokenKind};

/// The Wax parser
pub(crate) struct Parser<'a> {
  consumed: usize,
  tokens: Iter<'a, Token>,
}

impl<'a> Parser<'a> {
  pub fn new(input: Iter<'a, Token>) -> Self {
    Self {
      consumed: 0,
      tokens: input
    }
  }

  /// Moves to the next token.
  pub fn next(&mut self) -> Option<&Token> {
    let Some(next) = self.tokens.next() else {
      return None;
    };
    self.consumed += next.get_len();
    Some(next)
  }

  /// Peeks the next token from the tokens iterator without consuming it.
  /// Returns `TokenKind::EOF` if the next token was None.
  pub fn first(&self) -> &TokenKind {
    // `.next()` optimizes better than `.nth(0)`
    let Some(token) = self.tokens.clone().next() else {
      return &TokenKind::EOF;
    };
    &token.kind
  }

  /// Returns the number of characters consumed.
  pub fn consumed(&self) -> usize {
    self.consumed
  }
}