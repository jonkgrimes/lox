use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::lox_value::LoxValue;
use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct Environment {
  pub enclosing: Option<Rc<RefCell<Environment>>>,
  values: HashMap<String, LoxValue>
}

impl Environment {
  pub fn new() -> Environment {
    Environment { values: HashMap::new(), enclosing: None }
  }

  pub fn new_with(enclosing: Rc<RefCell<Environment>>) -> Environment {
    Environment { values: HashMap::new(), enclosing: Some(enclosing) }
  }

  pub fn define(&mut self, name: String, value: LoxValue) {
    self.values.insert(name, value);
  }

  pub fn get(&mut self, name: Token) -> LoxValue {
      if let Some(value) = self.values.get(&name.lexeme()) {
          value.clone()
      } else { 
        if let Some(enclosing) = &self.enclosing {
            return (*enclosing.borrow_mut()).get(name.clone());
        } else {
          LoxValue::Nil
        }
      }
  }

  pub fn assign(&mut self, name: Token, value: LoxValue) {
    let variable = name.lexeme();
    match self.values.insert(variable, value.clone()) {
      Some(_) => {
      },
      None => {
        if let Some(enclosing) = &self.enclosing {
          (*enclosing.borrow_mut()).assign(name, value.clone());
        } else {
          panic!("Variable did not exist!");
        }
      }
    }
  }

  pub fn get_at(&mut self, distance: usize, name: String) -> LoxValue {
      if distance == 0 {
        self.values.get(&name).unwrap().clone()
      } else {
        if let Some(enclosing) = &self.enclosing {
            (*enclosing.borrow_mut()).get_at(distance - 1, name)
        } else {
            LoxValue::Nil
        }
      }
  }

  pub fn assign_at(&mut self, distance: usize, name: String, value: LoxValue) {
      if distance == 0 {
          self.values.insert(name, value.clone()).unwrap();
      } else { 
          if let Some(enclosing) = &self.enclosing {
              (*enclosing.borrow_mut()).assign_at(distance - 1, name, value)
          }
      }
  }
}