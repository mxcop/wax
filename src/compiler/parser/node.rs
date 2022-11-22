#[derive(Debug)]
pub struct Attribute {
  pub name: String,
  pub value: String,
}

#[derive(Debug)]
pub enum NodeType {
  Root,
  Script { attributes: Vec<Attribute> },
  Style { attributes: Vec<Attribute> },
  Tag { attributes: Vec<Attribute> },
}
