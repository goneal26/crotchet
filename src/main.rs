use std::io;
use std::fmt;
use std::collections::HashMap;
use std::num::ParseFloatError;

// S-Expression type
#[derive(Clone)]
enum Exp {
  Symbol(String), // identifier
  Number(f64),
  List(Vec<Exp>),
  Func(fn(&[Exp]) -> Result<Exp, RunErr>),
}

impl fmt::Display for Exp {
  // for converting our S-Expression object to a string, for repl
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let str = match self {
      Exp::Symbol(s) => s.clone(), 
      Exp::Number(n) => n.to_string(),
      Exp::List(list) => {
        let xs: Vec<String> = list
          .iter()
          .map(|x| x.to_string())
          .collect();
        format!("({})", xs.join(","))
      },
      Exp::Func(_) => "Function {}".to_string(),
    };
    
    write!(f, "{}", str)
  }
}

// interpreter error type
#[derive(Debug)]
enum RunErr {
  Reason(String),
}

// interpreter environment (for var storage)
#[derive(Clone)]
struct Env {
  data: HashMap<String, Exp>,
}

// tokenization
fn tokenize(src: String) -> Vec<String> {
  src
    .replace("[", " [ ") // unlike lisp, we use angle brackets
    .replace("]", " ] ") // because then we don't have to hold shift
    .split_whitespace()
    .map(|x| x.to_string())
    .collect()
}

// parsing a list of tokens/possible sublists
fn read_seq<'a>(tokens: &'a [String]) -> Result<(Exp, &'a [String]), RunErr> {
  let mut res: Vec<Exp> = vec![];
  let mut xs = tokens;
  loop {
    let (next_token, rest) = xs
      .split_first()
      .ok_or(RunErr::Reason("could not find closing `]`".to_string()))
      ?;
    if next_token == "]" {
      return Ok((Exp::List(res), rest)) // skip `]`, head to the token after
    }
    let (exp, new_xs) = parse(&xs)?;
    res.push(exp);
    xs = new_xs;
  }
}

// parsing a single symbol
fn parse_atom(token: &str) -> Exp {
  let potential_float: Result<f64, ParseFloatError> = token.parse();
  match potential_float {
    Ok(v) => Exp::Number(v),
    Err(_) => Exp::Symbol(token.to_string().clone())
  }
}

// parse ALL the tokens
fn parse<'a>(tokens: &'a [String]) -> Result<(Exp, &'a [String]), RunErr> {
  let (token, rest) = tokens.split_first().ok_or(
    RunErr::Reason("could not get token".to_string())
  )?;

  match &token[..] {
    "[" => read_seq(rest), // found a list
    "]" => Err(RunErr::Reason("unexpected `]`".to_string())),
    _ => Ok((parse_atom(token), rest)),
  }
  
}

// parses a list of arguments for a function call 
fn parse_list_of_floats(args: &[Exp]) -> Result<Vec<f64>, RunErr> {
  args
    .iter()
    .map(parse_single_float)
    .collect()
}

// parses a single float for its err value
fn parse_single_float(exp: &Exp) -> Result<f64, RunErr> {
  match exp {
    Exp::Number(num) => Ok(*num),
    _ => Err(RunErr::Reason("expected a number".to_string())),
  }
}

// default environment with builtin functions (just math for now)
fn default_env() -> Env {
  let mut data: HashMap<String, Exp> = HashMap::new();

  // addition
  data.insert(
    "+".to_string(),
    Exp::Func(
      |args: &[Exp]| -> Result<Exp, RunErr> {
        // stuff like this makes me hate this language VVV
        let sum = parse_list_of_floats(args)?.iter()
          .fold(0.0, |sum, a| sum + a);
        Ok(Exp::Number(sum))
      }
    )
  );

  // subtraction
  data.insert(
    "-".to_string(),
    Exp::Func(
      |args: &[Exp]| -> Result<Exp, RunErr> {
        // stuff like this makes me hate this language VVV
        let floats = parse_list_of_floats(args)?;
        
        let first = *floats.first()
          .ok_or(RunErr::Reason("expected at least one number".to_string()))?;

        let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);
        
        Ok(Exp::Number(first - sum_of_rest))
      }
    )
  );

  
  // multiplication TODO left off here
  /*
  data.insert(
    "*".to_string(),
    Exp::Func(
      |args: &[Exp]| -> Result<Exp, RunErr> {
        // stuff like this makes me hate this language VVV
        let product = parse_list_of_floats(args)?.iter()
          .fold(0.0, |product, a| product * a);
        Ok(Exp::Number(product))
      }
    )
  );

  // division, TODO divide by 0 checking
  data.insert(
    "/".to_string(),
    Exp::Func(
      |args: &[Exp]| -> Result<Exp, RunErr> {
        // stuff like this makes me hate this language VVV
        let div = parse_list_of_floats(args)?.iter()
          .fold(0.0, |div, a| div / a);
        Ok(Exp::Number(div))
      }
    )
  );
  */
  
  Env {data}
}

// evaluate an expression
fn eval(exp: &Exp, env: &mut Env) -> Result<Exp, RunErr> {
  match exp {
    // if input is a symbol
    Exp::Symbol(k) =>
      env.data.get(k) // check env for symbol
      .ok_or(RunErr::Reason(format!("unexpected symbol k='{}'", k)))
      .map(|x| x.clone()), // if exists, grab it from the environment

    // if input is a number
    Exp::Number(_a) => Ok(exp.clone()), // return

    // if input is a list (starts with [)
    Exp::List(list) => {
      // eval first form, should be a func
      let first_form = list.first()
        .ok_or(RunErr::Reason("expected a non-empty list".to_string()))?;

      // call that func with the rest of the list as args
      let arg_forms = &list[1..];
      let first_eval = eval(first_form, env)?;
      match first_eval {
        Exp::Func(f) => {
          let args_eval = arg_forms.iter().map(|x| eval(x, env))
            .collect::<Result<Vec<Exp>, RunErr>>();
            
          f(&args_eval?)
        },
        _ => Err(RunErr::Reason("first form must be a function".to_string())),
      }
    },

    // otherwise, unexpected
    Exp::Func(_) => Err(RunErr::Reason("unexpected form".to_string())),
  }
}

// parse and eval an input string
fn parse_eval(expr: String, env: &mut Env) -> Result<Exp, RunErr> {
  let (parsed_exp, _) = parse(&tokenize(expr))?;
  let evaled_exp = eval(&parsed_exp, env)?;

  Ok(evaled_exp)
}

// read expr string from stdin and return
fn read_expr() -> String {
  let mut expr = String::new();

  io::stdin().read_line(&mut expr).expect("Failed to read line");

  expr
}

fn main() {
  println!("Welcome to Crutches!");

  // repl time!
  let env = &mut default_env();
  loop {
    println!("crutches >");
    let expr = read_expr();
    match parse_eval(expr, env) {
      Ok(res) => println!("; {}", res),
      Err(e) => match e {
        RunErr::Reason(msg) => println!("; ERROR: {}", msg),
      },
    }
  }
}
