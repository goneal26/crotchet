// declaring crates
mod environment;
mod evaluator;
mod expression;
mod parser;
mod repl;

// TODO arg parsing
fn main() {
  repl::run();
}
