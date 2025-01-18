mod token;
mod error;
mod ast;
mod lexer;
mod parser;
mod interpreter;

use crate::error::NepalError;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;

fn main() -> Result<(), NepalError> {
    let input = std::fs::read_to_string("example.nbh")
        .map_err(|_| NepalError::RuntimeError("Failed to read file"))?;

    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    
    match parser.parse() {
        Ok(statements) => {
            let mut interpreter = Interpreter::new();
            for statement in statements {
                interpreter.execute(statement);
            }
            Ok(())
        }
        Err(e) => Err(e)
    }
}
