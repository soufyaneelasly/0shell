use super::types::{Token, TokenKind, Span};

/// Error type for lexing issues.
#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnmatchedQuote(Span),
    #[allow(dead_code)]
    InvalidChar(char, Span),
}

/// Lexer state for handling different contexts
#[derive(PartialEq)]
enum LexerState {
    Normal,
    InQuote(char),
    Escaped,
}

/// Struct for the lexer: holds input and state.
pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    /// vreat  a new lexer from input string (userinput).
    pub fn new(input: &str) -> Self {
        Lexer {
            chars: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    /// move to next update line/col.
    fn advance(&mut self) {
        if self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
    }

    /// Peek at the next char without consomming  .
    fn peek(&self) -> Option<char> {
        if self.pos < self.chars.len() {
            Some(self.chars[self.pos]) //return current char if exit 
        } else {
            None //end of input 
        }
    }

    /// Peek at the next next char without advancing.
    fn peek_next(&self) -> Option<char> {
        if self.pos + 1 < self.chars.len() {
            Some(self.chars[self.pos + 1])
        } else {
            None
        }
    }

    /// Push current word to tokens if it exists
    fn push_word_if_exists(
        &self, 
        tokens: &mut Vec<Token>, 
        value: &mut String, 
        start_line: usize, 
        start_col: usize
    ) {
        if !value.is_empty() {
            tokens.push(Token::new(
                TokenKind::Word,
                value.clone(),
                Span::new(start_line, start_col, self.line, self.col.saturating_sub(1)),
            ));
            value.clear();
        }
    }

    /// Handle special characters that might be operators
    fn handle_special_char(
        &mut self,
        c: char,
        tokens: &mut Vec<Token>,
        current_value: &mut String,
        start_line: &mut usize,
        start_col: &mut usize,
    ) -> bool {
        // Only handle special chars in normal state (not in quotes)
        match c {
            '|' => {
                self.push_word_if_exists(tokens, current_value, *start_line, *start_col);
                if let Some('|') = self.peek_next() {
                    // Handle || operator
                    tokens.push(Token::new(
                        TokenKind::Or,
                        "||".to_string(),
                        Span::new(self.line, self.col, self.line, self.col + 1),
                    ));
                    self.advance(); // consume second |
                } else {
                    // Single | pipe
                    tokens.push(Token::new(
                        TokenKind::Pipe,
                        "|".to_string(),
                        Span::new(self.line, self.col, self.line, self.col),
                    ));
                }
                self.advance();
                *start_line = self.line;
                *start_col = self.col;
                true
            }
            '&' => {
                self.push_word_if_exists(tokens, current_value, *start_line, *start_col);
                if let Some('&') = self.peek_next() {
                    // Handle && operator
                    tokens.push(Token::new(
                        TokenKind::And,
                        "&&".to_string(),
                        Span::new(self.line, self.col, self.line, self.col + 1),
                    ));
                    self.advance(); // consume second &
                } else {
                    // Single & (could be background process, treat as word for now)
                    current_value.push(c);
                }
                true
            }
            ';' => {
                self.push_word_if_exists(tokens, current_value, *start_line, *start_col);
                tokens.push(Token::new(
                    TokenKind::Semicolon,
                    ";".to_string(),
                    Span::new(self.line, self.col, self.line, self.col),
                ));
                self.advance();
                *start_line = self.line;
                *start_col = self.col;
                true
            }
            '>' => {
                self.push_word_if_exists(tokens, current_value, *start_line, *start_col);
                tokens.push(Token::new(
                    TokenKind::RedirectOut,
                    ">".to_string(),
                    Span::new(self.line, self.col, self.line, self.col),
                ));
                self.advance();
                *start_line = self.line;
                *start_col = self.col;
                true
            }
            '<' => {
                self.push_word_if_exists(tokens, current_value, *start_line, *start_col);
                tokens.push(Token::new(
                    TokenKind::RedirectIn,
                    "<".to_string(),
                    Span::new(self.line, self.col, self.line, self.col),
                ));
                self.advance();
                *start_line = self.line;
                *start_col = self.col;
                true
            }
            _ => false,
        }
    }

    /// Main lexing function that will  returns all our tokens or error !.
    pub fn lex(mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        let mut state = LexerState::Normal;
        let mut current_value = String::new();
        let mut start_line = 1;
        let mut start_col = 1;
        let mut quote_char = '\0';

        while let Some(c) = self.peek() {
            match &state {
                LexerState::Normal => {
                    // Handle special characters first
                    if self.handle_special_char(c, &mut tokens, &mut current_value, &mut start_line, &mut start_col) {
                        continue;
                    }

                    match c {
                        ' ' | '\t' => {
                            // Whitespace ends current word
                            self.push_word_if_exists(&mut tokens, &mut current_value, start_line, start_col);
                            self.advance();
                            start_line = self.line;
                            start_col = self.col;
                        }
                        '"' | '\'' => {
                            // Start quote
                            self.push_word_if_exists(&mut tokens, &mut current_value, start_line, start_col);
                            state = LexerState::InQuote(c);
                            quote_char = c;
                            self.advance();
                            start_line = self.line;
                            start_col = self.col;
                        }
                        '\\' => {
                            // Escape character
                            state = LexerState::Escaped;
                            self.advance();
                        }
                        '\n' | '\r' => {
                            // Newline ends command
                            self.push_word_if_exists(&mut tokens, &mut current_value, start_line, start_col);
                            break;
                        }
                        _ => {
                            // Regular character
                            if current_value.is_empty() {
                                start_line = self.line;
                                start_col = self.col;
                            }
                            current_value.push(c);
                            self.advance();
                        }
                    }
                }
                LexerState::InQuote(q) => {
                    match c {
                        _ if c == *q => {
                            // End of quote - push the quoted content as a single word
                            self.push_word_if_exists(&mut tokens, &mut current_value, start_line, start_col);
                            state = LexerState::Normal;
                            self.advance();
                            start_line = self.line;
                            start_col = self.col;
                        }
                        '\\' => {
                            state = LexerState::Escaped;
                            self.advance();
                        }
                        _ => {
                            current_value.push(c);
                            self.advance();
                        }
                    }
                }
                LexerState::Escaped => {
                    // Handle escaped character - add it literally regardless of context
                    current_value.push(c);
                    // Return to previous state
                    state = if let LexerState::InQuote(_) = state { 
                        LexerState::InQuote(quote_char) 
                    } else { 
                        LexerState::Normal 
                    };
                    self.advance();
                }
            }
        }

        // Handle any remaining content at end of input
        self.push_word_if_exists(&mut tokens, &mut current_value, start_line, start_col);

        // Error checking
        if let LexerState::InQuote(_) = state {
            return Err(LexerError::UnmatchedQuote(Span::new(start_line, start_col, self.line, self.col)));
        }

        // Add EOF token
        tokens.push(Token::new(
            TokenKind::Eof,
            "".to_string(),
            Span::new(self.line, self.col, self.line, self.col),
        ));

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_basic() {
        let lexer = Lexer::new("echo hello world");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens.len(), 4); // 3 words + Eof
        assert_eq!(tokens[0].kind, TokenKind::Word);
        assert_eq!(tokens[0].value, "echo");
        assert_eq!(tokens[1].kind, TokenKind::Word);
        assert_eq!(tokens[1].value, "hello");
        assert_eq!(tokens[2].kind, TokenKind::Word);
        assert_eq!(tokens[2].value, "world");
        assert_eq!(tokens[3].kind, TokenKind::Eof);
    }

    #[test]
    fn test_lex_quoted() {
        let lexer = Lexer::new(r#"echo "hello world""#);
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens.len(), 3); // echo, "hello world", EOF
        assert_eq!(tokens[0].value, "echo");
        assert_eq!(tokens[1].value, "hello world"); // Single word with spaces
        assert_eq!(tokens[1].kind, TokenKind::Word);
    }

    #[test]
    fn test_lex_escaped() {
        let lexer = Lexer::new(r#"echo "hello \"world\"""#);
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens[0].value, "echo");
        assert_eq!(tokens[1].value, "hello \"world\""); // Quotes preserved inside
        assert_eq!(tokens[1].kind, TokenKind::Word);
    }

    #[test]
    fn test_lex_unmatched_quote() {
        let lexer = Lexer::new(r#"echo "unmatched"#);
        let err = lexer.lex().unwrap_err();
        assert!(matches!(err, LexerError::UnmatchedQuote(_)));
    }

    #[test]
    fn test_lex_empty_input() {
        let lexer = Lexer::new("");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens.len(), 1); // Just EOF
        assert_eq!(tokens[0].kind, TokenKind::Eof);
    }

    #[test]
    fn test_lex_multiple_spaces() {
        let lexer = Lexer::new("echo    world");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens.len(), 3); // echo, world, EOF
        assert_eq!(tokens[0].value, "echo");
        assert_eq!(tokens[1].value, "world");
    }

    #[test]
    fn test_lex_single_quotes() {
        let lexer = Lexer::new("echo 'hello world'");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens[0].value, "echo");
        assert_eq!(tokens[1].value, "hello world");
    }

    #[test]
    fn test_lex_mixed_quotes() {
        let lexer = Lexer::new(r#"echo "double" 'single'"#);
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens[0].value, "echo");
        assert_eq!(tokens[1].value, "double");
        assert_eq!(tokens[2].value, "single");
    }

    #[test]
    fn test_lex_with_pipes() {
        let lexer = Lexer::new("echo hello | cat");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens.len(), 5); // echo, hello, |, cat, EOF
        assert_eq!(tokens[0].value, "echo");
        assert_eq!(tokens[1].value, "hello");
        assert_eq!(tokens[2].kind, TokenKind::Pipe);
        assert_eq!(tokens[3].value, "cat");
    }

    #[test]
    fn test_lex_with_redirect() {
        let lexer = Lexer::new("echo hello > output.txt");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens.len(), 5); // echo, hello, >, output.txt, EOF
        assert_eq!(tokens[2].kind, TokenKind::RedirectOut);
    }

    #[test]
    fn test_lex_with_semicolon() {
        let lexer = Lexer::new("echo hello; ls -l");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens.len(), 6); // echo, hello, ;, ls, -l, EOF
        assert_eq!(tokens[2].kind, TokenKind::Semicolon);
    }

    #[test]
    fn test_lex_complex_command() {
        let lexer = Lexer::new(r#"echo "hello world" | grep "hello" > output.txt"#);
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens[0].value, "echo");
        assert_eq!(tokens[1].value, "hello world");
        assert_eq!(tokens[2].kind, TokenKind::Pipe);
        assert_eq!(tokens[3].value, "grep");
        assert_eq!(tokens[4].value, "hello");
        assert_eq!(tokens[5].kind, TokenKind::RedirectOut);
        assert_eq!(tokens[6].value, "output.txt");
    }
}