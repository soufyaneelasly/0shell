# 0shell 🚀

<div align="center">

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)
![Build](https://img.shields.io/badge/build-passing-brightgreen.svg?style=for-the-badge)

**A modern, powerful shell written in Rust from scratch**

*Blazingly fast • Memory safe • Feature-rich*

[Features](#-features) • [Installation](#-installation) • [Usage](#-usage) • [Architecture](#-architecture) • [Contributing](#-contributing)

</div>

---

## 📖 Overview

**0shell** is a fully-featured command-line shell implementation built entirely in Rust. Designed with modern programming principles, it provides a robust, efficient, and extensible platform for command execution with advanced features like piping, redirection, and logical operators.

### ✨ Key Highlights

- 🦀 **Pure Rust Implementation** - Leveraging Rust's memory safety and performance
- ⚡ **Zero-Cost Abstractions** - Minimal overhead, maximum speed
- 🔧 **Modular Architecture** - Clean separation of lexer, parser, and executor
- 🛡️ **Robust Error Handling** - Comprehensive error management
- 🎯 **POSIX-Inspired** - Familiar shell semantics and behavior

---

## 🎯 Features

### Core Functionality

- ✅ **Command Execution** - Execute built-in and external commands
- ✅ **File Operations** - `ls`, `cat`, `cp`, `mv`, `rm`, `mkdir`, and more
- ✅ **Directory Navigation** - `cd`, `pwd` with full path support
- ✅ **Input/Output** - `echo` with advanced formatting
- ✅ **Interactive Mode** - Line-by-line input with quote handling

### Advanced Features

- 🔄 **Command Chaining** - Sequential command execution (`;`)
- 🔗 **Logical Operators** - AND (`&&`) and OR (`||`) operators
- 🚰 **Pipes** - Chain commands with pipe operator (coming soon)
- 📝 **I/O Redirection** - Input/output redirection (in development)
- 💬 **Quote Handling** - Multi-line string support with `'` and `"`

### Built-in Commands

| Command | Description | Status |
|---------|-------------|--------|
| `cat` | Concatenate and display files | ✅ Active |
| `cd` | Change directory | ✅ Active |
| `clear` | Clear terminal screen | ✅ Active |
| `cp` | Copy files and directories | ✅ Active |
| `echo` | Display text | ✅ Active |
| `exit` | Exit the shell | ✅ Active |
| `ls` | List directory contents | ✅ Active |
| `mkdir` | Create directories | ✅ Active |
| `mv` | Move/rename files | ✅ Active |
| `pwd` | Print working directory | ✅ Active |
| `rm` | Remove files/directories | ✅ Active |

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         0shell Pipeline                          │
└─────────────────────────────────────────────────────────────────┘

    User Input
        │
        ▼
    ┌─────────┐
    │  Lexer  │  ← Tokenization & Quote Handling
    └─────────┘
        │
        ▼
    ┌─────────┐
    │ Parser  │  ← AST Construction & Syntax Analysis
    └─────────┘
        │
        ▼
    ┌──────────┐
    │ Executor │  ← Command Dispatch & Execution
    └──────────┘
        │
        ▼
   System Calls
        │
        ▼
     Output
```

### Module Breakdown

#### 🔤 **Lexer** (`src/lexer/`)
- Tokenizes raw input strings
- Handles quotes, operators, and special characters
- Produces a stream of tokens for parsing

#### 🌳 **Parser** (`src/parser/`)
- Builds Abstract Syntax Tree (AST)
- Validates command syntax
- Supports complex command structures (pipes, redirections, chains)

#### ⚙️ **Executor** (`src/executor/`)
- Dispatches commands to built-ins or external programs
- Manages process execution and I/O
- Handles redirections and pipes

---

## 🚀 Installation

### Prerequisites

- **Rust** 1.70+ and Cargo
- **Operating System**: Linux, macOS, or WSL2

### Build from Source

```bash
# Clone the repository
git clone https://github.com/soufyaneelasly/0shell.git
cd 0shell

# Build the project
cargo build --release

# Run the shell
cargo run --release
```

### Development Build

```bash
# Build and run in debug mode
cargo run

# Run with verbose output
RUST_LOG=debug cargo run
```

---

## 💻 Usage

### Starting the Shell

```bash
$ cargo run
0-shell v0.1.0
Type 'exit' to quit, 'help' for commands
$ 
```

### Example Commands

```bash
# Basic commands
$ pwd
/home/user/projects

$ ls -la
total 48
drwxr-xr-x  5 user user 4096 Oct 31 12:00 .
drwxr-xr-x 10 user user 4096 Oct 30 08:00 ..

$ echo "Hello, 0shell!"
Hello, 0shell!

# File operations
$ cat file.txt
Contents of file.txt

$ cp source.txt destination.txt
$ mv old_name.txt new_name.txt
$ rm unwanted.txt

# Directory operations
$ mkdir new_directory
$ cd new_directory
$ pwd
/home/user/projects/new_directory

# Command chaining
$ echo "First" ; echo "Second" ; echo "Third"
First
Second
Third

# Logical operators
$ mkdir test && cd test && pwd
/home/user/projects/test

$ false_command || echo "Previous command failed"
Previous command failed

# Multi-line strings
$ echo "This is a
> multi-line
> string"
This is a
multi-line
string
```

---

## 🛠️ Development

### Project Structure

```
0shell/
├── Cargo.toml              # Project configuration
├── README.md               # This file
├── .gitignore             # Git ignore rules
└── src/
    ├── main.rs            # Entry point & REPL
    ├── lexer/             # Tokenization
    │   ├── mod.rs
    │   ├── lexer.rs
    │   └── types.rs
    ├── parser/            # Syntax analysis
    │   ├── mod.rs
    │   ├── parser.rs
    │   ├── simple.rs
    │   └── types.rs
    └── executor/          # Command execution
        ├── mod.rs
        ├── types.rs
        ├── redirects.rs
        └── builtins/      # Built-in commands
            ├── mod.rs
            ├── cat.rs
            ├── cd.rs
            ├── echo.rs
            ├── ls.rs
            └── ...
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check for issues
cargo check
```

---

## 🗺️ Roadmap

### Phase 1: Core Features ✅
- [x] Basic command execution
- [x] Built-in commands
- [x] Command chaining
- [x] Logical operators (AND/OR)

### Phase 2: Advanced Features 🚧
- [ ] Full pipe implementation
- [ ] Input/output redirection (`>`, `<`, `>>`)
- [ ] Background processes (`&`)
- [ ] Job control
- [ ] Environment variables
- [ ] Command history
- [ ] Tab completion

### Phase 3: Enhancement 📋
- [ ] Scripting support
- [ ] Configuration files
- [ ] Aliases
- [ ] Custom prompts
- [ ] Color support
- [ ] Plugin system

---

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Guidelines

- Follow Rust style guidelines
- Write tests for new features
- Update documentation
- Keep commits atomic and descriptive

---

## 👥 Authors

<table>
  <tr>
    <td align="center">
      <a href="https://github.com/M-MDI">
        <img src="https://github.com/M-MDI.png" width="100px;" alt="M-MDI"/>
        <br />
        <sub><b>M-MDI</b></sub>
      </a>
    </td>
    <td align="center">
      <a href="https://github.com/BOUTAMGHARINE">
        <img src="https://github.com/BOUTAMGHARINE.png" width="100px;" alt="BOUTAMGHARINE"/>
        <br />
        <sub><b>aboutamgh</b></sub>
      </a>
    </td>
    <td align="center">
      <a href="https://github.com/soufyaneelasly">
        <img src="https://github.com/soufyaneelasly.png" width="100px;" alt="selasly"/>
        <br />
        <sub><b>selasly</b></sub>
      </a>
    </td>
  </tr>
</table>

---

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- The Rust community for amazing tools and libraries
- POSIX shell standards for inspiration
- All contributors who help improve this project

---

## 📞 Contact & Support

- 🐛 **Issues**: [GitHub Issues](https://github.com/soufyaneelasly/0shell/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/soufyaneelasly/0shell/discussions)
- ⭐ **Star this repo** if you find it useful!

---

<div align="center">

**Made with ❤️ and 🦀 by the 0-shell team**

*"A shell built for the future, written in Rust"*

</div> 