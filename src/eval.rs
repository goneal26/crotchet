use crate::env::Env;
use crate::object::Object;
use std::cell::RefCell;
use std::rc::Rc;
use crate::parser::parse;

pub fn eval(program: &str, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
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
        eval_binary_op(list, env) // returns - TODO more???
      }
      "def" => eval_def(list, env),
      "if" => eval_if(list, env),
      "fn" => eval_function_definition(list),
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

fn eval_def(
  list: &[Object],
  env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
  if list.len() != 3 {
    return Err("Invalid number of arguments for `def`".to_string());
  }

  let sym = match &list[1] {
    Object::Symbol(s) => s.clone(),
    _ => return Err("Invalid `def`".to_string()),
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

// #[cfg(test)]
// mod tests {
//   use super::*;
// }
