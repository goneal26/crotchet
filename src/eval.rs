use crate::env::Env;
use crate::object::Object;
use crate::parser::parse;
use rand::Rng;
use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

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
    Object::ListData(l) => Ok(Object::ListData(l.to_vec())),
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
      "input" => eval_input(list, env),
      "print" => eval_print(list, env),
      "while" => eval_while(list, env),
      "rand" => eval_rand(list, env),
      "round" => eval_round(list, env),
      "list" => eval_list_data(list, env),
      "first" => eval_first(list, env),
      "rest" => eval_rest(list, env),
      "len" => eval_len(list, env),
      // ^builtins go here
      _ => eval_function_call(s, list, env),
    },
    _ => {
      let mut new_list = Vec::new();
      for obj in (*list).iter() {
        let result = eval_obj(obj, env)?;
        match result {
          Object::Void => {}
          _ => new_list.push(result),
        }
      }

      match &new_list.first() {
        Some(Object::Lambda(_, _)) => {
          eval_obj(&Object::List(Rc::new(new_list)), env)
        }
        _ => Ok(Object::List(Rc::new(new_list))),
      }
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
      "%" => Ok(Object::Number(left_val % right_val)),
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
      for param in (*list).iter() {
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

  Ok(Object::Lambda(params, body.to_vec()))
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
      eval_obj(&Object::List(Rc::new(body)), &mut new_env)
    }
    _ => Err(format!("Not a lambda (`fn`): {}", s)),
  }
}

// print: takes a variable list of args, printing each on a single line
// after running, goes to new line and returns the number of things printed
fn eval_print(
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

// of the form [input "prompt"] where the prompt is optional
// prints "prompt" on a newline, accepts input from the user
// tries to parse input as a float, will return err upon fail
fn eval_input(
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
    Ok(_) => {}
    Err(error) => {
      return Err(format!("`get` failed to flush input: {}", error))
    }
  }

  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(_) => {}
    Err(error) => return Err(format!("`get` failed to read: {}", error)),
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

  let value = eval_obj(&list[2], env)?;

  match &list[1] {
    Object::Symbol(s) => {
      env.borrow_mut().set(s, value.clone());
      Ok(value)
    }
    _ => Err("First argument of `set` not symbol".to_string()),
  }
}

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

// of the form [rand x y]
// returns a random number on the interval [x, y)
fn eval_rand(
  list: &[Object],
  env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
  if list.len() != 3 {
    return Err("Invalid number of arguments for `rand`".to_string());
  }

  let min = eval_obj(&list[1], env)?;
  let max = eval_obj(&list[2], env)?;

  match (min, max) {
    (Object::Number(i), Object::Number(j)) => {
      let mut rng = rand::thread_rng();
      let random_value: f64 = rng.gen_range(i..j);
      Ok(Object::Number(random_value))
    }
    (_, _) => Err("Invalid argument types for `rand`".to_string()),
  }
}

fn eval_round(
  list: &[Object],
  env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
  if list.len() != 2 {
    return Err("Invalid number of arguments for `round`".to_string());
  }

  match eval_obj(&list[1], env) {
    Ok(Object::Number(n)) => Ok(Object::Number(n.round())),
    _ => Err("First argument of `set` must be a number".to_string()),
  }
}

// evals an s-expression as a list
fn eval_list_data(
  list: &[Object],
  env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
  let mut new_list = Vec::new();

  for obj in list[1..].iter() {
    new_list.push(eval_obj(obj, env)?);
  }

  Ok(Object::ListData(new_list))
}

// returns the first element of a list:
// [first [list 1 2 3]] ; returns 1
fn eval_first(
  list: &[Object],
  env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
  if list.len() != 2 {
    return Err("Invalid number of arguments for `first`".to_string());
  }

  match eval_obj(&list[1], env) {
    Ok(Object::ListData(l)) => Ok(l[0].clone()),
    _ => Err("First argument of `first` must be a list".to_string()),
  }
}

// returns all elements of a list, without the first
// [rest [list 1 2 3]] ; returns [2 3]
fn eval_rest(
  list: &[Object],
  env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
  if list.len() != 2 {
    return Err("Invalid number of arguments for `rest`".to_string());
  }

  let mut new_list = Vec::new();

  match eval_obj(&list[1], env) {
    Ok(Object::ListData(l)) => {
      for item in l[1..].iter() {
        new_list.push(item.clone());
      }
      Ok(Object::ListData(new_list))
    }
    _ => Err("First argument of `rest` must be a list".to_string()),
  }
}

// returns the length of a list
// [len [list 1 2 3]] ; returns 3
fn eval_len(
  list: &[Object],
  env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
  if list.len() != 2 {
    return Err("Invalid number of arguments for `len`".to_string());
  }

  match eval_obj(&list[1], env) {
    Ok(Object::ListData(l)) => Ok(Object::Number(l.len() as f64)),
    _ => Err("First argument of `rest` must be a list".to_string()),
  }
}

// TODO tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simple_add() {
    let mut env = Rc::new(RefCell::new(Env::new()));
    let result = eval("[+ 1 2]", &mut env).unwrap();
    assert_eq!(result, Object::Number(3.0));
  }

  #[test]
  fn test_area_of_a_circle() {
    let mut env = Rc::new(RefCell::new(Env::new()));
    let program = "[
                     [let r 10]
                     [let pi 3.14]
                     [* pi [* r r]]
                   ]";
    let result = eval(program, &mut env).unwrap();
    assert_eq!(
      result,
      Object::List(vec![Object::Number((3.14 * 10.0 * 10.0) as f64)].into())
    );
  }

  #[test]
  fn test_sqr_function() {
    let mut env = Rc::new(RefCell::new(Env::new()));
    let program = "[
                      [let sqr [fn [r] [* r r]]] 
                      [sqr 10]
                   ]
                  ";
    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::List(vec![Object::Number((10 * 10) as f64)].into()));
  }

  #[test]
  fn test_fibonaci() {
    let mut env = Rc::new(RefCell::new(Env::new()));
    let program = "
            [
              [let fib [fn [n] [if [< n 2] 1 [+ [fib [- n 1]] [fib [- n 2]]]]]]
              [fib 10]
            ]
        ";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::List(vec![Object::Number((89) as f64)].into()));
  }

  #[test]
  fn test_factorial() {
    let mut env = Rc::new(RefCell::new(Env::new()));
    let program = "
            [
              [let fact [fn [n] [if [< n 1] 1 [* n [fact [- n 1]]]]]]
              [fact 5]
            ]
        ";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::List(vec![Object::Number((120) as f64)].into()));
  }

  #[test]
  fn test_circle_area_function() {
    let mut env = Rc::new(RefCell::new(Env::new()));
    let program = "
            [
              [let pi 3.14]
              [let r 10]
              [let sqr [fn [r] [* r r]]]
              [let area [fn [r] [* pi [sqr r]]]]
              [area r]
            ]
        ";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(
      result,
      Object::List(vec![Object::Number((3.14 * 10.0 * 10.0) as f64)].into())
    );
  }
}
