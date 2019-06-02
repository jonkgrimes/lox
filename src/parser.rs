use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use crate::expr::{Assign, Binary, BoxedExpr, Call, Grouping, Literal, Logical, Unary, Variable};
use crate::lox_value::LoxValue;
use crate::stmt::{Block, Expression, Function, If, Print, Return, Stmt, Var, While};
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
    }

    pub fn parse(&mut self) -> Vec<Box<dyn Stmt>> {
        let mut statements: Vec<Box<dyn Stmt>> = Vec::new();
        while !self.is_end() {
            statements.push(self.declaration());
        }
        statements
    }

    // Statements
    fn declaration(&mut self) -> Box<dyn Stmt> {
        if self.matches(&[TokenType::Fun]) {
            return self.function("function");
        }
        if self.matches(&[TokenType::Var]) {
            return self.var_declaration();
        }

        self.statement()
    }

    fn function(&mut self, kind: &str) -> Box<dyn Stmt> {
        let name = self
            .consume(
                TokenType::Identifier,
                format!("Expect {} name", kind).as_str(),
            )
            .unwrap();

        self.consume(
            TokenType::LeftParen,
            format!("Expect '(' after {} name", kind).as_str(),
        )
        .ok();

        let mut parameters: Vec<Token> = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() > 10 {
                    panic!("Cannot a have more than 10 parameters for a function");
                }

                parameters.push(
                    self.consume(TokenType::Identifier, "Expect parameter name")
                        .unwrap(),
                );

                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters")
            .ok();

        self.consume(
            TokenType::LeftBrace,
            format!("Expect '{{' before {} body.", kind).as_str(),
        )
        .ok();

        let body = self.block();

        Function::new(name, parameters, body)
    }

    fn var_declaration(&mut self) -> Box<dyn Stmt> {
        let name = self
            .consume(TokenType::Identifier, "Expected variable name")
            .unwrap();

        let initializer = if self.matches(&[TokenType::Equal]) {
            Some(self.expression())
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )
        .ok();
        Var::new(name, initializer)
    }

    fn statement(&mut self) -> Box<dyn Stmt> {
        if self.matches(&[TokenType::For]) {
            return self.for_statement();
        }

        if self.matches(&[TokenType::If]) {
            return self.if_statement();
        }

        if self.matches(&[TokenType::Print]) {
            return self.print_statment();
        }

        if self.matches(&[TokenType::Return]) {
            return self.return_statement();
        }

        if self.matches(&[TokenType::While]) {
            return self.while_statement();
        }

        if self.matches(&[TokenType::LeftBrace]) {
            return Block::new(self.block());
        }

        self.expression_statement()
    }

    fn for_statement(&mut self) -> Box<dyn Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")
            .ok();

        let initializer = if self.matches(&[TokenType::Semicolon]) {
            None
        } else if self.matches(&[TokenType::Var]) {
            Some(self.var_declaration())
        } else {
            Some(self.expression_statement())
        };

        let condition = if !self.check(TokenType::Semicolon) {
            Some(self.expression())
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")
            .ok();

        let increment = if !self.check(TokenType::RightParen) {
            Some(self.expression())
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after 'for' clauses.")
            .ok();

        let mut body = self.statement();

        if let Some(increment) = increment {
            body = Block::new(vec![body, Expression::new(increment)])
        }

        let for_condition = match condition {
            Some(condition) => condition,
            None => Literal::new(LoxValue::Boolean(true)),
        };

        body = While::new(for_condition, body);

        if let Some(initializer) = initializer {
            body = Block::new(vec![initializer, body])
        }

        body
    }

    fn if_statement(&mut self) -> Box<dyn Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")
            .ok();
        let condition = self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")
            .ok();

        let then_branch = self.statement();
        let else_branch = if self.matches(&[TokenType::Else]) {
            Some(self.statement())
        } else {
            None
        };

        If::new(condition, then_branch, else_branch)
    }

    fn print_statment(&mut self) -> Box<dyn Stmt> {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.")
            .ok();
        Print::new(expr)
    }

    fn return_statement(&mut self) -> Box<dyn Stmt> {
        let previous = self.previous();
        let value = if !self.check(TokenType::Semicolon) {
            self.expression()
        } else {
            Literal::nil()
        };

        self.consume(TokenType::Semicolon, "Expect ';' after value.")
            .ok();

        Return::new(previous, value)
    }

    fn while_statement(&mut self) -> Box<dyn Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after while.")
            .ok();
        let condition = self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after condition.")
            .ok();
        let body = self.statement();

        While::new(condition, body)
    }

    fn expression_statement(&mut self) -> Box<dyn Stmt> {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")
            .ok();
        Expression::new(expr)
    }

    fn block(&mut self) -> Vec<Box<dyn Stmt>> {
        let mut statements: Vec<Box<dyn Stmt>> = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_end() {
            statements.push(self.declaration());
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")
            .ok();
        statements
    }

    // Expressions
    fn assignment(&mut self) -> BoxedExpr {
        let expr = self.or();

        if self.matches(&[TokenType::Equal]) {
            let _equals = self.previous();
            let value = self.assignment();

            if let Some(variable_expr) = expr.as_any().downcast_ref::<Variable>() {
                let name = variable_expr.name();
                return Assign::new(name, value);
            }
        }

        expr
    }

    fn or(&mut self) -> BoxedExpr {
        let mut expr = self.and();

        while self.matches(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.and();
            expr = Logical::new(expr, operator, right);
        }

        expr
    }

    fn and(&mut self) -> BoxedExpr {
        let mut expr = self.equality();

        while self.matches(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality();
            expr = Logical::new(expr, operator, right);
        }

        expr
    }

    fn expression(&mut self) -> BoxedExpr {
        self.assignment()
    }

    fn equality(&mut self) -> BoxedExpr {
        let mut expr = self.comparison();

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Binary::new(expr, operator, right);
        }
        expr
    }

    fn comparison(&mut self) -> BoxedExpr {
        let mut expr = self.addition();

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::LessEqual,
            TokenType::Less,
        ]) {
            let operator = self.previous();
            let right = self.addition();
            expr = Binary::new(expr, operator, right);
        }
        expr
    }

    fn addition(&mut self) -> BoxedExpr {
        let mut expr = self.multiplication();

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.multiplication();
            expr = Binary::new(expr, operator, right);
        }
        expr
    }

    fn multiplication(&mut self) -> BoxedExpr {
        let mut expr = self.unary();

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Binary::new(expr, operator, right);
        }
        expr
    }

    fn unary(&mut self) -> BoxedExpr {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Unary::new(operator, right);
        }

        self.call()
    }

    fn call(&mut self) -> BoxedExpr {
        let mut expr = self.primary();

        loop {
            if self.matches(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr);
            } else {
                break;
            }
        }

        expr
    }

    fn finish_call(&mut self, callee: BoxedExpr) -> BoxedExpr {
        let mut arguments: Vec<BoxedExpr> = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() > 10 {
                    panic!("Cannot have more than 10 arguments");
                }

                arguments.push(self.expression());

                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        let paren = self
            .consume(TokenType::RightParen, "Expect ')' after arguments")
            .unwrap();

        Call::new(callee, paren, arguments)
    }

    fn primary(&mut self) -> BoxedExpr {
        if self.matches(&[TokenType::False]) {
            return Literal::new(LoxValue::Boolean(false));
        }

        if self.matches(&[TokenType::True]) {
            return Literal::new(LoxValue::Boolean(true));
        }

        if self.matches(&[TokenType::Nil]) {
            return Literal::new(LoxValue::Nil);
        }

        if self.matches(&[TokenType::Number]) {
            return Literal::new(LoxValue::Number(
                f32::from_str(&self.previous().lexeme()).unwrap(),
            ));
        }

        if self.matches(&[TokenType::String]) {
            return Literal::new(LoxValue::String(self.previous().lexeme()));
        }

        if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression")
                .ok();
            return Grouping::new(expr);
        }

        if self.matches(&[TokenType::Identifier]) {
            return Variable::new(self.previous());
        }

        Literal::new(LoxValue::Number(0.0))
    }

    // helper methods not part of the parsing grammar
    fn previous(&mut self) -> Token {
        self.tokens[self.index - 1].clone()
    }

    fn consume(&mut self, token_type: TokenType, error: &str) -> Result<Token, ParserError> {
        if self.check(token_type) {
            let token = self.next().unwrap();
            Ok(token)
        } else {
            Err(ParserError::new(error.to_string()))
        }
    }

    fn matches(&mut self, tokens: &[TokenType]) -> bool {
        for token_type in tokens {
            if self.check(*token_type) {
                self.next();
                return true;
            }
        }
        false
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if let Some(token) = self.peek() {
            return token.token_type() == token_type;
        } else {
            return false;
        }
    }

    fn peek(&mut self) -> Option<Token> {
        match self.tokens.get(self.index) {
            Some(token) => Some(token.clone()),
            None => None,
        }
    }

    fn is_end(&mut self) -> bool {
        match self.peek() {
            Some(token) => token.token_type() == TokenType::Eof,
            None => true,
        }
    }

    fn synchronize(&mut self) {
        self.next();

        while let Some(token) = self.peek() {
            if self.previous().token_type() == TokenType::Semicolon {
                return;
            }

            match token.token_type() {
                TokenType::Class => return,
                TokenType::Fun => return,
                TokenType::For => return,
                TokenType::If => return,
                TokenType::Var => return,
                TokenType::While => return,
                TokenType::Print => return,
                TokenType::Return => return,
                _ => (),
            }

            self.next();
        }
    }
}

impl Iterator for Parser {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.index < self.tokens.len() {
            let token = self.tokens[self.index].clone();
            self.index += 1;
            Some(token)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct ParserError {
    description: String,
}

impl ParserError {
    fn new(description: String) -> ParserError {
        ParserError { description }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for ParserError {}
