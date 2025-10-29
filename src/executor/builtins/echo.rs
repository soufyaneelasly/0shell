use crate::executor::{ExecutionResult, ExecutorError};

pub fn execute(args: &[String]) -> Result<ExecutionResult, ExecutorError> {
    let output = args.join(" ");
    Ok(ExecutionResult {
        output: output + "\n",
        success: true,
        should_exit: false,
    })
}