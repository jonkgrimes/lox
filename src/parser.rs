use std::iter::Peekable;
use std::slice::Iter;

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
  
  fn previous(&mut self) -> Option<Token> {
    Some(self.tokens.get(self.index - 1).unwrap().clone())
  }

  fn expression(&mut self) -> BoxedExpr {
    self.equality() 
  }

  fn equality(&mut self) -> BoxedExpr {
    let mut expr = self.comparison();

    while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
      let operator = self.previous().unwrap();
      let right = self.comparison();
      expr = Binary::new(expr, operator, right);
    }
    expr
  }

  fn comparison(&mut self) -> BoxedExpr {
    Literal::new(1)
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
    self.peek().token_type() == TokenType::Eof
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
