use crate::bytecode::{BytecodeProgram, Opcode};
use crate::error::NepalError;
use std::io::{self, Write};

/// Runtime value representation in the VM
#[derive(Debug, Clone, PartialEq)]
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
                    // Push the variable's stored value (mirrors LoadVariable),
                    // not its name. The variables table is keyed by index.
                    if index < self.variables.len() {
                        self.stack.push(self.variables[index].clone());
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
                            match a_val.checked_add(b_val) {
                                Some(r) => self.stack.push(Value::Number(r)),
                                None => return Err(NepalError::RuntimeError("arithmetic overflow")),
                            }
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
                            match a_val.checked_sub(b_val) {
                                Some(r) => self.stack.push(Value::Number(r)),
                                None => return Err(NepalError::RuntimeError("arithmetic overflow")),
                            }
                        },
                        _ => {
                            return Err(NepalError::RuntimeError(
                                "Type error: Cannot subtract these types"
                            ));
                        }
                    }
                    
                    self.ip += 1;
                },
                
                Opcode::Multiply => {
                    if self.stack.len() < 2 {
                        return Err(NepalError::RuntimeError("Stack underflow"));
                    }
                    
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    
                    match (a, b) {
                        (Value::Number(a_val), Value::Number(b_val)) => {
                            match a_val.checked_mul(b_val) {
                                Some(r) => self.stack.push(Value::Number(r)),
                                None => return Err(NepalError::RuntimeError("arithmetic overflow")),
                            }
                        },
                        _ => {
                            return Err(NepalError::RuntimeError(
                                "Type error: Cannot multiply these types"
                            ));
                        }
                    }
                    
                    self.ip += 1;
                },
                
                Opcode::Divide => {
                    if self.stack.len() < 2 {
                        return Err(NepalError::RuntimeError("Stack underflow"));
                    }
                    
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    
                    match (a, b) {
                        (Value::Number(a_val), Value::Number(b_val)) => {
                            if b_val == 0 {
                                return Err(NepalError::RuntimeError("Division by zero"));
                            }
                            match a_val.checked_div(b_val) {
                                Some(r) => self.stack.push(Value::Number(r)),
                                None => return Err(NepalError::RuntimeError("arithmetic overflow")),
                            }
                        },
                        _ => {
                            return Err(NepalError::RuntimeError(
                                "Type error: Cannot divide these types"
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
                    if address > self.program.instructions.len() {
                        return Err(NepalError::RuntimeError("invalid jump target"));
                    }
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
                    if address > self.program.instructions.len() {
                        return Err(NepalError::RuntimeError("invalid jump target"));
                    }
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
                    if address > self.program.instructions.len() {
                        return Err(NepalError::RuntimeError("invalid jump target"));
                    }
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
                            // Case-insensitive, matching the tree-walking
                            // interpreter and the documented behaviour (RATO == rato).
                            Value::Boolean(a_str.to_lowercase() == b_str.to_lowercase())
                        },
                        (Value::Boolean(a_val), Value::Boolean(b_val)) => {
                            Value::Boolean(a_val == b_val)
                        },
                        // Mixed Number/String: compare numerically if the string
                        // parses as a number, otherwise unequal.
                        (Value::Number(a_val), Value::String(b_str)) => {
                            match b_str.trim().parse::<i32>() {
                                Ok(b_val) => Value::Boolean(a_val == b_val),
                                Err(_) => Value::Boolean(false),
                            }
                        },
                        (Value::String(a_str), Value::Number(b_val)) => {
                            match a_str.trim().parse::<i32>() {
                                Ok(a_val) => Value::Boolean(a_val == b_val),
                                Err(_) => Value::Boolean(false),
                            }
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
                            // Case-insensitive, mirroring the Equal opcode above.
                            Value::Boolean(a_str.to_lowercase() != b_str.to_lowercase())
                        },
                        (Value::Boolean(a_val), Value::Boolean(b_val)) => {
                            Value::Boolean(a_val != b_val)
                        },
                        // Mixed Number/String: compare numerically if the string
                        // parses as a number, otherwise unequal (=> not-equal true).
                        (Value::Number(a_val), Value::String(b_str)) => {
                            match b_str.trim().parse::<i32>() {
                                Ok(b_val) => Value::Boolean(a_val != b_val),
                                Err(_) => Value::Boolean(true),
                            }
                        },
                        (Value::String(a_str), Value::Number(b_val)) => {
                            match a_str.trim().parse::<i32>() {
                                Ok(a_val) => Value::Boolean(a_val != b_val),
                                Err(_) => Value::Boolean(true),
                            }
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
                    // No call-frame / return-address stack exists in this VM,
                    // so there is nowhere to return to. Halt execution like
                    // reaching the end of the program.
                    // TODO: implement proper call frames if functions are added.
                    self.running = false;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::StrSegment;
    use crate::ast::Statement;
    use crate::ast::Value as AstValue;
    use crate::compiler::Compiler;

    #[test]
    fn string_concat_vm_matches_literal_space() {
        let stmts = vec![
            Statement::Declaration("firstName".into(), AstValue::String("Ram".into())),
            Statement::Declaration("lastName".into(), AstValue::String("Bahadur".into())),
            Statement::StringConcat(
                "fullName".into(),
                vec![
                    StrSegment::Identifier("firstName".into()),
                    StrSegment::Literal(" ".into()),
                    StrSegment::Identifier("lastName".into()),
                ],
            ),
        ];
        let mut c = Compiler::new();
        let program = c.compile(stmts).unwrap();
        let mut vm = VM::new(program);
        vm.run().unwrap();
        let idx = vm
            .program
            .variable_names
            .iter()
            .position(|n| n == "fullName")
            .unwrap();
        assert_eq!(vm.variables[idx], Value::String("Ram Bahadur".into()));
    }

    #[test]
    fn multiplication_chain() {
        let stmts = vec![
            Statement::Declaration("a".into(), AstValue::Number(3)),
            Statement::Declaration("b".into(), AstValue::Number(4)),
            Statement::Multiplication("p".into(), vec!["a".into(), "b".into()]),
        ];
        let mut c = Compiler::new();
        let program = c.compile(stmts).unwrap();
        let mut vm = VM::new(program);
        vm.run().unwrap();
        let i = vm.program.variable_names.iter().position(|n| n == "p").unwrap();
        assert_eq!(vm.variables[i], Value::Number(12));
    }

    fn run_string_comparison(opcode: Opcode, left: &str, right: &str) -> Value {
        let mut program = BytecodeProgram::new();
        let left_idx = program.add_string(left.into());
        let right_idx = program.add_string(right.into());
        program.add_instruction(Opcode::PushString(left_idx), 1);
        program.add_instruction(Opcode::PushString(right_idx), 1);
        program.add_instruction(opcode, 1);
        program.add_instruction(Opcode::Halt, 1);

        let mut vm = VM::new(program);
        vm.run().unwrap();
        vm.peek().unwrap().clone()
    }

    #[test]
    fn string_comparisons_are_case_insensitive() {
        assert_eq!(
            run_string_comparison(Opcode::Equal, "RATO", "rato"),
            Value::Boolean(true)
        );
        assert_eq!(
            run_string_comparison(Opcode::NotEqual, "RATO", "rato"),
            Value::Boolean(false)
        );
        assert_eq!(
            run_string_comparison(Opcode::NotEqual, "rato", "nilo"),
            Value::Boolean(true)
        );
    }
}
