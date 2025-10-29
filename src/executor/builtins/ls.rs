use std::fs;
use crate::executor::{ExecutionResult, ExecutorError, Executor};

pub fn execute(args: &[String], executor: &Executor) -> Result<ExecutionResult, ExecutorError> {
    let path = if args.is_empty() {
        executor.get_current_dir().to_path_buf()
    } else {
        executor.get_current_dir().join(&args[0])
    };

    let entries = fs::read_dir(&path)
        .map_err(|e| ExecutorError::IoError(e))?;

    let mut output = String::new();
    let mut entries_vec: Vec<String> = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| ExecutorError::IoError(e))?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        
        // Skip hidden files unless -a flag is provided
        if args.contains(&"-a".to_string()) || !file_name.starts_with('.') {
            entries_vec.push(file_name);
        }
    }

    // Sort entries alphabetically
    entries_vec.sort();

    for entry in entries_vec {
        output.push_str(&entry);
        output.push('\n');
    }

    Ok(ExecutionResult {
        output,
        success: true,
        should_exit: false,
    })
}