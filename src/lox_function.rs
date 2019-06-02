use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::PartialEq;

use crate::stmt::{Function};
use crate::lox_callable::LoxCallable;
use crate::lox_value::LoxValue;
use crate::lox_error::LoxError;
use crate::environment::Environment;
use crate::interpreter::Interpreter;

#[derive(Debug, Clone)]
pub struct LoxFunction {
  declaration: Function,
  closure: Rc<RefCell<Environment>>
}

impl LoxCallable for LoxFunction {
  fn arity() {
  }

  fn call(self, interpreter: &mut Interpreter, arguments: Vec<LoxValue>) -> Result<LoxValue, LoxError> {
    let environment = Rc::new(RefCell::new(Environment::new_with(self.closure)));

    for (i, param) in self.declaration.params().iter().enumerate() {
      let name = param.lexeme();
      let argument = arguments.get(i).unwrap();
      let mut env_ref = environment.borrow_mut();
      env_ref.define(name, argument.clone());
    }

    match interpreter.execute_block(self.declaration.body(), environment) {
      Some(value) => {
        Ok(value)
      },
      None => {
        Ok(LoxValue::Nil)
      }
    }
  }
}

impl PartialEq for LoxFunction {
    fn eq(&self, _other: &LoxFunction) -> bool {
      false
    }
}

impl LoxFunction {
  pub fn new(declaration: Function, closure: Rc<RefCell<Environment>>) -> LoxFunction {
    LoxFunction { declaration, closure }
  }

  pub fn name(&self) -> String {
    self.declaration.name().lexeme()
  }
}