use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

use crate::environment::Environment;
use crate::expr::{
    Assign, Binary, BoxedExpr, Call, Grouping, Literal, Logical, Unary, Variable,
    Visitor as ExprVisitor,
};
use crate::lox_callable::LoxCallable;
use crate::lox_error::LoxError;
use crate::lox_function::LoxFunction;
use crate::lox_value::LoxValue;
use crate::stmt::{
    Block, Expression, Function, If, Print, Return, Stmt, Var, Visitor as StmtVisitor, While,
};
use crate::token::{Token, TokenType};

#[derive(Clone)]
pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
    locals: HashMap<Uuid, usize>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Rc::new(RefCell::new(Environment::new())),
            locals: HashMap::new(),
        }
    }

    pub fn environment(&self) -> Rc<RefCell<Environment>> {
        Rc::clone(&self.environment)
    }

    pub fn interpret(&mut self, statements: Vec<Box<dyn Stmt>>) {
        for statement in statements {
            self.execute(statement);
        }
    }

    pub fn execute(&mut self, stmt: Box<dyn Stmt>) -> Option<LoxValue> {
        stmt.accept(self)
    }

    pub fn evaluate(&mut self, expr: BoxedExpr) -> Result<LoxValue, LoxError> {
        expr.accept(self)
    }

    pub fn resolve(&mut self, expr: BoxedExpr, depth: usize) {
        self.locals.insert(expr.id(), depth);
    }

    fn look_up_variable(&mut self, name: Token, expr: &Variable) -> Result<LoxValue, LoxError> {
        let distance = self.locals.get(&expr.id());
        if let Some(dist) = distance {
            let mut env_ref = self.environment.borrow_mut();
            let value = env_ref.get_at(*dist, expr.name().lexeme());
            Ok(value)
        } else {
            let mut env_ref = self.environment.borrow_mut();
            let value = env_ref.get(expr.name());
            Ok(value)
        }
    }
}

impl ExprVisitor for Interpreter {
    type Value = LoxValue;

    fn visit_nil_literal(&mut self, expr: &Literal<LoxValue>) -> Result<Self::Value, LoxError> {
        Ok(expr.value())
    }

    fn visit_number_literal(&mut self, expr: &Literal<LoxValue>) -> Result<Self::Value, LoxError> {
        Ok(expr.value())
    }

    fn visit_string_literal(&mut self, expr: &Literal<LoxValue>) -> Result<Self::Value, LoxError> {
        Ok(expr.value())
    }

    fn visit_boolean_literal(&mut self, expr: &Literal<LoxValue>) -> Result<Self::Value, LoxError> {
        Ok(expr.value())
    }

    fn visit_logical(&mut self, expr: &Logical) -> Result<Self::Value, LoxError> {
        let left = self.evaluate(expr.left()).unwrap();
        let is_truthy = self.is_truthy(left.clone());

        if expr.operator().token_type() == TokenType::Or {
            match is_truthy {
                LoxValue::Boolean(true) => return Ok(left),
                _ => self.evaluate(expr.right()),
            }
        } else {
            match is_truthy {
                LoxValue::Boolean(false) => return Ok(left),
                _ => self.evaluate(expr.right()),
            }
        }
    }

    fn visit_call(&mut self, expr: &Call) -> Result<Self::Value, LoxError> {
        let callee = self.evaluate(expr.callee());

        let arguments: Vec<LoxValue> = expr
            .arguments()
            .iter()
            .map(|argument| self.evaluate(argument.clone()).unwrap())
            .collect();

        match callee {
            Ok(callee_value) => match callee_value {
                LoxValue::Function(function) => function.call(self, arguments),
                _ => Ok(callee_value),
            },
            _ => panic!("Could not evaluate function arguments"),
        }
    }

    fn visit_unary(&mut self, expr: &Unary) -> Result<Self::Value, LoxError> {
        let right = self.evaluate(expr.right())?;

        match expr.clone().operator().token_type() {
            TokenType::Minus => -right,
            TokenType::Bang => !right,
            _ => Ok(LoxValue::Number(0.0)),
        }
    }

    fn visit_binary(&mut self, expr: &Binary) -> Result<Self::Value, LoxError> {
        let left = self.evaluate(expr.left())?;
        let right = self.evaluate(expr.right())?;

        match expr.clone().operator().token_type() {
            TokenType::Minus => Ok(left - right),
            TokenType::Slash => Ok(left / right),
            TokenType::Star => Ok(left * right),
            TokenType::Plus => left + right,
            TokenType::Greater => Ok(LoxValue::Boolean(left > right)),
            TokenType::GreaterEqual => Ok(LoxValue::Boolean(left >= right)),
            TokenType::Less => Ok(LoxValue::Boolean(left < right)),
            TokenType::LessEqual => Ok(LoxValue::Boolean(left <= right)),
            TokenType::EqualEqual => Ok(LoxValue::Boolean(left == right)),
            TokenType::BangEqual => Ok(LoxValue::Boolean(left != right)),
            _ => Ok(LoxValue::Number(0.0)),
        }
    }

    fn visit_grouping(&mut self, expr: &Grouping) -> Result<Self::Value, LoxError> {
        self.evaluate(expr.expression())
    }

