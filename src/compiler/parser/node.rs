#[derive(Debug, Clone)]
pub enum GrammarItem {
    Product,
    Sum,
    Number(u64),
    Paren
}

#[derive(Debug, Clone)]
pub struct Node {
  pub children: Vec<Node>,
  pub entry: GrammarItem,
}

impl Node {
  pub fn new() -> Node {
    Node {
      children: Vec::new(),
      entry: GrammarItem::Paren,
    }
  }
}
