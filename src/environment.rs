use std::collections::HashMap;

use crate::{
    error::{Error, Result, RuntimeError},
    token::Token,
    value::Value,
};

pub struct Environment {
    pub value: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            value: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.value.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Value> {
        self.value.get(&name.lexeme).cloned().ok_or_else(|| {
            Error::Runtime(RuntimeError::new(
                name.clone(),
                format!("Undefined variable '{}'", name.lexeme),
            ))
        })
    }
}
