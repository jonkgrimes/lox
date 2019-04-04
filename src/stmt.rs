use crate::expr::{BoxedExpr};
use crate::token::Token;

pub trait Stmt: CloneableStmt
where Self: Visitable {

}

pub trait CloneableStmt {
    fn clone_box(&self) -> Box<dyn Stmt>;
}

impl<T> CloneableStmt for T
where
    T: 'static + Stmt + Clone,
{
    fn clone_box(&self) -> Box<dyn Stmt> {
        Box::new(self.clone())
    }

}

pub trait Visitor {
  type Value;

  fn visit_print_statement(&mut self, stmt: &Print) -> Self::Value;
  fn visit_expression_statement(&mut self, stmt: &Expression) -> Self::Value;
}

pub trait Visitable
{
    fn accept(&self, visitor: &mut Visitor<Value=()>);
}

#[derive(Clone)]
pub struct Expression {
  expr: BoxedExpr
}

impl Stmt for Expression {}

impl Expression {
  pub fn new(expr: BoxedExpr) -> Box<dyn Stmt> {
    Box::new(Expression { expr })
  }

  pub fn expr(self) -> BoxedExpr {
    self.expr
  }
}

impl Visitable for Expression {
  fn accept(&self, visitor: &mut Visitor<Value=()>) {
    visitor.visit_expression_statement(self)
  }
}

#[derive(Clone)]
pub struct Print {
  expr: BoxedExpr
}

impl Stmt for Print {}

impl Print {
  pub fn new(expr: BoxedExpr) -> Box<dyn Stmt> {
    Box::new(Print { expr })
  }

  pub fn expr(self) -> BoxedExpr {
    self.expr
  }
}

impl Visitable for Print {
  fn accept(&self, visitor: &mut Visitor<Value=()>) {
    visitor.visit_print_statement(self)
  }
}

#[derive(Clone)]
pub struct Var {
    name: Token,
    initializer: BoxedExpr
}

impl Stmt for Var {}

impl Var {
  pub fn new(name: Token, initializer: BoxedExpr) -> Box<dyn Stmt> {
    Box::new(Var { name, initializer })
  }
}

impl Visitable for Var {
  fn accept(&self, visitor: &mut Visitor<Value=()>) {
    ()
  }
}