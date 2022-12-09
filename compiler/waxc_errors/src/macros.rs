#[macro_export]
macro_rules! bail {
  ($desc:expr, $file:expr, $crumbs:expr, $line_num:expr, $line:expr, $tip:expr) => {{
    wax_logger::bail($desc, $file, Some($crumbs), $line_num, $line, Some($tip));
  }};
}

#[macro_export]
macro_rules! warn {
  ($desc:expr, $file:expr, $crumbs:expr, $line_num:expr, $line:expr, $tip:expr) => {{
    wax_logger::warn($desc, $file, Some($crumbs), $line_num, $line, Some($tip));
  }};
}