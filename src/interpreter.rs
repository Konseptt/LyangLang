use std::collections::HashMap;
use crate::ast::{Statement, Condition, Value, StrSegment};
use crate::error::NepalError;

pub struct Interpreter {
    variables: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    /// Print a runtime error to stderr. `execute` has no Result channel, so we
    /// surface failures here instead of silently dropping them.
    fn report_runtime_error(err: &NepalError) {
        eprintln!("{}", err);
    }

    /// Resolve a source variable to a number. On failure prints a clear runtime
    /// error (NameError for undefined, TypeError for non-number) naming the
    /// offending variable, then returns None so the caller can bail out.
    fn resolve_number(&self, name: &str) -> Option<i32> {
        match self.variables.get(name) {
            Some(Value::Number(n)) => Some(*n),
            Some(Value::String(_)) => {
                Self::report_runtime_error(&NepalError::TypeError(
                    "expected a number but found a string",
                ));
                eprintln!("  variable: {}", name);
                None
            }
            None => {
                Self::report_runtime_error(&NepalError::NameError("undefined variable"));
                eprintln!("  variable: {}", name);
                None
            }
        }
    }

    /// Execute one statement. Returns `true` to continue, `false` to halt the
    /// program (a runtime error was reported to stderr). The caller stops
    /// running further statements when this returns `false`.
    pub fn execute(&mut self, statement: Statement) -> bool {
        match statement {
            Statement::Declaration(name, value) => {
                self.variables.insert(name, value);
            }
            Statement::Addition(target, sources) => {
                let mut sum: i32 = 0;
                for name in &sources {
                    match self.resolve_number(name) {
                        Some(n) => match sum.checked_add(n) {
                            Some(v) => sum = v,
                            None => {
                                Self::report_runtime_error(&NepalError::RuntimeError(
                                    "arithmetic overflow",
                                ));
                                return false;
                            }
                        },
                        None => return false,
                    }
                }
                self.variables.insert(target, Value::Number(sum));
            }
            Statement::Subtraction(target, sources) => {
                let mut iter = sources.iter();
                let first = match iter.next() {
                    Some(name) => match self.resolve_number(name) {
                        Some(n) => n,
                        None => return false,
                    },
                    None => return true,
                };
                let mut diff = first;
                for name in iter {
                    match self.resolve_number(name) {
                        Some(n) => match diff.checked_sub(n) {
                            Some(v) => diff = v,
                            None => {
                                Self::report_runtime_error(&NepalError::RuntimeError(
                                    "arithmetic overflow",
                                ));
                                return false;
                            }
                        },
                        None => return false,
                    }
                }
                self.variables.insert(target, Value::Number(diff));
            }
            Statement::Multiplication(target, sources) => {
                if sources.is_empty() {
                    return true;
                }
                let mut prod: i32 = 1;
                for name in &sources {
                    match self.resolve_number(name) {
                        Some(n) => match prod.checked_mul(n) {
                            Some(v) => prod = v,
                            None => {
                                Self::report_runtime_error(&NepalError::RuntimeError(
                                    "arithmetic overflow",
                                ));
                                return false;
                            }
                        },
                        None => return false,
                    }
                }
                self.variables.insert(target, Value::Number(prod));
            }
            Statement::Division(target, sources) => {
                let mut iter = sources.iter();
                let first = match iter.next() {
                    Some(name) => match self.resolve_number(name) {
                        Some(n) => n,
                        None => return false,
                    },
                    None => return true,
                };
                let mut acc = first;
                for name in iter {
                    let n = match self.resolve_number(name) {
                        Some(n) => n,
                        None => return false,
                    };
                    if n == 0 {
                        Self::report_runtime_error(&NepalError::RuntimeError(
                            "Division by zero",
                        ));
                        return false;
                    }
                    match acc.checked_div(n) {
                        Some(v) => acc = v,
                        None => {
                            Self::report_runtime_error(&NepalError::RuntimeError(
                                "arithmetic overflow",
                            ));
                            return false;
                        }
                    }
                }
                self.variables.insert(target, Value::Number(acc));
            }
            Statement::Print(name) => {
                if let Some(value) = self.variables.get(&name) {
                    match value {
                        Value::Number(n) => println!("{}", n),
                        Value::String(s) => println!("{}", s),
                    }
                } else {
                    Self::report_runtime_error(&NepalError::NameError("undefined variable"));
                    eprintln!("    variable: {}", name);
                    return false;
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
                        } else {
                            Self::report_runtime_error(&NepalError::NameError("undefined variable"));
                            eprintln!("    variable: {}", var_name);
                            return false;
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
                if let Err(e) = io::stdout().flush() {
                    Self::report_runtime_error(&NepalError::IoError(e));
                    return false;
                }
                let mut input = String::new();
                if let Err(e) = io::stdin().read_line(&mut input) {
                    Self::report_runtime_error(&NepalError::IoError(e));
                    return false;
                }
                self.variables.insert(name, Value::String(input.trim().to_string()));
            }
            Statement::If(condition, statements, else_branch) => {
                let execute = match condition {
                    Condition::Equals(var1, string_literal) => {
                        // Mirror the VM (vm.rs Equal/NotEqual): a String variable is
                        // compared case-insensitively; a Number variable is compared
                        // numerically against the literal parsed as i32.
                        match self.variables.get(&var1) {
                            Some(Value::String(input)) => {
                                input.to_lowercase() == string_literal.to_lowercase()
                            }
                            Some(Value::Number(input)) => string_literal
                                .trim()
                                .parse::<i32>()
                                .is_ok_and(|lit| *input == lit),
                            _ => false,
                        }
                    }
                    Condition::NotEquals(var1, string_literal) => {
                        // Negation of Equals, including the Number arm above.
                        match self.variables.get(&var1) {
                            Some(Value::String(input)) => {
                                input.to_lowercase() != string_literal.to_lowercase()
                            }
                            // A non-numeric literal can't equal a Number var, so
                            // NotEquals is true on parse failure.
                            Some(Value::Number(input)) => string_literal
                                .trim()
                                .parse::<i32>()
                                .map_or(true, |lit| *input != lit),
                            _ => true,
                        }
                    }
                };

                if execute {
                    for stmt in statements {
                        if !self.execute(stmt) {
                            return false;
                        }
                    }
                } else if let Some(else_stmt) = else_branch {
                    return self.execute(*else_stmt);
                }
            }
            Statement::StringConcat(target, parts) => {
                let mut result = String::new();
                for seg in &parts {
                    match seg {
                        StrSegment::Literal(s) => result.push_str(s),
                        StrSegment::Identifier(name) => {
                            if let Some(value) = self.variables.get(name) {
                                match value {
                                    Value::String(s) => result.push_str(s),
                                    Value::Number(n) => result.push_str(&n.to_string()),
                                }
                            } else {
                                Self::report_runtime_error(&NepalError::NameError(
                                    "undefined variable",
                                ));
                                eprintln!("    variable: {}", name);
                                return false;
                            }
                        }
                    }
                }
                self.variables.insert(target, Value::String(result));
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equals_number_var_matches_numeric_string_literal() {
        // Regression: a Number variable compared against a numeric-looking
        // string literal must match (mirrors the VM). Before the fix this
        // branch was always false in the interpreter.
        let mut interp = Interpreter::new();
        interp.variables.insert("x".to_string(), Value::Number(5));

        let matched = interp.execute(Statement::If(
            Condition::Equals("x".to_string(), "5".to_string()),
            vec![Statement::PrintString(vec!["hit".to_string()])],
            None,
        ));
        assert!(matched);

        let not_matched = interp.execute(Statement::If(
            Condition::Equals("x".to_string(), "6".to_string()),
            vec![Statement::PrintString(vec!["miss".to_string()])],
            None,
        ));
        assert!(not_matched);
    }

    #[test]
    fn not_equals_number_var_mirrors_equals() {
        let mut interp = Interpreter::new();
        interp.variables.insert("x".to_string(), Value::Number(5));

        let neq_mismatch = interp.execute(Statement::If(
            Condition::NotEquals("x".to_string(), "6".to_string()),
            vec![Statement::PrintString(vec!["neq-hit".to_string()])],
            None,
        ));
        assert!(neq_mismatch);

        let neq_match = interp.execute(Statement::If(
            Condition::NotEquals("x".to_string(), "5".to_string()),
            vec![Statement::PrintString(vec!["should-not-run".to_string()])],
            None,
        ));
        assert!(neq_match);
    }
}
