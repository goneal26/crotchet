use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  Number(f64),
  Symbol(String),
  LCrutch,
  RCrutch, // coz that's what ] is called
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Token::Number(n) => write!(f, "{}", n),
      Token::Symbol(s) => write!(f, "{}", s),
      Token::LCrutch => write!(f, "["),
      Token::RCrutch => write!(f, "]"),
    }
  }
}

#[derive(Debug)]
pub struct LexerError {
  ch: char,
}

impl Error for LexerError {}

impl fmt::Display for LexerError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "unexpected char: {}", self.ch)
  }
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, LexerError> {
  // split into lines to handle comments
  let lines = source.lines();
  let mut tokens: Vec<Token> = Vec::new();

  // clean comment from line
  for line in lines {
    // inline comment starts with a semicolon!
    let clean_line = match line.split_once(';') {
      Some((before, _)) => before,
      None => line,
    };

    // now tokenize
    let temp = clean_line.replace("[", " [ ").replace("]", " ] ");
    let words = temp.split_whitespace();

    for word in words {
      match word {
        "[" => tokens.push(Token::LCrutch),
        "]" => tokens.push(Token::RCrutch),
        _ => {
          let x = word.parse::<f64>();
          if x.is_ok() {
            tokens.push(Token::Number(x.unwrap()));
          } else {
            // for now, if not number then is identifier
            tokens.push(Token::Symbol(word.to_string()));
          }
        }
      }
    }
  }

  Ok(tokens)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add() {
    let tokens = tokenize("[+ 1 2]").unwrap_or(vec![]);
    assert_eq!(
      tokens,
      vec![
        Token::LCrutch,
        Token::Symbol("+".to_string()),
        Token::Number(1.0),
        Token::Number(2.0),
        Token::RCrutch,
      ]
    );
  }

  #[test]
  fn test_area_of_a_circle() {
    let program = "
      [
        [let r 10]
        [let pi 3.14]
        [* pi [* r r]]
      ]
    ";

    let tokens = tokenize(program).unwrap_or(vec![]);

    assert_eq!(
      tokens,
      vec![
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
      ]
    );
  }
}
