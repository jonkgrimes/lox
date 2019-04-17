use std::fmt::Debug;

use crate::expr::{BoxedExpr};
use crate::token::Token;

pub type BoxedStmt = Box<dyn Stmt>;

pub trait Stmt: CloneableStmt
where Self: Visitable,
      Self: Debug
{
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
  fn visit_if_statement(&mut self, stmt: &If) -> Self::Value;
  fn visit_while_statement(&mut self, stmt: &While) -> Self::Value;
  fn visit_function_statement(&mut self, stmt: &Function) -> Self::Value;
}

pub trait Visitable
{
    fn accept(&self, visitor: &mut Visitor<Value=()>);
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct If {
  condition: BoxedExpr,
  then_branch: BoxedStmt,
  else_branch: Option<BoxedStmt>
}

impl Stmt for If {}

impl Visitable for If {
  fn accept(&self, visitor: &mut Visitor<Value=()>) {
    visitor.visit_if_statement(self)
  }
}

impl If {
  pub fn new(condition: BoxedExpr, then_branch: BoxedStmt, else_branch: Option<BoxedStmt>)  -> BoxedStmt {
    Box::new(If { condition, then_branch, else_branch })
  }

  pub fn condition(&self) -> BoxedExpr {
    self.condition.clone()
  }

  pub fn then_branch(&self) -> BoxedStmt {
    self.then_branch.clone()
  }

  pub fn else_branch(&self) -> Option<BoxedStmt> {
    self.else_branch.clone()
  }
}

#[derive(Debug, Clone)]
pub struct While {
  condition: BoxedExpr,
  body: BoxedStmt
}

impl Stmt for While {}

impl Visitable for While {
  fn accept(&self, visitor: &mut Visitor<Value=()>) {
    visitor.visit_while_statement(self)
  }
}

impl While {
  pub fn new(condition: BoxedExpr, body: BoxedStmt)  -> BoxedStmt {
    Box::new(While { condition, body })
  }

  pub fn condition(&self) -> BoxedExpr {
    self.condition.clone()
  }

  pub fn body(&self) -> BoxedStmt {
    self.body.clone()
  }
}

#[derive(Debug, Clone)]
pub struct Function {
  name: Token,
  params: Vec<Token>,
  body: Vec<BoxedStmt>,
}

impl Stmt for Function {}

impl Visitable for Function {
  fn accept(&self, visitor: &mut Visitor<Value=()>) {
    visitor.visit_function_statement(self)
  }
}

impl Function {
  pub fn new(name: Token, params: Vec<Token>, body: Vec<BoxedStmt>)  -> BoxedStmt {
    Box::new(Function { name, params, body })
  }

  pub fn name(&self) -> Token {
    self.name.clone()
  }

  pub fn params(&self) -> Vec<Token> {
    self.params.clone()
  }

  pub fn body(&self) -> Vec<BoxedStmt> {
    self.body.clone()
  }
}

