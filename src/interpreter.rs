use crate::token::{TokenType};
use crate::lox_value::{LoxValue};
use crate::lox_error::{LoxError};
use crate::expr::{Visitor as ExprVisitor, BoxedExpr, Literal, Grouping, Unary, Binary, Variable, Assign};
use crate::stmt::{Visitor as StmtVisitor, Stmt, Expression, Print, Var, Block};
use crate::environment::Environment;

pub struct Interpreter {
  environment: Environment
}

impl Interpreter {
  pub fn new() -> Interpreter {
    Interpreter {
      environment: Environment::new()
    }
  }

  pub fn interpret(&mut self, statements: Vec<Box<dyn Stmt>>) {
    for statement in statements {
      self.execute(statement)
    }
  }

  pub fn execute(&mut self, stmt: Box<dyn Stmt>) {
    stmt.accept(self)
  }

  pub fn evaluate(&mut self, expr: BoxedExpr) -> Result<LoxValue, LoxError> {
    expr.accept(self)
  }
}

impl ExprVisitor for Interpreter {
  type Value = LoxValue;

  fn visit_nil_literal(&mut self, expr: &Literal<LoxValue>) -> Result<Self::Value, LoxError> {
      Ok(expr.value())
  }

  fn visit_number_literal(&mut self, expr: &Literal<LoxValue>) -> Result<Self::Value, LoxError> {
      Ok(expr.value())
  }

  fn visit_string_literal(&mut self, expr: &Literal<LoxValue>) -> Result<Self::Value, LoxError> {
      Ok(expr.value())
  }

  fn visit_boolean_literal(&mut self, expr: &Literal<LoxValue>) -> Result<Self::Value, LoxError> {
      Ok(expr.value())
  }

  fn visit_unary(&mut self, expr: &Unary) -> Result<Self::Value, LoxError> {
    let right = self.evaluate(expr.right())?;

    match expr.clone().operator().token_type() {
      TokenType::Minus => {
        return -right
      },
      TokenType::Bang => {
        return Ok(!self.is_truthy(right))
      },
      _ => {
        return Ok(LoxValue::Number(0.0));
      }
    }
  }

  fn visit_binary(&mut self, expr: &Binary) -> Result<Self::Value, LoxError> {
    let left = self.evaluate(expr.left())?;
    let right = self.evaluate(expr.right())?;

    match expr.clone().operator().token_type() {
        TokenType::Minus => {
          return Ok(left - right)
        },
        TokenType::Slash => {
          return Ok(left / right)
        },
        TokenType::Star => {
          return Ok(left * right)
        },
        TokenType::Plus => {
          return left + right
        },
        TokenType::Greater => {
          return Ok(LoxValue::Boolean(left > right))
        },
        TokenType::GreaterEqual => {
          return Ok(LoxValue::Boolean(left >= right))
        },
        TokenType::Less => {
          return Ok(LoxValue::Boolean(left < right))
        },
        TokenType::LessEqual => {
          return Ok(LoxValue::Boolean(left <= right))
        },
        TokenType::EqualEqual => {
          return Ok(LoxValue::Boolean(left == right))
        },
        TokenType::BangEqual => {
          return Ok(LoxValue::Boolean(left != right))
        },
        _ => return Ok(LoxValue::Number(0.0))
    }
  }

  fn visit_grouping(&mut self, expr: &Grouping) -> Result<Self::Value, LoxError> {
      self.evaluate(expr.expression())
  }

  fn visit_variable(&mut self, expr: &Variable) -> Result<Self::Value, LoxError> {
      Ok(self.environment.get(expr.name()))
  }

  fn visit_assignment(&mut self, expr: &Assign) -> Result<Self::Value, LoxError> {
    let value = self.evaluate(expr.value()).unwrap();
    self.environment.assign(expr.name(), value.clone());
    Ok(value)
  }
}

impl StmtVisitor for Interpreter {
  type Value = ();

  fn visit_expression_statement(&mut self, stmt: &Expression) {
    self.evaluate(stmt.clone().expr()).ok();
  }

  fn visit_print_statement(&mut self, stmt: &Print) {
    let value = self.evaluate(stmt.clone().expr());
    println!("{}", value.unwrap());
  }

  fn visit_var_statement(&mut self, stmt: &Var) {
    let mut value = LoxValue::Nil;
    if let Ok(initializer) = self.evaluate(stmt.initializer()) {
      value = initializer;
    }
    self.environment.define(stmt.name().lexeme(), value);
  }

  fn visit_block_statement(&mut self, stmt: &Block) {
    self.execute_block(stmt.statements(), Environment::new());
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

  fn execute_block(&mut self, statements: Vec<Box<dyn Stmt>>, environment: Environment) {
    let previous = self.environment.clone();

    self.environment = environment;

    for statement in statements {
      self.execute(statement);
    }

    self.environment = previous;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::token::{Token, TokenType};
  use crate::expr::{Unary, Literal};

  #[test]
  fn it_evaluates_numeric_literals() {
    let expr = Literal::new(LoxValue::Number(5.0f32));
    let mut interpreter = Interpreter::new();
    assert_eq!(interpreter.evaluate(expr).unwrap(), LoxValue::Number(5.0));
  } 

  #[test]
  fn it_evaluates_string_literals() {
    let expr = Literal::new(LoxValue::String("A string".to_string()));
    let mut interpreter = Interpreter::new();
    assert_eq!(interpreter.evaluate(expr).unwrap(), LoxValue::String("A string".to_string()))
  }

  #[test]
  fn it_evaluates_unary_minus_operators() {
    let expr = Unary::new(Token::new("-".to_string(), TokenType::Minus), Literal::new(LoxValue::Number(5.0)));
    let mut interpreter = Interpreter::new();
    assert_eq!(interpreter.evaluate(expr).unwrap(), LoxValue::Number(-5.0))
  }
}