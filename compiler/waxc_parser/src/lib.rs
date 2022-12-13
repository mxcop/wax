pub mod node;
mod parser;
//mod scopes;
pub mod span;
pub mod tree;

use node::NodeKind;
use parser::Parser;
use tree::AST;

use waxc_errors::error::WaxError;
use waxc_lexer::{token::{Token, TokenKind}, iter::LexIter};

/// Parse an input stream of tokens into an abstract syntax tree.
pub fn parse<'a>(
  file: String,
  iter: LexIter,
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

impl Parser {
  /// Parse the next token from the lexer.
  pub fn advance(&mut self) -> Result<bool, WaxError> {
    use TokenKind::*;

    // Read the next token:
    let start = self.consumed();
    let tk = self.next();

    match tk.kind {
      
      Ident => match self.read(start) {
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
    let start = self.consumed();

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
    let name = self.read(start);

    /* Create the template node */
    self.add_scope(start, NodeKind::Template {
      name: name.to_string()
    });

    self.eat_until(TokenKind::Semi);

    self.retreat_scope();

    Ok(())
  }
}

// The Wax parser.
// pub struct Parser<'a> {
//   tokens: Iter<'a, Token>
// }

// impl<'a> Parser<'a> {
//   pub fn new(tokens: Iter<Token>) -> Self {
//     Self {
//       tokens
//     }
//   }

//   #[allow(unused_variables)]
//   /// ### Syntactic Analysis
//   /// Analize the input tokens and convert it into an abstract syntax tree.
//   pub fn parse(&mut self) -> Result<ArenaTree<SyntaxNode>, WaxError> {
//     let mut tree: ArenaTree<SyntaxNode> = ArenaTree::new();
//     let mut curr = tree.add_node("Root".to_string(), Span::new(0, 0), SyntaxNode::Root);

//     while let Some((dtk, tk)) = self.iter.next_de() {
//       match tk {
//         Token::Template => {
//           tmpl::parse(&mut self.tokens, dtk, &mut curr, &mut tree)?;
//         }
//         // Token::Slash => { return Err(WaxError::from_token(dtk.clone(), "test msg", None)); }
//         _ => {}
//       }
//     }

//     Ok(tree)
//   }
// }
