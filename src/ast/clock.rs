use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::ast::{
    error::Result,
    interpreter::Interpreter,
    value::{Callable, Value},
};

#[derive(Debug)]
pub struct Clock;

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _interpreter: &mut Interpreter, _args: Vec<Value>) -> Result<Value> {
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            // unwrap??
            .unwrap()
            .as_secs_f64();

        Ok(Value::Number(start))
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native function>")
    }
}
