use wax_lexer::{token::SyntaxToken, span::Span};
use crate::{lines::{get_line_num, find_line_start, find_line_end}, bail};

/// Wax parser error type.
#[derive(Debug, Clone)]
pub struct WaxError<'a> {
  desc: String,
  crumbs: Option<&'a str>,
  span: Span,
  tip: Option<&'a str>,
}

impl<'a> WaxError<'a> {
  /// Generate a Wax error from a syntax token.
  pub fn from_token(token: SyntaxToken, msg: &str, tip: Option<&'a str>) -> Self {
    if let Some(tip) = tip {
      Self {
        desc: msg.to_string(),
        crumbs: None,
        span: token.get_span().clone(),
        tip: Some(tip)
      }
    } else {
      Self {
        desc: msg.to_string(),
        crumbs: None,
        span: token.get_span().clone(),
        tip: None
      }
    }
  }

  /// Print the error to the console.
  pub fn print(&self, file: &str, filename: &str) {
    let num = get_line_num(file, self.span.start_index) + 1;

    let line_start = find_line_start(file, self.span.start_index);
    let line_end = find_line_end(file, self.span.start_index);

    let line = line_start..line_end;
    
    bail(&self.desc, filename, self.crumbs, num, &file[line], self.tip);
  }
}