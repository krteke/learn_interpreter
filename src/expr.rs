use crate::{
    error::{Error, Result, RuntimeError},
    token::{Literal, Token},
    token_type::TokenType,
    value::Value,
};

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Stmt(StmtExpr),
    Variable(Token),
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct LiteralExpr {
    pub value: Literal,
}

impl LiteralExpr {
    pub fn new(value: Literal) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum StmtExpr {
    Expr(Box<Expr>),
    Print(Box<Expr>),
    Var(Var),
}

#[derive(Debug)]
pub struct Var {
    pub name: Token,
    pub initializer: Option<Box<Expr>>,
}

impl Var {
    pub fn new(name: Token, initializer: Option<Expr>) -> Self {
        Self {
            name,
            initializer: initializer.map(Box::new),
        }
    }
}

impl Expr {
    pub fn eval(&self) -> Result<Value> {
        match self {
            Expr::Binary(b) => {
                let left = b.left.eval()?;
                let right = b.right.eval()?;

                match b.operator.token_type {
                    TokenType::Minus => {
                        check_number_operands(&b.operator, &left, &right)?;
                        Ok(left - right)
                    }
                    TokenType::Slash => {
                        check_number_operands(&b.operator, &left, &right)?;
                        Ok(left / right)
                    }
                    TokenType::Star => {
                        check_number_operands(&b.operator, &left, &right)?;
                        Ok(left * right)
                    }
                    TokenType::Plus => {
                        check_plus(&b.operator, &left)?;
                        check_plus(&b.operator, &right)?;
                        Ok(left + right)
                    }
                    TokenType::Greater => {
                        check_number_operands(&b.operator, &left, &right)?;
                        Ok((left > right).into())
                    }
                    TokenType::GreaterEqual => {
                        check_number_operands(&b.operator, &left, &right)?;
                        Ok((left >= right).into())
                    }
                    TokenType::Less => {
                        check_number_operands(&b.operator, &left, &right)?;
                        Ok((left < right).into())
                    }
                    TokenType::LessEqual => {
                        check_number_operands(&b.operator, &left, &right)?;
                        Ok((left <= right).into())
                    }
                    TokenType::EqualEqual => Ok((left == right).into()),
                    TokenType::BangEqual => Ok((left != right).into()),
                    _ => unreachable!(),
                }
            }
            Expr::Grouping(g) => g.expression.eval(),
            Expr::Literal(l) => Ok(l.value.clone().into()),
            Expr::Unary(u) => {
                let right = u.right.eval()?;

                match u.operator.token_type {
                    TokenType::Minus => {
                        check_number_operand(&u.operator, &right)?;
                        Ok(-right)
                    }
                    TokenType::Bang => Ok(!right),
                    _ => unreachable!(),
                }
            }
            Expr::Stmt(s) => match s {
                StmtExpr::Expr(e) => e.eval(),
                StmtExpr::Print(p) => {
                    let value = p.eval()?;
                    println!("{}", value);
                    Ok(value)
                }
                _ => todo!(),
            },
            Expr::Variable(v) => todo!(),
        }
    }
}

fn check_number_operand(operator: &Token, operand: &Value) -> Result<()> {
    if let Value::Number(_) = operand {
        Ok(())
    } else {
        Err(Error::Runtime(RuntimeError::new(
            operator.clone(),
            "Operand must be a number".to_string(),
        )))
    }
}

fn check_number_operands(operator: &Token, left: &Value, right: &Value) -> Result<()> {
    check_number_operand(operator, left)?;
    check_number_operand(operator, right)?;

    Ok(())
}

fn check_plus(operator: &Token, operand: &Value) -> Result<()> {
    if let Value::Number(_) | Value::String(_) = operand {
        Ok(())
    } else {
        Err(Error::Runtime(RuntimeError::new(
            operator.clone(),
            "Operand must be a number or a string".to_string(),
        )))
    }
}
