use std::fmt::Display;

#[derive(Debug)]
pub enum LoxError {
  RuntimeError(String)
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
        LoxError::RuntimeError(message) => write!(f, "RuntimeError: {}", message)
      }
    }
}