use crate::token::{Literal, Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Call(CallExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Variable(Token),
    Assign(AssignExpr),
    Logical(LogicalExpr),
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

impl GroupingExpr {
    pub fn new(expression: Expr) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LiteralExpr {
    pub value: Literal,
}

impl LiteralExpr {
    pub fn new(value: Literal) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(operator: Token, right: Expr) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Var {
    pub name: Token,
    pub initializer: Option<Expr>,
}

impl Var {
    pub fn new(name: Token, initializer: Option<Expr>) -> Self {
        Self { name, initializer }
    }
}

#[derive(Debug, Clone)]
pub struct AssignExpr {
    pub name: Token,
    pub value: Box<Expr>,
}

impl AssignExpr {
    pub fn new(name: Token, value: Expr) -> Self {
        Self {
            name,
            value: Box::new(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl LogicalExpr {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub args: Vec<Expr>,
}

impl CallExpr {
    pub fn new(callee: Expr, paren: Token, args: Vec<Expr>) -> Self {
        Self {
            callee: Box::new(callee),
            paren,
            args,
        }
    }
}
