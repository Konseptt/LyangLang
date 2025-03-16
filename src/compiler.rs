use crate::ast::{Statement, Value as AstValue, Condition};
use crate::bytecode::{BytecodeProgram, Opcode};
use crate::error::NepalError;

/// Compiler for LyangLang - converts AST to bytecode
pub struct Compiler {
    program: BytecodeProgram,
    current_line: usize,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            program: BytecodeProgram::new(),
            current_line: 1,
        }
    }

    pub fn compile(&mut self, statements: Vec<Statement>) -> Result<BytecodeProgram, NepalError> {
        for statement in statements {
            self.compile_statement(statement)?;
        }
        
        // Add halt instruction
        self.program.add_instruction(Opcode::Halt, self.current_line);
        
        Ok(self.program.clone())
    }

    fn compile_statement(&mut self, statement: Statement) -> Result<(), NepalError> {
        match statement {
            Statement::Declaration(name, value) => {
                self.compile_declaration(name, value)?;
            },
            Statement::Addition(target, sources) => {
                self.compile_addition(target, sources)?;
            },
            Statement::Subtraction(target, sources) => {
                self.compile_subtraction(target, sources)?;
            },
            Statement::StringConcat(target, parts) => {
                self.compile_string_concat(target, parts)?;
            },
            Statement::Print(name) => {
                self.compile_print_variable(name)?;
            },
            Statement::PrintString(parts) => {
                self.compile_print_string(parts)?;
            },
            Statement::Input(name) => {
                self.compile_input(name)?;
            },
            Statement::If(condition, then_statements, else_statement) => {
                self.compile_conditional(condition, then_statements, else_statement)?;
            },
        }

        Ok(())
    }

    fn compile_declaration(&mut self, name: String, value: AstValue) -> Result<(), NepalError> {
        let var_idx = self.program.add_variable(name);
        
        match value {
            AstValue::Number(num) => {
                self.program.add_instruction(Opcode::PushNumber(num), self.current_line);
            },
            AstValue::String(s) => {
                let str_idx = self.program.add_string(s);
                self.program.add_instruction(Opcode::PushString(str_idx), self.current_line);
            },
        }
        
        self.program.add_instruction(Opcode::StoreVariable(var_idx), self.current_line);
        
        Ok(())
    }

    fn compile_addition(&mut self, target: String, sources: Vec<String>) -> Result<(), NepalError> {
        if sources.is_empty() {
            return Err(NepalError::RuntimeError("Addition requires at least one source variable"));
        }
        
        let target_idx = self.program.add_variable(target);
        
        // Load the first source variable
        let first_var_idx = self.program.add_variable(sources[0].clone());
        self.program.add_instruction(Opcode::LoadVariable(first_var_idx), self.current_line);
        
        // Add the remaining source variables
        for i in 1..sources.len() {
            let src_var_idx = self.program.add_variable(sources[i].clone());
            self.program.add_instruction(Opcode::LoadVariable(src_var_idx), self.current_line);
            self.program.add_instruction(Opcode::Add, self.current_line);
        }
        
        // Store the result in the target variable
        self.program.add_instruction(Opcode::StoreVariable(target_idx), self.current_line);
        
        Ok(())
    }

    fn compile_subtraction(&mut self, target: String, sources: Vec<String>) -> Result<(), NepalError> {
        if sources.is_empty() {
            return Err(NepalError::RuntimeError("Subtraction requires at least one source variable"));
        }
        
        let target_idx = self.program.add_variable(target);
        
        // Load the first source variable
        let first_var_idx = self.program.add_variable(sources[0].clone());
        self.program.add_instruction(Opcode::LoadVariable(first_var_idx), self.current_line);
        
        // Subtract the remaining source variables
        for i in 1..sources.len() {
            let src_var_idx = self.program.add_variable(sources[i].clone());
            self.program.add_instruction(Opcode::LoadVariable(src_var_idx), self.current_line);
            self.program.add_instruction(Opcode::Subtract, self.current_line);
        }
        
        // Store the result in the target variable
        self.program.add_instruction(Opcode::StoreVariable(target_idx), self.current_line);
        
        Ok(())
    }

    fn compile_string_concat(&mut self, target: String, parts: Vec<String>) -> Result<(), NepalError> {
        if parts.is_empty() {
            return Err(NepalError::RuntimeError("String concatenation requires at least one part"));
        }
        
        let target_idx = self.program.add_variable(target);
        
        // Push the first part - could be a string literal or variable
        let first_part = &parts[0];
        if first_part.starts_with('"') && first_part.ends_with('"') {
            // String literal
            let str_value = &first_part[1..first_part.len() - 1];
            let str_idx = self.program.add_string(str_value.to_string());
            self.program.add_instruction(Opcode::PushString(str_idx), self.current_line);
        } else {
            // Variable
            let var_idx = self.program.add_variable(first_part.clone());
            self.program.add_instruction(Opcode::LoadVariable(var_idx), self.current_line);
        }
        
        // Process remaining parts
        for i in 1..parts.len() {
            let part = &parts[i];
            if part.starts_with('"') && part.ends_with('"') {
                // String literal
                let str_value = &part[1..part.len() - 1];
                let str_idx = self.program.add_string(str_value.to_string());
                self.program.add_instruction(Opcode::PushString(str_idx), self.current_line);
            } else {
                // Variable
                let var_idx = self.program.add_variable(part.clone());
                self.program.add_instruction(Opcode::LoadVariable(var_idx), self.current_line);
            }
            
            // Concatenate with the previous result
            self.program.add_instruction(Opcode::Concat, self.current_line);
        }
        
        // Store the result in the target variable
        self.program.add_instruction(Opcode::StoreVariable(target_idx), self.current_line);
        
        Ok(())
    }

    fn compile_print_variable(&mut self, name: String) -> Result<(), NepalError> {
        let var_idx = self.program.add_variable(name);
        self.program.add_instruction(Opcode::LoadVariable(var_idx), self.current_line);
        self.program.add_instruction(Opcode::Print, self.current_line);
        
        Ok(())
    }

    fn compile_print_string(&mut self, parts: Vec<String>) -> Result<(), NepalError> {
        if parts.is_empty() {
            return Ok(());
        }
        
        // Process first part
        let first_part = &parts[0];
        
        if first_part.starts_with('{') && first_part.ends_with('}') {
            // It's a variable interpolation
            let var_name = &first_part[1..first_part.len() - 1];
            let var_idx = self.program.add_variable(var_name.to_string());
            self.program.add_instruction(Opcode::LoadVariable(var_idx), self.current_line);
        } else {
            // It's a string literal
            let str_idx = self.program.add_string(first_part.clone());
            self.program.add_instruction(Opcode::PushString(str_idx), self.current_line);
        }
        
        // Process remaining parts, concatenating them
        for i in 1..parts.len() {
            let part = &parts[i];
            
            if part.starts_with('{') && part.ends_with('}') {
                // It's a variable interpolation
                let var_name = &part[1..part.len() - 1];
                let var_idx = self.program.add_variable(var_name.to_string());
                self.program.add_instruction(Opcode::LoadVariable(var_idx), self.current_line);
            } else {
                // It's a string literal
                let str_idx = self.program.add_string(part.clone());
                self.program.add_instruction(Opcode::PushString(str_idx), self.current_line);
            }
            
            // Concatenate with previous parts
            self.program.add_instruction(Opcode::Concat, self.current_line);
        }
        
        // Print the result
        self.program.add_instruction(Opcode::Print, self.current_line);
        
        Ok(())
    }

    fn compile_input(&mut self, name: String) -> Result<(), NepalError> {
        let var_idx = self.program.add_variable(name);
        
        // Generate bytecode to read input from user
        self.program.add_instruction(Opcode::Input, self.current_line);
        
        // Store input in the variable
        self.program.add_instruction(Opcode::StoreVariable(var_idx), self.current_line);
        
        Ok(())
    }

    fn compile_conditional(
        &mut self,
        condition: Condition, 
        then_statements: Vec<Statement>, 
        else_statement: Option<Box<Statement>>
    ) -> Result<(), NepalError> {
        match condition {
            Condition::Equals(var_name, literal) => {
                // Load variable
                let var_idx = self.program.add_variable(var_name);
                self.program.add_instruction(Opcode::LoadVariable(var_idx), self.current_line);
                
                // Load literal to compare with
                let str_idx = self.program.add_string(literal);
                self.program.add_instruction(Opcode::PushString(str_idx), self.current_line);
                
                // Compare for equality
                self.program.add_instruction(Opcode::Equal, self.current_line);
                
                // Add conditional jump (will update the address later)
                let jump_idx = self.program.instructions.len();
                self.program.add_instruction(Opcode::JumpIfFalse(0), self.current_line);
                
                // Compile the "then" statements
                for stmt in then_statements {
                    self.compile_statement(stmt)?;
                }
                
                // If there's an else branch, add a jump past the else code
                let else_jump_idx = if else_statement.is_some() {
                    let else_jump = self.program.instructions.len();
                    self.program.add_instruction(Opcode::Jump(0), self.current_line);
                    Some(else_jump)
                } else {
                    None
                };
                
                // Update the conditional jump target
                let after_then = self.program.instructions.len();
                if let Some(jump_instr) = self.program.instructions.get_mut(jump_idx) {
                    jump_instr.opcode = Opcode::JumpIfFalse(after_then);
                }
                
                // Compile else branch if it exists
                if let Some(else_stmt) = else_statement {
                    self.compile_statement(*else_stmt)?;
                    
                    // Update the else jump to skip over the else code
                    let after_else = self.program.instructions.len();
                    if let (Some(_else_jump), Some(jump_instr)) = (else_jump_idx, else_jump_idx.and_then(|idx| self.program.instructions.get_mut(idx))) {
                        jump_instr.opcode = Opcode::Jump(after_else);
                    }
                }
            },
            
            Condition::NotEquals(var_name, literal) => {
                // Load variable
                let var_idx = self.program.add_variable(var_name);
                self.program.add_instruction(Opcode::LoadVariable(var_idx), self.current_line);
                
                // Load literal to compare with
                let str_idx = self.program.add_string(literal);
                self.program.add_instruction(Opcode::PushString(str_idx), self.current_line);
                
                // Compare for inequality
                self.program.add_instruction(Opcode::NotEqual, self.current_line);
                
                // Add conditional jump (will update the address later)
                let jump_idx = self.program.instructions.len();
                self.program.add_instruction(Opcode::JumpIfFalse(0), self.current_line);
                
                // Compile the "then" statements
                for stmt in then_statements {
                    self.compile_statement(stmt)?;
                }
                
                // If there's an else branch, add a jump past the else code
                let else_jump_idx = if else_statement.is_some() {
                    let else_jump = self.program.instructions.len();
                    self.program.add_instruction(Opcode::Jump(0), self.current_line);
                    Some(else_jump)
                } else {
                    None
                };
                
                // Update the conditional jump target
                let after_then = self.program.instructions.len();
                if let Some(jump_instr) = self.program.instructions.get_mut(jump_idx) {
                    jump_instr.opcode = Opcode::JumpIfFalse(after_then);
                }
                
                // Compile else branch if it exists
                if let Some(else_stmt) = else_statement {
                    self.compile_statement(*else_stmt)?;
                    
                    // Update the else jump to skip over the else code
                    let after_else = self.program.instructions.len();
                    if let (Some(_else_jump), Some(jump_instr)) = (else_jump_idx, else_jump_idx.and_then(|idx| self.program.instructions.get_mut(idx))) {
                        jump_instr.opcode = Opcode::Jump(after_else);
                    }
                }
            },
        }
        
        Ok(())
    }
    
    // Mark with # to keep it but eliminate the unused warning
    #[allow(dead_code)]
    pub fn set_line(&mut self, line: usize) {
        self.current_line = line;
    }
}