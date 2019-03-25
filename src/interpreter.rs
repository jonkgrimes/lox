use crate::token::{Token, TokenType};
use crate::expr::{Visitor, BoxedExpr, Literal, Grouping, Unary};

struct Interpreter;

struct InterpretedValue(f32);

impl Interpreter {
  fn evaluate(self, expr: BoxedExpr) -> InterpretedValue {
    expr.accept(Box::new(self))
  }
}

impl Visitor for Interpreter {
  type Value = f32;

  fn visit_literal<T>(self, expr: Literal<T>) -> Self::Value {
      expr.value()
  }

  fn visit_unary(self, expr: Unary) -> Self::Value {
    let right = self.evaluate(expr.right());

    match expr.operator().token_type() {
      TokenType::Minus => {
        return -1.0;
      }
      _ => {
        return 0.0;
      }
    }

    panic!("Encountered an unrecoverable error while evaluating unary.");
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::expr::{Expr, BoxedExpr, Literal};

  #[test]
  fn it_evaluates_numeric_literals() {
    let expr = Literal::new(5.0);
    let interpreter = Box::new(Interpreter {});
    assert_eq!(expr.accept(interpreter) as f32, 5.0);
  } 

  #[test]
  fn it_evaluates_string_literals() {
    let expr = Literal::new("A string".to_string());
    assert_eq!(evaluateLiteral(expr), "A string".to_string())
  }
}