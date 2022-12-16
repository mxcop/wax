use colored::Colorize;
use waxc_lexer::token::Token;
use crate::lines::{add_space, usize_log10, get_lines, get_char_num};

/// Wax parser tip.
#[derive(Debug, Clone)]
pub enum WaxHint {
  None,
  Example(String),
  Hint(String)
}

/// Wax parser error.
#[derive(Debug, Clone)]
pub struct WaxError {
  pos: usize,
  len: usize,
  desc: String,
  crumbs: Option<String>,
  hint: WaxHint,
}

impl WaxError {
  /// Generate a Wax error from a syntax token.
  pub fn from_token(pos: usize, token: Token, msg: &str, hint: WaxHint) -> Self {
    Self {
      pos,
      len: *token.len(),
      desc: msg.to_string(),
      crumbs: None,
      hint
    }
  }

  pub fn new(pos: usize, len: usize, msg: &str, hint: WaxHint) -> Self {
    Self {
      pos,
      len,
      desc: msg.to_string(),
      crumbs: None,
      hint
    }
  }

  /// Print the error to the console.
  pub fn print(&self, file: &str, filename: &str) {

    // Get the line information:
    let (line_num, lines) = get_lines(file, self.pos);

    // Setup:
    let level = "error".red();
    let left_margin = usize_log10(line_num) + 1;

    // Error description:
    println!("\n{}", format!("{}: {}", level, self.desc));

    // Error location:
    add_space(left_margin - 1);
    match &self.crumbs {
      Some(crumbs) => println!(
        "{}",
        format!("--> {} : {}", filename.italic(), crumbs.italic()).bright_black()
      ),
      None => println!("{} {}", "-->".bright_black(), filename.bright_black().italic()),
    }

    // Error context:
    add_space(left_margin);
    println!("{}", ":".bright_black());

    if line_num > 2 { 
      println!("{} {}  {}", 
        (line_num - 2).to_string().bright_black(), 
        "|".bright_black(), 
        lines[0].bright_black()
      ); 
    }

    if line_num > 1 { 
      println!("{} {}  {}", 
        (line_num - 1).to_string().bright_black(), 
        "|".bright_black(), 
        lines[1].bright_black()
      ); 
    }

    println!("{}{}  {}", line_num.to_string().yellow(), "->".yellow(), lines[2]);

    if self.len > 0 {
      // Error pointer:
      add_space(left_margin);
      print!("{}", "|".bright_black());
      add_space(get_char_num(file, self.pos + 2));
      for _ in 0..self.len {
        print!("{}", "^".bright_yellow());
      }
      print!("\n");
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