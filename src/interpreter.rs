use crate::token::{TokenType};
use crate::lox_value::{LoxValue};
use crate::expr::{Visitor, BoxedExpr, Literal, Grouping, Unary, Binary};

pub struct Interpreter;

impl Interpreter {
  pub fn new() -> Interpreter {
    Interpreter {}
  }

  pub fn interpret(&mut self, expr: BoxedExpr) {
    let value = self.evaluate(expr);
    println!("=> {}", value)
  }

  pub fn evaluate(&mut self, expr: BoxedExpr) -> LoxValue {
    expr.accept(self)
  }
}

impl Visitor for Interpreter {
  type Value = LoxValue;

  fn visit_number_literal(&mut self, expr: &Literal<f32>) -> Self::Value {
      LoxValue::Number(expr.value())
  }

  fn visit_string_literal(&mut self, expr: &Literal<String>) -> Self::Value {
      LoxValue::String(expr.value())
  }

  fn visit_boolean_literal(&mut self, expr: &Literal<bool>) -> Self::Value {
      LoxValue::Boolean(expr.value())
  }

  fn visit_unary(&mut self, expr: &Unary) -> Self::Value {
    let right = self.evaluate(expr.right());

    match expr.clone().operator().token_type() {
      TokenType::Minus => {
        return -right;
      },
      TokenType::Bang => {
        return !self.is_truthy(right)
      },
      _ => {
        return LoxValue::Number(0.0);
      }
    }
  }

  fn visit_binary(&mut self, expr: &Binary) -> Self::Value {
    let left = self.evaluate(expr.left());
    let right = self.evaluate(expr.right());

    match expr.clone().operator().token_type() {
        TokenType::Minus => {
          return left - right
        },
        TokenType::Slash => {
          return left / right
        },
        TokenType::Star => {
          return left * right
        },
        TokenType::Plus => {
          return left + right
        },
        TokenType::Greater => {
          return LoxValue::Boolean(left > right)
        },
        TokenType::GreaterEqual => {
          return LoxValue::Boolean(left >= right)
        },
        TokenType::Less => {
          return LoxValue::Boolean(left < right)
        },
        TokenType::LessEqual => {
          return LoxValue::Boolean(left <= right)
        },
        TokenType::EqualEqual => {
          return LoxValue::Boolean(left == right)
        },
        TokenType::BangEqual => {
          return LoxValue::Boolean(left != right)
        },
        _ => return LoxValue::Number(0.0)
    }
  }

  fn visit_grouping(&mut self, expr: &Grouping) -> Self::Value {
      self.evaluate(expr.expression())
  }
}

impl Interpreter {
  fn is_truthy(&self, result: LoxValue) -> LoxValue {
      match result {
        LoxValue::Nil => LoxValue::Boolean(false),
        LoxValue::Number(_) => LoxValue::Boolean(true),
        LoxValue::String(_) => LoxValue::Boolean(true),
        LoxValue::Boolean(value) => LoxValue::Boolean(value),
      }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::token::{Token, TokenType};
  use crate::expr::{Unary, Literal};

  #[test]
  fn it_evaluates_numeric_literals() {
    let expr = Literal::new(5.0f32);
    let mut interpreter = Interpreter::new();
    assert_eq!(interpreter.evaluate(expr), LoxValue::Number(5.0));
  } 

  #[test]
  fn it_evaluates_string_literals() {
    let expr = Literal::new("A string".to_string());
    let mut interpreter = Interpreter::new();
    assert_eq!(interpreter.evaluate(expr), LoxValue::String("A string".to_string()))
  }

  #[test]
  fn it_evaluates_unary_minus_operators() {
    let expr = Unary::new(Token::new("-".to_string(), TokenType::Minus), Literal::new(5.0));
    let mut interpreter = Interpreter::new();
    assert_eq!(interpreter.evaluate(expr), LoxValue::Number(-5.0))
  }
}