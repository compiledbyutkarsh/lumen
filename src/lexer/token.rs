#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),

    // Identifiers & keywords
    Ident(String),
    Let,
    Fn,
    Return,
    If,
    Else,
    While,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Eq,
    EqEq,
    Bang,
    BangEq,
    Lt,
    Gt,
    LtEq,
    GtEq,

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Colon,
    Arrow,

    // Special
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize) -> Self {
        Self { kind, line }
    }
}
