use super::Annot;
use super::ast::Ast;
use super::data::Data;
use super::error::print_annot;

pub struct Interpreter;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InterpreterErrorKind {
  InvalidArguments,
  DivisionByZero,
  CarNotApplicable,
}

pub type InterpreterError = Annot<InterpreterErrorKind>;

impl Interpreter {
  pub fn new() -> Self {
    Interpreter
  }

  pub fn eval(&mut self, expr: &Ast) -> Result<Data, InterpreterError> {
    use super::ast::AstKind::*;
    match &expr.value {
      Num(n) => Ok(Data::num(*n as i32, expr.loc)),
      Nil => Ok(Data::nil(expr.loc)),
      Op { ref op } => {
        use super::ast::OpKind::*;
        match op.value {
          Add    => Ok(Data::symbol("add", expr.loc)),
          Sub    => Ok(Data::symbol("sub", expr.loc)),
          Mul    => Ok(Data::symbol("mul", expr.loc)),
          Div    => Ok(Data::symbol("div", expr.loc)),
          Gt     => Ok(Data::symbol("gt", expr.loc)),
          Equal  => Ok(Data::symbol("equal", expr.loc)),
          Lt     => Ok(Data::symbol("lt", expr.loc)),
          And    => Ok(Data::symbol("and", expr.loc)),
          Or     => Ok(Data::symbol("or", expr.loc)),
        }
      },
      Pair  { l, r } => {
        let car = self.eval(&l)?;
        use super::data::DataKind::*;
        match car.value {
          Symbol(name) => {
            let args = vec_args(r.clone())?;
            let args = args.into_iter().map(|arg| self.eval(&*arg))
              .filter(|arg| arg.is_ok())
              .map(|arg| arg.unwrap())
              .collect::<Vec<Data>>();
            match &*name {
              "add"   => Data::add(args),
              "sub"   => Data::sub(args),
              "mul"   => Data::mul(args),
              "div"   => Data::div(args),
              "gt"    => Data::gt(args), 
              "equal" => Data::equal(args),
              "lt"    => Data::lt(args), 
              "and"   => Data::and(args),
              "or"    => Data::or(args),
              _       => Err(InterpreterError::new(InterpreterErrorKind::CarNotApplicable, expr.loc)),
            }
          },
          _ => Err(InterpreterError::new(InterpreterErrorKind::CarNotApplicable, expr.loc)),
        }
      }
  	  Quote { q } => {
        match q.value {
          Num(n)                 => Ok(Data::num(n as i32, expr.loc)),
          Nil                    => Ok(Data::nil(expr.loc)),
          Op { ref op }          => Ok(Data::symbol(&format!("{:?}", op), expr.loc)),
          Pair { ref l, ref r }  => unimplemented!(),
          Quote { ref q }        => Ok(self.eval(q)?),
        }
      },
	  }
  }
}

impl InterpreterError {
  pub fn show_diagnostic(&self, input: &str) {
    eprintln!("{}", self);
    print_annot(input, self.loc);
  }
}

fn vec_args(args: Box<Ast>) -> Result<Vec<Box<Ast>>, InterpreterError>{
  let mut args = args;
  let mut vec_args = Vec::with_capacity(4);
  use super::ast::AstKind::*;
  loop {
    match args.value {
      Pair { l, r } => {
        vec_args.push(l);
        args = r;
      }
      _ => {
        return Ok(vec_args);
      },
    }
  }
}