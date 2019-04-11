use crate::stmt::{BoxedStmt, Function};
use crate::lox_callable::LoxCallable;
use crate::environment::Environment;

pub struct LoxFunction {
  declaration: Function
}

impl LoxCallable for LoxFunction {
  fn call(&self, interpreter: Interpreter, arguments: Vec<BoxedExpr>) {
    let environment = Environment::new(); 

    for param in self.declaration.params.iter().enumerable() {
      let name = param.lexeme();
      let argument = arguments.get
      environment.define(name: String, value: LoxValue)
    }
  }
}


