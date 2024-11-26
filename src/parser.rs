use crate::expression::{Exp, RunErr};
use std::num::ParseFloatError;

// tokenization
pub fn tokenize(src: String) -> Vec<String> {
  src
    .replace("[", " [ ") // unlike lisp, we use angle brackets
    .replace("]", " ] ") // because then we don't have to hold shift
    .split_whitespace()
    .map(|x| x.to_string())
    .collect()
}

// parsing a list of tokens/possible sublists
pub fn read_seq<'a>(
  tokens: &'a [String],
) -> Result<(Exp, &'a [String]), RunErr> {
  let mut res: Vec<Exp> = vec![];
  let mut xs = tokens;
  loop {
    let (next_token, rest) = xs
      .split_first()
      .ok_or(RunErr::Reason("could not find closing `]`".to_string()))?;
    if next_token == "]" {
      return Ok((Exp::List(res), rest)); // skip `]`, head to the token after
    }
    let (exp, new_xs) = parse(&xs)?;
    res.push(exp);
    xs = new_xs;
  }
}

// parse ALL the tokens
pub fn parse<'a>(tokens: &'a [String]) -> Result<(Exp, &'a [String]), RunErr> {
  let (token, rest) = tokens
    .split_first()
    .ok_or(RunErr::Reason("could not get token".to_string()))?;

  match &token[..] {
    "[" => read_seq(rest), // found a list
    "]" => Err(RunErr::Reason("unexpected `]`".to_string())),
    _ => Ok((parse_atom(token), rest)),
  }
}

// parsing a single symbol
fn parse_atom(token: &str) -> Exp {
  let potential_float: Result<f64, ParseFloatError> = token.parse();
  match potential_float {
    Ok(v) => Exp::Number(v),
    Err(_) => Exp::Symbol(token.to_string().clone()),
  }
}
