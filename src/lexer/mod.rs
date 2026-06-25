pub mod token;

use token::{Token, TokenKind};

pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
        }
    }

    fn peek(&self) -> char {
        self.source.get(self.pos).copied().unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        let c = self.peek();
        self.pos += 1;
        if c == '\n' { self.line += 1; }
        c
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_whitespace() {
            self.advance();
        }
    }

    fn read_number(&mut self) -> TokenKind {
        let mut s = String::new();
        let mut is_float = false;
        while self.peek().is_ascii_digit() || self.peek() == '.' {
            if self.peek() == '.' { is_float = true; }
            s.push(self.advance());
        }
        if is_float {
            TokenKind::Float(s.parse().unwrap())
        } else {
            TokenKind::Int(s.parse().unwrap())
        }
    }

    fn read_string(&mut self) -> TokenKind {
        self.advance(); // skip opening quote
        let mut s = String::new();
        while self.peek() != '"' && self.peek() != '\0' {
            s.push(self.advance());
        }
        self.advance(); // skip closing quote
        TokenKind::Str(s)
    }

    fn read_ident(&mut self) -> TokenKind {
        let mut s = String::new();
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            s.push(self.advance());
        }
        match s.as_str() {
            "let"    => TokenKind::Let,
            "fn"     => TokenKind::Fn,
            "return" => TokenKind::Return,
            "if"     => TokenKind::If,
            "else"   => TokenKind::Else,
            "while"  => TokenKind::While,
            "true"   => TokenKind::Bool(true),
            "false"  => TokenKind::Bool(false),
            _        => TokenKind::Ident(s),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace();
            let line = self.line;
            let c = self.peek();
            if c == '\0' {
                tokens.push(Token::new(TokenKind::EOF, line));
                break;
            }
            let kind = match c {
                '0'..='9' => self.read_number(),
                '"'       => self.read_string(),
                'a'..='z' | 'A'..='Z' | '_' => self.read_ident(),
                '+'  => { self.advance(); TokenKind::Plus }
                '-'  => { self.advance();
                           if self.peek() == '>' { self.advance(); TokenKind::Arrow }
                           else { TokenKind::Minus } }
                '*'  => { self.advance(); TokenKind::Star }
                '/'  => { self.advance(); TokenKind::Slash }
                '='  => { self.advance();
                           if self.peek() == '=' { self.advance(); TokenKind::EqEq }
                           else { TokenKind::Eq } }
                '!'  => { self.advance();
                           if self.peek() == '=' { self.advance(); TokenKind::BangEq }
                           else { TokenKind::Bang } }
                '<'  => { self.advance();
                           if self.peek() == '=' { self.advance(); TokenKind::LtEq }
                           else { TokenKind::Lt } }
                '>'  => { self.advance();
                           if self.peek() == '=' { self.advance(); TokenKind::GtEq }
                           else { TokenKind::Gt } }
                '('  => { self.advance(); TokenKind::LParen }
                ')'  => { self.advance(); TokenKind::RParen }
                '{'  => { self.advance(); TokenKind::LBrace }
                '}'  => { self.advance(); TokenKind::RBrace }
                ','  => { self.advance(); TokenKind::Comma }
                ';'  => { self.advance(); TokenKind::Semicolon }
                ':'  => { self.advance(); TokenKind::Colon }
                _    => { self.advance(); continue }
            };
            tokens.push(Token::new(kind, line));
        }
        tokens
    }
}
