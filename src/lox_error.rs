use std::fmt::Display;

#[derive(Debug)]
pub enum LoxError {
  RuntimeError(String)
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let LoxError::RuntimeError(message) = &self;
        write!(f, "{}", message)
    }
}