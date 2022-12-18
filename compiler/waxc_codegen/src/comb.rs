pub struct WaxComb {
  html: String,
  js: String,
  css: String,
}

impl WaxComb {
  pub fn new(html: String, js: String, css: String) -> Self {
    Self {
      html, js, css
    }
  }
}

impl std::fmt::Display for WaxComb
{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    use colored::Colorize;

    writeln!(f, "{}\n", "--- start comb ---".bright_black())?;

    /* html */
    writeln!(f, "{}: {{", "html".red())?;

    writeln!(f, "  {}", self.html)?;

    writeln!(f, "}}\n")?;

    /* js */
    writeln!(f, "{}: {{", "js".yellow())?;

    writeln!(f, "  {}", self.js)?;

    writeln!(f, "}}\n")?;

    /* css */
    writeln!(f, "{}: {{", "css".blue())?;

    writeln!(f, "  {}", self.css)?;

    writeln!(f, "}}")?;

    writeln!(f, "\n{}", "---- end comb ----".bright_black())
  }
}