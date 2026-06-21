# LyangLang

A toy programming language with Romanized Nepali keywords, written in Rust. Source files use `.nbh`. Run with `lyangpiler`.

Two execution backends: tree-walking interpreter (default) and a bytecode VM (`--vm`). Both produce identical results.

![LyangLang](https://img.shields.io/badge/LyangLang-v0.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Build](https://img.shields.io/github/actions/workflow/status/Konseptt/LyangLang/build.yml?branch=main&label=build)

## Install

**Quickest (no Rust needed):**

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/Konseptt/LyangLang/main/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/Konseptt/LyangLang/main/install.ps1 | iex
```

**From crates.io (needs Rust):**

```bash
cargo install lyanglyang --locked
```

**Build from source:**

```bash
git clone https://github.com/Konseptt/LyangLang.git
cd LyangLang
cargo build --release
./target/release/lyangpiler run example.nbh
```

**Direct download:** grab the binary for your OS from [Releases](https://github.com/Konseptt/LyangLang/releases/latest).

| System | Download |
|--------|----------|
| Windows 64-bit | [lyangpiler-windows-x86_64.zip](https://github.com/Konseptt/LyangLang/releases/latest/download/lyangpiler-windows-x86_64.zip) |
| Linux 64-bit Intel/AMD | [lyangpiler-linux-x86_64.tar.gz](https://github.com/Konseptt/LyangLang/releases/latest/download/lyangpiler-linux-x86_64.tar.gz) |
| Linux 64-bit ARM | [lyangpiler-linux-aarch64.tar.gz](https://github.com/Konseptt/LyangLang/releases/latest/download/lyangpiler-linux-aarch64.tar.gz) |
| macOS Apple Silicon | [lyangpiler-macos-aarch64.tar.gz](https://github.com/Konseptt/LyangLang/releases/latest/download/lyangpiler-macos-aarch64.tar.gz) |
| macOS Intel | [lyangpiler-macos-x86_64.tar.gz](https://github.com/Konseptt/LyangLang/releases/latest/download/lyangpiler-macos-x86_64.tar.gz) |

## Run a program

```bash
lyangpiler run file.nbh           # tree-walking interpreter (default)
lyangpiler run file.nbh --vm      # bytecode VM backend
```

Legacy shorthand: `lyangpiler file.nbh --vm` works the same.

```bash
lyangpiler check file.nbh         # validate syntax, don't execute
lyangpiler new myproject          # scaffold a new project
```

After install, Mac/Linux users may need `source ~/.lyangpiler/enable.sh` or a fresh terminal for `lyangpiler` to appear on PATH.

## Language reference

Keywords are Romanized Nepali. Every word in a two-word keyword (like `bol mug`, `oi mug`) must be typed as-is.

### Print

```bash
bol mug "Namaste, world!"              // print literal string
bol mug "Namaste " + naam + "!"         // string concatenation with +
bol mug "Number: " + age                // variable in string (autoconverted)
```

### Variables

```bash
oi mug age = 25                         // declare number variable
oi mug naam = "Ram"                     // declare string variable
```

### Input

```bash
oi mug bhan naam                        // prompt "> ", read input into naam
```

`bhan` reads a string from the user. To use input in arithmetic you'd need to convert it, but LyangLang doesn't have a parse/convert function yet. So `bhan` is for strings, and `oi mug x = 5` is for numbers.

### Arithmetic

All use `checked` arithmetic, so overflow crashes instead of wrapping silently.

```bash
mug jod a, b lai sum                    // sum = a + b
mug ghata a, b lai diff                 // diff = a - b
mug guna a, b lai prod                  // prod = a * b
mug bhag a, b lai quot                  // quot = a / b (division by zero crashes)
```

### Conditionals

```bash
// if-then-end
yedi age babaal "18" bhane
    bol mug "Adult"
sakiyo

// if-else if-end
yedi rang babaal "rato" bhane
    bol mug "Red"
aile feri rang babaal "nilo" bhane
    bol mug "Blue"
sakiyo
```

- `yedi` / `yadi`: if (both work, same token)
- `babaal`: equals comparison
- `laamo`: not-equals comparison
- `bhane`: then
- `aile feri`: else-if
- `sakiyo`: end of block

String comparisons are case-insensitive in both backends (`"RATO" babaal "rato"` is true).

### Comments

```bash
// This is a comment, everything after // is ignored
```

### Keyword quick reference

| Syntax | Meaning | Nepali word |
|--------|---------|-------------|
| `bol mug` | print | bol = speak, mug = emphasis |
| `oi mug` | declare variable | oi = exclamation, mug = emphasis |
| `oi mug bhan` | read input | bhan = speak/say |
| `mug jod` | add | jod = join/add |
| `mug ghata` | subtract | ghata = decrease |
| `mug guna` | multiply | guna = multiply |
| `mug bhag` | divide | bhag = share/divide |
| `lai` | into (target of operation) | lai = to (dative) |
| `yedi` / `yadi` | if | yedi/yadi = if |
| `babaal` | equals condition | (slang, repurposed) |
| `laamo` | not-equals condition | laamo = long (repurposed) |
| `bhane` | then | bhane = then |
| `aile feri` | else-if | aile = now, feri = again |
| `sakiyo` | end block | sakiyo = finished/done |

## Examples

### Hello World

```bash
bol mug "Namaste, world!"
```

### Greet the user

```bash
bol mug "Timro naam k ho?"
oi mug bhan naam
bol mug "Namaste, " + naam + "!"
```

### Calculator

```bash
oi mug a = 12
oi mug b = 4

mug jod a, b lai sum
bol mug "Jod (add): " + sum
mug ghata a, b lai diff
bol mug "Ghata (subtract): " + diff
mug guna a, b lai prod
bol mug "Guna (multiply): " + prod
mug bhag a, b lai quot
bol mug "Bhag (divide): " + quot
```

### Color picker (conditionals)

```bash
bol mug "Timro man pasand rang?"
oi mug bhan rang

yedi rang babaal "rato" bhane
    bol mug "Rato rang, energy high!"
aile feri rang babaal "nilo" bhane
    bol mug "Nilo rang, chill vibes."
aile feri rang babaal "hariyo" bhane
    bol mug "Hariyo rang, nature mode."
sakiyo
```

### Full demos in repo

```bash
lyangpiler run example.nbh              # variables, arithmetic, I/O, conditions
lyangpiler run myapp/main.nbh           # shop calculator, slang, multi-branch
```

## Error handling

Four error types, reported to stderr with line info:

- `LexError`: malformed tokens or unterminated strings
- `ParseError`: grammar violations (missing `bhane`, `sakiyo`, etc.)
- `RuntimeError`: overflow, division by zero
- `NameError`: undefined variable
- `TypeError`: wrong type in arithmetic or comparison

## Project structure

```
src/
  lexer.rs          source → tokens
  parser.rs         tokens → AST
  ast.rs            Statement / expression types
  token.rs          Token enum
  interpreter.rs    tree-walking backend (default)
  compiler.rs       AST → bytecode
  bytecode.rs       Opcode / Value definitions
  vm.rs             bytecode VM backend
  error.rs          error types + formatting
  main.rs           CLI (clap)
example.nbh         feature tour
myapp/main.nbh      longer interactive demo
```

## Development

```bash
cargo build                               # build
cargo test                                # run tests (lexer, parser, vm, interpreter)
cargo test -- --nocapture                 # verbose output
cargo clippy                              # lints
cargo run -- run example.nbh --vm         # quick try
```

## Contributing

1. Fork the repo
2. Create a branch: `git checkout -b feature-name`
3. Make changes, add tests alongside
4. `cargo build && cargo test` must stay green
5. Push and open a pull request

## Troubleshooting

**Command not found after install?**
- Mac/Linux: `source ~/.lyangpiler/enable.sh` or open a new terminal
- Windows: confirm `%USERPROFILE%\.lyangpiler\bin` is on your PATH

**Build failures?**
- `rustup update stable` then `cargo clean && cargo build`

**Runtime errors?**
- Check for missing `sakiyo` to close blocks
- String comparisons: use `yedi var babaal "value"` not `yadi var == "value"`
- Arithmetic operands must be declared numbers

## License

MIT
