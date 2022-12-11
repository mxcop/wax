// Referenced from https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6

use crate::{span::Span, node::{Node, NodeKind}};

#[derive(Default)]
pub struct AST
{
  arena: Vec<Node>,
}

impl AST
{
  pub fn new() -> Self {
    Self { arena: Vec::new() }
  }

  /// Add a new node to the tree. (without parent)
  pub fn add_node(&mut self, span: Span, kind: NodeKind) -> usize {
    let idx = self.arena.len();
    self.arena.push(Node::new(idx, span, kind));
    idx
  }

  /// Add a new node to the tree as child of an existing node.
  pub fn add_child(&mut self, parent_idx: usize, span: &Span, kind: NodeKind) -> usize {
    let idx = self.arena.len();
    // Create and add the node :
    let mut node = Node::new(idx, *span, kind);
    node.parent = Some(parent_idx);
    // Add the node into the arena and the children array on the parent :
    self.arena.push(node);
    self.arena[parent_idx].children.push(idx);
    idx
  }

  /// Get a node in the tree by its id.
  pub fn get(&self, idx: usize) -> &Node {
    &self.arena[idx]
  }

  /// Get the parent of a node in the tree by its id.
  pub fn get_parent(&self, idx: usize) -> Option<usize> {
    self.arena[idx].parent
  }
}

impl std::fmt::Display for AST
{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    use colored::Colorize;

    fn append_tabs(f: &mut std::fmt::Formatter, level: u32) -> std::fmt::Result {
      for _ in 0..level {
        write!(f, "  ")?;
      }
      Ok(())
    }

    fn recurse(f: &mut std::fmt::Formatter, tree: &AST, node: &Node, level: u32) -> std::fmt::Result
    {
      for child in &node.children {
        let child = tree.get(*child);

        append_tabs(f, level)?;

        if child.children.len() == 0 {

          match &child.kind {

            /* Tags */
            NodeKind::Tag { name, attributes, .. } => {
              writeln!(f, "{} {}", name, format!("({:?})", attributes).bright_black())?;
            }

            /* Comb Tags */
            NodeKind::Comb { name, attributes, .. } => {
              writeln!(f, "{}{} {}", "<-".bright_black(), name.green(), format!("({:?})", attributes).bright_black())?;
            }
            
            _ => { writeln!(f, "{} {}", child.get_name(), format!("({:?})", child.kind).bright_black())?; }
          }
          
        } else {

          match &child.kind {

            /* Templates */
            NodeKind::Template { name } => { 
              if name.starts_with('@') { writeln!(f, "{} {}: {{", "impl".red(), name.blue())?; }
              else { writeln!(f, "{} {}: {{", "impl".red(), name.green())?; }
            }

            /* Comb Tags */
            NodeKind::Comb { name, .. } => {
              writeln!(f, "{}{}: {{", "<-".bright_black(), name.green())?;
            }
            
            _ => { writeln!(f, "{}: {{", child.get_name())?; }
          }

          recurse(f, &tree, &child, level + 1)?;

          append_tabs(f, level)?;
          writeln!(f, "}}")?;
        }
      }

      Ok(())
    }

    writeln!(f, "{{")?;

    let level: u32 = 1;

    for node in &self.arena {
      if node.parent.is_none() {
        recurse(f, &self, &node, level)?;
      }
    }
    
    writeln!(f, "}}")
  }
}