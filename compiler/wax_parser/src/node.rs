#[derive(Debug, Clone)]
pub struct Attribute {
  pub name: String,
  pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SyntaxNode {
  Root,

  /** Template definition */
  Template{ name: String },
  Tag{ name: String, attributes: Vec<Attribute>, self_closing: bool }, /* Html tag */
  Comb{ name: String, attributes: Vec<Attribute>, self_closing: bool }, /* Comb tag */
}