use std::fs;
use std::path::Path;
use crate::executor::{ExecutionResult, ExecutorError};

pub fn execute(args: &[String]) -> Result<ExecutionResult, ExecutorError> {
    if args.len() != 2 {
        return Err(ExecutorError::InvalidArguments(
            "cp: usage: cp <source> <destination>".to_string()
        ));
    }
    
    let source = Path::new(&args[0]);
    let destination = Path::new(&args[1]);
    
     if !source.exists() {
        return Err(ExecutorError::InvalidArguments(
            format!("cp: cannot stat '{}': No such file or directory", source.display())
        ));
    }

     let source_metadata = fs::metadata(source)
        .map_err(|e| ExecutorError::IoError(e))?;

     if source_metadata.is_dir() {
        copy_directory(source, destination)
    } else {
        copy_file(source, destination)
    }?;
    
    Ok(ExecutionResult::default())
}

 fn copy_file(source: &Path, destination: &Path) -> Result<(), ExecutorError> {
     let final_destination = if destination.is_dir() {
        destination.join(source.file_name().ok_or_else(|| {
            ExecutorError::InvalidArguments("cp: source has invalid filename".to_string())
        })?)
    } else {
        destination.to_path_buf()
    };

     if final_destination.exists() {
         println!("cp: overwriting '{}'", final_destination.display());
    }

     fs::copy(source, &final_destination)
        .map_err(ExecutorError::IoError)?;

    Ok(())
}

 fn copy_directory(source: &Path, destination: &Path) -> Result<(), ExecutorError> {
     if destination.exists() && !destination.is_dir() {
        return Err(ExecutorError::InvalidArguments(
            format!("cp: cannot overwrite non-directory '{}' with directory '{}'", 
                   destination.display(), source.display())
        ));
    }

     let final_destination = if destination.is_dir() {
        destination.join(source.file_name().ok_or_else(|| {
            ExecutorError::InvalidArguments("cp: source directory has invalid name".to_string())
        })?)
    } else {
        destination.to_path_buf()
    };

     fs::create_dir_all(&final_destination)
        .map_err(ExecutorError::IoError)?;

     copy_dir_all(source, &final_destination)
}

 fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), ExecutorError> {
     let entries = fs::read_dir(src)
        .map_err(ExecutorError::IoError)?;

    for entry in entries {
        let entry = entry.map_err(ExecutorError::IoError)?;
        let file_type = entry.file_type().map_err(ExecutorError::IoError)?;
        let src_path = entry.path();
        
        // Get filename safely
        let file_name = entry.file_name();
        let dst_path = dst.join(file_name);

        if file_type.is_dir() {
             fs::create_dir_all(&dst_path).map_err(ExecutorError::IoError)?;
            copy_dir_all(&src_path, &dst_path)?;
        } else if file_type.is_file() {
             fs::copy(&src_path, &dst_path).map_err(ExecutorError::IoError)?;
        } else {
             println!("cp: skipping special file '{}'", src_path.display());
        }
    }

    Ok(())
}