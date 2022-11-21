#[derive(Debug, Clone)]
pub enum GrammarNode {
  HTML,
  CSS,
  JS,
  IMPORT
}

#[derive(Debug, Clone)]
pub struct Node {
  pub children: Vec<Node>,
  pub entry: GrammarNode,
}

impl Node {
  pub fn new(entry: GrammarNode) -> Node {
    Node {
      children: Vec::new(),
      entry,
    }
  }
}
