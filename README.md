<h1 align="center">
  calcli
</h1>
<div align="center">

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-green?style=flat)
![Version](https://img.shields.io/badge/version-0.1.2-blue?style=flat)
![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey?style=flat)

A lightweight TUI scientific calculator with Vi-style keybindings, built in Rust.

**Fast • Powerful • Terminal-Native**

<img src="https://raw.githubusercontent.com/Siphcy/calcli/main/assets/example.png">
</div>

---

## Table of Contents

- [Features](#features)
- [Usage](#usage)
- [Installation](#installation)
- [Supported Functions/Operators](#supported-functions-and-operators)
- [Keybindings](#keybindings)
- [Dependencies](#dependencies)
- [Project Structure](#project-structure)
- [Contributing](#contributing)

## Features

- **Mathematical Functions** - Trig (sin, cos, tan), logarithms (ln, log), hyperbolic functions, and more
- **Variable/Function System** - Define with `let x = 5`, 'let f(x) = 5x', supports `x`, `x1`, `y10` naming, automatic line references (`lin1`, `lin2`)
- **Implicit Multiplication** - Write `2x`, `3(5+2)`, `2sin(1)` naturally, decimal shortcuts (`.5` → `0.5`)
- **Vi-Style Keybindings** - Normal/Insert modes, `hjkl` navigation, `gg`/`GG`, word movements (`e`, `b`)
- **Rich TUI Interface** - Three-panel layout, scrollable history, live variable tracking, expression recall with `y`/`Enter`

## Usage

Open TUI

```bash
calcli -t
```

Open CLI

```bash
calcli
```

Import history

```
calcli --import <file>
```

#### Basic Calculations

```
2 + 2           # 4
5 * 3           # 15
sin(3.14159)    # ~0
ln(2.718)       # ~1
```

#### Variables

```
let x = 5       # Store value in variable x
let y = x * 2   # Use variables in expressions
x + y           # 15
```

#### Implicit Multiplication

```
2x              # 2 * x
3(5+2)          # 3 * (5 + 2) = 21
2sin(1)         # 2 * sin(1)
```

#### Line References

```
5 + 3           # Result stored as lin1
lin1 * 2        # Use previous result
```

#### Decimal Shortcuts

```
.5              # 0.5
2.              # 2.0
```

## Installation

#### Linux/macOS

```bash
curl -sSL https://raw.githubusercontent.com/Siphcy/calcli/main/install.sh | sh
```

#### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/Siphcy/calcli/main/install.ps1 | iex
```

### Package Managers

#### Arch Linux (AUR) [Not added yet]

```bash
yay -S calcli
# or
paru -S calcli
```

or from source

```bash
git clone https://github.com/Siphcy/calcli.git
cd calcli
makepkg -si
```

#### Nix/NixOS

```bash
# Run directly without installing
nix run github:Siphcy/calcli

# Install to profile
nix profile install github:Siphcy/calcli
```

Or add to your flake-based configuration:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    calcli.url = "github:Siphcy/calcli";
  };

  outputs = { self, nixpkgs, calcli, ... }: {
    nixosConfigurations.yourhostname = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        {
          environment.systemPackages = [
            calcli.packages.x86_64-linux.default
          ];
        }
      ];
    };
  };
}
```

#### Cargo (requires Rust)

```bash
cargo install calcli
```

### Pre-built Binaries

Download the latest binary for your platform from [Releases](https://github.com/Siphcy/calcli/releases):

| Platform                  | Binary                      |
| ------------------------- | --------------------------- |
| **Linux (x86_64)**        | `calcli-linux-x86_64`       |
| **Linux (musl)**          | `calcli-linux-x86_64-musl`  |
| **Windows**               | `calcli-windows-x86_64.exe` |
| **macOS (Intel)**         | `calcli-macos-x86_64`       |
| **macOS (Apple Silicon)** | `calcli-macos-aarch64`      |

### From Source

```bash
# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
cc861b6 (Added supprot to package managers and github work flow)
git clone https://github.com/Siphcy/calcli.git
cd calcli
cargo build --release
./target/release/calcli
```

## Supported Functions and Operators

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

| Command                                | Action                         |
| -------------------------------------- | ------------------------------ |
| `clear`                                | Clear all results              |
| `:w <filename>` / `:import<filename>`  | Export history as `<filename>` |
| `:r <filename>` / `:export <filename>` | Import history as `<filename>` |

### Syntax

- **Function Assignmet**: 'let <function> = <expression> (i.e. let f(x) = 5x)'
- **Variable Assignment**: `let <name> = <expression> (i.e. let n = 5)`
- **Line References**: `lin1`, `lin2`, `lin10`, etc.
- **Implicit Multiplication**: `2x`, `3(5+2)`, `2sin(1)`
- **Decimal Shortcuts**: `.5` → `0.5`, `2.` → `2.0`
- **Bracket Notation**: `[variable]` for explicit variable reference

## Keybindings

### Normal Mode

| Key       | Action                                    |
| --------- | ----------------------------------------- |
| `i`       | Enter insert mode at cursor               |
| `a`       | Enter insert mode after cursor            |
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
| `:`       | Open up command Prompt                    |

### Insert Mode

| Key         | Action                 |
| ----------- | ---------------------- |
| `Enter`     | Evaluate expression    |
| `Esc`       | Return to normal mode  |
| `←` / `→`   | Move cursor            |
| `k` / `↑`   | Previous history       |
| `j` / `↓`   | Next history           |
| `Backspace` | Delete character       |
| `:`         | Open up command Prompt |

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
