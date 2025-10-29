
#[derive(Debug)]
pub struct ExecutionResult {
    pub output: String,
    pub should_exit: bool,
}

impl Default for ExecutionResult {
    fn default() -> Self {
        Self {
            output: String::new(),
            should_exit: false,
        }
    }
}

#[derive(Debug)]
pub enum ExecutorError {
    CommandNotFound(String),
    IoError(std::io::Error),
    InvalidArguments(String),
}

impl std::fmt::Display for ExecutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutorError::CommandNotFound(cmd) => write!(f, "Command '{}' not found", cmd),
            ExecutorError::IoError(err) => write!(f, "I/O error: {}", err),
            ExecutorError::InvalidArguments(msg) => write!(f, "Invalid arguments: {}", msg),
        }
    }
}

impl std::error::Error for ExecutorError {}