pub mod echo;
pub mod cd;
pub mod pwd;
pub mod ls;
pub mod exit;
pub mod cat;      // NEW
pub mod mkdir;    // NEW

use crate::executor::{Executor, ExecutionResult, ExecutorError};

pub enum Builtin {
    Echo,
    Cd,
    Pwd,
    Ls,
    Exit,
    Cat,    // NEW
    Mkdir,  // NEW
}

impl Builtin {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "echo" => Some(Builtin::Echo),
            "cd" => Some(Builtin::Cd),
            "pwd" => Some(Builtin::Pwd),
            "ls" => Some(Builtin::Ls),
            "exit" => Some(Builtin::Exit),
            "cat" => Some(Builtin::Cat),      // NEW
            "mkdir" => Some(Builtin::Mkdir),  // NEW
            _ => None,
        }
    }

    pub fn execute(&self, args: &[String], executor: &mut Executor) -> Result<ExecutionResult, ExecutorError> {
        match self {
            Builtin::Echo => echo::execute(args),
            Builtin::Cd => cd::execute(args, executor),
            Builtin::Pwd => pwd::execute(executor),
            Builtin::Ls => ls::execute(args, executor),
            Builtin::Exit => exit::execute(),
            Builtin::Cat => cat::execute(args),      // NEW
            Builtin::Mkdir => mkdir::execute(args),  // NEW
        }
    }
}