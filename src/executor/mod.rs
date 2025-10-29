mod types;
pub mod builtins;

pub use types::{ExecutionResult, ExecutorError};
use crate::parser::Command;

pub struct Executor {
    current_dir: std::path::PathBuf,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            current_dir: std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
        }
    }

    pub fn execute(&mut self, command: &Command) -> Result<ExecutionResult, ExecutorError> {
        match command {
            Command::Simple(cmd) => self.execute_simple(&cmd.args),
            Command::Pipe(pipe_cmd) => self.execute_pipe(&pipe_cmd),
            Command::Redirect(redirect_cmd) => self.execute_redirect(&redirect_cmd),
            Command::And(and_cmd) => self.execute_and(&and_cmd),
            Command::Or(or_cmd) => self.execute_or(&or_cmd),
            Command::Sequence(seq_cmd) => self.execute_sequence(&seq_cmd),
        }
    }

    fn execute_simple(&mut self, args: &[String]) -> Result<ExecutionResult, ExecutorError> {
        if args.is_empty() {
            return Ok(ExecutionResult::default());
        }

        let command_name = &args[0];
        let command_args = &args[1..];

        match builtins::Builtin::from_name(command_name) {
            Some(builtin) => builtin.execute(command_args, self),
            None => Err(ExecutorError::CommandNotFound(command_name.clone())),
        }
    }

    fn execute_pipe(&mut self, _pipe_cmd: &crate::parser::PipeCommand) -> Result<ExecutionResult, ExecutorError> {
        // For now, just return a message - we'll implement pipes later
        Ok(ExecutionResult {
            output: "Pipes not yet implemented\n".to_string(),
            success: false,
            should_exit: false,
        })
    }

    fn execute_redirect(&mut self, redirect_cmd: &crate::parser::RedirectCommand) -> Result<ExecutionResult, ExecutorError> {
        // For now, just execute the command - we'll implement redirection later
        self.execute(&redirect_cmd.command)
    }

    fn execute_and(&mut self, and_cmd: &crate::parser::AndCommand) -> Result<ExecutionResult, ExecutorError> {
        let left_result = self.execute(&and_cmd.left)?;
        if left_result.success {
            self.execute(&and_cmd.right)
        } else {
            Ok(ExecutionResult {
                output: left_result.output,
                success: false,
                should_exit: false,
            })
        }
    }

    fn execute_or(&mut self, or_cmd: &crate::parser::OrCommand) -> Result<ExecutionResult, ExecutorError> {
        let left_result = self.execute(&or_cmd.left)?;
        if !left_result.success {
            self.execute(&or_cmd.right)
        } else {
            Ok(left_result)
        }
    }

    fn execute_sequence(&mut self, seq_cmd: &crate::parser::SequenceCommand) -> Result<ExecutionResult, ExecutorError> {
        let mut last_result = ExecutionResult::default();
        
        for cmd in &seq_cmd.commands {
            last_result = self.execute(cmd)?;
        }
        
        Ok(last_result)
    }

    // Helper method for builtins to update current directory
    pub fn set_current_dir(&mut self, path: std::path::PathBuf) {
        self.current_dir = path;
    }

    pub fn get_current_dir(&self) -> &std::path::Path {
        &self.current_dir
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}