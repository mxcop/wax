/// Spanner indicating the position and length of a [Node].
#[derive(Debug, Clone, Copy)]
pub struct Span {
  pub pos: usize,
  pub len: usize,
}

impl Span {
  pub fn new(pos: usize, len: usize) -> Self {
    Self {
      pos,
      len
    }
  }
}