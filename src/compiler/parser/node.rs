#[derive(Debug)]
pub struct Attribute {
  pub name: String,
  pub value: String,
}

#[derive(Debug)]
pub enum NodeType {
  Root,
  Tag { attributes: Vec<Attribute> },
  DefaultImport { name: String, path: String },
}
