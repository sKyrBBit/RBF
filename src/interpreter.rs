use super::{Loc, Annot};
use super::ast::Ast;
use super::data::Data;
use super::error::print_annot;
use std::collections::HashMap;

pub struct Interpreter {
  pub symbols: HashMap<Box<str>, Data>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InterpreterErrorKind {
  InvalidArguments,
  DivisionByZero,
  CarNotApplicable,
  SymbolNotFound,
}

pub type InterpreterError = Annot<InterpreterErrorKind>;

impl Interpreter {
  pub fn new() -> Self {
    Interpreter {
      symbols: HashMap::with_capacity(32),
    }
  }

  pub fn eval(&mut self, expr: &Ast) -> Result<Data, InterpreterError> {
    use super::ast::AstKind::*;
    match &expr.value {
      Num(n) => Ok(Data::number(*n as i32, expr.loc)),
      Sym(s) => {
        match &**s {
          // keyword
          "true"  => Ok(Data::boolean(true, expr.loc)),
          "false" => Ok(Data::boolean(false, expr.loc)),
          "nil"   => Ok(Data::nil(expr.loc)),
          _       => Ok(self.symbols
                       .get(s)
                       .map(|d| d.clone())
                       .unwrap_or(Data::symbol(s, expr.loc))),
        }
      },
      Nil => Ok(Data::nil(expr.loc)),
      Op { ref op } => Ok(symbolize(&op.value, expr.loc)),
      Pair  { l, r } => {
        let car = self.eval(&l)?;
        use super::data::DataKind::*;
        match car.value {
          Symbol(s) => {
            let args = vec_args(r.clone())?;
            let args = args.into_iter().map(|arg| self.eval(&arg))
              .filter(|arg| arg.is_ok())
              .map(|arg| arg.unwrap())
              .collect::<Vec<Data>>();
            match &*s {
              // builtin function
              "add"    => Data::add(args),
              "sub"    => Data::sub(args),
              "mul"    => Data::mul(args),
              "div"    => Data::div(args),
              "rem"    => Data::rem(args),
              "gt"     => Data::gt(args),
              "ge"     => Data::ge(args),
              "eq"     => Data::eq(args),
              "ne"     => Data::ne(args),
              "lt"     => Data::lt(args),
              "le"     => Data::le(args),
              "and"    => Data::and(args),
              "or"     => Data::or(args),
              "not"    => Data::not(args),
              "xor"    => Data::xor(args),
              "atom"   => Data::atom(args),
              "if"     => Data::_if(args),
              // utility function
              "car"    => Data::car(args),
              "cdr"    => Data::cdr(args),
              "cons"   => Data::cons(args),
              // special form
              "lambda" => Data::lambda(args),
              "define" => Data::define(self, args),
              _        => Err(InterpreterError::car_not_applicable(expr.loc)),
            }
          },
          _ => Err(InterpreterError::car_not_applicable(expr.loc)),
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
  pub fn invalid_arguments(loc: Loc) -> Self {
    Self::new(InterpreterErrorKind::InvalidArguments, loc)
  }
  pub fn division_by_zero(loc: Loc) -> Self {
    Self::new(InterpreterErrorKind::DivisionByZero, loc)
  }
  pub fn car_not_applicable(loc: Loc) -> Self {
    Self::new(InterpreterErrorKind::CarNotApplicable, loc)
  }
  pub fn symbol_not_found(loc: Loc) -> Self {
    Self::new(InterpreterErrorKind::SymbolNotFound, loc)
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
    Rem    => Data::symbol("rem", loc),
    Gt     => Data::symbol("gt", loc),
    Equal  => Data::symbol("eq", loc),
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