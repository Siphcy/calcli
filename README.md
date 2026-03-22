<h1 align="center">
  calcli
</h1>
<div align="center">

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![TUI](https://img.shields.io/badge/TUI-Terminal-blue?style=for-the-badge)
![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)
![Version](https://img.shields.io/badge/version-0.1.2-orange?style=for-the-badge)

A lightweight TUI scientific calculator with Vi-style keybindings, built in Rust.

**Fast • Powerful • Terminal-Native**

</div>

---

## Table of Contents

- [Installation](#installation)
- [Supported Functions & Operators](#supported-functions-&-operators)
- [Usage](#usage)
- [Keybindings](#keybindings)
- [Dependencies](#Dependencies)
- [Project Structure](#Project-Structure)
- [Contributing](##contributing)

## Installation

```bash
# Install Rust (if not already installed)

# Clone and run
git clone https://github.com/yourusername/calcli.git
cd calcli
cargo run --release
```

## Features

- **Mathematical Functions** - Trig (sin, cos, tan), logarithms (ln, log), hyperbolic functions, and more
- **Variable System** - Define with `let x = 5`, supports `x`, `x1`, `y10` naming, automatic line references (`lin1`, `lin2`)
- **Implicit Multiplication** - Write `2x`, `3(5+2)`, `2sin(1)` naturally, decimal shortcuts (`.5` → `0.5`)
- **Vi-Style Keybindings** - Modal editing (Normal/Editing), `hjkl` navigation, `gg`/`GG`, word movements (`e`, `b`)
- **Rich TUI Interface** - Three-panel layout, scrollable history, live variable tracking, expression recall with `y`/`Enter`

## Supported Functions & Operators

### Mathematical Functions

| Function   | Description        | Example             |
| ---------- | ------------------ | ------------------- |
| `sin(x)`   | Sine (radians)     | `sin(1.57)` → 1.0   |
| `cos(x)`   | Cosine (radians)   | `cos(0)` → 1.0      |
| `tan(x)`   | Tangent (radians)  | `tan(0.785)` → ~1.0 |
| `asin(x)`  | Arcsine            | `asin(1)` → 1.57    |
| `acos(x)`  | Arccosine          | `acos(0)` → 1.57    |
| `atan(x)`  | Arctangent         | `atan(1)` → 0.785   |
| `sinh(x)`  | Hyperbolic sine    | `sinh(0)` → 0       |
| `cosh(x)`  | Hyperbolic cosine  | `cosh(0)` → 1       |
| `tanh(x)`  | Hyperbolic tangent | `tanh(0)` → 0       |
| `ln(x)`    | Natural logarithm  | `ln(2.718)` → ~1.0  |
| `log(x)`   | Base-10 logarithm  | `log(100)` → 2.0    |
| `log2(x)`  | Base-2 logarithm   | `log2(8)` → 3.0     |
| `sqrt(x)`  | Square root        | `sqrt(16)` → 4.0    |
| `exp(x)`   | e^x                | `exp(1)` → 2.718    |
| `abs(x)`   | Absolute value     | `abs(-5)` → 5.0     |
| `floor(x)` | Round down         | `floor(3.7)` → 3.0  |
| `ceil(x)`  | Round up           | `ceil(3.2)` → 4.0   |
| `round(x)` | Round to nearest   | `round(3.5)` → 4.0  |

### Mathematical Constants

| Constant    | Description    | Value      |
| ----------- | -------------- | ---------- |
| `pi` or `π` | Pi             | 3.14159... |
| `e`         | Euler's number | 2.71828... |

### Operators

| Operator | Description       | Example        |
| -------- | ----------------- | -------------- |
| `+`      | Addition          | `5 + 3` → 8    |
| `-`      | Subtraction       | `10 - 4` → 6   |
| `*`      | Multiplication    | `7 * 6` → 42   |
| `/`      | Division          | `20 / 4` → 5   |
| `^`      | Exponentiation    | `2^8` → 256    |
| `%`      | Modulo            | `17 % 5` → 2   |
| `()`     | Grouping          | `(2+3)*4` → 20 |
| `[]`     | Variable grouping | `[x]2` → x\*2  |

## Commands

| Command | Action            |
| ------- | ----------------- |
| `clear` | Clear all results |

### Syntax

- **Variable Assignment**: `let <name> = <expression>`
- **Line References**: `lin1`, `lin2`, `lin10`, etc.
- **Implicit Multiplication**: `2x`, `3(5+2)`, `2sin(1)`
- **Decimal Shortcuts**: `.5` → `0.5`, `2.` → `2.0`
- **Bracket Notation**: `[variable]` for explicit variable reference

## Usage

### Basic Calculations

```
2 + 2           # 4
5 * 3           # 15
sin(3.14159)    # ~0
ln(2.718)       # ~1
```

### Variables

```
let x = 5       # Store value in variable x
let y = x * 2   # Use variables in expressions
x + y           # 15
```

### Implicit Multiplication

```
2x              # 2 * x
3(5+2)          # 3 * (5 + 2) = 21
2sin(1)         # 2 * sin(1)
```

### Line References

```
5 + 3           # Result stored as lin1
lin1 * 2        # Use previous result
```

### Decimal Shortcuts

```
.5              # 0.5
2.              # 2.0
```

## Keybindings

### Normal Mode

| Key       | Action                                    |
| --------- | ----------------------------------------- |
| `i`       | Enter editing mode at cursor              |
| `a`       | Enter editing mode after cursor           |
| `q`       | Quit                                      |
| `h` / `←` | Move cursor left                          |
| `l` / `→` | Move cursor right                         |
| `e`       | Move to end of word                       |
| `b`       | Move to beginning of word                 |
| `k` / `↑` | Previous history                          |
| `j` / `↓` | Next history                              |
| `J`       | Scroll results down                       |
| `K`       | Scroll results up                         |
| `gg`      | Scroll to top of results                  |
| `GG`      | Scroll to bottom of results               |
| `N`       | Scroll variables down                     |
| `P`       | Scroll variables up                       |
| `Enter`   | Evaluate expression                       |
| `y`       | Copy selected result (full line) to input |
| `Esc`     | Clear input                               |

### Editing Mode

| Key         | Action                |
| ----------- | --------------------- |
| `Enter`     | Evaluate expression   |
| `Esc`       | Return to normal mode |
| `←` / `→`   | Move cursor           |
| `Backspace` | Delete character      |

### Dependencies

| Component             | Dependencies                                              | Purpose                                  |
| --------------------- | --------------------------------------------------------- | ---------------------------------------- |
| **TUI Framework**     | [Ratatui](https://github.com/ratatui-org/ratatui)         | Terminal user interface                  |
| **Event Handling**    | [Crossterm](https://github.com/crossterm-rs/crossterm)    | Keyboard input & cursor control          |
| **Expression Parser** | [meval](https://github.com/rekka/meval-rs)                | Mathematical expression evaluation       |
| **Regex Engine**      | [fancy-regex](https://github.com/fancy-regex/fancy-regex) | Advanced regex with lookahead/lookbehind |
| **Error Handling**    | [color-eyre](https://github.com/eyre-rs/eyre)             | Beautiful error reports                  |

### Project Structure

```
calcli/
├── src/
│   ├── main.rs              # Entry point
│   ├── input_handler.rs     # TUI and keybinding logic
│   ├── eval.rs              # Expression evaluation engine
│   ├── eval_context.rs      # Evaluation context management
│   ├── vi_inputs.rs         # History navigation
│   ├── function.rs          # Function definitions
│   └── unit_conversion.rs   # Unit conversion support
├── tests/
│   └── eval_tests.rs        # Comprehensive test suite (23+ tests)
├── Cargo.toml
└── README.md
```

## Contributing

Contributions are welcome! Areas for improvement:

- [ ] Additional mathematical functions (factorial, combinations, etc.)
- [ ] Unit conversion system
- [ ] Custom function definitions
- [ ] Expression graphing
- [ ] Configuration file support
- [ ] Themes and color customization
- [ ] Export/import calculation history

---

<div align="center">

Made with Rust

**[Report Bug](https://github.com/Siphcy/calcli/issues)** • **[Request Feature](https://github.com/Siphcy/calcli/issues)**

</div>
