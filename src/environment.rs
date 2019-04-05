use std::collections::HashMap;

use crate::lox_value::LoxValue;
use crate::token::Token;

#[derive(Clone)]
pub struct Environment {
  enclosing: Option<Box<Environment>>,
  values: HashMap<String, LoxValue>
}

impl Environment {
  pub fn new() -> Environment {
    Environment { values: HashMap::new(), enclosing: None }
  }

  pub fn new_with(enclosing: Environment) -> Environment {
    Environment { values: HashMap::new(), enclosing: Some(Box::new(enclosing)) }
  }

  pub fn define(&mut self, name: String, value: LoxValue) {
    self.values.insert(name, value);
  }

  pub fn get(&mut self, name: Token) -> LoxValue {
      if let Some(mut enclosing) = self.enclosing.clone() {
          return enclosing.get(name.clone());
      }

      if let Some(value) = self.values.get(&name.lexeme()) {
          value.clone()
      } else { 
          LoxValue::Nil
      }
  }

  pub fn assign(&mut self, name: Token, value: LoxValue) {
    let variable = name.lexeme();
    match self.values.insert(variable, value.clone()) {
      Some(_) => (),
      None => println!("Variable did not exist.")
    }

    if let Some(mut enclosing) = self.enclosing.clone() {
      enclosing.assign(name, value.clone());
    }
  }
}