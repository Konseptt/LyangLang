# LyangLang

LyangLang is a programming language designed to make coding accessible to Nepali speakers. It features natural Nepali-like syntax while maintaining powerful programming capabilities. The language is now equipped with a bytecode virtual machine (Lyangpiler) for efficient execution.

![LyangLang](https://img.shields.io/badge/LyangLang-v0.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Running Programs](#running-programs)
- [Language Guide](#language-guide)
- [Sample Programs](#sample-programs)
- [Error Handling](#error-handling)
- [Lyangpiler VM](#lyangpiler-vm)
- [Project Structure](#project-structure)
- [Development](#development)
- [Troubleshooting](#troubleshooting)
- [License](#license)

## Features

- **Natural Nepali Syntax**: Write code using familiar Nepali words and phrases
- **Bytecode Virtual Machine**: Efficient execution with the Lyangpiler VM
- **Basic Programming Constructs**: 
  - Variables and data types
  - Arithmetic operations
  - String manipulation
  - Input/output operations
  - Conditional statements
- **Error Handling**: Comprehensive error messages in English
- **Easy to Learn**: Designed with beginners in mind

## Installation

### Prerequisites
- Rust programming environment
- Cargo package manager

### Building from Source

1. Clone the repository:
```bash
git clone https://github.com/konseptt/LyangLang.git
cd LyangLang
```

2. Build the project:
```bash
cargo build --release
```

The executable will be available at `target/release/lyangpiler.exe` (Windows) or `target/release/lyangpiler` (Unix-like systems).

### Quick Installation

For a quicker installation that sets up your system:

#### Windows
```bash
.\package_release.cmd
```
Then extract the resulting zip file and run `install.cmd` from the extracted folder.

#### Linux/macOS
```bash
chmod +x package_release.sh
./package_release.sh
```
Then extract the resulting tar.gz file and run `install.sh` from the extracted folder.

## Running Programs

LyangLang programs can be run using either the interpreter mode or the Lyangpiler VM for better performance.

### Using the Command Line

#### Windows
```bash
# Run with the interpreter
lyangpiler.cmd your_program.nbh

# Run with the Lyangpiler VM (recommended)
lyangpiler.cmd your_program.nbh --vm
```

#### Linux/macOS
```bash
# Run with the interpreter
./lyangpiler your_program.nbh

# Run with the Lyangpiler VM (recommended)
./lyangpiler your_program.nbh --vm
```

### Command Line Arguments

- First argument: Path to the LyangLang program file (.nbh)
- Second argument (optional): `--vm` to use the Lyangpiler VM

### Examples

```bash
# Windows
lyangpiler.cmd example.nbh --vm

# Linux/macOS
./lyangpiler example.nbh --vm
```

## Language Guide

### 1. Basic Syntax

#### Variables and Assignment
```
oi mug variable = 10          # Number variable
oi mug name = "Ram"           # String variable
```

#### Input/Output
```
bol mug "Namaste!"           # Print output
oi mug bhan userInput        # Get user input
```

#### Arithmetic Operations
```
mug jod a, b lai result     # Addition: result = a + b
mug ghata a, b lai diff     # Subtraction: diff = a - b
```

#### String Concatenation
```
oi mug fullName = firstName + " " + lastName
```

### 2. Control Flow

#### Conditional Statements
```
yedi age babaal "18" bhane
    bol mug "Adult"
sakiyo

yedi score babaal "80" bhane
    bol mug "Ramro Score!"
aile feri score babaal "60" bhane
    bol mug "Thikai Score"
sakiyo
```

## Sample Programs

### Hello World
```
bol mug "Namaste, world!"
```
**Expected Output:**
```
Namaste, world!
```

### Simple Calculator
```
# Simple calculator
bol mug "Pahilo Number:"
oi mug bhan num1

bol mug "Dosro Number:"
oi mug bhan num2

mug jod num1, num2 lai result
bol mug "Sum: " + result

mug ghata num1, num2 lai diff
bol mug "Difference: " + diff
```
**Expected Input/Output:**
```
Pahilo Number:
> 10
Dosro Number:
> 5
Sum: 15
Difference: 5
```

### User Greeting
```
bol mug "Timro naam k ho?"
oi mug bhan userName

bol mug "Namaste, " + userName + "!"
```
**Expected Input/Output:**
```
Timro naam k ho?
> Ram
Namaste, Ram!
```

### Age Verification
```
bol mug "Timro umer kati ho?"
oi mug bhan age

yedi age babaal "18" bhane
    bol mug "Tapai adult hununcha!"
aile feri age babaal "18" hoina bhane
    bol mug "Tapai adult hunuhunna!"
sakiyo
```
**Expected Input/Output:**
```
Timro umer kati ho?
> 20
Tapai adult hununcha!
```
or
```
Timro umer kati ho?
> 16
Tapai adult hunuhunna!
```

### Color Preference
```
bol mug "Timro favourite color k ho?"
oi mug bhan color

yedi color babaal "rato" bhane
    bol mug "Rato rang maya ko rang ho"
aile feri color babaal "nilo" bhane
    bol mug "Nilo rang aakash jastai shanta cha"
aile feri
    bol mug color + " ramro color ho"
sakiyo
```
**Expected Input/Output:**
```
Timro favourite color k ho?
> rato
Rato rang maya ko rang ho
```

## Error Handling

LyangLang provides detailed error messages to help you debug your programs. Here are some common errors and their solutions:

### Syntax Errors

```
Error: Parse error: Expected value
```
**Solution:** Check that all variable declarations have proper values.

```
Error: Parse error: Expected identifier
```
**Solution:** Ensure that variable names are properly formatted.

### Runtime Errors

```
Error: Runtime error: Division by zero
```
**Solution:** Check your arithmetic operations to avoid dividing by zero.

```
Error: Runtime error: Variable 'variableName' not found
```
**Solution:** Ensure the variable is declared before use.

### Type Errors

```
Error: Runtime error: Cannot add these types
```
**Solution:** Make sure you're performing operations on compatible types.

### File Errors

```
Error: Failed to read file
```
**Solution:** Check that the specified file exists and is accessible.

## Lyangpiler VM

The Lyangpiler VM is a bytecode virtual machine that executes LyangLang programs with improved performance.

### How It Works

1. LyangLang code is parsed into an Abstract Syntax Tree (AST)
2. The AST is compiled into bytecode
3. The bytecode is executed by the Lyangpiler VM

### Benefits

- **Performance**: Faster execution compared to the interpreter
- **Efficiency**: Reduced memory consumption
- **Optimization**: Bytecode can be optimized for better performance

### Debugging

You can see when the VM is active with this message:
```
Running with Lyangpiler VM
```

When program execution completes successfully, you'll see:
```
Program execution completed.
```

## Project Structure

```
LyangLang/
├── src/
│   ├── main.rs          # Entry point
│   ├── lexer.rs         # Tokenization
│   ├── parser.rs        # Syntax parsing
│   ├── ast.rs           # Abstract Syntax Tree definitions
│   ├── token.rs         # Token definitions
│   ├── interpreter.rs   # Direct code execution
│   ├── bytecode.rs      # Bytecode definitions
│   ├── compiler.rs      # AST to bytecode compiler
│   ├── vm.rs            # Virtual machine implementation
│   └── error.rs         # Error handling
├── examples/            # Example programs
└── README.md
```

## Development

### Building from Source

1. Clone the repository
2. Install dependencies:
```bash
cargo check
```
3. Build the project:
```bash
cargo build
```
4. Run tests:
```bash
cargo test
```

### Contributing

We welcome contributions! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

#### Contribution Guidelines

- Follow Rust coding standards
- Add tests for new features
- Update documentation as needed
- Keep commit messages clear and descriptive

## Troubleshooting

### Common Issues and Solutions:

1. **Compilation Errors**
   - **Issue**: `error: linker 'link.exe' not found`
   - **Solution**: Install Visual Studio Build Tools or run `rustup toolchain install stable-gnu` to use GCC instead

2. **Runtime Errors**
   - **Issue**: `Error: RuntimeError: Failed to read file`
   - **Solution**: Check file path and ensure the file exists

3. **VM Issues**
   - **Issue**: `Error: Stack underflow`
   - **Solution**: This is likely due to a compilation error; check your code for type mismatches

4. **Path Issues**
   - **Issue**: Command not found errors
   - **Solution**: Use `.\lyangpiler.cmd` instead of just `lyangpiler.cmd` in PowerShell

### Advanced Debugging

To get more information during execution:

1. Build with debug symbols:
```bash
cargo build
```

2. Run the program with debug logs:
```bash
$env:RUST_LOG="debug" # PowerShell
./target/debug/lyangpiler.exe your_program.nbh
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

