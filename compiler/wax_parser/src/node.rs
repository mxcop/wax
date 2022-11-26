#[derive(Debug)]
pub struct Attribute {
  pub name: String,
  pub value: String,
}

#[derive(Debug)]
pub enum NodeType {
  Root,
  Text(String),
  Tag { attributes: Vec<Attribute> },
  DefaultImport { specifier: String, source: String },
}