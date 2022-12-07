#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Span {
  start_index: usize,
  length: usize,
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

  #[allow(unused)]
  pub fn get_line<'a>(&self, file: &'a str) -> Line<'a> {
    todo!();
  }
}