use std::fs;
use crate::executor::{ExecutionResult, ExecutorError};

pub fn execute(args: &[String]) -> Result<ExecutionResult, ExecutorError> {
    if args.is_empty() {
        return Err(ExecutorError::InvalidArguments("mkdir: missing directory argument".to_string()));
    }
    
    for dirname in args {
        fs::create_dir_all(dirname)
            .map_err(|e| ExecutorError::IoError(e))?;
    }
    
    Ok(ExecutionResult::default())
}