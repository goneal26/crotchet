use crate::evaluator::eval_list_of_floats;
use crate::expression::{Exp, RunErr};
use std::collections::HashMap;

// environment for storing identifiers
#[derive(Clone)]
pub struct Env {
  pub data: HashMap<String, Exp>,
}

// default environment with builtin functions (just math for now)
pub fn default_env() -> Env {
  let mut data: HashMap<String, Exp> = HashMap::new();

  // addition
  data.insert(
    "+".to_string(),
    Exp::Func(|args: &[Exp]| -> Result<Exp, RunErr> {
      let sum = eval_list_of_floats(args)?
        .iter()
        .fold(0.0, |sum, a| sum + a);
      Ok(Exp::Number(sum))
    }),
  );

  // subtraction
  data.insert(
    "-".to_string(),
    Exp::Func(|args: &[Exp]| -> Result<Exp, RunErr> {
      let floats = eval_list_of_floats(args)?;

      let first = *floats
        .first()
        .ok_or(RunErr::Reason("expected at least one number".to_string()))?;

      if floats.len() == 1 {
        return Ok(Exp::Number(-first)); // 1 arg negates
      }

      let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);

      Ok(Exp::Number(first - sum_of_rest))
    }),
  );

  // multiplication
  data.insert(
    "*".to_string(),
    Exp::Func(|args: &[Exp]| -> Result<Exp, RunErr> {
      let prod = eval_list_of_floats(args)?
        .iter()
        .fold(1.0, |prod, a| prod * a);

      Ok(Exp::Number(prod))
    }),
  );

  // division
  data.insert(
    "/".to_string(),
    Exp::Func(|args: &[Exp]| -> Result<Exp, RunErr> {
      let floats = eval_list_of_floats(args)?;

      let first = *floats
        .first()
        .ok_or(RunErr::Reason("expected at least one number".to_string()))?;

      if floats.len() == 1 {
        if first == 0.0 {
          return Err(RunErr::Reason("divide by zero".to_string()));
        }

        return Ok(Exp::Number(1.0 / first)); // 1 arg means 1 / arg
      }

      let prod_of_rest = floats[1..].iter().fold(1.0, |div, a| div * a);

      if prod_of_rest == 0.0 {
        return Err(RunErr::Reason("divide by zero".to_string()));
      }

      Ok(Exp::Number(first / prod_of_rest))
    }),
  );

  Env { data }
}
