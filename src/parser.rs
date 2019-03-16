use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;

use crate::token::{Token, TokenType};
use crate::expr::{Expr, BoxedExpr, Unary, Binary, Literal, Grouping};

pub struct Parser {
  tokens: Vec<Token>,
  index: usize
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Parser {
    Parser { tokens: tokens, index: 0 }
  }

  pub fn parse(&mut self) -> BoxedExpr {
    self.expression()
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
      return Literal::new(false)
    }

    if self.matches(&[TokenType::True]) {
      return Literal::new(true)
    }

    if self.matches(&[TokenType::Number]) {

      return Literal::new(f32::from_str(&self.previous().lexeme()).unwrap())
    }

    Literal::new(0)
  }

  // helper methods not part of the parsing grammar
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
    self.peek().token_type() == token_type
  }

  fn peek(&mut self) -> Token {
    self.tokens.get(self.index).unwrap().clone()
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
