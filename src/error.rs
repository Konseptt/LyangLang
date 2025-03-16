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

    #[error("Type error: {0}")]
    TypeError(&'static str),
    
    #[error("Name error: {0}")]
    NameError(&'static str),
    
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    
    #[error("Error at line {line}: {error_type}\nDetails: {message}\nCode: {code}\n{pointer}")]
    FormattedError {
        line: usize,
        error_type: String,
        message: String,
        code: String,
        pointer: String,
    },
}

impl NepalError {
    /// Create a formatted error with line number and code snippet
    pub fn with_location(
        error: NepalError, 
        line: usize, 
        code: &str, 
        column: usize
    ) -> Self {
        let error_message = error.to_string();
        let error_parts: Vec<&str> = error_message.split(':').collect();
        let error_type = error_parts.get(0).unwrap_or(&"Error").to_string();
        let message = error_parts.get(1).unwrap_or(&"Unknown error").trim().to_string();
        
        let code_line = code.lines().nth(line - 1).unwrap_or("").to_string();
        let pointer = format!("{}^---- Error location", " ".repeat(column));
        
        Self::FormattedError {
            line,
            error_type,
            message,
            code: code_line,
            pointer,
        }
    }
    
    /// Convert standard Nepali error messages
    pub fn to_nepali_message(&self) -> String {
        match self {
            NepalError::NameError(_) => "अपरिभाषित चर".to_string(),
            NepalError::TypeError(_) => "प्रकार बेमेल".to_string(),
            NepalError::RuntimeError(_) if self.to_string().contains("Invalid operation") => 
                "अमान्य संचालन".to_string(),
            NepalError::ParseError(_) => "वाक्य रचना त्रुटि".to_string(),
            _ => self.to_string()
        }
    }
}
