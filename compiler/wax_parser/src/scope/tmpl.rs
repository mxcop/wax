use peekmore::PeekMoreIterator;
use wax_lexer::token::Token;
use std::slice::Iter;

use crate::{tree::ArenaTree, node::SyntaxNode};

pub struct TemplateParser {}

impl TemplateParser {
  /// ### Parse Template
  pub fn parse_tmpl<'a>(
    iter: &mut PeekMoreIterator<Iter<'a, Token>>, 
    tree: &mut ArenaTree<SyntaxNode>) 
  {
    // Peek through all tokens until we reach a semicolon:
    while let Some(&tk) = iter.peek_next() {
      match tk {
        Token::Semicolon => {
          iter.truncate_iterator_to_cursor();
          return;
        }
        _ => {}
      }
    }
  }
}