use super::{Loc, Annot};
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
      Num(n) => Ok(Data::number(*n as i32, expr.loc)),
      Sym(s) => Ok(Data::symbol(s, expr.loc)),
      Nil => Ok(Data::nil(expr.loc)),
      Op { ref op } => Ok(symbolize(&op.value, expr.loc)),
      Pair  { l, r } => {
        let car = self.eval(&l)?;
        use super::data::DataKind::*;
        match car.value {
          Symbol(name) => {
            let args = vec_args(r.clone())?;
            let args = args.into_iter().map(|arg| self.eval(&arg))
              .filter(|arg| arg.is_ok())
              .map(|arg| arg.unwrap())
              .collect::<Vec<Data>>();
            match &*name {
              // builtin function
              "add"   => Data::add(args),
              "sub"   => Data::sub(args),
              "mul"   => Data::mul(args),
              "div"   => Data::div(args),
              "gt"    => Data::gt(args),
              "equal" => Data::equal(args),
              "lt"    => Data::lt(args),
              "and"   => Data::and(args),
              "or"    => Data::or(args),
              "not"   => Data::not(args),
              "xor"   => Data::xor(args),
              "atom"  => Data::atom(args),
              "car"   => Data::car(args),
              "cdr"   => Data::cdr(args),
			        "cons"  => Data::cons(args),
              // keyword
              "true"  => Ok(Data::boolean(true, expr.loc)),
              "false" => Ok(Data::boolean(false, expr.loc)),
              "nil"   => Ok(Data::nil(expr.loc)),
              _       => Err(InterpreterError::new(InterpreterErrorKind::CarNotApplicable, expr.loc)),
            }
          },
          _ => Err(InterpreterError::new(InterpreterErrorKind::CarNotApplicable, expr.loc)),
        }
      }
      Quote { q } => Ok(quote(&q, expr.loc)),
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

use super::ast::OpKind;
fn symbolize(kind: &OpKind, loc: Loc) -> Data {
  use OpKind::*;
  match kind {
    Add    => Data::symbol("add", loc),
    Sub    => Data::symbol("sub", loc),
    Mul    => Data::symbol("mul", loc),
    Div    => Data::symbol("div", loc),
    Gt     => Data::symbol("gt", loc),
    Equal  => Data::symbol("equal", loc),
    Lt     => Data::symbol("lt", loc),
    And    => Data::symbol("and", loc),
    Or     => Data::symbol("or", loc),
    Not    => Data::symbol("not", loc),
    Xor    => Data::symbol("xor", loc),
  }
}

fn quote(ast: &Ast, loc: Loc) -> Data {
  use super::ast::AstKind::*;
  match ast.value {
    Num(n)         => Data::number(n as i32, loc),
    Sym(ref s)     => Data::symbol(&*s, loc),
    Nil            => Data::nil(loc),
    Op { ref op }  => symbolize(&op.value, loc),
    Pair { ref l, ref r }  => {
      Data::pair(quote(&l, l.loc), quote(&r, r.loc), loc)
    },
    Quote { ref q } => quote(&q, q.loc),
  }
}