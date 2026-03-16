use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::ast::{action::Action, environment::Environment, stmt::FunctionStmt, value::Callable};

#[derive(Debug)]
pub struct LoxFunction {
    declaration: FunctionStmt,
    closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    pub fn new(declaration: FunctionStmt, closure: Rc<RefCell<Environment>>) -> Self {
        Self {
            declaration,
            closure,
        }
    }
}

impl Callable for LoxFunction {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn call(
        &self,
        interpreter: &mut crate::ast::interpreter::Interpreter,
        args: Vec<crate::ast::value::Value>,
    ) -> crate::ast::error::Result<crate::ast::value::Value> {
        let mut env = Environment::new_with_enclosing(self.closure.clone());

        for (param, arg) in self.declaration.params.iter().zip(args.iter()) {
            env.define(param.lexeme.clone(), arg.clone());
        }

        let action =
            interpreter.execute_block(&self.declaration.body, Rc::new(RefCell::new(env)))?;
        if let Action::Return(value) = action {
            return Ok(value);
        }

        Ok(crate::ast::value::Value::Nil)
    }
}

impl Display for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<function {}>", self.declaration.name.lexeme)
    }
}
