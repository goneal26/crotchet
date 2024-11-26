use crate::environment::Env;
use crate::expression::{Exp, RunErr};
use crate::parser::{parse, tokenize};

// parse and eval an input string
pub fn eval(src: String, env: &mut Env) -> Result<Exp, RunErr> {
  let (parsed_exp, _) = parse(&tokenize(src))?;
  let evaled_exp = eval_expr(&parsed_exp, env)?;

  Ok(evaled_exp)
}

// evaluate an expression
fn eval_expr(exp: &Exp, env: &mut Env) -> Result<Exp, RunErr> {
  match exp {
    // if input is a symbol
    Exp::Symbol(k) => env
      .data
      .get(k) // check env for symbol
      .ok_or(RunErr::Reason(format!("unexpected symbol k='{}'", k)))
      .map(|x| x.clone()), // if exists, grab it from the environment

    // if input is a number
    Exp::Number(_a) => Ok(exp.clone()), // return

    // if input is a list (starts with [)
    Exp::List(list) => {
      // eval first form, should be a func
      let first_form = list
        .first()
        .ok_or(RunErr::Reason("expected a non-empty list".to_string()))?;

      // call that func with the rest of the list as args
      let arg_forms = &list[1..];
      let first_eval = eval_expr(first_form, env)?;
      match first_eval {
        Exp::Func(f) => {
          let args_eval = arg_forms
            .iter()
            .map(|x| eval_expr(x, env))
            .collect::<Result<Vec<Exp>, RunErr>>();

          f(&args_eval?)
        }
        _ => Err(RunErr::Reason("first form must be a function".to_string())),
      }
    }

    // otherwise, unexpected
    Exp::Func(_) => Err(RunErr::Reason("unexpected form".to_string())),
  }
}

// helper for parsing a single expr for its err value
fn eval_single_float(exp: &Exp) -> Result<f64, RunErr> {
  match exp {
    Exp::Number(num) => Ok(*num),
    _ => Err(RunErr::Reason("expected a number".to_string())),
  }
}

// helper for parsing a list of arguments for a function call
pub fn eval_list_of_floats(args: &[Exp]) -> Result<Vec<f64>, RunErr> {
  args.iter().map(eval_single_float).collect()
}
