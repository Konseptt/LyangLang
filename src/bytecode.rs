/// Bytecode module for LyangLang virtual machine (Lyangpiler)
/// Defines bytecode instructions that the VM will execute

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)] // Some opcodes may not be used yet but will be in future extensions
pub enum Opcode {
    // Stack operations
    PushNumber(i32),
    PushString(usize),    // Index into string constant pool
    PushVariable(usize),  // Index into variables table
    Pop,
    
    // Variable operations
    StoreVariable(usize), // Store value to variable (index in variable table)
    LoadVariable(usize),  // Load value from variable (index in variable table)
    
    // Arithmetic operations
    Add,
    Subtract,
    
    // String operations
    Concat,
    
    // I/O operations
    Print,
    Input,
    
    // Control flow
    JumpIfTrue(usize),    // Jump to instruction if top of stack is true
    JumpIfFalse(usize),   // Jump to instruction if top of stack is false
    Jump(usize),          // Unconditional jump to instruction
    
    // Comparison
    Equal,
    NotEqual,
    
    // Program flow
    Return,
    Halt,
}

/// A single instruction in the bytecode program
#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    #[allow(dead_code)] // Line number is used for debugging but not directly read
    pub line_number: usize,
}

impl Instruction {
    pub fn new(opcode: Opcode, line_number: usize) -> Self {
        Self { opcode, line_number }
    }
}

/// Bytecode program - a sequence of instructions
#[derive(Debug, Clone)]
pub struct BytecodeProgram {
    pub instructions: Vec<Instruction>,
    pub string_pool: Vec<String>,
    pub variable_names: Vec<String>,
}

impl BytecodeProgram {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            string_pool: Vec::new(),
            variable_names: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, opcode: Opcode, line_number: usize) {
        self.instructions.push(Instruction::new(opcode, line_number));
    }
    
    pub fn add_string(&mut self, string: String) -> usize {
        // Check if string already exists in pool
        for (i, s) in self.string_pool.iter().enumerate() {
            if s == &string {
                return i;
            }
        }
        
        // Add new string to pool
        let index = self.string_pool.len();
        self.string_pool.push(string);
        index
    }
    
    pub fn add_variable(&mut self, name: String) -> usize {
        // Check if variable already exists
        for (i, var_name) in self.variable_names.iter().enumerate() {
            if var_name == &name {
                return i;
            }
        }
        
        // Add new variable
        let index = self.variable_names.len();
        self.variable_names.push(name);
        index
    }
}