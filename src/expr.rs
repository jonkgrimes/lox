use std::fmt::Display;
use std::any::Any;

use crate::token::Token;
use crate::lox_value::LoxValue;
use crate::lox_error::LoxError;

pub type BoxedExpr = Box<dyn Expr>;
pub type LoxResult = Result<LoxValue, LoxError>;

pub trait Expr: CloneableExpr
where Self: std::fmt::Display,
      Self: Visitable
{
    fn as_any(&self) -> &dyn Any;

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
    fn visit_string_literal(&mut self, expr: &Literal<LoxValue>) -> Result<Self::Value, LoxError>;
    fn visit_number_literal(&mut self, expr: &Literal<LoxValue>) -> Result<Self::Value, LoxError>;
    fn visit_boolean_literal(&mut self, expr: &Literal<LoxValue>) -> Result<Self::Value, LoxError>;
    fn visit_unary(&mut self, expr: &Unary) -> Result<Self::Value, LoxError>;
    fn visit_binary(&mut self, expr: &Binary) -> Result<Self::Value, LoxError>;
    fn visit_grouping(&mut self, expr: &Grouping) -> Result<Self::Value, LoxError>;
    fn visit_variable(&mut self, expr: &Variable) -> Result<Self::Value, LoxError>;
    fn visit_assignment(&mut self, expr: &Assign) -> Result<Self::Value, LoxError>;
    fn visit_logical(&mut self, expr: &Logical) -> Result<Self::Value, LoxError>;
    fn visit_call(&mut self, expr: &Call) -> Result<Self::Value, LoxError>;
}

#[derive(Clone)]
pub struct Literal<T> {
    value: T
}

impl<T: Clone> Literal<T> {
    pub fn new(value: T) -> Box<Literal<T>> {
        Box::new(Literal { value })
    }

    pub fn value(&self) -> T {
        self.value.clone()
    }
}

impl Expr for Literal<LoxValue> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Visitable for Literal<LoxValue> {
    fn accept(&self, visitor: &mut Visitor<Value=LoxValue>) -> LoxResult {
        match self.value() {
            LoxValue::Nil => visitor.visit_nil_literal(self),
            LoxValue::Number(_number) => visitor.visit_number_literal(self),
            LoxValue::String(_string) => visitor.visit_string_literal(self),
            LoxValue::Boolean(_boolean) => visitor.visit_boolean_literal(self)
        }
    }
}

impl<T: Display> Display for Literal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Literal<LoxValue> {
    pub fn nil() -> Box<Literal<LoxValue>> {
        Box::new(Literal { value: LoxValue::Nil })
    }
}

#[derive(Clone)]
pub struct Logical {
    left: BoxedExpr,
    operator: Token,
    right: BoxedExpr
}

impl Expr for Logical {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Visitable for Logical {
    fn accept(&self, visitor: &mut Visitor<Value=LoxValue>) -> LoxResult {
        visitor.visit_logical(self)
    }
}

impl Display for Logical {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

impl Logical {
    pub fn new(left: BoxedExpr, operator: Token, right: BoxedExpr) -> Box<Logical> {
        Box::new(Logical { left, operator, right })
    }

    pub fn left(&self) -> BoxedExpr {
        self.left.clone()
    }

    pub fn operator(&self) -> Token {
        self.operator.clone()
    }

    pub fn right(&self) -> BoxedExpr {
        self.right.clone()
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
    fn as_any(&self) -> &dyn Any {
        self
    }
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
    fn as_any(&self) -> &dyn Any {
        self
    }
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
    fn as_any(&self) -> &dyn Any {
        self
    }
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

#[derive(Clone)]
pub struct Variable {
   name: Token
}

impl Expr for Variable {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Variable {
    pub fn new(name: Token) -> Box<Variable> {
        Box::new(Variable { name })
    }

    pub fn name(&self) -> Token {
        self.name.clone()
    }
}

impl Visitable for Variable {
    fn accept(&self, visitor: &mut Visitor<Value=LoxValue>) -> LoxResult {
        visitor.visit_variable(self)
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "var {}", self.name)
    }
}

#[derive(Clone)]
pub struct Assign {
    name: Token,
    value: BoxedExpr
}


impl Expr for Assign {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Visitable for Assign {
    fn accept(&self, visitor: &mut Visitor<Value=LoxValue>) -> LoxResult {
        visitor.visit_assignment(self)
    }
}

impl Display for Assign {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "var {} = {}", self.name, self.value)
    }
}

impl Assign {
    pub fn new(name: Token, value: BoxedExpr) -> Box<Assign> {
        Box::new(Assign { name, value })
    }

    pub fn name(&self) -> Token {
        self.name.clone()
    }

    pub fn value(&self) -> BoxedExpr {
        self.value.clone()
    }
}

#[derive(Clone)]
pub struct Call {
    callee: BoxedExpr,
    paren: Token,
    arguments: Vec<BoxedExpr>,
}

impl Expr for Call {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Visitable for Call {
    fn accept(&self, visitor: &mut Visitor<Value=LoxValue>) -> LoxResult {
        visitor.visit_call(self)
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "call {}", self.callee)
    }
}

impl Call {
    pub fn new(callee: BoxedExpr, paren: Token, arguments: Vec<BoxedExpr>) -> Box<Call> {
        Box::new(Call { callee, paren, arguments })
    }

    pub fn callee(&self) -> BoxedExpr {
        self.callee.clone()
    }

    pub fn paren(&self) -> Token {
        self.paren.clone()
    }

    pub fn arguments(&self) -> Vec<BoxedExpr> {
        self.arguments.clone()
    }
}