use super::{Annot, Loc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OpKind {
  Add,
  Sub,
  Mul,
  Div,
  Rem,
  Lt,
  Equal,
  Gt,
  And,
  Or,
  Not,
  Xor,
}

pub type Op = Annot<OpKind>;

impl Op {
  pub(crate) fn add(loc: Loc) -> Self {
    Self::new(OpKind::Add, loc)
  }
  pub(crate) fn sub(loc: Loc) -> Self {
    Self::new(OpKind::Sub, loc)
  }
  pub(crate) fn mul(loc: Loc) -> Self {
    Self::new(OpKind::Mul, loc)
  }
  pub(crate) fn div(loc: Loc) -> Self {
    Self::new(OpKind::Div, loc)
  }
  pub(crate) fn rem(loc: Loc) -> Self {
    Self::new(OpKind::Rem, loc)
  }
  pub(crate) fn lt(loc: Loc) -> Self {
    Self::new(OpKind::Lt, loc)
  }
  pub(crate) fn equal(loc: Loc) -> Self {
    Self::new(OpKind::Equal, loc)
  }
  pub(crate) fn gt(loc: Loc) -> Self {
    Self::new(OpKind::Gt, loc)
  }
  pub(crate) fn and(loc: Loc) -> Self {
    Self::new(OpKind::And, loc)
  }
  pub(crate) fn or(loc: Loc) -> Self {
    Self::new(OpKind::Or, loc)
  }
  pub(crate) fn not(loc: Loc) -> Self {
    Self::new(OpKind::Not, loc)
  }
  pub(crate) fn xor(loc: Loc) -> Self {
    Self::new(OpKind::Xor, loc)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstKind {
  Num(u32),
  Sym(Box<str>),
  Op    { op: Op },
  Nil,
  Pair  { l: Box<Ast>, r: Box<Ast> },
  Quote { q: Box<Ast> },
}

pub type Ast = Annot<AstKind>;

impl Ast {
  pub(crate) fn num(n: u32, loc: Loc) -> Self {
    Self::new(AstKind::Num(n), loc)
  }

  pub(crate) fn sym(s: Box<str>, loc: Loc) -> Self {
    Self::new(AstKind::Sym(s), loc)
  }

  pub(crate) fn op(op: Op, loc: Loc) -> Self {
    Self::new(
      AstKind::Op {
        op: op
      },
      loc,
    )
  }

  pub(crate) fn nil(loc: Loc) -> Self {
    Self::new(
      AstKind::Nil,
      loc,
    )
  }

  pub(crate) fn pair(l: Ast, r: Ast, loc: Loc) -> Self {
    Self::new(
      AstKind::Pair {
        l: Box::new(l),
        r: Box::new(r),
      },
      loc,
    )
  }

  pub(crate) fn quote(q: Ast, loc: Loc) -> Self {
    Self::new(
      AstKind::Quote {
        q: Box::new(q),
      },
      loc,
    )
  }
}