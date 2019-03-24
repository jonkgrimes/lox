use std::fmt::Display;
use std::convert::From;

use crate::token::Token;

pub type BoxedExpr = Box<dyn Expr>;

pub trait Expr 
where Self: std::fmt::Display
{
    fn accept<V: Visitor>(self, visitor: &mut V) -> V::Value;
    fn print(&self) {
        println!("{}", self);
    }
}

pub trait Visitor {
    type Value;

    fn visitLiteral(self, expr: Literal<f32>) -> Self::Value;
    fn visitUnary(self, expr: Unary) -> Self::Value;
    // fn visitBinary(self, expr: Binary) -> Self::Value;
    // fn visitGrouping(self, expr: Grouping) -> Self::Value;
}

pub struct Literal<T> {
    value: T
}

impl<T> Literal<T> {
    pub fn new(value: T) -> Box<Literal<T>> {
        Box::new(Literal { value: value })
    }

    pub fn value(self) -> T {
        self.value
    }
}

impl From<f32> for Literal<f32> {
    fn from(item: f32) -> Literal<f32> {
        Literal { value: item }
    }
}

impl<T: Display> Expr for Literal<T>  {
    fn accept(self, visitor: Box<dyn Visitor<Value=f32>>) -> f32 {
        visitor.visitLiteral(self);
    }
}

impl<T: Display> Display for Literal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

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

    pub fn right(self) -> BoxedExpr {
        self.right
    }
}

impl Expr for Unary {
    fn accept(self, visitor: Box<dyn Visitor<Value=f32>>) {
        ()
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({} {})", self.operator, self.right)
    }
}

pub struct Binary {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>
}

impl Binary {
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Box<Binary> {
        Box::new(Binary { left, operator, right })
    }
}

impl Expr for Binary {
    fn accept(self, visitor: Box<dyn Visitor<Value=f32>>) {
        ()
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({} {} {})", self.operator, self.left, self.right)
    }
}

pub struct Grouping {
    expression: Box<dyn Expr>
}

impl Grouping {
    pub fn new(expression: Box<dyn Expr>) -> Box<Grouping> {
        Box::new(Grouping { expression })
    }

    pub fn expression(self) -> BoxedExpr {
        self.expression
    }
}

impl Expr for Grouping {
    fn accept(self, visitor: Box<dyn Visitor<Value=f32>>) {
        ()
    }
}

impl Display for Grouping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(group {})", self.expression)
    }
}