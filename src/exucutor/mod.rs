mod types;
mod builtins;

pub use types::{ExecutionResult, ExecutorError};
pub use builtins::Builtin;

pub struct Executor {
    // Shell state: current directory, environment variables, etc.
    current_dir: std::path::PathBuf,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            current_dir: std::env::current_dir().unwrap(),
        }
    }
    
    pub fn execute(&mut self, command: &crate::parser::Command) -> Result<ExecutionResult, ExecutorError> {
        // TODO: Implement command execution
        Ok(ExecutionResult::default())
    }
}