use std::fs;
use std::path::Path;
use crate::executor::{ExecutionResult, ExecutorError};

pub fn execute(args: &[String]) -> Result<ExecutionResult, ExecutorError> {
    if args.is_empty() {
        return Err(ExecutorError::InvalidArguments(
            "rm: missing file argument".to_string()
        ));
    }
    
    let recursive = args.contains(&"-r".to_string()) || args.contains(&"-R".to_string());
    let files: Vec<&String> = args.iter()
        .filter(|arg| !arg.starts_with('-'))
        .collect();
    
    if files.is_empty() {
        return Err(ExecutorError::InvalidArguments(
            "rm: missing file argument".to_string()
        ));
    }
    
    for file in files {
        let path = Path::new(file);
        
        if !path.exists() {
            return Err(ExecutorError::IoError(
                std::io::Error::new(std::io::ErrorKind::NotFound,
                format!("rm: {}: No such file or directory", file))
            ));
        }
        
        if path.is_dir() {
            if recursive {
                fs::remove_dir_all(path)?;
            } else {
                return Err(ExecutorError::InvalidArguments(
                    format!("rm: {}: is a directory (use -r to remove recursively)", file)
                ));
            }
        } else {
            fs::remove_file(path)?;
        }
    }
    
    Ok(ExecutionResult::default())
}