use crate::env::Env;
use crate::object::Object;
use crate::parser::parse;
use std::cell::RefCell;
use std::rc::Rc;
use rand::random;
use std::io::{self, Write};

pub fn eval(
  program: &str,
  env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
  let parsed_list = parse(program);
  if parsed_list.is_err() {
    return Err(format!("{}", parsed_list.err().unwrap()));
  }
  eval_obj(&parsed_list.unwrap(), env)
}

fn eval_obj(
  obj: &Object,
  env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
  match obj {
    Object::Void => Ok(Object::Void),
    Object::Lambda(_params, _body) => Ok(Object::Void),
    Object::Bool(_) => Ok(obj.clone()),
    Object::Number(n) => Ok(Object::Number(*n)),
    Object::Symbol(s) => eval_symbol(s, env),
    Object::List(list) => eval_list(list, env),
    Object::String(_) => Ok(obj.clone()),
  }
}

fn eval_symbol(s: &str, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
  let val = env.borrow_mut().get(s);
  if val.is_none() {
    return Err(format!("Unbound symbol: {}", s));
  };
  Ok(val.unwrap().clone())
}

fn eval_list(
  list: &Vec<Object>,
  env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
  let head = &list[0];

  match head {
    Object::Symbol(s) => match s.as_str() {
      "+" | "-" | "*" | "/" | "<" | "<=" | ">" | ">=" | "=" | "!=" => {
        eval_binary_op(list, env) // returns
      }
      
      "let" => eval_let(list, env),
      "if" => eval_if(list, env),
      "fn" => eval_function_definition(list),
      "set" => eval_set(list, env),
      "put" => eval_put(list, env),
      "get" => eval_get(list, env),
      "while" => eval_while(list, env),
      "rand" => eval_rand(),
      "round" => eval_round(list, env),
      // ^builtins go here
      
      _ => eval_function_call(s, list, env),
    },
    _ => {
      let mut new_list = Vec::new();
      for obj in list {
        let result = eval_obj(obj, env)?;
        match result {
          Object::Void => {}
          _ => new_list.push(result),
        }
      }
      Ok(Object::List(new_list))
    }
  }
}

fn eval_binary_op(
  list: &[Object],
  env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
  if list.len() != 3 {
    return Err("Invalid number of arguments for binary operator".to_string());
  }

  let operator = list[0].clone();
  let left = eval_obj(&list[1].clone(), env)?;
  let right = eval_obj(&list[2].clone(), env)?;

  let left_val = match left {
    Object::Number(n) => n,
    _ => return Err(format!("Left operand must be a number {:?}", left)),
  };
  let right_val = match right {
    Object::Number(n) => n,
    _ => return Err(format!("Right operand must be a number {:?}", right)),
  };

  match operator {
    Object::Symbol(s) => match s.as_str() {
      "+" => Ok(Object::Number(left_val + right_val)),
      "-" => Ok(Object::Number(left_val - right_val)),
      "*" => Ok(Object::Number(left_val * right_val)),
      "/" => Ok(Object::Number(left_val / right_val)),
      "<" => Ok(Object::Bool(left_val < right_val)),
      "<=" => Ok(Object::Bool(left_val <= right_val)),
      ">" => Ok(Object::Bool(left_val > right_val)),
      ">=" => Ok(Object::Bool(left_val >= right_val)),
      "=" => Ok(Object::Bool(left_val == right_val)),
      "!=" => Ok(Object::Bool(left_val != right_val)),
      _ => Err(format!("Invalid binary operator: {}", s)),
    },
    _ => Err("Operator must be a symbol".to_string()),
  }
}

fn eval_let(
  list: &[Object],
  env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
  if list.len() != 3 {
    return Err("Invalid number of arguments for `let`".to_string());
  }

  let sym = match &list[1] {
    Object::Symbol(s) => s.clone(),
    _ => return Err("Invalid `let`".to_string()),
  };
  let val = eval_obj(&list[2], env)?;
  env.borrow_mut().set(&sym, val);
  Ok(Object::Void)
}

fn eval_if(
  list: &[Object],
  env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
  if list.len() != 4 {
    return Err("Invalid number of arguments for `if`".to_string());
  }

  let cond_obj = eval_obj(&list[1], env)?;
  let cond = match cond_obj {
    Object::Bool(b) => b,
    _ => return Err("Condition must be a boolean".to_string()),
  };

  if cond {
    eval_obj(&list[2], env)
  } else {
    eval_obj(&list[3], env)
  }
}

