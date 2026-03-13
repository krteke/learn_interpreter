use std::{cell::RefCell, sync::OnceLock};

use crate::{
    environment::Environment,
    error::Result,
    expr::{Expr, StmtExpr},
    value::Value,
};

thread_local! {
    pub static ENV: RefCell<Environment> = RefCell::new(Environment::new());
}

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&mut self, stmts: Vec<StmtExpr>) -> Result<()> {
        for statement in stmts {
            self.evaluate(Expr::Stmt(statement.clone()))?;

            // if let StmtExpr::Var(v) = statement {
            //     self.environment.define(v.name.lexeme, value);
            // }
        }

        Ok(())
    }

    pub fn evaluate(&self, expr: Expr) -> Result<Value> {
        expr.eval()
    }
}
