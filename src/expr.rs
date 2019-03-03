use crate::token::Token;

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
    pub fn new(value: T) -> Literal<T> {
        Literal { value: value }
    }
}

impl<T: std::fmt::Display> Expr for Literal<T>  {}

impl<T: std::fmt::Display> std::fmt::Display for Literal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub struct Unary { 
    operator: Token,
    right: Box<dyn Expr>
}

impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expr>) -> Unary {
        Unary { operator, right }
    }
}

impl Expr for Unary {
    fn print(&self) {
        println!("{}", self);
    }
}

impl std::fmt::Display for Unary {
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
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Binary {
        Binary { left, operator, right }
    }
}

impl Expr for Binary {
    fn print(&self) {
        println!("{}", self);
    }
}

impl std::fmt::Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({} {} {})", self.operator, self.left, self.right)
    }
}

pub struct Grouping {
    expression: Box<dyn Expr>
}

impl Grouping {
    pub fn new(expression: Box<dyn Expr>) -> Grouping {
        Grouping { expression }
    }
}

impl Expr for Grouping {
    fn print(&self) {
        println!("{}", self);
    }
}

impl std::fmt::Display for Grouping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(group {})", self.expression)
    }
}