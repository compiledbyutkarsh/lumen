<div align="center">

# 🔆 Lumen

**A bytecode compiler and stack-based virtual machine — built from scratch in Rust.**

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-yellow?style=for-the-badge)
![Status](https://img.shields.io/badge/Status-Active-brightgreen?style=for-the-badge)

*No LLVM. No external crates. Just raw systems programming.*

</div>

---

## What is Lumen?

Lumen is a fully hand-crafted language runtime — every component written from scratch in Rust with zero external dependencies. It takes source code through a complete compilation pipeline and executes it on a custom virtual machine.

This is not a tutorial project. It is a ground-up implementation of the core machinery that powers real language runtimes like CPython, the JVM, and Lua.

---

## Architecture

| Module | File | Role |
|--------|------|------|
| lexer | mod.rs | Character stream to token stream |
| lexer | token.rs | Token types: literals, keywords, operators |
| parser | mod.rs | Recursive descent parser: tokens to AST |
| parser | ast.rs | AST nodes: Expr, Stmt, Function, Program |
| compiler | bytecode.rs | AST walker: emits bytecode, resolves locals |
| vm | mod.rs | Stack-based VM: fetch, decode, execute loop |
| vm | instruction.rs | Instruction set and Chunk (code unit) |

### Lexer
Hand-written character-level scanner. Handles integer and float literals, string literals, identifiers, keywords, and all operators including multi-character tokens.

### Parser
Recursive descent parser with correct operator precedence: comparison, additive, multiplicative, unary, primary.

Produces a typed AST with full support for expressions, statements, functions, and control flow.

### Bytecode Compiler
Tree-walking compiler that traverses the AST and emits a flat instruction stream per function. Handles local variable slot allocation, forward jump patching for if/else and while, and built-in function resolution.

### Virtual Machine
Register-free, stack-based execution engine with an operand stack for all expression evaluation, fixed-size local variable slots with O(1) access, and direct jump support for control flow.

---

## Language

### Types

| Type | Example |
|------|---------|
| Integer | 42, -7 |
| Float | 3.14, -0.5 |
| String | "hello" |
| Boolean | true, false |

### Example

```rust
fn main() {
    let x = 10;
    let y = 20;
    let sum = x + y;
    print(sum);

    if sum == 30 {
        print("correct!");
    } else {
        print("wrong");
    }

    print("Hello from Lumen!");
    print(10 + 20 * 3);
}
```

### Instruction Set

| Instruction | Description |
|-------------|-------------|
| PushInt, PushFloat, PushStr, PushBool | Push literal onto stack |
| Add, Sub, Mul, Div, Neg | Arithmetic |
| Eq, NotEq, Lt, Gt, LtEq, GtEq | Comparison |
| LoadLocal, StoreLocal | Variable access |
| Jump, JumpIfFalse | Control flow |
| Call, Return | Function calls |
| Print | Built-in output |
| Halt | Stop execution |

---

## Build and Run

Requirements: Rust 1.70+

```bash
git clone https://github.com/compiledbyutkarsh/lumen.git
cd lumen
cargo run
```

No dependencies. No setup. Just cargo.

---

## Why Build This?

Most programmers use languages. Few understand what happens between source code and execution. Building Lumen required implementing a lexer that handles edge cases in number and string scanning, a parser with correct precedence climbing and recursive descent, a compiler that resolves variable scopes and patches forward jumps, and a VM with a proper fetch-decode-execute loop.

This is the same architecture used by CPython, Lua 5.x, and early Ruby MRI.

---

## Roadmap

- [ ] REPL — interactive shell
- [ ] Proper error reporting with line numbers
- [ ] Arrays and hashmaps
- [ ] First-class functions and closures
- [ ] JIT compilation via Cranelift

---

<div align="center">
<sub>Made with 🔆 by <a href="https://github.com/compiledbyutkarsh">compiledbyutkarsh</a></sub>
</div>
