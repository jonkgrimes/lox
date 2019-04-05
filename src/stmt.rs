use crate::expr::{BoxedExpr};
use crate::token::Token;

pub type BoxedStmt = Box<dyn Stmt>;

pub trait Stmt: CloneableStmt
where Self: Visitable {
}

pub trait CloneableStmt {
    fn clone_box(&self) -> BoxedStmt;
}

impl<T> CloneableStmt for T
where
    T: 'static + Stmt + Clone,
{
    fn clone_box(&self) -> BoxedStmt {
        Box::new(self.clone())
    }

}

impl Clone for Box<dyn Stmt> {
    fn clone(&self) -> Box<dyn Stmt> {
        self.clone_box()
    }
}

pub trait Visitor {
  type Value;

  fn visit_print_statement(&mut self, stmt: &Print) -> Self::Value;
  fn visit_expression_statement(&mut self, stmt: &Expression) -> Self::Value;
  fn visit_var_statement(&mut self, stmt: &Var) -> Self::Value;
  fn visit_block_statement(&mut self, stmt: &Block) -> Self::Value;
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

  pub fn name(&self) -> Token {
    self.name.clone()
  }

  pub fn initializer(&self) -> BoxedExpr {
    self.initializer.clone()
  }
}

impl Visitable for Var {
  fn accept(&self, visitor: &mut Visitor<Value=()>) {
    visitor.visit_var_statement(self)
  }
}

#[derive(Clone)]
pub struct Block {
  statements: Vec<BoxedStmt>
}

impl Stmt for Block {}

impl Visitable for Block {
  fn accept(&self, visitor: &mut Visitor<Value=()>) {
    visitor.visit_block_statement(self)
  }
}

impl Block {
  pub fn new(statements: Vec<BoxedStmt>) -> BoxedStmt {
      Box::new(Block { statements })
  }

  pub fn statements(&self) -> Vec<BoxedStmt> {
      self.statements.clone()
  }
}