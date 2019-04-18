use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::PartialEq;

use crate::stmt::{BoxedStmt, Function};
use crate::lox_callable::LoxCallable;
use crate::lox_value::LoxValue;
use crate::lox_error::LoxError;
use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::expr::BoxedExpr;

#[derive(Debug, Clone)]
pub struct LoxFunction {
  declaration: Function
}

impl LoxCallable for LoxFunction {
  fn arity() {
  }

  fn call(self, interpreter: &mut Interpreter, arguments: Vec<LoxValue>) -> Result<LoxValue, LoxError> {
    let environment = Rc::new(RefCell::new(Environment::new())); 

    for (i, param) in self.declaration.params().iter().enumerate() {
      let name = param.lexeme();
      let argument = arguments.get(i).unwrap();
      let mut env_ref = environment.borrow_mut();
      env_ref.define(name, argument.clone());
    }

    interpreter.execute_block(self.declaration.body(), environment);

    Ok(LoxValue::Nil)
  }
}

impl PartialEq for LoxFunction {
    fn eq(&self, other: &LoxFunction) -> bool {
      false
    }
}

impl LoxFunction {
  pub fn new(declaration: Function) -> LoxFunction {
    LoxFunction { declaration }
  }
}