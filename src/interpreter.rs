use crate::expr::{BoxedExpr, Literal, Grouping};

fn interpret(expr: BoxedExpr) {
}

fn evaluate(expr: BoxedExpr) {

}

fn evaluateLiteral<T>(expr: Box<Literal<T>>) -> T {
  expr.value()
}

fn evaluateGrouping(expr: Box<Grouping>) {
  evaluate(expr.expression())
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::expr::{BoxedExpr, Literal};
  #[test]
  fn it_evaluates_numeric_literals() {
    let expr = Literal::new(5.0);
    assert_eq!(evaluateLiteral(expr), 5.0);
  } 

  #[test]
  fn it_evaluates_string_literals() {
    let expr = Literal::new("A string".to_string());
    assert_eq!(evaluateLiteral(expr), "A string".to_string())
  }
}