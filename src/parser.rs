use crate::{
    error::{Error, ParserError, Result, RuntimeError},
    expr::{
        AssignExpr, BinaryExpr, CallExpr, Expr, GroupingExpr, LiteralExpr, LogicalExpr, UnaryExpr,
        Var,
    },
    stmt::{FunctionStmt, IfStmt, ReturnStmt, Stmt, WhileStmt},
    token::{Literal, Token},
    token_type::TokenType::{self, *},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();
        while !self.at_end() {
            statements.push(self.declaration().inspect_err(|e| {
                eprintln!("{e}");
                self.synchronize();
            })?);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt> {
        let token_type = self.peek().token_type;

        match token_type {
            Var => {
                self.advance();
                self.var_declaration()
            }
            Fun => {
                self.advance();
                self.function("function")
            }
            _ => self.statement(),
        }
    }

    fn function(&mut self, kind: &str) -> Result<Stmt> {
        let name = self.consume(Identifier, &format!("Expect {} name.", kind))?;
        self.consume(LeftParen, &format!("Expect '(' after {} name.", kind))?;

        let mut parameters = Vec::new();
        if self.peek().token_type != RightParen {
            loop {
                if parameters.len() >= u16::MAX as usize {
                    return Err(Error::Runtime(RuntimeError::new(
                        self.peek().clone(),
                        "Too many arguments.".to_string(),
                    )));
                }

                parameters.push(self.consume(Identifier, "Expect parameter name.")?);

                if self.peek().token_type == Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.consume(RightParen, "Expect ')' after parameters.")?;
        self.consume(LeftBrace, "Expect '{' before function body.")?;

        let body = self.block()?;

        Ok(Stmt::Function(FunctionStmt::new(name, parameters, body)))
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = self.consume(Identifier, "Expect variable name.")?;
        let mut init = None;

        if self.peek().token_type == Equal {
            self.advance();
            init = Some(self.expression()?);
        }

        self.consume(Semicolon, "Expect ';' after variable declaration.")?;

        Ok(Stmt::Var(Var::new(name, init)))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.or()?;

        if self.peek().token_type == Equal {
            self.advance();

            let value = self.assignment()?;

            if let Expr::Variable(v) = expr {
                return Ok(Expr::Assign(AssignExpr::new(v, value)));
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr> {
        let mut expr = self.and()?;

        while self.peek().token_type == Or {
            let operator = self.advance();
            let right = self.and()?;
            expr = Expr::Logical(LogicalExpr::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;

        while self.peek().token_type == And {
            let operator = self.advance();
            let right = self.equality()?;
            expr = Expr::Logical(LogicalExpr::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn statement(&mut self) -> Result<Stmt> {
        match self.peek().token_type {
            Print => {
                self.advance();
                self.print_statement()
            }
            LeftBrace => {
                self.advance();
                Ok(Stmt::Block(self.block()?))
            }
            If => {
                self.advance();
                self.if_statement()
            }
            While => {
                self.advance();
                self.while_statement()
            }
            For => {
                self.advance();
                self.for_statement()
            }
            Return => {
                self.advance();
                self.return_statement()
            }
            _ => self.expression_statement(),
        }
    }

    fn return_statement(&mut self) -> Result<Stmt> {
        let keyword = self.previous().clone();
        let mut value = Expr::Literal(LiteralExpr::new(Literal::Nil));

        if self.peek().token_type != Semicolon {
            value = self.expression()?;
        }

        self.consume(Semicolon, "Expect ';' after return value.")?;

        Ok(Stmt::Return(ReturnStmt::new(keyword, value)))
    }

    fn for_statement(&mut self) -> Result<Stmt> {
        self.consume(LeftParen, "Expect '(' after 'for'.")?;
        let init = match self.peek().token_type {
            Semicolon => {
                self.advance();
                None
            }
            Var => {
                self.advance();
                Some(self.var_declaration()?)
            }
            _ => Some(self.expression_statement()?),
        };

        let condition = if self.peek().token_type != Semicolon {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(Semicolon, "Expect ';' after for loop condition.")?;

        let increment = if self.peek().token_type != RightParen {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(RightParen, "Expect ')' after for loop increment.")?;
        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Stmt::Block(vec![body, Stmt::Expr(increment)]);
        }

        let condition = condition.unwrap_or(Expr::Literal(LiteralExpr::new(Literal::Bool(true))));
        body = Stmt::While(WhileStmt::new(condition, body));

        if let Some(init) = init {
            body = Stmt::Block(vec![init, body]);
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Stmt> {
        self.consume(LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(RightParen, "Expect ')' after condition.")?;
        let body = self.statement()?;

        Ok(Stmt::While(WhileStmt::new(condition, body)))
    }

    fn if_statement(&mut self) -> Result<Stmt> {
        self.consume(LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(RightParen, "Expect ')' after 'if condition.")?;

        let then_branch = self.statement()?;
        let mut else_branch = None;

        if self.peek().token_type == Else {
            self.advance();
            else_branch = Some(self.statement()?);
        }

        Ok(Stmt::If(IfStmt::new(condition, then_branch, else_branch)))
    }

    fn block(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();

        while !self.at_end() && self.peek().token_type != RightBrace {
            statements.push(self.declaration()?);
        }
        self.consume(RightBrace, "Expect '}' after block.")?;

        Ok(statements)
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let value = self.expression()?;
        self.consume(Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(Semicolon, "Expect ';' after expression.")?;

        Ok(Stmt::Expr(expr))
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while let BangEqual | EqualEqual = self.peek().token_type {
            let operator = self.advance();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;

        while let Greater | GreaterEqual | Less | LessEqual = self.peek().token_type {
            let operator = self.advance();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        while let Minus | Plus = self.peek().token_type {
            let operator = self.advance();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        while let Slash | Star = self.peek().token_type {
            let operator = self.advance();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        match self.peek().token_type {
            Bang | Minus => {
                let operator = self.advance();
                let right = self.unary()?;

                Ok(Expr::Unary(UnaryExpr::new(operator, right)))
            }
            _ => self.call(),
        }
    }

    fn call(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.peek().token_type == LeftParen {
                self.advance();
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr> {
        let mut args = Vec::new();
        if self.peek().token_type != RightParen {
            loop {
                if args.len() >= u16::MAX as usize {
                    return Err(Error::Parser(ParserError::new(
                        self.peek().clone(),
                        "Too many arguments.".to_string(),
                    )));
                }
                args.push(self.expression()?);

                if self.peek().token_type == Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let paren = self.consume(RightParen, "Expected ')' after arguments.")?;

        Ok(Expr::Call(CallExpr::new(callee, paren, args)))
    }

    fn primary(&mut self) -> Result<Expr> {
        match self.peek().token_type {
            False => {
                self.advance();
                Ok(Expr::Literal(LiteralExpr::new(Literal::Bool(false))))
            }
            True => {
                self.advance();
                Ok(Expr::Literal(LiteralExpr::new(Literal::Bool(true))))
            }
            Nil => {
                self.advance();
                Ok(Expr::Literal(LiteralExpr::new(Literal::Nil)))
            }
            Number | String => {
                let value = self.advance();
                Ok(Expr::Literal(LiteralExpr::new(value.literal.unwrap())))
            }
            LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(RightParen, "Expected ')' after expression.")?;
                Ok(Expr::Grouping(GroupingExpr::new(expr)))
            }
            Identifier => {
                self.advance();
                Ok(Expr::Variable(self.previous().clone()))
            }
            _ => Err(Error::Parser(ParserError::new(
                self.peek().clone(),
                "Expected expression.".to_string(),
            ))),
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token> {
        if token_type == self.peek().token_type {
            return Ok(self.advance());
        }

        Err(Error::Parser(ParserError::new(
            self.peek().clone(),
            message.to_string(),
        )))
    }

    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.current += 1;
        }

        self.previous().clone()
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.at_end() {
            if self.previous().token_type == Semicolon {
                return;
            }

            if let Class | Fun | Var | For | If | While | Print | Return = self.peek().token_type {
                return;
            }

            self.advance();
        }
    }

    fn at_end(&self) -> bool {
        self.peek().token_type == EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
