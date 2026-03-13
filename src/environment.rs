use std::collections::HashMap;

use crate::{
    error::{Error, Result, RuntimeError},
    token::Token,
    value::Value,
};

#[derive(Default, Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_with_enclosing(enclosing: Environment) -> Self {
        Self {
            enclosing: Some(Box::new(enclosing)),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Value> {
        let value = self.values.get(&name.lexeme);

        if let Some(v) = value {
            return Ok(v.clone());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name);
        }

        Err(Error::Runtime(RuntimeError::new(
            name.clone(),
            format!("Undefined variable '{}'", name.lexeme),
        )))
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<()> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }

        if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, value)?;
            return Ok(());
        }

        Err(Error::Runtime(RuntimeError::new(
            name.clone(),
            format!("Undefined variable '{}'.", name.lexeme),
        )))
    }
}
