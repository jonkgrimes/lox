use std::collections::{HashMap};
use std::rc::Rc;
use std::cell::RefCell;
use crate::interpreter::Interpreter;
use crate::lox_value::LoxValue;
use crate::lox_error::LoxError;
use crate::token::Token;
use crate::expr::{Visitor as ExprVisitor, BoxedExpr, Literal, Grouping, Unary, Binary, Variable, Assign, Logical, Call};
use crate::stmt::{Visitor as StmtVisitor, BoxedStmt, Expression, Print, Var, Block, If, While, Function, Return};

pub struct Resolver<'a> {
  interpreter: &'a mut Interpreter,
  scopes: Vec<Rc<RefCell<HashMap<String, bool>>>>
}

impl<'a> Resolver<'a> {
  pub fn new(interpreter: &'a mut Interpreter) -> Resolver {
    Resolver { interpreter, scopes: Vec::new() }
  }

  pub fn resolve(&mut self, statments: Vec<BoxedStmt>) {
    for statement in statments {
      self.resolve_stmt(statement)
    }
  }

  fn resolve_stmt(&mut self, stmt: BoxedStmt) {
    stmt.accept(self);
  }

  fn resolve_expr(&mut self, expr: BoxedExpr) {
    expr.accept(self);
  }

  fn begin_scope(&mut self) {
    self.scopes.push(Rc::new(RefCell::new(HashMap::new())))
  }

  fn end_scope(&mut self) {
    self.scopes.pop();
  }

  fn declare(&mut self, name: Token) {
      if self.scopes.len() == 0 {
        return
      }

      if let Some(scope_ref) = self.scopes.last() {
        let mut scope = scope_ref.borrow_mut();
        scope.insert(name.lexeme(), false);
      }
  }

  fn define(&mut self, name: Token) {
      if self.scopes.len() == 0 {
        return
      }

      if let Some(scope_ref) = self.scopes.last() {
        let mut scope = scope_ref.borrow_mut();
        scope.insert(name.lexeme(), true);
      }
  }

  fn resolve_local(&mut self, expr: BoxedExpr, name: String) {
      for (index, scope_ref) in self.scopes.iter().rev().enumerate() {
          let mut scope = scope_ref.borrow();
          if scope.contains_key(&name) {
              self.interpreter.resolve(expr.clone(), index);
          }
      }
  }
}

impl<'a> ExprVisitor for Resolver<'a> {
  type Value = LoxValue;

  fn visit_nil_literal(&mut self, expr: &Literal<LoxValue>) -> Result<Self::Value, LoxError> {
    Ok(LoxValue::Nil)
  }

  fn visit_number_literal(&mut self, expr: &Literal<LoxValue>) -> Result<Self::Value, LoxError> {
    Ok(LoxValue::Nil)
  }

  fn visit_string_literal(&mut self, expr: &Literal<LoxValue>) -> Result<Self::Value, LoxError> {
    Ok(LoxValue::Nil)
  }

  fn visit_boolean_literal(&mut self, expr: &Literal<LoxValue>) -> Result<Self::Value, LoxError> {
    Ok(LoxValue::Nil)
  }

  fn visit_logical(&mut self, expr: &Logical) -> Result<Self::Value, LoxError> {
    Ok(LoxValue::Nil)
  }

  fn visit_call(&mut self, expr: &Call) -> Result<Self::Value, LoxError> {
    Ok(LoxValue::Nil)
  }

  fn visit_unary(&mut self, expr: &Unary) -> Result<Self::Value, LoxError> {
    Ok(LoxValue::Nil)
  }

  fn visit_binary(&mut self, expr: &Binary) -> Result<Self::Value, LoxError> {
    Ok(LoxValue::Nil)
  }

  fn visit_grouping(&mut self, expr: &Grouping) -> Result<Self::Value, LoxError> {
    Ok(LoxValue::Nil)
  }

  fn visit_variable(&mut self, expr: &Variable) -> Result<Self::Value, LoxError> {
    let name = expr.name().lexeme();
    if self.scopes.len() == 0 && self.scopes.last().unwrap().borrow().get(&name) == Some(&false) {
        let error = LoxError::BindingError(name, "Cannot read local vairable in its own initializer.".to_string());
        return Err(error)
    }

    self.resolve_local(Box::new(expr.clone()), name);

    Ok(LoxValue::Nil)
  }

  fn visit_assignment(&mut self, expr: &Assign) -> Result<Self::Value, LoxError> {
    Ok(LoxValue::Nil)
  }
}

impl<'a> StmtVisitor for Resolver<'a> {
  type Value = Option<LoxValue>;

  fn visit_expression_statement(&mut self, stmt: &Expression) -> Option<LoxValue> {
    None
  }

  fn visit_if_statement(&mut self, stmt: &If) -> Option<LoxValue> {
    None
  }

  fn visit_print_statement(&mut self, stmt: &Print) -> Option<LoxValue> {
    None
  }

  fn visit_return_statement(&mut self, stmt: &Return) -> Option<LoxValue> {
    None
  }

  fn visit_var_statement(&mut self, stmt: &Var) -> Option<LoxValue> {
    self.declare(stmt.name());
    if let Some(initializer) = stmt.initializer() {
      self.resolve_expr(initializer);
    }
    self.define(stmt.name());
    None
  }

  fn visit_block_statement(&mut self, stmt: &Block) -> Option<LoxValue> {
    self.begin_scope();
    self.resolve(stmt.statements());
    self.end_scope();
    None
  }

  fn visit_while_statement(&mut self, stmt: &While) -> Option<LoxValue> {
    None
  }

  fn visit_function_statement(&mut self, stmt: &Function) -> Option<LoxValue> {
    None
  }
}