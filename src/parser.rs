use crate::lexer::{tokenize, Token};
use crate::object::Object;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct ParseError {
  err: String,
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Parser error: {}", self.err)
  }
}

impl Error for ParseError {}

pub fn parse(program: &str) -> Result<Object, ParseError> {
  let token_result = tokenize(program);
  if token_result.is_err() {
    return Err(ParseError {
      err: format!("{}", token_result.err().unwrap()),
    });
  }
  let mut tokens = token_result.unwrap().into_iter().rev().collect::<Vec<_>>();
  let parsed_list = parse_list(&mut tokens)?;
  Ok(parsed_list)
}

fn parse_list(tokens: &mut Vec<Token>) -> Result<Object, ParseError> {
  let token = tokens.pop();

  if token != Some(Token::LBracket) {
    return Err(ParseError {
      err: format!("Expected `[`, found {:?}", token),
    });
  }

  let mut list: Vec<Object> = Vec::new();
  while !tokens.is_empty() {
    let token = tokens.pop();
    if token.is_none() {
      return Err(ParseError {
        err: "Did not find enough tokens".to_string(),
      });
    }

    match token.unwrap() {
      Token::Number(n) => list.push(Object::Number(n)),
      Token::Symbol(s) => match s.as_ref() {
        "true" => list.push(Object::Bool(true)),
        "false" => list.push(Object::Bool(false)),
        _ => list.push(Object::Symbol(s)),
      },
      Token::StringLit(s) => list.push(Object::String(s)),
      Token::LBracket => {
        tokens.push(Token::LBracket);
        let sub_list = parse_list(tokens)?; // recursive call
        list.push(sub_list);
      }
      Token::RBracket => {
        return Ok(Object::List(Rc::new(list)));
      }
    }
  }

  Ok(Object::List(list.into()))
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_parse_add() {
    let program = "[+ 2 1]";

    let list = parse(program).unwrap();

    assert_eq!(
      list,
      Object::List(vec![
        Object::Symbol("+".to_string()),
        Object::Number(2.0),
        Object::Number(1.0),
      ])
    );
  }

  #[test]
  fn test_parse_area_of_circle() {
    let program = "[
      [let r 10]
      [let pi 3.14]
      [* pi [* r r]]
    ]";
    let list = parse(program).unwrap();

    assert_eq!(
      list,
      Object::List(vec![
        Object::List(vec![
          Object::Symbol("let".to_string()),
          Object::Symbol("r".to_string()),
          Object::Number(10.0),
        ]),
        Object::List(vec![
          Object::Symbol("let".to_string()),
          Object::Symbol("pi".to_string()),
          Object::Number(3.14),
        ]),
        Object::List(vec![
          Object::Symbol("*".to_string()),
          Object::Symbol("pi".to_string()),
          Object::List(vec![
            Object::Symbol("*".to_string()),
            Object::Symbol("r".to_string()),
            Object::Symbol("r".to_string()),
          ]),
        ]),
      ])
    );
  }
}
