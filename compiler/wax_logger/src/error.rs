use colored::Colorize;
use wax_lexer::{token::SyntaxToken, span::Span};
use crate::lines::{add_space, usize_log10, get_lines};

/// Wax parser tip.
#[derive(Debug, Clone)]
pub enum WaxHint<'a> {
  None,
  Example(&'a str),
  Hint(String)
}

/// Wax parser error.
#[derive(Debug, Clone)]
pub struct WaxError<'a> {
  desc: String,
  crumbs: Option<&'a str>,
  span: Span,
  hint: WaxHint<'a>,
}

impl<'a> WaxError<'a> {
  /// Generate a Wax error from a syntax token.
  pub fn from_token(token: SyntaxToken, msg: &str, hint: WaxHint<'a>) -> Self {
    Self {
      desc: msg.to_string(),
      crumbs: None,
      span: token.get_span().clone(),
      hint
    }
  }

  /// Generate a Wax error from a span.
  pub fn from_span(span: Span, msg: &str, hint: WaxHint<'a>) -> Self {
    Self {
      desc: msg.to_string(),
      crumbs: None,
      span,
      hint
    }
  }

  /// Print the error to the console.
  pub fn print(&self, file: &str, filename: &str) {

    // Get the line information:
    let (line_num, lines) = get_lines(file, self.span.start_index);

    // Setup:
    let level = "error".red();
    let left_margin = usize_log10(line_num) + 1;

    // Error description:
    println!("\n{}", format!("{}: {}", level, self.desc));

    // Error location:
    add_space(left_margin - 1);
    match self.crumbs {
      Some(crumbs) => println!(
        "{}",
        format!("--> {} : {}", filename.italic(), crumbs.italic()).bright_black()
      ),
      None => println!("{} {}", "-->".bright_black(), filename.bright_black().italic()),
    }

    // Error context:
    add_space(left_margin);
    println!("{}", ":".bright_black());

    if line_num > 1 { 
      println!("{} {}  {}", 
        (line_num - 1).to_string().bright_black(), 
        "|".bright_black(), 
        lines[0].bright_black()
      ); 
    }

    println!("{}{}  {}", line_num.to_string().yellow(), "->".yellow(), lines[1]);

    if lines[2] != "" { 
      println!("{} {}  {}", 
        (line_num + 1).to_string().bright_black(), 
        "|".bright_black(), 
        lines[2].bright_black()
      ); 
    }

    add_space(left_margin);
    println!("{}", ":".bright_black());

    // Error hint:
    match &self.hint {

      WaxHint::Example(txt) => {
        add_space(left_margin);
        println!(
          "{} {}: {}",
          "+".bright_black(),
          "example".yellow(),
          txt.italic()
        );
      },
      WaxHint::Hint(txt) => {
        add_space(left_margin);
        println!(
          "{} {}: {}",
          "=".bright_black(),
          "hint".cyan(),
          txt.italic()
        );
      },
      
      _ => {}
    }

    // Exit the process:
    println!("");
    std::process::exit(0);
  }
}