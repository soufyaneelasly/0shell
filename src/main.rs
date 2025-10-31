mod lexer;
mod parser;
mod executor;
mod core;

fn main() {
    core::print_banner();
    let mut exec = executor::Executor::new();
    core::run_shell_loop(&mut exec);
}
