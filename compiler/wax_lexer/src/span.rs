#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Span {
  pub start_index: usize,
  pub length: usize,
}

pub struct Line<'a> {
  pub num: usize,
  pub str: &'a str,
}

impl Span {
  pub fn new(idx: usize, len: usize) -> Self {
    Self {
      start_index: idx,
      length: len
    }
  }
}