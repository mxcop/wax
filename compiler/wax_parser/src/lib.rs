pub mod node;
pub mod tree;
mod scope;

use node::SyntaxNode;
use tree::ArenaTree;
use scope::tmpl::TemplateParser;

use wax_lexer::{token::{Token, SyntaxToken}, iter::TokenIter};
use wax_logger::error::WaxError;

/// The Wax parser.
pub struct Parser<'a> {
  iter: TokenIter<'a>
}

impl<'a> Parser<'a> {
  pub fn new(input: &'a Vec<SyntaxToken>) -> Self {
    Self {
      iter: TokenIter::new(input)
    }
  }

  #[allow(unused_variables)]
  /// ### Syntactic Analysis
  /// Analize the input tokens and convert it into an abstract syntax tree.
  pub fn parse(&mut self) -> Result<ArenaTree<SyntaxNode>, WaxError> {
    let mut tree: ArenaTree<SyntaxNode> = ArenaTree::new();
    let mut curr = tree.add_node("Root".to_string(), SyntaxNode::Root);

    while let Some((dtk, tk)) = self.iter.next_de() {
      match tk {
        Token::Template => {
          TemplateParser::parse_tmpl(&mut self.iter, &mut curr, &mut tree)?;
        }
        // Token::Slash => { return Err(WaxError::from_token(dtk.clone(), "test msg", None)); }
        _ => {}
      }
    }

    Ok(tree)
  }
}
