use std::fs;
use crate::executor::{ExecutionResult, ExecutorError};

pub fn execute(args: &[String]) -> Result<ExecutionResult, ExecutorError> {
    if args.is_empty() {
        return Err(ExecutorError::InvalidArguments("cat: missing file argument".to_string()));
    }
    
    let mut output = String::new();
    for filename in args {
        let content = fs::read_to_string(filename)
            .map_err(|e| ExecutorError::IoError(e))?;
        output.push_str(&content);
    }
    
    Ok(ExecutionResult {
        output,
        success: true,
        should_exit: false,
    })
}