    fn visit_variable(&mut self, expr: &Variable) -> Result<Self::Value, LoxError> {
        self.look_up_variable(expr.name(), expr)
    }

    fn visit_assignment(&mut self, expr: &Assign) -> Result<Self::Value, LoxError> {
        let value = self.evaluate(expr.value()).unwrap();
        let distance = self.locals.get(&expr.id());
        if let Some(dist) = distance {
            let mut env_ref = self.environment.borrow_mut();
            env_ref.assign_at(*dist, expr.name().lexeme(), value.clone());
            Ok(value)
        } else {
            let mut env_ref = self.environment.borrow_mut();
            env_ref.assign(expr.name(), value.clone());
            Ok(value)
        }
    }
}

impl StmtVisitor for Interpreter {
    type Value = Option<LoxValue>;

    fn visit_expression_statement(&mut self, stmt: &Expression) -> Option<LoxValue> {
        self.evaluate(stmt.clone().expr()).ok();
        None
    }

    fn visit_if_statement(&mut self, stmt: &If) -> Option<LoxValue> {
        let condition = self.evaluate(stmt.condition()).unwrap();
        let is_truthy = self.is_truthy(condition);
        match is_truthy {
            LoxValue::Boolean(true) => self.execute(stmt.then_branch()),
            _ => {
                if let Some(else_branch) = stmt.else_branch() {
                    self.execute(else_branch);
                }
                None
            }
        }
    }

    fn visit_print_statement(&mut self, stmt: &Print) -> Option<LoxValue> {
        let value = self.evaluate(stmt.clone().expr());
        println!("{}", value.unwrap());
        None
    }

    fn visit_return_statement(&mut self, stmt: &Return) -> Option<LoxValue> {
        if let Ok(value) = self.evaluate(stmt.value()) {
            Some(value)
        } else {
            None
        }
    }

    fn visit_var_statement(&mut self, stmt: &Var) -> Option<LoxValue> {
        let mut value = LoxValue::Nil;
        if let Some(initializer) = stmt.initializer() {
            if let Ok(initializer) = self.evaluate(initializer) {
                value = initializer;
            }
        }
        let mut env_ref = self.environment.borrow_mut();
        env_ref.define(stmt.name().lexeme(), value);
        None
    }

    fn visit_block_statement(&mut self, stmt: &Block) -> Option<LoxValue> {
        let env_ref = Rc::clone(&self.environment);
        self.execute_block(
            stmt.statements(),
            Rc::new(RefCell::new(Environment::new_with(env_ref))),
        );
        None
    }

    fn visit_while_statement(&mut self, stmt: &While) -> Option<LoxValue> {
        let truth = LoxValue::Boolean(true);
        while self.evaluate(stmt.condition()).unwrap() == truth {
            self.execute(stmt.body());
        }
        None
    }

    fn visit_function_statement(&mut self, stmt: &Function) -> Option<LoxValue> {
        let function = LoxFunction::new(stmt.clone(), Rc::clone(&self.environment));
        let mut env_ref = self.environment.borrow_mut();
        env_ref.define(stmt.name().lexeme(), LoxValue::Function(function));
        None
    }
}

impl Interpreter {
    fn is_truthy(&self, result: LoxValue) -> LoxValue {
        match result {
            LoxValue::Nil => LoxValue::Boolean(false),
            LoxValue::Number(_) => LoxValue::Boolean(true),
            LoxValue::String(_) => LoxValue::Boolean(true),
            LoxValue::Boolean(value) => LoxValue::Boolean(value),
            LoxValue::Function(_) => LoxValue::Boolean(true),
        }
    }

    pub fn execute_block(
        &mut self,
        statements: Vec<Box<dyn Stmt>>,
        environment: Rc<RefCell<Environment>>,
    ) -> Option<LoxValue> {
        let previous = Rc::clone(&self.environment);
        let mut value = None;

        self.environment = environment;

        for statement in statements {
            if let Some(return_value) = self.execute(statement) {
                value = Some(return_value);
                break;
            };
        }

        self.environment = previous;
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Literal, Unary};
    use crate::token::{Token, TokenType};

    #[test]
    fn it_evaluates_numeric_literals() {
        let expr = Literal::new(LoxValue::Number(5.0f32));
        let mut interpreter = Interpreter::new();
        assert_eq!(interpreter.evaluate(expr).unwrap(), LoxValue::Number(5.0));
    }

    #[test]
    fn it_evaluates_string_literals() {
        let expr = Literal::new(LoxValue::String("A string".to_string()));
        let mut interpreter = Interpreter::new();
        assert_eq!(
            interpreter.evaluate(expr).unwrap(),
            LoxValue::String("A string".to_string())
        )
    }

    #[test]
    fn it_evaluates_unary_minus_operators() {
        let expr = Unary::new(
            Token::new("-".to_string(), TokenType::Minus),
            Literal::new(LoxValue::Number(5.0)),
        );
        let mut interpreter = Interpreter::new();
        assert_eq!(interpreter.evaluate(expr).unwrap(), LoxValue::Number(-5.0))
    }
}
