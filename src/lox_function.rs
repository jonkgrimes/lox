use std::rc::Rc;
use std::cell::RefCell;

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
      environment.define(name, *argument);
    }

    interpreter.execute_block(self.declaration.body, environment)
  }
}


