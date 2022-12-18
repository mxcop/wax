pub mod comb;

use std::collections::HashMap;

use comb::WaxComb;
use waxc_errors::error::{WaxError, WaxHint};
use waxc_parser::tree::AST;
use waxc_parser::node::{NodeKind, Node, Attribute};

/// Generate HTML, JS, and CSS from an abstract syntax tree.
pub fn generate(ast: AST) -> Result<WaxComb, WaxError> {
  let mut root_nodes = ast.get_children(0);

  /* Template cache (name, contents) */
  let mut templates: HashMap<String, String> = HashMap::new();
  let mut html: String = String::with_capacity(256);
  
  /* Find the file root node (@html) */
  while let Some(base_node) = root_nodes.next() {
    match &base_node.kind {
      /* tmpl <name>: */
      NodeKind::Template { name } => match name {
        n if is_base(n) => {
          html.push_str(&build_template(&ast, &templates, base_node)?);
        }
        _ => { 
          templates.insert(
            name.clone(), 
            build_template(&ast, &templates, base_node)?
          );
        }
      },

      /* impl <name>() */
      NodeKind::Implementation { .. } => (),

      /* styl <name> */
      NodeKind::Stylesheet { .. } => (),
      _ => {}
    }
  }

  Ok(WaxComb::new(
    html, 
    "".into(), 
    "".into())
  )
}

/// Is this template a base template? (@html)
fn is_base(name: &str) -> bool {
  name == "@html"
}

/// Recursively build a template node.
fn build_template(ast: &AST, cache: &HashMap<String, String>, scope: &Node) -> Result<String, WaxError> {
  let mut iter = ast.get_children(scope.idx);
  let mut contents = String::with_capacity(256);

  while let Some(node) = iter.next() {
    if node.children.len() == 0 {
      match &node.kind {
        NodeKind::Tag { name, attributes, self_closing } => {
          contents.push_str(&build_tag(name, attributes, *self_closing));
        },
        NodeKind::Comb { name, .. } => {
          let Some(comb) = build_comb(cache, name) else {
            let span = node.get_span();
            return Err(WaxError::new(
              span.pos, span.len, 
              &format!("`tmpl {}` not found", node.get_name()), 
            WaxHint::None));
          };
          contents.push_str(&comb);
        },
        NodeKind::Text(content) => contents.push_str(content),
        _ => ()
      }
    } else if let NodeKind::Tag { name, attributes, .. } = &node.kind {
      contents.push_str(&build_tag(name, attributes, false));
      contents.push_str(&build_template(ast, cache, node)?);
      contents.push_str(&build_end_tag(name));
    } else {
      println!("unhandled node is template ({})", node.get_name());
    }
  }

  Ok(contents)
}

/// Construct a HTML tag from name and attributes.
fn build_tag(name: &str, attributes: &Vec<Attribute>, self_closing: bool) -> String {
  let mut tag = String::with_capacity(64);

  tag.push('<');
  tag.push_str(name);

  for attrib in attributes {
    tag.push(' ');
    tag.push_str(&attrib.name);
    if let Some(value) = &attrib.value {
      tag.push('=');
      tag.push_str(value);
    }
  }

  if self_closing { tag.push('/'); }
  tag.push('>');
  tag
}

/// Construct a HTML ending tag from a name.
fn build_end_tag(name: &str) -> String {
  let mut tag = String::with_capacity(16);

  tag.push('<');
  tag.push('/');
  tag.push_str(name);
  tag.push('>');

  tag
}

/// Grab the comb as HTML from the template cache. (if it exists)
fn build_comb(cache: &HashMap<String, String>, name: &str) -> Option<String> {

  let Some(comb) = cache.get(name) else {
    return None;
  };

  Some(comb.clone())
}