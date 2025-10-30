use crate::executor::{ExecutionResult, ExecutorError};

pub fn execute(args: &[String]) -> Result<ExecutionResult, ExecutorError> {
    // Reconstituer la commande entière (avec espaces)
    let full = args.join(" ");

    // Si le texte est entre quotes simples ou doubles, on les enlève
    let output = if (full.starts_with('\'') && full.ends_with('\''))
        || (full.starts_with('"') && full.ends_with('"'))
    {
        full[1..full.len() - 1].to_string()
    } else {
        full
    };

    Ok(ExecutionResult {
        output: output + "\n",
        success: true,
        should_exit: false,
    })
}
