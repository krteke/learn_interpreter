use std::{collections::HashMap, fmt::Display, ops::Not, rc::Rc};

use crate::bytecode::token::Literal;

pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
    Obj(Obj),
}

impl Value {
    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Bool(v) => !v,
            Value::Nil => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Obj {
    String(Rc<str>),
}

pub struct StringInterner<T> {
    strings: HashMap<Rc<str>, T>,
}

impl<T> StringInterner<T> {
    pub fn new() -> Self {
        Self {
            strings: HashMap::new(),
        }
    }

    pub fn get(&self, s: &str) -> Option<&T> {
        self.strings.get(s)
    }

    pub fn get_mut(&mut self, s: &str) -> Option<&mut T> {
        self.strings.get_mut(s)
    }

    pub fn intern(&mut self, s: &str, value: T) -> Rc<str> {
        if let Some((existing, _)) = self.strings.get_key_value(s) {
            return existing.clone();
        }

        let interned: Rc<str> = Rc::from(s);
        self.strings.insert(interned.clone(), value);
        interned
    }
}

impl From<&Literal> for Value {
    fn from(value: &Literal) -> Self {
        match value {
            Literal::Number(value) => Self::Number(*value),
            Literal::String(value) => Self::Obj(Obj::String(Rc::from(value.as_str()))),
            Literal::Bool(value) => Self::Bool(*value),
            Literal::Nil => Self::Nil,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(value) => write!(f, "{}", value),
            Value::Bool(value) => write!(f, "{}", value),
            Value::Nil => write!(f, "nil"),
            Value::Obj(obj) => match obj {
                Obj::String(s) => write!(f, "{}", s),
            },
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::Obj(a), Value::Obj(b)) => a == b,
            _ => false,
        }
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        let value = self.is_falsey();
        Value::Bool(value)
    }
}
