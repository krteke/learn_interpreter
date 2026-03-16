use crate::expr::{Expr, Var};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Var(Var),
}
