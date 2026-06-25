#[derive(Debug, Clone)]
pub enum Instruction {
    // Stack ops
    PushInt(i64),
    PushFloat(f64),
    PushStr(String),
    PushBool(bool),
    Pop,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Neg,

    // Comparison
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,

    // Logic
    Not,

    // Variables
    LoadLocal(usize),
    StoreLocal(usize),

    // Control flow
    Jump(usize),
    JumpIfFalse(usize),

    // Functions
    Call(String, usize),  // name, arg count
    Return,

    // Built-ins
    Print,

    Halt,
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub instructions: Vec<Instruction>,
    pub name: String,
}

impl Chunk {
    pub fn new(name: &str) -> Self {
        Self {
            instructions: Vec::new(),
            name: name.to_string(),
        }
    }

    pub fn emit(&mut self, instr: Instruction) -> usize {
        self.instructions.push(instr);
        self.instructions.len() - 1
    }

    pub fn patch(&mut self, idx: usize, instr: Instruction) {
        self.instructions[idx] = instr;
    }
}
