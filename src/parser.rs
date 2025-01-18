use crate::ast::{Statement, Condition, Value};
use crate::error::NepalError;
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, NepalError> {
        let mut statements = Vec::new();
        while self.position < self.tokens.len() {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, NepalError> {
        match &self.tokens[self.position] {
            Token::OiMug => {
                self.position += 1;
                if matches!(self.tokens.get(self.position), Some(Token::Bhan)) {
                    self.position += 1;
                    self.parse_input()
                } else {
                    self.parse_declaration()
                }
            },
            Token::BolMug => self.parse_print(),
            Token::Mug => {
                self.position += 1;  // Skip 'mug'
                match &self.tokens[self.position] {
                    Token::Jod => {
                        self.position += 1;
                        self.parse_standalone_arithmetic(true)
                    },
                    Token::Ghata => {
                        self.position += 1;
                        self.parse_standalone_arithmetic(false)
                    },
                    _ => Err(NepalError::ParseError("Expected 'jod' or 'ghata' after 'mug'")),
                }
            },
            Token::Yedi => self.parse_if_statement(),
            Token::Aile => {
                self.position += 1;
                if let Some(Token::Feri) = self.tokens.get(self.position) {
                    self.position += 1;
                    self.parse_if_statement()
                } else {
                    Err(NepalError::ParseError("Expected 'feri' after 'aile'"))
                }
            },
            _ => Err(NepalError::ParseError("Unexpected token")),
        }
    }

    fn parse_if_statement(&mut self) -> Result<Statement, NepalError> {
        if matches!(self.tokens.get(self.position), Some(Token::Yedi)) {
            self.position += 1;
        }
        
        let var1 = if let Token::Identifier(name) = &self.tokens[self.position] {
            self.position += 1;
            name.clone()
        } else {
            return Err(NepalError::ParseError("Expected variable name after yedi"));
        };

        let condition = match &self.tokens[self.position] {
            Token::Babaal => {
                self.position += 1;
                true
            }
            Token::Laamo => {
                self.position += 1;
                false
            }
            _ => return Err(NepalError::ParseError("Expected babaal or laamo")),
        };

        let var2 = if let Token::String(s) = &self.tokens[self.position] {
            self.position += 1;
            s.clone()
        } else {
            return Err(NepalError::ParseError("Expected string literal"));
        };

        if !matches!(self.tokens.get(self.position), Some(Token::Bhane)) {
            return Err(NepalError::ParseError("Expected 'bhane' after condition"));
        }
        self.position += 1;

        let mut statements = Vec::new();
        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Sakiyo => {
                    self.position += 1;
                    break;
                }
                Token::Aile => break,
                _ => statements.push(self.parse_statement()?),
            }
        }

        Ok(Statement::If(
            if condition {
                Condition::Equals(var1, var2)
            } else {
                Condition::NotEquals(var1, var2)
            },
            statements,
            None,
        ))
    }

    fn parse_declaration(&mut self) -> Result<Statement, NepalError> {
        let name = if let Token::Identifier(name) = &self.tokens[self.position] {
            self.position += 1;
            name.clone()
        } else {
            return Err(NepalError::ParseError("Expected identifier"));
        };

        if let Token::Equals = &self.tokens[self.position] {
            self.position += 1;
        } else {
            return Err(NepalError::ParseError("Expected '='"));
        }

        match &self.tokens[self.position] {
            Token::Number(value) => {
                self.position += 1;
                Ok(Statement::Declaration(name, Value::Number(*value)))
            }
            Token::String(value) => {
                self.position += 1;
                if matches!(self.tokens.get(self.position), Some(Token::Plus)) {
                    self.position += 1;
                    let mut parts = vec![value.clone()];
                    
                    while self.position < self.tokens.len() {
                        match &self.tokens[self.position] {
                            Token::String(s) => {
                                parts.push(s.clone());
                                self.position += 1;
                            }
                            Token::Identifier(id) => {
                                parts.push(id.clone());
                                self.position += 1;
                            }
                            Token::Plus => {
                                self.position += 1;
                                continue;
                            }
                            _ => break,
                        }
                    }
                    Ok(Statement::StringConcat(name, parts))
                } else {
                    Ok(Statement::Declaration(name, Value::String(value.clone())))
                }
            }
            Token::Identifier(value) => {
                self.position += 1;
                if matches!(self.tokens.get(self.position), Some(Token::Plus)) {
                    self.position += 1;
                    let mut parts = vec![value.clone()];
                    
                    while self.position < self.tokens.len() {
                        match &self.tokens[self.position] {
                            Token::String(s) => {
                                parts.push(s.clone());
                                self.position += 1;
                            }
                            Token::Identifier(id) => {
                                parts.push(id.clone());
                                self.position += 1;
                            }
                            Token::Plus => {
                                self.position += 1;
                                continue;
                            }
                            _ => break,
                        }
                    }
                    Ok(Statement::StringConcat(name, parts))
                } else {
                    Ok(Statement::Declaration(name, Value::String(value.clone())))
                }
            }
            Token::Jod | Token::Ghata => self.parse_arithmetic_operation(name),
            _ => Err(NepalError::ParseError("Expected value")),
        }
    }

    fn parse_arithmetic_operation(&mut self, target: String) -> Result<Statement, NepalError> {
        let op = self.tokens[self.position].clone();
        self.position += 1;

        let mut sources = Vec::new();
        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Identifier(id) => {
                    sources.push(id.clone());
                    self.position += 1;
                    if matches!(self.tokens.get(self.position), Some(Token::Comma)) {
                        self.position += 1;
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        match op {
            Token::Jod => Ok(Statement::Addition(target, sources)),
            Token::Ghata => Ok(Statement::Subtraction(target, sources)),
            _ => unreachable!(),
        }
    }

    fn parse_standalone_arithmetic(&mut self, is_addition: bool) -> Result<Statement, NepalError> {
        let mut sources = Vec::new();
        
        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Identifier(name) => {
                    sources.push(name.clone());
                    self.position += 1;
                    
                    match self.tokens.get(self.position) {
                        Some(Token::Comma) => {
                            self.position += 1;
                            continue;
                        }
                        Some(Token::Lai) => {
                            self.position += 1;
                            break;
                        }
                        _ => return Err(NepalError::ParseError("Expected comma or 'lai'")),
                    }
                }
                _ => return Err(NepalError::ParseError("Expected identifier")),
            }
        }

        let target = if let Token::Identifier(name) = &self.tokens[self.position] {
            self.position += 1;
            name.clone()
        } else {
            return Err(NepalError::ParseError("Expected target identifier after 'lai'"));
        };

        if is_addition {
            Ok(Statement::Addition(target, sources))
        } else {
            Ok(Statement::Subtraction(target, sources))
        }
    }

    fn parse_print(&mut self) -> Result<Statement, NepalError> {
        self.position += 1; // Skip 'bol mug'
        if let Token::String(_) = &self.tokens[self.position] {
            let mut parts = Vec::new();
            while self.position < self.tokens.len() {
                match &self.tokens[self.position] {
                    Token::String(s) => {
                        parts.push(s.clone());
                        self.position += 1;
                    }
                    Token::Plus => {
                        self.position += 1;
                    }
                    Token::Identifier(name) => {
                        parts.push(format!("{{{}}}", name));
                        self.position += 1;
                    }
                    _ => break,
                }
            }
            Ok(Statement::PrintString(parts))
        } else {
            let name = if let Token::Identifier(name) = &self.tokens[self.position] {
                self.position += 1;
                name.clone()
            } else {
                return Err(NepalError::ParseError("Expected identifier or string"));
            };
            Ok(Statement::Print(name))
        }
    }

    fn parse_input(&mut self) -> Result<Statement, NepalError> {
        let name = if let Token::Identifier(name) = &self.tokens[self.position] {
            self.position += 1;
            name.clone()
        } else {
            return Err(NepalError::ParseError("Expected identifier after bhan"));
        };
        Ok(Statement::Input(name))
    }
}
