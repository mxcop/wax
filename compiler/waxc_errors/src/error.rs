use std::{fs::read_to_string, path::Path};

use crate::lines::{add_space, get_char_num, get_lines, usize_log10};
use colored::Colorize;
use waxc_lexer::token::Token;

/// Wax parser tip.
#[derive(Debug, Clone)]
pub enum WaxHint {
  None,
  Example(String),
  Hint(String),
}

/// Wax parser error.
#[derive(Debug, Clone)]
pub struct WaxError {
  pos: usize,
  len: usize,
  desc: String,
  /* Path of the file in which the error occurred */
  file: Option<String>,
  crumbs: Option<String>,
  hint: WaxHint,
}

impl WaxError {
  /// Generate a Wax error from a syntax token.
  pub fn from_token(pos: usize, token: Token, msg: &str, hint: WaxHint, file: Option<String>) -> Self {
    Self {
      pos,
      len: *token.len(),
      desc: msg.to_string(),
      file,
      crumbs: None,
      hint,
    }
  }

  pub fn new(pos: usize, len: usize, msg: &str, hint: WaxHint, file: Option<&Path>) -> Self {
    let file = match file {
      Some(file) => {
        Some(file.to_string_lossy().to_string())
      },
      None => None
    };
    Self {
      pos,
      len,
      desc: msg.to_string(),
      file,
      crumbs: None,
      hint,
    }
  }

  /// Print the error to the console.
  pub fn print(&self) {

    match &self.file {
      // If this error is focused on a file:
      Some(file) => {
        // Load the file information:
        let filepath = Path::new(&file);
        let Ok(file) = read_to_string(filepath) else {
          panic!("couldn't find file!");
        };
        let Some(filename) = filepath.file_name() else {
          panic!("coudln't find file name!");
        };
        let filename = filename.to_string_lossy().to_string();

        // Get the line information:
        let (line_num, lines) = get_lines(&file, self.pos);

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
          None => println!(
            "{} {}",
            "-->".bright_black(),
            filename.bright_black().italic()
          ),
        }

        // Error context:
        add_space(left_margin);
        println!("{}", ":".bright_black());

        if line_num > 2 {
          println!(
            "{} {}  {}",
            (line_num - 2).to_string().bright_black(),
            "|".bright_black(),
            lines[0].bright_black()
          );
        }

        if line_num > 1 {
          println!(
            "{} {}  {}",
            (line_num - 1).to_string().bright_black(),
            "|".bright_black(),
            lines[1].bright_black()
          );
        }

        println!(
          "{}{}  {}",
          line_num.to_string().yellow(),
          "->".yellow(),
          lines[2]
        );

        if self.len > 0 {
          // Error pointer:
          add_space(left_margin);
          print!("{}", "|".bright_black());
          add_space(get_char_num(&file, self.pos + 2));
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
          }
          WaxHint::Hint(txt) => {
            add_space(left_margin);
            println!("{} {}: {}", "=".bright_black(), "hint".cyan(), txt.italic());
          }

          _ => {}
        }
      }

      // If this error is global:
      None => {
        // Setup:
        let level = "error".red();

        // Error description:
        println!("\n{}", format!("{}: {}", level, self.desc));

        // Error hint:
        match &self.hint {
          WaxHint::Example(txt) => {
            println!(
              "{} {}: {}",
              "+".bright_black(),
              "example".yellow(),
              txt.italic()
            );
          }
          WaxHint::Hint(txt) => {
            println!("{} {}: {}", "=".bright_black(), "hint".cyan(), txt.italic());
          }

          _ => {}
        }
      }
    }

    // Exit the process:
    println!("");
    std::process::exit(0);
  }
}
