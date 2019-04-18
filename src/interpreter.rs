use std::rc::Rc;
use std::cell::RefCell;
use crate::token::{TokenType};
use crate::lox_value::{LoxValue};
use crate::lox_error::{LoxError};
use crate::lox_callable::{LoxCallable};
use crate::expr::{Visitor as ExprVisitor, BoxedExpr, Literal, Grouping, Unary, Binary, Variable, Assign, Logical, Call};
use crate::stmt::{Visitor as StmtVisitor, Stmt, Expression, Print, Var, Block, If, While, Function, Return};
use crate::environment::Environment;
use crate::lox_function::LoxFunction;

pub struct Interpreter {
  environment: Rc<RefCell<Environment>>
}

impl Interpreter {
  pub fn new() -> Interpreter {
    Interpreter {
      environment: Rc::new(RefCell::new(Environment::new()))
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

  fn visit_logical(&mut self, expr: &Logical) -> Result<Self::Value, LoxError> {
    let left = self.evaluate(expr.left()).unwrap();
    let is_truthy = self.is_truthy(left.clone());
    
    if expr.operator().token_type() == TokenType::Or {
      match is_truthy {
        LoxValue::Boolean(true) => return Ok(left),
        _ =>  self.evaluate(expr.right())
      }
    } else { 
      match is_truthy {
        LoxValue::Boolean(false) => return Ok(left),
        _ =>  self.evaluate(expr.right())
      }
    }

  }

  fn visit_call(&mut self, expr: &Call) -> Result<Self::Value, LoxError> {
    let callee = self.evaluate(expr.callee());

    let arguments: Vec<LoxValue> = expr.arguments().iter().map(|argument| {
      self.evaluate(argument.clone()).unwrap()
    }).collect();

    match callee {
      Ok(callee_value) => {
        match callee_value {
          LoxValue::Function(function) => { function.call(self, arguments) },
          _ => Ok(callee_value)
        }
      },
      _ => {
        panic!("Could not evaluate function arguments")
      }
    }
  }

  fn visit_unary(&mut self, expr: &Unary) -> Result<Self::Value, LoxError> {
    let right = self.evaluate(expr.right())?;

    match expr.clone().operator().token_type() {
      TokenType::Minus => {
        -right
      },
      TokenType::Bang => {
        Ok(!self.is_truthy(right))
      },
      _ => {
        Ok(LoxValue::Number(0.0))
      }
    }
  }

  fn visit_binary(&mut self, expr: &Binary) -> Result<Self::Value, LoxError> {
    let left = self.evaluate(expr.left())?;
    let right = self.evaluate(expr.right())?;

    match expr.clone().operator().token_type() {
        TokenType::Minus => {
          Ok(left - right)
        },
        TokenType::Slash => {
          Ok(left / right)
        },
        TokenType::Star => {
          Ok(left * right)
        },
        TokenType::Plus => {
          left + right
        },
        TokenType::Greater => {
          Ok(LoxValue::Boolean(left > right))
        },
        TokenType::GreaterEqual => {
          Ok(LoxValue::Boolean(left >= right))
        },
        TokenType::Less => {
          Ok(LoxValue::Boolean(left < right))
        },
        TokenType::LessEqual => {
          Ok(LoxValue::Boolean(left <= right))
        },
        TokenType::EqualEqual => {
          Ok(LoxValue::Boolean(left == right))
        },
        TokenType::BangEqual => {
          Ok(LoxValue::Boolean(left != right))
        },
        _ => Ok(LoxValue::Number(0.0))
    }
  }

  fn visit_grouping(&mut self, expr: &Grouping) -> Result<Self::Value, LoxError> {
      self.evaluate(expr.expression())
  }

  fn visit_variable(&mut self, expr: &Variable) -> Result<Self::Value, LoxError> {
      let mut env_ref = self.environment.borrow_mut();
      Ok(env_ref.get(expr.name()))
  }

  fn visit_assignment(&mut self, expr: &Assign) -> Result<Self::Value, LoxError> {
    let value = self.evaluate(expr.value()).unwrap();
    let mut env_ref = self.environment.borrow_mut();
    env_ref.assign(expr.name(), value.clone());
    Ok(value)
  }
}

impl StmtVisitor for Interpreter {
  type Value = ();

  fn visit_expression_statement(&mut self, stmt: &Expression) {
    self.evaluate(stmt.clone().expr()).ok();
  }

  fn visit_if_statement(&mut self, stmt: &If) {
    let condition = self.evaluate(stmt.condition()).unwrap();
    let is_truthy = self.is_truthy(condition);
    match is_truthy {
        LoxValue::Boolean(true) => self.execute(stmt.then_branch()),
        _ => {
          if let Some(else_branch) = stmt.else_branch() {
              self.execute(else_branch);
          }
        }
    }
  }

  fn visit_print_statement(&mut self, stmt: &Print) {
    let value = self.evaluate(stmt.clone().expr());
    println!("{}", value.unwrap());
  }

  fn visit_return_statement(&mut self, stmt: &Return) {
    unimplemented!()
  }

  fn visit_var_statement(&mut self, stmt: &Var) {
    let mut value = LoxValue::Nil;
    if let Ok(initializer) = self.evaluate(stmt.initializer()) {
      value = initializer;
    }
    let mut env_ref = self.environment.borrow_mut();
    env_ref.define(stmt.name().lexeme(), value);
  }

  fn visit_block_statement(&mut self, stmt: &Block) {
    let env_ref = Rc::clone(&self.environment);
    self.execute_block(stmt.statements(), Rc::new(RefCell::new(Environment::new_with(env_ref))));
  }

  fn visit_while_statement(&mut self, stmt: &While) {
    let truth = LoxValue::Boolean(true);
    while self.evaluate(stmt.condition()).unwrap() == truth {
      self.execute(stmt.body())
    }
  }

  fn visit_function_statement(&mut self, stmt: &Function) {
    let function = LoxFunction::new(stmt.clone());
    let mut env_ref = self.environment.borrow_mut();
    env_ref.define(stmt.name().lexeme(), LoxValue::Function(function));
  }
}

impl Interpreter {
  fn is_truthy(&self, result: LoxValue) -> LoxValue {
      match result {
        LoxValue::Nil => LoxValue::Boolean(false),
        LoxValue::Number(_) => LoxValue::Boolean(true),
        LoxValue::String(_) => LoxValue::Boolean(true),
        LoxValue::Boolean(value) => LoxValue::Boolean(value),
        LoxValue::Function(_) => LoxValue::Boolean(true)
      }
  }

  pub fn execute_block(&mut self, statements: Vec<Box<dyn Stmt>>, environment: Rc<RefCell<Environment>>) {
    let previous = Rc::clone(&self.environment);

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