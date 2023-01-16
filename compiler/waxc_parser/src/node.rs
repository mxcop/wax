/// Abstract syntax tree node
#[derive(Debug, Clone)]
pub struct Node {
  /* Data */
  pub kind: NodeKind,
  span: Span,

  /* Context */
  pub idx: usize,
  pub parent: Option<usize>,
  pub children: Vec<usize>,
}

impl Node {
  pub fn new(idx: usize, span: Span, kind: NodeKind) -> Self {
    Self {
      /* Data */
      kind,
      span,

      /* Context */
      idx,
      parent: None,
      children: vec![],
    }
  }

  /// Get a reference to the name of the node.
  /// [Debug] This is for debugging purposes!
  pub fn get_name(&self) -> String {
    match &self.kind {
      NodeKind::Tag { name, .. } => name.clone(),
      NodeKind::Comb { name, .. } => name.clone(),
      _ => format!("{:?}", &self)
    }
  }

  /// Get a reference to the span of the node.
  pub fn get_span(&self) -> &Span {
    &self.span
  }
}

#[derive(Debug, Clone)]
pub struct Attribute {
  pub name: String,
  pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub enum NodeKind {
  Root,

  /** `use "path/to/file";` */
  Using {
    path: String,
  },
  /** `tmpl <name>: ;` */
  Template {
    name: String,
  },
  /** `impl <name>() {}` */
  Implementation {
    name: String,
  },
  /** `styl <name> {}` */
  Stylesheet {
    name: String,
  },
  /** `<tag> ... </tag>` */
  Tag {
    name: String,
    attributes: Vec<Attribute>,
    self_closing: bool,
  }, 
  /** `<-comb> ... </comb>` */
  Comb {
    name: String,
    attributes: Vec<Attribute>,
    self_closing: bool,
  }, 
  Text(String),
}

/// Spanner indicating the position and length of a [Node].
#[derive(Debug, Clone, Copy)]
pub struct Span {
  pub pos: usize,
  pub len: usize,
}

impl Span {
  pub fn new(pos: usize, len: usize) -> Self {
    Self { pos, len }
  }
}
