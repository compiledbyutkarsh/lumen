pub mod ast;

use crate::lexer::token::{Token, TokenKind};
use ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &TokenKind {
        &self.tokens[self.pos].kind
    }

    fn advance(&mut self) -> &TokenKind {
        let k = &self.tokens[self.pos].kind;
        if self.pos + 1 < self.tokens.len() { self.pos += 1; }
        k
    }

    fn expect(&mut self, expected: &TokenKind) -> bool {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut functions = Vec::new();
        while self.peek() != &TokenKind::EOF {
            if self.peek() == &TokenKind::Fn {
                functions.push(self.parse_function());
            } else {
                self.advance();
            }
        }
        Program { functions }
    }

    fn parse_function(&mut self) -> Function {
        self.advance(); // fn
        let name = if let TokenKind::Ident(n) = self.advance().clone() { n } else { "unknown".to_string() };
        self.expect(&TokenKind::LParen);
        let mut params = Vec::new();
        while self.peek() != &TokenKind::RParen && self.peek() != &TokenKind::EOF {
            if let TokenKind::Ident(p) = self.advance().clone() { params.push(p); }
            if self.peek() == &TokenKind::Comma { self.advance(); }
        }
        self.expect(&TokenKind::RParen);
        self.expect(&TokenKind::LBrace);
        let body = self.parse_block();
        Function { name, params, body }
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.peek() != &TokenKind::RBrace && self.peek() != &TokenKind::EOF {
            stmts.push(self.parse_stmt());
        }
        self.expect(&TokenKind::RBrace);
        stmts
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.peek().clone() {
            TokenKind::Let => {
                self.advance();
                let name = if let TokenKind::Ident(n) = self.advance().clone() { n } else { "x".to_string() };
                self.expect(&TokenKind::Eq);
                let value = self.parse_expr();
                self.expect(&TokenKind::Semicolon);
                Stmt::Let { name, value }
            }
            TokenKind::Return => {
                self.advance();
                let expr = self.parse_expr();
                self.expect(&TokenKind::Semicolon);
                Stmt::Return(expr)
            }
            TokenKind::While => {
                self.advance();
                let cond = self.parse_expr();
                self.expect(&TokenKind::LBrace);
                let body = self.parse_block();
                Stmt::While { cond, body }
            }
            _ => {
                let expr = self.parse_expr();
                self.expect(&TokenKind::Semicolon);
                Stmt::Expr(expr)
            }
        }
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut left = self.parse_additive();
        loop {
            let op = match self.peek() {
                TokenKind::EqEq  => BinOpKind::Eq,
                TokenKind::BangEq => BinOpKind::NotEq,
                TokenKind::Lt    => BinOpKind::Lt,
                TokenKind::Gt    => BinOpKind::Gt,
                TokenKind::LtEq  => BinOpKind::LtEq,
                TokenKind::GtEq  => BinOpKind::GtEq,
                _ => break,
            };
            self.advance();
            let right = self.parse_additive();
            left = Expr::BinOp { op, left: Box::new(left), right: Box::new(right) };
        }
        left
    }

    fn parse_additive(&mut self) -> Expr {
        let mut left = self.parse_multiplicative();
        loop {
            let op = match self.peek() {
                TokenKind::Plus  => BinOpKind::Add,
                TokenKind::Minus => BinOpKind::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative();
            left = Expr::BinOp { op, left: Box::new(left), right: Box::new(right) };
        }
        left
    }

    fn parse_multiplicative(&mut self) -> Expr {
        let mut left = self.parse_unary();
        loop {
            let op = match self.peek() {
                TokenKind::Star  => BinOpKind::Mul,
                TokenKind::Slash => BinOpKind::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary();
            left = Expr::BinOp { op, left: Box::new(left), right: Box::new(right) };
        }
        left
    }

    fn parse_unary(&mut self) -> Expr {
        match self.peek().clone() {
            TokenKind::Minus => { self.advance(); Expr::UnaryOp { op: UnaryOpKind::Neg, expr: Box::new(self.parse_primary()) } }
            TokenKind::Bang  => { self.advance(); Expr::UnaryOp { op: UnaryOpKind::Not, expr: Box::new(self.parse_primary()) } }
            _ => self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Expr {
        match self.peek().clone() {
            TokenKind::Int(n)   => { self.advance(); Expr::Int(n) }
            TokenKind::Float(f) => { self.advance(); Expr::Float(f) }
            TokenKind::Str(s)   => { self.advance(); Expr::Str(s) }
            TokenKind::Bool(b)  => { self.advance(); Expr::Bool(b) }
            TokenKind::Ident(name) => {
                self.advance();
                if self.peek() == &TokenKind::LParen {
                    self.advance();
                    let mut args = Vec::new();
                    while self.peek() != &TokenKind::RParen && self.peek() != &TokenKind::EOF {
                        args.push(self.parse_expr());
                        if self.peek() == &TokenKind::Comma { self.advance(); }
                    }
                    self.expect(&TokenKind::RParen);
                    Expr::Call { name, args }
                } else {
                    Expr::Ident(name)
                }
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expr();
                self.expect(&TokenKind::RParen);
                expr
            }
            TokenKind::If => {
                self.advance();
                let cond = self.parse_expr();
                self.expect(&TokenKind::LBrace);
                let then_block = self.parse_block();
                let else_block = if self.peek() == &TokenKind::Else {
                    self.advance();
                    self.expect(&TokenKind::LBrace);
                    Some(self.parse_block())
                } else { None };
                Expr::If { cond: Box::new(cond), then_block, else_block }
            }
            _ => { self.advance(); Expr::Int(0) }
        }
    }
}
