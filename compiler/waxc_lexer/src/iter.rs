use crate::token::{Token, TokenKind, EOF_TOKEN};

trait ClonableIterator: Iterator {
  /// Clone the iterator within the Box and put it inside a new Box.
  fn clone_heap(&self) -> Box<dyn ClonableIterator<Item = Self::Item>>;
}

impl<T> ClonableIterator for T
where
  T: 'static + Iterator + Clone,
{
  fn clone_heap(&self) -> Box<dyn ClonableIterator<Item = Self::Item>> {
    Box::new(self.clone())
  }
}

/// Lexical iterator wrapper.
/// Used to abstract the iterator returned by the lexer.
pub struct LexIter {
  len_consumed: usize,
  /// Iterator over tokens. Stored on the heap because of it's dynamic size.
  iter: Box<dyn ClonableIterator<Item = Token>>
}

impl LexIter {
  pub fn new(iter: impl Iterator<Item = Token> + Clone + 'static) -> Self {
    Self {
      len_consumed: 0,
      iter: Box::new(iter)
    }
  }

  /// Bump the iterator to the next token.
  /// Returns an EOF token if the next token doesn't unwrap.
  pub fn next(&mut self) -> Token {
    let token = self.iter.next().unwrap_or(EOF_TOKEN);
    self.len_consumed += token.get_len();
    token
  }

  /// Returns the next element without consuming it.
  pub fn first(&mut self) -> TokenKind {
    self.iter.clone_heap().next().unwrap_or(EOF_TOKEN).kind
  }

  /// Returns the next next element without consuming it.
  pub fn second(&mut self) -> TokenKind {
    let mut clone = self.iter.clone_heap();
    clone.next();
    clone.next().unwrap_or(EOF_TOKEN).kind
  }

  /// Get a range from start to the end of the last consumed token.
  pub fn range_to_consumed(&self, start: usize) -> std::ops::Range<usize> {
    start..self.len_consumed
  }

  /// Returns the total length of all consumed tokens.
  pub fn consumed(&self) -> usize {
    self.len_consumed
  }
}
