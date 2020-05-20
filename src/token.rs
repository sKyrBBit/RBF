use super::{Loc, Annot};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
  Number(u32),
  Symbol(Box<str>),
  Plus,
  Minus,
  Asterisk,
  Slash,
  Less,
  Equal,
  Greater,
  And,
  Or,
  Not,
  Xor,
  Quote,
  Dot,
  LParen,
  RParen,
  // LBrace,
  // RBrace,
  // LBracket,
  // RBracket,
}

pub type Token = Annot<TokenKind>;

impl Token {
  pub fn number(n: u32, loc: Loc) -> Self {
    Self::new(TokenKind::Number(n), loc)
  }
  pub fn symbol(s: &str, loc: Loc) -> Self {
    Self::new(TokenKind::Symbol(Box::from(s)), loc)
  }
  pub fn plus(loc: Loc) -> Self {
    Self::new(TokenKind::Plus, loc)
  }
  pub fn minus(loc: Loc) -> Self {
    Self::new(TokenKind::Minus, loc)
  }
  pub fn asterisk(loc: Loc) -> Self {
    Self::new(TokenKind::Asterisk, loc)
  }
  pub fn slash(loc: Loc) -> Self {
    Self::new(TokenKind::Slash, loc)
  }
  pub fn less(loc: Loc) -> Self {
    Self::new(TokenKind::Less, loc)
  }
  pub fn equal(loc: Loc) -> Self {
    Self::new(TokenKind::Equal, loc)
  }
  pub fn greater(loc: Loc) -> Self {
    Self::new(TokenKind::Greater, loc)
  }
  pub fn and(loc: Loc) -> Self {
    Self::new(TokenKind::And, loc)
  }
  pub fn or(loc: Loc) -> Self {
    Self::new(TokenKind::Or, loc)
  }
  pub fn not(loc: Loc) -> Self {
    Self::new(TokenKind::Not, loc)
  }
  pub fn xor(loc: Loc) -> Self {
    Self::new(TokenKind::Xor, loc)
  }
  pub fn quote(loc: Loc) -> Self {
    Self::new(TokenKind::Quote, loc)
  }
  pub fn dot(loc: Loc) -> Self {
    Self::new(TokenKind::Dot, loc)
  }
  pub fn lparen(loc: Loc) -> Self {
    Self::new(TokenKind::LParen, loc)
  }
  pub fn rparen(loc: Loc) -> Self {
    Self::new(TokenKind::RParen, loc)
  }
  // pub fn lbrace(loc: Loc) -> Self {
  //   Self::new(TokenKind::LBrace, loc)
  // }
  // pub fn rbrace(loc: Loc) -> Self {
  //   Self::new(TokenKind::RBrace, loc)
  // }
  // pub fn lbracket(loc: Loc) -> Self {
  //   Self::new(TokenKind::LBracket, loc)
  // }
  // pub fn rbracket(loc: Loc) -> Self {
  //   Self::new(TokenKind::RBracket, loc)
  // }
}