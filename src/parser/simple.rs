use crate::parser::types::{Command, SimpleCommand, ParseError,};
use crate::lexer::{Token, TokenKind};

/// Parse a simple command - just words/arguments
/// Example: `echo "hello world"` >>>> SimpleCommand { args: ["echo", "hello world"] }
pub fn parse_simple(tokens: &[Token]) -> Result<(Command, usize), ParseError> {
    let mut args = Vec::new();
    let mut pos = 0;

    // Collect words until operator or end 

    while pos < tokens.len() {
        match &tokens[pos].kind {
            TokenKind::Word => {
                args.push(tokens[pos].value.clone());
                pos += 1;
            }
            TokenKind::Eof => {
                break; // End of input
            }
            // Stop at any operator - these will be handled by higher-level >> parsers
            TokenKind::Pipe | TokenKind::RedirectOut | TokenKind::RedirectIn |
            TokenKind::RedirectAppend | TokenKind::And | TokenKind::Or | TokenKind::Semicolon => {
                break;
            }
        }
    }

    // Check if we have at least one argument (the command name)atlest one
    if args.is_empty() {
        return Err(ParseError::EmptyCommand);
    }

    let command = Command::Simple(SimpleCommand { args });
    Ok((command, pos))
}
////some test 


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{Token, TokenKind, Span};

    fn create_word_token(value: &str) -> Token {
        Token::new(
            TokenKind::Word,
            value.to_string(),
            Span::new(1, 1, 1, value.len()),
        )
    }

    fn create_eof_token() -> Token {
        Token::new(
            TokenKind::Eof,
            "".to_string(),
            Span::new(1, 1, 1, 1),
        )
    }

    #[test]
    fn test_parse_simple_basic() {
        let tokens = vec![
            create_word_token("echo"),
            create_word_token("hello"),
            create_word_token("world"),
            create_eof_token(),
        ];

        let (command, consumed) = parse_simple(&tokens).unwrap();
        
        assert_eq!(consumed, 3); // Should consume 3 tokens (not including EOF)
        
        if let Command::Simple(simple_cmd) = command {
            assert_eq!(simple_cmd.args, vec!["echo", "hello", "world"]);
        } else {
            panic!("Expected Simple command");
        }
    }

    #[test]
    fn test_parse_simple_single_command() {
        let tokens = vec![
            create_word_token("ls"),
            create_eof_token(),
        ];

        let (command, consumed) = parse_simple(&tokens).unwrap();
        
        assert_eq!(consumed, 1);
        
        if let Command::Simple(simple_cmd) = command {
            assert_eq!(simple_cmd.args, vec!["ls"]);
        } else {
            panic!("Expected Simple command");
        }
    }
}