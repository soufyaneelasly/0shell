use std::io::{self, Write};
use crate::executor::{ExecutionResult, ExecutorError};

pub fn execute(_args: &[String]) -> Result<ExecutionResult, ExecutorError> {
    print!("\x1B[2J\x1B[3J\x1B[H");
    io::stdout().flush().map_err(|e| ExecutorError::IoError(e))?;
    
    Ok(ExecutionResult {
        output: String::new(),
        success: true,
        should_exit: false,
    })
}