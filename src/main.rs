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
  match interpreter.eval(&ast) {
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
use rlisp::interpreter::Interpreter;

  fn _interpret(line: &str, interpreter: &mut Interpreter) {
    let ast = line.parse::<Ast>().unwrap();
    println!("{:?}", ast);
    let result = interpreter.eval(&ast).unwrap();
    println!("{}", result)
  }
  // #[test]
  // fn empty() {
  //   let mut interpreter = Interpreter::new();
  //   _interpret("", &mut interpreter);
  // }
  #[test]
  fn atom() {
    let mut interpreter = Interpreter::new();
    _interpret("1", &mut interpreter);
    _interpret("true", &mut interpreter);
    _interpret("a", &mut interpreter);
    _interpret("!", &mut interpreter);
    _interpret("(atom 1)", &mut interpreter);
    _interpret("(atom true)", &mut interpreter);
    _interpret("(atom a)", &mut interpreter);
    _interpret("(atom !)", &mut interpreter);
  }
  #[test]
  fn quote() {
    let mut interpreter = Interpreter::new();
    _interpret("'1", &mut interpreter);
    _interpret("'true", &mut interpreter);
    _interpret("'a", &mut interpreter);
    _interpret("'!", &mut interpreter);
  }
  #[test]
  fn pair() {
    let mut interpreter = Interpreter::new();
    _interpret("'(1 . 2)", &mut interpreter);
    _interpret("'(true . false)", &mut interpreter);
    _interpret("'(a . b)", &mut interpreter);
    _interpret("'(& . |)", &mut interpreter);
    _interpret("(car '(1 . 2))", &mut interpreter);
    _interpret("(car '(true . false))", &mut interpreter);
    _interpret("(car '(a . b))", &mut interpreter);
    _interpret("(car '(& . |))", &mut interpreter);
    _interpret("(cdr '(1 . 2))", &mut interpreter);
    _interpret("(cdr '(true . false))", &mut interpreter);
    _interpret("(cdr '(a . b))", &mut interpreter);
    _interpret("(cdr '(& . |))", &mut interpreter);
    _interpret("(cons 1 2)", &mut interpreter);
    _interpret("(cons true false)", &mut interpreter);
    _interpret("(cons a b)", &mut interpreter);
    _interpret("(cons & |)", &mut interpreter);
  }
  #[test]
  fn list() {
    let mut interpreter = Interpreter::new();
    _interpret("'(1 2 3)", &mut interpreter);
    _interpret("'(true false)", &mut interpreter);
    _interpret("'(a b c d)", &mut interpreter);
    _interpret("'(< = >)", &mut interpreter);
  }
  #[test]
  fn application() {
    let mut interpreter = Interpreter::new();
    _interpret("(+ 3 2)", &mut interpreter);
    _interpret("(- 3 2)", &mut interpreter);
    _interpret("(* 3 2)", &mut interpreter);
    _interpret("(/ 3 2)", &mut interpreter);
    _interpret("(% 3 2)", &mut interpreter);
    _interpret("(< 1 0)", &mut interpreter);
    _interpret("(= 1 0)", &mut interpreter);
    _interpret("(> 1 0)", &mut interpreter);
    _interpret("(& 1 0)", &mut interpreter);
    _interpret("(| 1 0)", &mut interpreter);
    _interpret("(! 1)", &mut interpreter);
    _interpret("(& true false)", &mut interpreter);
    _interpret("(| true false)", &mut interpreter);
    _interpret("(! false)", &mut interpreter);
    _interpret("(ge 3 2)", &mut interpreter);
    _interpret("(le 3 2)", &mut interpreter);
    _interpret("(ne 3 2)", &mut interpreter);
    _interpret("(+ 1 (* 2 3))", &mut interpreter);
  }
  #[test]
  fn _if() {
    let mut interpreter = Interpreter::new();
    _interpret("(if true 1 2)", &mut interpreter);
    _interpret("(if true true false)", &mut interpreter);
    _interpret("(if false a b)", &mut interpreter);
    _interpret("(if false & |)", &mut interpreter);
  }
}