// declaring crates
mod env;
mod eval;
mod lexer;
mod object;
mod parser;

use linefeed::{Interface, ReadResult};
use object::Object;
use std::cell::RefCell;
use std::env as e; // TODO eww alias
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

const PROMPT: &str = "crutch> ";
const EXTENSION: &str = ".cx"; // TODO different extension?

fn main() {
  let args: Vec<String> = e::args().collect();
  let argc = args.len();

  match argc {
    // TODO better usage error
    argc if argc > 2 => {
      eprintln!("; crutch usage error: too many args");
      eprintln!("; usage: crutch [file.cx]");
    }
    argc if argc < 2 => {
      match repl() {
        Ok(_) => println!("; crutch program exited successfully"),
        Err(error) => eprintln!("; crutch error: {}", error),
      };
    }
    _ => {
      // TODO run file
      match run_file(&args[1]) {
        Ok(_) => println!("; crutch program exited successfully"),
        Err(error) => eprintln!("; crutch error: {}", error),
      }
    }
  }
}

fn run_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
  if !filename.ends_with(EXTENSION) {
    return Err(format!("File must have extension {}", EXTENSION).into());
  }

  let mut file = File::open(filename)?;
  let mut program = String::new();
  file.read_to_string(&mut program)?; // file contents stored in "program"

  // eval the file contents
  let mut env = Rc::new(RefCell::new(env::Env::new()));

  eval::eval(program.as_ref(), &mut env)?;

  Ok(())
}

fn repl() -> Result<(), Box<dyn std::error::Error>> {
  println!(
    "; Welcome to crutch v{}, type `exit` to exit",
    env!("CARGO_PKG_VERSION")
  );
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
      Object::Number(n) => println!("; {}", n),
      Object::Bool(b) => println!("; {}", b),
      Object::Symbol(s) => println!("; {}", s),
      Object::Lambda(params, body) => {
        println!("; fn[");
        for param in params {
          println!("{} ", param);
        }
        println!("]");
        for expr in body {
          println!(" {}", expr);
        }
      }
      _ => println!("; {}", val),
    }
  }

  Ok(())
}
