use crate::lexer::Token;

/// Main AST node representing any shell command
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Simple(SimpleCommand),
    Pipe(PipeCommand),
    Redirect(RedirectCommand),
    And(AndCommand),
    Or(OrCommand),
    Sequence(SequenceCommand),
}

/// A simple command with arguments: `echo hello world`
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleCommand {
    pub args: Vec<String>,
}

/// Pipe command: `left | right`
#[derive(Debug, Clone, PartialEq)]
pub struct PipeCommand {
    pub left: Box<Command>,
    pub right: Box<Command>,
}

/// Redirect command: `command > filename` or `command < filename`
#[derive(Debug, Clone, PartialEq)]
pub struct RedirectCommand {
    pub command: Box<Command>,
    pub operator: RedirectOp,
    pub filename: String,
}

/// Redirect operators
#[derive(Debug, Clone, PartialEq)]
pub enum RedirectOp {
    Output,    // >
    Input,     // <
    Append,    // >>
}

/// Logical AND: `left && right`
#[derive(Debug, Clone, PartialEq)]
pub struct AndCommand {
    pub left: Box<Command>,
    pub right: Box<Command>,
}

/// Logical OR: `left || right`
#[derive(Debug, Clone, PartialEq)]
pub struct OrCommand {
    pub left: Box<Command>,
    pub right: Box<Command>,
}

/// Command sequence: `first; second; third`
#[derive(Debug, Clone, PartialEq)]
pub struct SequenceCommand {
    pub commands: Vec<Command>,
}

/// Parser error types
#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token, Expected),
    UnexpectedEof,
    UnmatchedPipe,
    InvalidRedirect,
    EmptyCommand,
}

/// What the parser expected to find
#[derive(Debug, PartialEq)]
pub enum Expected {
    Command,
    Argument,
    Filename,
    Operator(String),  // "|", ">", "&&", etc.
}

// Helper implementations
impl Command {
    /// Create a simple command from arguments
    pub fn simple(args: Vec<String>) -> Self {
        Command::Simple(SimpleCommand { args })
    }
    
    /// Create a pipe command
    pub fn pipe(left: Command, right: Command) -> Self {
        Command::Pipe(PipeCommand {
            left: Box::new(left),
            right: Box::new(right),
        })
    }
    
    /// Create a redirect command
    pub fn redirect(command: Command, operator: RedirectOp, filename: String) -> Self {
        Command::Redirect(RedirectCommand {
            command: Box::new(command),
            operator,
            filename,
        })
    }
}

 impl RedirectOp {
    /// Convert from token kind to redirect operator
    pub fn from_token(token: &Token) -> Option<Self> {
        match token.kind {
            crate::lexer::TokenKind::RedirectOut => Some(RedirectOp::Output),
            crate::lexer::TokenKind::RedirectIn => Some(RedirectOp::Input),
            crate::lexer::TokenKind::RedirectAppend => Some(RedirectOp::Append),
            _ => None,
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(token, expected) => {
                write!(f, "Unexpected token '{:?}', expected {}", token, expected)
            }
            ParseError::UnexpectedEof => {
                write!(f, "Unexpected end of input")
            }
            ParseError::UnmatchedPipe => {
                write!(f, "Unmatched pipe operator '|'")
            }
            ParseError::InvalidRedirect => {
                write!(f, "Invalid redirect syntax")
            }
            ParseError::EmptyCommand => {
                write!(f, "Empty command")
            }
        }
    }
}

impl std::fmt::Display for Expected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expected::Command => write!(f, "command"),
            Expected::Argument => write!(f, "argument"),
            Expected::Filename => write!(f, "filename"),
            Expected::Operator(op) => write!(f, "operator '{}'", op),
        }
    }
}

// Convenience for error handling
impl std::error::Error for ParseError {}