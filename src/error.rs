use std::fmt;

#[derive(Debug)]
pub enum NepalError {
    LexError(&'static str),
    ParseError(&'static str),
    RuntimeError(&'static str),
}

impl fmt::Display for NepalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NepalError::LexError(msg) => write!(f, "Lexer Error: {}", msg),
            NepalError::ParseError(msg) => write!(f, "Parser Error: {}", msg),
            NepalError::RuntimeError(msg) => write!(f, "Runtime Error: {}", msg),
        }
    }
}
