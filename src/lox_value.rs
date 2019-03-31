use std::fmt::Display;
use std::ops::{Sub, Add, Not, Div, Mul, Neg};
use std::cmp::{PartialOrd, Ordering};

#[derive(Debug, PartialEq)]
pub enum LoxValue {
    Nil,
    String(String),
    Number(f32),
    Boolean(bool)
}

impl Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoxValue::Boolean(value) => { write!(f, "{}", value) },
            LoxValue::Number(value) => { write!(f, "{}", value) },
            LoxValue::String(value) => { write!(f, "{}", value) },
            LoxValue::Nil => { write!(f, "nil") }
        }
    }
}

impl Neg for LoxValue {
    type Output = LoxValue;

    fn neg(self) -> LoxValue {
        match self {
            LoxValue::Number(value) => LoxValue::Number(-value),
            _ => panic!("Can't switch the sign of a boolean value")
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
    type Output = LoxValue;

    fn add(self, rhs: LoxValue) -> LoxValue {
        match self {
            LoxValue::Number(value) => {
                match rhs {
                    LoxValue::Number(rhs_value) => LoxValue::Number(value + rhs_value),
                    _ => panic!("Can't add these two values")
                }
            },
            LoxValue::String(value) => {
                match rhs {
                    LoxValue::String(rhs_value) => {
                        let mut new_str = value.clone();
                        new_str.push_str(&rhs_value);
                        LoxValue::String(new_str)
                    },
                    _ => panic!("Can't add these two values")
                }
            }
            _ => panic!("Can't add these two values")
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