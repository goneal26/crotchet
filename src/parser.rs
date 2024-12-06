use crate::lexer::Token;
use crate::object::Object;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParserError {
  err: String,
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Parser error: {}", self.err)
  }
}

impl Error for ParseError {}

pub fn parse(tokens: &mut Vec<Token>) -> Result<Object, ParseError> {
  let token = tokens.pop();

  if token != Some(Token::LCrutch) {
    return Err(ParseError {err: format!("Expected `[`, found {:?}", token)});
  }

  let mut list: Vec<Object> = Vec::new();
  while !tokens.is_empty() {
    let token = tokens.pop();
    if token == None {
      return Err(ParseError {err: format!("Did not find enough tokens")});
    }

    match token.unwrap() {
      Token::Number(n) => list.push(Object::Number(n)),
      Token::Symbol(s) => list.push(Object::Symbol(s)),
      Token::LCrutch => {
        tokens.push(Token::LCrutch);
        let sub_list = parse(tokens)?; // recursive call
        list.push(sub_list);
      }
      Token::RCrutch => {
        return Ok(Object::List(list));
      }
    }
  }

  Ok(Object::List(list))
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_add() {
    let tokens = vec![
      Token::LCrutch,
      Token::Symbol("+".to_string()),
      Token::Number(1.0),
      Token::Number(2.0),
      Token::RCrutch,
    ];

    let list = parse(&mut tokens).unwrap();

    assert_eq!(list, Object::List(vec![
      Object::Symbol("+".to_string()),
      Object::Number(1.0),
      Object::Number(2.0),
    ]));
  }

  #[test]
  fn test_parse_area_of_circle() {
    let tokens = vec![
      Token::LCrutch,
      Token::LCrutch,
      Token::Symbol("let".to_string()),
      Token::Symbol("r".to_string()),
      Token::Number(10.0),
      Token::RCrutch,
      Token::LCrutch,
      Token::Symbol("let".to_string()),
      Token::Symbol("pi".to_string()),
      Token::Number(3.14),
      Token::RCrutch,
      Token::LCrutch,
      Token::Symbol("*".to_string()),
      Token::Symbol("pi".to_string()),
      Token::LCrutch,
      Token::Symbol("*".to_string()),
      Token::Symbol("r".to_string()),
      Token::Symbol("r".to_string()),
      Token::RCrutch,
      Token::RCrutch,
      Token::RCrutch
    ];

    let list = parse(&mut tokens).unwrap();

    assert_eq!(
      list,
      Object::List(vec![
        Object::List(vec![
          Object::Symbol("define".to_string()),
          Object::Symbol("r".to_string()),
          Object::Number(10.0),
        ]),
        Object::List(vec![
          Object::Symbol("define".to_string()),
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
