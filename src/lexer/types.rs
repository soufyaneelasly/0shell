/// Enum representing the kind (type) of a token.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Word,           // Alphanumeric words or symbols (e.g., "echo", "path/to/dir")
    Pipe,           // | 
    RedirectOut,    // >
    RedirectIn,     // <
    RedirectAppend, // >> (bonus +++)
    Semicolon,      // ;
    And,            // &&
    Or,             // ||
    Eof,            // End of input
}

/// Struct for tracking token positions (for error reporting).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

impl Span {
    pub fn new(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Span { start_line, start_col, end_line, end_col }
    }
}

/// Struct representing a full token: kind, value (string content), and span (position).
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub span: Span,
}
#[allow(dead_code)]
impl Token {
    pub fn new(kind: TokenKind, value: String, span: Span) -> Self {
        Token { kind, value, span }
    }
    
    /// Helper to check if token is a word jst match char
    pub fn is_word(&self) -> bool {
        matches!(self.kind, TokenKind::Word)
    }
    
    /// Helper to check if token is an operator
    pub fn is_operator(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Pipe | TokenKind::RedirectOut | TokenKind::RedirectIn | 
            TokenKind::Semicolon | TokenKind::And | TokenKind::Or
        )
    }
    
    /// Helper to check if token is a redirect
    pub fn is_redirect(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::RedirectOut | TokenKind::RedirectIn
        )
    }
}