use std::fmt::Display;

use crate::token::Token;
use crate::lox_value::LoxValue;
use crate::lox_error::LoxError;

pub type BoxedExpr = Box<dyn Expr>;
pub type LoxResult = Result<LoxValue, LoxError>;

pub trait Expr: CloneableExpr
where Self: std::fmt::Display,
      Self: Visitable
{
    fn print(&self) {
        println!("{}", self);
    }
}

pub trait CloneableExpr {
    fn clone_box(&self) -> BoxedExpr;
}

impl<T> CloneableExpr for T
where
    T: 'static + Expr + Clone,
{
    fn clone_box(&self) -> Box<dyn Expr> {
        Box::new(self.clone())
    }

}

impl Clone for Box<dyn Expr> {
    fn clone(&self) -> Box<dyn Expr> {
        self.clone_box()
    }
}

pub trait Visitable
{
    fn accept(&self, visitor: &mut Visitor<Value=LoxValue>) -> LoxResult;
}

pub trait Visitor {
    type Value;

    fn visit_nil_literal(&mut self, expr: &Literal<LoxValue>) -> Result<Self::Value, LoxError>;
    fn visit_string_literal(&mut self, expr: &Literal<String>) -> Result<Self::Value, LoxError>;
    fn visit_number_literal(&mut self, expr: &Literal<f32>) -> Result<Self::Value, LoxError>;
    fn visit_boolean_literal(&mut self, expr: &Literal<bool>) -> Result<Self::Value, LoxError>;
    fn visit_unary(&mut self, expr: &Unary) -> Result<Self::Value, LoxError>;
    fn visit_binary(&mut self, expr: &Binary) -> Result<Self::Value, LoxError>;
    fn visit_grouping(&mut self, expr: &Grouping) -> Result<Self::Value, LoxError>;
}

#[derive(Clone)]
pub struct Literal<T> {
    value: T
}

impl<T: Clone> Literal<T> {
    pub fn new(value: T) -> Box<Literal<T>> {
        Box::new(Literal { value: value })
    }

    pub fn value(&self) -> T {
        self.value.clone()
    }
}

impl Expr for Literal<f32> {}
impl Expr for Literal<String> {}
impl Expr for Literal<bool> {}
impl Expr for Literal<LoxValue> {}
// impl<T: Display> Expr for Literal<T> {}

impl Visitable for Literal<String>  {
    fn accept(&self, visitor: &mut Visitor<Value=LoxValue>) -> LoxResult {
        visitor.visit_string_literal(self)
    }
}

impl Visitable for Literal<f32>  {
    fn accept(&self, visitor: &mut Visitor<Value=LoxValue>) -> LoxResult {
        visitor.visit_number_literal(self)
    }
}

impl Visitable for Literal<bool>  {
    fn accept(&self, visitor: &mut Visitor<Value=LoxValue>) -> LoxResult {
        visitor.visit_boolean_literal(self)
    }
}

impl Visitable for Literal<LoxValue> {
    fn accept(&self, visitor: &mut Visitor<Value=LoxValue>) -> LoxResult {
        match self.value() {
            LoxValue::Nil => visitor.visit_nil_literal(self),
            _ => panic!("Panic!!!")
        }
    }
}

impl<T: Display> Display for Literal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Clone)]
pub struct Unary { 
    operator: Token,
    right: Box<dyn Expr>
}

impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expr>) -> Box<Unary> {
        Box::new(Unary { operator, right })
    }

    pub fn operator(self) -> Token {
        self.operator
    }

    pub fn right(&self) -> BoxedExpr {
        self.right.clone()
    }
}

impl Expr for Unary {
}

impl Visitable for Unary {
    fn accept(&self, visitor: &mut Visitor<Value=LoxValue>) -> LoxResult {
        visitor.visit_unary(self)
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({} {})", self.operator, self.right)
    }
}

#[derive(Clone)]
pub struct Binary {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>
}

impl Binary {
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Box<Binary> {
        Box::new(Binary { left, operator, right })
    }

    pub fn operator(self) -> Token {
        self.operator
    }

    pub fn left(&self) -> BoxedExpr {
        self.left.clone()
    }

    pub fn right(&self) -> BoxedExpr {
        self.right.clone()
    }
}

impl Expr for Binary {
}

impl Visitable for Binary {
    fn accept(&self, visitor: &mut dyn Visitor<Value=LoxValue>) -> LoxResult {
        visitor.visit_binary(self)
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({} {} {})", self.operator, self.left, self.right)
    }
}

#[derive(Clone)]
pub struct Grouping {
    expression: Box<dyn Expr>
}

impl Grouping {
    pub fn new(expression: Box<dyn Expr>) -> Box<Grouping> {
        Box::new(Grouping { expression })
    }

    pub fn expression(&self) -> BoxedExpr {
        self.expression.clone()
    }
}

impl Expr for Grouping {
}

impl Visitable for Grouping {
    fn accept(&self, visitor: &mut Visitor<Value=LoxValue>) -> LoxResult {
        visitor.visit_grouping(self)
    }
}

impl Display for Grouping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(group {})", self.expression)
    }
}