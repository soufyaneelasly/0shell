use crate::executor::{ExecutionResult, ExecutorError};

pub fn execute() -> Result<ExecutionResult, ExecutorError> {
    Ok(ExecutionResult {
        output: String::new(),
        success: true,
        should_exit: true,
    })
}