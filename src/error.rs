use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NepalError {
    #[error("Lexical error: {0}")]
    LexError(&'static str),

    #[error("Parse error: {0}")]
    ParseError(&'static str),

    #[error("Runtime error: {0}")]
    RuntimeError(&'static str),

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}
