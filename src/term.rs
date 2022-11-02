// #[macro_export]
// macro_rules! printpro {
//   ($name:expr, $color:expr, $info:expr) => {
//     use colored::{Colorize, Color};
//     println!("{} {} {}", "~".color(Color::Black), $name.color($color), $info);
//   };
// }

#[macro_export]
macro_rules! info {
  ($title:expr, $color:expr, $fmt_str:expr) => {{
    use colored::{Colorize, Color};
    println!("{} {} {}", "~".color(Color::Black), $title.color($color), $fmt_str);
  }};

  ($title:expr, $color:expr, $fmt_str:literal, $($args:expr),*) => {{
    use colored::{Colorize, Color};
    println!("{} {} {}", "~".color(Color::Black), $title.color($color), format!($fmt_str, $($args),*));
  }};
}

#[macro_export]
macro_rules! error {
  ($fmt_str:expr) => {{
    use colored::{Colorize, Color};
    println!("{} {} {}", "~".color(Color::Black), "error! ".color(Color::Red), $fmt_str);
  }};

  ($fmt_str:literal, $($args:expr),*) => {{
    use colored::{Colorize, Color};
    println!("{} {} {}", "~".color(Color::Black), "error! ".color(Color::Red), format!($fmt_str, $($args),*));
  }};
}

#[macro_export]
macro_rules! warn {
  ($fmt_str:expr) => {{
    use colored::{Colorize, Color};
    println!("{} {} {}", "~".color(Color::Black), "warn!  ".color(Color::Yellow), $fmt_str);
  }};

  ($fmt_str:literal, $($args:expr),*) => {{
    use colored::{Colorize, Color};
    println!("{} {} {}", "~".color(Color::Black), "warn!  ".color(Color::Yellow), format!($fmt_str, $($args),*));
  }};
}