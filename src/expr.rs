use std::fmt::Display;
use std::convert::From;

use crate::token::Token;

pub type BoxedExpr = Box<dyn Expr>;

pub trait Expr
where Self: std::fmt::Display,
      Self: Visitable
{
    fn print(&self) {
        println!("{}", self);
    }
}

#[derive(Debug, PartialEq)]
pub enum ExprResult {
    String(String),
    Number(f32),
    Boolean(bool)
}

pub trait Visitable
{
    fn accept(&self, visitor: &mut Visitor<Value=ExprResult>) -> ExprResult;
}

pub trait Visitor {
    type Value;

    // fn visit_string_literal(&mut self, expr: &Literal<String>) -> Self::Value;
    fn visit_number_literal(&mut self, expr: &Literal<f32>) -> Self::Value;
    fn visit_boolean_literal(&mut self, expr: &Literal<bool>) -> Self::Value;
    fn visit_unary(&mut self, expr: &Unary) -> Self::Value;
    fn visit_binary(&mut self, expr: &Binary) -> Self::Value;
    fn visit_grouping(&mut self, expr: &Grouping) -> Self::Value;
}

pub struct Literal<T> {
    value: T
}

impl<T: Copy> Literal<T> {
    pub fn new(value: T) -> Box<Literal<T>> {
        Box::new(Literal { value: value })
    }

    pub fn value(&self) -> T {
        self.value
    }
}

impl Expr for Literal<f32> {}
// impl Expr for Literal<String> {}
impl Expr for Literal<bool> {}
// impl<T: Display> Expr for Literal<T> {}

impl From<f32> for Literal<f32> {
    fn from(item: f32) -> Literal<f32> {
        Literal { value: item }
    }
}

/* impl Visitable for Literal<String>  {
    fn accept(&self, visitor: &mut Visitor<Value=ExprResult>) -> ExprResult {
        visitor.visit_string_literal(self)
    }
} */

impl Visitable for Literal<f32>  {
    fn accept(&self, visitor: &mut Visitor<Value=ExprResult>) -> ExprResult {
        visitor.visit_number_literal(self)
    }
}

impl Visitable for Literal<bool>  {
    fn accept(&self, visitor: &mut Visitor<Value=ExprResult>) -> ExprResult {
        visitor.visit_boolean_literal(self)
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
}

impl Visitable for Unary {
    fn accept(&self, visitor: &mut Visitor<Value=ExprResult>) -> ExprResult {
        visitor.visit_unary(self)
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
}

impl Visitable for Binary {
    fn accept(&self, visitor: &mut dyn Visitor<Value=ExprResult>) -> ExprResult {
        visitor.visit_binary(self)
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
}

impl Visitable for Grouping {
    fn accept(&self, visitor: &mut Visitor<Value=ExprResult>) -> ExprResult {
        visitor.visit_grouping(self)
    }
}

impl Display for Grouping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(group {})", self.expression)
    }
}