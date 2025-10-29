pub mod echo;
pub mod cd;
pub mod pwd;
// ... other builtins

pub enum Builtin {
    Echo,
    Cd,
    Pwd,
    // ... others
}

impl Builtin {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "echo" => Some(Builtin::Echo),
            "cd" => Some(Builtin::Cd),
            "pwd" => Some(Builtin::Pwd),
            _ => None,
        }
    }
    
    pub fn execute(&self, args: &[String]) -> Result<String, crate::executor::ExecutorError> {
        match self {
            Builtin::Echo => echo::execute(args),
            Builtin::Cd => cd::execute(args),
            Builtin::Pwd => pwd::execute(args),
        }
    }
}