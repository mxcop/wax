// #[macro_export]
// macro_rules! info {
//   ($title:expr, $color:expr, $fmt_str:expr) => {{
//     use colored::{Colorize, Color};
//     println!("{} {} {}", "~".color(Color::BrightBlack), $title.color($color), $fmt_str);
//   }};

//   ($title:expr, $color:expr, $fmt_str:literal, $($args:expr),*) => {{
//     use colored::{Colorize, Color};
//     println!("{} {} {}", "~".color(Color::BrightBlack), $title.color($color), format!($fmt_str, $($args),*));
//   }};
// }

// #[macro_export]
// macro_rules! error {
//   ($fmt_str:expr) => {{
//     use colored::{Colorize, Color};
//     println!("{} {} {}", "~".color(Color::BrightBlack), "error! ".color(Color::Red), $fmt_str);
//   }};

//   ($fmt_str:literal, $($args:expr),*) => {{
//     use colored::{Colorize, Color};
//     println!("{} {} {}", "~".color(Color::BrightBlack), "error! ".color(Color::Red), format!($fmt_str, $($args),*));
//   }};
// }

// #[macro_export]
// macro_rules! warn {
//   ($fmt_str:expr) => {{
//     use colored::{Colorize, Color};
//     println!("{} {} {}", "~".color(Color::BrightBlack), "warn!  ".color(Color::Yellow), $fmt_str);
//   }};

//   ($fmt_str:literal, $($args:expr),*) => {{
//     use colored::{Colorize, Color};
//     println!("{} {} {}", "~".color(Color::BrightBlack), "warn!  ".color(Color::Yellow), format!($fmt_str, $($args),*));
//   }};
// }