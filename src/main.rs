// declaring crates
mod env;
mod eval;
mod lexer;
mod object;
mod parser;

use linefeed::{Interface, ReadResult};
use object::Object;
use std::cell::RefCell;
use std::rc::Rc;

const PROMPT: &str = "crutch> ";

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let reader = Interface::new(PROMPT).unwrap();
  let mut env = Rc::new(RefCell::new(env::Env::new()));

  reader.set_prompt(PROMPT.as_ref()).unwrap();

  // TODO split this out into its own function?
  while let ReadResult::Input(input) = reader.read_line().unwrap() {
    if input.eq("exit") {
      break;
    }
    
    let val = eval::eval(input.as_ref(), &mut env)?;
    match val {
      Object::Void => {}
      Object::Number(n) => println!("{}", n),
      Object::Bool(b) => println!("{}", b),
      Object::Symbol(s) => println!("{}", s),
      Object::Lambda(params, body) => {
        println!("fn[");
        for param in params {
          println!("{} ", param);
        }
        println!("]");
        for expr in body {
          println!(" {}", expr);
        }
      }
      _ => println!("{}", val),
    }
  }

  println!("[program exited successfully]");
  Ok(())
}
