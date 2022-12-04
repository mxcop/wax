## Tokens

```perl
                     # template, implementation, style
  keyword            : tmpl,     impl,           styl


  # Systematic
  Illegal,
  EOF,

  # Generic
  Ident(String),       # ([a-zA-Z][a-zA-Z0-9_-]+)
  Comma,               # ,
  Dot,                 # .
  SingleQuote,         # '
  DoubleQuote,         # "
  Grave,               # `
  Colon,               # :
  Semicolon,           # ;
  Plus,                # +
  Minus,               # -
  Equals,              # =
  Star,                # *
  Hash,                # #
  Percent,             # %
  Ampersand,           # &
  Atsign,              # @
  Dollarsign,          # $
  Tilde,               # ~
  Slash,               # /
  Bang,                # !
  Quest,               # ?
  LessThen,            # <
  GreaterThen,         # >
  LeftArrow,           # <-
  RightArrow,          # ->
  LeftParenthesis,     # (
  RightParenthesis,    # )
  LeftCurlyBracket,    # {
  RightCurlyBracket,   # }
  LeftSquareBracket,   # [
  RightSquareBracket,  # ]

  # Special Keywords
  Template,            # tmpl
  Implementation,      # impl
  Stylesheet,          # styl

  # Keywords
  Let,                 # let
  Const,               # const
  Import,              # import
  Export,              # export
  From,                # from
  Function,            # function
  True,                # true
  False,               # false
  If,                  # if
  Else,                # else
  Return,              # return

```