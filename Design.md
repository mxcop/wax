## Tokens

```perl
                     # template, implementation, style
  keyword            : tmpl,     impl,           styl


  # Systematic
  Illegal,
  EOF,

  # Generic
  Ident(String),
  Comma,       # ,
  Dot,         # .
  Colon,       # :
  Semicolon,   # ;
  Plus,        # +
  Minus,       # -
  Star,        # *
  Slash,       # /
  Bang,        # !
  Quest,       # ?

  # Script
  Let,
  Const,
  Import,
  Export,
  From,
  Function,
  True,
  False,
  If,
  Else,
  Return,

  # Template
  HashIdent(String),
  LessThen,    # <
  GreaterThen, # >
  Insert,      # <-

```