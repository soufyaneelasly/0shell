use std::fs;
use std::path::Path;
use crate::executor::{ExecutionResult, ExecutorError};

pub fn execute(args: &[String]) -> Result<ExecutionResult, ExecutorError> {
    if args.len() != 2 {
        return Err(ExecutorError::InvalidArguments(
            "cp: usage: cp <source> <destination>".to_string()
        ));
    }
    
    let source = &args[0];
    let destination = &args[1];
    
    // Check if source    exists 
    if !Path::new(source).exists() {
        return Err(ExecutorError::IoError(
            std::io::Error::new(std::io::ErrorKind::NotFound, 
            format!("cp: {}: No such file or directory", source))
        ));
    }
    
    // Copy file or directory
    let metadata = fs::metadata(source)?;
    if metadata.is_dir() {
        // For directories, we need recursive copy
        copy_dir_all(source, destination)?;
    } else {
        // For files, simple copy
        fs::copy(source, destination)?;
    }
    
    Ok(ExecutionResult::default())
}

// Helper function for recursive directory    copy
fn copy_dir_all(src: &str, dst: &str) -> Result<(), std::io::Error> {
    fs::create_dir_all(dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = Path::new(dst).join(entry.file_name());
        
        if file_type.is_dir() {
            copy_dir_all(src_path.to_str().unwrap(), dst_path.to_str().unwrap())?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}