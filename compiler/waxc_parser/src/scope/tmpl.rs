mod void;
mod attrib;

use waxc_lexer::{token::{Token, SyntaxToken}, iter::TokenIter};
use waxc_errors::error::{WaxError, WaxHint};

use crate::{tree::ArenaTree, node::SyntaxNode};

use void::is_void;
use attrib::parse_attributes;

/// ### Parse Template
pub fn parse<'a>(
  iter: &mut TokenIter<'a>, 
  tmpl_tk: &'a SyntaxToken,
  curr: &mut usize,
  tree: &mut ArenaTree<SyntaxNode>)
-> Result<(), WaxError<'a>> {

  // Check if there is whitespace after the `tmpl` keyword:
  let Some(Token::Whitespace(_)) = iter.next() else {
    return Err(WaxError::from_token(tmpl_tk.clone(), 
      "`tmpl` must be followed by whitespace", 
      WaxHint::Example("`tmpl name:`".into())
    )); 
  };

  let Some((dtk, tk)) = iter.next_de() else {
    return Err(WaxError::from_token(tmpl_tk.clone(), 
      "missing name on template definition", 
      WaxHint::Example("`tmpl name:`".into())
    )); 
  };

  // And then there should be an indentifier for the template.
  match tk {
    /* <name> */
    Token::Ident(ident) => {
      *curr = tree.add_child(*curr, ident.clone(), dtk.get_span(), SyntaxNode::Template {
        name: ident.clone()
      });
    },
    /* @ */
    Token::Atsign => {
      if let Some((dtk, tk)) = iter.next_de() {
        /* <name> */
        let Token::Ident(ident) = tk else {
          return Err(WaxError::from_token(dtk.clone(), 
            "invalid template name `@`", 
            WaxHint::Hint("you can add a name after the `@`".into())
          ));
        };

        let ident = format!("@{ident}");
        *curr = tree.add_child(*curr, ident.clone(), dtk.get_span(), SyntaxNode::Template {
          name: ident
        });
      }
    },
    _ => {
      return Err(WaxError::from_token(tmpl_tk.clone(), 
        "missing name on template definition", 
        WaxHint::Example("`tmpl name:`".into())
      )); 
    }
  }

  iter.eat_whitespace();

  // Check if there is a double dot after the name:
  let Some(Token::Colon) = iter.next() else {
    return Err(WaxError::from_token(tmpl_tk.clone(), 
      "missing `:` on template definition", 
      WaxHint::Example("`tmpl name:`".into())
    )); 
  };

  let scope = *curr;

  // Move through all tokens until we reach a semicolon:
  while let Some((dtk, tk)) = iter.next_de() {
    match tk {

      /* < */
      Token::LessThen => {
        let Some((dtk, tk)) = iter.peek_de() else {
          continue;
        };

        match tk {
          /* Opening Tag <tag> */
          Token::Ident(ident) => {
            iter.next();
            let tag = parse_attributes(ident.clone(), iter, false)?;

            let tag_idx = tree.add_child(
              *curr, 
              ident.clone(), 
              &dtk.get_span_offset(1, 1),
              tag.clone()
            );

            if let SyntaxNode::Tag { self_closing: false, .. } = tag {
              *curr = tag_idx;
            }
          }
          /* Closing Tag </tag> */
          Token::Slash => {

            let Some(Token::Ident(ident)) = iter.peek_next() else {
              iter.retreat_cursor().expect("failed to move back cursor");
              continue;
            };

            /* Void Tag */
            if is_void(ident) {
              let example = format!("void elements should only have a start tag `<{}>`", ident);
              return Err(WaxError::from_token(dtk.clone(), 
                "void element with end tag", 
                WaxHint::Example(example)
              ));
            }

            // Make sure we're closing the current scope:
            if ident != tree.get(*curr).get_name() {
              let tag = tree.get(*curr);
              let hint = format!("try closing <{}> before it's parent tag is closed", tag.get_name());
              return Err(WaxError::from_span(tag.get_span(), 
                "misnested tag", 
                WaxHint::Hint(hint)
              ));
            }

            *curr = tree.get_parent(*curr).expect("No parent");
          }
          _ => {}
        }
      }

      /* <- */
      Token::LeftArrow => {
        let Some((dtk, tk)) = iter.peek_de() else {
          continue;
        };

        match tk {
          /* Comb Opening Tag <-comb> */
          Token::Ident(ident) => {
            iter.next();
            let tag = parse_attributes(ident.clone(), iter, true)?;

            let tag_idx = tree.add_child(
              *curr, 
              ident.clone(), 
              dtk.get_span(),
              tag.clone()
            );

            if let SyntaxNode::Comb { self_closing: false, .. } = tag {
              *curr = tag_idx;
            }
          }
          /* Comb Closing Tag <-/comb> */
          Token::Slash => {

            let Some(Token::Ident(ident)) = iter.peek_next() else {
              iter.retreat_cursor().expect("failed to move back cursor");
              continue;
            };

            // Make sure we're closing the current scope:
            if ident != tree.get(*curr).get_name() {
              let tag = tree.get(*curr);
              let hint = format!("try closing <{}> before it's parent tag is closed", tag.get_name());
              return Err(WaxError::from_span(tag.get_span(), 
                "misnested tag", 
                WaxHint::Hint(hint)
              ));
            }

            *curr = tree.get_parent(*curr).expect("No parent");
          }
          _ => {}
        }
      }

      /* ; */
      Token::Semicolon => {
        if *curr == scope { break; }
        if *curr > scope {
          return Err(WaxError::from_span(tree.get(scope).get_span(), 
            "overflowing template", 
            WaxHint::Hint("make sure all tags witin the template are closed".into())
          ));
        }
      }

      /* End of File */
      Token::EOF => {
        if *curr == scope {
          return Err(WaxError::from_token(dtk.clone(), 
            "dangling template", 
            WaxHint::Hint("templates should end with a `;`".into())
          ));
        }
      }

      _ => {}
    }
  }

  // Move back out of this template.
  *curr = tree.get_parent(*curr).expect("No parent");

  Ok(())
}
