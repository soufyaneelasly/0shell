mod lexer;
mod parser;
mod executor;
use colored::Colorize;

use std::io::{self, Write};

fn main() {
    print_banner();
    let mut exec = executor::Executor::new();
    run_shell_loop(&mut exec);
}

fn print_banner() {
    let banner = vec![
        "╔══════════════════════════════════════════════════════════════════════╗",
        "║                                                                      ║",
        "║  ██████╗       ███████╗██╗  ██╗███████╗██╗     ██╗                   ║",
        "║ ██╔═████╗      ██╔════╝██║  ██║██╔════╝██║     ██║                   ║",
        "║ ██║██╔██║█████╗███████╗███████║█████╗  ██║     ██║                   ║",
        "║ ████╔╝██║╚════╝╚════██║██╔══██║██╔══╝  ██║     ██║                   ║",
        "║ ╚██████╔╝      ███████║██║  ██║███████╗███████╗███████╗              ║",
        "║  ╚═════╝       ╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝              ║",
        "║                                                                      ║",
        "║                                                                      ║",
        "║                                                                      ║",
        "╚══════════════════════════════════════════════════════════════════════╝",
    ];

    for line in banner {
        println!("{}", line.red().bold());
    }
    println!();

    println!("{}", "Welcome to 0-Shell!".green().bold());
    println!(
        "Type {} for available commands or {} to quit.",
        "help".cyan(),
        "exit".cyan()
    );
    println!("{}", "Press Ctrl+D to exit gracefully.".yellow());
    println!();
}

fn run_shell_loop(exec: &mut executor::Executor) {
    loop {
        // Afficher l’invite principale
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let mut current_quote: Option<char> = None; // ← on garde le type de quote ouverte

        loop {
            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(0) => {
                    println!("\nGoodbye!");
                    return;
                }
                Ok(_) => {
                    for c in line.chars() {
                        match c {
                            // Si on ouvre une quote alors qu’il n’y en avait pas
                            '\'' | '"' if current_quote.is_none() => {
                                current_quote = Some(c);
                            }
                            // Si on ferme la même quote qu’on avait ouverte
                            '\'' | '"' if current_quote == Some(c) => {
                                current_quote = None;
                            }
                            _ => {}
                        }
                    }

                    input.push_str(&line);

                    // Si aucune quote n’est ouverte → on exécute la commande
                    if current_quote.is_none() {
                        break;
                    } else {
                        // Invite secondaire pendant le multiline
                        print!("> ");
                        io::stdout().flush().unwrap();
                    }
                }
                Err(error) => {
                    eprintln!("Error reading input: {}", error);
                    return;
                }
            }
        }

        process_command(input.trim_end(), exec);
    }
}

fn process_command(input: &str, exec: &mut executor::Executor) {
    if input.trim().is_empty() {
        return;
    }

    match input {
        "exit" => {
            println!("Goodbye!");
            std::process::exit(0);
        }
        "help" => {
            show_help();
            return;
        }
        _ => {}
    }

    let lexer = lexer::Lexer::new(input);
    match lexer.lex() {
        Ok(tokens) => {
            let mut parser = parser::Parser::new(tokens);
            match parser.parse() {
                Ok(ast) => match exec.execute(&ast) {
                    Ok(result) => {
                        if !result.output.is_empty() {
                            print!("{}", result.output);
                        }
                        if result.should_exit {
                            println!("Goodbye!");
                            std::process::exit(0);
                        }
                    }
                    Err(err) => eprintln!("Execution error: {}", err),
                },
                Err(err) => eprintln!("Parse error: {}", err),
            }
        }
        Err(err) => eprintln!("Lex error: {:?}", err),
    }
}

fn show_help() {
    println!("0-shell - Minimal Unix Shell\n");
    println!("Available commands:");
    println!("  echo [text]        - Display text");
    println!("  cd [dir]           - Change directory");
    println!("  pwd                - Show current directory");
    println!("  ls [dir]           - List files (-a for hidden)");
    println!("  cat <file>         - Show file content");
    println!("  cp <src> <dst>     - Copy files/directories");
    println!("  rm <file>          - Remove files (-r for directories)");
    println!("  mv <src> <dst>     - Move/rename files");
    println!("  mkdir <dir>        - Create directories");
    println!("  clear              - Clear the terminal screen");
    println!("  exit               - Exit shell");
    println!("  help               - This help message\n");
    println!("All core Unix commands are now implemented!");
}

