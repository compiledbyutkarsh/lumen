mod lexer;
mod parser;
mod compiler;
mod vm;

use lexer::Lexer;
use parser::Parser;
use compiler::bytecode::Compiler;
use vm::VM;

fn main() {
    let source = r#"
        fn main() {
            let x = 10;
            let y = 20;
            let sum = x + y;
            print(sum);

            if sum == 30 {
                print(1);
            } else {
                print(0);
            }

            print("Hello from Lumen!");
            print(10 + 20 * 3);
        }
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    let mut compiler = Compiler::new();
    compiler.compile_program(&program);

    let mut vm = VM::new(compiler.chunks);
    vm.run("main");
}
