use crate::object::Object;
use crate::env::Env;
use std::cell::RefCell;
use std::rc::Rc;

pub fn eval(obj: &Object, env: &mut Rc<RefCell<Env>>) -> Result<Object, String>
{
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
  if val.is_none() { return Err(format!("Unbound symbol: {}", s)) };
  Ok(val.unwrap().clone())
}

fn eval_list(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
  let head = &list[0];

  match head {
    Object::Symbol(s) => match s.as_str() {
      "+" | "-" | "*" | "/" | "<" | "<=" | ">" | ">=" | "=" | "!=" => {
        return eval_binary_op(&list, env); // TODO more???
      }
      "def" => eval_def(&list, env),
      "if" => eval_if(&list, env),
      "fn" => eval_function_definition(&list),
      _ => eval_function_call(&s, &list, env),
    },
    _ => {
      let mut new_list = Vec::new();
      for obj in list {
        let result = eval(obj, env)?;
        match result {
          Object::Void => {}
          _ => new_list.push(result),
        }
      }
      Ok(Object::List(new_list))
    }
  }
}

fn eval_binary_op(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
  if list.len() != 3 {
    return Err(format!("Invalid number of arguments for binary operator"));
  }

  let operator = list[0].clone();
  let left = eval(&list[1].clone(), env)?;
  let right = eval(&list[2].clone(), env)?;

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
    _ => Err(format!("Operator must be a symbol")),
  }
}

fn eval_def(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
  if list.len() != 3 {
    return Err(format!("Invalid number of arguments for `def`"));
  }

  let sym = match &list[1] {
    Object::Symbol(s) => s.clone(),
    _ => return Err(format!("Invalid `def`")),
  };
  let val = eval_obj(&list[2], env)?;
  env.borrow_mut().set(&sym, val);
  Ok(Object::Void)
}

fn eval_if(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
  if list.len() != 4 {
    return Err(format!("Invalid number of arguments for `if`"));
  }

  let cond_obj = eval(&list[1], env)?;
  let cond = match cond_obj {
    Object::Bool(b) => b,
    _ => return Err(format!("Condition must be a boolean")),
  };

  if cond == true {
    return eval(&list[2], env);
  } else {
    return eval(&list[3], env);
  }
}

fn eval_function_definition(list: &Vec<Object>) -> Result<Object, String> {
  let params = match &list[1] {
    Object::List(list) => {
      let mut params = Vec::new();
      for param in list {
        match param {
          Object::Symbol(s) => params.push(s.clone()),
          _ => return Err(format!("Invalid `fn` parameter")),
        }
      }
      params
    }
    _ => return Err(format!("Invalid `fn`")),
  };

  let body = match &list[2] {
    Object::List(list) => list.clone(),
    _ => return Err(format!("Invalid `fn`")),
  };

  Ok(Object::Lambda(params, body))
}

fn eval_function_call(s: &str, list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
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
        let val = eval(&list[i + 1], env)?;
        new_env.borrow_mut().set(param, val);
      }
      return eval(&Object::List(body), &mut new_env);
    }
    _ => return Err(format!("Not a lambda (`fn`): {}", s)),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
}
