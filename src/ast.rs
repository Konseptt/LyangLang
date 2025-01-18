#[derive(Debug)]
pub enum Statement {
    Declaration(String, Value),
    Addition(String, Vec<String>),
    Subtraction(String, Vec<String>),
    StringConcat(String, Vec<String>), // New: for string concatenation
    Print(String),
    PrintString(Vec<String>),
    Input(String),
    If(Condition, Vec<Statement>, Option<Box<Statement>>),  // Added Option<Box<Statement>> for else branch
}

#[derive(Debug)]
pub enum Condition {
    Equals(String, String),
    NotEquals(String, String),
}

#[derive(Debug)]
pub enum Value {
    Number(i32),
    String(String),
}
