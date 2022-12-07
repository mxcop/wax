pub mod node;
pub mod tree;

mod lines;
mod scope;

use std::sync::Arc;
use node::SyntaxNode;
use tree::ArenaTree;
use scope::tmpl::TemplateParser;
use wax_lexer::{token::{Token, SyntaxToken}, iter::TokenIter};

pub struct Parser<'a> {
  /* File information */
  file: Arc<String>,
  filename: String,

  iter: TokenIter<'a>
}

impl<'a> Parser<'a> {
  pub fn new(file: Arc<String>, filename: String, input: &'a Vec<SyntaxToken>) -> Self {
    Self {
      file, filename, iter: TokenIter::new(input)
    }
  }

  /// ### Syntactic Analysis
  /// Analize the input tokens and convert it into an abstract syntax tree.
  pub fn parse(&mut self) -> ArenaTree<SyntaxNode> {
    let mut tree: ArenaTree<SyntaxNode> = ArenaTree::new();
    let mut curr = tree.add_node("Root".to_string(), SyntaxNode::Root);

    while let Some(tk) = self.iter.next() {
      match tk {
        Token::Template => {
          TemplateParser::parse_tmpl(&mut self.iter, &mut curr, &mut tree);
        }
        _ => {}
      }
    }

    tree
  }

  #[allow(unused)]
  /// Bail out of parsing, and throw an error.
  fn bail(&self, desc: &str, idx: usize, tip: Option<&str>) {
    use wax_logger::bail;

    let line_num = lines::get_line_num(&self.file, idx);

    let line_start = lines::find_line_start(&self.file, idx);
    let line_end = lines::find_line_end(&self.file, idx);

    let line = line_start..line_end;
  
    bail(desc, &self.filename, None, line_num, &self.file[line], tip);
  }
}
