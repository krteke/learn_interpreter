use crate::{
    error::Result,
    expr::{Expr, StmtExpr},
    value::Value,
};

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&self, stmts: Vec<StmtExpr>) -> Result<()> {
        // self.evaluate(expr).map(|v| {
        //     println!("{}", v);
        // })?;
        for statement in stmts {
            self.evaluate(Expr::Stmt(statement))?;
        }

        Ok(())
    }

    pub fn evaluate(&self, expr: Expr) -> Result<Value> {
        expr.eval()
    }
}
