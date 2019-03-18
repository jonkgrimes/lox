use std::fmt::Display;

use crate::token::Token;

pub type BoxedExpr = Box<dyn Expr>;

pub trait Expr 
where Self: std::fmt::Display
{
    fn print(&self) {
        println!("{}", self);
    }
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

impl<T: Display> Expr for Literal<T>  {}

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
}

impl Expr for Unary {}

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

impl Expr for Binary {}

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

impl Expr for Grouping {}

impl Display for Grouping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(group {})", self.expression)
    }
}