// Wax Logging Output Logic

pub mod error;
mod macros;
mod lines;
use colored::Colorize;

use lines::{add_space, usize_log10};

/// ### Formatting
/// ```
/// error: <desc>
///           --> <file> : <?crumbs>
///            |
/// <line_num> |  <line>
///            |
///            = tip: <?tip>
/// ```
pub fn bail(
  desc: &str,
  file: &str,
  crumbs: Option<&str>,
  line_num: usize,
  line: &str,
  tip: Option<&str>,
) {
  println!("");

  let level = "error".red();
  let left_margin = usize_log10(line_num) + 1;

  println!("{}", format!("{}: {}", level, desc));

  add_space(left_margin - 1);
  match crumbs {
    Some(crumbs) => println!(
      "{}",
      format!("--> {} : {}", file.italic(), crumbs.italic()).bright_black()
    ),
    None => println!("{} {}", "-->".bright_black(), file.bright_black().italic()),
  }

  add_space(left_margin);
  println!("{}", "|".bright_black());

  println!("{} {}  {}", line_num, "|".bright_black(), line);

  add_space(left_margin);
  println!("{}", "|".bright_black());

  if let Some(tip) = tip {
    add_space(left_margin);
    println!(
      "{} {}: {}",
      "=".bright_black(),
      "tip".cyan().bold(),
      tip.italic()
    );
  }

  // Exit the process:
  println!("");
  std::process::exit(0);
}

/// ### Formatting
/// ```
/// warning: <desc>
///           --> <file> : <?crumbs>
///            |
/// <line_num> |  <line>
///            |
///            = tip: <?tip>
/// ```
pub fn warn(
  desc: &str,
  file: &str,
  crumbs: Option<&str>,
  line_num: usize,
  line: &str,
  tip: Option<&str>,
) {
  println!("");

  let level = "warning".yellow();
  let left_margin = usize_log10(line_num) + 1;

  println!("{}", format!("{}: {}", level, desc).bold());

  add_space(left_margin - 1);
  match crumbs {
    Some(crumbs) => println!(
      "{}",
      format!("--> {} : {}", file.italic(), crumbs.italic()).bright_black()
    ),
    None => println!("{} {}", "-->".bright_black(), file.bright_black().italic()),
  }

  add_space(left_margin);
  println!("{}", "|".bright_black());

  println!("{} {}  {}", line_num, "|".bright_black(), line);

  add_space(left_margin);
  println!("{}", "|".bright_black());

  if let Some(tip) = tip {
    add_space(left_margin);
    println!(
      "{} {}: {}",
      "=".bright_black(),
      "tip".cyan().bold(),
      tip.italic()
    );
  }
}
