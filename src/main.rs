extern crate rlisp;
use rlisp::ast::Ast;
use rlisp::error::show_trace;
use rlisp::interpreter::Interpreter;
use std::io;

fn prompt(s: &str) -> io::Result<()> {
  use io::{stdout, Write};
  let stdout = stdout();
  let mut stdout = stdout.lock();
  stdout.write(s.as_bytes())?;
  stdout.flush()
}

fn main() -> io::Result<()> {
  use io::{stdin, BufRead, BufReader};
  let mut interp = Interpreter::new();

  let stdin = stdin();
  let stdin = stdin.lock();
  let stdin = BufReader::new(stdin);
  let mut lines = stdin.lines();

  loop {
    prompt("> ")?;
    if let Some(Ok(line)) = lines.next() {
      if line == "exit" { break; }
      let ast = match line.parse::<Ast>() {
        Ok(ast) => ast,
        Err(e) => {
          e.show_diagnostic(&line);
          show_trace(e);
          continue;
        }
      };
      println!("{:?}", ast);
      let n = match interp.eval(&ast) {
        Ok(n) => n,
        Err(e) => {
          e.show_diagnostic(&line);
          show_trace(e);
          continue;
        }
      };
      println!("{}", n);
    } else {
      break;
    }
  };
  Ok(())
}
