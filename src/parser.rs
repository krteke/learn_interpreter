use crate::{
    error::{Error, ParserError, Result},
    expr::{AssignExpr, BinaryExpr, Expr, GroupingExpr, LiteralExpr, StmtExpr, UnaryExpr, Var},
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

    pub fn parse(&mut self) -> Result<Vec<StmtExpr>> {
        let mut statements = Vec::new();
        while !self.at_end() {
            statements.push(self.declaration().inspect_err(|e| {
                eprintln!("{e}");
                self.synchronize();
            })?);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<StmtExpr> {
        let token_type = self.peek().token_type;

        match token_type {
            Var => {
                self.advance();
                self.var_declaration()
            }
            _ => self.statement(),
        }
    }

    fn var_declaration(&mut self) -> Result<StmtExpr> {
        let name = self.consume(Identifier, "Expect variable name.")?;
        let mut init = None;

        if self.peek().token_type == Equal {
            self.advance();
            init = Some(self.expression()?);
        }

        self.consume(Semicolon, "Expect ';' after variable declaration.")?;

        Ok(StmtExpr::Var(Var::new(name, init)))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.equality()?;

        if self.peek().token_type == Equal {
            self.advance();

            let value = self.assignment()?;

            if let Expr::Variable(v) = expr {
                return Ok(Expr::Assign(AssignExpr::new(v, value)));
            }
        }

        Ok(expr)
    }

    fn statement(&mut self) -> Result<StmtExpr> {
        match self.peek().token_type {
            Print => {
                self.advance();
                self.print_statement()
            }
            LeftBrace => {
                self.advance();
                Ok(StmtExpr::Block(self.block()?))
            }
            If => {
                self.advance();
                self.if_statement()
            }
            _ => self.expression_statement(),
        }
    }

    fn if_statement(&mut self) -> Result<StmtExpr> {
        self.consume(LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(RightParen, "Expect ')' after 'if condition.")?;

        let then_branch = Expr::Stmt(self.statement()?);
        let mut else_branch = None;

        if self.peek().token_type == Else {
            self.advance();
            else_branch = Some(Expr::Stmt(self.statement()?));
        }

        Ok(StmtExpr::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        })
    }

    fn block(&mut self) -> Result<Vec<StmtExpr>> {
        let mut statements = Vec::new();

        while !self.at_end() && self.peek().token_type != RightBrace {
            statements.push(self.declaration()?);
        }
        self.consume(RightBrace, "Expect '}' after block.")?;

        Ok(statements)
    }

    fn print_statement(&mut self) -> Result<StmtExpr> {
        let value = self.expression()?;
        self.consume(Semicolon, "Expect ';' after value.")?;

        Ok(StmtExpr::Print(Box::new(value)))
    }

    fn expression_statement(&mut self) -> Result<StmtExpr> {
        let expr = self.expression()?;
        self.consume(Semicolon, "Expect ';' after expression.")?;

        Ok(StmtExpr::Expr(Box::new(expr)))
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
            _ => self.primary(),
        }
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
