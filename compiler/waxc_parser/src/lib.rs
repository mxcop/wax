pub mod node;
mod parser;
mod scopes;
pub mod tree;

use node::NodeKind;
use parser::Parser;
use scopes::tmpl;
use tree::AST;

use waxc_errors::error::WaxError;
use waxc_lexer::{token::{Token, TokenKind}, lexer::LexIter};

/// Parse an input stream of tokens into an abstract syntax tree.
pub fn parse<'a, I: Iterator<Item = Token> + Clone>(
  file: String,
  iter: LexIter<I>,
) -> Result<AST, WaxError> {

  let mut parser 
    = Parser::new(file, iter);

  /* Move through all the tokens */
  loop {
    if parser.advance()? {
      continue;
    }
    break;
  }

  Ok(parser.get_tree())
}

impl<I: Iterator<Item = Token> + Clone> Parser<I> {
  /// Parse the next token from the lexer.
  pub fn advance(&mut self) -> Result<bool, WaxError> {
    use TokenKind::*;
    self.update_cursor();
    
    // Read the next token:
    let Some(tk) = self.next() else {
      return Ok(false);
    };

    match tk.kind {
      
      Ident => match self.read().as_str() {
        "tmpl" => self.template()?,
        _ => (),
      },

      EOF => return Ok(false),

      _ => (),
    };

    Ok(true)
  }

  /// Check if an identity is really the start of a template.
  /// If so then start the template parser.
  fn template(&mut self) -> Result<(), WaxError> {
    /* Eat whitespace */
    self.eat_while(TokenKind::Whitespace);
    self.update_cursor();

    /* Match the template name */
    match self.first() {
      TokenKind::Ident => (),

      /* Special names */
      TokenKind::Atsign => {
        self.next();
        let TokenKind::Ident = self.first() else {
          return Ok(())
        };
      }

      _ => return Ok(())
    }

    /* Read the template name */
    self.next();
    let name = self.read();

    /* Create the template node */
    self.add_scope(NodeKind::Template {
      name: name.to_string()
    });

    tmpl::parse(self)?;

    //self.eat_until(TokenKind::Semi);
    self.retreat_scope();

    Ok(())
  }
}
