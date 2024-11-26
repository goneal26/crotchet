use std::fmt;

#[derive(Clone)]
pub enum Exp {
  Symbol(String), // identifier
  Number(f64),
  List(Vec<Exp>),
  Func(fn(&[Exp]) -> Result<Exp, RunErr>),
}

impl fmt::Display for Exp {
  // for converting our S-Expression object to a string, for repl
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let str = match self {
      Exp::Symbol(s) => s.clone(),
      Exp::Number(n) => n.to_string(),
      Exp::List(list) => {
        let xs: Vec<String> = list.iter().map(|x| x.to_string()).collect();
        format!("[{}]", xs.join(","))
      }
      Exp::Func(_) => "Function {}".to_string(),
    };

    write!(f, "{}", str)
  }
}

// interpreter error type
#[derive(Debug)]
pub enum RunErr {
  Reason(String),
}
