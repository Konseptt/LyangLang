mod token;
mod error;
mod ast;
mod lexer;
mod parser;
mod interpreter;
mod bytecode;
mod vm;
mod compiler;

use clap::{Parser, Subcommand};
use crate::error::NepalError;
use crate::lexer::Lexer;
use crate::parser::Parser as LyangParser;
use crate::compiler::Compiler;
use crate::vm::VM;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Input file to process
    #[arg(value_name = "FILE")]
    input: Option<PathBuf>,

    /// Use VM mode for execution
    #[arg(short, long)]
    vm: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a LyangLang program
    Run {
        /// Input file to run
        file: PathBuf,
        
        /// Use VM mode
        #[arg(short, long)]
        vm: bool,
    },
    
    /// Create a new LyangLang project
    New {
        /// Project name
        name: String,
    },

    /// Check a program for errors without running it
    Check {
        /// Input file to check
        file: PathBuf,
    },
}

fn main() -> Result<(), NepalError> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run { file, vm }) => {
            run_program(&file, vm)
        },
        Some(Commands::New { name }) => {
            create_project(&name)
        },
        Some(Commands::Check { file }) => {
            check_program(&file)
        },
        None => {
            // Legacy mode - handle direct file input
            if let Some(file) = cli.input {
                run_program(&file, cli.vm)
            } else {
                if let Ok(example) = std::fs::canonicalize("example.nbh") {
                    run_program(&example, cli.vm)
                } else {
                    Err(NepalError::RuntimeError("No input file specified"))
                }
            }
        },
    }
}

fn run_program(file: &PathBuf, use_vm: bool) -> Result<(), NepalError> {
    let input = std::fs::read_to_string(file)
        .map_err(|_| NepalError::RuntimeError("Failed to read file"))?;

    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize()?;
    let mut parser = LyangParser::new(tokens);
    
    match parser.parse() {
        Ok(statements) => {
            if use_vm {
                println!("Running with Lyangpiler VM");
                let mut compiler = Compiler::new();
                let program = compiler.compile(statements)?;
                let mut vm = VM::new(program);
                vm.run()?;
                println!("Program execution completed.");
            } else {
                let mut interpreter = interpreter::Interpreter::new();
                for statement in statements {
                    interpreter.execute(statement);
                }
            }
            Ok(())
        }
        Err(e) => Err(e)
    }
}

fn create_project(name: &str) -> Result<(), NepalError> {
    let project_dir = PathBuf::from(name);
    if project_dir.exists() {
        return Err(NepalError::RuntimeError("Project directory already exists"));
    }

    std::fs::create_dir(&project_dir)?;
    
    // Create main.nbh
    let main_file = project_dir.join("main.nbh");
    std::fs::write(main_file, "bol mug \"Namaste, world!\"")?;
    
    // Create README.md
    let readme = project_dir.join("README.md");
    std::fs::write(readme, format!("# {}\n\nA LyangLang project.\n\n## Running\n\n```bash\nlyangpiler main.nbh --vm\n```\n", name))?;

    println!("Created new LyangLang project: {}", name);
    println!("  cd {}", name);
    println!("  lyangpiler main.nbh --vm");
    
    Ok(())
}

fn check_program(file: &PathBuf) -> Result<(), NepalError> {
    let input = std::fs::read_to_string(file)
        .map_err(|_| NepalError::RuntimeError("Failed to read file"))?;

    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize()?;
    let mut parser = LyangParser::new(tokens);
    
    match parser.parse() {
        Ok(_) => {
            println!("Program syntax is valid!");
            Ok(())
        }
        Err(e) => Err(e)
    }
}
