mod token;
mod error;
mod ast;
mod lexer;
mod parser;
mod interpreter;
mod bytecode;
mod vm;
mod compiler;

use crate::error::NepalError;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::compiler::Compiler;
use crate::vm::VM;
use std::env;

fn main() -> Result<(), NepalError> {
    let args: Vec<String> = env::args().collect();
    
    // Check if filename is provided
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "example.nbh" // Default file
    };
    
    // Check if VM mode is enabled
    let use_vm = args.len() > 2 && args[2] == "--vm";
    
    let input = std::fs::read_to_string(filename)
        .map_err(|_| NepalError::RuntimeError("Failed to read file"))?;

    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    
    match parser.parse() {
        Ok(statements) => {
            if use_vm {
                // Use Lyangpiler VM
                println!("Running with Lyangpiler VM");
                
                // Compile the AST to bytecode
                let mut compiler = Compiler::new();
                let program = compiler.compile(statements)?;
                
                // Create and run the VM
                let mut vm = VM::new(program);
                vm.run()?;
                
                println!("Program execution completed.");
            } else {
                // Use the original interpreter
                let mut interpreter = interpreter::Interpreter::new();
                for statement in statements {
                    interpreter.execute(statement);
                }
            }
            Ok(())
        }
        Err(e) => Err(e)
    }
}
