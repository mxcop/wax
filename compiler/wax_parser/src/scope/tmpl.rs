use peekmore::PeekMoreIterator;
use wax_lexer::token::Token;
use std::slice::Iter;

use crate::{tree::ArenaTree, node::SyntaxNode};

/// Wax template parser.
pub struct TemplateParser {}

impl TemplateParser {
  /// ### Parse Template
  pub fn parse_tmpl<'a>(
    iter: &mut PeekMoreIterator<Iter<'a, Token>>, 
    curr: &mut usize,
    tree: &mut ArenaTree<SyntaxNode>)
  {
    // Make sure that the next token is whitespace.
    if let Some(Token::Whitespace(_)) = iter.next() {} 
    else { panic!("No whitespace after `tmpl`"); }

    // And then there should be an indentifier for the template.
    if let Some(Token::Ident(ident)) = iter.peek() {
      *curr = tree.add_child(*curr, ident.clone(), SyntaxNode::Template {
        name: ident.clone()
      });
      iter.next();
    } 
    // Also check if the identifier starts with an atsign.
    else if let Some(Token::Atsign) = iter.next() {
      if let Some(Token::Ident(ident)) = iter.next() {
        let ident = format!("@{ident}");
        *curr = tree.add_child(*curr, ident.clone(), SyntaxNode::Template {
          name: ident.clone()
        });
      } 
    }
    else { panic!("No name after `tmpl`"); }

    // Move through all tokens until we reach a semicolon:
    while let Some(tk) = iter.next() {
      match tk {

        /* < */
        Token::LessThen => {
          /* Opening Tag <tag> */
          if let Some(Token::Ident(ident)) = iter.peek() {
            *curr = tree.add_child(*curr, ident.clone(), SyntaxNode::Tag { 
              name: ident.clone(), 
              attributes: vec![], 
              self_closing: false 
            });
          /* Closing Tag </tag> */
          } else if let Some(Token::Slash) = iter.peek() {
            if let Some(Token::Ident(_)) = iter.peek_next() {
              *curr = tree.get_parent(*curr).expect("No parent");
            } else {
              iter.move_cursor_back().expect("failed to move back cursor");
            }
          }
        }

        /* ; */
        Token::Semicolon => {
          break;
        }

        _ => {}
      }
    }

    // Move back out of this template.
    *curr = tree.get_parent(*curr).expect("No parent");
  }
}