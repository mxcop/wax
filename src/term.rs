#[macro_export]
macro_rules! printpro {
  ($name:expr, $color:expr, $info:expr) => {
    println!("{} {} {}", "~".color(Color::Black), $name.color($color), $info);
  };
}