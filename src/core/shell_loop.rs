use std::io::{self, Write};
use crate::executor::Executor;
use super::process_command;

pub fn run_shell_loop(exec: &mut Executor) {
    loop {
        // Afficher l'invite principale
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
                            // Si on ouvre une quote alors qu'il n'y en avait pas
                            '\'' | '"' if current_quote.is_none() => {
                                current_quote = Some(c);
                            }
                            // Si on ferme la même quote qu'on avait ouverte
                            '\'' | '"' if current_quote == Some(c) => {
                                current_quote = None;
                            }
                            _ => {}
                        }
                    }

                    input.push_str(&line);

                    // Si aucune quote n'est ouverte → on exécute la commande
                    if current_quote.is_none() {
                        break;
                    } else {
             
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

        process_command::process_command(input.trim_end(), exec);
    }
}
