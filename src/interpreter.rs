use std::collections::HashMap;
use crate::ast::{Statement, Condition, Value};

pub struct Interpreter {
    variables: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, statement: Statement) {
        match statement {
            Statement::Declaration(name, value) => {
                self.variables.insert(name, value);
            }
            Statement::Addition(target, sources) => {
                let sum: i32 = sources
                    .iter()
                    .filter_map(|name| {
                        if let Some(Value::Number(n)) = self.variables.get(name) {
                            Some(*n)
                        } else {
                            None
                        }
                    })
                    .sum();
                self.variables.insert(target, Value::Number(sum));
            }
            Statement::Subtraction(target, sources) => {
                if let Some(&Value::Number(first)) = sources.first().and_then(|name| self.variables.get(name)) {
                    let diff = sources.iter().skip(1).fold(first, |acc, name| {
                        if let Some(Value::Number(n)) = self.variables.get(name) {
                            acc - n
                        } else {
                            acc
                        }
                    });
                    self.variables.insert(target, Value::Number(diff));
                }
            }
            Statement::Print(name) => {
                if let Some(value) = self.variables.get(&name) {
                    match value {
                        Value::Number(n) => println!("{}", n),
                        Value::String(s) => println!("{}", s),
                    }
                }
            }
            Statement::PrintString(parts) => {
                let mut output = String::new();
                for part in parts {
                    if part.starts_with('{') && part.ends_with('}') {
                        let var_name = &part[1..part.len()-1];
                        if let Some(value) = self.variables.get(var_name) {
                            match value {
                                Value::Number(n) => output.push_str(&n.to_string()),
                                Value::String(s) => output.push_str(s),
                            }
                        }
                    } else {
                        output.push_str(&part);
                    }
                }
                println!("{}", output);
            }
            Statement::Input(name) => {
                use std::io::{self, Write};
                print!("> ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                self.variables.insert(name, Value::String(input.trim().to_string()));
            }
            Statement::If(condition, statements, else_branch) => {
                let execute = match condition {
                    Condition::Equals(var1, string_literal) => {
                        if let Some(Value::String(input)) = self.variables.get(&var1) {
                            input == &string_literal
                        } else {
                            false
                        }
                    }
                    Condition::NotEquals(var1, string_literal) => {
                        if let Some(Value::String(input)) = self.variables.get(&var1) {
                            input != &string_literal
                        } else {
                            false
                        }
                    }
                };

                if execute {
                    for stmt in statements {
                        self.execute(stmt);
                    }
                } else if let Some(else_stmt) = else_branch {
                    self.execute(*else_stmt);
                }
            }
            Statement::StringConcat(target, parts) => {
                let result = parts.iter()
                    .map(|part| {
                        if let Some(value) = self.variables.get(part) {
                            match value {
                                Value::String(s) => s.clone(),
                                Value::Number(n) => n.to_string(),
                            }
                        } else {
                            part.clone()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("");
                self.variables.insert(target, Value::String(result));
            }
        }
    }
}
