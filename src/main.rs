extern crate rlisp;
use rlisp::ast::Ast;
use rlisp::interpreter::Interpreter;
use rlisp::error::show_trace;
use std::io;

fn prompt(s: &str) -> io::Result<()> {
  use io::{stdout, Write};
  let stdout = stdout();
  let mut stdout = stdout.lock();
  stdout.write(s.as_bytes())?;
  stdout.flush()
}

fn interpret(input: &str, interpreter: &mut Interpreter) {
  let ast = match input.parse::<Ast>() {
    Ok(ast) => ast,
    Err(e) => {
      e.show_diagnostic(&input);
      show_trace(e);
      return
    }
  };
  println!("{:?}", ast);
  match interpreter.eval(&Interpreter::ast2data(&ast)) {
    Ok(result) => println!("{}", result),
    Err(e) => {
      e.show_diagnostic(&input);
      show_trace(e);
    }
  };
}

fn main() -> io::Result<()> {
  use io::{stdin, BufRead, BufReader};
  let mut interpreter = Interpreter::new();

  let stdin = stdin();
  let stdin = stdin.lock();
  let stdin = BufReader::new(stdin);
  let mut lines = stdin.lines();

  loop {
    prompt("> ")?;
    if let Some(Ok(line)) = lines.next() {
      if line == "exit" { break }
      interpret(&line, &mut interpreter);
    } else {
      break;
    }
  };
  Ok(())
}

#[cfg(test)]
mod tests {
use rlisp::ast::Ast;
use rlisp::data::{DataKind, DataKind::*};
use rlisp::interpreter::Interpreter;

const ONE: DataKind = Number(1);
const TWO: DataKind = Number(2);
const TRUE: DataKind = Boolean(true);
const FALSE: DataKind = Boolean(false);
const NIL: DataKind = Nil;

