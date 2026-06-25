pub mod instruction;

use instruction::{Chunk, Instruction};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Int(n)   => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::Str(s)   => write!(f, "{}", s),
            Value::Bool(b)  => write!(f, "{}", b),
            Value::Nil      => write!(f, "nil"),
        }
    }
}

pub struct VM {
    stack: Vec<Value>,
    locals: Vec<Value>,
    chunks: HashMap<String, Chunk>,
}

impl VM {
    pub fn new(chunks: HashMap<String, Chunk>) -> Self {
        Self {
            stack: Vec::new(),
            locals: vec![Value::Nil; 256],
            chunks,
        }
    }

    fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap_or(Value::Nil)
    }

    pub fn run(&mut self, entry: &str) {
        let chunk = match self.chunks.get(entry) {
            Some(c) => c.clone(),
            None => { eprintln!("Error: function '{}' not found", entry); return; }
        };
        self.exec(&chunk);
    }

    fn exec(&mut self, chunk: &Chunk) {
        let mut ip = 0;
        let instrs = chunk.instructions.clone();

        while ip < instrs.len() {
            match &instrs[ip] {
                Instruction::PushInt(n)   => self.push(Value::Int(*n)),
                Instruction::PushFloat(f) => self.push(Value::Float(*f)),
                Instruction::PushStr(s)   => self.push(Value::Str(s.clone())),
                Instruction::PushBool(b)  => self.push(Value::Bool(*b)),
                Instruction::Pop          => { self.pop(); }

                Instruction::Add => {
                    let b = self.pop(); let a = self.pop();
                    self.push(match (a, b) {
                        (Value::Int(x),   Value::Int(y))   => Value::Int(x + y),
                        (Value::Float(x), Value::Float(y)) => Value::Float(x + y),
                        (Value::Str(x),   Value::Str(y))   => Value::Str(x + &y),
                        _ => Value::Nil,
                    });
                }
                Instruction::Sub => {
                    let b = self.pop(); let a = self.pop();
                    self.push(match (a, b) {
                        (Value::Int(x),   Value::Int(y))   => Value::Int(x - y),
                        (Value::Float(x), Value::Float(y)) => Value::Float(x - y),
                        _ => Value::Nil,
                    });
                }
                Instruction::Mul => {
                    let b = self.pop(); let a = self.pop();
                    self.push(match (a, b) {
                        (Value::Int(x),   Value::Int(y))   => Value::Int(x * y),
                        (Value::Float(x), Value::Float(y)) => Value::Float(x * y),
                        _ => Value::Nil,
                    });
                }
                Instruction::Div => {
                    let b = self.pop(); let a = self.pop();
                    self.push(match (a, b) {
                        (Value::Int(x),   Value::Int(y))   => Value::Int(x / y),
                        (Value::Float(x), Value::Float(y)) => Value::Float(x / y),
                        _ => Value::Nil,
                    });
                }
                Instruction::Neg => {
                    let a = self.pop();
                    self.push(match a {
                        Value::Int(x)   => Value::Int(-x),
                        Value::Float(x) => Value::Float(-x),
                        _ => Value::Nil,
                    });
                }

                Instruction::Eq    => { let b = self.pop(); let a = self.pop(); self.push(Value::Bool(format!("{}", a) == format!("{}", b))); }
                Instruction::NotEq => { let b = self.pop(); let a = self.pop(); self.push(Value::Bool(format!("{}", a) != format!("{}", b))); }
                Instruction::Lt    => { let b = self.pop(); let a = self.pop(); self.push(match (a,b) { (Value::Int(x), Value::Int(y)) => Value::Bool(x < y), _ => Value::Bool(false) }); }
                Instruction::Gt    => { let b = self.pop(); let a = self.pop(); self.push(match (a,b) { (Value::Int(x), Value::Int(y)) => Value::Bool(x > y), _ => Value::Bool(false) }); }
                Instruction::LtEq  => { let b = self.pop(); let a = self.pop(); self.push(match (a,b) { (Value::Int(x), Value::Int(y)) => Value::Bool(x <= y), _ => Value::Bool(false) }); }
                Instruction::GtEq  => { let b = self.pop(); let a = self.pop(); self.push(match (a,b) { (Value::Int(x), Value::Int(y)) => Value::Bool(x >= y), _ => Value::Bool(false) }); }
                Instruction::Not   => { let a = self.pop(); self.push(match a { Value::Bool(b) => Value::Bool(!b), _ => Value::Bool(false) }); }

                Instruction::LoadLocal(idx)  => { self.push(self.locals[*idx].clone()); }
                Instruction::StoreLocal(idx) => { let v = self.pop(); self.locals[*idx] = v; }

                Instruction::Jump(target)        => { ip = *target; continue; }
                Instruction::JumpIfFalse(target) => {
                    let v = self.pop();
                    if let Value::Bool(false) = v { ip = *target; continue; }
                }

                Instruction::Print => {
                    let v = self.pop();
                    println!("{}", v);
                }

                Instruction::Call(name, _argc) => {
                    let chunk = match self.chunks.get(name) {
                        Some(c) => c.clone(),
                        None => { eprintln!("Error: function '{}' not found", name); break; }
                    };
                    self.exec(&chunk);
                }

                Instruction::Return => { break; }
                Instruction::Halt   => { break; }
            }
            ip += 1;
        }
    }
}
