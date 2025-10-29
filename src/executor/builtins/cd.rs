use std::path::PathBuf;
use crate::executor::{ExecutionResult, ExecutorError, Executor};

pub fn execute(args: &[String], executor: &mut Executor) -> Result<ExecutionResult, ExecutorError> {
    let target_dir = if args.is_empty() {
        // cd with no arguments >>> Home directory
        std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/"))
    } else {
        PathBuf::from(&args[0])
    };

    // If relative path, make it absolute relative to current directory
    let absolute_path = if target_dir.is_relative() {
        executor.get_current_dir().join(target_dir)
    } else {
        target_dir
    };

    // Check if directory exists  is actually a directory
    if !absolute_path.exists() {
        return Err(ExecutorError::ChangeDirectoryError(
            format!("{}: No such file or directory", args[0])
        ));
    }

    if !absolute_path.is_dir() {
        return Err(ExecutorError::ChangeDirectoryError(
            format!("{}: Not a directory", args[0])
        ));
    }

    // change  the current directory
    if let Err(e) = std::env::set_current_dir(&absolute_path) {
        return Err(ExecutorError::ChangeDirectoryError(
            format!("{}: {}", args[0], e)
        ));
    }

    // Update executor's current directory >> return absolutpath
    executor.set_current_dir(absolute_path);

    Ok(ExecutionResult::default())
}