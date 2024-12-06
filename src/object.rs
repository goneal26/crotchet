pub enum Object {
  Void,
  Number(f64),
  Bool(bool),
  Symbol(String),
  Lambda(Vec<String>, Vec<Object>),
  List(Vec<Object>),
}
