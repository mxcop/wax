use std::{slice::Iter, iter::Peekable};
use peekmore::{PeekMoreIterator, PeekMore};
use super::token::{SyntaxToken, Token};

/// Custom iterator for syntax tokens.
pub struct TokenIter<'a> {
  iter: PeekMoreIterator<Iter<'a, SyntaxToken>>,
  pos: usize,
}

impl<'a> TokenIter<'a> {
  pub fn new(tokens: &'a Vec<SyntaxToken>) -> Self {
    Self {
      iter: tokens.iter().peekmore(),
      pos: 0
    }
  }

  pub fn current_pos(&self) -> usize {
    self.pos
  }

  /// Advances the iterator and returns the next value.
  pub fn next(&mut self) -> Option<&'a Token> {
    if let Some(tk) = self.iter.next() {
      self.pos += 1;
      Some(&tk.kind)
    } else {
      None
    }
  }

  /// Advances the iterator and returns the next value. (detailed return)
  pub fn next_de(&mut self) -> Option<(&'a SyntaxToken, &'a Token)> {
    if let Some(tk) = self.iter.next() {
      self.pos += 1;
      Some((tk, &tk.kind))
    } else {
      None
    }
  }

  /// Get a reference to the element where the cursor currently points to.
  pub fn peek(&mut self) -> Option<&'a Token> {
    if let Some(tk) = self.iter.peek() {
      Some(&tk.kind)
    } else {
      None
    }
  }

  /// Get a reference to the element where the cursor currently points to. (detailed return)
  pub fn peek_de(&mut self) -> Option<(&'a SyntaxToken, &'a Token)> {
    if let Some(tk) = self.iter.peek() {
      Some((tk, &tk.kind))
    } else {
      None
    }
  }

  /// Advance the cursor to the next element and return a reference to that value.
  pub fn peek_next(&mut self) -> Option<&'a Token> {
    if let Some(tk) = self.iter.peek_next() {
      Some(&tk.kind)
    } else {
      None
    }
  }

  /// Remove all elements from the start of the iterator until reaching the same position as the cursor by calling `Iterator::next()`.
  pub fn next_until_cursor(&mut self) {
    self.iter.truncate_iterator_to_cursor();
  }

  /// Move the cursor to the previous peekable element.
  pub fn retreat_cursor(&mut self) -> Result<(), peekmore::PeekMoreError> {
    match self.iter.move_cursor_back() {
      Ok(_) => Ok(()),
      Err(e) => Err(e),
    }
  }
}

/// Custom iterator that tracks the position of the iterator.
pub struct TrackingIter<'a, T> {
  iter: Peekable<Iter<'a, T>>,
  pos: usize,
}

impl<'a, T> TrackingIter<'a, T> {
  pub fn new(vec: &'a Vec<T>) -> Self {
    Self {
      iter: vec.iter().peekable(),
      pos: 0
    }
  }

  pub fn current_pos(&self) -> usize {
    self.pos
  }

  /// Advances the iterator and returns the next value.
  pub fn next(&mut self) -> Option<&'a T> {
    self.pos += 1;
    self.iter.next()
  }

  /// Returns a reference to the next() value without advancing the iterator.
  pub fn peek(&mut self) -> Option<&&'a T> {
    self.iter.peek()
  }
}