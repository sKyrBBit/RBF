use super::{Loc, Annot};
use super::interpreter::InterpreterError;
use super::interpreter::InterpreterErrorKind::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataKind {
  Number(i32),
  Boolean(bool),
  Nil,
  Symbol(Box<str>),
  Pair(Box<Data>, Box<Data>),
}

pub type Data = Annot<DataKind>;
use DataKind::*;

impl Data {
  pub fn number(n: i32, loc: Loc) -> Self {
    Data::new (
      Number(n),
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
  pub fn pair(l: Data, r: Data, loc: Loc) -> Self {
    Data::new (
      Pair(Box::from(l), Box::from(r)),
      loc,
    )
  }
  pub fn add(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = args[0].loc;
      match (&args[0].value, &args[1].value) {
        (Number(l), Number(r)) => Ok(Self::number(l + r, loc)),
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
        (Number(l), Number(r)) => Ok(Self::number(l - r, loc)),
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
        (Number(l), Number(r)) => Ok(Self::number(l * r, loc)),
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
        (Number(l), Number(r)) => Ok(Self::number(l / r, loc)),
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
        (Number(l), Number(r)) => Ok(Self::boolean(l > r, loc)),
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
        (Number(l), Number(r)) => Ok(Self::boolean(l == r, loc)),
        (Boolean(l), Boolean(r)) => Ok(Self::boolean(l == r, loc)),
        (Nil, Nil) => Ok(Self::boolean(true, loc)),
        (Nil, _) | (_, Nil) => Ok(Self::boolean(false, loc)),
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
        (Number(l), Number(r)) => Ok(Self::boolean(l < r, loc)),
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
        (Number(l), Number(r)) => Ok(Self::number(l & r, loc)),
        (Boolean(l), Boolean(r)) => Ok(Self::boolean(l & r, loc)),
        _ => Err(Annot::new(InvalidArguments, loc)),
      }
    } else {
      Err(Annot::new(InvalidArguments, Loc(0, 1)))
    }
  }
  pub fn or(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = args[0].loc;
      match (&args[0].value, &args[1].value) {
        (Number(l), Number(r)) => Ok(Self::number(l | r, loc)),
        (Boolean(l), Boolean(r)) => Ok(Self::boolean(l | r, loc)),
        _ => Err(Annot::new(InvalidArguments, loc)),
      }
    } else {
      Err(Annot::new(InvalidArguments, Loc(0, 1)))
    }
  }
  pub fn not(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 1 {
      let loc = args[0].loc;
      match &args[0].value {
        Number(n) => Ok(Self::number(!n, loc)),
        Boolean(b) => Ok(Self::boolean(!b, loc)),
        _ => Err(Annot::new(InvalidArguments, loc)),
      }
    } else {
      Err(Annot::new(InvalidArguments, Loc(0, 1)))
    }
  }
  pub fn xor(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = args[0].loc;
      match (&args[0].value, &args[1].value) {
        (Number(l), Number(r)) => Ok(Self::number(l ^ r, loc)),
        (Boolean(l), Boolean(r)) => Ok(Self::boolean(l ^ r, loc)),
        _ => Err(Annot::new(InvalidArguments, loc)),
      }
    } else {
      Err(Annot::new(InvalidArguments, Loc(0, 1)))
    }
  }
}