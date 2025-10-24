mod lexer;
mod parser;
// mod executor; // Coming soon!
// this main just for testing my parser and lexer 

use std::io::{self, Write};

fn main() {
    println!("0-shell v0.1.0");
    println!("Type 'exit' to quit, 'help' for commands");
    
    run_shell_loop();
}

fn run_shell_loop() {
    loop {
        // Display prompt and read input
        print!("$ ");
        io::stdout().flush().unwrap(); // Important: flush the prompt
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // Ctrl+D was pressed
                println!("\nGoodbye!");
                break;
            }
            Ok(_) => {
                // Process the input
                process_command(input.trim());
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
            }
        }
    }
}

fn process_command(input: &str) {
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
            // Debug: show tokens (you can comment this out later)
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
                    // Debug: show AST (you can comment this out later)
                    if cfg!(debug_assertions) {
                        println!("[DEBUG] AST: {:?}", ast);
                    }
                    
                    // TODO: Add command execution here  
                    // executor::execute(ast).unwrap_or_else(|e| eprintln!("Error: {}", e));
                    
                    // Temporary: Show what we parsed
                    show_parsed_command(&ast);
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

/// Temporary function to demonstrate parsing results
fn show_parsed_command(command: &parser::Command) {
    match command {
        parser::Command::Simple(cmd) => {
            println!("Simple command: {}", cmd.args.join(" "));
        }
        parser::Command::Pipe(pipe_cmd) => {
            println!("Pipe command: left | right");
            // You could recursively show the structure here
        }
        parser::Command::Redirect(redirect_cmd) => {
            let op = match redirect_cmd.operator {
                parser::RedirectOp::Output => ">",
                parser::RedirectOp::Input => "<", 
                parser::RedirectOp::Append => ">>",
            };
            println!("Redirect command: {} {} {}", 
                     format_command(&redirect_cmd.command), op, redirect_cmd.filename);
        }
        parser::Command::And(and_cmd) => {
            println!("And command: {} && {}", 
                     format_command(&and_cmd.left), format_command(&and_cmd.right));
        }
        parser::Command::Or(or_cmd) => {
            println!("Or command: {} || {}", 
                     format_command(&or_cmd.left), format_command(&or_cmd.right));
        }
        parser::Command::Sequence(seq_cmd) => {
            println!("Sequence command: {} commands", seq_cmd.commands.len());
            for (i, cmd) in seq_cmd.commands.iter().enumerate() {
                println!("  {}. {}", i + 1, format_command(cmd));
            }
        }
    }
}

/// Helper to format any command for display
fn format_command(command: &parser::Command) -> String {
    match command {
        parser::Command::Simple(cmd) => cmd.args.join(" "),
        parser::Command::Pipe(_) => "(pipe)".to_string(),
        parser::Command::Redirect(_) => "(redirect)".to_string(),
        parser::Command::And(_) => "(and)".to_string(),
        parser::Command::Or(_) => "(or)".to_string(),
        parser::Command::Sequence(_) => "(sequence)".to_string(),
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
    println!("Parser is active - commands will be parsed but not executed yet.");
}

// Optional: Add some integration tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        process_command(""); // Should not panic
        process_command("   "); // Should not panic
    }

    #[test] 
    fn test_builtin_commands() {
        // These should be handled without parsing
        process_command("exit");
        process_command("help");
    }
}