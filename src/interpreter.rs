use std::{cell::RefCell, rc::Rc};

use crate::{
    action::Action,
    clock::Clock,
    environment::Environment,
    error::{Error, Result, RuntimeError},
    expr::Expr,
    lox_function::LoxFunction,
    stmt::Stmt,
    token::Token,
    token_type::TokenType,
    value::{Callable, Value},
};

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::default()));
        globals
            .borrow_mut()
            .define("clock".to_string(), Value::Function(Rc::new(Clock)));

        Self {
            globals: globals.clone(),
            env: globals,
        }
    }

    pub fn interpret(&mut self, stmts: &[Stmt]) -> Result<()> {
        for stmt in stmts {
            let action = self.execute(stmt)?;
            // if let Action::Return(value) = action {
            //     return Ok(());
            // }
        }

        Ok(())
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Binary(b) => {
                let left = self.evaluate(&b.left)?;
                let right = self.evaluate(&b.right)?;

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
            Expr::Grouping(g) => self.evaluate(&g.expression),
            Expr::Literal(l) => Ok(l.value.clone().into()),
            Expr::Unary(u) => {
                let right = self.evaluate(&u.right)?;

                match u.operator.token_type {
                    TokenType::Minus => {
                        check_number_operand(&u.operator, &right)?;
                        Ok(-right)
                    }
                    TokenType::Bang => Ok(!right),
                    _ => unreachable!(),
                }
            }
            Expr::Variable(v) => self.env.borrow().get(v),
            Expr::Assign(a) => {
                let value = self.evaluate(&a.value)?;
                self.env.borrow_mut().assign(&a.name, value.clone())?;

                Ok(value)
            }
            Expr::Logical(l) => {
                let left = self.evaluate(&l.left)?;

                if l.operator.token_type == TokenType::Or {
                    if left.is_truthy() {
                        return Ok(left);
                    }
                } else if !left.is_truthy() {
                    return Ok(left);
                }

                self.evaluate(&l.right)
            }
            Expr::Call(c) => {
                let callee = self.evaluate(&c.callee)?;

                let mut args = Vec::new();
                for arg in c.args.iter() {
                    args.push(self.evaluate(arg)?);
                }

                if let Value::Function(function) = callee {
                    if args.len() != function.arity() {
                        return Err(Error::Runtime(RuntimeError::new(
                            c.paren.clone(),
                            format!(
                                "Expected {} arguments but got {}.",
                                function.arity(),
                                args.len()
                            ),
                        )));
                    }

                    function.call(self, args)
                } else {
                    Err(Error::Runtime(RuntimeError::new(
                        c.paren.clone(),
                        "Can only call functions and classes".to_string(),
                    )))
                }
            }
        }
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<Action> {
        match stmt {
            Stmt::Expr(e) => {
                self.evaluate(e)?;
            }
            Stmt::Print(p) => {
                let value = self.evaluate(p)?;
                println!("{}", value);
            }
            Stmt::Var(v) => {
                let value = v
                    .initializer
                    .as_ref()
                    .map(|v| self.evaluate(v))
                    .transpose()?
                    .unwrap_or(Value::Nil);

                self.env.borrow_mut().define(v.name.lexeme.clone(), value);
            }
            Stmt::Block(b) => {
                let new_env = Rc::new(RefCell::new(Environment::new_with_enclosing(
                    self.env.clone(),
                )));

                self.execute_block(b, new_env)?;
            }
            Stmt::If(i) => {
                let condition_value = self.evaluate(&i.condition)?;

                if condition_value.is_truthy() {
                    return self.execute(&i.then_branch);
                } else if let Some(else_branch) = &i.else_branch {
                    return self.execute(else_branch);
                }
            }
            Stmt::While(w) => {
                while self.evaluate(&w.condition)?.is_truthy() {
                    let action = self.execute(&w.body)?;
                    if let Action::Return(_) = action {
                        return Ok(action);
                    }
                }
            }
            Stmt::Function(f) => {
                let fun = LoxFunction::new(f.clone(), self.env.clone());
                self.env
                    .borrow_mut()
                    .define(f.name.lexeme.clone(), Value::Function(Rc::new(fun)));
            }
            Stmt::Return(r) => {
                let value = self.evaluate(&r.value)?;
                return Ok(Action::Return(value));
            }
        };

        Ok(Action::None)
    }

    pub fn execute_block(
        &mut self,
        stmts: &[Stmt],
        env: Rc<RefCell<Environment>>,
    ) -> Result<Action> {
        let previous = self.env.clone();
        self.env = env;

        let result = (|| {
            for stmt in stmts {
                let action = self.execute(stmt)?;
                if let Action::Return(_) = action {
                    return Ok(action);
                }
            }
            Ok(Action::None)
        })();

        self.env = previous;

        result
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
