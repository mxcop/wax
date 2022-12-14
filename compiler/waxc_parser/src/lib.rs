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
        "styl" => self.stylesheet()?,
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
    let Some(name) = self.declaration()? else {
      // todo: throw error that styl must be followed by a name.
      todo!();
    };

    /* Create the template node */
    self.add_scope(NodeKind::Template {
      name: name.to_string()
    });

    tmpl::parse(self)?;
    self.retreat_scope();

    Ok(())
  }

  fn stylesheet(&mut self) -> Result<(), WaxError> {
    let Some(name) = self.declaration()? else {
      // todo: throw error that styl must be followed by a name.
      todo!();
    };

    /* Create the stylesheet node */
    self.add_scope(NodeKind::Stylesheet {
      name: name.to_string()
    });

    /* Eat until we reach the end of the stylesheet */
    self.eat_until(TokenKind::OpenBrace);
    self.update_cursor();
    self.eat_scope(TokenKind::OpenBrace, TokenKind::CloseBrace)?;

    /* Read contents and remove the outer braces */
    let mut content = self.read().trim_matches(|c| c == '{' || c == '}').to_string();
    content.retain(|c| !c.is_whitespace());

    /* Put the contents into a text node and retreat */
    self.add_node(NodeKind::Text(content));
    self.retreat_scope();

    Ok(())
  }

  /// Try to parse a declaration, e.g. `tmpl, styl, impl`
  fn declaration(&mut self) -> Result<Option<String>, WaxError> {
    /* Eat whitespace */
    self.eat_while(TokenKind::Whitespace);
    self.update_cursor();

    /* Match the declared name */
    match self.first() {
      TokenKind::Ident => (),

      /* Special names */
      TokenKind::Atsign => {
        self.next();
        let TokenKind::Ident = self.first() else {
          // todo: throw error that atsign must be followed by a name.
          todo!();
        };
      }

      _ => return Ok(None)
    }

    /* Read the declared name */
    self.next();
    Ok(Some(self.read()))
  }
}
