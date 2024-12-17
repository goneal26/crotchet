use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
  Void,
  Number(f64),
  Bool(bool),
  Symbol(String),
  Lambda(Vec<String>, Vec<Object>),
  List(Rc<Vec<Object>>),
  ListData(Vec<Object>),
  String(String),
}

impl fmt::Display for Object {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Object::Void => write!(f, "Void"),
      Object::Number(n) => write!(f, "{n}"),
      Object::Bool(b) => write!(f, "{b}"),
      Object::Symbol(s) | Object::String(s) => write!(f, "{s}"),
      Object::Lambda(params, body) => {
        writeln!(f, "fn[")?;
        for param in params {
          writeln!(f, "{param} ")?;
        }
        writeln!(f, "]")?;
        for expr in body {
          writeln!(f, " {expr}")?;
        }
        Ok(())
      }
      Object::List(list) => {
        write!(f, "[")?;
        for (i, obj) in (*list).iter().enumerate() {
          if i > 0 {
            write!(f, " ")?;
          }
          write!(f, "{obj}")?;
        }
        write!(f, "]")
      }
      Object::ListData(list) => {
        write!(f, "[")?;
        for (i, obj) in list.iter().enumerate() {
          if i > 0 {
            write!(f, " ")?;
          }
          write!(f, "{obj}")?;
        }
        write!(f, "]")
      }
    }
  }
}
