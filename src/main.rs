use std::io::{self, Write};
use colored::Colorize;

mod commands;


fn main() {
    print_banner();
    shell_loop();
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


fn shell_loop() {
    loop {
        if is_tty() {
            print!("$ ");
            io::stdout().flush().unwrap();
        }

        let input = read_complete_input();
        
        if input.trim().is_empty() {
            continue;
        }

        let parts = shell_split(&input);
        if !parts.is_empty() {
            execute_command(&parts);
        }
    }
}

fn read_complete_input() -> String {
    let mut input = String::new();

    loop {
        let mut line = String::new();
        
        match io::stdin().read_line(&mut line) {
            Ok(0) => return String::new(),
            Err(_) => return String::new(),
            Ok(_) => {}
        }

        input.push_str(&line);

        if has_balanced_quotes(&input) {
            return input;
        }

        if is_tty() {
            print!("> ");
            io::stdout().flush().unwrap();
        }
    }
}

fn is_tty() -> bool {
    atty::is(atty::Stream::Stdin)
}

fn has_balanced_quotes(input: &str) -> bool {
    let mut in_single = false;
    let mut in_double = false;
    let mut escape_next = false;

    for ch in input.chars() {
        if escape_next {
            escape_next = false;
            continue;
        }

        match ch {
            '\\' if !in_single => escape_next = true,
            '\'' if !in_double => in_single = !in_single,
            '"' if !in_single => in_double = !in_double,
            _ => {}
        }
    }

    !in_single && !in_double
}

fn shell_split(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_single = false;
    let mut in_double = false;
    let mut escape_next = false;

    for ch in input.chars() {
        if escape_next {
            current.push(ch);
            escape_next = false;
            continue;
        }

        match ch {
            '\\' if !in_single => {
                escape_next = true;
            }
            '\'' if !in_double => {
                in_single = !in_single;
            }
            '"' if !in_single => {
                in_double = !in_double;
            }
            ' ' | '\t' | '\n' if !in_single && !in_double => {
                if !current.is_empty() {
                    parts.push(current.trim_end().to_string());
                    current.clear();
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if !current.is_empty() {
        parts.push(current.trim_end().to_string());
    }

    parts
}

fn execute_command(parts: &[String]) {
    let cmd = parts[0].as_str();
    let args: Vec<&str> = parts[1..].iter().map(|s| s.as_str()).collect();

    match cmd {
        "echo" => commands::cmd_echo(&args),
        "cd" => commands::cmd_cd(&args),
        "pwd" => commands::cmd_pwd(),
        "exit" => std::process::exit(0),
        "ls" => commands::cmd_ls(&args),
        "cat" => commands::cmd_cat(&args),
        "mkdir" => commands::cmd_mkdir(&args),
        "mv" => commands::cmd_mv(&args),
        _ => eprintln!("Command '{}' not found", cmd),
    }
}