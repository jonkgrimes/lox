use crate::token::Token;

pub enum LoxError {
  RuntimeError(Token)
}