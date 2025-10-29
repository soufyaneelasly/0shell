use crate::executor::{ExecutionResult, ExecutorError, Executor};

pub fn execute(executor: &Executor) -> Result<ExecutionResult, ExecutorError> {
    let current_dir = executor.get_current_dir();
    let output = current_dir.to_string_lossy().to_string() + "\n";
    
    Ok(ExecutionResult {
        output,
        success: true,
        should_exit: false,
    })
}