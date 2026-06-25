use crate::parser::ast::*;
use crate::vm::instruction::{Chunk, Instruction};
use std::collections::HashMap;

pub struct Compiler {
    pub chunks: HashMap<String, Chunk>,
    locals: Vec<HashMap<String, usize>>,
    local_count: usize,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            locals: Vec::new(),
            local_count: 0,
        }
    }

    pub fn compile_program(&mut self, program: &Program) {
        for func in &program.functions {
            self.compile_function(func);
        }
    }

    fn compile_function(&mut self, func: &Function) {
        let mut chunk = Chunk::new(&func.name);
        self.locals.push(HashMap::new());
        self.local_count = 0;

        // bind params as locals
        for param in &func.params {
            let idx = self.local_count;
            self.local_count += 1;
            self.locals.last_mut().unwrap().insert(param.clone(), idx);
        }

        for stmt in &func.body {
            self.compile_stmt(stmt, &mut chunk);
        }

        chunk.emit(Instruction::Halt);
        self.locals.pop();
        self.chunks.insert(func.name.clone(), chunk);
    }

    fn compile_stmt(&mut self, stmt: &Stmt, chunk: &mut Chunk) {
        match stmt {
            Stmt::Let { name, value } => {
                self.compile_expr(value, chunk);
                let idx = self.local_count;
                self.local_count += 1;
                self.locals.last_mut().unwrap().insert(name.clone(), idx);
                chunk.emit(Instruction::StoreLocal(idx));
            }
            Stmt::Return(expr) => {
                self.compile_expr(expr, chunk);
                chunk.emit(Instruction::Return);
            }
            Stmt::Expr(expr) => {
                self.compile_expr(expr, chunk);
                chunk.emit(Instruction::Pop);
            }
            Stmt::While { cond, body } => {
                let loop_start = chunk.instructions.len();
                self.compile_expr(cond, chunk);
                let jump_idx = chunk.emit(Instruction::JumpIfFalse(0));
                for s in body {
                    self.compile_stmt(s, chunk);
                }
                chunk.emit(Instruction::Jump(loop_start));
                let after = chunk.instructions.len();
                chunk.patch(jump_idx, Instruction::JumpIfFalse(after));
            }
        }
    }

    fn compile_expr(&mut self, expr: &Expr, chunk: &mut Chunk) {
        match expr {
            Expr::Int(n)   => { chunk.emit(Instruction::PushInt(*n)); }
            Expr::Float(f) => { chunk.emit(Instruction::PushFloat(*f)); }
            Expr::Str(s)   => { chunk.emit(Instruction::PushStr(s.clone())); }
            Expr::Bool(b)  => { chunk.emit(Instruction::PushBool(*b)); }

            Expr::Ident(name) => {
                if name == "print" { return; }
                let idx = self.locals.last()
                    .and_then(|m| m.get(name))
                    .copied()
                    .unwrap_or(0);
                chunk.emit(Instruction::LoadLocal(idx));
            }

            Expr::BinOp { op, left, right } => {
                self.compile_expr(left, chunk);
                self.compile_expr(right, chunk);
                let instr = match op {
                    BinOpKind::Add   => Instruction::Add,
                    BinOpKind::Sub   => Instruction::Sub,
                    BinOpKind::Mul   => Instruction::Mul,
                    BinOpKind::Div   => Instruction::Div,
                    BinOpKind::Eq    => Instruction::Eq,
                    BinOpKind::NotEq => Instruction::NotEq,
                    BinOpKind::Lt    => Instruction::Lt,
                    BinOpKind::Gt    => Instruction::Gt,
                    BinOpKind::LtEq  => Instruction::LtEq,
                    BinOpKind::GtEq  => Instruction::GtEq,
                };
                chunk.emit(instr);
            }

            Expr::UnaryOp { op, expr } => {
                self.compile_expr(expr, chunk);
                match op {
                    UnaryOpKind::Neg => { chunk.emit(Instruction::Neg); }
                    UnaryOpKind::Not => { chunk.emit(Instruction::Not); }
                };
            }

            Expr::Call { name, args } => {
                if name == "print" {
                    for arg in args { self.compile_expr(arg, chunk); }
                    chunk.emit(Instruction::Print);
                } else {
                    for arg in args { self.compile_expr(arg, chunk); }
                    chunk.emit(Instruction::Call(name.clone(), args.len()));
                }
            }

            Expr::If { cond, then_block, else_block } => {
                self.compile_expr(cond, chunk);
                let jump_idx = chunk.emit(Instruction::JumpIfFalse(0));
                for s in then_block { self.compile_stmt(s, chunk); }
                let else_jump = chunk.emit(Instruction::Jump(0));
                let else_start = chunk.instructions.len();
                chunk.patch(jump_idx, Instruction::JumpIfFalse(else_start));
                if let Some(else_stmts) = else_block {
                    for s in else_stmts { self.compile_stmt(s, chunk); }
                }
                let after = chunk.instructions.len();
                chunk.patch(else_jump, Instruction::Jump(after));
            }
        }
    }
}
