# Lumen 🔆

A bytecode compiler and stack-based virtual machine built from scratch in Rust.

Lumen implements a complete language pipeline:
**Source Code → Lexer → Parser → AST → Bytecode Compiler → VM Execution**

## Architecture
## Language Features

- **Types**: integers, floats, strings, booleans
- **Variables**: `let x = 10;`
- **Arithmetic**: `+`, `-`, `*`, `/`
- **Comparisons**: `==`, `!=`, `<`, `>`, `<=`, `>=`
- **Control flow**: `if / else`, `while`
- **Functions**: `fn name(params) { ... }`
- **Built-in**: `print(value)`

## Example

```lumen
fn main() {
    let x = 10;
    let y = 20;
    let sum = x + y;
    print(sum);          // 30

    if sum == 30 {
        print(1);        // 1
    }

    print("Hello from Lumen!");
    print(10 + 20 * 3);  // 70
}
```

## Build & Run

```bash
cargo build
cargo run
```

## Tech

- **Language**: Rust
- **Parsing**: Hand-written recursive descent parser
- **Execution**: Stack-based bytecode VM with local variable slots
- **No dependencies**: zero external crates
