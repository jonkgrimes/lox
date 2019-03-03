use crate::token::Token;

pub trait Expr 
where Self: std::fmt::Display
{
    fn print(&self);
}

pub struct Literal<T> {
    value: T
}

impl Literal<f32> {
    pub fn new(value: f32) -> Literal<f32> {
        Literal { value: value }
    }
}

impl Expr for Literal<f32>  {
    fn print(&self) {
        println!("{}", self);
    }
}

impl std::fmt::Display for Literal<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub struct Unary { 
    operator: Token,
    right: Box<dyn Expr>
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
    expression: Expr,
}