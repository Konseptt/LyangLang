# MUJI LANG

MUJI LANG is a programming language designed to make coding accessible to Nepali speakers. It features natural Nepali-like syntax while maintaining powerful programming capabilities.

## Features

- **Natural Nepali Syntax**: Write code using familiar Nepali words and phrases
- **Basic Programming Constructs**: 
  - Variables and data types
  - Arithmetic operations
  - String manipulation
  - Input/output operations
  - Conditional statements
- **Easy to Learn**: Designed with beginners in mind

## Installation

### Prerequisites
- Rust programming environment
- Cargo package manager

### Steps

1. Clone the repository:
```bash
git clone https://github.com/konseptt/MUJI-LANG.git
cd MUJI-LANG
```

2. Build the project:
```bash
cargo build --release
```

3. Run the interpreter:
```bash
cargo run
```

## Running the Program

You can run the program using the bash file `./mujchal`. 

### Example Command
```bash
./mujchal example.nbh
```

## Language Guide

### 1. Basic Syntax

#### Variables and Assignment
```nbh
oi mug variable = 10          // Number variable
oi mug name = "Ram"           // String variable
oi mug flag = true           // Boolean variable
```

#### Input/Output
```nbh
bol mug "Namaste!"           // Print output
oi mug bhan userInput        // Get user input
```

#### Arithmetic Operations
```nbh
mug jod a, b lai result     // Addition
mug ghata a, b lai diff     // Subtraction
```

### 2. Control Flow

#### Conditional Statements
```nbh
yedi age babaal "18" bhane
    bol mug "Adult"
sakiyo

yedi score babaal "80" bhane
    bol mug "Ramro Score!"
aile feri score babaal "60" bhane
    bol mug "Thikai Score"
sakiyo
```

### 3. Complete Program Example
```nbh
// Simple calculator
bol mug "Pahilo Number:"
oi mug bhan num1
bol mug "Dosro Number:"
oi mug bhan num2

mug jod num1, num2 lai result
bol mug "Sum: " + result
```

## Project Structure

```
MUJI-LANG/
├── src/
│   ├── main.rs          # Entry point
│   ├── lexer.rs         # Tokenization
│   ├── parser.rs        # Syntax parsing
│   ├── interpreter.rs    # Code execution
│   └── error.rs         # Error handling
├── examples/            # Example programs
├── tests/              # Test cases
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

Common issues and solutions:

1. **Compilation Errors**
   - Ensure Rust is properly installed
   - Run `cargo clean` and try rebuilding

2. **Runtime Errors**
   - Check syntax in your MUJI LANG code
   - Verify input types match expectations

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

