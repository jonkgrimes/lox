use crate::token::{Token, TokenType};
use crate::expr::{Visitor, BoxedExpr, Literal, Grouping, Unary, Binary, ExprResult};

struct Interpreter;

impl Interpreter {
  fn evaluate(&mut self, expr: BoxedExpr) -> ExprResult {
    expr.accept(self)
  }
}

impl Visitor for Interpreter {
  type Value = ExprResult;

  fn visit_number_literal(&mut self, expr: &Literal<f32>) -> Self::Value {
      ExprResult::Number(expr.value())
  }

  fn visit_string_literal(&mut self, expr: &Literal<String>) -> Self::Value {
      ExprResult::String(expr.value())
  }

  fn visit_boolean_literal(&mut self, expr: &Literal<bool>) -> Self::Value {
      ExprResult::Boolean(expr.value())
  }

  fn visit_unary(&mut self, expr: &Unary) -> Self::Value {
    let right = self.evaluate(expr.right());

    match expr.operator().token_type() {
      TokenType::Minus => {
        return ExprResult::Number(-1.0);
      }
      _ => {
        return ExprResult::Number(0.0);
      }
    }

    panic!("Encountered an unrecoverable error while evaluating unary.");
  }

  fn visit_binary(&mut self, expr: &Binary) -> Self::Value {
      ExprResult::Number(0.0)
  }

  fn visit_grouping(&mut self, expr: &Grouping) -> Self::Value {
      ExprResult::Number(0.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::expr::{Expr, BoxedExpr, Literal, Visitor, Visitable};

  #[test]
  fn it_evaluates_numeric_literals() {
    let expr = Box::new(Literal::new(5.0));
    let mut interpreter = Interpreter {};
    assert_eq!(interpreter.evaluate(expr), ExprResult::Number(5.0));
  } 

/*   #[test]
  fn it_evaluates_string_literals() {
    let expr = Literal::new("A string".to_string());
    let mut interpreter = Interpreter {};
    assert_eq!(expr.accept(&mut interpreter), ExprResult::String("A string".to_string()))
  } */
}