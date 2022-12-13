use crate::span::Span;

/// Abstract syntax tree node
#[derive(Debug, Clone)]
pub struct Node
{
  /* Data */
  pub kind: NodeKind,
  span: Span,

  /* Context */
  pub idx: usize,
  pub parent: Option<usize>,
  pub children: Vec<usize>,
}

impl Node
{
  pub fn new(idx: usize, span: Span, kind: NodeKind) -> Self {
    Self {
      /* Data */
      kind,
      span,

      /* Context */
      idx,
      parent: None,
      children: vec![],
    }
  }

  /// Get a reference to the name of the node.
  /// [Debug] This is for debugging purposes!
  pub fn get_name(&self) -> String {
    format!("{:?}", &self)
  }

  /// Get a reference to the span of the node.
  pub fn get_span(&self) -> &Span {
    &self.span
  }
}

#[derive(Debug, Clone)]
pub struct Attribute {
  pub name: String,
  pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub enum NodeKind {
  Root,

  /** Template definition */
  Template{ name: String },
  Implementation{ name: String },
  Stylesheet{ name: String },
  Tag{ name: String, attributes: Vec<Attribute>, self_closing: bool }, /* Html tag */
  Comb{ name: String, attributes: Vec<Attribute>, self_closing: bool }, /* Comb tag */
  Text(String)
}
