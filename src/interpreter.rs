use super::{Loc, Annot};
use super::ast::Ast;
use super::data::Data;
use super::symbols::Symbols;
use super::error::print_annot;

pub struct Interpreter {
  pub symbols: Symbols
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InterpreterErrorKind {
  InvalidArguments,
  DivisionByZero,
  CarNotApplicable,
  SymbolNotFound(Box<str>),
}

pub type InterpreterError = Annot<InterpreterErrorKind>;

impl Interpreter {
  pub fn new() -> Self {
    Interpreter {
      symbols: Symbols::new(),
    }
  }
  pub fn enclose(&mut self) {
    let symbols = std::mem::replace(&mut self.symbols, Symbols::new());
    self.symbols.enclose(symbols);
  }
  pub fn disclose(&mut self) -> Option<()> {
    let symbols = self.symbols.disclose();
    self.symbols = symbols?;
    Some(())
  }
  pub fn ast2data(expr: &Ast) -> Data {
    use super::ast::AstKind::*;
    match &expr.value {
      Num(n)         => Data::number(*n as i32, expr.loc),
      Sym(s)         => Data::symbol(s, expr.loc),
      Nil            => Data::nil(expr.loc),
      Op { ref op }  => symbolize(&op.value, expr.loc),
      Pair  { l, r } => Data::pair(Self::ast2data(l), Self::ast2data(r), expr.loc),
      Quote { q }    => Data::pair(Data::symbol(&Box::from("quote"), expr.loc), Self::ast2data(q), expr.loc),
    }
  }

  pub fn eval(&mut self, expr: &Data) -> Result<Data, InterpreterError> {
    use super::data::DataKind::*;
    match &expr.value {
      Number(_) | Boolean(_) => Ok(expr.clone()),
      Symbol(s)  => {
        match &**s {
          // keyword
          "true"  => Ok(Data::boolean(true, expr.loc)),
          "false" => Ok(Data::boolean(false, expr.loc)),
          "nil"   => Ok(Data::nil(expr.loc)),
          _       => self.symbols
                       .get(s)
                       .ok_or(InterpreterError::symbol_not_found(s.clone(), expr.loc))
                       .map(|d| d.clone())
        }
      },
      Nil => Ok(Data::nil(expr.loc)),
      Pair(l, r) => {
        match &l.value {
          Symbol(s) => {
            match &**s {
              // special form
              "quote"  => return Ok(*r.clone()),
              "lambda" => return Data::lambda(vec_args(r.clone())?.into_iter().map(|param| *param).collect::<Vec<Data>>()),
              "define" => return Data::define(self, vec_args(r.clone())?.into_iter().map(|param| *param).collect::<Vec<Data>>()),
              _ => (),
            }
            let args = vec_args(r.clone())?;
            let args = args.into_iter()
              .map(|arg| self.eval(&arg))
              .collect::<Result<Vec<Data>, InterpreterError>>()?;
            match &**s {
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
              "shl"    => Data::shl(args),
              "shr"    => Data::shr(args),
              "atom"   => Data::atom(args),
              "if"     => Data::_if(args),
              // utility function
              "car"    => Data::car(args),
              "cdr"    => Data::cdr(args),
              "cons"   => Data::cons(args),
              _        => {
                let body = self.symbols
                  .get(&s)
                  .ok_or(InterpreterError::car_not_applicable(expr.loc))?.clone();
                Data::apply(self, body, args)
              },
            }
          },
          _ => Err(InterpreterError::car_not_applicable(l.loc)),
        }
      },
      _ => unreachable!(),
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
  pub fn symbol_not_found(s: Box<str>, loc: Loc) -> Self {
    Self::new(InterpreterErrorKind::SymbolNotFound(s), loc)
  }
}

fn vec_args(args: Box<Data>) -> Result<Vec<Box<Data>>, InterpreterError>{
  let mut args = args;
  let mut vec_args = Vec::with_capacity(4);
  use super::data::DataKind::*;
  loop {
    match args.value {
      Pair(l, r) => {
        vec_args.push(l);
        args = r;
      }
      Nil => {
        return Ok(vec_args);
      },
      _ => unreachable!(),
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