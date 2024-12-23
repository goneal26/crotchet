use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  Number(f64),
  Symbol(String),
  LBracket,
  RBracket, // coz that's what ] is called
  StringLit(String),
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Token::Number(n) => write!(f, "{n}"),
      Token::Symbol(s) | Token::StringLit(s) => write!(f, "{s}"),
      Token::LBracket => write!(f, "["),
      Token::RBracket => write!(f, "]"),
    }
  }
}

#[derive(Debug)]
pub struct LexErr {
  msg: String,
}

impl Error for LexErr {}

impl fmt::Display for LexErr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "syntax: {}", self.msg)
  }
}

#[allow(clippy::unnecessary_wraps)]
pub fn tokenize(source: &str) -> Result<Vec<Token>, LexErr> {
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
    let temp = clean_line.replace('[', " [ ").replace(']', " ] ");

    let mut chars = temp.chars().peekable();
    while let Some(c) = chars.next() {
      if c.is_whitespace() {
        continue;
      }

      match c {
        '[' => tokens.push(Token::LBracket),
        ']' => tokens.push(Token::RBracket),
        '"' => {
          // start parsing string literal
          let mut literal = String::new();
          while let Some(&next_char) = chars.peek() {
            if next_char == '"' {
              chars.next(); // consume closing quote
              break;
            }
            literal.push(next_char);
            chars.next();
          }
          tokens.push(Token::StringLit(literal));
        }
        _ => {
          // symbols and numbers
          let mut word = c.to_string();
          while let Some(&next_char) = chars.peek() {
            if next_char.is_whitespace()
              || next_char == '['
              || next_char == ']'
              || next_char == '"'
            {
              break;
            }
            word.push(next_char);
            chars.next();
          }
          if let Ok(number) = word.parse::<f64>() {
            tokens.push(Token::Number(number));
          } else {
            tokens.push(Token::Symbol(word));
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
        Token::LBracket,
        Token::Symbol("+".to_string()),
        Token::Number(1.0),
        Token::Number(2.0),
        Token::RBracket,
      ]
    );
  }

  #[test]
  fn test_string_literal() {
    let tokens = tokenize("[puts \"hello \" \"world!\"]").unwrap_or(vec![]);
    assert_eq!(
      tokens,
      vec![
        Token::LBracket,
        Token::Symbol("puts".to_string()),
        Token::StringLit("hello ".to_string()),
        Token::StringLit("world!".to_string()),
        Token::RBracket,
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
        Token::LBracket,
        Token::LBracket,
        Token::Symbol("let".to_string()),
        Token::Symbol("r".to_string()),
        Token::Number(10.0),
        Token::RBracket,
        Token::LBracket,
        Token::Symbol("let".to_string()),
        Token::Symbol("pi".to_string()),
        Token::Number(3.14),
        Token::RBracket,
        Token::LBracket,
        Token::Symbol("*".to_string()),
        Token::Symbol("pi".to_string()),
        Token::LBracket,
        Token::Symbol("*".to_string()),
        Token::Symbol("r".to_string()),
        Token::Symbol("r".to_string()),
        Token::RBracket,
        Token::RBracket,
        Token::RBracket
      ]
    );
  }
}
