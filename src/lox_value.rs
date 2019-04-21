use std::fmt::Display;
use std::ops::{Sub, Add, Not, Div, Mul, Neg};
use std::cmp::{PartialOrd, Ordering};

use crate::lox_error::LoxError;
use crate::lox_function::LoxFunction;

#[derive(Debug, Clone, PartialEq)]
pub enum LoxValue {
    Nil,
    String(String),
    Number(f32),
    Boolean(bool),
    Function(LoxFunction)
}

impl Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoxValue::Boolean(value) => { write!(f, "{}", value) },
            LoxValue::Number(value) => { write!(f, "{}", value) },
            LoxValue::String(value) => { write!(f, "{}", value) },
            LoxValue::Nil => { write!(f, "nil") },
            LoxValue::Function(function) => { write!(f, "function {}", function.name())}
        }
    }
}

impl Neg for LoxValue {
    type Output = Result<LoxValue, LoxError>;

    fn neg(self) -> Result<LoxValue, LoxError> {
        match self {
            LoxValue::Nil => Ok(LoxValue::Boolean(true)),
            LoxValue::Number(value) => Ok(LoxValue::Number(-value)),
            LoxValue::Boolean(_) => Err(LoxError::RuntimeError("Boolean values cannot be negated".to_string())),
            LoxValue::String(_) => Err(LoxError::RuntimeError("String values cannot be negated".to_string())),
            LoxValue::Function(_) => Err(LoxError::RuntimeError("Functions cannot be negated".to_string()))
        }
    }
}

impl Not for LoxValue {
    type Output = LoxValue;

    fn not(self) -> LoxValue {
        match self {
            LoxValue::Boolean(value) => LoxValue::Boolean(!value),
            LoxValue::Nil => LoxValue::Boolean(true),
            _ => LoxValue::Boolean(false)
        }
    }
}

impl Div for LoxValue {
    type Output = LoxValue;

    fn div(self, rhs: LoxValue) -> LoxValue {
        match self {
            LoxValue::Number(value) => {
                match rhs {
                    LoxValue::Number(rhs_value) => LoxValue::Number(value / rhs_value),
                    _ => panic!("Can't divide these two values")
                }
            },
            _ => panic!("Can't divide these two values")
        }
    }
}

impl Sub for LoxValue {
    type Output = LoxValue;

    fn sub(self, rhs: LoxValue) -> LoxValue {
        match self {
            LoxValue::Number(value) => {
                match rhs {
                    LoxValue::Number(rhs_value) => LoxValue::Number(value - rhs_value),
                    _ => panic!("Can't subtract these two values")
                }
            },
            _ => panic!("Can't subtract these two values")
        }
    }
}

impl Add for LoxValue {
    type Output = Result<LoxValue, LoxError>;

    fn add(self, rhs: LoxValue) -> Result<LoxValue, LoxError> {
        match self {
            LoxValue::Number(value) => {
                match rhs {
                    LoxValue::Number(rhs_value) => Ok(LoxValue::Number(value + rhs_value)),
                    _ => Err(LoxError::RuntimeError("right hand side must also be a number".to_string()))
                }
            },
            LoxValue::String(value) => {
                match rhs {
                    LoxValue::String(rhs_value) => {
                        let mut new_str = value.clone();
                        new_str.push_str(&rhs_value);
                        Ok(LoxValue::String(new_str))
                    },
                    _ => panic!("TypeError: Can't add a string to a number.")
                }
            }
            LoxValue::Boolean(_value) => {
                Err(LoxError::RuntimeError("Cannot add value to boolean.".to_string()))
            },
            LoxValue::Nil => {
                Err(LoxError::RuntimeError("Cannot add value to nil.".to_string()))
            },
            LoxValue::Function(_) => {
                Err(LoxError::RuntimeError("Cannot add value to a function".to_string()))
            }
        }
    }
}

impl Mul for LoxValue {
    type Output = LoxValue;

    fn mul(self, rhs: LoxValue) -> LoxValue {
        match self {
            LoxValue::Number(value) => {
                match rhs {
                    LoxValue::Number(rhs_value) => LoxValue::Number(value * rhs_value),
                    _ => panic!("Can't multiply these two values")
                }
            },
            _ => panic!("Can't multiply these two values")
        }
    }
}

impl PartialOrd for LoxValue {
  fn partial_cmp(&self, other: &LoxValue) -> Option<Ordering> {
    match self {
      LoxValue::Nil => {
        match other {
          LoxValue::Nil => Some(Ordering::Equal),
          _ => Some(Ordering::Greater)
        }
      },
      LoxValue::Number(value) => {
        match other {
          LoxValue::Number(other_value) => value.partial_cmp(other_value),
          _ => panic!("Can't compare a number with this value")
        }
      },
      _ => panic!("Can't compare these two types")
    }
  }
}