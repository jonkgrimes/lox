use std::str::FromStr;
use std::error::Error;
use std::fmt;
use std::fmt::Display;

use crate::token::{Token, TokenType};
use crate::expr::{BoxedExpr, Unary, Binary, Literal, Grouping};
use crate::lox_value::LoxValue;
use crate::stmt::{Stmt, Print, Expression};

pub struct Parser {
  tokens: Vec<Token>,
  index: usize
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Parser {
    Parser { tokens: tokens, index: 0 }
  }

  pub fn parse(&mut self) -> Vec<Box<dyn Stmt>> {
    let mut statements: Vec<Box<dyn Stmt>> = Vec::new();
    while !self.is_end() {
      statements.push(self.statement());
    }
    statements
  }

  fn statement(&mut self) -> Box<dyn Stmt> {
    if self.matches(&[TokenType::Print]) {
      return self.print_statment();
    }

    self.expression_statement()
  }

  fn print_statment(&mut self) -> Box<dyn Stmt> {
    let expr = self.expression();
    self.consume(TokenType::Semicolon, "Expect ';' after value.").ok();
    Print::new(expr)
  }

  fn expression_statement(&mut self) -> Box<dyn Stmt> {
    let expr = self.expression();
    self.consume(TokenType::Semicolon, "Expect ';' after expression.").ok();
    Expression::new(expr) 
  }
  
  fn previous(&mut self) -> Token {
    self.tokens.get(self.index - 1).unwrap().clone()
  }

  fn expression(&mut self) -> BoxedExpr {
    self.equality() 
  }

  fn equality(&mut self) -> BoxedExpr {
    let mut expr = self.comparison();

    while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
      let operator = self.previous();
      let right = self.comparison();
      expr = Binary::new(expr, operator, right);
    }
    expr
  }

  fn comparison(&mut self) -> BoxedExpr {
    let mut expr = self.addition();

    while self.matches(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::LessEqual, TokenType::Less]) {
      let operator = self.previous();
      let right = self.addition();
      expr = Binary::new(expr, operator, right);
    }
    expr
  }

  fn addition(&mut self) -> BoxedExpr {
    let mut expr = self.multiplication();

    while self.matches(&[TokenType::Minus, TokenType::Plus]) {
      let operator = self.previous();
      let right = self.multiplication();
      expr = Binary::new(expr, operator, right);
    }
    expr
  }

  fn multiplication(&mut self) -> BoxedExpr {
    let mut expr = self.unary();

    while self.matches(&[TokenType::Slash, TokenType::Star]) {
      let operator = self.previous();
      let right = self.unary();
      expr = Binary::new(expr, operator, right);
    }
    expr
  }

  fn unary(&mut self) -> BoxedExpr {
    if self.matches(&[TokenType::Bang, TokenType::Minus]) {
      let operator = self.previous();
      let right = self.unary();
      return Unary::new(operator, right);
    }

    self.primary()
  }

  fn primary(&mut self) -> BoxedExpr {
    if self.matches(&[TokenType::False]) {
      return Literal::new(LoxValue::Boolean(false))
    }

    if self.matches(&[TokenType::True]) {
      return Literal::new(LoxValue::Boolean(true))
    }

    if self.matches(&[TokenType::Nil]) {
      return Literal::new(LoxValue::Nil)
    }

    if self.matches(&[TokenType::Number]) {
      return Literal::new(LoxValue::Number(f32::from_str(&self.previous().lexeme()).unwrap()))
    }

    if self.matches(&[TokenType::String]) {
      return Literal::new(LoxValue::String(self.previous().lexeme()))
    }

    if self.matches(&[TokenType::LeftParen]) {
      let expr = self.expression();
      self.consume(TokenType::RightParen, "Expect ')' after expression").ok();
      return Grouping::new(expr);
    }

    Literal::new(LoxValue::Number(0.0))
  }

  // helper methods not part of the parsing grammar
  fn consume(&mut self, token_type: TokenType, error: &str) -> Result<Token, ParserError> {
    if self.check(token_type) {
      Ok(self.next().unwrap())
    } else {
      Err(ParserError::new(error.to_string()))
    }
  }

  fn matches(&mut self, tokens: &[TokenType]) -> bool {
    for token_type in tokens {
      if self.check(*token_type) {
        self.next();
        return true
      }
    }
    false
  }

  fn check(&mut self, token_type: TokenType) -> bool {
    if let Some(token) = self.peek() {
      return token.token_type() == token_type
    } else {
      return false
    }
  }

  fn peek(&mut self) -> Option<Token> {
    match self.tokens.get(self.index) {
      Some(token) => Some(token.clone()),
      None => None
    }
  }

  fn is_end(&mut self) -> bool {
    match self.peek() {
      Some(token) => token.token_type() == TokenType::Eof,
      None => true
    }
  }

  fn synchronize(& mut self) {
    self.next();

    while let Some(token) = self.peek() {
        if self.previous().token_type() == TokenType::Semicolon {
          return ()
        }

        match token.token_type() {
          TokenType::Class => return,
          TokenType::Fun   => return,
          TokenType::For   => return,
          TokenType::If   => return,
          TokenType::Var   => return,
          TokenType::While   => return,
          TokenType::Print   => return,
          TokenType::Return   => return,
          _ => ()
        }

        self.next();
    }
  }
}

impl Iterator for Parser {
  type Item = Token;

  fn next(&mut self) -> Option<Token> {
    self.index += 1;
    if self.index < self.tokens.len() {
      Some(self.tokens.get(self.index).unwrap().clone())
    } else {
      None
    }
  }
}

#[derive(Debug)]
struct ParserError {
  description: String
}

impl ParserError {
  fn new(description: String) -> ParserError {
      ParserError { description }
  }
}

impl Display for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.description)
  }
}

impl Error for ParserError {}