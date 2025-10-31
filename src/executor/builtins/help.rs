use crate::executor::{ExecutionResult, ExecutorError};

pub fn execute(_args: &[String]) -> Result<ExecutionResult, ExecutorError> {
    let output = r#"0-shell - Minimal Unix Shell

Available commands:
  echo [text]        - Display text
  cd [dir]           - Change directory
  pwd                - Show current directory
  ls [dir]           - List files (-a for hidden)
  cat <file>         - Show file content
  cp <src> <dst>     - Copy files/directories
  rm <file>          - Remove files (-r for directories)
  mv <src> <dst>     - Move/rename files
  mkdir <dir>        - Create directories
  clear              - Clear the terminal screen
  exit               - Exit shell
  help               - This help message

"#;

    Ok(ExecutionResult {
        output: output.to_string(),
        success: true,
        should_exit: false,
    })
}