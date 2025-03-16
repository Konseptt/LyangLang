use crate::bytecode::{BytecodeProgram, Opcode};
use crate::error::NepalError;
use std::io::{self, Write};

/// Runtime value representation in the VM
#[derive(Debug, Clone)]
pub enum Value {
    Number(i32),
    String(String),
    Boolean(bool),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
        }
    }
}

/// Virtual Machine for executing LyangLang bytecode
pub struct VM {
    program: BytecodeProgram,
    ip: usize,                // Instruction pointer
    stack: Vec<Value>,        // Operand stack
    variables: Vec<Value>,    // Variable storage
    running: bool,
}

impl VM {
    pub fn new(program: BytecodeProgram) -> Self {
        let var_count = program.variable_names.len();
        Self {
            program,
            ip: 0,
            stack: Vec::new(),
            variables: vec![Value::Number(0); var_count], // Initialize variables with default values
            running: false,
        }
    }

    /// Runs the bytecode program
    pub fn run(&mut self) -> Result<(), NepalError> {
        self.running = true;
        
        while self.running && self.ip < self.program.instructions.len() {
            let instruction = &self.program.instructions[self.ip];
            
            match instruction.opcode {
                // Stack operations
                Opcode::PushNumber(value) => {
                    self.stack.push(Value::Number(value));
                    self.ip += 1;
                },
                
                Opcode::PushString(index) => {
                    if let Some(s) = self.program.string_pool.get(index) {
                        self.stack.push(Value::String(s.clone()));
                        self.ip += 1;
                    } else {
                        return Err(NepalError::RuntimeError(
                            "String constant index out of bounds"
                        ));
                    }
                },
                
                Opcode::PushVariable(index) => {
                    if let Some(var_name) = self.program.variable_names.get(index) {
                        self.stack.push(Value::String(var_name.clone()));
                        self.ip += 1;
                    } else {
                        return Err(NepalError::RuntimeError(
                            "Variable index out of bounds"
                        ));
                    }
                },
                
                Opcode::Pop => {
                    if self.stack.pop().is_none() {
                        return Err(NepalError::RuntimeError("Stack underflow"));
                    }
                    self.ip += 1;
                },
                
                // Variable operations
                Opcode::StoreVariable(index) => {
                    if let Some(value) = self.stack.pop() {
                        if index < self.variables.len() {
                            self.variables[index] = value;
                            self.ip += 1;
                        } else {
                            return Err(NepalError::RuntimeError(
                                "Variable index out of bounds"
                            ));
                        }
                    } else {
                        return Err(NepalError::RuntimeError("Stack underflow"));
                    }
                },
                
                Opcode::LoadVariable(index) => {
                    if index < self.variables.len() {
                        self.stack.push(self.variables[index].clone());
                        self.ip += 1;
                    } else {
                        return Err(NepalError::RuntimeError(
                            "Variable index out of bounds"
                        ));
                    }
                },
                
                // Arithmetic operations
                Opcode::Add => {
                    if self.stack.len() < 2 {
                        return Err(NepalError::RuntimeError("Stack underflow"));
                    }
                    
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    
                    match (a, b) {
                        (Value::Number(a_val), Value::Number(b_val)) => {
                            self.stack.push(Value::Number(a_val + b_val));
                        },
                        (Value::String(a_str), Value::String(b_str)) => {
                            self.stack.push(Value::String(a_str + &b_str));
                        },
                        (Value::String(a_str), Value::Number(b_val)) => {
                            self.stack.push(Value::String(a_str + &b_val.to_string()));
                        },
                        (Value::Number(a_val), Value::String(b_str)) => {
                            self.stack.push(Value::String(a_val.to_string() + &b_str));
                        },
                        _ => {
                            return Err(NepalError::RuntimeError(
                                "Type error: Cannot add these types"
                            ));
                        }
                    }
                    
                    self.ip += 1;
                },
                
                Opcode::Subtract => {
                    if self.stack.len() < 2 {
                        return Err(NepalError::RuntimeError("Stack underflow"));
                    }
                    
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    
                    match (a, b) {
                        (Value::Number(a_val), Value::Number(b_val)) => {
                            self.stack.push(Value::Number(a_val - b_val));
                        },
                        _ => {
                            return Err(NepalError::RuntimeError(
                                "Type error: Cannot subtract these types"
                            ));
                        }
                    }
                    
                    self.ip += 1;
                },
                
                // String operations
                Opcode::Concat => {
                    if self.stack.len() < 2 {
                        return Err(NepalError::RuntimeError("Stack underflow"));
                    }
                    
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    
                    let result = match (a, b) {
                        (Value::String(a_str), Value::String(b_str)) => {
                            Value::String(a_str + &b_str)
                        },
                        (Value::String(a_str), Value::Number(b_val)) => {
                            Value::String(a_str + &b_val.to_string())
                        },
                        (Value::Number(a_val), Value::String(b_str)) => {
                            Value::String(a_val.to_string() + &b_str)
                        },
                        (Value::String(a_str), Value::Boolean(b_val)) => {
                            Value::String(a_str + &b_val.to_string())
                        },
                        (Value::Boolean(a_val), Value::String(b_str)) => {
                            Value::String(a_val.to_string() + &b_str)
                        },
                        _ => {
                            return Err(NepalError::RuntimeError(
                                "Type error: Cannot concatenate these types"
                            ));
                        }
                    };
                    
                    self.stack.push(result);
                    self.ip += 1;
                },
                
                // I/O operations
                Opcode::Print => {
                    if let Some(value) = self.stack.pop() {
                        println!("{}", value);
                        self.ip += 1;
                    } else {
                        return Err(NepalError::RuntimeError("Stack underflow"));
                    }
                },
                
                Opcode::Input => {
                    let mut input = String::new();
                    print!("> ");
                    io::stdout().flush().map_err(|_| NepalError::RuntimeError("IO error"))?;
                    io::stdin()
                        .read_line(&mut input)
                        .map_err(|_| NepalError::RuntimeError("Failed to read input"))?;
                    
                    // Trim newline character
                    let input = input.trim().to_string();
                    self.stack.push(Value::String(input));
                    self.ip += 1;
                },
                
                // Control flow
                Opcode::JumpIfTrue(address) => {
                    if let Some(condition) = self.stack.pop() {
                        match condition {
                            Value::Boolean(true) => self.ip = address,
                            Value::Boolean(false) => self.ip += 1,
                            _ => {
                                return Err(NepalError::RuntimeError(
                                    "Type error: Condition must be boolean"
                                ));
                            }
                        }
                    } else {
                        return Err(NepalError::RuntimeError("Stack underflow"));
                    }
                },
                
                Opcode::JumpIfFalse(address) => {
                    if let Some(condition) = self.stack.pop() {
                        match condition {
                            Value::Boolean(false) => self.ip = address,
                            Value::Boolean(true) => self.ip += 1,
                            _ => {
                                return Err(NepalError::RuntimeError(
                                    "Type error: Condition must be boolean"
                                ));
                            }
                        }
                    } else {
                        return Err(NepalError::RuntimeError("Stack underflow"));
                    }
                },
                
                Opcode::Jump(address) => {
                    self.ip = address;
                },
                
                // Comparison
                Opcode::Equal => {
                    if self.stack.len() < 2 {
                        return Err(NepalError::RuntimeError("Stack underflow"));
                    }
                    
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    
                    let result = match (a, b) {
                        (Value::Number(a_val), Value::Number(b_val)) => {
                            Value::Boolean(a_val == b_val)
                        },
                        (Value::String(a_str), Value::String(b_str)) => {
                            Value::Boolean(a_str == b_str)
                        },
                        (Value::Boolean(a_val), Value::Boolean(b_val)) => {
                            Value::Boolean(a_val == b_val)
                        },
                        _ => {
                            return Err(NepalError::RuntimeError(
                                "Type error: Cannot compare these types"
                            ));
                        }
                    };
                    
                    self.stack.push(result);
                    self.ip += 1;
                },
                
                Opcode::NotEqual => {
                    if self.stack.len() < 2 {
                        return Err(NepalError::RuntimeError("Stack underflow"));
                    }
                    
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    
                    let result = match (a, b) {
                        (Value::Number(a_val), Value::Number(b_val)) => {
                            Value::Boolean(a_val != b_val)
                        },
                        (Value::String(a_str), Value::String(b_str)) => {
                            Value::Boolean(a_str != b_str)
                        },
                        (Value::Boolean(a_val), Value::Boolean(b_val)) => {
                            Value::Boolean(a_val != b_val)
                        },
                        _ => {
                            return Err(NepalError::RuntimeError(
                                "Type error: Cannot compare these types"
                            ));
                        }
                    };
                    
                    self.stack.push(result);
                    self.ip += 1;
                },
                
                // Program flow
                Opcode::Return => {
                    // Simply increment instruction pointer
                    self.ip += 1;
                },
                
                Opcode::Halt => {
                    self.running = false;
                },
            }
        }
        
        Ok(())
    }

    /// Gets the top value from the stack without removing it
    #[allow(dead_code)]
    pub fn peek(&self) -> Option<&Value> {
        self.stack.last()
    }

    /// Clears the stack and resets instruction pointer
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.ip = 0;
        self.stack.clear();
        self.running = false;
        // Clear variables to default values
        self.variables = vec![Value::Number(0); self.variables.len()];
    }
}