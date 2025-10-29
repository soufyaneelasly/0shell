// Module declarations
mod types;
mod simple;
mod pipes;
mod redirects;
mod logical;
mod sequences;
mod parser;

// Public API - only what main.rs needs to use
pub use types::Command;
pub use parser::Parser;

// Internal re-exports for parser modules to use each other
pub(crate) use types::{
    SimpleCommand, PipeCommand, RedirectCommand, AndCommand, OrCommand, SequenceCommand
};