fn eval_function_definition(list: &[Object]) -> Result<Object, String> {
  let params = match &list[1] {
    Object::List(list) => {
      let mut params = Vec::new();
      for param in list {
        match param {
          Object::Symbol(s) => params.push(s.clone()),
          _ => return Err("Invalid `fn` parameter".to_string()),
        }
      }
      params
    }
    _ => return Err("Invalid `fn`".to_string()),
  };

  let body = match &list[2] {
    Object::List(list) => list.clone(),
    _ => return Err("Invalid `fn`".to_string()),
  };

  Ok(Object::Lambda(params, body))
}

fn eval_function_call(
  s: &str,
  list: &[Object],
  env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
  let lambda = env.borrow_mut().get(s);
  if lambda.is_none() {
    return Err(format!("Unbound symbol: {}", s));
  }

  let func = lambda.unwrap();
  match func {
    Object::Lambda(params, body) => {
      // eeyikes rust your syntax is grody
      let mut new_env = Rc::new(RefCell::new(Env::extend(env.clone())));
      for (i, param) in params.iter().enumerate() {
        let val = eval_obj(&list[i + 1], env)?;
        new_env.borrow_mut().set(param, val);
      }
      eval_obj(&Object::List(body), &mut new_env)
    }
    _ => Err(format!("Not a lambda (`fn`): {}", s)),
  }
}

// builtin function time!

// put: takes a variable list of args, printing each on a single line
// after running, goes to new line and returns the number of things printed
fn eval_put(
  list: &[Object],
  env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
  if list.len() <= 1 {
    println!();
    return Ok(Object::Number(0.0));
  }

  for item in &list[1..] {
    let val = eval_obj(item, env)?;
    match val {
      Object::Void => {}
      Object::Number(n) => print!("{}", n),
      Object::Bool(b) => print!("{}", b),
      Object::Symbol(s) => print!("{}", s),
      Object::Lambda(params, body) => {
        print!("fn[");
        for param in params {
          print!("{} ", param);
        }
        print!("]");
        for expr in body {
          print!(" {}", expr);
        }
      }
      _ => print!("{}", val),
    }
  }

  println!();
  Ok(Object::Number((list.len() - 1) as f64)) // TODO beware "as" conversion?
}

// of the form [get "prompt"] where the prompt is optional
// prints "prompt" on a newline, accepts input from the user
// tries to parse input as a float, will return err upon fail
fn eval_get(
  list: &[Object],
  env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
  if list.len() > 2 {
    return Err("Invalid number of arguments for `get`".to_string());
  }

  
  let prompt = if list.len() == 2 {
    let val = eval_obj(&list[1], env)?;
    format!("{}", val)
  } else {
    "".to_string()
  };
  print!("{}", prompt);
  match io::stdout().flush() {
    Ok(_) => {},
    Err(error) => return Err(format!("`get` failed to flush input: {}", error))
  }
  
  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(_) => {},
    Err(error) => return Err(format!("`get` failed to read: {}", error))
  }

  match input.trim().parse::<f64>() {
    Ok(number) => Ok(Object::Number(number)),
    Err(_) => Err(format!("`get` failed to parse {} as float", input)),
  }
  
}

fn eval_set(
  list: &[Object],
  env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
  if list.len() != 3 {
    return Err("Invalid number of arguments for `set`".to_string());
  }

  let var = match &list[1] {
    Object::Symbol(s) => s.clone(),
    _ => return Err("First argument of `set` must be a symbol".to_string()),
  };

  let value = eval_obj(&list[2], env)?;

  let exists = {
    let borrowed_env = env.borrow();
    borrowed_env.get(&var).is_some()
  };

  if exists {
    env.borrow_mut().set(&var, value.clone());
    Ok(value)
  } else {
    Err(format!("Variable `{}` not found", var))
  }
}

// 
fn eval_while(
  list: &[Object],
  env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
  if list.len() < 3 {
    return Err("Invalid number of arguments for `while`".to_string());
  }

  let condition = &list[1];
  let body = &list[2..];

  let mut last_result = Object::Void;

  loop {
    let cond_obj = eval_obj(condition, env)?;
    let cond = match cond_obj {
      Object::Bool(b) => b,
      _ => {
        return Err(
          "Condition of `while` must evaluate to a boolean".to_string(),
        )
      }
    };

    if !cond {
      break;
    }

    for body_expr in body {
      last_result = eval_obj(body_expr, env)?;
    }
  }

  Ok(last_result)
}

// returns a random float between [0, 1)
fn eval_rand() -> Result<Object, String> {
  let value: f64 = random();
  Ok(Object::Number(value))
}

fn eval_round(
  list: &[Object], 
  env: &mut Rc<RefCell<Env>>
) -> Result<Object, String> {
  if list.len() != 2 {
    return Err("Invalid number of arguments for `round`".to_string());
  }

  match eval_obj(&list[1], env) {
    Ok(Object::Number(n)) => Ok(Object::Number(n.round())),
    _ => Err("First argument of `set` must be a number".to_string()),
  }
}
