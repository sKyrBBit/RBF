use super::{Annot, Loc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OpKind {
  Add,
  Sub,
  Mul,
  Div,
  Lt,
  Equal,
  Gt,
  And,
  Or,
}

pub type Op = Annot<OpKind>;

impl Op {
  pub fn add(loc: Loc) -> Self {
    Self::new(OpKind::Add, loc)
  }
  pub fn sub(loc: Loc) -> Self {
    Self::new(OpKind::Sub, loc)
  }
  pub fn mul(loc: Loc) -> Self {
    Self::new(OpKind::Mul, loc)
  }
  pub fn div(loc: Loc) -> Self {
    Self::new(OpKind::Div, loc)
  }
  pub fn lt(loc: Loc) -> Self {
    Self::new(OpKind::Lt, loc)
  }
  pub fn equal(loc: Loc) -> Self {
    Self::new(OpKind::Equal, loc)
  }
  pub fn gt(loc: Loc) -> Self {
    Self::new(OpKind::Gt, loc)
  }
  pub fn and(loc: Loc) -> Self {
    Self::new(OpKind::And, loc)
  }
  pub fn or(loc: Loc) -> Self {
    Self::new(OpKind::Or, loc)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstKind {
  Num(u32),
  Op    { op: Op },
  Nil,
  Pair  { l: Box<Ast>, r: Box<Ast> },
  Quote { q: Box<Ast> },
}

pub type Ast = Annot<AstKind>;

impl Ast {
  pub fn num(n: u32, loc: Loc) -> Self {
    Self::new(AstKind::Num(n), loc)
  }

  pub fn op(op: Op, loc: Loc) -> Self {
    Self::new(
      AstKind::Op {
        op: op
      },
      loc,
    )
  }

  pub fn nil(loc: Loc) -> Self {
    Self::new(
      AstKind::Nil,
      loc,
    )
  }

  pub fn pair(l: Ast, r: Ast, loc: Loc) -> Self {
    Self::new(
      AstKind::Pair {
        l: Box::new(l),
        r: Box::new(r),
      },
      loc,
    )
  }

  pub fn quote(q: Ast, loc: Loc) -> Self {
    Self::new(
      AstKind::Quote {
        q: Box::new(q),
      },
      loc,
    )
  }
}