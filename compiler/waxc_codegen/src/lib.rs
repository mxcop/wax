pub mod comb;
//mod hash;

use std::collections::HashMap;

use comb::WaxComb;
use tiny_id::ShortCodeGenerator;
use waxc_errors::error::{WaxError, WaxHint};
use waxc_parser::tree::AST;
use waxc_parser::node::{NodeKind, Node, Attribute};

/// Generate HTML, JS, and CSS from an abstract syntax tree.<br>
/// ```toml
/// index # index.html file
/// ast   # abstract syntax tree
/// ```
pub fn generate(index: String, ast: AST) -> Result<WaxComb, WaxError> {
  let mut root_nodes = ast.get_children(0);
  
  /* Create hasher instance */
  let mut hasher = ShortCodeGenerator::new_alphanumeric(4);

  /* Template cache (name, contents) */
  let mut templates: HashMap<String, String> = HashMap::new();
  let mut html: String = String::with_capacity(256);
  let mut js: String = String::with_capacity(128);
  let mut css: String = String::with_capacity(256);

  let mut base_found = false;
  
  /* Find the file root node (@html) */
  while let Some(base_node) = root_nodes.next() {
    match &base_node.kind {
      /* tmpl <name>: */
      NodeKind::Template { name } => match name {
        n if is_base(n) => {
          base_found = true;
          html.push_str(&build_template(&ast, &templates, base_node, &mut hasher)?);
        }
        _ => { 
          templates.insert(
            name.clone(), 
            build_template(&ast, &templates, base_node, &mut hasher)?
          );
        }
      },

      /* impl <name>() */
      NodeKind::Implementation { .. } => {
        if let Some(node) = ast.get_children(base_node.idx).next() {
          if let NodeKind::Text(contents) = &node.kind {
            js.push_str(contents);
          }
        }
      },

      /* styl <name> */
      NodeKind::Stylesheet { .. } => {
        if let Some(node) = ast.get_children(base_node.idx).next() {
          if let NodeKind::Text(contents) = &node.kind {
            css.push_str(contents);
          }
        }
      },
      _ => {}
    }
  }

  if !base_found {
    return Err(WaxError::new(
      0, 0, 
      "missing base template", 
    WaxHint::Example("tmpl @base:".into())));
  }

  /* Insert html */
  html = index.replace("@wax.base", &html);
  html = html.replace("@wax.head", ""); /* Temp */

  /* Trim whitespace */
  js = js.trim().to_string();

  Ok(WaxComb::new(
    html, 
    js, 
    css
  ))
}

/// Is this template a base template? (@html)
fn is_base(name: &str) -> bool {
  name == "@base"
}

/// Recursively build a template node.
fn build_template(ast: &AST, cache: &HashMap<String, String>, scope: &Node, hasher: &mut ShortCodeGenerator<char>) -> Result<String, WaxError> {
  let mut iter = ast.get_children(scope.idx);
  let mut contents = String::with_capacity(256);
  let hash: String = hasher.next_string();

  while let Some(node) = iter.next() {
    /* No child nodes */
    if node.children.len() == 0 {
      match &node.kind {
        NodeKind::Tag { name, attributes, self_closing } => {
          contents.push_str(&build_tag(&hash, name, attributes, *self_closing));
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
    /* Is tag node with child nodes */
    } else if let NodeKind::Tag { name, attributes, .. } = &node.kind {
      contents.push_str(&build_tag(&hash, name, attributes, false));
      contents.push_str(&build_template(ast, cache, node, hasher)?);
      contents.push_str(&build_end_tag(name));
    } else {
      unreachable!("unhandled node is template ({})", node.get_name());
    }
  }

  Ok(contents)
}

/// Construct a HTML tag from name and attributes.
fn build_tag(_hash: &str, name: &str, attributes: &Vec<Attribute>, self_closing: bool) -> String {
  let mut tag = String::with_capacity(64);

  tag.push('<');
  tag.push_str(name);

  for attrib in attributes {
    if attrib.name.starts_with('#') {
      // todo: implement links
      // let id = &attrib.name[1..];
      // tag.push_str(" id=\"");
      // tag.push_str(hash);
      // tag.push('_');
      // tag.push_str(id);
      // tag.push('"');
      continue;
    }
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