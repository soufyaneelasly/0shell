use std::fs;
use std::io::{self, BufRead};
use crate::executor::{ExecutionResult, ExecutorError};

pub fn execute(args: &[String]) -> Result<ExecutionResult, ExecutorError> {
    if args.is_empty() {
        
        let stdin = io::stdin();
        let mut output = String::new();
        
        for line in stdin.lock().lines() {
            match line {
                Ok(content) => {
                    println!("{}", content);
                    output.push_str(&content);
                    output.push('\n');
                }
                Err(e) => return Err(ExecutorError::IoError(e)),
            }
        }
       
        return Ok(ExecutionResult {
            output,
            success: true,
            should_exit: false,
        });
        
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