  fn interpret(line: &str, interpreter: &mut Interpreter) {
    let ast = line.parse::<Ast>().unwrap();
    println!("{:?}", ast);
    let result = interpreter.eval(&Interpreter::ast2data(&ast)).unwrap();
    println!("{}", result);
  }
  fn _interpret(line: &str, normal: DataKind, interpreter: &mut Interpreter) {
    let ast = line.parse::<Ast>().unwrap();
    println!("{:?}", ast);
    let result = interpreter.eval(&Interpreter::ast2data(&ast)).unwrap();
    println!("{}", result);
    result.assert_eq(normal)
  }
  #[should_panic]
  #[test]
  fn empty() {
    let mut interpreter = Interpreter::new();
    interpret("", &mut interpreter);
  }
  #[test]
  fn atom() {
    let mut interpreter = Interpreter::new();
    _interpret("1", ONE, &mut interpreter);
    _interpret("true", TRUE, &mut interpreter);
    _interpret("'a", Symbol(Box::from("a")), &mut interpreter);
    _interpret("'&", Symbol(Box::from("and")), &mut interpreter);
    _interpret("(atom 1)", TRUE, &mut interpreter);
    _interpret("(atom true)", TRUE, &mut interpreter);
    _interpret("(atom 'a)", TRUE, &mut interpreter);
    _interpret("(atom '&)", TRUE, &mut interpreter);
  }
  #[test]
  fn quote() {
    let mut interpreter = Interpreter::new();
    _interpret("'1", ONE, &mut interpreter);
    _interpret("'true", Symbol(Box::from("true")), &mut interpreter);
  }
  #[test]
  fn pair() {
    let mut interpreter = Interpreter::new();
    interpret("'(1 . 2)", &mut interpreter);
    interpret("'(true . false)", &mut interpreter);
    interpret("'(a . b)", &mut interpreter);
    interpret("'(& . |)", &mut interpreter);
    _interpret("(car '(1 . 2))", ONE, &mut interpreter);
    _interpret("(car '(true . false))", Symbol(Box::from("true")), &mut interpreter);
    _interpret("(car '(a . b))", Symbol(Box::from("a")), &mut interpreter);
    _interpret("(car '(& . |))", Symbol(Box::from("and")),  &mut interpreter);
    _interpret("(cdr '(1 . 2))", TWO, &mut interpreter);
    _interpret("(cdr '(true . false))", Symbol(Box::from("false")), &mut interpreter);
    _interpret("(cdr '(a . b))", Symbol(Box::from("b")), &mut interpreter);
    _interpret("(cdr '(& . |))", Symbol(Box::from("or")), &mut interpreter);
    interpret("(cons 1 2)", &mut interpreter);
    interpret("(cons true false)", &mut interpreter);
    interpret("(cons 'a 'b)", &mut interpreter);
    interpret("(cons '& '|)", &mut interpreter);
  }
  #[test]
  fn list() {
    let mut interpreter = Interpreter::new();
    interpret("'(1 2 3)", &mut interpreter);
    interpret("'(true false)", &mut interpreter);
    interpret("'('a 'b 'c 'd)", &mut interpreter);
    interpret("'('< '= '>)", &mut interpreter);
  }
  #[test]
  fn application() {
    let mut interpreter = Interpreter::new();
    _interpret("(+ 3 2)", Number(5), &mut interpreter);
    _interpret("(- 3 2)", Number(1), &mut interpreter);
    _interpret("(* 3 2)", Number(6), &mut interpreter);
    _interpret("(/ 3 2)", Number(1), &mut interpreter);
    _interpret("(% 3 2)", Number(1), &mut interpreter);
    _interpret("(< 1 0)", FALSE, &mut interpreter);
    _interpret("(= 1 0)", FALSE, &mut interpreter);
    _interpret("(> 1 0)", TRUE, &mut interpreter);
    _interpret("(& 1 0)", Number(0), &mut interpreter);
    _interpret("(| 1 0)", Number(1), &mut interpreter);
    _interpret("(! 1)", Number(-2), &mut interpreter);
    _interpret("(& true false)", FALSE, &mut interpreter);
    _interpret("(| true false)", TRUE, &mut interpreter);
    _interpret("(! false)", TRUE, &mut interpreter);
    _interpret("(le 3 2)", FALSE, &mut interpreter);
    _interpret("(ge 3 2)", TRUE, &mut interpreter);
    _interpret("(ne 3 2)", TRUE, &mut interpreter);
    _interpret("(shl 3 2)", Number(12),  &mut interpreter);
    _interpret("(shr 3 2)", Number(0), &mut interpreter);
    _interpret("(+ 1 (* 2 3))", Number(7), &mut interpreter);
  }
  #[test]
  fn _if() {
    let mut interpreter = Interpreter::new();
    _interpret("(if true 1 2)", ONE, &mut interpreter);
    _interpret("(if true true false)", TRUE, &mut interpreter);
    _interpret("(if false 'a 'b)", Symbol(Box::from("b")), &mut interpreter);
    _interpret("(if false '& '|)", Symbol(Box::from("or")), &mut interpreter);
  }
  #[test]
  fn lambda() {
    let mut interpreter = Interpreter::new();
    interpret("(lambda (a) 1)", &mut interpreter);
    interpret("(lambda (a) true)", &mut interpreter);
    interpret("(lambda (a) a)", &mut interpreter);
    interpret("(lambda (a) &)", &mut interpreter);
  }
  #[test]
  fn define() {
    let mut interpreter = Interpreter::new();
    _interpret("(define p 1)", NIL, &mut interpreter);
    _interpret("(define q true)", NIL, &mut interpreter);
    _interpret("(define r 'a)", NIL, &mut interpreter);
    _interpret("(define s '&)", NIL, &mut interpreter);
    _interpret("(define t (lambda (n) n))", NIL, &mut interpreter);
    _interpret("p", ONE, &mut interpreter);
    _interpret("q", TRUE, &mut interpreter);
    _interpret("r", Symbol(Box::from("a")), &mut interpreter);
    _interpret("s", Symbol(Box::from("and")), &mut interpreter);
    _interpret("(t 1)", ONE, &mut interpreter);
    _interpret("(t true)", TRUE, &mut interpreter);
    _interpret("(t 'a)", Symbol(Box::from("a")), &mut interpreter);
    _interpret("(t '&)", Symbol(Box::from("and")), &mut interpreter);
  }
}