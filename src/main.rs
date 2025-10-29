mod lexer;
mod parser;
mod executor; // NEW: Now we're ready!

use std::io::{self, Write};

fn main() {
    println!("0-shell v0.1.0");
    println!("Type 'exit' to quit, 'help' for commands");
    
    // Initialize executor with shell state
    let mut exec = executor::Executor::new();
    
    run_shell_loop(&mut exec);
}

fn run_shell_loop(exec: &mut executor::Executor) {
    loop {
        // Display prompt and read input
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // Ctrl+D was pressed
                println!("\nGoodbye!");
                break;
            }
            Ok(_) => {
                // Process the input
                process_command(input.trim(), exec);
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
            }
        }
    }
}

fn process_command(input: &str, exec: &mut executor::Executor) {
    // Skip empty inputs and whitespace-only inputs
    if input.trim().is_empty() {
        return;
    }
    
    // Handle built-in commands that don't need parsing
    match input {
        "exit" => {
            println!("Goodbye!");
            std::process::exit(0);
        }
        "help" => {
            show_help();
            return;
        }
        _ => {} // Continue to lexing/parsing
    }
    
    // Lex the input
    let lexer = lexer::Lexer::new(input);
    match lexer.lex() {
        Ok(tokens) => {
            // Debug: show tokens (comment out in production)
            if cfg!(debug_assertions) {
                println!("[DEBUG] Tokens:");
                for token in &tokens {
                    if !matches!(token.kind, lexer::TokenKind::Eof) {
                        println!("  {:?}", token);
                    }
                }
            }
            
            // Parse the tokens into AST
            let mut parser = parser::Parser::new(tokens);
            match parser.parse() {
                Ok(ast) => {
                    // Debug: show AST (comment out in production)
                    if cfg!(debug_assertions) {
                        println!("[DEBUG] AST: {:?}", ast);
                    }
                    
                    // NEW: Execute the command!
                    match exec.execute(&ast) {
                        Ok(result) => {
                            // Handle execution result
                            if !result.output.is_empty() {
                                print!("{}", result.output);
                            }
                            if result.should_exit {
                                println!("Goodbye!");
                                std::process::exit(0);
                            }
                        }
                        Err(err) => {
                            eprintln!("Execution error: {}", err);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Parse error: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Lex error: {:?}", err);
        }
    }
}

fn show_help() {
    println!("0-shell - Minimal Unix Shell");
    println!();
    println!("Available commands:");
    println!("  echo [text]    - Display text");
    println!("  ls             - List files");
    println!("  pwd            - Show current directory");
    println!("  cd [dir]       - Change directory");
    println!("  cat [file]     - Show file content");
    println!("  cp <src> <dst> - Copy files");
    println!("  rm <file>      - Remove files");
    println!("  mv <src> <dst> - Move files");
    println!("  mkdir <dir>    - Create directory");
    println!("  exit           - Exit shell");
    println!("  help           - This help message");
    println!();
    println!("Supported features:");
    println!("  - Quotes: echo \"hello world\"");
    println!("  - Pipes: ls | grep rs");
    println!("  - Redirects: echo test > file.txt");
    println!("  - Logical: cmd1 && cmd2, cmd1 || cmd2");
    println!("  - Sequences: cmd1; cmd2; cmd3");
    println!();
    println!("Shell is now ACTIVE - commands will be executed!");
}

// Optional: Add some integration tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let mut exec = executor::Executor::new();
        process_command("", &mut exec); // Should not panic
        process_command("   ", &mut exec); // Should not panic
    }

    #[test] 
    fn test_builtin_commands() {
        let mut exec = executor::Executor::new();
        // These should be handled without parsing
        process_command("exit", &mut exec);
        process_command("help", &mut exec);
    }
}