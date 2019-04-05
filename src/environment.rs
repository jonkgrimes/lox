use std::collections::HashMap;

use crate::lox_value::LoxValue;
use crate::token::Token;

pub struct Environment {
  values: HashMap<String, LoxValue>
}

impl Environment {
  pub fn new() -> Environment {
    Environment { values: HashMap::new() }
  }

  pub fn define(&mut self, name: String, value: LoxValue) {
    self.values.insert(name, value);
  }

  pub fn get(&mut self, name: Token) -> LoxValue {
      if let Some(value) = self.values.get(&name.lexeme()) {
          value.clone()
      } else { 
          LoxValue::Nil
      }
  }

  pub fn assign(&mut self, name: Token, value: LoxValue) {
    let variable = name.lexeme();
    match self.values.insert(variable, value) {
      Some(_) => (),
      None => println!("Variable did not exist.")
    }
  }
}