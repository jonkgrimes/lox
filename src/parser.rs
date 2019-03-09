use std::iter::Peekable;
use std::slice::Iter;

use crate::token::{Token, TokenType};
use crate::expr::{Expr, Unary, Binary, Literal, Grouping};

pub struct Parser {
  tokens: Vec<Token>,
}

struct Cursor<'a> {
  previous: Option<Token>,
  iterator: Peekable<Iter<'a, Token>>
}

impl Parser {
  fn new(tokens: Vec<Token>) -> Parser {
    Parser { tokens: tokens }
  }

  fn parse(&self) -> Box<dyn Expr> {
    let mut cursor = Cursor {
      previous: None,
      iterator: self.tokens.iter().peekable()
    };
    expression(&mut cursor)
  }
}

fn expression(cursor: &mut Cursor) -> Box<dyn Expr> {
  equality(cursor) 
}

fn equality(cursor: &mut Cursor) -> Box<dyn Expr> {
  let mut expr = equality(cursor);

  while token_match(cursor, &[TokenType::BangEqual, TokenType::EqualEqual]) {
    let operator = cursor.previous.unwrap();
    let right = equality(cursor);
    expr = Binary::new(expr, operator, right);
  }

  expr
}

fn token_match(cursor: &mut Cursor, tokens: &[TokenType]) -> bool {
  for token_type in tokens {
    if check(cursor, *token_type) {
      advance(cursor);
      return true;
    }
  }
  false
}

fn check(cursor: &mut Cursor, token_type: TokenType) -> bool {
  if at_end(cursor) { return false; }
  match cursor.iterator.peek() {
    Some(token) => token.token_type() == token_type,
    None => false
  }
}

fn advance(cursor: &mut Cursor) -> Option<Token> {
  cursor.previous = match cursor.iterator.next() {
    Some(token) => Some(*token),
    None => None
  };
  cursor.previous
}

fn at_end(cursor: &mut Cursor) -> bool {
  if let Some(token) = cursor.iterator.peek() {
    token.token_type() == TokenType::Eof
  } else { 
    false
  }
}
/*
fn comparison(&self) -> Box<dyn Expr> {
  let mut expr = self.addition();

  while self.token_match(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
    let operator = self.previous().unwrap();
    let right = self.addition();
    expr = Binary::new(expr, operator, right);
  }

  expr
}

fn addition(&self) -> Box<dyn Expr> {
  let mut expr = self.multiplication();

  while self.token_match(&[TokenType::Minus, TokenType::Plus]) {
    let operator = self.previous().unwrap();
    let right = self.multiplication();
    expr = Binary::new(expr, operator, right);
  }

  expr
}

fn multiplication(&self) -> Box<dyn Expr> {
  let mut expr = self.unary();

  while self.token_match(&[TokenType::Slash, TokenType::Star])  {
    let operator = self.previous().unwrap();
    let right = self.unary();
    expr = Binary::new(expr, operator, right);
  }

  expr
}

fn unary(&self) -> Box<dyn Expr> {
  if self.token_match(&[TokenType::Bang, TokenType::Minus]) {
    let operator = self.previous().unwrap();
    let right = self.unary();
    return Unary::new(operator, right)
  }
  self.primary()
}

fn primary(&self) -> Box<dyn Expr> {
  if self.token_match(&[TokenType::False]) {
    return Literal::new(false)
  }
  Literal::new(true)
}

// associated helper functions
fn token_match(&self, tokens: &[TokenType]) -> bool {
  for token_type in tokens {
    if self.check(*token_type) {
      self.cursor.next();
      return true;
    }
  }
  false
}

fn previous(&self) -> Option<Token> {
  self.previous
}

*/