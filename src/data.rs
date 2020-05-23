use super::{Loc, Annot};
use super::interpreter::{Interpreter, InterpreterError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataKind {
  Number(i32),
  Boolean(bool),
  Nil,
  Symbol(Box<str>),
  Pair(Box<Data>, Box<Data>),
  _Fn(Vec<Box<str>>, Box<Data>),
}

pub type Data = Annot<DataKind>;
use DataKind::*;

impl Data {
  pub(crate) fn number(n: i32, loc: Loc) -> Self {
    Data::new (
      Number(n),
      loc,
    )
  }
  pub(crate) fn boolean(b: bool, loc: Loc) -> Self {
    Data::new (
      Boolean(b),
      loc,
    )
  }
  pub(crate) fn nil(loc: Loc) -> Self {
    Data::new (
      Nil,
      loc,
    )
  }
  pub(crate) fn symbol(name: &str, loc: Loc) -> Self {
    Data::new (
      Symbol(Box::from(name)),
      loc,
    )
  }
  pub(crate) fn pair(l: Data, r: Data, loc: Loc) -> Self {
    Data::new (
      Pair(Box::from(l), Box::from(r)),
      loc,
    )
  }
  pub(crate) fn _fn(args: Vec<Box<str>>, body: Data, loc: Loc) -> Self {
    Data::new (
      _Fn(args, Box::from(body)),
      loc,
    )
  }

  pub(crate) fn add(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = Loc::merge(&args[0].loc, &args[1].loc);
      match (&args[0].value, &args[1].value) {
        (Number(l), Number(r)) => Ok(Self::number(l + r, loc)),
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn sub(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = Loc::merge(&args[0].loc, &args[1].loc);
      match (&args[0].value, &args[1].value) {
        (Number(l), Number(r)) => Ok(Self::number(l - r, loc)),
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn mul(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = Loc::merge(&args[0].loc, &args[1].loc);
      match (&args[0].value, &args[1].value) {
        (Number(l), Number(r)) => Ok(Self::number(l * r, loc)),
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn div(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = Loc::merge(&args[0].loc, &args[1].loc);
      match (&args[0].value, &args[1].value) {
        (Number(l), Number(r)) => {
          if *r == 0 {
            Err(InterpreterError::division_by_zero(loc))
          } else {
            Ok(Self::number(l / r, loc))
          }
        },
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn rem(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = Loc::merge(&args[0].loc, &args[1].loc);
      match (&args[0].value, &args[1].value) {
        (Number(l), Number(r)) => {
          if *r == 0 {
            Err(InterpreterError::division_by_zero(loc))
          } else {
            Ok(Self::number(l % r, loc))
          }
        },
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn gt(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = Loc::merge(&args[0].loc, &args[1].loc);
      match (&args[0].value, &args[1].value) {
        (Number(l), Number(r)) => Ok(Self::boolean(l > r, loc)),
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn ge(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = Loc::merge(&args[0].loc, &args[1].loc);
      match (&args[0].value, &args[1].value) {
        (Number(l), Number(r)) => Ok(Self::boolean(l >= r, loc)),
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn eq(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = Loc::merge(&args[0].loc, &args[1].loc);
      match (&args[0].value, &args[1].value) {
        (Number(l), Number(r)) => Ok(Self::boolean(l == r, loc)),
        (Boolean(l), Boolean(r)) => Ok(Self::boolean(l == r, loc)),
        (Nil, Nil) => Ok(Self::boolean(true, loc)),
        (Nil, _) | (_, Nil) => Ok(Self::boolean(false, loc)),
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn ne(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = Loc::merge(&args[0].loc, &args[1].loc);
      match (&args[0].value, &args[1].value) {
        (Number(l), Number(r)) => Ok(Self::boolean(l != r, loc)),
        (Boolean(l), Boolean(r)) => Ok(Self::boolean(l != r, loc)),
        (Nil, Nil) => Ok(Self::boolean(false, loc)),
        (Nil, _) | (_, Nil) => Ok(Self::boolean(true, loc)),
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn lt(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = Loc::merge(&args[0].loc, &args[1].loc);
      match (&args[0].value, &args[1].value) {
        (Number(l), Number(r)) => Ok(Self::boolean(l < r, loc)),
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn le(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = Loc::merge(&args[0].loc, &args[1].loc);
      match (&args[0].value, &args[1].value) {
        (Number(l), Number(r)) => Ok(Self::boolean(l <= r, loc)),
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn and(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = Loc::merge(&args[0].loc, &args[1].loc);
      match (&args[0].value, &args[1].value) {
        (Number(l), Number(r)) => Ok(Self::number(l & r, loc)),
        (Boolean(l), Boolean(r)) => Ok(Self::boolean(l & r, loc)),
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn or(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = Loc::merge(&args[0].loc, &args[1].loc);
      match (&args[0].value, &args[1].value) {
        (Number(l), Number(r)) => Ok(Self::number(l | r, loc)),
        (Boolean(l), Boolean(r)) => Ok(Self::boolean(l | r, loc)),
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn not(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 1 {
      let loc = args[0].loc;
      match &args[0].value {
        Number(n) => Ok(Self::number(!n, loc)),
        Boolean(b) => Ok(Self::boolean(!b, loc)),
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn xor(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = Loc::merge(&args[0].loc, &args[1].loc);
      match (&args[0].value, &args[1].value) {
        (Number(l), Number(r)) => Ok(Self::number(l ^ r, loc)),
        (Boolean(l), Boolean(r)) => Ok(Self::boolean(l ^ r, loc)),
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn shl(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = Loc::merge(&args[0].loc, &args[1].loc);
      match (&args[0].value, &args[1].value) {
        (Number(l), Number(r)) => Ok(Self::number(l << r, loc)),
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn shr(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = Loc::merge(&args[0].loc, &args[1].loc);
      match (&args[0].value, &args[1].value) {
        (Number(l), Number(r)) => Ok(Self::number(l >> r, loc)),
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn atom(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 1 {
      let loc = args[0].loc;
      match &args[0].value {
        Number(_) |
        Boolean(_) |
        Nil |
        Symbol(_) => Ok(Self::boolean(true, loc)),
        _ => Ok(Self::boolean(false, loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn car(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 1 {
      let loc = args[0].loc;
      match &args[0].value {
        Pair(l, _) => Ok((**l).clone()),
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn cdr(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 1 {
      let loc = args[0].loc;
      match &args[0].value {
        Pair(_, r) => Ok((**r).clone()),
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn cons(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 2 {
      let loc = Loc::merge(&args[0].loc, &args[1].loc);
      Ok(Data::pair(args[0].clone(), args[1].clone(), loc))
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn _if(args: Vec<Data>) -> Result<Data, InterpreterError> {
    if args.len() == 3 {
      let loc = Loc::merge(&Loc::merge(&args[0].loc, &args[1].loc), &args[2].loc);
      match (&args[0].value, &args[1].value, &args[2].value) {
        (Boolean(b), _, _) => if *b { Ok(args[1].clone()) } else { Ok(args[2].clone()) },
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn lambda(params: Vec<Data>) -> Result<Data, InterpreterError> {
    if params.len() == 2 {
      let loc = Loc::merge(&params[0].loc, &params[1].loc);
      // unfold params
      let mut args = &params[0];
      let mut vec_args = Vec::with_capacity(4);
      loop {
        match &args.value {
          Pair(l, r) => {
            if let Symbol(s) = &l.value {
              vec_args.push(s.clone());
              args = &*r;
            } else {
              return Err(InterpreterError::invalid_arguments(loc))
            }
          }
          Nil => {
            return Ok(Data::_fn(vec_args, params[1].clone(), loc));
          },
          _ => return Err(InterpreterError::invalid_arguments(Loc(0, 1)))
        }
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn define(interpreter: &mut Interpreter, params: Vec<Data>) -> Result<Data, InterpreterError> {
    if params.len() == 2 {
      let loc = Loc::merge(&params[0].loc, &params[1].loc);
      match (&params[0].value, &params[1].value) {
        (Symbol(s), _) => {
          let result = interpreter.eval(&params[1])?;
          interpreter.symbols.insert(s.clone(), result);
          Ok(Self::nil(loc))
        },
        _ => Err(InterpreterError::invalid_arguments(loc)),
      }
    } else {
      Err(InterpreterError::invalid_arguments(Loc(0, 1)))
    }
  }
  pub(crate) fn apply(interpreter: &mut Interpreter, _fn: Data, params: Vec<Data>) -> Result<Data, InterpreterError> {
    interpreter.enclose();
    let result = match _fn.value {
      _Fn(args, body) => {
        if params.len() == args.len() {
          for i in 0..params.len() {
            interpreter.symbols.insert(args[i].clone(), params[i].clone());
          }
          interpreter.eval(&*body)
        } else {
          Err(InterpreterError::invalid_arguments(Loc(0, 1)))
        }
      },
      _ => Err(InterpreterError::car_not_applicable(_fn.loc))
    };
    interpreter.disclose().unwrap();
    result
  }
  pub fn assert_eq(&self, other: DataKind) {
    assert_eq!(self.value, other)
  }
}