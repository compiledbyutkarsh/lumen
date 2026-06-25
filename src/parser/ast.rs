#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Ident(String),

    BinOp {
        op: BinOpKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    UnaryOp {
        op: UnaryOpKind,
        expr: Box<Expr>,
    },

    Call {
        name: String,
        args: Vec<Expr>,
    },

    If {
        cond: Box<Expr>,
        then_block: Vec<Stmt>,
        else_block: Option<Vec<Stmt>>,
    },
}

#[derive(Debug, Clone)]
pub enum BinOpKind {
    Add, Sub, Mul, Div,
    Eq, NotEq, Lt, Gt, LtEq, GtEq,
}

#[derive(Debug, Clone)]
pub enum UnaryOpKind {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        value: Expr,
    },
    Return(Expr),
    Expr(Expr),
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
}
