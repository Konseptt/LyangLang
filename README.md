# LyangLang

A programming language featuring natural Nepali syntax, designed to help Nepali speakers learn programming concepts in a familiar language. LyangLang bridges the gap between natural Nepali language and programming logic, creating an accessible entry point for native speakers.

![LyangLang](https://img.shields.io/badge/LyangLang-v0.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Build Status](https://img.shields.io/github/workflow/status/konseptt/LyangLang/Build)

## Table of Contents

- [Features](#features)
- [Installation](#installation)
  - [Prerequisites](#prerequisites)
  - [Windows](#windows)
  - [Linux/macOS](#linuxmacos)
  - [Building from Source](#building-from-source)
  - [Quick Installation](#quick-installation)
- [Running Programs](#running-programs)
  - [Using the Command Line](#using-the-command-line)
  - [Command Line Arguments](#command-line-arguments)
  - [Examples](#examples)
  - [Command Line Interface](#command-line-interface)
  - [Command Line Options](#command-line-options)
- [Language Guide](#language-guide)
  - [Basic Syntax](#1-basic-syntax)
  - [Control Flow](#2-control-flow)
  - [Operations](#3-operations)
  - [Data Types](#4-data-types)
  - [Functions and Procedures](#5-functions-and-procedures)
- [Loop Examples](#loop-examples)
  - [Counted Loop](#counted-loop-for-loop)
  - [While Loop](#while-loop)
- [Sample Programs](#sample-programs)
  - [Hello World](#hello-world)
  - [Simple Calculator](#simple-calculator)
  - [User Greeting](#user-greeting)
  - [Age Verification](#age-verification)
  - [Color Preference](#color-preference)
- [Error Handling](#error-handling)
  - [Error Types](#error-types)
  - [Error Format](#error-format)
  - [Common Error Messages](#common-error-messages)
- [Lyangpiler VM](#lyangpiler-vm)
  - [Architecture](#architecture)
  - [Bytecode Instructions](#bytecode-instructions)
  - [Memory Management](#memory-management)
  - [Execution Model](#execution-model)
- [Project Structure](#project-structure)
- [Development](#development)
  - [Building from Source](#building-from-source-1)
  - [Running Tests](#running-tests)
  - [Contributing](#contributing)
- [Troubleshooting](#troubleshooting)
  - [Common Issues](#common-issues)
  - [Getting Help](#getting-help)
- [License](#license)

## Features

- **Natural Nepali Syntax**: Write code using familiar Nepali words and phrases for a more intuitive programming experience. Commands like `bol mug` (print) and `oi mug bhan` (input) provide a close connection to everyday language.

- **Bytecode Virtual Machine**: The Lyangpiler VM offers efficient execution with precompiled bytecode for optimal performance across different platforms. The VM is stack-based with a clean and simple architecture.

- **Comprehensive Programming Constructs**: 
  - Variables and data types (numbers, text, boolean)
  - Arithmetic operations (`jod` for add, `ghata` for subtract, `guna` for multiply, `bhag` for divide)
  - String manipulation and concatenation using `+` operator
  - Input/output operations with console through `bol mug` and `oi mug bhan`
  - Conditional statements with `yadi`/`natra`/`sakiyo` (if/else/end)
  - Loops and iteration using `ghumu` (for loop) and `jabsamma` (while loop)
  - Functions and subroutines with `kaam` declarations

- **Error Handling**: Comprehensive error messages in English with clear line indicators and detailed explanations for debugging.

- **Cross-Platform Support**: Runs on Windows, Linux, and macOS with consistent behavior and native installation packages.

- **Beginner-Friendly Design**: Specifically crafted for educational purposes, making programming accessible to Nepali speakers with little to no programming experience.

- **Interpreter and Compiler**: Use either the direct interpreter or the VM compilation mode to run your programs with a simple command-line interface.

## Installation

### Prerequisites

- Rust toolchain (for building from source)
- Cargo package manager
- Basic command line familiarity

### Windows

1. Using the installer script:
```powershell
.\install.ps1
```

2. Manual installation:
```powershell
cargo build --release
copy target\release\lyangpiler.exe %PATH%
```

3. Using pre-built binary:
   - Download the latest `lyangpiler-windows-amd64.zip` from the releases page
   - Extract the zip file
   - Run `install.cmd` from the extracted folder
   - Alternatively, add the extracted folder to your PATH manually

### Linux/macOS

1. Using the installer script:
```bash
./install.sh
```

2. Manual installation:
```bash
cargo build --release
sudo cp target/release/lyangpiler /usr/local/bin/
```

3. Using pre-built binary:
   - Download the latest `lyangpiler-linux-amd64.tar.gz` (Linux) or `lyangpiler-macos-amd64.tar.gz` (macOS) from the releases page
   - Extract the archive: `tar -xzf lyangpiler-*-amd64.tar.gz`
   - Run the installation script: `cd lyangpiler && ./install.sh`

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

3. Make the file executable (Unix-like systems only):
```bash
chmod +x target/release/lyangpiler
```

### Quick Installation

For a quicker installation that sets up your system:

#### Windows
```bash
.\package_release.cmd
```
Then extract the resulting zip file and run `install.cmd` from the extracted folder. This will add the Lyangpiler to your PATH automatically.

#### Linux/macOS
```bash
chmod +x package_release.sh
./package_release.sh
```
Then extract the resulting tar.gz file and run `install.sh` from the extracted folder. This will configure your shell profile to include the Lyangpiler in your PATH.

## Running Programs

1. Create a new file with `.nbh` extension
2. Write your LyangLang code
3. Run using the Lyangpiler:
```bash
lyangpiler your_program.nbh
```

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

### Command Line Interface

The Lyangpiler provides a modern CLI with several subcommands for different operations:

```bash
# Run a program
lyangpiler run your_program.nbh

# Check a program for errors
lyangpiler check your_program.nbh

# Create a new project
lyangpiler new project_name

# Running with VM (recommended for performance)
lyangpiler run your_program.nbh --vm
```

### Command Line Options

- `run`: Execute a LyangLang program
  - Example: `lyangpiler run program.nbh`
  - Options: `--vm` to use the virtual machine execution mode
- `check`: Validate syntax without executing
  - Example: `lyangpiler check program.nbh`
- `new`: Create a new LyangLang project with template files
  - Example: `lyangpiler new myproject`
- `version`: Display version information
  - Example: `lyangpiler --version`
- `help`: Show help message and available commands
  - Example: `lyangpiler --help` or `lyangpiler help`

## Language Guide

### 1. Basic Syntax

#### Variables and Assignment
```
# Number variable
mug rakhnu 42 lai number

# String variable
mug rakhnu "Hello" lai text

# Boolean variable
mug rakhnu sahi lai bool_var

# Variable assignment with calculation
number = 10 jod 5    # number = 15
```

#### Input/Output
```
# Output to console
bol mug "Namaste!"

# Output with variable interpolation
bol mug "Your score is: " jod score

# Input from user
oi mug bhan user_input

# Input with prompt
bol mug "Enter your name:"
oi mug bhan user_name
```

#### Operators
- Arithmetic: `jod` (add), `ghata` (subtract), `guna` (multiply), `bhag` (divide)
- Comparison: `barabar` (equals), `thulo` (greater than), `sano` (less than)
- Logical: `ra` (and), `wa` (or), `hoina` (not)

#### Functions
```
# Function definition
kaam namaste_bhan(naam) {
    bol mug "Namaste " jod naam jod "!"
}

# Function call
namaste_bhan("Sathi")
```

### 2. Control Flow

#### Conditional Statements
```
# Simple if-else
yadi age >= 18 bhane
    bol mug "You can vote!"
natra
    bol mug "Too young to vote"
sakiyo

# Multiple conditions
yadi score >= 90 bhane
    bol mug "Grade A"
aile feri score >= 80 bhane
    bol mug "Grade B"
aile feri score >= 70 bhane
    bol mug "Grade C"
natra
    bol mug "Need improvement"
sakiyo
```

#### Loops
```
# Basic for loop (repeat 5 times)
ghumu 5 choti
    bol mug "Iteration"
sakiyo

# While loop
jabsamma number <= 10 cha
    bol mug number
    number = number jod 1
sakiyo

# Loop with break
jabsamma sahi cha
    oi mug bhan input
    yadi input barabar "exit" bhane
        rokana
    sakiyo
    bol mug "You entered: " jod input
sakiyo

# Loop with continue
ghumu 10 choti
    yadi count % 2 barabar 0 bhane
        arko hernu  # Skip even numbers
    sakiyo
    bol mug count  # Print odd numbers only
sakiyo
```

### Control Flow Examples

#### Loops
```
# Count-controlled loop (repeat 5 times)
ghumu 5 choti
    bol mug "Iteration number" jod count
sakiyo

# Condition-controlled loop (while loop)
jabsamma number sano 10 cha
    bol mug "Current number:" jod number
    number = number jod 1
sakiyo
```

#### Nested Control Flow
You can nest both loops and conditionals:
```
ghumu 3 choti
    yedi count babaal 2 bhane
        bol mug "Second iteration"
    natra
        bol mug "Other iteration"
    sakiyo
sakiyo
```

### 3. Operations

#### Arithmetic
- `jod`: Addition
  ```
  result = 5 jod 3  # result = 8
  ```
- `ghata`: Subtraction
  ```
  result = 10 ghata 4  # result = 6
  ```
- `guna`: Multiplication
  ```
  result = 6 guna 3  # result = 18
  ```
- `bhag`: Division
  ```
  result = 15 bhag 3  # result = 5
  ```

#### Comparison
- `barabar`: Equal to
  ```
  yadi a barabar b bhane
  ```
- `thulo`: Greater than
  ```
  yadi age thulo 18 bhane
  ```
- `sano`: Less than
  ```
  yadi score sano 60 bhane
  ```

#### Logical Operations
- `ra`: Logical AND
  ```
  yadi (age thulo 18) ra (score thulo 70) bhane
  ```
- `wa`: Logical OR
  ```
  yadi (member barabar sahi) wa (vip barabar sahi) bhane
  ```
- `hoina`: Logical NOT
  ```
  yadi (finished hoina) bhane
  ```

### 4. Data Types

LyangLang supports three primary data types:

#### Numbers
```
age = 25
temperature = -5
result = 10 jod 15
```

#### Strings
```
name = "Ram Bahadur"
greeting = "Namaste"
message = greeting jod ", " jod name jod "!"
```

#### Booleans
```
isValid = sahi      # true
isComplete = galat  # false
```

### 5. Functions and Procedures

Define reusable blocks of code:

```
kaam add_numbers(a, b) {
    result = a jod b
    bol mug "Sum: " jod result
    result      # Return value
}

# Call the function
sum = add_numbers(5, 7)
```

## Loop Examples

### Counted Loop (For Loop)
```
# Print numbers 1 to 5
ghumu 5 choti
    bol mug k
sakiyo
```

### While Loop
```
k = 1
jabsamma k <= 5 cha
    bol mug k
    k = k jod 1
sakiyo
```

Both loops support break ("rokana") and continue ("arko hernu") statements.

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

mug rakhnu num1 jod num2 lai result
bol mug "Sum: " jod result

mug rakhnu num1 ghata num2 lai diff
bol mug "Difference: " jod diff

mug rakhnu num1 guna num2 lai product
bol mug "Product: " jod product

mug rakhnu num1 bhag num2 lai quotient
bol mug "Quotient: " jod quotient
```
**Expected Input/Output:**
```
Pahilo Number:
> 10
Dosro Number:
> 5
Sum: 15
Difference: 5
Product: 50
Quotient: 2
```

### User Greeting
```
bol mug "Timro naam k ho?"
oi mug bhan userName

bol mug "Namaste, " jod userName jod "!"
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

yadi age babaal "18" bhane
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
    bol mug color jod " ramro color ho"
sakiyo
```
**Expected Input/Output:**
```
Timro favourite color k ho?
> rato
Rato rang maya ko rang ho
```

## Error Handling

### Error Types
- **Syntax Errors**: Malformed code or invalid token sequences
- **Runtime Errors**: Issues that occur during program execution
- **Type Errors**: Invalid operations between incompatible data types
- **Name Errors**: Using undefined variables or functions

### Error Format
```
Error at line X: [Error Type]
Details: [Error Description]
Code: [Problematic code snippet]
     ^---- Error location indicator
```

### Common Error Messages
- `अपरिभाषित चर` (Undefined variable): Occurs when you try to use a variable that hasn't been declared.
- `अमान्य संचालन` (Invalid operation): Happens when you perform an operation that's not permitted, like dividing by zero.
- `प्रकार बेमेल` (Type mismatch): When you try to perform operations on incompatible types, like adding a number to a boolean.
- `वाक्य रचना त्रुटि` (Syntax error): This occurs when your code doesn't follow the LyangLang grammar rules.

## Lyangpiler VM

### Architecture
- **Stack-based virtual machine**: Uses a stack to store and manipulate values during execution
- **Register-free design for simplicity**: No need to manage registers, making the VM easier to understand
- **Bytecode instruction set**: Optimized for Nepali language constructs and efficient execution

### Bytecode Instructions
- `PUSH`: Push value onto stack (`PushNumber`, `PushString`, `PushBoolean`)
- `POP`: Remove top value from stack
- `LOAD/STORE`: Load and store variables (`LoadVariable`, `StoreVariable`)
- `ADD/SUB/MUL/DIV`: Arithmetic operations
- `CONCAT`: String concatenation
- `PRINT`: Output value to console
- `INPUT`: Read user input from console
- `JMP`: Conditional and unconditional jumps (`Jump`, `JumpIfTrue`, `JumpIfFalse`)
- `CMP`: Compare values (`Equal`, `LessThan`, `GreaterThan`)
- `HALT`: Stop program execution

### Memory Management
- **Stack-based memory allocation**: Values are pushed and popped from the stack as needed
- **Variable storage**: Separate from the stack for quick access
- **String pool**: Efficiently stores string literals
- **Automatic cleanup**: Resources are automatically reclaimed when the VM terminates

### Execution Model
1. Source code is parsed into an Abstract Syntax Tree (AST)
2. AST is compiled into bytecode instructions
3. VM initializes the stack and variable storage
4. VM executes instructions sequentially, manipulating the stack
5. Control flow instructions alter execution path as needed
6. Program terminates when the HALT instruction is reached

## Project Structure

```
src/
  ├── lexer.rs     # Tokenization of source code
  ├── parser.rs    # Parsing tokens into AST
  ├── ast.rs       # Abstract Syntax Tree definitions
  ├── token.rs     # Token definitions and types
  ├── bytecode.rs  # Bytecode instruction definitions
  ├── compiler.rs  # Compiles AST to bytecode
  ├── vm.rs        # Virtual Machine implementation
  ├── error.rs     # Error handling definitions
  ├── interpreter.rs # Direct interpreter (alternative to VM)
  └── main.rs      # Entry point and CLI handling
examples/
  ├── hello.nbh    # Example programs
  ├── calculator.nbh
  └── loops.nbh
tests/
  └── integration_tests.rs  # Integration tests
```

## Development

### Building from Source
```bash
# Clone the repository
git clone https://github.com/konseptt/LyangLang.git
cd LyangLang

# Build the project
cargo build --release

# Run the executable
./target/release/lyangpiler examples/hello.nbh --vm
```

### Running Tests
```bash
# Run all tests
cargo test

# Run specific tests
cargo test lexer
cargo test parser

# Run with verbose output
cargo test -- --nocapture
```

### Contributing
1. Fork the repository on GitHub
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and add tests when possible
4. Ensure all tests pass: `cargo test`
5. Commit your changes: `git commit -m 'Add new feature'`
6. Push to the branch: `git push origin feature-name`
7. Submit a pull request through GitHub

## Troubleshooting

### Common Issues
1. **Installation failures**
   - Verify Rust toolchain installation: `rustc --version`
   - Check system PATH configuration: Ensure the installation directory is in your PATH
   - For permissions issues on Linux/macOS, try using `sudo` for installation
   
2. **Compilation errors**
   - Update Rust to latest stable version: `rustup update stable`
   - Clean and rebuild project: `cargo clean && cargo build`
   - Check for missing dependencies: `cargo check`

3. **Runtime errors**
   - Check syntax matches LyangLang specifications
   - Verify variable declarations and types
   - Look for missing `sakiyo` statements to close blocks
   - Ensure string literals are properly quoted
   - Check for variable scope issues

4. **Command not found errors**
   - Make sure the installation was successful
   - Verify the PATH configuration in your shell
   - Try using the full path to the executable

### Getting Help
- Open an issue on the [project repository](https://github.com/konseptt/LyangLang)
- Check existing documentation in the [wiki](https://github.com/konseptt/LyangLang/wiki)
- Review example programs in the `examples/` directory
- Join the community discussions in the [forums](https://github.com/konseptt/LyangLang/discussions)

## License

MIT License - See LICENSE file for details.

Copyright (c) 2023 LyangLang Team

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

