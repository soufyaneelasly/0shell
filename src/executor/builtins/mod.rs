pub mod echo;
pub mod cd;
pub mod pwd;
pub mod ls;
pub mod exit;
pub mod cat;    
pub mod cp;     
pub mod rm;     
pub mod mv;     
pub mod mkdir; 
pub mod clear;
pub mod help;

use crate::executor::{Executor, ExecutionResult, ExecutorError};
 
#[derive(Debug)]
 pub enum Builtin {
    Echo,
    Cd,
    Pwd,
    Ls,
    Exit,
    Cat,    
    Cp,    
    Rm,     
    Mv,     
    Mkdir,  
    Clear,
    Help,
}

impl Builtin {
    pub fn from_name(name: &str) -> Option<Self> {
 
        match name {
            "echo" => Some(Builtin::Echo),
            "cd" => Some(Builtin::Cd),
            "pwd" => Some(Builtin::Pwd),
            "ls" => Some(Builtin::Ls),
            "exit" => Some(Builtin::Exit),
            "cat" => Some(Builtin::Cat),    
            "cp" => Some(Builtin::Cp),      
            "rm" => Some(Builtin::Rm),      
            "mv" => Some(Builtin::Mv),      
            "mkdir" => Some(Builtin::Mkdir),
            "clear" => Some(Builtin::Clear),
            "help" => Some(Builtin::Help),
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
            Builtin::Cat => cat::execute(args),    
            Builtin::Cp => cp::execute(args),      
            Builtin::Rm => rm::execute(args),      
            Builtin::Mv => mv::execute(args),      
            Builtin::Mkdir => mkdir::execute(args),  
            Builtin::Clear => clear::execute(args),
            Builtin::Help => help::execute(args),
        }
    }
}

 