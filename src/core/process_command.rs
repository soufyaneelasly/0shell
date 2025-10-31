use crate::lexer;
use crate::parser;
use crate::executor::Executor;

pub fn process_command(input: &str, exec: &mut Executor) {
    if input.trim().is_empty() {
        return;
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
