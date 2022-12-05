#[derive(Debug)]
pub struct Attribute {
  pub name: String,
  pub value: String,
}

#[derive(Debug)]
pub enum SyntaxNode {
  Root,

  Template{ name: String },
  Tag{ name: String, attributes: Vec<Attribute>, self_closing: bool }
  // TODO: build the enum ...
}