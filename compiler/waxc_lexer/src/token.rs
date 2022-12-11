#[derive(Debug, Clone)]
pub struct Token {
  pub kind: TokenKind,
  len: usize,
}

impl Token {
  pub fn new(kind: TokenKind, len: usize) -> Self {
    Self {
      kind,
      len
    }
  }

  /// Get the length of the token.
  pub fn get_len(&self) -> &usize {
    &self.len
  }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
  // Systematic:
  Whitespace,
  EOF,

  LineComment,
  BlockComment,

  /// Words that start with a letter.
  Ident,

  /// Examples: `128`, `'c'`, `"string"`.
  /// See [LiteralKind] for more details.
  Literal { kind: LiteralKind },

  // Single Character Tokens:
  /**" : "*/ Colon,
  /**" ; "*/ Semi,
  /**" , "*/ Comma,
  /**" . "*/ Dot,
  /**" ( "*/ OpenParen,
  /**" ) "*/ CloseParen,
  /**" { "*/ OpenBrace,
  /**" } "*/ CloseBrace,
  /**" [ "*/ OpenBracket,
  /**" ] "*/ CloseBracket,
  /**" < "*/ Lt,
  /**" > "*/ Gt,
  /**" <-"*/ LeftArrow,
  /**"-> "*/ RightArrow,
  /**" & "*/ And,
  /**" | "*/ Or,
  /**" @ "*/ Atsign,
  /**" # "*/ Hash,
  /**" $ "*/ Dollar,
  /**" ~ "*/ Tilde,
  /**" ? "*/ Quest,
  /**" ! "*/ Bang,
  /**" + "*/ Plus,
  /**" * "*/ Star,
  /**" - "*/ Minus,
  /**" / "*/ Slash,
  /**" \ "*/ BackSlash,
  /**" = "*/ Eq,
  /**" ^ "*/ Caret,
  /**" % "*/ Percent,
  /**" " "*/ DoubleQuote,
  /**" ' "*/ Quote,
  /**" ` "*/ Grave,

  Unknown
}

#[derive(PartialEq, Debug, Clone)]
pub enum LiteralKind {
  /// "128", "12.8", ".27"
  Number,
  // ""abc"", "'abc'"
  // Str { terminated: bool },
}