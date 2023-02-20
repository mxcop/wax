pub mod page;
mod visit;

use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use page::OutputPage;
use tiny_id::ShortCodeGenerator;
use visit::walk_pages;
use waxc_errors::error::{WaxError, WaxHint};
use waxc_parser::tree::AST;
use waxc_parser::node::{NodeKind, Node, Attribute};

pub fn generate(index_path: &Path, pages_dir: &Path) -> Result<Vec<OutputPage>, WaxError> {
  /* Path related checks: */
  if !index_path.exists() {
    return Err(WaxError::new(
      0, 0, 
      "missing index.html file", 
    WaxHint::None, None));
  }
  if !pages_dir.exists() {
    return Err(WaxError::new(
      0, 0, 
      "pages directory doesn't exist", 
    WaxHint::None, None));
  }
  if !pages_dir.is_dir() {
    return Err(WaxError::new(
      0, 0, 
      "pages directory needs to be a directory", 
    WaxHint::None, None));
  }

  let Ok(index_html) = read_to_string(index_path) else {
    return Err(WaxError::new(
      0, 0, 
      "couldn't read index.html file", 
    WaxHint::None, None));
  };

  /* Create the output pages vector */
  let mut pages: Vec<OutputPage> = Vec::with_capacity(8);

  /* Walk the pages directory recursively */
  walk_pages(&mut pages, pages_dir, &|page_path| {
    genpage(page_path, &index_html, index_path.parent()
      .expect("couldn't find index.html directory"))
  })?;

  Ok(pages)
}

/// Generate HTML, JS, and CSS from an abstract syntax tree.<br>
/// ```toml
/// index # index.html file
/// ast   # abstract syntax tree
/// ```
pub fn genpage(page_path: &Path, index: &String, index_path: &Path) -> Result<OutputPage, WaxError> {
  let Ok(page_file) = read_to_string(page_path) else {
    return Err(WaxError::new(
      0, 0, 
      "couldn't read page file", 
    WaxHint::None, None));
  };

  let ast = genmod(page_file, page_path)?;
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
      /* use "<path>"; */
      NodeKind::Using { parts: _, path } => {
        // See if the path exists, if so then load the wax file.
        let path = index_path.join(path.trim_matches('"'));
        
        if path.exists() {
          // TODO: Make this using part recursive.
          // Write all the found modules into the templates hashmap.

          let file = std::fs::read_to_string(&path).unwrap();

          let module = genmod(file, page_path)?;
          let mut module_root_nodes = module.get_children(0);

          while let Some(module_node) = module_root_nodes.next() {
            if let NodeKind::Template { name } = &module_node.kind {
              templates.insert(
                name.clone(), 
                build_template(&path, &module, &templates, module_node, &mut hasher)?
              );
            }
          }
        } else {
          return Err(WaxError::new(
            0, 0, 
            "module not found", 
          WaxHint::None, Some(page_path)));
        }
      },

      /* tmpl <name>: */
      NodeKind::Template { name } => match name {
        n if is_base(n) => {
          base_found = true;
          html.push_str(&build_template(page_path, &ast, &templates, base_node, &mut hasher)?);
        }
        _ => { 
          templates.insert(
            name.clone(), 
            build_template(page_path, &ast, &templates, base_node, &mut hasher)?
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
    WaxHint::Example("tmpl @base:".into()), Some(page_path)));
  }

  /* Insert html */
  html = index.replace("@wax.base", &html);
  html = html.replace("@wax.head", "<link rel=\"stylesheet\" href=\"index.css\">"); /* Temp */

  /* Trim whitespace */
  js = js.trim().to_string();

  Ok(OutputPage::new(
    page_path.to_string_lossy().to_string(),
    html, 
    js, 
    css
  ))
}

/// Generate the abstract syntax tree for a wax module.
fn genmod(file: String, filepath: &Path) -> Result<AST, WaxError> {
  // Initialize the lexical iterator:
  let lexer = waxc_lexer::lex(&file);
  let iter = waxc_lexer::lexer::LexIter::new(lexer);

  // Start the parsing process:
  waxc_parser::parse(file.clone(), filepath, iter)
}

/// Is this template a base template? (@html)
fn is_base(name: &str) -> bool {
  name == "@base"
}

/// Build a template node.
fn build_template(page_path: &Path, ast: &AST, cache: &HashMap<String, String>, scope: &Node, hasher: &mut ShortCodeGenerator<char>) -> Result<String, WaxError> {
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
            WaxHint::None, Some(page_path)));
          };
          contents.push_str(&comb);
        },
        NodeKind::Text(content) => contents.push_str(content),
        _ => ()
      }
    /* Is tag node with child nodes */
    } else if let NodeKind::Tag { name, attributes, .. } = &node.kind {
      contents.push_str(&build_tag(&hash, name, attributes, false));
      contents.push_str(&build_template(page_path, ast, cache, node, hasher)?);
      contents.push_str(&build_end_tag(name));
    } else {
      unreachable!("unhandled node in template ({})", node.get_name());
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