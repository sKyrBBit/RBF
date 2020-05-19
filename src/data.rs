use super::{Loc, Annot};
use super::interpreter::InterpreterError;
use super::interpreter::InterpreterErrorKind::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataKind {
  Num(i32),
  Boolean(bool),
  Nil,
  Symbol(Box<str>),
}

pub type Data = Annot<DataKind>;
use DataKind::*;

impl Data {
  pub fn num(n: i32, loc: Loc) -> Self {
    Data::new (
      Num(n),
      loc,
    )
  }
  pub fn boolean(b: bool, loc: Loc) -> Self {
    Data::new (
      Boolean(b),
      loc,
    )
  }
  pub fn nil(loc: Loc) -> Self {
    Data::new (
      Nil,
      loc,
    )
  }
  pub fn symbol(name: &str, loc: Loc) -> Self {
    Data::new (
      Symbol(Box::from(name)),
      loc,
    )
  }
  pub fn add(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = args[0].loc;
      match (&args[0].value, &args[1].value) {
        (Num(l), Num(r)) => Ok(Self::num(l + r, loc)),
        _ => Err(Annot::new(InvalidArguments, loc)),
      }
    } else {
      Err(Annot::new(InvalidArguments, Loc(0, 1)))
    }
  }
  pub fn sub(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = args[0].loc;
      match (&args[0].value, &args[1].value) {
        (Num(l), Num(r)) => Ok(Self::num(l - r, loc)),
        _ => Err(Annot::new(InvalidArguments, loc)),
      }
    } else {
      Err(Annot::new(InvalidArguments, Loc(0, 1)))
    }
  }
  pub fn mul(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = args[0].loc;
      match (&args[0].value, &args[1].value) {
        (Num(l), Num(r)) => Ok(Self::num(l * r, loc)),
        _ => Err(Annot::new(InvalidArguments, loc)),
      }
    } else {
      Err(Annot::new(InvalidArguments, Loc(0, 1)))
    }
  }
  pub fn div(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = args[0].loc;
      match (&args[0].value, &args[1].value) {
        (Num(l), Num(r)) => Ok(Self::num(l / r, loc)),
        _ => Err(Annot::new(InvalidArguments, loc)),
      }
    } else {
      Err(Annot::new(InvalidArguments, Loc(0, 1)))
    }
  }
  pub fn gt(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = args[0].loc;
      match (&args[0].value, &args[1].value) {
        (Num(l), Num(r)) => Ok(Self::boolean(l > r, loc)),
        _ => Err(Annot::new(InvalidArguments, loc)),
      }
    } else {
      Err(Annot::new(InvalidArguments, Loc(0, 1)))
    }
  }
  pub fn equal(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = args[0].loc;
      match (&args[0].value, &args[1].value) {
        (Num(l), Num(r)) => Ok(Self::boolean(l == r, loc)),
        _ => Err(Annot::new(InvalidArguments, loc)),
      }
    } else {
      Err(Annot::new(InvalidArguments, Loc(0, 1)))
    }
  }
  pub fn lt(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = args[0].loc;
      match (&args[0].value, &args[1].value) {
        (Num(l), Num(r)) => Ok(Self::boolean(l < r, loc)),
        _ => Err(Annot::new(InvalidArguments, loc)),
      }
    } else {
      Err(Annot::new(InvalidArguments, Loc(0, 1)))
    }
  }
  pub fn and(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = args[0].loc;
      match (&args[0].value, &args[1].value) {
        (Num(l), Num(r)) => Ok(Self::num(l & r, loc)),
        _ => Err(Annot::new(InvalidArguments, loc)),
      }
    } else {
      Err(Annot::new(InvalidArguments, Loc(0, 1)))
    }
  }
  pub fn or(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = args[0].loc;
      match (&args[0].value, &&args[1].value) {
        (Num(l), Num(r)) => Ok(Self::num(l | r, loc)),
        _ => Err(Annot::new(InvalidArguments, loc)),
      }
    } else {
      Err(Annot::new(InvalidArguments, Loc(0, 1)))
    }
  }
}