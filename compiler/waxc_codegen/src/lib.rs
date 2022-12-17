use waxc_parser::tree::AST;
use waxc_parser::node::NodeKind;

pub fn generate(ast: AST) {
  let mut root_nodes = ast.get_children(0);

  // loop through all nodes in the root :
  // | if template node is found check if it imports any templates.
  
  /* Find the file root node (@html) */
  while let Some(base_node) = root_nodes.next() {
    match &base_node.kind {
      NodeKind::Template { name } => todo!(),
      NodeKind::Implementation { name } => todo!(),
      NodeKind::Stylesheet { name } => todo!(),
      _ => {}
    }
  }
}