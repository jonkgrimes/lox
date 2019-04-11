use crate::interpreter::Interpreter;
use crate::expr::BoxedExpr;

pub trait LoxCallable {
  fn arity();
  fn call(interpreter: Interpreter, arguments: Vec<BoxedExpr>);
}