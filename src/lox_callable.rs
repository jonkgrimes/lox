use crate::interpreter::Interpreter;
use crate::lox_value::LoxValue;
use crate::lox_error::LoxError;

pub trait LoxCallable {
  fn arity();
  fn call(self, interpreter: &mut Interpreter, arguments: Vec<LoxValue>) -> Result<LoxValue, LoxError>;
}