use std::fs;
use std::path::Path;
use crate::executor::{ExecutionResult, ExecutorError};

pub fn execute(args: &[String]) -> Result<ExecutionResult, ExecutorError> {
    if args.len() != 2 {
        return Err(ExecutorError::InvalidArguments(
            "mv: usage: mv <source> <destination>".to_string()
        ));
    }
    
    let source = &args[0];
    let destination = &args[1];
    
    // Check if source exists
    if !Path::new(source).exists() {
        return Err(ExecutorError::IoError(
            std::io::Error::new(std::io::ErrorKind::NotFound,
            format!("mv: {}: No such file or directory", source))
        ));
    }
    
    // If destination is an existing directory, move source into it
    let final_destination = if Path::new(destination).is_dir() {
        Path::new(destination).join(Path::new(source).file_name().unwrap())
    } else {
        Path::new(destination).to_path_buf()
    };
    
    // Perform the move/rename
    fs::rename(source, final_destination)?;
    
    Ok(ExecutionResult::default())
}