pub mod node;
mod parser;
pub mod span;
pub mod tree;

use node::NodeKind;
use parser::Parser;
use tree::AST;

use waxc_errors::error::WaxError;
use waxc_lexer::token::{Token, TokenKind};

type WaxErrors = Vec<WaxError<'static>>;

/// Parse an input stream of tokens into an abstract syntax tree.
pub fn parse<'a>(
  file: String,
  mut input: impl Iterator<Item = Token> + Clone + 'a,
) -> Result<AST, WaxErrors> {
  let mut parser = Parser::new(file, &mut input);
  let errors: WaxErrors = Vec::new();

  /* Move through all the tokens */
  loop {
    if let Ok(false) = parser.advance() {
      break;
    }
    parser.reset_cursor();
  }

  match errors.len() {
    0 => Ok(parser.get_tree()),
    _ => Err(errors),
  }
}

impl<'a, I> Parser<'a, I>
where
  I: Iterator<Item = Token> + Clone + 'a,
{
  pub fn advance(&mut self) -> Result<bool, WaxError> {
    use TokenKind::*;

    // Read the next token:
    let Some(tk) = self.next() else {
      return Ok(false);
    };

    match tk.kind {
      Ident => match self.read() {
        "tmpl" => self.template()?,
        _ => (),
      },
      _ => (),
    };

    Ok(true)
  }

  fn template(&mut self) -> Result<(), WaxError> {
    /* Eat whitespace */
    self.eat_while(TokenKind::Whitespace);
    self.reset_cursor();

    /* Match the template name */
    match self.first() {
      TokenKind::Ident => (),
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
