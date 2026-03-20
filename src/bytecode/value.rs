use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Not, Sub},
};

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
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl From<&Literal> for Value {
    fn from(value: &Literal) -> Self {
        match value {
            Literal::Number(value) => Self::Number(*value),
            Literal::String(value) => Self::String(value.clone()),
            Literal::Bool(value) => Self::Bool(*value),
            Literal::Nil => Self::Nil,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(value) => write!(f, "{}", value),
            Value::Number(value) => write!(f, "{}", value),
            Value::Bool(value) => write!(f, "{}", value),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Value::Bool(b) => Value::Bool(!b),
            Value::Nil => Value::Bool(true),
            _ => Value::Bool(false),
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            _ => unreachable!(),
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => unreachable!(),
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => unreachable!(),
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}
