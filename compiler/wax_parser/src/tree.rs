// Referenced from https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6

use std::fmt::Debug;

use crate::node::SyntaxNode;

#[derive(Default)]
pub struct ArenaTree<T> 
  where T : Debug
{
  arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T> 
  where T : Debug
{
  pub fn new() -> ArenaTree<T> {
    ArenaTree { arena: Vec::new() }
  }

  /// Add a new node to the tree. (without parent)
  pub fn add_node(&mut self, name: String, val: T) -> usize {
    let idx = self.arena.len();
    self.arena.push(Node::new(idx, name, val));
    idx
  }

  /// Add a new node to the tree as child of an existing node.
  pub fn add_child(&mut self, parent_idx: usize, name: String, val: T) -> usize {
    let idx = self.arena.len();
    // Create and add the node :
    let mut node = Node::new(idx, name, val);
    node.parent = Some(parent_idx);
    // Add the node into the arena and the children array on the parent :
    self.arena.push(node);
    self.arena[parent_idx].children.push(idx);
    idx
  }

  /// Get a node in the tree by its id.
  pub fn get(&self, idx: usize) -> &Node<T> {
    &self.arena[idx]
  }

  /// Get the parent of a node in the tree by its id.
  pub fn get_parent(&self, idx: usize) -> Option<usize> {
    self.arena[idx].parent
  }
}

impl std::fmt::Display for ArenaTree<SyntaxNode>
{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    use colored::Colorize;

    fn append_tabs(f: &mut std::fmt::Formatter, level: u32) -> std::fmt::Result {
      for _ in 0..level {
        write!(f, "  ")?;
      }
      Ok(())
    }

    fn recurse(f: &mut std::fmt::Formatter, tree: &ArenaTree<SyntaxNode>, node: &Node<SyntaxNode>, level: u32) -> std::fmt::Result
    {
      for child in &node.children {
        let child = tree.get(*child);

        append_tabs(f, level)?;

        if child.children.len() == 0 {

          match &child.val {

            /* Tags */
            SyntaxNode::Tag { name, attributes, .. } => {
              writeln!(f, "{} {}", name, format!("({:?})", attributes).bright_black())?;
            }

            /* Comb Tags */
            SyntaxNode::Comb { name, attributes, .. } => {
              writeln!(f, "{}{} {}", "<-".bright_black(), name.green(), format!("({:?})", attributes).bright_black())?;
            }
            
            _ => { writeln!(f, "{} {}", child.name, format!("({:?})", child.val).bright_black())?; }
          }
          
        } else {

          match &child.val {

            /* Templates */
            SyntaxNode::Template { name } => { 
              if name.starts_with('@') { writeln!(f, "{} {}: {{", "impl".red(), name.blue())?; }
              else { writeln!(f, "{} {}: {{", "impl".red(), name.green())?; }
            }

            /* Comb Tags */
            SyntaxNode::Comb { name, .. } => {
              writeln!(f, "{}{}: {{", "<-".bright_black(), name.green())?;
            }
            
            _ => { writeln!(f, "{}: {{", child.name)?; }
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

pub struct Node<T> 
  where T : Debug
{
  _idx: usize,
  name: String,
  pub val: T,
  parent: Option<usize>,
  children: Vec<usize>,
}

impl<T> Node<T> 
  where T : Debug
{
  pub fn new(_idx: usize, name: String, val: T) -> Self {
    Self {
      _idx,
      name,
      val,
      parent: None,
      children: vec![],
    }
  }

  /// Get a reference to the name of the node.
  pub fn get_name(&self) -> &str {
    &self.name
  }
}
