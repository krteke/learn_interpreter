use crate::ast::{
    expr::{Expr, Var},
    token::Token,
};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Block(Vec<Stmt>),
    If(IfStmt),
    While(WhileStmt),
    Function(FunctionStmt),
    Var(Var),
    Return(ReturnStmt),
}

#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Expr,
}

impl ReturnStmt {
    pub fn new(keyword: Token, value: Expr) -> Self {
        Self { keyword, value }
    }
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

impl IfStmt {
    pub fn new(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Self {
        Self {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

impl WhileStmt {
    pub fn new(condition: Expr, body: Stmt) -> Self {
        Self {
            condition,
            body: Box::new(body),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionStmt {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

impl FunctionStmt {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        Self { name, params, body }
    }
}
