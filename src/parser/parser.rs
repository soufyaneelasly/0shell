use crate::lexer::{Token, TokenKind};
use crate::parser::types::{Command, ParseError, Expected, RedirectOp};
use crate::parser::simple;

/// Main parser that coordinates the main parsing  process
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    /// Create a new parser with tokens from our  lexer
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Main entry point - parse tokens into an AST (look readme.md AST)
    pub fn parse(&mut self) -> Result<Command, ParseError> {
        let command = self.parse_sequence()?;
        
        // Ensure we consumed all tokens (except EOF)
        if !self.is_at_end() && !self.check(TokenKind::Eof) {
            return Err(ParseError::UnexpectedToken(
                self.current().clone(),
                Expected::Operator("end of command".to_string())
            ));
        }
        
        Ok(command)
    }

    /// Parse sequence of commands: cmd1 cmd2 cmd3 ..
    fn parse_sequence(&mut self) -> Result<Command, ParseError> {
        // Start with logical commands (AND/OR)
        let command = self.parse_logical()?;
        
        // If we have a semicolon, parse the rest of the sequence
        if self.match_token(TokenKind::Semicolon) {
            let mut commands = vec![command];
            
            // Parse all commands in the sequence
            while !self.is_at_end() && !self.check(TokenKind::Eof) {
                commands.push(self.parse_logical()?);
                
                // Stop if no more semicolons
                if !self.match_token(TokenKind::Semicolon) {
                    break;
                }
            }
            
            if commands.len() == 1 {
                Ok(commands.remove(0))
            } else {
                Ok(Command::Sequence(crate::parser::types::SequenceCommand { commands }))
            }
        } else {
            Ok(command)
        }
    }

    /// Parse logical operators: cmd1 && cmd2, cmd1 || cmd2
    fn parse_logical(&mut self) -> Result<Command, ParseError> {
        let mut command = self.parse_pipe()?;
        
        // Handle && and  || operators (left-associative) (explained in the readme.md)
        while self.check(TokenKind::And) || self.check(TokenKind::Or) {
            let operator_kind = self.current().kind.clone();
            
            self.advance(); // Consume it (the operator

            let right = self.parse_pipe()?;
            
            command = match operator_kind {
                TokenKind::And => Command::And(crate::parser::types::AndCommand {
                    left: Box::new(command),
                    right: Box::new(right),
                }),
                TokenKind::Or => Command::Or(crate::parser::types::OrCommand {
                    left: Box::new(command),
                    right: Box::new(right),
                }),
                _ => unreachable!(),
            };
        }
        
        Ok(command)
    }

    /// Parse pipe operators: cmd1 | cmd2 | cmd3
    fn parse_pipe(&mut self) -> Result<Command, ParseError> {
        let mut command = self.parse_redirect()?;
        
        // Handle pipe operators (left-associative)
        while self.match_token(TokenKind::Pipe) {
            let right = self.parse_redirect()?;
            
            command = Command::Pipe(crate::parser::types::PipeCommand {
                left: Box::new(command),
                right: Box::new(right),
            });
        }
        
        Ok(command)
    }

    /// Parse redirect operators: cmd > file, cmd < file
    fn parse_redirect(&mut self) -> Result<Command, ParseError> {
        let mut command = self.parse_simple()?;
        
        // Handle redirect operators
        while self.check(TokenKind::RedirectOut) || self.check(TokenKind::RedirectIn) || self.check(TokenKind::RedirectAppend) {
            let operator_token = self.advance();
            let operator = RedirectOp::from_token(&operator_token)
                .ok_or_else(|| ParseError::InvalidRedirect)?;
            
            // Next token should be a filename
            if !self.check(TokenKind::Word) {
                return Err(ParseError::UnexpectedToken(
                    self.current().clone(),
                    Expected::Filename
                ));
            }
            
            let filename = self.advance().value.clone();
            
            command = Command::Redirect(crate::parser::types::RedirectCommand {
                command: Box::new(command),
                operator,
                filename,
            });
        }
        
        Ok(command)
    }

    /// Parse simple commands: echo hello world
    fn parse_simple(&mut self) -> Result<Command, ParseError> {
        // Use our simple parser module
        let start_pos = self.pos;
        let (command, consumed) = simple::parse_simple(&self.tokens[start_pos..])?;
        
        // Advance our position by the number of tokens consumed
        self.pos += consumed;
        
        Ok(command)
    }

    // ===== Parser Helper Methods =====

    /// Check if we've consumed all tokens >len?
    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    /// Get the current token without advancing
    fn current(&self) -> &Token {
        if self.is_at_end() {
            &self.tokens[self.tokens.len() - 1] // Return EOF token
        } else {
            &self.tokens[self.pos]
        }
    }

    /// Check if current token matches the given kind
    fn check(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.tokens[self.pos].kind == kind
        }
    }

    /// Advance and return the current token
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.pos += 1;
        }
        &self.tokens[self.pos - 1]
    }

    /// If current token matches, advance and return true
    fn match_token(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }
}

//ss