pub struct OutputPage {
  /* page parts */
  pub html: String,
  pub js: String,
  pub css: String,
  /* page path (./index.html | ./blog.html | ./shop/index.html ...) */
  pub page_path: String
}

impl OutputPage {
  pub fn new(page_path: String, html: String, js: String, css: String) -> Self {
    Self {
      html, js, css, page_path
    }
  }
}