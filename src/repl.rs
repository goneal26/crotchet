use crate::environment::default_env;
use crate::evaluator::eval;
use crate::expression::RunErr;
use std::io::{self, Write};

pub fn run() {
  let env = &mut default_env();

  loop {
    print!(">> ");
    let source = read();
    match eval(source, env) {
      Ok(res) => println!(";; {}", res),
      Err(e) => match e {
        RunErr::Reason(msg) => println!(";; ERROR: {}", msg),
      },
    }
  }
}

// read expr string from stdin and return
fn read() -> String {
  let mut expr = String::new();
  let _ = io::stdout().flush();
  io::stdin()
    .read_line(&mut expr)
    .expect("Failed to read line");
  expr
}
